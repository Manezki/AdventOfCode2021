use std::{fs};

#[derive(Debug)]
enum Packet {
    Operator(Operator),
    Literal(Literal)
}

#[derive(Debug)]
struct Operator {
    version: u8,
    type_id: u8,
    sub_packets: Vec<Packet>
}

#[derive(Debug)]
struct Literal {
    version: u8,
    value: u128
}

fn to_binary (transmission: String) -> String {
    return transmission.chars().map(|c| {
        match c {
            '0' => "0000".to_string(),
            '1' => "0001".to_string(),
            '2' => "0010".to_string(),
            '3' => "0011".to_string(),
            '4' => "0100".to_string(),
            '5' => "0101".to_string(),
            '6' => "0110".to_string(),
            '7' => "0111".to_string(),
            '8' => "1000".to_string(),
            '9' => "1001".to_string(),
            'A' => "1010".to_string(),
            'B' => "1011".to_string(),
            'C' => "1100".to_string(),
            'D' => "1101".to_string(),
            'E' => "1110".to_string(),
            'F' => "1111".to_string(),
            _ => panic!("Unrecognized character")
        }
    }).collect::<Vec<String>>().join("");
}

fn three_bits_to_u8 (binary_string: String) -> u8 {
    return match binary_string.as_str() {
        "000" => 0,
        "001" => 1,
        "010" => 2,
        "011" => 3,
        "100" => 4,
        "101" => 5,
        "110" => 6,
        "111" => 7,
        _ => panic!("Unrecognized input '{:?}'", binary_string)
    };
}

fn parse_next_packet (binary_transmission: String) -> (Packet, usize) {
    let version = three_bits_to_u8(binary_transmission[0..3].to_string());
    let type_id = three_bits_to_u8(binary_transmission[3..6].to_string());
    // println!("{:?}, {:?}", version, type_id);

    let mut bits_read: usize = 6;

    if type_id == 4 {
        // Literal

        let mut value_binary_string = "".to_string();

        while &binary_transmission[bits_read..bits_read + 1] == "1" {
            bits_read += 1;
            value_binary_string += &binary_transmission[bits_read..bits_read + 4].to_string();
            bits_read += 4;
        }

        bits_read += 1;
        value_binary_string += &binary_transmission[bits_read..bits_read + 4].to_string();
        bits_read += 4;

        let value = u128::from_str_radix(&value_binary_string, 2).unwrap();

        return (Packet::Literal(Literal{version, value}), bits_read);

    } else {
        // Operator
        let length_type = binary_transmission[6..7].to_string();
        bits_read += 1;

        let mut sub_packets: Vec<Packet> = Vec::new();

        if length_type == "0" {
            // Bit defined length
            let sub_packet_bits = usize::from_str_radix(&binary_transmission[7..22], 2).unwrap();
            bits_read += 15;

            let final_index= sub_packet_bits + 22;
            let mut current_index: usize = 22;

            // Literal packet is smallest and requires 11 bits
            while final_index - current_index > 10 {
                let (sub_packet, bits) = parse_next_packet(binary_transmission[current_index..final_index].to_string());
                sub_packets.push(sub_packet);
                current_index += bits;
            }

            bits_read += sub_packet_bits;
        } else {
            let n_sub_packets = usize::from_str_radix(&binary_transmission[7..18], 2).unwrap();
            bits_read += 11;

            let mut current_index = 18;
            for _ in 0..n_sub_packets {
                let (sub_packet, bits) = parse_next_packet(binary_transmission[current_index..].to_string());
                sub_packets.push(sub_packet);
                current_index += bits;
                bits_read += bits;
            }
        }
        return (Packet::Operator(Operator {version, type_id, sub_packets}), bits_read);
    }
}

fn version_count (packets: Vec<Packet>) -> u32 {

    let mut version_total = 0;

    for packet in packets {
        match packet {
            Packet::Operator(op) => version_total += op.version as u32 + version_count(op.sub_packets),
            Packet::Literal(lit) => version_total += lit.version as u32,
            _ => panic!("Unrecognized package {:?}", packet)
        }
    }

    return version_total;
}

#[test]
fn puzzle_1_confers_to_example() {
    assert_eq!(version_count(vec![parse_next_packet(to_binary("8A004A801A8002F478".to_string())).0]), 16);
    assert_eq!(version_count(vec![parse_next_packet(to_binary("620080001611562C8802118E34".to_string())).0]), 12);
    assert_eq!(version_count(vec![parse_next_packet(to_binary("C0015000016115A2E0802F182340".to_string())).0]), 23);
    assert_eq!(version_count(vec![parse_next_packet(to_binary("A0016C880162017C3686B18A3D4780".to_string())).0]), 31);
}

fn main () {
    let file_content = fs::read_to_string("src\\day_16_input.txt").expect("Something went wrong reading the file");
    let binary_transmission = to_binary(file_content);
    println!("{:?}", version_count(vec![parse_next_packet(binary_transmission).0]));
}