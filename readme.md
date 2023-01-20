# Rust GameBoy

<table><tr>
<td> <img src="images/nintendo.png" alt="Drawing" style="width: 300px;"/> </td>
<td> <img src="images/flappyboy.png" alt="Drawing" style="width: 300px;"/> </td>
</tr></table>

## Description

*!work in progress!*

Rust Gameboy is a open source Game Boy (GB) written in rust. It has a minibf front end.

## Features

### Implemented 

- cpu
- interupts
- joypad
- lcd controller

### Not Implemented

- sound
- Game Boy color features
- choice of palette
- debugger *!work in progress!*
- automatic blargg tests for CI

Note that the cpu and interups blaggs tests pass on this emulator when tested manually.

## Controls

```
Z           - A
X           - B
ENTER       - start
BACKSPACE   - select
UP          - UP
DOWN        - DOWN
RIGHT       - RIGHT
LEFT        - LEFT
```

## How to Run

The crate `rust-gameboy` contains the gameboy. It can be run as follow. 
```bash
cargo run --bin rust-gameboy <file.gb>
```

The carte `codegenerator` is used to generate the CPU code from a json file. It does not need to be run for playing the GameBoy. If you want to run it, perform as follow:
```bash
cargo run --bin codegenerator
```


## Source

gameboy manual: http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf

opcodes.json: https://gbdev.io/gb-opcodes//optables/

Tutorial gameboy emulator in java: https://cs108.epfl.ch/archive/18/p/00_introduction.html