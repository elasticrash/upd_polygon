use serde_derive::Deserialize;
use std::env;
use std::fs;
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub address: Vec<(IpAddr, u16)>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            address: vec![(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 5060)],
        }
    }
}

pub trait FromToml {
    fn from_toml(filename: String) -> Self;
}

pub trait FromArguments {
    fn from_arguments(addresses: Vec<(IpAddr, u16)>) -> Self;
}

pub trait FromDefault {
    fn from_default() -> Self;
}

pub trait FromEnv {
    fn from_env(envvar: String) -> Self;
}

impl FromToml for Config {
    fn from_toml(filename: String) -> Self {
        let contents = fs::read_to_string(filename);

        match contents {
            Ok(cf) => toml::from_str(&cf).unwrap(),
            Err(_) => Config::default(),
        }
    }
}

impl FromArguments for Config {
    fn from_arguments(addresses: Vec<(IpAddr, u16)>) -> Self {
        Config { address: addresses }
    }
}

impl FromDefault for Config {
    fn from_default() -> Self {
        Config::default()
    }
}

/// FromEnv only supports a single address/port pair.
/// This is because I wanted to avoid having to name
/// the enviroment variables, you can pass your own
/// instead
impl FromEnv for Config {
    fn from_env(envvar: String) -> Self {
        let address = match env::var(envvar).unwrap().parse::<IpAddr>() {
            Ok(addr) => addr,
            Err(err) => panic!("{}", err),
        };

        let port = match env::var("PORT").unwrap().parse::<u16>() {
            Ok(port) => port,
            Err(err) => panic!("{}", err),
        };

        Config {
            address: vec![(address, port)],
        }
    }
}
