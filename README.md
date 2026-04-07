# ZKMC
This project contains an implementation of a Zero-Knowledge Prover and Verifier for proving that a player knows the location of a Minecraft stronghold. It leverages SP1 as a zkVM, and implements native rust stronghold generation based on cubiomes and my own reverse engineering efforts. It should work for MC 1.21+.

## Building
Building requires having rust installed.

You will also have to install the succinct sp1 rust toolchain.
[https://docs.succinct.xyz/docs/sp1/getting-started/install](https://docs.succinct.xyz/docs/sp1/getting-started/install)
```sh
git clone https://github.com/c-ola/zkmc.git
cargo build --release
```


## Running
```sh
cargo run --release -p mc-script -- args
or
./target/release/mc-script <args>
```

```sh
Prove Knowledge of Minecraft strongholds with a ZKVM

Usage: mc-script <COMMAND>

Commands:
  prove
  verify
  execute
  average
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Proving
```sh
sage: mc-script prove [OPTIONS] --seed <SEED> --x <X> --z <Z>

Options:
  -s, --seed <SEED>      Minecraft World Seed
  -x, --x <X>            Block X Position
  -z, --z <Z>            Block Z Position
  -o, --output <OUTPUT>  [default: proof.bin]
  -h, --help             Print help
```
### Verifying
```sh
Usage: mc-script verify [OPTIONS] --seed <SEED>

Options:
  -s, --seed <SEED>    Minecraft World Seed
  -i, --input <INPUT>  [default: proof.bin]
  -h, --help           Print help
```


### Example
With `seed=-6152149964729591252`, finding the second generated stronghold based on the coords `1284, -1210`.

The proof can be generated like this:
```sh
./target/release/mc-script prove --seed=-6152149964729591252 -x=1284 -z=-1210
```

And verified like this:
```sh
./target/release/mc-script verify --seed=-6152149964729591252
```


