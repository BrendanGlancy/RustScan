pub mod payloads {
    use std::fs::File;
    use std::io::{BufReader, Read};

    pub fn read_payloads() {
        let mut data = String::new();
        let f = File::open("nmap-payloads").expect("File not found.");
        let mut file_reader = BufReader::new(f);
        file_reader.read_to_string(&mut data).expect("Unable to read file.");
        for line in data.split("\n") {
            if line.trim_start().starts_with("#") {
                continue;
            }
            if line.starts_with("udp") {
                let rest = &line[4..];
                let mut parts = rest.split(" ");
                let ports = parts.next().unwrap();
                println!("UDP: {}", ports);
                if let Some(_) = parts.next() {
                    let payload = get_payload_from_line(line);
                    println!("{}", payload);
                }
            }
            if line.starts_with("  ") {
                let payload = get_payload_from_line(line);
                println!("{}", payload);
            }
        }
    }

    pub fn get_payload_from_line(line: &str) -> String {
        let start = line.find("\"").expect("No opening quote found.");
        let end = line.rfind("\"").expect("No closing quote found.");

        line[start + 1..end].to_string()
    }

    pub fn get_ports_from_line(line: &str) -> String {

    }
}
