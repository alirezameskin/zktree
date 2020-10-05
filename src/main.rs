use std::time::Duration;

use clap::{App, Arg};
use zookeeper::{Stat, WatchedEvent, Watcher, ZooKeeper};

struct LoggingWatcher;

impl Watcher for LoggingWatcher {
    fn handle(&self, _: WatchedEvent) {
        unimplemented!()
    }
}

fn main() {
    let matches = App::new("zootree")
        .version("0.0.1")
        .about("Display znodes tree of zookeeper")
        .arg(
            Arg::with_name("server")
                .long("server")
                .short("s")
                .help("Server info (server:port)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("timeout")
                .long("timeout")
                .short("t")
                .help("Timeout in seconds")
                .takes_value(true),
        )
        .get_matches();

    let server = matches.value_of("server").unwrap_or("localhost:2181");

    let timeout: u64 = match matches.value_of("timeout") {
        None => 2,
        Some(s) => match s.parse::<u64>() {
            Ok(n) => n,
            Err(_) => panic!("Timeout should be a number! {}", s),
        },
    };

    let zookeeper =
        ZooKeeper::connect(server, Duration::from_secs(timeout), LoggingWatcher).unwrap();

    println!("/");
    walk(&zookeeper, "/", 0);
}

fn walk(client: &ZooKeeper, path: &str, level: u64) {
    let result = client.get_children(path, false);

    if let Ok(children) = result {
        for name in children {
            let path = if level == 0 {
                format!("{}{}", path, name)
            } else {
                format!("{}/{}", path, name)
            };

            let stat = client.get_acl(&path).map(|i| i.1).ok();

            display(&path, &stat);
            walk(client, &path, level + 1);
        }
    }
}

fn display(path: &String, stat: &Option<Stat>) {
    let parts = path.split("/").collect::<Vec<&str>>();
    let len = parts.len();
    let space = std::iter::repeat("    |").take(len - 2).collect::<String>();
    let data_size = stat.as_ref().map(|s| s.data_length).unwrap_or(0);

    println!(
        "|{}----/{} ({} bytes)",
        space,
        parts.last().unwrap_or(&""),
        data_size
    )
}
