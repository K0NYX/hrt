# hrt - ham radio terminal

![Actions Status](https://github.com/beaorn/hrt/workflows/Rust/badge.svg)
[![](https://img.shields.io/crates/v/hrt.svg?colorB=00aa88)](https://crates.io/crates/hrt)

`hrt` is a simple terminal based application aimed at providing various tools for Amateur Radio operators.

## Installation

Currently while in early development stage you will need [Rust and Cargo installed](https://www.rust-lang.org/tools/install) to run `hrt`

```bash
cargo install hrt
```

## Config

After installing, run the config generator:

```bash
hrt init
```

This will create a `.hrt.toml` file in your home directory and output the path to it. Open this file up with an editor (vim, nano, notepad, VS Code, etc.) and set the config values for the commands you plan to use.

## Commands

To see a list of all available commands, simply run `hrt` without any arguments.

### Callsign lookup

You will need to set your account info in `.hrt.toml` for [qrz](https://www.qrz.com) and/or [hamqth](https://www.hamqth.com).

Lookup with QRZ:
```bash
hrt call K0NYX
```

example output:
```
K0NYX (QRZ)
  Name: Bryce Johnston
  Email: bryce@beaorn.com
  Address: 100 Antenna Way
  Location: Manhattan, KS 66502
  Country: United States
  Class: G
```

Use alternative lookup source HamQTH:
```bash
hrt call K0NYX -s hamqth
```

example output:
```
K0NYX (HamQTH)
  Name: Bryce Johnston
  Location: Manhattan, KS
  Country United States
```

### DXCC lookup

You will need to set your account info in `.hrt.toml` for [qrz](https://www.qrz.com). No account info needed for [hamqth](https://www.hamqth.com) DXCC lookups.

Lookup by entity code / adif number with QRZ:
```bash
hrt dxcc 291
```

example output:
```
291 (QRZ)
  Name: United States
  ITU: 0
  CQ: 0
  UTC: -5
```

Use alternative lookup source HamQTH:
```bash
hrt dxcc 291 -s hamqth
```

example output:
```
291 (HamQTH)
  Name: United States
  ITU: 8
  UTC: 5
  Details:
```

Lookup by callsign
```bash
hrt dxcc K0NYX
```

example output:
```
291 (QRZ)
  Country: United States
  ITU: 0
  CQ: 0
  UTC: -5
```

Use alternative lookup source HamQTH:
```bash
hrt dxcc K0NYX -s hamqth
```

example output:
```
291 (HamQTH)
  Name: United States
  ITU: 07
  UTC: 7
  Details: USA - CO,IA,KS,MN,MO,ND,NE,SD
```

## Contributing

Feature requests, bug reports, and pull requests are welcome on GitHub at https://github.com/beaorn/hrt.

## License

Licensed under the [MIT License](LICENSE.md).