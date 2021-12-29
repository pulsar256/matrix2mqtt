use commandline_opts::CommandlineOpts;
use log::{debug, error, info, warn};
use matrix_sdk::{
    event_handler::RawEvent,
    room::Room,
    ruma::{
        events::{
            room::message::MessageEventContent, room::message::MessageType::Text, SyncMessageEvent,
        },
        UserId,
    },
    Client, Result, SyncSettings,
};
use paho_mqtt as mqtt;
use paho_mqtt::AsyncClient;
use std::convert::TryFrom;
use std::process;
use std::sync::Arc;
use std::time::Duration;

mod commandline_opts;

#[tokio::main]
async fn main() -> Result<()> {
    let opts: CommandlineOpts = CommandlineOpts::parse_and_setup_logger();
    let matrix_client = create_matrix_client(&opts).await;
    let mqtt_client = Arc::new(create_mqtt_client(
        opts.mqtt_host.as_str(),
        opts.mqtt_username.as_str(),
        opts.mqtt_password.as_str(),
    ));

    matrix_client
        .register_event_handler(
            move |sync_message_event: SyncMessageEvent<MessageEventContent>,
                  room: Room,
                  raw_event: RawEvent| {
                debug!(
                    "Incoming message {:?} on room {:?}, raw event> {:?}",
                    sync_message_event, room, raw_event
                );

                let local_mqtt_client = mqtt_client.clone();
                let sanitizer = |c| !r#"#/+"#.contains(c);

                async move {
                    let room_name = match room.canonical_alias() {
                        None => {
                            let mut room_id = String::from(room.room_id().as_str());
                            warn!("No canonical alias for room {:?} configured.", room_id);
                            room_id.retain(sanitizer);
                            room_id
                        }
                        Some(room_alias_id) => {
                            let mut room_id = String::from(room_alias_id.as_str());
                            room_id.retain(sanitizer);
                            room_id
                        }
                    };

                    local_mqtt_client.publish(
                        mqtt::MessageBuilder::new()
                            .topic(format!("matrix2mqtt/json/{}", room_name))
                            .payload(raw_event.0.get())
                            .retained(false)
                            .qos(0)
                            .finalize(),
                    );

                    let body: Option<String> = match sync_message_event.content.msgtype {
                        Text(ref text) => Some(String::from(text.clone().body)),
                        other => {
                            warn!("Non-Text message: {:?}, ignoring.", other);
                            None
                        }
                    };

                    match body {
                        None => {}
                        Some(body) => {
                            info!("forwarding to '{}' payload: '{}'", room_name, body);
                            local_mqtt_client.publish(
                                mqtt::MessageBuilder::new()
                                    .topic(format!("matrix2mqtt/text/{}", room_name))
                                    .payload(body)
                                    .retained(false)
                                    .qos(0)
                                    .finalize(),
                            );
                        }
                    }
                }
            },
        )
        .await;

    // enters the sync-loop, does not resolve.
    matrix_client.sync(SyncSettings::default()).await;
    Ok(())
}

async fn create_matrix_client(opts: &CommandlineOpts) -> Client {
    if opts.matrix_username.is_empty() || opts.matrix_password.is_empty() {
        error!("Missing matrix credentials.");
        process::exit(1);
    }
    let matrix_user = UserId::try_from(opts.matrix_username.as_str()).unwrap_or_else(|error| {
        error!(
            "Invalid matrix user id, use a fully qualified representation (@user:server):\n{:?}",
            error
        );
        process::exit(1);
    });
    let matrix_client = Client::new_from_user_id(matrix_user.clone())
        .await
        .unwrap_or_else(|error| {
            error!(
                "unable to infer a matrix client from username:\n{:?}",
                error
            );
            process::exit(1);
        });
    matrix_client
        .login(
            matrix_user.localpart(),
            opts.matrix_password.as_str(),
            None,
            None,
        )
        .await
        .unwrap_or_else(|error| {
            error!("could not connect to matrix:\n{:?}", error);
            process::exit(1);
        });
    matrix_client
}

fn create_mqtt_client(host: &str, username: &str, password: &str) -> AsyncClient {
    let mqtt_client = mqtt::AsyncClient::new(host).unwrap_or_else(|err| {
        error!("Error creating the mqtt client: {:?}", err);
        process::exit(1);
    });

    let mut builder = mqtt::ConnectOptionsBuilder::new();
    builder
        .keep_alive_interval(Duration::from_secs(20))
        .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(60))
        .clean_session(true);

    if !username.is_empty() {
        builder.user_name(username).password(password);
    }

    mqtt_client
        .connect(builder.finalize())
        .wait_for(Duration::from_secs(10))
        .unwrap_or_else(|err| {
            error!("could not connect to mqtt: {:?}", err);
            process::exit(1);
        });
    mqtt_client
}
