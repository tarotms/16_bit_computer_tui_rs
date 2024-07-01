use crate::cpu::Opcode;

pub static PROGRAM_SUM_OF_0_TO_100: &[(Opcode, u16, u16, u16)] = &[
    (Opcode::LDI, 0, 0, 0),   // 0.R0 = 0             
    (Opcode::LDI, 1, 1, 0),   // 1.R1 = 1             
    (Opcode::LDI, 2, 1, 0),   // 2.R2 = 1 unit        
    (Opcode::LDI, 3, 100, 0), // 3.R3 = 100           
    (Opcode::ADD, 0, 0, 1),   // 4.R0 += R1 ◄────────────┐
    (Opcode::LTE, 4, 1, 3),   // 5.R4 = R1 <= R3         │
    (Opcode::ADD, 1, 1, 2),   // 6.R1 += R2              │
    (Opcode::JNE, 4, 4, 0),   // 7.IF R4!=0 GOTO 4 ──────┘
    (Opcode::END, 0, 0, 0),
];

pub static PROGRAM_FIBONACCI: &[(Opcode, u16, u16, u16)] = &[
    (Opcode::LDI, 0, 0, 0),   // 0.R0 = 0
    (Opcode::LDI, 1, 1, 0),   // 1.R1 = 1
    (Opcode::LDI, 2, 2, 0),   // 2.R2 = 2
    (Opcode::LDI, 3, 10, 0),  // 3.R3 = 10
    (Opcode::LDI, 5, 1, 0),   // 4.R5 = 1 unit ◄─────────┐
    (Opcode::ADD, 4, 0, 1),   // 5.R4 = R0 + R1          │
    (Opcode::MOV, 0, 1, 0),   // 6.R0 = R1               │
    (Opcode::MOV, 1, 4, 0),   // 7.R1 = R4               │
    (Opcode::LTE, 6, 2, 3),   // 8.R6 = R2 <= R3         │
    (Opcode::ADD, 2, 2, 5),   // 9.R2 = R2 + R5          │
    (Opcode::JNE, 6, 5, 0),   //10.IF R6!=0 GOTO 4 ──────┘
    (Opcode::END, 0, 0, 0),   
];