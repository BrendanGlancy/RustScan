pub mod payloads {
    use std::collections::{HashMap, HashSet};
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::path::PathBuf;
    use std::{env, u16, u8};

    use colorful::core::StrMarker;

    use crate::payloads;

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

        let mut blocks: Vec<Vec<String>> = Vec::new();
        let mut block: Vec<String> = Vec::new();

        for line in data.split("\n") {
            if line.trim().starts_with("#") {
                continue;
            }

            if line.trim().starts_with("udp") && !block.is_empty() {
                blocks.push(block.clone());
            }

            block.push(line.trim().to_str());
        }

        if !block.is_empty() {
            blocks.push(block);
        }

        let port_payload = port_map(&blocks);
        for (port, payloads) in port_payload {
            println!("Port: {}", port);
            for payload in payloads {
                println!("payload: {:?}", payload);
            }
        }


    }

    fn port_map(blocks: &Vec<Vec<String>>) -> HashMap<u16, Vec<Vec<u8>>> {
        let mut port_payload: HashMap<u16, Vec<Vec<u8>>> = HashMap::new();

        let payloads_fb = get_payloads(&blocks);

        for block in blocks {
            let ports_fb = get_ports(block.to_vec());

            for port in ports_fb {
                port_payload.insert(port, payloads_fb.to_vec());
            }
        }

        port_payload
    }

    // get the payload based on the current port
    fn get_payloads(blocks: &Vec<Vec<String>>) -> Vec<Vec<u8>> {
        let mut payloads: Vec<Vec<u8>> = vec![];

        for block in blocks {
            for line in block {
                let start = line.find("\"").expect("No opening quote found.");
                let end = line.rfind("\"").expect("No closing quote found.");

                payloads.push(line[start..end].to_string().into());
            }
        }

        payloads
    }

    fn get_ports(block: Vec<String>) -> HashSet<u16> {
        let mut port_list: HashSet<u16> = HashSet::new();

        for line in block {
            let rest = &line[4..];
            let mut parts = rest.split(" ");

            let ports = parts.next().unwrap();
            let port_segments: Vec<&str> = ports.split(",").collect();

            for segment in port_segments {
                if segment.contains("-") {
                    let range: Vec<&str> = segment.trim().split("-").collect();
                    let start = range[0].parse::<u16>().unwrap();
                    let end = range[1].parse::<u16>().unwrap();

                    for port in start..end {
                        port_list.insert(port);
                    }
                } else if !segment.is_empty() {
                    let port: u16 = segment.parse().unwrap();
                    port_list.insert(port);
                }
            }
        }

        port_list
    }
}
