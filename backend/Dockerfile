# This dockerfile is used to build the backend for development purposes only.
# In production, use the dockerfile in the root directory which also builds the frontend.

# Unlike the the production dockerfile, this one is also not multi-platform, it only
# builds for the host architecture.

# It's also not multi-stage, it builds and runs the backend in the same container using
# your existing cargo installation, which should avoid rebuilding the dependencies
# every time you change a file.

# BACKEND BUILDER
FROM rust:1 AS rustbuilder

RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked \
    --mount=target=/var/cache/apt,type=cache,sharing=locked \
    rm -f /etc/apt/apt.conf.d/docker-clean \
    && apt-get update \
    && apt-get -y --no-install-recommends install \
        docker.io

COPY ./src /app/backend/src
COPY ./Cargo.toml /app/backend/Cargo.toml
COPY ./Cargo.lock /app/backend/Cargo.lock
COPY ./config.toml /app/backend/config.toml
COPY ./migrations /app/backend/migrations
COPY ./main.sqlite /app/backend/main.sqlite
COPY ./operations.sqlite /app/backend/operations.sqlite



EXPOSE 8000
EXPOSE 2022/udp
EXPOSE 2023/udp
WORKDIR /app/backend
CMD ["cargo", "run"]
