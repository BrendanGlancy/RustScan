use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;

use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::{env, i32, mem, u16, u8};

pub fn file_to_map() {
    let current_dir = env::current_dir().expect("cant find curr dir");
    let mut file_path = PathBuf::from(current_dir);
    file_path.push("nmap-payloads");

    let data = String::new();
    let file = File::open(&file_path).expect("File not found.");
    let file_buf = BufReader::new(file);

    f_btree(file_buf, data);
}

fn f_btree(mut file_buf: BufReader<File>, mut data: String) {
    file_buf
        .read_to_string(&mut data)
        .expect("unable to read file");

    let mut file_map: BTreeMap<i32, String> = BTreeMap::new();
    let mut count = 0;
    let mut capturing = false;
    let mut curr = String::new();

    for line in data.split("\n") {
        if line.trim().contains("#") || line.is_empty() {
            continue;
        }

        if line.starts_with("udp") {
            if !curr.is_empty() {
                file_map.insert(count, curr);
                curr = String::new();
            }
            capturing = true;
            count += 1;
        }

        if capturing {
            if !curr.is_empty() {
                curr.push(' ');
            }
            curr.push_str(line);
        }
    }

    for (line_nr, data) in file_map {
        println!("{} {}", line_nr, data);
    }
}

fn ports_v(ports: &Vec<String>) -> Vec<u16> {
    let mut port_list: Vec<u16> = Vec::new();

    for idx in ports {
        // we already check this
        if idx.contains("udp ") {
            let remain = &idx[4..];
            let mut start = remain.split(" ");

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

fn payloads_v(payloadp: &Vec<String>) -> Vec<u8> {
    let mut payloads: Vec<u8> = Vec::new();

    for payload in payloadp {
        payloads = payload.as_bytes().to_vec();
    }

    payloads
}

fn init_map(combos: &Vec<String>) -> HashMap<Vec<u16>, Vec<u8>> {
    let mut map: HashMap<Vec<u16>, Vec<u8>> = HashMap::new();

    let port_v = ports_v(&combos.to_vec());
    let payload_v = payloads_v(&combos.to_vec());
    map.insert(port_v, payload_v);

    map
}
