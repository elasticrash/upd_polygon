extern crate udp_polygon;
use serde_derive::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};
use std::{thread, time};
use udp_polygon::{config::Config, config::FromArguments, Polygon};

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub id: u32,
    pub msg: String,
}

fn main() {
    let config = Config::from_arguments(
        vec![(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5061)],
        Some((IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5060)),
    );

    let mut polygon = Polygon::configure(config);

    loop {
        println!("sending");
        polygon.send(
            serde_json::to_string(&Message {
                id: 1,
                msg: String::from("Hello"),
            })
            .unwrap(),
        );
        thread::sleep(time::Duration::from_secs(2));
    }
}
