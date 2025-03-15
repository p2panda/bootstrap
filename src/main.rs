use std::hash::Hash as StdHash;

use anyhow::Result;
use p2panda_core::{Hash, PrivateKey};
use p2panda_discovery::mdns::LocalDiscovery;
use p2panda_net::{NetworkBuilder, TopicId};
use p2panda_sync::TopicQuery;
use serde::{Deserialize, Serialize};
use tokio::signal;

const PRIVATE_KEY: [u8; 32] = [
    202, 81, 96, 79, 125, 131, 126, 254, 100, 95, 45, 242, 80, 127, 43, 0, 207, 96, 96, 30, 247,
    93, 96, 60, 174, 170, 241, 239, 100, 113, 50, 130,
];

const RELAY_URL: &str = "https://staging-euw1-1.relay.iroh.network/";

#[derive(Clone, Debug, PartialEq, Eq, StdHash, Serialize, Deserialize)]
struct Dummy {}

impl TopicQuery for Dummy {}

impl TopicId for Dummy {
    fn id(&self) -> [u8; 32] {
        unreachable!()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let network_id = Hash::new(b"aardvark <3");
    let private_key = PrivateKey::from_bytes(&PRIVATE_KEY);
    let relay_url = RELAY_URL.parse().expect("valid relay url");

    println!("public_key=\"{}\"", private_key.public_key());

    let network = NetworkBuilder::<Dummy>::new(network_id.into())
        .private_key(private_key)
        .bootstrap()
        .relay(relay_url, false, 0)
        .discovery(LocalDiscovery::new())
        .build()
        .await?;

    tokio::spawn(async move {
        let mut rx = network.events().await.unwrap();
        while let Ok(event) = rx.recv().await {
            println!("{:?}", event);
        }
    });

    signal::ctrl_c().await?;

    Ok(())
}
