# nornpulse

## Prerequisites

### Modifying engine.exe

Apply the following modifications to engine.exe with OllyDbg:

```
0058744F  |. FF15 48216600  CALL DWORD PTR DS:[<&nornpulse.nornpulse>;  nornpuls.nornpulse_main
00587455  |. 90             NOP
00587456  |. 90             NOP
```

### Getting rust dependencies

```sh
rustup install nightly
rustup target add i686-pc-windows-msvc
```

## Build command

```
cargo +nightly build --target 1686-pc-windows-msvc`
```