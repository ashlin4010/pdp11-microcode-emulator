use crate::alu_74181;

/// ## PDP-11 ALU (K1-2 E74)
/// 
/// This function emulates the PDP-11s 16 bit ALU
/// 
/// ### Arguments
/// * `SALUM` - Select ALU mode (Logic or Arithmetic)
/// * `SALU` - Select ALU lines, selects the individual ALU function
/// * `A` - A input
/// * `B` - B input
/// * `C` - Carry in
/// 
/// ### Returns
/// * `F` - ALU Output
/// * `COUT03` - Carry Out Stage 1
/// * `COUT07` - Carry Out Stage 2 (8 bit operations)
/// * `COUT11` - Carry Out Stage 3
/// * `COUT15` - Carry Out Stage 4 (16 bit operations)
/// 
pub fn pdp_alu(SALUM: u8,  SALU: u8, A: u16, B: u16, C: bool) -> (u16, bool, bool, bool, bool) {
    let M: bool = SALUM == 1;
    let S: [bool; 4] = core::array::from_fn(|i| ((SALU >> i) & 1) == 1);

    // ALU 0 - Inputs
    let a0: [bool; 4] = core::array::from_fn(|i| ((A >> i + (4 * 0)) & 1) == 1);
    let b0: [bool; 4] = core::array::from_fn(|i| ((B >> i + (4 * 0)) & 1) == 1);

    // ALU 1 - Inputs
    let a1: [bool; 4] = core::array::from_fn(|i| ((A >> i + (4 * 1)) & 1) == 1);
    let b1: [bool; 4] = core::array::from_fn(|i| ((B >> i + (4 * 1)) & 1) == 1);

    // ALU 2 - Inputs
    let a2: [bool; 4] = core::array::from_fn(|i| ((A >> i + (4 * 2)) & 1) == 1);
    let b2: [bool; 4] = core::array::from_fn(|i| ((B >> i + (4 * 2)) & 1) == 1);

    // ALU 3 - Inputs
    let a3: [bool; 4] = core::array::from_fn(|i| ((A >> i + (4 * 3)) & 1) == 1);
    let b3: [bool; 4] = core::array::from_fn(|i| ((B >> i + (4 * 3)) & 1) == 1);

    // ALU 0 - E74
    let (f0, _, _, G0, P0) = alu_74181::alu_slice(a0, b0, S, C, M);
    let COUT03 = alu_74181::look_ahead_carry_unit_0(C, G0, P0);

    // ALU 1 - E81
    let (f1, _, _, G1, P1) = alu_74181::alu_slice(a1, b1, S, COUT03, M);
    let COUT07 = alu_74181::look_ahead_carry_unit_1(C, G0, G1, P0, P1);

    // ALU 2 - E42
    let (f2, _, _, G2, P2) = alu_74181::alu_slice(a2, b2, S, COUT07, M);
    let COUT11 = alu_74181::look_ahead_carry_unit_2(C, G0, G1, G2, P0, P1, P2);

    // ALU 3 - E18
    let (f3, COUT15, _, _, _) = alu_74181::alu_slice(a3, b3, S, COUT11, M);

    // ALU - Outputs
    let alu_outout: [[bool; 4]; 4] = [f0, f1, f2, f3];

    let mut F: u16 = 0;
    for (row, bits) in alu_outout.iter().enumerate() {
        for (col, &bit) in bits.iter().enumerate() {
            F |= (bit as u16) << (row * 4 + col);
        }
    }

    (F, COUT03, COUT07, COUT11, COUT15)

}


/// ## Evaluate Carry Out Mux (K1-5 E23)
/// 
/// The bit sliced ALU has several possible carry out bits,
/// the carry mux select between the following:
/// 
/// * ALU Carry Out for 16 bits (COUT15)
/// * ALU Carry Out for 8 bits (COUT07)
/// * !PS(C) - Not sure what this is
/// * !ALU(15:15) - NOT bit 15 from the ALU output
pub fn evaluate_cout_mux(COMUX: u8, COUT15: bool, COUT07: bool, PS_C_: u8, F: u16) -> bool {
    let alu_15 = (F >> 15) & 1;
    match COMUX {
        0b00 => COUT15,
        0b01 => COUT07,
        0b10 => !(PS_C_ == 1),
        0b11 => !(alu_15 == 1),
        _ => panic!("Invalid COMUX: {:b}", COMUX)
    }
}

/// ## Evaluate D Mux (K1-1 E82)
/// 
/// The D MUX is is used to route data internally the processor,
/// it has 4 inputs and connects to approximately 5 destinations
/// (Data Display, B Register, PS Register, Insrt Register and REG)
pub fn evaluate_dmux(SDM: u8, BUS_RD: u16, BUS_D: u16, D: u16, D_C: u8) -> u16 {
    // 00 => BUS RD
    // 01 => BUS_D (Unibus data)
    // 10 => D REG
    // 11 => (D REG >> 1) | D_C
    match SDM {
        0b00 => BUS_RD,
        0b01 => BUS_D,
        0b10 => D,
        0b11 => (D >> 1) | ((D_C as u16) << 15),
        _ => panic!("Invalid SDM: {:o}", SDM)
    }
}

/// ## Evaluate B Mux (K1-1 E72)
/// 
/// The B Mux is used to select ALU B inputs.
/// 
/// It has 2 input sources (B Register and B Constant) and 4 inputs.
/// 
/// The B Mux selects between permutations of B Register and B Constant
pub fn evaluate_bmux(SBM: u8, B: u16, B_CONST: u16) -> u16 {
    // MUX input - Consider the B Mux as two independently controlled mux
    // High Byte and Low Byte
    let sbmL = SBM & 0b0011;
    let sbmH = (SBM & 0b1100) >> 2;

    // Split B REG
    let BL: u8 = (B & 0b11111111) as u8;
    let BH: u8 = ((B & 0b11111111_00000000) >> 8) as u8;
    let B7: u8 = ((B >> 7) & 1) as u8;
    let B7 = 0u8.wrapping_sub(B7 & 1); // Complex way to do B7,B7,B7,B7,B7,B7,B7,B7

    // Split B CONST
    let bConstL = (B_CONST & 0b11111111) as u8;
    let bConstH: u8 = ((B_CONST & 0b11111111_00000000) >> 8) as u8;

    // Check for more then 4 sbm bits in uses
    if(SBM >> 4) > 0 {
        panic!("Invalid SBM: {:o}", SBM);
    }

    // Low byte
    // 00 => B REG
    // 01 => B REG
    // 10 => B[7:0] B[15:8] (Swap H with L) 
    // 11 => B Constant
    let bmuxL: u8 = match sbmL {
        0b00 => BL,
        0b01 => BL,
        0b10 => BH,
        0b11 => bConstL,
        _ => panic!("Invalid SBM: {:o}", SBM)
    };

    // High byte
    // 00 => B REG
    // 01 => B7 B7 B7 B7
    // 10 => B[7:0] B[15:8] (Swap H with L)
    // 11 => B Constant ()
    // Note that the high byte of the B Constant is only truly 2 bits duplicated to four bits
    // This makes the schematics look complicated but at an emulation level we are just doing all 16 bits
    // as if the constant was truly 16 bits. See K5-5
    let bmuxH: u8 = match sbmH {
        0b00 => BH,
        0b01 => B7,
        0b10 => BL,
        0b11 => bConstH,
        _ => panic!("Invalid SBM: {:o}", SBM)
    };

    ((bmuxH as u16) << 8) | (bmuxL as u16)
}