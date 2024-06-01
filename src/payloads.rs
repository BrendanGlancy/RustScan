pub mod payloads {
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::path::PathBuf;
    use std::{env, u16};

    pub fn parse() {
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
                    let ports = get_ports(ports);

                    for port in ports {
                        println!("Port: {}", port);
                    }

                    if let Some(_) = parts.next() {
                        let payload = get_payload(line);
                        println!("{}", payload);
                    }
                }
                Some(' ') => {
                    let payload = get_payload(line);
                    println!("{}", payload);
                }
                _ => {}
            }
        }
    }

    fn get_payload(line: &str) -> String {
        let start = line.find("\"").expect("No opening quote found.");
        let end = line.rfind("\"").expect("No closing quote found.");

        line[start..end].to_string()
    }

    fn get_ports(ports: &str) -> HashSet<u16> {
        let port_segments: Vec<&str> = ports.split(",").collect();
        let mut port_list: HashSet<u16> = HashSet::new();

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
        port_list
    }

}
