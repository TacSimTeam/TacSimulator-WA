#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum OPCode {
    NOP = 0x00,
    LD = 0x01,
    ST = 0x02,
    ADD = 0x03,
    SUB = 0x04,
    CMP = 0x05,
    AND = 0x06,
    OR = 0x07,
    XOR = 0x08,
    ADDS = 0x09,
    MUL = 0x0a,
    DIV = 0x0b,
    MOD = 0x0c,
    SHLA = 0x10,
    SHLL = 0x11,
    SHRA = 0x12,
    SHRL = 0x13,

    JMP = 0x14,

    CALL = 0x15,
    IN = 0x16,
    OUT = 0x17,

    PUSH_POP = 0x18,

    RET_RETI = 0x1a,
    SVC = 0x1e,
    HALT = 0x1f,
}

#[allow(dead_code)]
pub enum JMP {
    JZ,
    JC,
    JM,
    JO,
    JGT,
    JGE,
    JLE,
    JLT,
    JNZ,
    JNC,
    JNM,
    JNO,
    JHI,
    JLS,
    JMP,
}
