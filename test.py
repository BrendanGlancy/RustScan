import re

# Function to convert hexadecimal string format into a vector of numbers
def hex_string_to_vector(hex_string):
    # Extract all hexadecimal sequences (e.g., \x00, \x1F)
    hex_matches = re.findall(r'\\x[0-9a-fA-F]{2}', hex_string)
    # Convert each hexadecimal match to its byte value
    byte_sequence = bytes(int(hm[2:], 16) for hm in hex_matches)
    return list(byte_sequence)

# Read the content of your file (replace 'keepme' with your actual file path)
with open('keepme', 'r') as file:
    lines = file.readlines()

# Process each line individually
for line in lines:
    # Remove any surrounding whitespace characters
    line = line.strip()
    if line:
        # Convert the line to a vector of numbers
        vector = hex_string_to_vector(line)
        # Print the original line and its hex conversion
        print(f"Line: {line}")
        print(f"Hex Conversion: {vector}\n")
