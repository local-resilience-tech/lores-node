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
FROM ubuntu AS runner
COPY --from=rustbuilder /app/lores-node /app/backend/lores-node
COPY --from=vitebuilder /app/dist /app/frontend
ENV FRONTEND_PATH=/app/frontend
ENV DATABASE_URL=sqlite:/app/lores-node.db
ENV CONFIG_PATH=/app/config.toml
EXPOSE 8000
EXPOSE 2022/udp
EXPOSE 2023/udp
WORKDIR /app/backend
CMD ["./lores-node"]
