#![allow(non_snake_case)]
#![allow(clippy::print_literal)]

fn u8_to_bools(n: u8) -> [bool; 8] {
    std::array::from_fn(|i| (n >> (7 - i)) & 1 == 1)
}

fn bools_to_strings(bools: [bool; 8]) -> Vec<String> {
    bools.iter()
        .map(|&bit| if bit { "●".to_string() } else { "○".to_string() })
        .collect()
}

pub fn print_data_bus(data: u16) {
    let border = "      x--------------------------------------------------x";
    let mut data_display = String::from("Data  |");
    for i in (0..16).rev() {
        let bit = (data >> i) & 1;
        if bit == 0 {
            data_display += "  ○";
        } else {
            data_display += "  ●";
        }
    }
    data_display += "  |";
    print!("{}\n{}\n{}\n", border, data_display, border);
}

pub fn print_diagnostic_tool(PUPP: u8, BUPP: u8) {
    let mut p = bools_to_strings(u8_to_bools(PUPP));
    let mut b = bools_to_strings(u8_to_bools(BUPP));
    p.reverse();
    b.reverse();
    
    println!("+-----------------------+");
    println!("|  {}  {}  {}  {}  {}  {}  {}  |", p[6], p[3], p[0], b[6], b[3], b[0], "○");
    println!("|  {}  {}  {}  {}  {}  {}  {}  |", p[7], p[4], p[1], b[7], b[4], b[1], "○");
    println!("|  {}  {}  {}  {}  {}  {}  {}  |", "○", p[5], p[2], "○", b[5], b[2], "○");
    println!("|  {}  {}  {}  {}  {}  {}  {}  |", "○", "○", "○", "○", "○", "○", "○");
    println!("+-----------------------+");
}