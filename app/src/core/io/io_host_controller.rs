use crate::core::console::components::Components;
use crate::core::console::console_state::ConsoleState;
use crate::core::io::device::logger::Logger;
use crate::core::io::device::sd_host_controller::SdHostController;
use crate::core::io::device::terminal_io::TerminalIO;
use crate::core::io::device::timer::Timer;
use crate::core::io::device::timer_core::TimerNum;
use crate::core::io::io_map_addr::IOMapAddr;
use crate::core::memory::mmu::Mmu;
use crate::core::traits::console::console::IConsoleState;
use crate::core::traits::io::device::io_serial::IIOSerial;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Clone)]
pub struct IOHostController {
    timers: Rc<RefCell<Timer>>,
    terminal: Rc<RefCell<TerminalIO>>,
    log: Rc<RefCell<Logger>>,
    mmu: Rc<RefCell<Mmu>>,
    sd_host: Rc<RefCell<SdHostController>>,
    console_state: Rc<RefCell<ConsoleState>>,
    console_components: Rc<RefCell<Components>>,
    pid: u16,
}

impl IOHostController {
    pub fn new(
        timers: Rc<RefCell<Timer>>,
        terminal: Rc<RefCell<TerminalIO>>,
        log: Rc<RefCell<Logger>>,
        mmu: Rc<RefCell<Mmu>>,
        sd_host: Rc<RefCell<SdHostController>>,
        console_state: Rc<RefCell<ConsoleState>>,
        console_components: Rc<RefCell<Components>>,
    ) -> Self {
        Self {
            timers,
            terminal,
            log,
            mmu,
            sd_host,
            console_state,
            console_components,
            pid: 0,
        }
    }

    pub fn input(&mut self, addr: IOMapAddr) -> u16 {
        match addr {
            IOMapAddr::TIMER0_COUNTER_CYCLE => self.timers.borrow().get_counter(TimerNum::TIMER0),
            IOMapAddr::TIMER0_FLAG_CTRL => self.get_timer_flag(TimerNum::TIMER0),
            IOMapAddr::TIMER1_COUNTER_CYCLE => self.timers.borrow().get_counter(TimerNum::TIMER1),
            IOMapAddr::TIMER1_FLAG_CTRL => self.get_timer_flag(TimerNum::TIMER1),
            IOMapAddr::FT232RL_RECEIVE_SERVE => self.terminal.borrow_mut().receive().into(),
            IOMapAddr::FT232RL_STAT_CTRL => self.get_ft232rl_status(),
            IOMapAddr::TECSERIAL_RECEIVE_SERVE => self.terminal.borrow_mut().receive().into(),
            IOMapAddr::TECSERIAL_STAT_CTRL => self.get_ft232rl_status(),
            IOMapAddr::MICROSD_STAT_CTRL => self.get_micro_sd_status(),
            IOMapAddr::MICROSD_MEMADDR => self.sd_host.borrow().get_mem_addr(),
            IOMapAddr::MICROSD_SECTORHIGH => self.sd_host.borrow().get_sec_addr_high(),
            IOMapAddr::MICROSD_SECTORLOW => self.sd_host.borrow().get_sec_addr_low(),
            IOMapAddr::PIO_MODE_00 => 0x01,
            IOMapAddr::RN4020_RECEIVE_SERVE => self.log.borrow_mut().receive().into(),
            IOMapAddr::RN4020_STAT_CTRL => self.get_rn4020_status(),
            IOMapAddr::RN4020_CONNECTION => 0x01,
            IOMapAddr::MMU_TLB0HIGH => self.mmu.borrow().get_tlb_high_8(0) as u16,
            IOMapAddr::MMU_TLB0LOW => self.mmu.borrow().get_tlb_low_16(0),
            IOMapAddr::MMU_TLB1HIGH => self.mmu.borrow().get_tlb_high_8(1) as u16,
            IOMapAddr::MMU_TLB1LOW => self.mmu.borrow().get_tlb_low_16(1),
            IOMapAddr::MMU_TLB2HIGH => self.mmu.borrow().get_tlb_high_8(2) as u16,
            IOMapAddr::MMU_TLB2LOW => self.mmu.borrow().get_tlb_low_16(2),
            IOMapAddr::MMU_TLB3HIGH => self.mmu.borrow().get_tlb_high_8(3) as u16,
            IOMapAddr::MMU_TLB3LOW => self.mmu.borrow().get_tlb_low_16(3),
            IOMapAddr::MMU_TLB4HIGH => self.mmu.borrow().get_tlb_high_8(4) as u16,
            IOMapAddr::MMU_TLB4LOW => self.mmu.borrow().get_tlb_low_16(4),
            IOMapAddr::MMU_TLB5HIGH => self.mmu.borrow().get_tlb_high_8(5) as u16,
            IOMapAddr::MMU_TLB5LOW => self.mmu.borrow().get_tlb_low_16(5),
            IOMapAddr::MMU_TLB6HIGH => self.mmu.borrow().get_tlb_high_8(6) as u16,
            IOMapAddr::MMU_TLB6LOW => self.mmu.borrow().get_tlb_low_16(6),
            IOMapAddr::MMU_TLB7HIGH => self.mmu.borrow().get_tlb_high_8(7) as u16,
            IOMapAddr::MMU_TLB7LOW => self.mmu.borrow().get_tlb_low_16(7),
            // IOMapAddr::MMU_TLB8HIGH => self.mmu.borrow().get_tlb_high_8(8).into(),
            // IOMapAddr::MMU_TLB8LOW => self.mmu.borrow().get_tlb_low_16(8).into(),
            // IOMapAddr::MMU_TLB9HIGH => self.mmu.borrow().get_tlb_high_8(9).into(),
            // IOMapAddr::MMU_TLB9LOW => self.mmu.borrow().get_tlb_low_16(9).into(),
            // IOMapAddr::MMU_TLB10HIGH => self.mmu.borrow().get_tlb_high_8(10).into(),
            // IOMapAddr::MMU_TLB10LOW => self.mmu.borrow().get_tlb_low_16(10).into(),
            // IOMapAddr::MMU_TLB11HIGH => self.mmu.borrow().get_tlb_high_8(11).into(),
            // IOMapAddr::MMU_TLB11LOW => self.mmu.borrow().get_tlb_low_16(11).into(),
            // IOMapAddr::MMU_TLB12HIGH => self.mmu.borrow().get_tlb_high_8(12).into(),
            // IOMapAddr::MMU_TLB12LOW => self.mmu.borrow().get_tlb_low_16(12).into(),
            // IOMapAddr::MMU_TLB13HIGH => self.mmu.borrow().get_tlb_high_8(13).into(),
            // IOMapAddr::MMU_TLB13LOW => self.mmu.borrow().get_tlb_low_16(13).into(),
            // IOMapAddr::MMU_TLB14HIGH => self.mmu.borrow().get_tlb_high_8(14).into(),
            // IOMapAddr::MMU_TLB14LOW => self.mmu.borrow().get_tlb_low_16(14).into(),
            // IOMapAddr::MMU_TLB15HIGH => self.mmu.borrow().get_tlb_high_8(15).into(),
            // IOMapAddr::MMU_TLB15LOW => self.mmu.borrow().get_tlb_low_16(15).into(),
            IOMapAddr::MMU_ERRORADDR_MMUON => self.mmu.borrow().get_error_addr(),
            IOMapAddr::MMU_ERRORCAUSE_00 => self.mmu.borrow_mut().get_error_cause().into(),
            IOMapAddr::MMU_PAGE_00 => self.mmu.borrow().get_error_page().into(),
            IOMapAddr::CONSOLE_DATASW_DATAREG => {
                self.console_components.borrow().get_data_switches() as u16
            }
            IOMapAddr::CONSOLE_ADDRREG_00 => self.console_state.borrow().get_mem_addr(),
            IOMapAddr::CONSOLE_ROTSW_00 => self.console_state.borrow().get_rot_sw().into(),
            IOMapAddr::CONSOLE_FUNCREG_00 => {
                self.console_components.borrow().get_func_switch().into()
            }
            _ => 0u16,
        }
    }

    pub fn output(&mut self, addr: IOMapAddr, val: u16) {
        match addr {
            IOMapAddr::TIMER0_COUNTER_CYCLE => {
                self.timers.borrow_mut().timer0.borrow_mut().set_cycle(val);
            }
            IOMapAddr::TIMER0_FLAG_CTRL => {
                self.set_timer_ctrl_flag(TimerNum::TIMER0, val);
            }
            IOMapAddr::TIMER1_COUNTER_CYCLE => {
                self.timers.borrow_mut().timer1.borrow_mut().set_cycle(val);
            }
            IOMapAddr::TIMER1_FLAG_CTRL => {
                self.set_timer_ctrl_flag(TimerNum::TIMER1, val);
            }
            IOMapAddr::FT232RL_RECEIVE_SERVE => {
                self.terminal.borrow_mut().send((val & 0x00ff) as u8);
            }
            IOMapAddr::FT232RL_STAT_CTRL => {
                self.terminal
                    .borrow_mut()
                    .set_sendable_intr_flag((val & 0x0080) != 0);
                self.terminal
                    .borrow_mut()
                    .set_receivable_intr_flag((val & 0x0040) != 0);
            }
            IOMapAddr::MICROSD_STAT_CTRL => {
                self.set_micro_sd_ctrl_flag(val);
            }
            IOMapAddr::MICROSD_MEMADDR => {
                self.sd_host.borrow_mut().set_mem_addr(val);
            }
            IOMapAddr::MICROSD_SECTORHIGH => {
                self.sd_host.borrow_mut().set_sec_addr_high(val);
            }
            IOMapAddr::MICROSD_SECTORLOW => {
                self.sd_host.borrow_mut().set_sec_addr_low(val);
            }
            IOMapAddr::RN4020_RECEIVE_SERVE => {
                self.log.borrow_mut().send(val as u8);
            }
            IOMapAddr::RN4020_STAT_CTRL => {
                self.log
                    .borrow_mut()
                    .set_receivable_intr_flag((val & 0x0040) != 0);
                self.log
                    .borrow_mut()
                    .set_sendable_intr_flag((val & 0x0080) != 0);
            }
            IOMapAddr::MMU_TLB0HIGH => {
                self.mmu.borrow_mut().set_tlb_high_8(0, val);
            }
            IOMapAddr::MMU_TLB0LOW => {
                self.mmu.borrow_mut().set_tlb_low_16(0, val);
            }
            IOMapAddr::MMU_TLB1HIGH => {
                self.mmu.borrow_mut().set_tlb_high_8(1, val);
            }
            IOMapAddr::MMU_TLB1LOW => {
                self.mmu.borrow_mut().set_tlb_low_16(1, val);
            }
            IOMapAddr::MMU_TLB2HIGH => {
                self.mmu.borrow_mut().set_tlb_high_8(2, val);
            }
            IOMapAddr::MMU_TLB2LOW => {
                self.mmu.borrow_mut().set_tlb_low_16(2, val);
            }
            IOMapAddr::MMU_TLB3HIGH => {
                self.mmu.borrow_mut().set_tlb_high_8(3, val);
            }
            IOMapAddr::MMU_TLB3LOW => {
                self.mmu.borrow_mut().set_tlb_low_16(3, val);
            }
            IOMapAddr::MMU_TLB4HIGH => {
                self.mmu.borrow_mut().set_tlb_high_8(4, val);
            }
            IOMapAddr::MMU_TLB4LOW => {
                self.mmu.borrow_mut().set_tlb_low_16(4, val);
            }
            IOMapAddr::MMU_TLB5HIGH => {
                self.mmu.borrow_mut().set_tlb_high_8(5, val);
            }
            IOMapAddr::MMU_TLB5LOW => {
                self.mmu.borrow_mut().set_tlb_low_16(5, val);
            }
            IOMapAddr::MMU_TLB6HIGH => {
                self.mmu.borrow_mut().set_tlb_high_8(6, val);
            }
            IOMapAddr::MMU_TLB6LOW => {
                self.mmu.borrow_mut().set_tlb_low_16(6, val);
            }
            IOMapAddr::MMU_TLB7HIGH => {
                self.mmu.borrow_mut().set_tlb_high_8(7, val);
            }
            IOMapAddr::MMU_TLB7LOW => {
                self.mmu.borrow_mut().set_tlb_low_16(7, val);
            }
            // IOMapAddr::MMU_TLB8HIGH => {
            //     self.mmu.borrow_mut().set_tlb_high_8(8, val as u32);
            // }
            // IOMapAddr::MMU_TLB8LOW => {
            //     if val == self.input(IOMapAddr::MMU_TLB8LOW) & !(1 << 12)  {
            //         gloo::console::log!("tlb reference flag cleared");
            //     }
            //     self.mmu.borrow_mut().set_tlb_low_16(8, val as u32);
            //     if self.mmu.borrow().get_tlb_low_16(8) & (1 << 12) == 0 {
            //         gloo::console::log!("ちゃんとcleared");
            //     }
            // }
            // IOMapAddr::MMU_TLB9HIGH => {
            //     self.mmu.borrow_mut().set_tlb_high_8(9, val as u32);
            // }
            // IOMapAddr::MMU_TLB9LOW => {
            //     self.mmu.borrow_mut().set_tlb_low_16(9, val as u32);
            // }
            // IOMapAddr::MMU_TLB10HIGH => {
            //     self.mmu.borrow_mut().set_tlb_high_8(10, val as u32);
            // }
            // IOMapAddr::MMU_TLB10LOW => {
            //     self.mmu.borrow_mut().set_tlb_low_16(10, val as u32);
            // }
            // IOMapAddr::MMU_TLB11HIGH => {
            //     self.mmu.borrow_mut().set_tlb_high_8(11, val as u32);
            // }
            // IOMapAddr::MMU_TLB11LOW => {
            //     self.mmu.borrow_mut().set_tlb_low_16(11, val as u32);
            // }
            // IOMapAddr::MMU_TLB12HIGH => {
            //     self.mmu.borrow_mut().set_tlb_high_8(12, val as u32);
            // }
            // IOMapAddr::MMU_TLB12LOW => {
            //     self.mmu.borrow_mut().set_tlb_low_16(12, val as u32);
            // }
            // IOMapAddr::MMU_TLB13HIGH => {
            //     self.mmu.borrow_mut().set_tlb_high_8(13, val as u32);
            // }
            // IOMapAddr::MMU_TLB13LOW => {
            //     self.mmu.borrow_mut().set_tlb_low_16(13, val as u32);
            // }
            // IOMapAddr::MMU_TLB14HIGH => {
            //     self.mmu.borrow_mut().set_tlb_high_8(14, val as u32);
            // }
            // IOMapAddr::MMU_TLB14LOW => {
            //     self.mmu.borrow_mut().set_tlb_low_16(14, val as u32);
            // }
            // IOMapAddr::MMU_TLB15HIGH => {
            //     self.mmu.borrow_mut().set_tlb_high_8(15, val as u32);
            // }
            // IOMapAddr::MMU_TLB15LOW => {
            //     self.mmu.borrow_mut().set_tlb_low_16(15, val as u32);
            // }
            IOMapAddr::MMU_00_IPLBANK => {
                if val & 0x0001 != 0 {
                    self.mmu.borrow_mut().detach_ipl();
                }
            }
            IOMapAddr::MMU_ERRORADDR_MMUON => {
                if val & 0x0001 != 0 {
                    self.mmu.borrow_mut().enable();
                }
            }
            IOMapAddr::CONSOLE_DATASW_DATAREG => {
                self.console_components.borrow_mut().set_led_lamps(val);
            }
            _ => {}
        }
        if addr as u16 == 0x38 {
            self.pid = addr as u16
        }
    }

    fn get_timer_flag(&self, timer_num: TimerNum) -> u16 {
        let binding = self.timers.borrow();
        let is_matched = match timer_num {
            TimerNum::TIMER0 => &binding.timer0,
            TimerNum::TIMER1 => &binding.timer1,
        }
        .borrow()
        .is_matched();
        if is_matched {
            return 0x8000;
        }
        0u16
    }

    fn get_ft232rl_status(&self) -> u16 {
        let mut val = 0u16;
        if self.terminal.borrow().is_writable() {
            val |= 0x0080;
        }
        if self.terminal.borrow().is_readable() {
            val |= 0x0040;
        }
        val
    }

    fn get_rn4020_status(&self) -> u16 {
        let mut val = 0u16;
        if self.log.borrow().is_writable() {
            val |= 0x0080;
        }
        if self.log.borrow().is_readable() {
            val |= 0x0040;
        }
        if !self.sd_host.borrow().is_sd_loaded() {
            val |= 0x0001;
        }
        val
    }

    fn get_micro_sd_status(&self) -> u16 {
        let mut val = 0u16;
        if self.sd_host.borrow().is_idle() {
            val |= 0x0080;
        }
        if self.sd_host.borrow().is_error_occurred() {
            val |= 0x0040;
        }
        // SDカードは基本的に挿入されているものとして動作する
        val
    }

    fn set_timer_ctrl_flag(&mut self, timer_num: TimerNum, val: u16) {
        if timer_num == TimerNum::TIMER0 {
            self.timers
                .borrow()
                .timer0
                .borrow_mut()
                .set_intr_flag((val & 0x8000) != 0);
            if (val & 0x0001) != 0 {
                self.timers.borrow_mut().start_timer(TimerNum::TIMER0);
            } else {
                self.timers.borrow_mut().stop_timer(TimerNum::TIMER0);
            }
        } else {
            self.timers
                .borrow()
                .timer1
                .borrow_mut()
                .set_intr_flag((val & 0x8000) != 0);
            if (val & 0x0001) != 0 {
                self.timers.borrow_mut().start_timer(TimerNum::TIMER1);
            } else {
                self.timers.borrow_mut().stop_timer(TimerNum::TIMER1);
            }
        }
    }

    fn set_micro_sd_ctrl_flag(&mut self, val: u16) {
        self.sd_host.borrow_mut().set_intr_flag((val & 0x0080) != 0);
        if (val & 0x0004) != 0 {
            self.sd_host.borrow_mut().init();
        }
        if (val & 0x0002) != 0 {
            self.sd_host.borrow_mut().start_reading();
        }
        if (val & 0x0001) != 0 {
            self.sd_host.borrow_mut().start_writing();
        }
    }

    pub fn get_pid(&self) -> u16 {
        self.pid
    }
}
