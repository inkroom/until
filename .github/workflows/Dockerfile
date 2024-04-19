ARG RUST_VERSION=1.74.1
FROM inkbox/rust:${RUST_VERSION} as w
#FROM ghcr.io/inkbox/rust as w
WORKDIR /app
COPY . /app

RUN arch && mkdir out && if [ "$(arch)" = "x86_64" ]; then RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-pc-windows-gnu --config source.crates-io.replace-with=\"mirror\" --config source.mirror.registry=\"sparse+https://mirrors.ustc.edu.cn/crates.io-index/\" && mv /app/target/x86_64-pc-windows-gnu/release/un.exe out ; fi
ARG RUST_VERSION=1.74.1
FROM rust:${RUST_VERSION} as m
RUN sed -i 's/deb.debian.org/mirrors.ustc.edu.cn/g' /etc/apt/sources.list.d/debian.sources && apt update -y && apt upgrade -y

#FROM registry.gitlab.com/rust_musl_docker/image:stable-latest as m
WORKDIR /app
RUN arch && apt install -y musl-tools && rustc --version && rustup target add $(arch)-unknown-linux-musl 
COPY . /app
RUN cargo build --release --config source.crates-io.replace-with=\"mirror\" \
    --config source.mirror.registry=\"sparse+https://mirrors.ustc.edu.cn/crates.io-index/\" \
    && cargo build --release --config source.crates-io.replace-with=\"mirror\" \
    --config source.mirror.registry=\"sparse+https://mirrors.ustc.edu.cn/crates.io-index/\" -vv \
    --target=$(arch)-unknown-linux-musl \
    && mkdir out \
    && mv target/release/un ./out/un-dyn \
    && mv target/$(arch)-unknown-linux-musl/release/un ./out/un


FROM scratch
#FROM alpine
COPY --from=w /app/out/ /app/
COPY --from=m /app/out/ /app/
# COPY --from=m /app/target/aarch64-unknown-linux-musl/release/un /app/un-arm64
