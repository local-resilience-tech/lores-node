# lores-dev-server

In-memory development gRPC server for the lores-p2panda API.

## Context

[LoRes Mesh](https://lores.tech/) (short for Local Resilience Mesh) is a project to provide a network of local servers in your community. It is an example of [Neighbourhood-first Software](https://tv.lumbung.space/w/nzuB248U2LQA1LCn7vYmER).

In LoRes, each server hosts web software using recipes from [Co-op Cloud](https://coopcloud.tech/). Additionally, servers are in peer-to-peer communication with each other using [P2Panda](https://p2panda.org/). This P2P network of servers provides a redundant local infrastructure at key points around the neighbourhood that could be made to stay online in a power or internet outage.

## Usage

## As a lib (recommended)

Add `lores-dev-server` to your app project as a library dependency and wrap it in your own binary. This lets you pin a version and avoids relying on the global `cargo install` path.

Then, you need an entrypoint to it. It's recommended that you create a crate called `[YOURAPPNAME]-dev-server` with the following main.rs.

```rust
// src/main.rs
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    lores_dev_server::run_from_env().await
}
```

The server listens on `127.0.0.1:50051` by default, or on the address in the `PANDA_DEV_SERVER_ADDR` environment variable.

## As a binary

Install the crate directly with Cargo:

```sh
cargo install lores-dev-server
```

Then run the binary:

```sh
lores-dev-server
```

The server listens on `127.0.0.1:50051` by default. Set `PANDA_DEV_SERVER_ADDR` to override the bind address:

```sh
PANDA_DEV_SERVER_ADDR=127.0.0.1:8080 lores-dev-server
```

## License

This library is licensed under [The Anti-Capitalist Software License](https://anticapitalist.software/). This is intended to be a provocation to get us discussing the fact that open source software licencing has not stopped our software from enabling wealth extraction by corporations and billionaires, or from being used to enable wars and genocide.

The Anti-Capitalist license is one example of a license which makes restrictions on use inline with values describing a better world. There are other example licenses that limit other important things, such as harming humans, or use by AI. This one has been picked for this project because it aligned with the values of [Co-op Cloud](https://coopcloud.tech/).

Obviously using the Anti-Capitalist license in a library like this intended to be used in other software is almost certainly going to clash with whatever license you need for your application, even potentially with other licenses with more detailed values-based protections against misuse.

If you're a community run software project that just needs a different license, please have a human drop us a line by filing a [github issue](https://github.com/local-resilience-tech/lores-node) and we can sort you out.
