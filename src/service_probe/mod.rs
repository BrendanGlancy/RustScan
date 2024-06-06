use std::collections::HashMap;
use std::fs::File;

use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::{env, mem, u16, u8};

use colorful::core::StrMarker;

pub fn file_to_map() {
    let current_dir = env::current_dir().expect("cant find curr dir");
    let mut file_path = PathBuf::from(current_dir);
    file_path.push("nmap-payloads");

    let data = String::new();
    let file = File::open(&file_path).expect("File not found.");
    let file_buf = BufReader::new(file);

    get_block(file_buf, data);
}

fn get_block(mut file_buf: BufReader<File>, mut data: String) {
    file_buf
        .read_to_string(&mut data)
        .expect("unable to read file");

    let mut ports: Vec<String> = Vec::new();
    let mut payloads: Vec<String> = Vec::new();
    let mut combo: Vec<String> = Vec::new();

    for line in data.split("\n") {
        if line.trim().starts_with("#") {
            continue;
        }

        if line.trim().contains("udp") && !line.contains("\"") {
            ports.push(line.trim().to_str());
        }

        if line.trim().contains("udp") && line.contains("\"") {
            combo.push(line.trim().to_str());
        }

        if line.contains("\"") && !line.contains("udp") {
            payloads.push(line.trim().to_str());
        }
    }

    ports_v(&ports);
    ports_v(&combo);
}

fn ports_v(ports: &Vec<String>) -> Vec<u16> {
    let mut port_list: Vec<u16> = Vec::new();

    for idx in ports {
        // we already check this
        if idx.contains("udp ") {
            let remain = &idx[4..];
            let mut start = remain.split(" ");
            println!("start of ports {:?}", start);

            let ports = start.next().unwrap();
            let port_segments: Vec<&str> = ports.split(",").collect();

            for segment in port_segments {
                if segment.contains("-") {
                    let range: Vec<&str> = segment.trim().split("-").collect();
                    let start = range[0].parse::<u16>().unwrap();
                    let end = range[1].parse::<u16>().unwrap();

                    for port in start..end {
                        port_list.push(port);
                    }
                } else if !segment.is_empty() {
                    let port: u16 = segment.parse().unwrap();
                    port_list.push(port);
                }
            }
        }
    }

    port_list
}

fn payloads(payloadp: Vec<String>) -> Vec<String> {
    let payload_v: Vec<String> = Vec::new();
    payload_v
}

fn combo_map(combos: &Vec<String>) -> HashMap<Vec<u16>, Vec<String>> {
    let mut map: HashMap<Vec<u16>, Vec<String>> = HashMap::new();

    let port_v = ports_v(&combos.to_vec());
    let payload_v = payloads(combos.to_vec());
    map.insert(port_v, payload_v);

    map
}
