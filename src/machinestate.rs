#![allow(non_snake_case)]

use crate::{clock::{ClockState, CL1}, microrom::{MicroWord, MICROROM}};

const SINGLE_CLOCK_MODE: bool = true;
const STARTING_UPP: u8 = 0;


pub struct MachineState<'a> {

    // The data display is directly driven off the out of the DMUX
    // For now I will explicitly set DATA_DISPLAY after evaluating the DMUX value
    pub DATA_DISPLAY: u16,

    // A structure that represents the current clocking mode
    // contains associated information
    pub CLK_MODE: &'a ClockState,

    // Single Clock Mode
    // This is true if in Single Clock Mode (using KM11), it skips the SWITCH debounce.
    pub SINCLK: bool,

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
    pub SWITCH: bool,

    // B Register
    pub B: u16,

    // D Register
    pub D: u16,
}


impl MachineState<'_> {
    pub fn new() -> MachineState<'static> {
        MachineState {
            DATA_DISPLAY: 0,
            CLK_MODE: &CL1,
            SINCLK: SINGLE_CLOCK_MODE,
            U_WORD: &MICROROM[STARTING_UPP as usize],
            UPP: STARTING_UPP,
            BUPP: 0,
            PUPP: 0,
            BUBC: 0,
            BUBC_FLUX: 0,
            SWITCH: false,
            B: 0,
            D: 0,
        }
    }
}