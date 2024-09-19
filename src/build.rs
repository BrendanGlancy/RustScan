use std::collections::BTreeMap;
use std::fs::{self, File};

use std::{env, u8};
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::process::Command;

struct service_probe {
    service: str,
    protocol: str,
    rarity: u8,
    payload: Vec::new(),
}

// Reads in a file with payloads based on port
pub fn file_reader() {
    let mut file_path = env::current_dir().expect("cant find curr dir");
    file_path.push("./nmap-services");

    let mut data = String::new();
    let file = File::open(&file_path).expect("File not found.");
    let mut file_buf = BufReader::new(file);
    file_buf
        .read_to_string(&mut data)
        .expect("unable to read file");

    for element in data.trim().split("\nProbe ") {
        let rarity = 0;

        let is_udp =

        for line in element.split('\n') {
            if line.starts_with("#") {
                continue;
            }
        }
    }
}
