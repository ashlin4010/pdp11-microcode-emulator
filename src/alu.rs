// This is going to be hard


pub fn evaluate_alu_mux(SXT: bool, SALU: u8, ESALU: u8) -> u8 {
    // SALU: Select ALU
    // ESALU: Extended Select ALU (Probably)
    // SXT: Select Extended (Probably)
    if SXT {
        ESALU
    } else {
        SALU
    }
}