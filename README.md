# rust-little-computer-3

This project is a rust implementation of a virtual machine for runing 
[LC-3](https://en.wikipedia.org/wiki/Little_Computer_3) (Little Computer 3), 
which is a simplifies educational assembly language. I am doing this for 
educational purposes, as I am learning more about low level programming 
for reverse engeneering.

The idea came from [this](https://www.rodrigoaraujo.me/posts/lets-build-an-lc-3-virtual-machine/)
guide, but I followed
[this](https://www.rodrigoaraujo.me/posts/lets-build-an-lc-3-virtual-machine/) ther blog
on rust.

## Depedencies

You need `rustc` and `cargo` to run this project. If you use Nix, you can enter
the developement environment with:
```bash
nix-shell
```

## Running

You can run the project with:
```bash
cargo run examples/rogue.obj
```

# Notes

LC-3 uses 15 instructions of 16 bits and has an address space of 2^16.

TODO: Add instructions image
