FROM docker.io/rust:latest as builder
RUN apt-get update && apt-get install cmake openssl -y
WORKDIR /src
ADD Cargo.toml ./Cargo.toml
ADD Cargo.lock ./Cargo.lock
ADD src ./src/
RUN cargo build --release

FROM docker.io/rust:slim as runtime
LABEL description="matrix2mqtt, a Matrix to MQTT message forwarder." \
      url="https://github.com/pulsar256/matrix2mqtt"
WORKDIR /opt/matrix2mqtt
RUN apt update && apt install -y libssl1.1 ca-certificates
COPY --from=builder /src/target/release/matrix2mqtt .
ENTRYPOINT ["/opt/matrix2mqtt/matrix2mqtt"]
