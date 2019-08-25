# hrt - ham radio terminal

`hrt` is a simple terminal based application aimed at providing various tools for Amateur Radio operators.

## Config

Copy the example [hrt.toml](hrt.toml) config file to `~/.hrt.toml` and set the config values for the commands you plan to use.

## Commands

### Callsign lookup

You will need to set the default lookup source and its respective account info in config file before attempting to do callsign lookups.

```bash
hrt call KE0TSN
```

```
KE0TSN (QRZ)
  Name: Bryce D Johnston
  Location: Saint George, KS, United States
  Class: G
```

Use alternative lookup source than the default specified in config:

```bash
hrt call KE0TSN -s hamqth
```

```
ke0tsn (HamQTH)
  Name: Bryce D Johnston
  Location: Saint George, United States
```

Additional call command flags will be added to allow returning additional information.

## Contributing

Feature requests, bug reports, and pull requests are welcome on GitHub at https://github.com/beaorn/hrt.

## License

Licensed under the [MIT License](LICENSE.md).