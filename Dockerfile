FROM docker.io/rust:latest as builder
RUN apt-get update && apt-get install cmake openssl -y
WORKDIR /src
ADD Cargo.toml ./Cargo.toml
ADD Cargo.lock ./Cargo.lock
ADD src ./src/
#this can potentially fix the armv7 builds.
#RUN --security=insecure mkdir -p /root/.cargo && chmod 777 /root/.cargo && mount -t RUN --security=insecure mkdir -p /root/.cargo && chmod 777 /root/.cargo && mount -t tmpfs none /root/.cargo && cargo build --release
RUN cargo build --release

FROM docker.io/rust:slim as runtime
WORKDIR /opt/matrix2mqtt
RUN apt update && apt install -y libssl1.1 ca-certificates
COPY --from=builder /src/target/release/matrix2mqtt .
ENTRYPOINT ["/opt/matrix2mqtt/matrix2mqtt"]
