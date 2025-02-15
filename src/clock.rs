#![allow(non_snake_case)]
#![allow(clippy::upper_case_acronyms)]


#[derive(Debug, PartialEq)]
pub enum ClockPulse {
    NONE,
    P1,
    P2,
    P3,
}

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum ClockMode {
    OFF,
    CL1,
    CL2,
    CL3,
}

pub struct ClockState {
    pub mode: ClockMode,
    pub pulse_count: u32,
    pub pulses: &'static [ClockPulse; 2]
}

pub const CL1: ClockState = ClockState {
    mode: ClockMode::CL1,
    pulse_count: 1,
    pulses: &[ClockPulse::P1, ClockPulse::NONE]
};

pub const CL2: ClockState = ClockState {
    mode: ClockMode::CL2,
    pulse_count: 1,
    pulses: &[ClockPulse::P2, ClockPulse::NONE]
};

pub const CL3: ClockState = ClockState {
    mode: ClockMode::CL3,
    pulse_count: 2,
    pulses: &[ClockPulse::P2, ClockPulse::P3]
};