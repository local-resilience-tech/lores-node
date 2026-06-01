# LoRes Node

Manage a [Node](https://jade.hopepunk.me/posts/sites-the-main-component-of-merri-bek-tech/)
in a LoRes Mesh.

LoRes Mesh allows you to run a network of low powered, resilient local web hosts that work with the internet, but are not dependent on it.

Developed by [Merri-bek Tech](https://www.merri-bek.tech/), but useful for all similar organisations.

# Development

## Requirements

On linux, you'll need the following things to build this:

- nodejs 24.16.0
  - I recommend that you install [asdf](https://asdf-vm.com/) in which case you can then run `asdf install` in the root of this repo to get the correct version.
- rust (latest)
  - You can use asdf for rust, but it's not commonly done. It's more common to use [rustup](https://rustup.rs/)
- build dependencies for rust libraries
  - `sudo apt install build-essentials libssl-dev pkg-config`
- direnv (there are workarounds if you don't want to install this)
  - `sudo apt install direnv` and then add to your shell, see [direnv.net](https://direnv.net/). I do this via adding the direnv plugin for oh-my-zsh.
- To work with installing LoRes Apps at all, you'll need to [install docker locally](https://docs.docker.com/engine/install/).
- To satisfy CORS, we need a specific host. It is recommended that you add the following line to your `/etc/hosts` file: `127.0.0.1 lores.localhost`. Then use http://lores.localhost:5173/ to access the frontend.

## Tech Stack

### Backend

The backend is a rust app, using the [Rocket framework](https://rocket.rs/). The rust package management tool "cargo" is used. To fetch what you need and start the server, run `cargo run`.

## Frontend

The frontend (web interface) is built using React, using the [Vite](https://vitejs.dev/) as the tooling to build and run. Packages are managed using npm. It's also heavily dependent on several other library choices:

- [Chakra UI](https://chakra-ui.com/) For the component library and styling

Running the frontend us done with `npm run dev`

## Generating a release

We use [Release it](https://github.com/release-it/release-it). For the first time, you'll need to install it with 'npm install' from the root directory of this project. After that, to run it, use `npm run release`

# Running

For running in development mode, we recommend using the run commands located in the [justfile](./justfile). To run these, install [Just](https://github.com/casey/just) and then execute the following to get started:

To fetch needed dependencies:

```bash
just setup
```

To run:

```bash
just dev
```

## Developing with Multiple Nodes (P2P)

To test P2P behaviour locally you can run two full instances on the same machine using docker. To execute, run:

```
just two-node
```

The two backends run as separate Docker containers on a shared bridge network. This gives each node its own IP address, which is necessary for mDNS multicast discovery and Iroh's QUIC/UDP traffic to work correctly between them — something that isn't possible when both processes share the same loopback interface.

This starts three processes:

| Process    | URL                          |
| ---------- | ---------------------------- |
| backends   | Docker (both nodes)          |
| frontend-1 | http://lores.localhost:5173  |
| frontend-2 | http://lores2.localhost:5174 |

The backends are accessible at:

| Node      | URL                          |
| --------- | ---------------------------- |
| backend-1 | http://lores.localhost:8200  |
| backend-2 | http://lores2.localhost:8201 |

Each node uses a separate set of SQLite databases under `backend/data/node_data/` and `backend/data/node2/` respectively, so they maintain independent state.

**Prerequisites:**

Both hostnames must resolve to `127.0.0.1`. Add this line to `/etc/hosts` if it is not already there:

```

127.0.0.1 lores.localhost lores2.localhost

```

Docker must be running. The two nodes will discover each other automatically via mDNS once they are on the same bridge network.

## Running Local Version in Release Mode

To run the app locally in release mode you can build it and run it using docker.

```
just docker
```

In release mode, the front end is not rendered, it's just built and placed in the docker container at the dir `/app/frontend`. The backend rust app builds an executable in `/app/backend` which is the command run by docker.

The backend will serve up the frontend, which only happens because the environment variable `ROCKET_FRONTEND_ASSET_PATH` is set in the docker container.

## Running Published Version

On every release, a version is published to Docker Hub at:
https://hub.docker.com/r/resilientlocaltech/lores-node

This can be run using a single docker command, like:
`sudo docker run -p 8000:80 resilientlocaltech/site-manager:latest`

However, since this app is likely to gain other dependent services, it is recommended that you use docker swarm.

### Using Docker Compose

This app is designed to be deployed using Docker Swarm. As such it provides a `compose.yml` file based on the [older v3 of the docker compose format](<https://github.com/docker/compose/blob/v1/docs/Compose%20file%20reference%20(legacy)/version-3.md>).

Before trying this with docker swarm, it's worth trying to run it with docker compose. In the root directory of the project, run:

`docker compose up`

The app should then be running at http//localhost:8000.

### Using Docker Swarm

This app is designed to be deployed on Raspberry Pis used in a docker swarm.

To deploy it, fetch the latest compose file using:

`curl https://raw.githubusercontent.com/local-resilience-tech/lores-node/refs/heads/main/compose.yml > lores-node.yml`

The, providing you have a docker swarm running, use:

`docker stack deploy -c ./lores-node.yml lores-node`

The app should then be running on post 8000 of your pi. We recommend using [swarmpit](https://swarmpit.io/) to monitor your swarm.

# Database Handling

The Backend uses an SQLite database. The rust integration uses a library called `sqlx` that handles queries and database migrations, and also performs compile time checking of SQL queries against the DB structure. There are some command-line tools to help out with this

## SQLX Command Line Tools

### Installing

You'll need the following pre-requisites on linux:

`sudo apt install pkg-config libssl-dev`

Then install with `cargo install sqlx-cli`

### Useful commands

Re-create the database:
`DATABASE_URL=sqlite:./projections.sqlite cargo sqlx database reset`

Re-build the query indexes:
`DATABASE_URL=sqlite:./projections.sqlite cargo sqlx prepare`
