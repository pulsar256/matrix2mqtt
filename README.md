# Matrix2MQTT

Matrix to MQTT message forwarder

## About

Connects the matrix network based on provided credentials, and forwards all text messages from all channels the user has joined to the configured mqtt server. Topic naming is derived from the room name:

```text
matrix2mqtt/{$type}/${matrix_room_name}
```

`${type}` is either `text` or `json`. Payload is respectively either the extracted text value from the chat message or the whole, unprocessed json payload. If the event type is `text`, both subtopics will be populated, otherwise only the `json` subtopic will be used.

`${matrix_room_name}` is sanitized (#, / and + are stripped). If available, the cannonical room alias is used instead of the internal matrix room id.

## Usage:

### Docker

```shell

docker run --rm \
  -e MATRIX_PASSWORD=muchsecritverysecure \
  -e MATRIX_USERNAME=@arthur_dent:earth.milkyway.euclid \
  -e MQTT_HOST=mqtt  \
  pulsar256/matrix2mqtt
```

### Standalone
```text
> matrix2mqtt [OPTIONS]

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

```shell
docker build . -t whatever
```

or use the `docker_image` Make target.

```
❯ make help
build                          builds the binary (./target/release/ws-to-mqtt)
docker_image                   builds a docker container
push_docker                    builds and and pushes the docker image
help                           print help
```

### Local Builds

```
cargo build --release
```
