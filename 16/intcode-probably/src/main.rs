fn main() {
    println!("Hello World!");
}
enum Packet {
    Literal(Literal),
    Operator(Operator)
}
struct Literal {
    version: u8,
    value: u32,
    id: u8
}
struct Operator {
    version: u8, 
    id: u8,
    packets: Vec<Packet>
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
        _ => { return None; }
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
// fn parse_operator_packet(mut binstring: String) -> (Packet, String) {
//     let length_type = binbinstring.remove(0);

// }

#[test]
fn find_first_packet() {
    let mut binstring = convert_2_bin("D2FE28");
    if let Some((packet, binstring)) = find_next_packet(binstring) {
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