FROM inkbox/rust as w
#FROM ghcr.io/inkbox/rust as w
WORKDIR /app
COPY . /app

RUN cargo build --release && RUSTFLAGS="-C target-feature=+crt-static" cargo build --release --target x86_64-pc-windows-gnu

FROM ghcr.io/inkroomtemp/rust_musl_build as m
#FROM registry.gitlab.com/rust_musl_docker/image:stable-latest as m
WORKDIR /app
COPY . /app
RUN cargo build --release -vv --target=x86_64-unknown-linux-musl


FROM scratch
#FROM alpine
COPY --from=w /app/target/x86_64-pc-windows-gnu/release/until.exe /app/until.exe
COPY --from=m /app/target/x86_64-unknown-linux-musl/release/until /app/until
COPY --from=w /app/target/release/until /app/until-dyn


