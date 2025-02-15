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


fn evaluate_but(flag: bool) -> u8 {
    if flag {
        0x01
    } else {
        0        
    }
}

fn set_but(but: u8) -> bool {
    but == 0o6
}

fn main() {

    //-------------------------------
    //---------Machine State---------
    //-------------------------------
    let mut MACHINE_STATE = MachineState {
        CLK_MODE: &CL1,
        UPP: STARTING_UPP,
        BUPP: 0,
        PUPP: 0,
        BUBC: 0,
        U_WORD: &MICROROM[STARTING_UPP as usize],
    };

    let mut switch_flag = false;


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
            // Set BUT MUX
            // GET value from last but mux
            switch_flag = set_but(MACHINE_STATE.U_WORD.UBF);


            println!("Current UBF = {:o}", MACHINE_STATE.U_WORD.UBF);

            // if P1 || (P2 && clock_mode.mode != ClockMode::CL3) || P3 {
            //     // Demo: on microstate 26 evaluate BUT(switch)
            //     if (UPP == 0o46) {
            //         BUBC = 0x01;
            //     } else {
            //         BUBC = 0x00; // no switch
            //     }
            // }


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

                
                MACHINE_STATE.BUBC = evaluate_but(switch_flag); // update BUBC

                // Compute next UPP based off the content of BUBC from the last machine cycle
                // next_UPP = U_WORD.UPF | BUBC;
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
