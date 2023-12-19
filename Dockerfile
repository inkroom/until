FROM inkbox/rust:1.74.1 as w
#FROM ghcr.io/inkbox/rust as w
WORKDIR /app
COPY . /app

RUN cargo build --release && RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-pc-windows-gnu

FROM ghcr.m.daocloud.io/inkroomtemp/rust_musl_build:1.74.1 as m
#FROM registry.gitlab.com/rust_musl_docker/image:stable-latest as m
WORKDIR /app
RUN rustup target add aarch64-apple-darwin && rustup target add aarch64-unknown-linux-musl
COPY . /app
RUN cargo build --release -vv --target=x86_64-unknown-linux-musl
RUN sed -i "s@http://deb.debian.org@https://mirrors.huaweicloud.com@g" /etc/apt/sources.list && sed -i "s@http://security.debian.org@https://mirrors.huaweicloud.com@g" /etc/apt/sources.list && apt update -y 
# RUN apt remove -y gcc && apt install -y gcc-aarch64-linux-gnu 
# RUN echo '[target.aarch64-unknown-linux-musl]' >> ~/.cargo/config.toml && echo 'linker = "aarch64-linux-gnu-gcc"' >> ~/.cargo/config.toml && echo 'ar = "aarch64-unknown-linux-gnu-gcc"' >> ~/.cargo/config.toml && cat ~/.cargo/config.toml && cargo build --release -vv --target=aarch64-unknown-linux-musl


FROM scratch
#FROM alpine
COPY --from=w /app/target/x86_64-pc-windows-gnu/release/until.exe /app/until.exe
COPY --from=m /app/target/x86_64-unknown-linux-musl/release/until /app/until
COPY --from=w /app/target/release/until /app/until-dyn
# COPY --from=m /app/target/aarch64-unknown-linux-musl/release/until /app/until-arm64