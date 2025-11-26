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

## Behaviour

If no private key path is provided, the node will run with a randomly-generated ephemeral keypair.
Otherwise the keypair will be loaded from file (or created and saved to file if one was not
previously generated).

The network ID and relay URL **must** be provided.

Network system event logging may be optionally enabled and will print to `stdout`.

## Help

`bootstrap --help`

```
Configurable p2panda bootstrap node

Usage: bootstrap [OPTIONS] --network-id <NETWORK_ID> --relay-url <RELAY_URL>

Options:
  -p, --private-key <PRIVATE_KEY>  Path to private key
  -n, --network-id <NETWORK_ID>    Network ID
  -r, --relay-url <RELAY_URL>      Relay URL
  -h, --help                       Print help
```

## Example

`RUST_LOG=info bootstrap --network-id chat --relay-url https://euc1-1.relay.n0.iroh-canary.iroh.link.`

```
2026-01-26T16:13:43.759430Z  INFO bootstrap: node id:
2026-01-26T16:13:43.759460Z  INFO bootstrap: 	bdde8d429630cc8cf2a7bf89afb527ce1483ab2254f6e01cb83c4685bcc0034d
2026-01-26T16:13:43.759514Z  INFO bootstrap: network id:
2026-01-26T16:13:43.759537Z  INFO bootstrap: 	chat
2026-01-26T16:13:43.759556Z  INFO bootstrap: 	504c1dbb87fc1cd93594bd6baad1b520229bd222e16d9c48138998f602993c67
```
