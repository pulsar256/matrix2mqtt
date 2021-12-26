use paho_mqtt as mqtt;
use std::{env, process};
use std::time::Duration;
use log::{debug, error, info, warn};
use clap::{Parser};
use paho_mqtt::AsyncClient;

use std::convert::TryFrom;
use matrix_sdk::{
  Client, SyncSettings, Result,
  ruma::{UserId, events::{SyncMessageEvent, room::message::MessageEventContent}},
};
use matrix_sdk::room::Room;
use matrix_sdk::ruma::events::room::message::MessageType;
use matrix_sdk::ruma::events::room::message::MessageType::Text;

extern crate clap;

#[derive(Parser, Clone)]
#[clap(version = "1.0-beta",
author = "Paul Rogalinski-Pinter, matrix2mqtt@t00ltime.de",
about = "forwards messages from matrix to mqtt")]
struct Opts {
  #[clap(long, default_value = "tcp://mqtt.localdomain:1883", env = "MQTT_HOST")]
  mqtt_host: String,

  #[clap(long, help = "If left empty, no authentication will be used", default_value = "", env = "MQTT_USERNAME")]
  mqtt_username: String,

  #[clap(long, default_value = "", env = "MQTT_PASSWORD")]
  mqtt_password: String,

  #[clap(long, default_value = "", env = "MATRIX_USERNAME")]
  matrix_username: String,

  #[clap(long, default_value = "", env = "MATRIX_PASSWORD")]
  matrix_password: String,

  #[clap(short, help = "verbose output if not specified otherwise by the RUST_LOG environment variable.", env = "DEBUG")]
  verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
  let opts: Opts = Opts::parse();
  let matrix_user = UserId::try_from(opts.matrix_username.as_str())?;
  let matrix_client = Client::new_from_user_id(matrix_user.clone()).await?;
  let mqtt_client = Box::new(connect_mqtt(opts.mqtt_host.as_str(), opts.mqtt_username.as_str(), opts.mqtt_password.as_str()));
  setup_logger(&opts);
  matrix_client.login(matrix_user.localpart(), opts.matrix_password.as_str(), None, None).await?;


  matrix_client
    .register_event_handler(
      move |ev: SyncMessageEvent<MessageEventContent>,
            room: Room|
        {
          let client = mqtt_client.clone();
          // let sanitizer = |x| {!['#', '/'].contains(&x)};
          let sanitizer = |c| !r#"#/+"#.contains(c);
          async move {
            let room_name = match room.canonical_alias() {
              None => {
                let mut room_id = String::from(room.room_id().as_str());
                room_id.retain(sanitizer);
                warn!("No canonical alias for room {:?} configured.",room_id);
                room_id
              }
              Some(room_alias_id) => {
                let mut room_id = String::from(room_alias_id.as_str());
                room_id.retain(sanitizer);
                room_id
              }
            };

            let body: Option<String> = match ev.content.msgtype {
              Text(ref text) => { Some(String::from(text.clone().body)) }
              other => {
                warn!("Non-Text message: {:?}, ignoring.",other);
                None
              }
            };

            match body {
              None => {}
              Some(body) => {
                info!("forwarding to '{}' payload: '{}'", room_name, body);
                client.publish(
                  mqtt::MessageBuilder::new()
                    .topic(format!("matrix2mqtt/{}", room_name))
                    .payload(body)
                    .retained(false)
                    .qos(0)
                    .finalize());
              }
            }
          }
        }
    )
    .await;

  // Syncing is important to synchronize the client state with the server.
  // This method will never return.
  matrix_client.sync(SyncSettings::default()).await;

  Ok(())
}

fn setup_logger(opts: &Opts) {
  if opts.verbose {
    if env::var("RUST_LOG").is_err() {
      env::set_var("RUST_LOG", "debug")
    }
  } else {
    if env::var("RUST_LOG").is_err() {
      env::set_var("RUST_LOG", "info")
    }
  }
  env_logger::init();
}

fn connect_mqtt(host: &str, username: &str, password: &str) -> AsyncClient {
  info!("connecting to mqtt {}",host);
  let cli = mqtt::AsyncClient::new(host).unwrap_or_else(|err| {
    error!("Error creating the client: {:?}", err);
    process::exit(1);
  });

  let conn_opts = mqtt::ConnectOptionsBuilder::new()
    .keep_alive_interval(Duration::from_secs(20))
    .automatic_reconnect(Duration::from_secs(1), Duration::from_secs(60))
    .clean_session(true)
    .user_name(username).password(password).finalize();
  cli.connect(conn_opts).wait_for(Duration::from_secs(10)).unwrap_or_else(|err| {
    error!("could not connect to mqtt: {:?}", err);
    process::exit(1);
  });

  cli
}
