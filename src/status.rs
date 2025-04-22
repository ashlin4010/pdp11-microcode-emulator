use crate::machinestate::MachineState;


/// ## Evaluate B Constant (K5-5)
/// 
/// The PDP-11 has selection of constants that it can use in
/// various alu and other operations.
/// 
/// Despite the name, constants, the values maybe dependant on internal state
/// this is especially true for the "PWR UP" and "TRAPS" constants.
pub fn evaluate_bconstant(state: &MachineState, sbc: u8) -> u16 {

    // In the hardware some of the b constant data lines are used for more then one bit
    // The true lines are as follows:
    // 
    // BC00, BC01, BC02, BC03, BC04, BC05, BC06, BC07, BC(11,08), BC(15:12, 10:09)
    // 
    // In the case of the emulation its simpler to just do all 16 bits
    // as if they we're truly implemented in hardware.
    // It makes no difference to any other part of the emulation.

    match sbc {
        0o00 => 0,          // TODO: Complete Constant 00 to enable traps
        0o01 => 1,          // CONST1
        0o02 => 2,          // CONST2
        0o03 => todo!("Constant 03"),
        0o04 => panic!("Constant 04 NOT USED"),
        0o05 => panic!("Constant 04 NOT USED"),
        0o06 => panic!("Constant 04 NOT USED"),
        0o07 => todo!("Constant 07"),
        0o10 => 0b1111111101111000, // SR Address
        0o11 => todo!("Constant 11"),
        0o12 => 0b0000000000001111, // CC MASK
        0o13 => 0b0000000000111111, // SOB MASK
        0o14 => {                   // SINCLK - This is used as a switch debounce, if SINCLK don't debounce
            match state.SINCLK {
                true => 0,
                false => 0b0000000000010000,
            }
        },
        0o15 => 0b0000000010101000, // MM VECTOR
        0o16 => todo!("Constant 16"),
        0o17 => 0b0000000000000100, // STACK04
        _ => panic!("Invalid 'Select B Constant'")
    }
}