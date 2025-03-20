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

If no data path is provided, the node will run with a randomly-generated ephemeral keypair.
Otherwise the keypair will be loaded from file (or created and saved to file if one was not
previously generated).

The network ID and relay URL **must** be provided.

Network system event logging may be optionally enabled and will print to `stdout`.

## Help

`bootstrap --help`

```
Configurable p2panda bootstrap node.

Usage: bootstrap [OPTIONS] --network-id <NETWORK_ID> --relay-url <RELAY_URL>

Options:
  -p, --data-path <DATA_PATH>
          Path to data directory

  -n, --network-id <NETWORK_ID>
          Network ID

  -r, --relay-url <RELAY_URL>
          Relay URL

  -l, --log-events
          Print network events to `stdout`

  -h, --help
          Print help (see a summary with '-h')
```

## Example

`bootstrap -p "." -n chat -r https://wasser.liebechaos.org/`

```
node id:
    9edfa7abefc995f4a9ebf8ad35d2c79f96045ce6a42cb0421ce411d08f8373e5
network id:
    chat
    504c1dbb87fc1cd93594bd6baad1b520229bd222e16d9c48138998f602993c67
node relay server url:
    https://wasser.liebechaos.org./
node listening addresses:
    10.145.136.48:2022
    146.70.189.45:2022
    192.168.160.22:2022
log events:
    disabled
```
