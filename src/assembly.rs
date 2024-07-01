use crate::cpu::Opcode;

pub static COMMANDS: &[(Opcode, u16, u16, u16)] = &[
    (Opcode::LDI, 0, 0, 0),   // R0 = 0
    (Opcode::LDI, 1, 1, 0),   // R1 = 1
    (Opcode::LDI, 2, 1, 0),   // R2 = 1 单位1
    (Opcode::LDI, 3, 100, 0), // R3 = 100

    (Opcode::ADD, 0, 0, 1),   // R0 += R1
    (Opcode::LTE, 4, 1, 3),   // R4 = R1 <= R3
    (Opcode::ADD, 1, 1, 2),   // R1 += R2
    (Opcode::JNE, 4, 3, 0),   // IF R4!=0 GOTO 4

    (Opcode::END, 0, 0, 0),
];