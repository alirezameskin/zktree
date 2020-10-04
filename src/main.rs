use std::fmt;
use std::time::Duration;

use clap::{App, Arg};
use zookeeper::{Stat, WatchedEvent, Watcher, ZkError, ZooKeeper};

struct LoggingWatcher;

struct ZNode(String, Result<Stat, ZkError>);

impl Watcher for LoggingWatcher {
    fn handle(&self, _: WatchedEvent) {
        unimplemented!()
    }
}

impl fmt::Display for ZNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let parts = self.0.split("/").collect::<Vec<&str>>();
        let len = parts.len();
        let space = std::iter::repeat("    |").take(len - 2).collect::<String>();
        let data_size = self.1.as_ref().ok().map(|s| s.data_length).unwrap_or(0);

        write!(
            f,
            "|{}----/{} ({} bytes)",
            space,
            parts.last().unwrap_or(&""),
            data_size
        )
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

    match result {
        Err(_) => {}
        Ok(children) => {
            for name in children {
                let next_path: String = if level == 0 {
                    path.to_owned() + &*name
                } else {
                    path.to_owned() + "/" + &*name
                };

                let stat = client.get_data(&*next_path, false).map(|i| i.1);

                let znode = ZNode(next_path.clone(), stat);
                println!("{}", znode);

                walk(client, &*next_path, level + 1);
            }
        }
    }
}
