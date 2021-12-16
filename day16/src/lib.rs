use num_bigint::BigUint;

/// Convert hexadecimal to an integer version
///
/// Wish I had smaller than u8 :(
/// ```
/// assert_eq!(day16::parse_hexadecimal("D2FE28"), vec![0xD, 0x2, 0xF, 0xE, 0x2, 0x8]);
/// ```
pub fn parse_hexadecimal(input: &str) -> Vec<u8> {
    return input
        .chars()
        .map(|x| u8::from_str_radix(&x.to_string(), 16).unwrap())
        .collect();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum PacketType {
    Literal,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Packet {
    version: u8,
    packet_type: PacketType,
    literal_value: BigUint,
}

fn parse_packet(packet: &Vec<u8>) -> Packet {
    // Standard header
    // first three bits -> version
    let header_version = (packet.first().unwrap() & 0xE) >> 1;
    // Next three bits is type ID, need to assemble this from two numbers.
    let mut header_type_id = (packet.first().unwrap() & 0x1) << 2;
    header_type_id += packet.iter().nth(1).unwrap() & 0xC;

    // Literal value, encodes a single binary number
    if header_type_id == 4 {
        let mut current_bit = 6;
        let mut parts: Vec<u8> = Vec::new();
        let mut is_done = false;
        // Get the next 5 packets,
        while !is_done {
            let (not_done, part) = parse_literal_piece(packet, current_bit);
            is_done = !not_done;
            parts.push(part);
            current_bit += 5;
        }
        let value = parts_to_literal(parts);
        return Packet {
            version: header_version,
            packet_type: PacketType::Literal,
            literal_value: value,
        };
    }

    // Temp invalid packet fopr testing
    return Packet {
        version: 0000,
        packet_type: PacketType::Literal,
        literal_value: BigUint::parse_bytes(b"0", 10).unwrap(),
    };
}

fn parse_literal_piece(packet: &Vec<u8>, start_bit: usize) -> (bool, u8) {
    //    0   1    2    3   4    5
    //  +--++---++---++--+ +--++---+
    //  110100 10111 11110 00101 000
    //  VVVTTT AAAAA BBBBB CCCCC
    //
    //  000000 00001 11111 11122 222
    //  012345 67890 12345 78901 234
    // We should be called on A, B, and C, so start of 6, end of 10
    // Start of 11, end of 15
    // Start of 16, end of 20

    let start_bit_in_index = start_bit % 4;
    let start_index = start_bit / 4;
    let end_index = (start_bit + 4) / 4;
    let mut start_mask = 0;
    // from the first part, we want the last X parts of the bits.
    for _ in start_bit_in_index..4 {
        start_mask = start_mask << 1;
        start_mask += 1;
    }
    // from the second part, we want the first X parts of the bits
    let mut end_mask = 0xF;
    // if we want 4, we want this loop to run 0 times
    // if we want 3, we want one run
    // if we want 2, we want two runs
    let mut start_mask_clone = start_mask >> 1;
    while start_mask_clone > 0 {
        end_mask = end_mask << 1;
        start_mask_clone = start_mask_clone >> 1;
    }
    // Throw away any value not in 4 bits, needed because this is u8 not a u4 :(
    end_mask = end_mask & 0xF;

    println!(
        "start_bit {}, start_mask {}, start_index {}",
        start_bit, start_mask, start_index
    );
    println!(
        "end_bit {}, end_mask {}, end_index {}",
        start_bit + 4,
        end_mask,
        end_index
    );
    // If start_mask is 3, then end mask should be 14
    // if start mask is 1, end mask should be 15
    // If start mask is 15, end mask should be 8.
    let first_bit_part = packet.iter().nth(start_index).unwrap() & start_mask;
    let second_bit_part = packet.iter().nth(end_index).unwrap() & end_mask;

    let mut not_end = true;
    let mut result: u8 = 0;
    // From the start_mask we can figure out these values
    // There is probably math I could do here...
    if start_mask == 0x1 {
        // we have 000N, aaaa
        not_end = first_bit_part == 1;
        result = second_bit_part;
    } else if start_mask == 0x3 {
        // we have 00Na, aaa0
        not_end = (first_bit_part & 0x2) == 2;
        result = (first_bit_part & 0x1) << 3;
        result += second_bit_part >> 1;
    } else if start_mask == 0x7 {
        // we have 0Naa, aa00
        not_end = (first_bit_part & 0x4) == 4;
        result = (first_bit_part & 0x3) << 2;
        result += second_bit_part >> 2;
    } else if start_mask == 0xF {
        // we have Naaa, a000
        not_end = (first_bit_part & 0x8) == 8;
        result = (first_bit_part & 0x7) << 1;
        result += second_bit_part >> 3;
    }
    return (not_end, result);
}

fn parts_to_literal(parts: Vec<u8>) -> BigUint {
    let mut v = BigUint::new(vec![]);
    for x in parts {
        v = v << 4;
        v += x;
    }
    return v;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_hex_literal() -> Vec<u8> {
        return parse_hexadecimal("D2FE28");
    }

    #[test]
    fn test_parse_packet() {
        let packet = make_hex_literal();
        let parsed = parse_packet(&packet);
        assert_eq!(parsed.literal_value, BigUint::new(vec![2021]));
        assert_eq!(parsed.version, 6);
        assert_eq!(parsed.packet_type, PacketType::Literal);
    }

    #[test]
    fn test_parse_literal_piece() {
        let packet = make_hex_literal();
        let (not_done, result) = parse_literal_piece(&packet, 6);
        assert_eq!(not_done, true);
        assert_eq!(result, 0x7);
        let (not_done, result) = parse_literal_piece(&packet, 11);
        assert_eq!(not_done, true);
        assert_eq!(result, 0xE);
        let (not_done, result) = parse_literal_piece(&packet, 16);
        assert_eq!(not_done, false);
        assert_eq!(result, 0x5);
    }

    #[test]
    fn test_parts_to_literal() {
        assert_eq!(
            parts_to_literal(vec![0x7, 0xE, 0x5]),
            BigUint::new(vec![2021])
        );
    }
}
