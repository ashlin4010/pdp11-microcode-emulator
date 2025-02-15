#![allow(non_snake_case)]

mod machinestate;
mod bconstant;
mod clock;
mod microrom;
mod microbranch_control;
mod debug;

use std::thread;
use std::time::{self, Duration, Instant};
use machinestate::MachineState;
use microrom::MICROROM;
use clock::{ClockMode, CL1, CL2, CL3};
use clock::ClockPulse;
use debug::{print_data_bus, print_diagnostic_tool};

const SINGLE_STEP_EMULATION: bool = true;
const MACHINE_CYCLE_SLEEP: Duration = time::Duration::from_millis(100);


fn main() {

    //-------------------------------
    //---------Machine State---------
    //-------------------------------
    let mut MACHINE_STATE = MachineState::new();

    // TODO: Build UI?
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

            // Test code only
            let bconst = bconstant::evaluate_bconstant(&MACHINE_STATE, MACHINE_STATE.U_WORD.SBC);
            println!("B Const {}", bconst);
            MACHINE_STATE.D = bconst; // REMOVE THIS AFTER TESTING replace mux
            MACHINE_STATE.DATA_DISPLAY = bconst;


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
                MACHINE_STATE.BUBC_FLUX = microbranch_control::evaluate_bubc(&MACHINE_STATE);

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
        print_data_bus(MACHINE_STATE.DATA_DISPLAY);
        print_diagnostic_tool(MACHINE_STATE.PUPP, MACHINE_STATE.BUPP);

        println!("\nThe values below are that will be used at the start of the next machine cycle");
        println!("UPP = {:o}", MACHINE_STATE.UPP);
        println!("CLK = {:?}", MACHINE_STATE.CLK_MODE.mode);

        // Alow time for the users to watch the simulator
        match SINGLE_STEP_EMULATION {
            true => { let _ = std::io::stdin().read_line(&mut String::new());},
            false => { thread::sleep(MACHINE_CYCLE_SLEEP);},
        };
    }
}
