# nornpulse

## Prerequisites

### Modifying engine.exe

### Getting rust dependencies

```sh
rustup install nightly
rustup target add i686-pc-windows-msvc
```

## Build command

```
cd nornpulse
cargo build
```

## Execute

Copy `nornpulse.dll` into your `Docking Station` folder and run `engine.exe`.

## How was engine.exe modified?

The following op-codes were inserted into engine.exe:

```
0058744F  |. FF15 48216600  CALL DWORD PTR DS:[<&nornpulse.nornpulse>;  nornpuls.nornpulse_main
00587455  |. 90             NOP
00587456  |. 90             NOP
```
