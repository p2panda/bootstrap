// SPDX-License-Identifier: MIT OR Apache-2.0

mod keystore;

use std::collections::HashMap;
use std::convert::Infallible;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use p2panda_core::{Hash, PrivateKey};
use p2panda_discovery::address_book::AddressBookStore as _;
use p2panda_discovery::address_book::memory::MemoryStore as AddressBookStore;
use p2panda_net::{NetworkBuilder, NodeId, NodeInfo, TopicId};
use p2panda_sync::TopicSyncManager;
use p2panda_sync::log_sync::Logs;
use p2panda_sync::managers::topic_sync_manager::TopicSyncManagerConfig;
use p2panda_sync::topic_log_sync::TopicLogMap;
use rand_chacha::ChaCha20Rng;
use rand_chacha::rand_core::SeedableRng;
use tokio::signal;

use keystore::KeyStore;
use tracing::{info, warn};

/// Automatically remove node info which is older than one day.
const REMOVE_OLDER_THAN: Duration = Duration::from_secs(60 * 60 * 24);

/// Configurable p2panda bootstrap node.
#[derive(Parser)]
struct Args {
    /// Path to private key.
    #[arg(short = 'p', long, value_name = "PRIVATE_KEY")]
    private_key: Option<PathBuf>,

    /// Network ID.
    #[arg(short = 'n', long)]
    network_id: String,

    /// Relay URL.
    #[arg(short = 'r', long)]
    relay_url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    setup_logging();

    let args = Args::parse();

    // Use an ephemeral private key if one was not provided.
    let private_key = if let Some(path) = args.private_key {
        PrivateKey::load_or_create_new(&path)?
    } else {
        PrivateKey::new()
    };

    let public_key = private_key.public_key();
    let network_id = Hash::new(&args.network_id);

    let store = {
        let rng = ChaCha20Rng::from_os_rng();
        AddressBookStore::<ChaCha20Rng, NodeId, NodeInfo>::new(rng.clone())
    };

    let sync_config = {
        let store = p2panda_store::MemoryStore::<u64, ()>::new();
        let topic_map = DummyMap;
        TopicSyncManagerConfig { topic_map, store }
    };

    // Run a frequent "garbage collection" task which removes expired node info's from the address
    // book. Note that this doesn't mean that the node was actually "stale", it will simply be
    // removed based on the time from when it was inserted into our address book.
    //
    // If this node is still active, it will re-send us their latest info after a while again.
    let cleanup_handle = {
        let store = store.clone();
        tokio::task::spawn(async move {
            loop {
                tokio::time::sleep(REMOVE_OLDER_THAN).await;
                match store.remove_older_than(REMOVE_OLDER_THAN).await {
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

    // Build the bootstrap network.
    let _network = NetworkBuilder::new(network_id.into())
        .private_key(private_key)
        .relay(args.relay_url.parse().expect("valid relay url"))
        .build::<_, TopicSyncManager<TopicId, _, _, _, ()>>(store, sync_config)
        .await?;

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
    info!("\t{}", public_key);

    info!("network id:");
    info!("\t{}", args.network_id);
    info!("\t{}", network_id);

    signal::ctrl_c().await?;

    cleanup_handle.abort();

    Ok(())
}

#[derive(Clone, Default, Debug)]
pub struct DummyMap;

impl TopicLogMap<TopicId, u64> for DummyMap {
    type Error = Infallible;

    async fn get(&self, _topic_query: &TopicId) -> Result<Logs<u64>, Self::Error> {
        Ok(HashMap::new())
    }
}

pub fn setup_logging() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init();
}
