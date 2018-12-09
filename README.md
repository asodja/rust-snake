# Rust Snake
---
Server for multiplayer snake game written in [Rust](https://github.com/rust-lang/rust). It is my playground for trying out Rust and all it offers. 

Server-client protocol is a simpler version of one described in [Slither.io-Protocol](https://github.com/ClitherProject/Slither.io-Protocol). 

Since I was more interested in learning Rust and implementing server in Rust, game logic is very simple. Only basic snake movement and death when out of screen are implemented. 

### Web Client
There is also hacked-up game client written in javascript under the web folder. 

### Build and run
- cargo build or cargo build --release
- ./target/debug/rust-snake or ./target/release/rust-snake