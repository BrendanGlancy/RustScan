// const MAX_PAYLOAD_IPV4: usize = 65507; // 65,535 - 20 (IP header) - 8 (UDP header)
// const MAX_PAYLOAD_IPV6: usize = 65527; // 65,535 - 40 (IPv6 header) - 8 (UDP header)

/// From RFC 768
/// We send blank UDP packets, unless the port is determined to be special.
///
/// ```text
/// HEADER Handled by the OS
/// 0                   1                   2                   3
/// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |          Source Port          |       Destination Port        |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |            Length             |           Checksum            |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// This is create for us via UDPSocket::bind
///
/// DATA
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// |                    Data (based on port)                       |
/// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
/// ```
///
/// # Fields of a UDP Packet
///
/// - **Source Port**: The source port number.
/// - **Destination Port**: The destination port number.
/// - **Length**: The length of the UDP header and data.
/// - **Checksum**: The checksum for error-checking.
///
/// - **Data**: The payload data, which can vary based on the port.
pub fn custom_payload(dst_prt: u16) -> Vec<u8> {
    match dst_prt {
        _ => vec![],
    }
}
