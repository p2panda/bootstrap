mod keystore;

use std::{hash::Hash as StdHash, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use p2panda_core::{Hash, PrivateKey};
use p2panda_net::{NetworkBuilder, TopicId};
use p2panda_sync::TopicQuery;
use serde::{Deserialize, Serialize};
use tokio::signal;

use keystore::KeyStore;

#[derive(Clone, Debug, PartialEq, Eq, StdHash, Serialize, Deserialize)]
struct Dummy {}

impl TopicQuery for Dummy {}

impl TopicId for Dummy {
    fn id(&self) -> [u8; 32] {
        unreachable!()
    }
}

/// Configurable p2panda bootstrap node.
///
/// If no data path is provided, the node will run with a randomly-generated ephemeral keypair.
/// Otherwise the keypair will be loaded from file (or created and saved to file if one was not
/// previously generated).
///
/// The network ID and relay URL must be provided.
///
/// Network system event logging may be optionally enabled and will print to `stdout`.
#[derive(Parser)]
struct Args {
    /// Path to data directory.
    #[arg(short = 'p', long, value_name = "DATA_PATH")]
    data_path: Option<PathBuf>,

    /// Network ID.
    #[arg(short = 'n', long)]
    network_id: String,

    /// Relay URL.
    #[arg(short = 'r', long)]
    relay_url: String,

    /// Print network events to `stdout`.
    #[arg(short = 'l', long)]
    log_events: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse the ClI arguments.
    let args = Args::parse();

    // Use an ephemeral private key if one was not provided.
    let private_key = if let Some(path) = args.data_path {
        let private_key_path = path.join("private_key.txt");
        PrivateKey::load_or_create_new(&private_key_path)?
    } else {
        PrivateKey::new()
    };

    // Define the public key, network ID and relay URL.
    let public_key = private_key.public_key();
    let network_id = Hash::new(&args.network_id);
    let relay_url = args.relay_url.parse().expect("valid relay url");

    // Build the bootstrap network.
    let network = NetworkBuilder::<Dummy>::new(network_id.into())
        .private_key(private_key)
        .bootstrap()
        .relay(relay_url, false, 0)
        .build()
        .await?;

    // Print banner.
    println!(
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

    // Print network info to the terminal.
    println!("node id:");
    println!("\t{}", public_key);

    println!("network id:");
    println!("\t{}", args.network_id);
    println!("\t{}", network_id);

    println!("node relay server url:");
    let relay_url = network
        .endpoint()
        .home_relay()
        .get()
        .unwrap()
        .expect("should be connected to a relay server");
    println!("\t{relay_url}");

    println!("node listening addresses:");
    for local_endpoint in network
        .endpoint()
        .direct_addresses()
        .initialized()
        .await
        .unwrap()
    {
        println!("\t{}", local_endpoint.addr)
    }

    println!("log events:");
    if args.log_events {
        println!("\tenabled");
    } else {
        println!("\tdisabled");
    }

    println!();

    // Print network events to `stdout` if enabled.
    if args.log_events {
        tokio::spawn(async move {
            let mut rx = network.events().await.unwrap();
            while let Ok(event) = rx.recv().await {
                println!("{:?}", event);
            }
        });
    }

    // Listen for `Ctrl+c` to terminate.
    signal::ctrl_c().await?;

    Ok(())
}
