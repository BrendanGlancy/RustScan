use std::collections::BTreeMap;
use std::fs::{self, File};

use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::process::Command;
use std::{env, u8};

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
        for line in element.split('\n') {
            if line.starts_with("#") {
                continue;
            }

            if line.starts_with("ports") {}

            if line.starts_with("sslports") {}

            if line.starts_with("Probe") {}
        }
    }
}

/// Generates a file called Generated.rs and calls cargo fmt from the command line
///
/// # Arguments
///
/// * `port_payload_map` - A BTreeMap mapping port numbers to payload data
fn generate_code(port_payload_map: BTreeMap<Vec<u16>, Vec<u8>>) {
    let dest_path = PathBuf::from("src/generated.rs");

    let mut generated_code = String::new();
    generated_code.push_str("use std::collections::BTreeMap;\n");
    generated_code.push_str("use once_cell::sync::Lazy;\n\n");

    generated_code.push_str("fn generated_data() -> BTreeMap<Vec<u16>, Vec<u8>> {\n");
    generated_code.push_str("    let mut map = BTreeMap::new();\n");

    for (ports, payloads) in port_payload_map {
        generated_code.push_str("    map.insert(vec![");
        generated_code.push_str(
            &ports
                .iter()
                .map(|&p| p.to_string())
                .collect::<Vec<_>>()
                .join(","),
        );
        generated_code.push_str("], vec![");
        generated_code.push_str(
            &payloads
                .iter()
                .map(|&p| p.to_string())
                .collect::<Vec<_>>()
                .join(","),
        );
        generated_code.push_str("]);\n");
    }

    generated_code.push_str("    map\n");
    generated_code.push_str("}\n\n");

    generated_code.push_str(
        "static PARSED_DATA: Lazy<BTreeMap<Vec<u16>, Vec<u8>>> = Lazy::new(generated_data);\n",
    );
    generated_code.push_str("pub fn get_parsed_data() -> &'static BTreeMap<Vec<u16>, Vec<u8>> {\n");
    generated_code.push_str("    &PARSED_DATA\n");
    generated_code.push_str("}\n");

    fs::write(dest_path, generated_code).unwrap();

    // format the generated code
    Command::new("cargo")
        .arg("fmt")
        .arg("--all")
        .output()
        .expect("Failed to execute cargo fmt");
}

/// Creates a BTreeMap of line numbers mapped to a Vec<u16> of ports
///
/// # Arguments
///
/// * `fp_map` - A BTreeMap containing the parsed file data
///
/// # Returns
///
/// A BTreeMap where keys are line numbers and values are vectors of ports
fn ports_v(ports: String) -> Vec<u16> {
    let mut ports_vec: Vec<u16> = Vec::new();

    let remain = &ports[4..];
    let mut start = remain.split(' ');

    let ports = start.next().unwrap();
    let port_segments: Vec<&str> = ports.split(',').collect();

    for segment in port_segments {
        if segment.contains('-') {
            let range: Vec<&str> = segment.trim().split('-').collect();
            let start = range[0].parse::<u16>().unwrap();
            let end = range[1].parse::<u16>().unwrap();

            for port in start..end {
                ports_vec.push(port);
            }
        } else if !segment.is_empty() {
            match segment.parse::<u16>() {
                Ok(port) => ports_vec.push(port),
                Err(_) => println!("Error parsing port: {}", segment),
            }
        }
    }

    ports_vec
}

/// Converts a hexadecimal string to a Vec<u8>
///
/// # Arguments
///
/// * `payload` - A string slice containing the hexadecimal payload
///
/// # Returns
///
/// A vector of bytes representing the decoded payload
fn parser(payload: &str) -> Vec<u8> {
    let payload = payload.trim_matches('"');
    let mut tmp_str = String::new();
    let mut bytes: Vec<u8> = Vec::new();

    for (idx, char) in payload.chars().enumerate() {
        if char == '\\' && payload.chars().nth(idx + 1) == Some('x') {
            continue;
        } else if char.is_ascii_digit() {
            tmp_str.push(char);
            if tmp_str.len() == 2 {
                bytes.push(u8::from_str_radix(&tmp_str, 16).unwrap());
                tmp_str.clear();
            }
        }
    }

    bytes
}

/// Combines the ports BTreeMap and the Payloads BTreeMap
///
/// # Arguments
///
/// * `pb_linenr` - A BTreeMap mapping line numbers to vectors of ports
/// * `payb_linenr` - A BTreeMap mapping line numbers to vectors of payload bytes
///
/// # Returns
///
/// A BTreeMap mapping vectors of ports to vectors of payload bytes
fn port_payload_map(pb_linenr: Vec<u16>, payb_linenr: Vec<u8>) -> BTreeMap<Vec<u16>, Vec<u8>> {
    let mut ppm_fin: BTreeMap<Vec<u16>, Vec<u8>> = BTreeMap::new();

    for (port_linenr, ports) in pb_linenr {
        for (pay_linenr, payloads) in &payb_linenr {
            if pay_linenr == &port_linenr {
                ppm_fin.insert(ports.to_vec(), payloads.to_vec());
            }
        }
    }

    ppm_fin
}
