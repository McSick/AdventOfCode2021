fn main() {
    let mut binstring = convert_2_bin("A20D790042F1274011955491808B802F1C60B20030327AF2CC248AA800E7CDD726F3D78F4966F571A300BA54D668E2519249265160803EA9DE562A1801204ACE53C954ACE53C94C659BDF318FD1366EF44D96EB11005FB39154E0068A7C3A6B379646C80348A0055E6642B332109B8D6F0F12980452C9D322B28012EC72D51B300426CF70017996DE6C2B2C70C01A04B67B9F9EC8DAFE679D0992A80380104065FA8012805BD380120051E380146006380142004A00E920034C0801CA007B0099420053007144016E28018800CCC8CBB5FE79A3D91E1DC9FB151A1006CC0188970D6109803B1D61344320042615C198C2A014C589D00943096B3CCC081009173D015B004C401C8E10421E8002110BA18C193004A52257E0094BCE1ABB94C2C9005112DFAA5E80292B405927020106BC01494DFA6E329BF4DD273B69E233DB04C435BEF7A0CC00CFCDF31DC6AD20A3002A498CC01D00042229479890200E4438A91700010F88F0EA251802D33FE976802538EF38E2401B84CA05004833529CD2A5BD9DDAC566009CC33E8024200CC528E71F40010A8DF0C61D8002B5076719A5D418034891895CFD320730F739A119CB2EA0072D25E870EA465E189FDC1126AF4B91100A03600A0803713E2FC7D00043A25C3B8A12F89D2E6440242489A7802400086C788FB09C0010C8BB132309005A1400D2CBE7E7F2F9F9F4BB83803B25286DFE628E129EBCB7483C8802F3D0A2542E3004AC0169BD944AFF263361F1B48010496089807100BA54A66675769B1787D230C621EF8B9007893F058A009AE4ED7A5BBDBE05262CEC0002FC7C20082622E0020D0D66A2D04021D5003ED3D396E19A1149054FCA3586BD00020129B0037300042E0CC1184C000874368F70A251D840239798AC8DC9A56F7C6C0E0728015294D9290030B226938A928D0");
    if let Some((packet, _binstring)) = find_next_packet(binstring) {
        let sum = add_all_versions(packet);
        println!("Part 1: {}", sum); 
    }
}
#[derive(Debug, PartialEq)]
enum Packet {
    Literal(Literal),
    Operator(Operator)
}
#[derive(Debug, PartialEq)]
struct Literal {
    version: u8,
    value: u32,
    id: u8
}
#[derive(Debug, PartialEq)]
struct Operator {
    version: u8, 
    id: u8,
    packets: Vec<Packet>
}
fn add_all_versions(packet: Packet) -> u32 {
    match packet {
        Packet::Operator(op) => {
            let mut sum = op.version as u32;
            for sub_packet in op.packets {
                sum += add_all_versions(sub_packet);
            }
            return sum;
        }
        Packet::Literal(lit)=> lit.version as u32
   }
}
fn find_next_packet(mut binstring:String) -> Option<(Packet, String)> {
    let version = u8::from_str_radix(binstring.drain(..3).collect::<String>().as_str(), 2).unwrap();
    let packet_id = u8::from_str_radix(binstring.drain(..3).collect::<String>().as_str(), 2).unwrap();
    match packet_id {
        4 =>  {
            let (value, binstring) = parse_literal_packet(binstring);
            let packet = Packet::Literal(Literal {version: version, value: value, id: packet_id});
            return Some((packet, binstring));
        } ,        
        _ => {
            let (packets, binstring) = parse_operator_packet(binstring);
            let packet = Packet::Operator(Operator { version: version, id: packet_id, packets:packets});
            return Some((packet, binstring));
        }
    };
}
fn parse_literal_packet(mut binstring: String) -> (u32, String) {
    let mut value_str:String = "".to_string();
    let mut should_read = true;
    while should_read {
        should_read = binstring.remove(0) == '1';
        let next_digits = binstring.drain(..4).collect::<String>();
        value_str += &next_digits;
    }
    (u32::from_str_radix(value_str.as_str(), 2).unwrap(), binstring)
}
fn parse_operator_packet(mut binstring: String)  -> (Vec<Packet>, String)  {
    let length_type = binstring.remove(0);
    match length_type {
        '0' =>  {
            let n_bits = u32::from_str_radix(binstring.drain(..15).collect::<String>().as_str(), 2).unwrap();
            parse_n_bits(n_bits, binstring)
        },
        '1' => {
            let n_packets = u32::from_str_radix(binstring.drain(..11).collect::<String>().as_str(), 2).unwrap();
            parse_n_subpackets(n_packets, binstring)
        },
        _ => panic!("Invalid Character in string")
    }
   
}

fn parse_n_bits(n_bits: u32, mut binstring: String) -> (Vec<Packet>, String) {
    let beginning_len = binstring.len() as u32;
    let mut consumed_bits = 0;
    let mut packets: Vec<Packet> = vec![];
    while consumed_bits < n_bits {
        if let Some((packet, newstring)) = find_next_packet(binstring.clone()) {
            binstring = newstring;
            consumed_bits = beginning_len - binstring.len() as u32;
            packets.push(packet); 
        }
    }
    (packets, binstring)
}

fn parse_n_subpackets(n_packets: u32, mut binstring: String) -> (Vec<Packet>, String) {
    let mut packets: Vec<Packet> = vec![];
    for _ in 0..n_packets {
        if let Some((packet, newstring)) = find_next_packet(binstring.clone()) {
            binstring = newstring;
            packets.push(packet); 
        }
    }
    (packets, binstring)
}
#[test]
fn test_sums() {
    let mut binstring = convert_2_bin("8A004A801A8002F478");
    if let Some((packet, _binstring)) = find_next_packet(binstring) {
        let sum = add_all_versions(packet);
        assert_eq!(sum, 16);
    }
    let mut binstring = convert_2_bin("620080001611562C8802118E34");
    if let Some((packet, _binstring)) = find_next_packet(binstring) {
        let sum = add_all_versions(packet);
        assert_eq!(sum, 12);
    }
    let mut binstring = convert_2_bin("C0015000016115A2E0802F182340");
    if let Some((packet, _binstring)) = find_next_packet(binstring) {
        let sum = add_all_versions(packet);
        assert_eq!(sum, 23);
    }
    let mut binstring = convert_2_bin("A0016C880162017C3686B18A3D4780");
    if let Some((packet, _binstring)) = find_next_packet(binstring) {
        let sum = add_all_versions(packet);
        assert_eq!(sum, 31);
    }
}
#[test]
fn find_operator_packet_sub() {
    let mut binstring = convert_2_bin("EE00D40C823060");
    if let Some((packet, _binstring)) = find_next_packet(binstring) {
        match packet {
             Packet::Operator(op) => {
                 println!("{:?}", op);
                assert_eq!(op.version, 7);
                assert_eq!(op.id, 3);
                assert_eq!(op.packets.len(), 3);
             }
             _ => ()
        }
    }
}
#[test]
fn find_operator_packet_bits() {
    let mut binstring = convert_2_bin("38006F45291200");
    if let Some((packet, _binstring)) = find_next_packet(binstring) {
        match packet {
             Packet::Operator(op) => {
                 println!("{:?}", op);
                assert_eq!(op.version, 1);
                assert_eq!(op.id, 6);
                assert_eq!(op.packets.len(), 2);
             }
             _ => ()
        }
       
    }

}

#[test]
fn find_first_packet() {
    let mut binstring = convert_2_bin("D2FE28");
    if let Some((packet, _binstring)) = find_next_packet(binstring) {
        match packet {
            Packet::Literal(lit) => {
                assert_eq!(lit.version, 6);
                assert_eq!(lit.value, 2021);
             },
             _ => ()
        }
       
    }

}

fn convert_2_bin(hex:&str) -> String {
    let mut result = "".to_string();
    for digit in hex.chars() {
        result.push_str(char_to_bin(digit));
    }
    result
}   
fn char_to_bin(character: char) -> &'static str { 
    match character {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => ""
    }
}