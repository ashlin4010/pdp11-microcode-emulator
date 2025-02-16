#![allow(non_snake_case)]

use std::ops::Index;

#[allow(dead_code)]
#[derive(Debug)]
pub struct MicroWord  {
    pub implemented: bool,

    /// Microprogram Field
    pub UPF: u8,    // UPF 08:00, 8 bits
    
    /// General Register Select
    pub RIF: u8,    // RIF0-RIF3 4 bits

    /// Source of General Register Address
    pub SRX: u8,    // SRI, SRBA, SRD, SRS

    /// Microbranch Field - Multiplexer input for BUT selection 
    pub UBF: u8,    // UBF0-UBF4 5bits
    
    /// Select Bus Address MUX Input
    pub SBA: u8,    // SBAM

    /// Select DMUX Input
    pub SDM: u8,    // SDM0-SDM1 2 bits

    /// Select BMUX Input
    pub SBM: u8,    // SBML SBMH

    /// Select B Constant 
    pub SBC: u8,    // SBC0-SBC3 4 bits

    /// Select ALU Operation
    pub ALU: u8,    // SALUM SALU0-SALU3 4 bit

    /// Select Processor Status Word
    pub SPS: u8,    // SPS0-SPS2 3 bits

    /// Discrete Alteration of Data
    pub DAD: u8,    // DAD0-DAD4 4 bits

    /// Bus Controls
    pub BUS: u8,    // C1BUS C0BUS BGBUS

    /// Clock Bus Address
    pub CBA: u8,    // CLKBA

    /// Clock ALU Output into D REG
    pub CD: u8,     // CLKD

    /// Clock DMUX into B
    pub CB: u8,     // CLKB

    /// Write DMUX into General Register
    pub WR: u8,     // WRL, WRH

    /// Clock Unibus Data into instruction register
    pub CIR: u8,    // CLKIR

    /// Clock Mode
    pub CLK: u8,    // CLKIR, CLKOFF, CLKL0, CLKL1
}


#[derive(Debug)]
pub struct MicroRom([MicroWord; 256]);

impl Index<usize> for MicroRom {
    type Output = MicroWord;
    
    // This will trigger a panic if an unimplemented micoword is addressed
    fn index(&self, index: usize) -> &Self::Output {
        let output = &self.0[index];
        if !output.implemented {
            todo!("Micoword: {:o}", index);
        }
        output
    }
}

pub static MICROROM: MicroRom = MicroRom([
    MicroWord { implemented: true, CLK: 2, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0o26 }, // 000 000
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 001 001
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 002 002
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 003 003
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 004 004
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 005 005
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 006 006
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 007 007
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 008 010
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 009 011
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 010 012
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 011 013
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 012 014
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 013 015
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 014 016
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 015 017
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 016 020
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 017 021
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 018 022
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 019 023
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 020 024
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 021 025
    MicroWord { implemented: true, CLK: 0o6, CIR: 0o0, WR: 0o0, CB: 0o0, CD: 0o0, CBA: 0o0, BUS: 0o0, DAD: 0o00, SPS: 0o0, ALU: 0o00, SBC: 0o00, SBM: 0o00, SDM: 0o2, SBA: 0o0, UBF: 0o06, SRX: 0o00, RIF: 0o00, UPF: 0o046 }, // 022 026   | CON04   | TEST FOR SWITCH

    MicroWord { implemented: true, CLK: 0o4, CIR: 0o0, WR: 0o0, CB: 0o0, CD: 0o1, CBA: 0o0, BUS: 0o0, DAD: 0o00, SPS: 0o0, ALU: 0o32, SBC: 0o14, SBM: 0o17, SDM: 0o2, SBA: 0o0, UBF: 0o00, SRX: 0o00, RIF: 0o00, UPF: 0o044 }, // 023 027   | CON07   | CONTACT BOUNCE COUNT
    
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 024 030
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 025 031
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 026 032
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 027 033
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 028 034
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 029 035
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 030 036
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 031 037
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 032 040
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 033 041
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 034 042
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 035 043
    MicroWord { implemented: true, CLK: 0o6, CIR: 0o0, WR: 0o3, CB: 0o0, CD: 0o0, CBA: 0o0, BUS: 0o0, DAD: 0o00, SPS: 0o0, ALU: 0o00, SBC: 0o00, SBM: 0o00, SDM: 0o2, SBA: 0o0, UBF: 0o12, SRX: 0o01, RIF: 0o15, UPF: 0o047 }, // 036 044    | CON08 | TEST COUNT
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 037 045
    MicroWord { implemented: true, CLK: 0o2, CIR: 0o0, WR: 0o0, CB: 0o0, CD: 0o0, CBA: 0o0, BUS: 0o0, DAD: 0o00, SPS: 0o0, ALU: 0o00, SBC: 0o00, SBM: 0o00, SDM: 0o2, SBA: 0o0, UBF: 0o00, SRX: 0o00, RIF: 0o00, UPF: 0o026 }, // 038 046   | CONS06   | NO-OP FOR BUT
    
    MicroWord { implemented: true, CLK: 0o6, CIR: 0o0, WR: 0o3, CB: 0o0, CD: 0o1, CBA: 0o0, BUS: 0o0, DAD: 0o00, SPS: 0o0, ALU: 0o11, SBC: 0o01, SBM: 0o17, SDM: 0o2, SBA: 0o0, UBF: 0o00, SRX: 0o01, RIF: 0o15, UPF: 0o44 }, // 039 047   | CON09 | INCREMENT COUNT | 
    
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 040 050
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 041 051
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 042 052
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 043 053
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 044 054
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 045 055
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 046 056
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 047 057
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 048 060
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 049 061
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 050 062
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 051 063
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 052 064
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 053 065
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 054 066
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 055 067
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 056 070
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 057 071
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 058 072
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 059 073
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 060 074
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 061 075
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 062 076
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 063 077
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 064 100
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 065 101
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 066 102
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 067 103
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 068 104
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 069 105
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 070 106
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 071 107
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 072 110
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 073 111
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 074 112
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 075 113
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 076 114
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 077 115
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 078 116
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 079 117
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 080 120
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 081 121
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 082 122
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 083 123
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 084 124
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 085 125
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 086 126
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 087 127
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 088 130
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 089 131
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 090 132
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 091 133
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 092 134
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 093 135
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 094 136
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 095 137
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 096 140
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 097 141
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 098 142
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 099 143
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 100 144
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 101 145
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 102 146
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 103 147
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 104 150
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 105 151
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 106 152
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 107 153
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 108 154
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 109 155
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 110 156
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 111 157
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 112 160
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 113 161
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 114 162
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 115 163
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 116 164
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 117 165
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 118 166
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 119 167
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 120 170
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 121 171
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 122 172
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 123 173
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 124 174
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 125 175
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 126 176
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 127 177
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 128 200
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 129 201
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 130 202
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 131 203
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 132 204
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 133 205
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 134 206
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 135 207
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 136 210
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 137 211
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 138 212
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 139 213
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 140 214
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 141 215
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 142 216
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 143 217
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 144 220
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 145 221
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 146 222
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 147 223
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 148 224
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 149 225
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 150 226
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 151 227
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 152 230
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 153 231
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 154 232
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 155 233
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 156 234
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 157 235
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 158 236
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 159 237
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 160 240
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 161 241
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 162 242
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 163 243
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 164 244
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 165 245
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 166 246
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 167 247
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 168 250
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 169 251
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 170 252
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 171 253
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 172 254
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 173 255
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 174 256
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 175 257
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 176 260
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 177 261
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 178 262
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 179 263
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 180 264
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 181 265
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 182 266
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 183 267
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 184 270
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 185 271
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 186 272
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 187 273
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 188 274
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 189 275
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 190 276
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 191 277
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 192 300
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 193 301
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 194 302
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 195 303
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 196 304
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 197 305
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 198 306
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 199 307
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 200 310
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 201 311
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 202 312
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 203 313
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 204 314
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 205 315
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 206 316
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 207 317
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 208 320
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 209 321
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 210 322
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 211 323
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 212 324
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 213 325
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 214 326
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 215 327
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 216 330
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 217 331
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 218 332
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 219 333
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 220 334
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 221 335
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 222 336
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 223 337
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 224 340
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 225 341
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 226 342
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 227 343
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 228 344
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 229 345
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 230 346
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 231 347
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 232 350
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 233 351
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 234 352
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 235 353
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 236 354
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 237 355
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 238 356
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 239 357
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 240 360
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 241 361
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 242 362
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 243 363
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 244 364
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 245 365
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 246 366
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 247 367
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 248 370
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 249 371
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 250 372
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 251 373
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 252 374
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 253 375
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 254 376
    MicroWord { implemented: false, CLK: 0, CIR: 0, WR: 0, CB: 0, CD: 0, CBA: 0, BUS: 0, DAD: 0, SPS: 0, ALU: 0, SBC: 0, SBM: 0, SDM: 0, SBA: 0, UBF: 0, SRX: 0, RIF: 0, UPF: 0 }, // 255 377    
]);



