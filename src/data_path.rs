pub fn evaluate_bmux(sbm: u8, b: u16, bConst: u16) -> u16 {
    let B = b;

    // MUX input
    let sbmL = sbm & 0b0011;
    let sbmH = (sbm & 0b1100) >> 2;

    // Split B REG
    let BL: u8 = (B & 0b11111111) as u8;
    let BH: u8 = ((B & 0b11111111_00000000) >> 8) as u8;
    let B7: u8 = ((B >> 7) & 1) as u8;
    let B7 = 0u8.wrapping_sub(B7 & 1);

    // Split b const
    let bConstL = (bConst & 0b11111111) as u8;
    let bConstH: u8 = ((bConst & 0b11111111_00000000) >> 8) as u8;

    // Check for more then 4 sbm bits in uses
    if(sbm >> 4) > 0 {
        panic!("Invalid SBM: {:o}", sbm);
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
        _ => panic!("Invalid SBM: {:o}", sbm)
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
        _ => panic!("Invalid SBM: {:o}", sbm)
    };

    ((bmuxH as u16) << 8) | (bmuxL as u16)
}