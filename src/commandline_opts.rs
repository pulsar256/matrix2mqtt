use std::{env};
use clap::{Parser};

#[derive(Parser, Clone)]
#[clap(version = "0.2.3",
author = "Paul Rogalinski-Pinter, matrix2mqtt@t00ltime.de",
about = "forwards messages from matrix to mqtt")]
pub struct CommandlineOpts {
  #[clap(long, default_value = "tcp://mqtt.localdomain:1883", env = "MQTT_HOST")]
  pub mqtt_host: String,

  #[clap(long, help = "If left empty, no authentication will be used", default_value = "", env = "MQTT_USERNAME")]
  pub mqtt_username: String,

  #[clap(long, default_value = "", env = "MQTT_PASSWORD")]
  pub mqtt_password: String,

  #[clap(long, default_value = "", env = "MATRIX_USERNAME")]
  pub matrix_username: String,

  #[clap(long, default_value = "", env = "MATRIX_PASSWORD")]
  pub matrix_password: String,

  #[clap(short, help = "verbose output if not specified otherwise by the RUST_LOG environment variable.", env = "DEBUG")]
  pub verbose: bool,
}

impl CommandlineOpts {
  pub fn setup_logger(&self) {
    if self.verbose {
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
}
