#![allow(non_snake_case)]

mod machinestate;
mod microrom;
mod clock;
mod debug;

use std::thread;
use std::time::{self, Duration, Instant};
use machinestate::MachineState;
use microrom::MICROROM;
use clock::{ClockMode, CL1, CL2, CL3};
use clock::ClockPulse;
use debug::{print_data_bus, print_diagnostic_tool};

const SINGLE_STEP: bool = true;
const STARTING_UPP: u8 = 0;
const MACHINE_CYCLE_SLEEP: Duration = time::Duration::from_millis(100);


fn evaluate_bubc(state: &MachineState) -> u8 {

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

    let mut BUBC: u8 = 0;
    let mut BUBC0: u8 = 0;
    let mut BUBC1: u8 = 0;

    if UBF_4 == 0 {
        // E97
        BUBC0 = match UBF_0_1_2_3 {
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
            0b1010 => todo!("E97 10"),  // 10
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

    return BUBC | BUBC0 | (BUBC1 << 1);
}

fn main() {

    //-------------------------------
    //---------Machine State---------
    //-------------------------------
    let mut MACHINE_STATE = MachineState::new();
    MACHINE_STATE.U_WORD = &MICROROM[STARTING_UPP as usize];

    MACHINE_STATE.SWITCH = true; // Fake a key press

    //---------------------------------
    //--------Application Loop---------
    //---------------------------------
    loop {
        // Set up next machine cycle
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Clear the screen

        println!("New Machine Cycle");
        println!("Current UPP = {:o}", MACHINE_STATE.UPP);
        println!("Current clk = {:?}", MACHINE_STATE.CLK_MODE.mode);

        let time_zero = Instant::now(); // Time = 0, start of machine cycle

        // Each machine cycle may have either one or two clock pulses
        // Various parts of the processor subscribe to particular clock pulses
        for pulse in 0..MACHINE_STATE.CLK_MODE.pulse_count {
            // Theses flags signify a trigger of a clock pulse, only one pulses is active a loop
            let active_pulse = &MACHINE_STATE.CLK_MODE.pulses[pulse as usize];
            let P1 = *active_pulse == ClockPulse::P1;
            let P2 = *active_pulse == ClockPulse::P2;
            let P3 = *active_pulse == ClockPulse::P3;

            // Add clock delay
            match *active_pulse {
                ClockPulse::NONE => todo!(),
                ClockPulse::P1 => println!("Delay 140ms, pulse P1"),
                ClockPulse::P2 => println!("Delay 200ms, pulse P2"),
                ClockPulse::P3 => println!("Delay 100ms, pulse P3"),
            }

            // For the next 140ms to 300ms the cpu will do stuff
            println!("Current UBF = {:o}", MACHINE_STATE.U_WORD.UBF);


            // At the end of the machine cycle latch the UWORD from the ROM
            if P1 || (P2 && MACHINE_STATE.CLK_MODE.mode != ClockMode::CL3) || P3 {
                // Latch UPP into PUPP
                MACHINE_STATE.PUPP = MACHINE_STATE.UPP;

                // Compute next UPP based off the content of BUBC from the last machine cycle
                MACHINE_STATE.UPP = MACHINE_STATE.U_WORD.UPF | MACHINE_STATE.BUBC; // Latch in the next UPP

                // At the start of the next machine cycle BUPP/UPP will point the the "current" microstate.
                MACHINE_STATE.BUPP = MACHINE_STATE.UPP;

                // Clock so latch in new microword
                MACHINE_STATE.U_WORD = &MICROROM[MACHINE_STATE.UPP as usize];
                
                // The BUBC lines are in flux for one machine cycle, we save that state in BUBC_NEXT
                MACHINE_STATE.BUBC = MACHINE_STATE.BUBC_FLUX;
                // evaluate_bubc must be called at the end of the machine cycle to allow time for BUTs to settle
                MACHINE_STATE.BUBC_FLUX = evaluate_bubc(&MACHINE_STATE);

                // TODO: Clear switch
            }
        }


        // RE-CLOCK
        //  The next clock cycle must be known before the next machine cycle
        MACHINE_STATE.CLK_MODE = match &MACHINE_STATE.U_WORD.CLK {
            6 => &CL3,
            5 => &CL3,
            4 => &CL2,
            3 => &CL2,
            2 => &CL1,
            1 => &CL1,
            _ => panic!("Impossible clock state")
        };

        // End of machine cycle (logging only!)
        let elapsed = time_zero.elapsed();
        println!("Machine cycle elapsed time: {:.2?}\n", elapsed);

        // The only at the end of the machine cycle can we consider the data output to be stable
        print_data_bus(MACHINE_STATE.PUPP as u16);
        print_diagnostic_tool(MACHINE_STATE.PUPP, MACHINE_STATE.BUPP);

        println!("\nThe values below are that will be used at the start of the next machine cycle");
        println!("UPP = {:o}", MACHINE_STATE.UPP);
        println!("CLK = {:?}", MACHINE_STATE.CLK_MODE.mode);

        // Alow time for the users to watch the simulator
        match SINGLE_STEP {
            true => { let _ = std::io::stdin().read_line(&mut String::new());},
            false => { thread::sleep(MACHINE_CYCLE_SLEEP);},
        };
    }
}
