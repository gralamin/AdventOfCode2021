pub use filelib::load;
use num_bigint::BigUint;

/// Convert hexadecimal to an integer version
///
/// Wish I had smaller than u8 :(
/// ```
/// assert_eq!(day16::parse_hexadecimal("D2FE28"), vec![0xD, 0x2, 0xF, 0xE, 0x2, 0x8]);
/// ```
pub fn parse_hexadecimal(input: &str) -> Vec<u8> {
    let no_filler: String = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect();
    return no_filler
        .chars()
        .map(|x| u8::from_str_radix(&x.to_string(), 16).unwrap())
        .collect();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum PacketType {
    Literal,
    Operator(u32),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Packet {
    version: u8,
    packet_type: PacketType,
    literal_value: BigUint,
    length_type_id: bool,
    sub_packets: Vec<Packet>,
}

impl Packet {
    fn solve(&self) -> u64 {
        let v: u64;
        let mut solved_sub_packets = self.sub_packets.iter().map(|x| x.solve());
        let mut solved_sub_packets2 = self.sub_packets.iter().map(|x| x.solve());
        v = match self.packet_type {
            PacketType::Literal => get_u64_from_big(self.literal_value.clone()),
            PacketType::Operator(0) => solved_sub_packets.sum(),
            PacketType::Operator(1) => solved_sub_packets.product(),
            PacketType::Operator(2) => solved_sub_packets.min().unwrap(),
            PacketType::Operator(3) => solved_sub_packets.max().unwrap(),
            PacketType::Operator(5) => {
                (solved_sub_packets.nth(0).unwrap() > solved_sub_packets2.nth(1).unwrap()) as u64
            }
            PacketType::Operator(6) => {
                (solved_sub_packets.nth(0).unwrap() < solved_sub_packets2.nth(1).unwrap()) as u64
            }
            PacketType::Operator(7) => {
                (solved_sub_packets.nth(0).unwrap() == solved_sub_packets2.nth(1).unwrap()) as u64
            }
            _ => 0,
        };
        return v;
    }
}

fn get_bits_from_packet_stream(packet_stream: &Vec<u8>, start: usize, end: usize) -> BigUint {
    let end_as_u64: u64 = end.try_into().unwrap();
    let mut as_literal = BigUint::new(vec![]);
    for i in packet_stream.iter() {
        as_literal = as_literal << 4;
        as_literal += *i;
    }
    // If I use "size" here we lose out on leading 0s, instead use the packet_stream to maintain.
    let size: u64 = (packet_stream.len() * 4).try_into().unwrap();
    //println!("Getting {} to {} of a {} size", start, end, size);
    assert_eq!(end_as_u64 < size, true);
    // consider 1110 0011 1001
    //  If I want  ^----^
    //  start = 3
    //  end = 7
    // size = 12
    // size - end - 1 = 4 number of right shifts I should do.
    let num_bits = end - start + 1;
    let shift_count: u64 = size - end_as_u64 - 1;
    as_literal = as_literal >> shift_count;
    let u_1: u8 = 1;
    let mask = (BigUint::new(vec![1]) << num_bits) - u_1;
    return mask & as_literal;
}

fn get_u32_from_big(v: BigUint) -> u32 {
    let digits = v.to_u32_digits();
    let r: u32;
    if digits.len() == 0 {
        r = 0;
    } else {
        r = *(digits.first().unwrap());
    }
    return r;
}

fn get_u64_from_big(v: BigUint) -> u64 {
    let digits = v.to_u64_digits();
    let r: u64;
    if digits.len() == 0 {
        r = 0;
    } else {
        r = *(digits.first().unwrap());
    }
    return r;
}

fn recursively_parse_packet(packet: &Vec<u8>, start_bit: usize) -> (Packet, usize) {
    let header_version_u32 = get_u32_from_big(get_bits_from_packet_stream(
        packet,
        start_bit,
        start_bit + 2,
    ));
    let header_version: u8 = header_version_u32.try_into().unwrap();
    let mut current_bit = start_bit + 3;
    let header_type_id = get_u32_from_big(get_bits_from_packet_stream(
        packet,
        current_bit,
        current_bit + 2,
    ));

    current_bit = current_bit + 3;
    //println!("{}", header_type_id);

    if header_type_id == 4 {
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
        let result = Packet {
            version: header_version,
            packet_type: PacketType::Literal,
            literal_value: value,
            length_type_id: false,
            sub_packets: vec![],
        };
        return (result, current_bit);
    }
    let length_type_id_u32 = get_u32_from_big(get_bits_from_packet_stream(
        packet,
        current_bit,
        current_bit,
    ));
    let length_type_id = length_type_id_u32 == 1;
    current_bit += 1;
    let mut parts: Vec<Packet> = Vec::new();
    if !length_type_id {
        //println!("In a len0");
        let length_bits = get_u32_from_big(get_bits_from_packet_stream(
            packet,
            current_bit,
            current_bit + 14,
        ));
        let length_bits: usize = length_bits.try_into().unwrap();
        current_bit += 15;
        let end_bit = current_bit + length_bits - 1;
        while current_bit < end_bit {
            //println!("on bit {} of {}", current_bit, end_bit);
            let (part, end_of_packet) = recursively_parse_packet(packet, current_bit);
            parts.push(part);
            current_bit = end_of_packet;
        }
    } else {
        //println!("In a len1");
        let num_sub_packets = get_u32_from_big(get_bits_from_packet_stream(
            packet,
            current_bit,
            current_bit + 10,
        ));
        let num_sub_packets: usize = num_sub_packets.try_into().unwrap();
        current_bit += 11;

        for _i in 0..num_sub_packets {
            //println!("sub packet {} of {}", _i + 1, num_sub_packets);
            let (part, end_of_packet) = recursively_parse_packet(packet, current_bit);
            parts.push(part);
            current_bit = end_of_packet;
        }
    }
    let result = Packet {
        version: header_version,
        packet_type: PacketType::Operator(header_type_id),
        literal_value: BigUint::new(vec![]),
        length_type_id: length_type_id,
        sub_packets: parts,
    };
    return (result, current_bit);
}

fn parse_packet(packet: &Vec<u8>) -> Packet {
    let (result, _) = recursively_parse_packet(packet, 0);
    return result;
}

fn parse_literal_piece(packet: &Vec<u8>, start_bit: usize) -> (bool, u8) {
    let not_end_u32 = get_u32_from_big(get_bits_from_packet_stream(packet, start_bit, start_bit));
    let not_end = not_end_u32 == 1;
    let big_int = get_bits_from_packet_stream(packet, start_bit + 1, start_bit + 4);
    let value_u32 = get_u32_from_big(big_int);
    let result: u8 = value_u32.try_into().unwrap();
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

/// Add up all the packet version numbers
///
/// ```
/// let packet_stream = day16::parse_hexadecimal("A0016C880162017C3686B18A3D4780");
/// assert_eq!(day16::puzzle_a(&packet_stream), 31);
/// println!("-------");
/// let packet_stream = day16::parse_hexadecimal("C0015000016115A2E0802F182340");
/// assert_eq!(day16::puzzle_a(&packet_stream), 23);
/// println!("-------");
/// let packet_stream = day16::parse_hexadecimal("620080001611562C8802118E34");
/// assert_eq!(day16::puzzle_a(&packet_stream), 12);
/// println!("-------");
/// let packet_stream = day16::parse_hexadecimal("8A004A801A8002F478");
/// assert_eq!(day16::puzzle_a(&packet_stream), 16);
/// ```
pub fn puzzle_a(packet_stream: &Vec<u8>) -> u32 {
    let packet = parse_packet(packet_stream);
    let mut packet_stack: Vec<Packet> = vec![packet];
    let mut versions: u32 = 0;
    while let Some(p) = packet_stack.pop() {
        let v: u32 = p.version.try_into().unwrap();
        versions += v;
        packet_stack.append(&mut p.sub_packets.clone());
    }
    return versions;
}

/// Run the packet operations
///
/// ```
/// let packet_stream = day16::parse_hexadecimal("C200B40A82");
/// assert_eq!(day16::puzzle_b(&packet_stream), 3);
/// println!("-------");
/// let packet_stream = day16::parse_hexadecimal("04005AC33890");
/// assert_eq!(day16::puzzle_b(&packet_stream), 54);
/// println!("-------");
/// let packet_stream = day16::parse_hexadecimal("880086C3E88112");
/// assert_eq!(day16::puzzle_b(&packet_stream), 7);
/// println!("-------");
/// let packet_stream = day16::parse_hexadecimal("CE00C43D881120");
/// assert_eq!(day16::puzzle_b(&packet_stream), 9);
/// println!("-------");
/// let packet_stream = day16::parse_hexadecimal("D8005AC2A8F0");
/// assert_eq!(day16::puzzle_b(&packet_stream), 1);
/// println!("-------");
/// let packet_stream = day16::parse_hexadecimal("F600BC2D8F");
/// assert_eq!(day16::puzzle_b(&packet_stream), 0);
/// println!("-------");
/// let packet_stream = day16::parse_hexadecimal("9C005AC2F8F0");
/// assert_eq!(day16::puzzle_b(&packet_stream), 0);
/// println!("-------");
/// let packet_stream = day16::parse_hexadecimal("9C0141080250320F1802104A08");
/// assert_eq!(day16::puzzle_b(&packet_stream), 1);
/// ```
pub fn puzzle_b(packet_stream: &Vec<u8>) -> u64 {
    let packet = parse_packet(packet_stream);
    return packet.solve();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_hex_literal() -> Vec<u8> {
        return parse_hexadecimal("D2FE28");
    }

    fn make_hex_lentype0() -> Vec<u8> {
        return parse_hexadecimal("38006F45291200");
    }

    fn make_hex_lentype1() -> Vec<u8> {
        return parse_hexadecimal("EE00D40C823060");
    }

    fn make_hex_embed_type0() -> Vec<u8> {
        return parse_hexadecimal("C0015000016115A2E0802F182340");
    }

    fn make_hex_embed_type1() -> Vec<u8> {
        return parse_hexadecimal("620080001611562C8802118E34");
    }

    fn make_hex_embed_deep() -> Vec<u8> {
        return parse_hexadecimal("A0016C880162017C3686B18A3D4780");
    }

    #[test]
    fn test_parse_packet_literal() {
        let packet = make_hex_literal();
        let parsed = parse_packet(&packet);
        assert_eq!(parsed.literal_value, BigUint::new(vec![2021]));
        assert_eq!(parsed.version, 6);
        assert_eq!(parsed.packet_type, PacketType::Literal);
    }

    #[test]
    fn test_parse_packet_operator_len0() {
        let packet = make_hex_lentype0();
        // 00111000000000000110111101000101001010010001001000000000
        // VVVTTTILLLLLLLLLLLLLLLAAAAAAAAAAABBBBBBBBBBBBBBBB
        // 00000000001111111111222222222233333333334444444444555555
        // 01234567890123456789012345678901234567890123456789012345
        // We want the Ls, start at 7, end at 17.
        let parsed = parse_packet(&packet);
        assert_eq!(parsed.literal_value, BigUint::new(vec![]));
        assert_eq!(parsed.version, 1);
        assert_eq!(parsed.length_type_id, false);
        assert_eq!(parsed.packet_type, PacketType::Operator(6));
        let sub_one = parsed.sub_packets.first().unwrap();
        assert_eq!(sub_one.literal_value, BigUint::new(vec![10]));
        assert_eq!(sub_one.version, 6);
        assert_eq!(sub_one.packet_type, PacketType::Literal);
        let sub_two = parsed.sub_packets.iter().nth(1).unwrap();
        assert_eq!(sub_two.literal_value, BigUint::new(vec![20]));
        assert_eq!(sub_two.version, 2);
        assert_eq!(sub_two.packet_type, PacketType::Literal);
    }

    #[test]
    fn test_parse_packet_operator_len1() {
        let packet = make_hex_lentype1();
        let parsed = parse_packet(&packet);
        assert_eq!(parsed.literal_value, BigUint::new(vec![]));
        assert_eq!(parsed.version, 7);
        assert_eq!(parsed.length_type_id, true);
        assert_eq!(parsed.packet_type, PacketType::Operator(3));
        let sub_one = parsed.sub_packets.first().unwrap();
        assert_eq!(sub_one.literal_value, BigUint::new(vec![1]));
        assert_eq!(sub_one.version, 2);
        assert_eq!(sub_one.packet_type, PacketType::Literal);
        let sub_two = parsed.sub_packets.iter().nth(1).unwrap();
        assert_eq!(sub_two.literal_value, BigUint::new(vec![2]));
        assert_eq!(sub_two.version, 4);
        assert_eq!(sub_two.packet_type, PacketType::Literal);
        let sub_three = parsed.sub_packets.iter().nth(2).unwrap();
        assert_eq!(sub_three.literal_value, BigUint::new(vec![3]));
        assert_eq!(sub_three.version, 1);
        assert_eq!(sub_three.packet_type, PacketType::Literal);
    }

    #[test]
    fn test_parse_packet_embed_type0() {
        // 1100000000000001010100000000000000000001011000010001010110100010111000001000000000101111000110000010001101000000
        // VVVTTTILLLLLLLLLLL
        let packet = make_hex_embed_type0();
        let binary_string: String = packet.iter().map(|x| format!("{:04b}", x)).collect();
        println!("{}", binary_string);
        let parsed = parse_packet(&packet);
        assert_eq!(parsed.version, 6);
    }

    #[test]
    fn test_parse_packet_embed_type1() {
        // 01100010000000001000000000000000000101100001000101010110001011001000100000000010000100011000111000110100
        // VVVTTTILLLLLLLLLLLVVVTTTILLLLLLLLLLLLLLLVVVTTTCZZZZVVVTTTCZZZZ
        // 000000000011111111112222222222333333333344444444445555555555566666666666777777777778888888888899999999999
        // 01234567890123456789012345678901234567890123456789012345678901234560789012345607890123456078901234560789
        let packet = make_hex_embed_type1();
        let binary_string: String = packet.iter().map(|x| format!("{:04b}", x)).collect();
        println!("{}", binary_string);
        let parsed = parse_packet(&packet);
        assert_eq!(parsed.version, 3);
    }

    #[test]
    fn test_parse_packet_embed_deep() {
        let packet = make_hex_embed_deep();
        let binary_string: String = packet.iter().map(|x| format!("{:04b}", x)).collect();
        println!("{}", binary_string);
        let parsed = parse_packet(&packet);
        assert_eq!(parsed.version, 5);
    }

    #[test]
    fn test_get_bits_from_packet_stream() {
        let packet = make_hex_lentype1();
        // 11101110000000001101010000001100100000100011000001100000
        // VVVTTTILLLLLLLLLLLAAAAAAAAAAABBBBBBBBBBBCCCCCCCCCCC
        // 000000000011111111112222222222333333333344444444445
        // 012345678901234567890123456789012345678901234567890
        // We want the Ls, start at 7, end at 17.
        let v = get_bits_from_packet_stream(&packet, 0, 2);
        assert_eq!(v, BigUint::new(vec![7]));
        let t = get_bits_from_packet_stream(&packet, 3, 5);
        assert_eq!(t, BigUint::new(vec![3]));
        let i = get_bits_from_packet_stream(&packet, 6, 6);
        assert_eq!(i, BigUint::new(vec![1]));
        let l = get_bits_from_packet_stream(&packet, 7, 17);
        assert_eq!(l, BigUint::new(vec![3]));
        let a = get_bits_from_packet_stream(&packet, 18, 28);
        assert_eq!(a, BigUint::new(vec![641]));
        let b = get_bits_from_packet_stream(&packet, 29, 39);
        assert_eq!(b, BigUint::new(vec![1154]));
        let c = get_bits_from_packet_stream(&packet, 40, 50);
        assert_eq!(c, BigUint::new(vec![387]));
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
