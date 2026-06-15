// SPDX-License-Identifier: MIT OR Apache-2.0

mod keystore;

use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use p2panda_core::{Hash, SigningKey, VerifyingKey};
use p2panda_net::{AddressBook, Discovery, Endpoint, addrs::NodeInfo};
use p2panda_store::{SqliteStore, SqliteStoreBuilder, address_book::AddressBookStore};
use tokio::signal;

use keystore::KeyStore;
use tracing::{info, warn};

/// Automatically remove node info which is older than one day.
const REMOVE_OLDER_THAN: Duration = Duration::from_secs(60 * 60 * 24);

/// Configurable p2panda bootstrap node.
#[derive(Parser)]
struct Args {
    /// Path to signing key.
    #[arg(short = 'p', long, value_name = "SIGNING_KEY")]
    signing_key: Option<PathBuf>,

    /// Network ID.
    #[arg(short = 'n', long)]
    network_id: String,

    /// Relay URL.
    #[arg(short = 'r', long)]
    relay_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging();

    let args = Args::parse();

    // Use an ephemeral signing key if one was not provided.
    let signing_key = if let Some(path) = args.signing_key {
        SigningKey::load_or_create_new(&path)?
    } else {
        SigningKey::generate()
    };

    let verifying_key = signing_key.verifying_key();
    let network_id = Hash::digest(&args.network_id);

    // Initialise in-memory SQLite database. The bootstrap doesn't need to persist anything.
    let store = SqliteStoreBuilder::new().build().await?;

    let address_book = AddressBook::builder().store(store.clone()).spawn().await?;

    let endpoint = Endpoint::builder(address_book.clone())
        .signing_key(signing_key)
        .relay_url(args.relay_url.parse().expect("valid iroh relay URL"))
        .network_id(network_id.into())
        .spawn()
        .await?;

    let _discovery = Discovery::builder(address_book, endpoint).spawn().await?;

    // Run a frequent "garbage collection" task which removes expired node info's from the address
    // book. Note that this doesn't mean that the node was actually "stale", it will simply be
    // removed based on the time from when it was inserted into our address book.
    //
    // If this node is still active, it will re-send us their latest info after a while again.
    let cleanup_handle = {
        tokio::task::spawn(async move {
            loop {
                tokio::time::sleep(REMOVE_OLDER_THAN).await;
                match <SqliteStore as AddressBookStore<VerifyingKey, NodeInfo>>::remove_older_than(
                    &store,
                    REMOVE_OLDER_THAN,
                )
                .await
                {
                    Ok(result) => {
                        info!("garbage collection removed {result} node infos from address book");
                    }
                    Err(err) => {
                        warn!("calling remove_older_than failed: {err}");
                    }
                }
            }
        })
    };

    info!(
        r#"
     ⢀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⢰⣿⡿⠗⠀⠠⠄⡀⠀⠀⠀⠀
⠀⠀⠀⠀⡜⠁⠀⠀⠀⠀⠀⠈⠑⢶⣶⡄
⢀⣶⣦⣸⠀⢼⣟⡇⠀⠀⢀⣀⠀⠘⡿⠃
⠀⢿⣿⣿⣄⠒⠀⠠⢶⡂⢫⣿⢇⢀⠃⠀  d8888b.  .d88b.   .d88b.  d888888b .d8888. d888888b d8888b.  .d8b.  d8888b.
⠀⠈⠻⣿⣿⣿⣶⣤⣀⣀⣀⣂⡠⠊⠀⠀  88  `8D .8P  Y8. .8P  Y8. `~~88~~' 88'  YP `~~88~~' 88  `8D d8' `8b 88  `8D
⠀⠀⠀⠃⠀⠀⠉⠙⠛⠿⣿⣿⣧⠀⠀⠀  88oooY' 88    88 88    88    88    `8bo.      88    88oobY' 88ooo88 88oodD'
⠀⠀⠘⡀⠀⠀⠀⠀⠀⠀⠘⣿⣿⡇⠀⠀  88~~~b. 88    88 88    88    88      `Y8b.    88    88`8b   88~~~88 88~~~
⠀⠀⠀⣷⣄⡀⠀⠀⠀⢀⣴⡟⠿⠃⠀⠀  88   8D `8b  d8' `8b  d8'    88    db   8D    88    88 `88. 88   88 88
⠀⠀⠀⢻⣿⣿⠉⠉⢹⣿⣿⠁⠀⠀⠀⠀  Y8888P'  `Y88P'   `Y88P'     YP    `8888Y'    YP    88   YD YP   YP 88
⠀⠀⠀⠀⠉⠁⠀⠀⠀⠉⠁			⠀⠀⠀⠀⠀
        "#
    );

    info!("node id:");
    info!("\t{}", verifying_key);

    info!("network id:");
    info!("\t{}", args.network_id);
    info!("\t{}", network_id);

    signal::ctrl_c().await?;

    cleanup_handle.abort();

    Ok(())
}

pub fn setup_logging() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();
}
