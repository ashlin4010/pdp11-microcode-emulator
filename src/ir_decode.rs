use crate::constants;



// K3-8
pub fn evaluate_alu_mux(SXT: bool, ALU: u8, ESALU: u8) -> (u8, u8) {
    let SALU: u8 = ALU & 0x0F;
    let SALUM: u8 = (ALU >> 4) & 1;

    // The ALU mux switches ALU Select lines between the
    // base instruction and extended instruction set

    // SXT: Select Extended Instruction Set (Probably) SXT & DAD (3*2) TODO: DAD (3*2) 
    // SALU: Select ALU
    // ESALU: Extended Select ALU (Probably)
    // RETURN: SALU (4 bits)

    // EIS = false
    if SXT {
        if constants::HAS_EIS {
            (SALUM, ESALU)
        } else {
            (SALUM, 0)
        }
    } else {
        (SALUM, SALU)
    }
}