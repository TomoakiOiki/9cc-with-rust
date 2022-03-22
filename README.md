# What's this?
A implementation of [9cc](https://www.sigbus.info/compilerbook) with Rust.

The purpose of this project is to learn low layer knowledge and basic Rust implementation skill.

# How to run?

```
$ docker build . -t 9cc-with-rust
$ docker run --rm -it -v $(pwd)/9cc:/9cc -w /9cc 9cc-with-rust
```
