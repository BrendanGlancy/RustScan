pub mod payloads {
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::path::PathBuf;
    use std::{env, u16};

    pub fn read_payloads() {
        let current_dir = env::current_dir().expect("cant find curr dir");
        let mut file_path = PathBuf::from(current_dir);
        file_path.push("nmap-payloads");

        let mut data = String::new();
        let file = File::open(&file_path).expect("File not found.");
        let mut file_reader = BufReader::new(file);
        file_reader
            .read_to_string(&mut data)
            .expect("Unable to read file.");

        for line in data.split("\n") {
            match line.trim_start().chars().next() {
                Some('#') => continue,
                Some('u') => {
                    let rest = &line[4..];
                    let mut parts = rest.split(" ");
                    let ports = parts.next().unwrap();
                    let ports = get_ports_from_line(ports);
                    for port in ports {
                        println!("Port: {}", port);
                    }

                    if let Some(_) = parts.next() {
                        let payload = get_payload_from_line(line);
                        println!("{}", payload);
                    }
                }
                Some(' ') => {
                    let payload = get_payload_from_line(line);
                    println!("{}", payload);
                }
                _ => {}
            }
        }
    }

    pub fn get_payload_from_line(line: &str) -> String {
        let start = line.find("\"").expect("No opening quote found.");
        let end = line.rfind("\"").expect("No closing quote found.");
        println!("start {}", start);

        line[start..end].to_string()
    }

    pub fn get_ports_from_line(ports: &str) -> Vec<u16> {
        let port_segments: Vec<&str> = ports.split(",").collect();
        let mut port_list: Vec<u16> = Vec::new();

        for segment in port_segments {

            if segment.contains("-") {
                let range: Vec<&str> = segment.trim().split("-").collect();
                let start = range[0].parse::<u16>().unwrap();
                let end = range[1].parse::<u16>().unwrap();
                println!("start {} end {}", start, end);

                for port in start..end {
                    port_list.push(port);
                }

            } else if !segment.is_empty() {
                let int: u16 = segment.parse().unwrap();
                port_list.push(int);
            }

        }
        port_list
    }
}
