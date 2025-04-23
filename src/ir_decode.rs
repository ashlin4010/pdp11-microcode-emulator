use crate::{constants, microrom::MicroWord};

impl Default for DecodedInstruction {
    fn default() -> Self {
        Self {
            // IR (05:03)
            DM__7_L: 0,
            DM__6_L: 0,
            DM__5_L: 0,
            DM__4_L: 0,
            DM__3_L: 0,
            DM__2_L: 0,
            DM__1_L: 0,
            DM__0_L: 0,

            // IR (08:06)
            IR_08_06__7_L: 0,
            IR_08_06__6_L: 0,
            IR_08_06__5_L: 0,
            IR_08_06__4_L: 0,
            IR_08_06__3_L: 0,
            IR_08_06__2_L: 0,
            IR_08_06__1_L: 0,
            IR_08_06__0_L: 0,
            
            // IR (11:09)
            SM__7_L: 0,
            SM__6_L: 0,
            SM__5_L: 0,
            SM__4_L: 0,
            SM__3_L: 0,
            SM__2_L: 0,
            SM__1_L: 0,
            SM__0_L: 0,
            
            // IR (14:12)
            IR_14_12__7_L: 0,
            ADD_SUB_L: 0,
            BIS_L: 0,
            BIC_L: 0,
            BIT_L: 0,
            CMP_L: 0,
            MOV_L: 0,
            IR_14_12__0_L: 0,

            SXT_L: 0,

            COMUX_H: 0,

        }
    }
}

pub struct DecodedInstruction  {
    pub DM__7_L: u8,
    pub DM__6_L: u8,
    pub DM__5_L: u8,
    pub DM__4_L: u8,
    pub DM__3_L: u8,
    pub DM__2_L: u8,
    pub DM__1_L: u8,
    pub DM__0_L: u8,

    pub IR_08_06__7_L: u8,
    pub IR_08_06__6_L: u8,
    pub IR_08_06__5_L: u8,
    pub IR_08_06__4_L: u8,
    pub IR_08_06__3_L: u8,
    pub IR_08_06__2_L: u8,
    pub IR_08_06__1_L: u8,
    pub IR_08_06__0_L: u8,

    pub SM__7_L: u8,
    pub SM__6_L: u8,
    pub SM__5_L: u8,
    pub SM__4_L: u8,
    pub SM__3_L: u8,
    pub SM__2_L: u8,
    pub SM__1_L: u8,
    pub SM__0_L: u8,

    pub IR_14_12__7_L: u8,
    pub ADD_SUB_L: u8,
    pub BIS_L: u8,
    pub BIC_L: u8,
    pub BIT_L: u8,
    pub CMP_L: u8,
    pub MOV_L: u8,
    pub IR_14_12__0_L: u8,

    pub SXT_L: u8,

    pub COMUX_H: u8,
}

impl DecodedInstruction {
    pub fn new(instruction: u16, ba_reg: u16, u_word: &MicroWord, data_carry_bit: u8) -> DecodedInstruction {
        // data_carry_bit => D(C) (1)H


        let mut ir = Self::default();
        let IR_15_1_H: u8 = ((instruction >> 15) & 1) as u8;
        let IR_14_1_H: u8 = ((instruction >> 14) & 1) as u8;
        let IR_13_1_H: u8 = ((instruction >> 13) & 1) as u8;
        let IR_12_1_H: u8 = ((instruction >> 12) & 1) as u8;
        let IR_11_1_H: u8 = ((instruction >> 11) & 1) as u8;
        let IR_10_1_H: u8 = ((instruction >> 10) & 1) as u8;
        let IR_09_1_H: u8 = ((instruction >> 9) & 1) as u8;
        let IR_08_1_H: u8 = ((instruction >> 8) & 1) as u8;
        let IR_07_1_H: u8 = ((instruction >> 7) & 1) as u8;
        let IR_06_1_H: u8 = ((instruction >> 6) & 1) as u8;

        let IR_15_0_H = !(IR_15_1_H) & 1;
        let IR_14_0_H = !(IR_14_1_H) & 1;
        let IR_13_0_H = !(IR_13_1_H) & 1;
        let IR_12_0_H = !(IR_12_1_H) & 1;
        let IR_11_0_H = !(IR_11_1_H) & 1;
        let IR_10_0_H = !(IR_10_1_H) & 1;
        let IR_09_0_H = !(IR_09_1_H) & 1;
        let IR_08_0_H = !(IR_08_1_H) & 1;
        let IR_07_0_H = !(IR_07_1_H) & 1;
        let IR_06_0_H = !(IR_06_1_H) & 1;

        let IR_15_0_L = IR_15_1_H;
        let IR_14_0_L = IR_14_1_H;
        let IR_13_0_L = IR_13_1_H;
        let IR_12_0_L = IR_12_1_H;
        let IR_11_0_L = IR_11_1_H;
        let IR_10_0_L = IR_10_1_H;
        let IR_09_0_L = IR_09_1_H;
        let IR_08_0_L = IR_08_1_H;
        let IR_07_0_L = IR_07_1_H;
        let IR_06_0_L = IR_06_1_H;

        let IR_15_1_L = IR_15_0_H;
        let IR_14_1_L = IR_14_0_H;
        let IR_13_1_L = IR_13_0_H;
        let IR_12_1_L = IR_12_0_H;
        let IR_11_1_L = IR_11_0_H;
        let IR_10_1_L = IR_10_0_H;
        let IR_09_1_L = IR_09_0_H;
        let IR_08_1_L = IR_08_0_H;
        let IR_07_1_L = IR_07_0_H;
        let IR_06_1_L = IR_06_0_H;

        let IR15_L = !(IR_15_1_H) & 1;
        let IR15_H = !(IR_15_1_L) & 1;

        let DAD3 = (u_word.DAD >> 3) & 1;
        let DAD2 = (u_word.DAD >> 2) & 1;
        let DAD1 = (u_word.DAD >> 1) & 1;
        let DAD0 = (u_word.DAD >> 0) & 1;

        let SPS2 = (u_word.SPS >> 2) & 1;
        let SPS1 = (u_word.SPS >> 1) & 1;
        let SPS0 = (u_word.SPS >> 0) & 1;

        let BA00_1_H: u8 = ((ba_reg >> 0) & 1) as u8;

        // K3-3
        let ir_03_05 = ((instruction >> 3) & 7) as u8;
        let DM = four_to_ten_decode(ir_03_05) as u8;
        ir.DM__7_L = (DM >> 7) & 1;
        ir.DM__6_L = (DM >> 6) & 1;
        ir.DM__5_L = (DM >> 5) & 1;
        ir.DM__4_L = (DM >> 4) & 1;
        ir.DM__3_L = (DM >> 3) & 1;
        ir.DM__2_L = (DM >> 2) & 1;
        ir.DM__1_L = (DM >> 1) & 1;
        ir.DM__0_L = (DM >> 0) & 1;

        // K3-3
        let ir_06_08 = ((instruction >> 6) & 7) as u8;
        let IR_SELECT = four_to_ten_decode(ir_06_08) as u8;
        ir.IR_08_06__7_L = (IR_SELECT >> 7) & 1;
        ir.IR_08_06__6_L = (IR_SELECT >> 6) & 1;
        ir.IR_08_06__5_L = (IR_SELECT >> 5) & 1;
        ir.IR_08_06__4_L = (IR_SELECT >> 4) & 1;
        ir.IR_08_06__3_L = (IR_SELECT >> 3) & 1;
        ir.IR_08_06__2_L = (IR_SELECT >> 2) & 1;
        ir.IR_08_06__1_L = (IR_SELECT >> 1) & 1;
        ir.IR_08_06__0_L = (IR_SELECT >> 0) & 1;

        // K3-3
        let ir_09_11 = ((instruction >> 9) & 7) as u8;
        let SM = four_to_ten_decode(ir_09_11) as u8;
        ir.SM__7_L = (SM >> 7) & 1;
        ir.SM__6_L = (SM >> 6) & 1;
        ir.SM__5_L = (SM >> 5) & 1;
        ir.SM__4_L = (SM >> 4) & 1;
        ir.SM__3_L = (SM >> 3) & 1;
        ir.SM__2_L = (SM >> 2) & 1;
        ir.SM__1_L = (SM >> 1) & 1;
        ir.SM__0_L = (SM >> 0) & 1;

        // K3-3
        let ir_12_14 = ((instruction >> 12) & 7) as u8;
        let IR_TYPE = four_to_ten_decode(ir_12_14) as u8;
        ir.IR_14_12__7_L    = (IR_TYPE >> 7) & 1;
        ir.ADD_SUB_L        = (IR_TYPE >> 6) & 1;
        ir.BIS_L            = (IR_TYPE >> 5) & 1;
        ir.BIC_L            = (IR_TYPE >> 4) & 1;
        ir.BIT_L            = (IR_TYPE >> 3) & 1;
        ir.CMP_L            = (IR_TYPE >> 2) & 1;
        ir.MOV_L            = (IR_TYPE >> 1) & 1;
        ir.IR_14_12__0_L    = (IR_TYPE >> 0) & 1;

        // Decode SOP
        let E22 = !(IR_14_1_H | IR_13_1_H | IR_12_1_H | IR_10_1_H) & 1;         // K3-4 E22
        let SOP_L = !(E22 & IR_11_1_H & IR_09_1_H) & 1;


        let sop_select = (SOP_L << 3) | (IR_08_1_H << 2) | (IR_07_1_H << 1) | IR_06_1_H;
        let de_sop = four_to_ten_decode(sop_select) as u8;

        let TST_L = (de_sop >> 7) & 1;
        let SBC_L = (de_sop >> 6) & 1;
        let ADC_L = (de_sop >> 5) & 1;
        let NEG_L = (de_sop >> 4) & 1;
        let DEC_L = (de_sop >> 3) & 1;
        let INC_L = (de_sop >> 2) & 1;
        let COM_L = (de_sop >> 1) & 1;
        let CLR_L = (de_sop >> 0) & 1;

        // K3-5 E32
        let XOR_L = IR15_H | ir.IR_14_12__7_L | ir.SM__4_L;
        let XOR_H = !(XOR_L) & 1;

        // K3-5 E23
        let ROTSHF_L = ir.IR_14_12__0_L | ir.SM__6_L | IR_08_0_L;
        let ROTSHF_H = !(ROTSHF_L) & 1;

        // K3-5 E22 & E33 - Select extended instruction set?
        ir.SXT_L = IR_15_0_L | ir.IR_14_12__0_L | ir.SM__6_L | ir.IR_08_06__7_L;
        let SXT_H = !(ir.SXT_L) & 1;

        let DOP_L = !(ir.IR_14_12__7_L & ir.IR_14_12__0_L) & 1; // K3-5 E43
        

        let E47 = !(DOP_L & ROTSHF_L & SOP_L) & 1;              // K3-6 E47_08
        let BYTE_INSTR_H = ir.ADD_SUB_L& E47 & IR15_H;          // K3-6 E36_08

        // Rotate Right
        let IR_08_06__0_H = !(ir.IR_08_06__0_L) & 1;            // K3-8 E62_02
        let ROT_R_L = !(IR_08_06__0_H & ROTSHF_H) & 1;          // K3-8 E63_03
        
        // Rotate Left
        let IR_08_06__1_H = !(ir.IR_08_06__1_L) & 1;            // K3-8 E62_04
        let ROT_L_H = IR_08_06__1_H & ROTSHF_H;                 // K3-8 E14_06
        
        // Shift Right
        let IR_08_06__2_H = !(ir.IR_08_06__2_L) & 1;            // K3-8 E62_06
        let SHF_R_L  = !(IR_08_06__2_H & ROTSHF_H) & 1;         // K3-8 E14_06

        let ROTSHF_R_L = !(IR_06_0_H & ROTSHF_H) & 1;
        let ROTSHF_R_H = IR_06_0_H & ROTSHF_H;


        // Carry Out MUX //
        if constants::HAS_EIS {
            todo!("Extended instruction set has not been implemented");
        }

        let E63_08= !(ROT_R_L & BYTE_INSTR_H) & 1;              // K3-8 E63
        let ECOMUXS0: u8 = 1; // KE-5
        let COMUXS0_H = !(ECOMUXS0 & SHF_R_L & E63_08) & 1;     // K3-8 E64_08

        let ECOMUXS1: u8 = 1;                                       // KE-5
        let COMUXS1_H = !(ECOMUXS1 & SHF_R_L & ROT_R_L) & 1;    // K3-8 E64_08
        ir.COMUX_H = (COMUXS1_H << 1) | COMUXS0_H;



        let PS_C_H: u8 = 1;             //TODO: K5-2
        let PS_C_L: u8 = !(PS_C_H) & 1; //TODO: K5-2

        // Carry In 00 // TODO: 
        let E16_13 = !(DAD2 | DAD1) & 1;
        
        let E14_08 = DAD3 & DAD2; // DAD (3*2) H
        let E3_06 = !(ir.CMP_L & INC_L) & 1; // TODO: CMP+INC H
        let E16_04 = !(ADC_L | PS_C_L) & 1;

        let E17_and_unit_1 = DAD3 & E16_13;
        let E17_and_unit_2 = E14_08 & E3_06;
        let E17_and_unit_3 = E14_08 & E16_04;
        let E17_and_unit_4 = E14_08 & ROT_L_H & PS_C_H;

        // C DATA (K3-9)
        let PASTC_L = 0;    //TODO: PASTC_L K5-2
        let PASTA_H = 1;    //TODO: K5-2
        let N_DATA_L = 1;   //TODO: K5-2
        let LOAD_PS_H = 0;  //TODO: LOAD_PS_H K5-2
        let LOAD_PS_L = !(LOAD_PS_H) & 1;
        let D00 = 0;        //TODO
        let DMUX00_H = 0;   //TODO

        let BIC_BIT_L = ir.BIT_L & ir.BIC_L;
        let BIC_BIT_H = !(BIC_BIT_L) & 1; // K3-8 E2_08 - BIC+BIT_H
        let DM__0_H = !(ir.DM__0_L) & 1;

        let data_carry_low = !(data_carry_bit) & 1;             // E29_04 This comes from the flip-flop just after the carry out mux
        let SUB_H = !(IR15_L | ir.ADD_SUB_L) & 1;               // K3-9 E9_13
        let SUB_L = !(SUB_H) & 1;                               // K3-9 E8_04
        let DEC_H = !(DEC_L) & 1;

        let ODD_BYTE_H = !(BA00_1_H & BYTE_INSTR_H) & 1;
        
        let E10_04 = !(SBC_L | PASTC_L) & 1; // K3-9 E10_04 PASTC (1)L
        let E16_01 = !(SPS2 | DM__0_H) & 1;
        let E25_08 = !(E16_01 & SPS1 & SPS0 & ODD_BYTE_H) & 1;
        let E24_08 = !(DEC_L & ir.MOV_L & XOR_L & ir.SXT_L & INC_L & BIC_BIT_L & ir.BIS_L & E25_08) & 1;
        let E10_10 = !(DEC_H | E10_04) & 1;     // K3-9 E10_10
        let E30_08 = !(ir.CMP_L & E10_10 & SUB_L & NEG_L) & 1; // Done
        let E15_08 = !(E30_08 & E25_08 & DEC_L) & 1; // Done
        let E18_06 = !(COM_L & E15_08) & 1; // Done
        let E27_06 = !(LOAD_PS_H | E18_06 | ROTSHF_R_H | E24_08) & 1; // Done
        let E36_12 = SBC_L & E27_06; // Done

        let E29_04 = data_carry_low;
        let E28_and_unit_1 = E29_04 & E36_12;

        let E9_04 = !(PS_C_L | SBC_L) & 1;
        let E9_10 = !(PASTA_H | N_DATA_L) & 1;
        let E28_and_unit_2 = E9_04 & E9_10;


        let E28_and_unit_3 = LOAD_PS_H & DMUX00_H;

        let E59_12 = SBC_L & E18_06;
        let E28_and_unit_4 = data_carry_bit & LOAD_PS_L & E59_12;

        let E28_and_unit_5 = LOAD_PS_L & PS_C_H & E24_08;
        let E28_and_unit_6 = LOAD_PS_L & ROTSHF_R_H & D00;
        
        return ir;
    }
}

/// ## Four to ten decode
/// Maps 0-10 to a single bit in a u16.
/// 
/// ### Arguments
/// `address` - Decode Input
/// 
/// ### Returns
/// `u16` - Active low 16-bit value (2 => 1101111111)
/// ### Panic
/// If `address` is greater than 9 will panic.
fn four_to_ten_decode(address: u8) -> u16 {
    // More then 9 is invalid, but that my still happen in hardware, well see
    if address > 9 {
        panic!("{} is out of range for 4 to 10 decode", address);
        // return u16::MAX
    }
    return u16::MAX & !(1 << address);
}

/// ## Evaluate ALU Mux (K3-8)
/// The ALU mux selects the "ALU Select lines" to be used with the ALU.
/// 
/// Either base instruction and extended instruction set select lines are used.
/// 
/// ### Arguments
/// * `SXT` - Select Extended Instruction Set
/// * `ALU` - ALU Microword (SALUM + SALU)
/// * `ESALU` - "Extended Select ALU" lines
pub fn evaluate_alu_mux(SXT: bool, ALU: u8, ESALU: u8) -> (u8, u8) {
    let SALU: u8 = ALU & 0x0F;      // Select ALU
    let SALUM: u8 = (ALU >> 4) & 1; // Select ALU Mode

    // SXT: Select Extended Instruction Set (Probably) SXT & DAD (3*2) TODO: DAD (3*2) 
    // SALU: Select ALU
    // ESALU: Extended Select ALU (Probably)
    // RETURN: SALU (4 bits)

    if SXT { // if select extended instruction set
        if constants::HAS_EIS { // if has extended instruction set hardware
            (SALUM, ESALU)
        } else {
            (SALUM, 0) // I am not sure if this is a possible outcome
        }
    } else {
        (SALUM, SALU) // Regular pdp-11 capabilities, no extended instruction
    }
}
