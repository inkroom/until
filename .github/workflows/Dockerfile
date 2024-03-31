FROM inkbox/rust:1.74.1 as w
#FROM ghcr.io/inkbox/rust as w
WORKDIR /app
COPY . /app

RUN mkdir out && if [ "$(arch)" = "x86_64" ]; then RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-pc-windows-gnu --config source.crates-io.replace-with=\"mirror\" --config source.mirror.registry=\"sparse+https://mirrors.ustc.edu.cn/crates.io-index/\" && mv /app/target/x86_64-pc-windows-gnu/release/until.exe out ; fi

FROM ghcr.m.daocloud.io/inkroomtemp/rust_musl_build:1.74.1 as m
#FROM registry.gitlab.com/rust_musl_docker/image:stable-latest as m
WORKDIR /app
RUN rustup target add $(arch)-unknown-linux-musl
COPY . /app
RUN cargo build --release --config source.crates-io.replace-with=\"mirror\" \
    --config source.mirror.registry=\"sparse+https://mirrors.ustc.edu.cn/crates.io-index/\" \
    && cargo build --release --config source.crates-io.replace-with=\"mirror\" \
    --config source.mirror.registry=\"sparse+https://mirrors.ustc.edu.cn/crates.io-index/\" -vv \
    --target=$(arch)-unknown-linux-musl \
    && mkdir out \
    && mv target/release/until ./out/until-dyn \
    && mv target/$(arch)-unknown-linux-musl/release/until ./out/until


FROM scratch
#FROM alpine
COPY --from=w /app/out/ /app/
COPY --from=m /app/out/ /app/
# COPY --from=m /app/target/aarch64-unknown-linux-musl/release/until /app/until-arm64