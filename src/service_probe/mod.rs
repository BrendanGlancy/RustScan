pub mod payloads {
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

            println!("ports {:?}", ports);
            println!("combos {:?}", combo);
            println!("payloads {:?}", payloads);
        }
    }

    fn port_map(blocks: &Vec<Vec<String>>) -> HashMap<u16, Vec<String>> {
        let mut port_payload: HashMap<u16, Vec<String>> = HashMap::new();

        for block in blocks {
            let ports_fb = get_ports(block.to_vec());
            let payloads_fb = get_payloads(block.to_vec());

            for port in ports_fb {
                port_payload.insert(port, payloads_fb.clone());
            }
        }

        port_payload
    }

    // get the payload based on the current port
    fn get_payloads(block: Vec<String>) -> Vec<String> {
        let mut payloads: Vec<String> = vec![];

        for line in block {
            // println!("line in block: {}", line);
            if line.contains("\"") {
                let start = line.find("\"").expect("No opening quote found.");
                let end = line.rfind("\"").expect("No closing quote found.");

                payloads.push(line[start + 1..end].to_string());
            }
        }

        payloads
    }

    fn get_ports(block: Vec<String>) -> Vec<u16> {
        let mut port_list: Vec<u16> = Vec::new();

        for line in block {
            if line.contains("udp ") {
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
}
