use crate::machinestate::MachineState;

pub fn evaluate_bubc(state: &MachineState) -> u8 {
    /*
     * BUBC consists of:
     * BUBC0 - BUT 17:00 (74LS150)
     * BUBC0 - BUT 37:20 (74LS150)
     * BUBC1 - (74LS158)
     * BUBC2 - (74LS151)
     * BUBC3 - (74LS151)
     * BUBC4 - (74LS153)
     * BUBC5 - (74LS153)
     */

    let UBF = state.U_WORD.UBF;
    let UBF_4 = UBF >> 4 & 1;
    let UBF_0_1_2_3 = UBF & 0b1111; // Used a mux input

    let BUBC: u8 = 0;
    let mut BUBC0: u8 = 0;
    let mut BUBC1: u8 = 0;

    // Multiplexers E97 and (E81,E72) are mutually exclusive
    if UBF_4 == 0 {
        BUBC0 = match UBF_0_1_2_3 {     // E97
            0b0000 => 0,                // 0 NOOP
            0b0001 => todo!("E97 1"),   // 1
            0b0010 => todo!("E97 2"),   // 2
            0b0011 => todo!("E97 3"),   // 3
            0b0100 => todo!("E97 4"),   // 4
            0b0101 => todo!("E97 5"),   // 5
            0b0110 => state.SWITCH as u8, // 6 BUT(SWITCH)
            0b0111 => todo!("E97 7"),   // 7
            0b1000 => todo!("E97 8"),   // 8
            0b1001 => todo!("E97 9"),   // 9
            0b1010 => (state.D == 0) as u8,  // 10 BUT(D=0)
            0b1011 => todo!("E97 11"),  // 11
            0b1100 => todo!("E97 12"),  // 12
            0b1101 => todo!("E97 13"),  // 13
            0b1110 => todo!("E97 14"),  // 14
            0b1111 => todo!("E97 15"),  // 15 BUT(IR03)
            _ => panic!("Invalid UBF mux on E97")
        };
    } else {
        BUBC0 = match UBF_0_1_2_3 { // E81
            0b0000 => todo!("E81 0"),   // 0 BUT(20)
            0b0001 => todo!("E81 1"),   // 1
            0b0010 => todo!("E81 2"),   // 2
            0b0011 => panic!("E81 3 - Not Used"),   // 3
            0b0100 => 0,                // 4
            0b0101 => todo!("E81 5"),   // 5
            0b0110 => todo!("E81 6"),   // 6
            0b0111 => todo!("E81 7"),   // 7
            0b1000 => todo!("E81 8"),   // 8
            0b1001 => todo!("E81 9"),   // 9
            0b1010 => todo!("E81 10"),  // 10
            0b1011 => todo!("E81 11"),  // 11
            0b1100 => todo!("E81 12"),  // 12
            0b1101 => todo!("E81 13"),  // 13
            0b1110 => todo!("E81 14"),  // 14
            0b1111 => todo!("E81 15"),  // 15
            _ => panic!("Invalid UBF mux on E81")
        };
        BUBC1 = match UBF_0_1_2_3 { // E72
            0b0000 => todo!("E72 0"),   // 0
            0b0001 => todo!("E72 1"),   // 1
            0b0010 => todo!("E72 2"),   // 2
            0b0011 => panic!("E72 3 - Not Used"),   // 3
            0b0100 => todo!("E72 2"),   // 4
            0b0101 => todo!("E72 5"),   // 5
            0b0110 => todo!("E72 6"),   // 6
            0b0111 => todo!("E72 7"),   // 7
            0b1000 => todo!("E72 8"),   // 8
            0b1001 => todo!("E72 9"),   // 9
            0b1010 => todo!("E72 10"),  // 10
            0b1011 => todo!("E72 11"),  // 11
            0b1100 => todo!("E72 12"),  // 12
            0b1101 => todo!("E72 13"),  // 13
            0b1110 => todo!("E72 14"),  // 14
            0b1111 => todo!("E72 15"),  // 15
            _ => panic!("Invalid UBF mux on E72")
        };
    }

    BUBC | BUBC0 | (BUBC1 << 1)
}