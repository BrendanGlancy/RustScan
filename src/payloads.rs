pub mod payloads {
    use std::collections::{HashMap, HashSet};
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

        tokenize_file(file_buf, data);
    }

    fn tokenize_file(mut file_buf: BufReader<File>, mut data: String) {
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
                blocks.push(mem::take(&mut block));
            }

            block.push(line.trim().to_str());
        }

        if !block.is_empty() {
            blocks.push(block);
        }

        init_port_map(&blocks);
    }

    fn init_port_map(blocks: &Vec<Vec<String>>) -> HashMap<u16, Vec<Vec<u8>>> {
        let mut port_payload: HashMap<u16, Vec<Vec<u8>>> = HashMap::new();

        let mut payloads_fb = parse_payloads(blocks);

        for block in blocks {
            let ports_fb = parse_ports(block.to_vec());

            for port in ports_fb {
                port_payload.insert(port, mem::take(&mut payloads_fb));
            }
        }

        port_payload
    }

    // get the payload based on the current port
    fn parse_payloads(blocks: &Vec<Vec<String>>) -> Vec<Vec<u8>> {
        let mut payloads: Vec<Vec<u8>> = vec![];

        for block in blocks {
            for line in block {
                if line.contains("\"") {
                    let start = line.find("\"").expect("No opening quote found.");
                    let end = line.rfind("\"").expect("No closing quote found.");

                    let payload_str = &line[start + 1..end];
                    let payload_bytes: Vec<u8> = payload_str
                        .split("\\x")
                        .filter(|s| !s.is_empty())
                        .map(|s| u8::from_str_radix(s, 16).expect("Invalid hex byte"))
                        .collect();

                    payloads.push(payload_bytes);
                }
            }
        }

        payloads
    }

    fn parse_ports(block: Vec<String>) -> HashSet<u16> {
        let mut port_list: HashSet<u16> = HashSet::new();

        for line in block {
            if line.contains("udp ") {
                let remander = &line[4..];
                let mut parts = remander.split(" ");

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
        }

        port_list
    }
}
