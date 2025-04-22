use crate::constants;



pub fn decode_ir(IR: u16) {
    
    // E80 - DM=N
    let ir_03_05 = ((IR >> 3) & 7) as u8;
    let DM = four_to_ten_decode(ir_03_05) as u8;

    // E61 - IR(08:06)=N
    let ir_06_08 = ((IR >> 6) & 7) as u8;
    let IR_SELECT = four_to_ten_decode(ir_06_08) as u8;

    // E31 - SM=N
    let ir_09_11 = ((IR >> 9) & 7) as u8;
    let SM = four_to_ten_decode(ir_09_11) as u8;

    // E11 - Instruction type?
    let ir_12_14 = ((IR >> 12) & 7) as u8;
    let IR_TYPE = four_to_ten_decode(ir_12_14) as u8;
}

/// ## Four to ten decode
/// Maps 0-10 to a single bit in a u16.
/// 
/// ### Arguments
/// `address` - Decode Input
/// 
/// ### Returns
/// `u16` - Active low 16-bit value (2 => 1101111111)
/// ### Panic
/// If `address` is greater than 9 will panic.
fn four_to_ten_decode(address: u8) -> u16 {
    // The truth table for a 4 to 10 decoder looks like this:  
    // 0111111111 - 0
    // 1011111111 - 1
    // 1101111111 - 2
    // 1110111111 - 3
    // 1111011111 - 4
    // 1111101111 - 5
    // 1111110111 - 6
    // 1111111011 - 7
    // 1111111101 - 8
    // 1111111110 - 9
    // The key takeaway is that the active bit gets shifted to the right

    // More then 9 is invalid, but that my still happen in hardware, well see
    if address > 9 {
        panic!("{} is out of range for 4 to 10 decode", address);
        // return u16::MAX
    }
    return u16::MAX & !(1 << address);
}

/// ## Evaluate ALU Mux (K3-8)
///
/// The ALU mux selects the "ALU Select lines" to be used with the ALU.
/// 
/// Either base instruction and extended instruction set select lines are used.
/// 
/// ### Arguments
/// * `SXT` - Select Extended Instruction Set
/// * `ALU` - ALU Microword (SALUM + SALU)
/// * `ESALU` - "Extended Select ALU" lines
pub fn evaluate_alu_mux(SXT: bool, ALU: u8, ESALU: u8) -> (u8, u8) {
    let SALU: u8 = ALU & 0x0F;      // Select ALU
    let SALUM: u8 = (ALU >> 4) & 1; // Select ALU Mode

    // SXT: Select Extended Instruction Set (Probably) SXT & DAD (3*2) TODO: DAD (3*2) 
    // SALU: Select ALU
    // ESALU: Extended Select ALU (Probably)
    // RETURN: SALU (4 bits)

    if SXT { // if select extended instruction set
        if constants::HAS_EIS { // if has extended instruction set hardware
            (SALUM, ESALU)
        } else {
            (SALUM, 0) // I am not sure if this is a possible outcome
        }
    } else {
        (SALUM, SALU) // Regular pdp-11 capabilities, no extended instruction
    }
}