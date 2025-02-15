#![allow(non_snake_case)]

use crate::{clock::ClockState, microrom::MicroWord};

pub struct MachineState<'a> {

    // A structure that represents the current clocking mode
    // contains associated information
    pub CLK_MODE: &'a ClockState,


    // Microprogram Pointer - 8 bits + 1 (K2-2)
    // Initially we'll leave UPP8 unimplemented
    pub UPP: u8,


    // Buffered Microprogram Pointer - 8 bits
    // Essentially the same as the UPP, used by the diagnostic tool
    pub BUPP: u8,


    // Past Microprogram Pointer - 8 bits (K2-2)
    // UPP is stored in the PUPP, the PUPP Register is clocked each time the microword is clocked
    pub PUPP: u8,


    // Basic Microbranch Control - 5 bits
    // This is a collection of latching multiplexers
    pub BUBC: u8,


    // U WORD Register - Comprises multiple chips
    // At the beginning of the current machine state a ROM microword is clocked into the U WORD Register
    pub U_WORD: &'a MicroWord
}
