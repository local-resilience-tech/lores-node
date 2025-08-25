## RUNTIME BASE
FROM ubuntu AS runtime_base
RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked \
    --mount=target=/var/cache/apt,type=cache,sharing=locked \
    rm -f /etc/apt/apt.conf.d/docker-clean \
    && apt-get update \
    && apt-get -y --no-install-recommends install \
        docker.io libssl-dev ca-certificates pkg-config libgit2-dev

# FRONTEND BUILDER
FROM --platform=$BUILDPLATFORM node:22 AS vitebuilder
WORKDIR /app
COPY ./frontend .
RUN npm install && npm run build

# BACKEND BUILDER
FROM --platform=$BUILDPLATFORM rust:1 AS rustbuilder
ARG TARGETARCH
WORKDIR /app

# should write /.platform and /.compiler
COPY --chmod=555 deployment/platform.sh .
RUN ./platform.sh

# setup rust compilation for the target platform
RUN rustup component add rustfmt
CMD /bin/bash
RUN rustup target add $(cat /app/.platform)
RUN apt-get update && apt-get install -y unzip $(cat /app/.compiler) pkg-config libssl-dev
COPY deployment/cargo-config.toml ./.cargo/config

# Compile the backend
COPY ./backend .
RUN RUSTFLAGS=-g SQLX_OFFLINE=true cargo build --release --target $(cat /app/.platform)
RUN cp /app/target/$(cat /app/.platform)/release/lores-node /app/lores-node

# RUNNER
FROM runtime_base AS runner
COPY --from=rustbuilder /app/lores-node /app/backend/lores-node
COPY --from=vitebuilder /app/dist /app/frontend
COPY ./backend/migrations_nodedatadb /app/backend/migrations_nodedatadb
COPY ./backend/migrations_projectiondb /app/backend/migrations_projectiondb
ENV FRONTEND_PATH=/app/frontend
ENV DATABASE_URL=sqlite:/app/lores-node.db
ENV CONFIG_PATH=/app/config.toml
EXPOSE 8200
EXPOSE 2022/udp
EXPOSE 2023/udp
WORKDIR /app/backend
CMD ["./lores-node"]
