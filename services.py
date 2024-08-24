from argparse import ArgumentParser
from typing import Iterator, List, Generator, Tuple, Optional


def read_nmap_payloads(path: str):
    """
    Read the nmap-payloads file and extract the payloads for each port.

    :param path: The path to the build.rs file to write the extracted payloads to.
    """

    # Create the generator to write the build.rs file
    output_gen = write_build_rs_file(path)

    with open("nmap-payloads") as file:
        # Read and iterate over the file
        line_iter: Iterator[str] = iter_without_comments_or_empty_lines(iter(file.read().splitlines(keepends=True)))

        # Create a list to store the currently read payload entry
        entry: List[str] = []

        # Get the ports for the current payload
        ports = next(line_iter)

        parts = ports.split(" ")
        if len(parts) == 2:
            ports = parts[1]
        elif len(parts) == 3:
            ports = parts[1]
            entry.append(parts[2].rstrip("\n").lstrip(" ").strip("\""))

        # Iterate over the lines and extract the payloads
        while line := next(line_iter, None):
            if line.startswith("udp "):
                # The previous payload if completed.
                payload_str = "".join(entry)

                # Convert the payload to bytes
                payload: bytes = bytes(payload_str, encoding="utf-8").decode("unicode_escape").encode("raw_unicode_escape")

                # Send the payload to the generator writing the build.rs file
                next(output_gen)
                output_gen.send((ports, payload))

                # Clear the entry for the new payload
                entry.clear()

                # Get the ports for the new (current) payload
                parts = line.split(" ")
                if len(parts) == 2:
                    _, ports = parts
                    continue
                if len(parts) == 3:
                    # If this line has 3 parts that means it includes part of the payload inline
                    _, ports, payload_data = parts
                    entry.append(payload_data.rstrip("\n").lstrip(" ").strip("\""))
                    continue
                if len(parts) > 3:
                    _, ports, *payload_data = parts
                    payload_data = " ".join(payload_data)
                    entry.append(payload_data.rstrip("\n").lstrip(" ").strip("\""))
                    continue
            # Otherwise, the line is a part of the payload
            entry.append(line.rstrip("\n").lstrip(" ").strip("\""))

        # The last
        payload: bytes = bytes("".join(entry), encoding="utf-8")
        next(output_gen)
        output_gen.send((ports, payload))

        # Close the generator
        next(output_gen)
        output_gen.send(None)

        # Assert we've reached the end i.e. file is closed (context manager is over)
        assert next(line_iter, None) is None

def iter_without_comments_or_empty_lines(lines: Iterator[str]) -> Iterator[str]:
    # Iterate over the lines and skip the ones that are comments or empty
    for line in lines:
        if line.startswith("#") or line == "\n":
            continue
        # Yield the remaining
        yield line


def write_build_rs_file(path: str) -> Generator[None, Optional[Tuple[str, bytes]], None]:
    # Open the file
    with open(path, "w") as file:
        # Write the header
        file.write("use std::collections::BTreeMap;\n\npub fn get_parsed_data() -> BTreeMap<Vec<u16>, Vec<u8>> {\n    let mut map = BTreeMap::new();\n")

        # Get the inital input
        input = yield
        while input is not None:
            # Write the payload to the file
            ports, payload = input

            # Ports are currently in the format "80,90,100..." or "80-90,100...". Replace "-" with actual port lists
            ports_strs = ports.split(",")

            # Parse the ports into a set
            ports = set()
            for port_str in ports_strs:
                if "-" in port_str:
                    # Get the start and end and add the apporpriate ports
                    start, end = map(int, port_str.split("-"))
                    ports |= {*range(start, end + 1)}
                else:
                    ports |= {int(port_str)}

            ports = [*ports]
            payload = [*payload]

            # Write the ports and the payload to the file
            file.write(f"    map.insert(vec!{ports}, vec!{payload});\n")

            # Wait for the next input
            yield
            input = yield

        # Put the eof newline and last closing bracket
        file.write("}\n")

        # Wait for the last input
        yield

        # Close the file
        return

if __name__ == "__main__":
    args: ArgumentParser = ArgumentParser()
    args.add_argument("--path", type=str, required=True, dest="path")

    vars = vars(args.parse_args())

    read_nmap_payloads(**vars)
