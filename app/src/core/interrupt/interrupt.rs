#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq)]
pub enum Interrupt {
    TIMER0,
    TIMER1,
    RN4020_RECEIVED,
    RN4020_SENT,
    FT232RL_RECEIVED,
    FT232RL_SENT,
    TEC_RECEIVED,
    TEC_SENT,
    MICRO_SD,
    PIO,
    EXCP_TLB_MISS,
    EXCP_MEMORY_ERROR,
    EXCP_ZERO_DIV,
    EXCP_PRIV_ERROR,
    EXCP_OP_UNDEFINED,
    EXCP_SVC,
}
