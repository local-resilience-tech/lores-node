# Lores P2Panda Client

Rust client for the lores-p2panda-server gRPC API.

## What is this?

[LoRes Mesh](https://lores.tech/) (short for Local Resilience Mesh) is a project to provide a network of local servers in your community. It is an example of [Neighbourhood-first Software](https://tv.lumbung.space/w/nzuB248U2LQA1LCn7vYmER).

In LoRes, each server hosts web software using recipes from [Co-op Cloud](https://coopcloud.tech/). Additionally, servers are in peer-to-peer communication with each other using [P2Panda](https://p2panda.org/). This P2P network of servers provides a redundant local infrastructure at key points around the neighbourhood that could be made to stay online in a power or internet outage.

Each server in a LoRes Mesh has a management app installed, called LoRes Node. This app runs the P2Panda node and manages communication with other servers. In order to allow other apps on the server to communicate with each other over the P2Panda network, it also provides a gRPC API.

## The Lores P2Panda Server gRPC API

The Lores P2Panda Server is designed to be accessed by apps running on the same docker network. Given that, it has no access protection.

Proto definitions for this server can be found in [panda.proto](https://github.com/local-resilience-tech/lores-node/blob/main/backend/lores-p2panda-server/proto/panda.proto).

This library is a thin wrapper around a generated client for these protos.

## Licence

This library is licenced under [The Anti-Capitalist Software License](https://anticapitalist.software/). This is intended to be a provocation to get us discussing the fact that open source software licencing has not stopped our software from enabling wealth extraction by corporations and billionaires, or from being used to enable wars and genocide.

The Anti-Capitalist license is one example of a licence which makes restrictions on use inline with values describing a better world. There are other example licenses that limit other important things, such as harming humans, or use by AI. This one has been picked for this project because it aligned with the values of [Co-op Cloud](https://coopcloud.tech/).

Obviously using the Anti-Capitalist license in a library like this intended to be used in other software is almost certainly going to clash with whatever license you need for your application, even potentially with other licenses with more detailed values-based protections against misuse.

If you're a community run software project that just needs a different license, please have a human drop us a line by filing a [github issue](https://github.com/local-resilience-tech/lores-node) and we can sort you out.
