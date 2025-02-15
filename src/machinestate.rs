#![allow(non_snake_case)]

use crate::{clock::{ClockState, CL1}, microrom::{MicroWord, MICROROM}};

pub struct MachineState<'a> {

    // A structure that represents the current clocking mode
    // contains associated information
    pub CLK_MODE: &'a ClockState,

    // U WORD Register - Comprises multiple chips
    // At the beginning of the current machine state a ROM microword is clocked into the U WORD Register
    pub U_WORD: &'a MicroWord,

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
    // BUBC represents the logical OR of the inputs into the
    // OR gates that feed the UPP latches excluding the UPP
    // Both BUBC0 and lines have previously been ORed
    pub BUBC: u8,
    pub BUBC_FLUX: u8,

    // SWITCH Flip-Flip
    // Set on: ADRS_SW, EXAM_SW, CONT_SW, DEP_SW, START and BEGIN
    // Cleared on: not sure
    pub SWITCH: bool
}


impl MachineState<'_> {
    pub fn new() -> MachineState<'static> {
        MachineState {
            CLK_MODE: &CL1,
            U_WORD: &MICROROM[0],
            UPP: 0,
            BUPP: 0,
            PUPP: 0,
            BUBC: 0,
            BUBC_FLUX: 0,
            SWITCH: false,
        }
    }
}