/****************
 *  74181 ALU   *
 ****************/
pub fn alu_slice(
    A: [bool; 4],
    B: [bool; 4],
    S: [bool; 4],
    CIN: bool, M: bool) -> ([bool; 4], bool, bool, bool, bool){
        let [A0, A1, A2, A3] = A;
        let [B0, B1, B2, B3] = B;
        let [S0, S1, S2, S3] = S;

        let p0 = alu_stage_product(A0, B0, S0, S1);
        let p1 = alu_stage_product(A1, B1, S0, S1);
        let p2 = alu_stage_product(A2, B2, S0, S1);
        let p3 = alu_stage_product(A3, B3, S0, S1);

        let c0 = alu_state_carry(A0, B0, S2, S3);
        let c1 = alu_state_carry(A1, B1, S2, S3);
        let c2 = alu_state_carry(A2, B2, S2, S3);
        let c3 = alu_state_carry(A3, B3, S2, S3);

        let g = alu_g(p0, p1, p2, p3, c1, c2, c3);
        let p = !(c0 && c1 && c2 && c3);
        let carry_out = alu_carry_out(CIN, c0, c1, c2, c3, g);

        // Output F3
        let f3_g1 = !M && p2;
        let f3_g2 = !M && p1 && c2;
        let f3_g3 = !M && p0 && c1 && c2;
        let f3_g4 = !M && CIN && c0 && c1 && c2;
        let f3_o = !p3 && c3;
        let f3 = !(f3_g1 || f3_g2 || f3_g3 || f3_g4) ^ f3_o;

        // Output F2
        let f2_g1 = !M && p1;
        let f2_g2 = !M && p0 && c1;
        let f2_g3 = !M && CIN && c0 && c1;
        let f2_o = !p2 && c2;
        let f2 = !(f2_g1 || f2_g2 || f2_g3) ^ f2_o;

        // Output F1
        let f1_g1 = !M && p0;
        let f1_g2 = !M && CIN && c0;
        let f1_o = !p1 && c1;
        let f1 = !(f1_g1 || f1_g2) ^ f1_o;

        // Output F0
        let f0_g1 = !(!M && CIN); 
        let f0_o = !p0 && c0;
        let f0 = f0_g1 ^ f0_o;

        let a_qe_b = f0 && f1 && f2 && f3;

        ([f0, f1, f2, f3], carry_out, a_qe_b, g, p)
}

fn alu_state_carry(A: bool, B: bool, S2: bool, S3: bool) -> bool {
    !((A && B && S3) || (A && !B && S2))
}

fn alu_stage_product(A: bool, B: bool, S0: bool, S1: bool) -> bool {
    !(A || (S0 && B) || (S1 && !B))
}

fn alu_g(P0: bool, P1: bool, P2: bool, P3: bool, C1: bool, C2: bool, C3: bool) -> bool {
    let g1 = P0 && C1 && C2 && C3;
    let g2 = P1 && C2 && C3;
    let g3 = P2 && C3;
    let g4 = P3;
    !(g1 || g2 || g3 || g4)
}

fn alu_carry_out(C: bool, C0: bool, C1: bool, C2: bool, C3: bool, G: bool) -> bool {
    (C && C0 && C1 && C2 && C3) || !G
}


/******************
 *  74182 Carry   *
 ******************/
pub fn look_ahead_carry_unit_0(C: bool, G0: bool, P0: bool) -> bool {
   !((G0 && P0) || (!C && G0))
}

pub fn look_ahead_carry_unit_1(C: bool, G0: bool, G1: bool, P0: bool, P1: bool) -> bool {
    !((G1 && P1) || (G0 && G1 && P0) || (!C && G0 && G1))
}

pub fn look_ahead_carry_unit_2(C: bool, G0: bool, G1: bool, G2: bool, P0: bool, P1: bool, P2: bool) -> bool {
    !((G2 && P2) || (G1 && G2 && P1) || (G0 && G1 && G2 && P0) || (!C && G0 && G1 && G2))
}

pub fn look_ahead_carry_unit_3(G0: bool, G1: bool, G2: bool, G3: bool, P0: bool, P1: bool, P2: bool, P3: bool) -> (bool, bool) {
    let g = (G3 && P3) || (G2 && G3 && P2) || (G1 && G2 && G3 && P1) || (G0 && G1 && G2 && G3);
    let p = P0 || P1 || P2 || P3;
    (g, p)
}