ARG RUST_VERSION=1.68
FROM rust:${RUST_VERSION}-slim-bullseye as build

RUN apt update \
    && apt install --yes binutils build-essential pkg-config libssl-dev clang lld git protobuf-compiler \
    && rm -rf /var/lib/{apt,dpkg,cache,log}

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt update \
    && apt install --yes ca-certificates gettext-base libssl1.1 --no-install-recommends \
    && rm -rf /var/lib/{apt,dpkg,cache,log}

COPY --from=build "/target/release/annotator" "/bin/annotator"

EXPOSE 8000

CMD ["/bin/annotator"]