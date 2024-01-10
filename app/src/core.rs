pub mod console {
    pub mod button;
    pub mod components;
    pub mod console;
    pub mod console_state;
    pub mod led;
    pub mod switch;
}

pub mod cpu {
    pub mod alu;
    pub mod cpu;
    pub mod instruction;
    pub mod psw;
    pub mod register;
    pub mod consts {
        pub mod addr_mode;
        pub mod flags;
        pub mod opcode;
        pub mod reg_num;
    }
}

pub mod error {
    pub mod sd_io_error;
    pub mod tlb_error;
}

pub mod interrupt {
    pub mod interrupt;
    pub mod intr_controller;
}

pub mod io {
    pub mod io_host_controller;
    pub mod io_map_addr;
    pub mod device {
        pub mod logger;
        pub mod sd_host_controller;
        pub mod terminal_io;
        pub mod timer;
        pub mod timer_core;
    }
}

pub mod memory {
    pub mod memory;
    pub mod mmu;
    pub mod tlb;
}

pub mod traits {
    pub mod console {
        pub mod console;
    }
    pub mod io {
        pub mod device {
            pub mod io_serial;
            pub mod timer;
        }
    }
}

pub mod consts;
pub mod ipl;
pub mod tac;
pub mod tac_wrap;
