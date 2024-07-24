# rust-little-computer-3

This project is a rust implementation of a virtual machine for runing 
[LC-3](https://en.wikipedia.org/wiki/Little_Computer_3) (Little Computer 3), 
which is a simplified educational assembly language. I am doing this for 
educational purposes as I am learning more about low level programming 
for reverse engeneering.

The idea came from [this](https://www.rodrigoaraujo.me/posts/lets-build-an-lc-3-virtual-machine/)
guide, but I followed
[this](https://www.rodrigoaraujo.me/posts/lets-build-an-lc-3-virtual-machine/) other blog
on rust.

## Depedencies

You need `rustc` and `cargo` to run this project. If you use Nix, you can enter
the developement environment with:
```bash
nix-shell
```

## Running

You can run any LC-3 program with:
```bash
cargo run <path>
```

There are some examples in `examples/`.

# Notes

LC-3 uses 15 instructions of 16 bits and has an address space of 2^16.

The instruction set:

![image](https://github.com/user-attachments/assets/88e60776-0fd5-4792-9ce8-3042a80d83cd)

[Specification](https://www.cs.colostate.edu/~cs270/.Spring23/resources/PattPatelAppA.pdf)
