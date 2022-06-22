# What's this?
A implementation of [9cc](https://www.sigbus.info/compilerbook) with Rust.

The purpose of this project is to learn low layer knowledge and basic Rust implementation skill.

# Status
**Completed 11/28(From Chapter 12 onwards, it is in the middle of writing)(12章から先は執筆途中らしい)**

# How to run?
## Run playground container
```
$ docker build . -t 9cc-with-rust
$ docker run --rm -it -v $(pwd)/compiler:/compiler -w /compiler 9cc-with-rust
```
