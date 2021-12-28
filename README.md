# Matrix2MQTT

Connects the matrix network based on provided credentials, and forwards all text messages from all channels the user has joined to the configured mqtt server. Topic naming is derived from the room name:

```text
matrix2mqtt/{$type}/${matrix_room_name}
```

`${type}` is either `text` or `json`. Payload is respectively either the extracted text value from the chat message or the whole, unprocessed json payload. If the event type is `text`, both topics will be populated, otherwise json will be used.

`${matrix_room_name}` is sanitized (#, / and + are stripped). If possible, the cannonical room alias is used instead of the internal matrix room id.

## Usage:

```text
matrix2mqtt 1.0-beta
Paul Rogalinski-Pinter, matrix2mqtt@t00ltime.de
forwards messages from matrix to mqtt

USAGE:
    matrix2mqtt [OPTIONS]

OPTIONS:
    -h, --help                                 Print help information
        --matrix-password <MATRIX_PASSWORD>    [env: MATRIX_PASSWORD=] [default: ]
        --matrix-username <MATRIX_USERNAME>    [env: MATRIX_USERNAME=] [default: ]
        --mqtt-host <MQTT_HOST>                [env: MQTT_HOST=] [default: tcp://mqtt.localdomain:1883]
        --mqtt-password <MQTT_PASSWORD>        [env: MQTT_PASSWORD=] [default: ]
        --mqtt-username <MQTT_USERNAME>        If left empty, no authentication will be used [env: MQTT_USERNAME=] [default: ]
    -v                                         verbose output if not specified otherwise by the RUST_LOG environment variable. [env: DEBUG=]
    -V, --version                              Print version information
```

## Building

### Docker

```
❯ make help
build                          builds the binary (./target/release/ws-to-mqtt)
docker_image                   builds a docker container
push_docker                    builds and and pushes the docker image
push_multiarch_docker          experimental, builds and publishes docker images for i386, amd64 and arm64. prepare buildx environment using `docker buildx create --use`
help                           print help
```

### Local Builds

```
cargo build --release
```
