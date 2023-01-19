# Proto Rust
Protobuf generated code for Rust 

# Usage

To create a new plugin, simply import this library into your crate, implement the [`Provider`](src/provider.rs#L529) trait and configure the server.

Read [here](https://github.com/done-devs/done/blob/70cc340795421b1c27686776c5f6160f0b56a83a/PLUGINS.md) for more details on plugins.

# Configuration
After [`provider.proto`](https://github.com/done-devs/proto/blob/c05fcf407cec1558509e0af0ba0cb7dac6f17117/provider.proto) is modified, run `cargo build` to generate the necessary code.
