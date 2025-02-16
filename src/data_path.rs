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