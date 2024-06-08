use std::collections::BTreeMap;
use std::fs::File;

use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::{env, i32, u16};

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

    let mut fp_map: BTreeMap<i32, String> = BTreeMap::new();

    let mut count = 0;
    let mut capturing = false;
    let mut curr = String::new();

    for line in data.split("\n") {
        if line.trim().contains("#") || line.is_empty() {
            continue;
        }

        if line.starts_with("udp") {
            if !curr.is_empty() {
                fp_map.insert(count, curr);
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

    let pb_linenr = ports_v(&fp_map);
    let payb_linenr = payloads_v(&fp_map);

    for (line_nr, ports) in pb_linenr {
        println!("{} {:?}", line_nr, ports);
    }

    for (line_nr, payloads) in payb_linenr {
        println!("{} {:?}", line_nr, payloads);
    }
}

fn ports_v(fp_map: &BTreeMap<i32, String>) -> BTreeMap<i32, Vec<u16>> {
    let mut pb_linenr: BTreeMap<i32, Vec<u16>> = BTreeMap::new();
    let mut port_list: Vec<u16> = Vec::new();

    for (&line_nr, ports) in fp_map {
        if ports.contains("udp ") {
            let remain = &ports[4..];
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

        pb_linenr.insert(line_nr, port_list.clone());
        port_list.clear();
    }

    pb_linenr
}

fn payloads_v(fp_map: &BTreeMap<i32, String>) -> BTreeMap<i32, Vec<Vec<String>>> {
    let mut payb_linenr: BTreeMap<i32, Vec<Vec<String>>> = BTreeMap::new();
    let mut ploads: Vec<Vec<String>> = Vec::new();
    let mut sin_pload: Vec<String> = Vec::new();

    for (&line_nr, data) in fp_map {
        if data.contains("\"") {
            let start = data.find("\"").expect("payload opening \" not found");
            let end = data.rfind("\"").expect("payload closing \" not found");

            let payloads = data[start..end].split(" ");
            for payload in payloads {
                sin_pload.push(payload.to_string());
                ploads.push(sin_pload.to_vec());
                sin_pload.clear();
            }
        }

        payb_linenr.insert(line_nr, ploads.to_vec());
        ploads.clear();
    }

    payb_linenr
}
