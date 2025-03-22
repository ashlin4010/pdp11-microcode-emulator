use crate::alu_74181;


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

    let mut f: u16 = 0;
    for (row, bits) in alu_outout.iter().enumerate() {
        for (col, &bit) in bits.iter().enumerate() {
            f |= (bit as u16) << (row * 4 + col);
        }
    }

    (f, COUT03, COUT07, COUT11, COUT15)

}


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

pub fn evaluate_bmux(SBM: u8, B: u16, B_CONST: u16) -> u16 {
    // MUX input
    let sbmL = SBM & 0b0011;
    let sbmH = (SBM & 0b1100) >> 2;

    // Split B REG
    let BL: u8 = (B & 0b11111111) as u8;
    let BH: u8 = ((B & 0b11111111_00000000) >> 8) as u8;
    let B7: u8 = ((B >> 7) & 1) as u8;
    let B7 = 0u8.wrapping_sub(B7 & 1);

    // Split b const
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
    // 00 => B
    // 01 => B7 B7 B7 B7
    // 10 => B[7:0] B[15:8] (Swap H with L)
    // 11 => B Constant ()
    // Note that the high byte of the B Constant is only truly 2 bits duplicated to four bits
    // This makes the look complicated schematics but at an emulation level we are just doing all 16 bits
    let bmuxH: u8 = match sbmH {
        0b00 => BH,
        0b01 => B7,
        0b10 => BL,
        0b11 => bConstH,
        _ => panic!("Invalid SBM: {:o}", SBM)
    };

    ((bmuxH as u16) << 8) | (bmuxL as u16)
}