```
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
⠀⠀⠀⠀⠉⠁⠀⠀⠀⠉⠁
```

Configurable p2panda bootstrap node.

## Usage

If no signing key path is provided, the node will run with a randomly-generated ephemeral keypair.
Otherwise the key will be loaded from file (or created and saved to file if one was not previously
generated).

The network ID and relay URL **must** be provided.

Network system event logging may be optionally enabled with the `RUST_LOG` environment variable.

```
Usage: bootstrap [OPTIONS] --network-id <NETWORK_ID> --relay-url <RELAY_URL>

Options:
  -p, --signing-key <SIGNING_KEY>  Path to signing key
  -n, --network-id <NETWORK_ID>    Network ID
  -r, --relay-url <RELAY_URL>      Relay URL
  -h, --help                       Print help
```

Running `bootstrap` will print the node's ID you can then use in your application to bootstrap into
the network. We provide methods for this in
[`p2panda`](https://docs.rs/p2panda/latest/p2panda/struct.NodeBuilder.html#method.bootstrap) and
[`p2panda-net`](https://docs.rs/p2panda-net/latest/p2panda_net/struct.AddressBook.html#example).

## Example

```bash
RUST_LOG=info bootstrap \
    --signing-key ./key.secret \
    --network-id chat \
    --relay-url https://euc1-1.relay.n0.iroh-canary.iroh.link.
```
