# LoRes App Node

Client code to embed in a web application, to use P2P functionality as part of a **LoRes Mesh**.

## Context

### LoRes

[LoRes Mesh](https://lores.tech/) (short for Local Resilience Mesh) is a project to provide a network of local servers in your community. It is an example of [Neighbourhood-first Software](https://tv.lumbung.space/w/nzuB248U2LQA1LCn7vYmER).

In LoRes, each server hosts web software using recipes from [Co-op Cloud](https://coopcloud.tech/). Additionally, servers are in peer-to-peer communication with each other using [P2Panda](https://p2panda.org/). This P2P network of servers provides a redundant local infrastructure at key points around the neighbourhood that could be made to stay online in a power or internet outage.

### Communicating with LoRes Node

Each server in a LoRes Mesh has a management app installed, called LoRes Node. This app runs the P2Panda node and manages communication with other servers. In order to allow other apps on the server to communicate with each other over the P2Panda network, it also provides a gRPC API.

Applications installed on the server can communicate with it, using the [lores-p2panda-client](https://crates.io/crates/lores-p2panda-client). If your application only wants to send messages, and subscribe to ones incoming right now, then using that crate will be sufficient.

However, if you're building a more robust event-sourced application, you might find that this crate, `lores-app-node` provides a better layer on top.

## Features

The goal of LoRes App Node is to be the engine that powers event-sourced application developments using a local p2panda server, such as lores-node.

It's key features, compared to just communicating with the node directly using the lores-p2panda-client, are:

- Ability to buffer messages locally if the lores-node is offline. This supports both resilience, and the ability to make use of a lores-node optional.
- Optimistic loopback of events, allowing your application to project their effects without waiting for communication to the server.
- Resilient subscriptions, with support for re-connection with exponential backoff.
- Structured wire format container
- Support for a sqlite projection database, built from a schema rather than migrated.
- Replaying all operations, to allow for projections to be re-created.

## License

This library is licensed under [The Anti-Capitalist Software License](https://anticapitalist.software/). This is intended to be a provocation to get us discussing the fact that open source software licencing has not stopped our software from enabling wealth extraction by corporations and billionaires, or from being used to enable wars and genocide.

The Anti-Capitalist license is one example of a license which makes restrictions on use inline with values describing a better world. There are other example licenses that limit other important things, such as harming humans, or use by AI. This one has been picked for this project because it aligned with the values of [Co-op Cloud](https://coopcloud.tech/).

Obviously using the Anti-Capitalist license in a library like this intended to be used in other software is almost certainly going to clash with whatever license you need for your application, even potentially with other licenses with more detailed values-based protections against misuse.

If you're a community run software project that just needs a different license, please have a human drop us a line by filing a [github issue](https://github.com/local-resilience-tech/lores-node) and we can sort you out.
