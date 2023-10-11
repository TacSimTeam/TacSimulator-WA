use crate::core::io::device::logger::Logger;
use crate::core::io::device::sd_host_controller::SdHostController;
use crate::core::io::device::terminal_io::TerminalIO;
use crate::core::io::device::timer::Timer;
use crate::core::io::device::timer_core::TimerNum;
use crate::core::io::io_map_addr::IOMapAddr;
use crate::core::memory::mmu::Mmu;
use crate::core::traits::io::device::io_serial::IIOSerial;

pub struct IOHostController {
    timers: Timer,
    terminal: TerminalIO,
    log: Logger,
    mmu: Mmu,
    sd_host: SdHostController, // TODO consoleの追加
}

impl IOHostController {
    pub fn input(&mut self, addr: IOMapAddr) -> u16 {
        match addr {
            IOMapAddr::TIMER0_COUNTER_CYCLE => self.timers.get_counter(TimerNum::TIMER0),
            IOMapAddr::TIMER0_FLAG_CTRL => self.get_timer_flag(TimerNum::TIMER0),
            IOMapAddr::TIMER1_COUNTER_CYCLE => self.timers.get_counter(TimerNum::TIMER1),
            IOMapAddr::TIMER1_FLAG_CTRL => self.get_timer_flag(TimerNum::TIMER1),
            IOMapAddr::FT232RL_RECEIVE_SERVE => self.terminal.receive(),
            IOMapAddr::FT232RL_STAT_CTRL => self.get_ft232rl_status(),
            IOMapAddr::TECSERIAL_RECEIVE_SERVE => {}
            IOMapAddr::TECSERIAL_STAT_CTRL => {}
            IOMapAddr::MICROSD_STAT_CTRL => self.get_micro_sd_status(),
            IOMapAddr::MICROSD_MEMADDR => self.sd_host.get_mem_addr(),
            IOMapAddr::MICROSD_SECTORHIGH => self.sd_host.get_sec_addr_high(),
            IOMapAddr::MICROSD_SECTORLOW => self.sd_host.get_sec_addr_low(),
            IOMapAddr::PIO_IN_OUT => {}
            IOMapAddr::PIO_00_ADC => {}
            IOMapAddr::PIO_00_OUTEX => {}
            IOMapAddr::PIO_MODE_00 => {}
            IOMapAddr::SPI_SHIFT => {}
            IOMapAddr::SPI_STAT_SCLKFREQ => {}
            IOMapAddr::IOINTR_00_MASK => {}
            IOMapAddr::IOINTR_00_XOR => {}
            IOMapAddr::RN4020_RECEIVE_SERVE => {}
            IOMapAddr::RN4020_STAT_CTRL => {}
            IOMapAddr::RN4020_00_COMMAND => {}
            IOMapAddr::RN4020_CONNECTION => {}
            IOMapAddr::TEC_DATALAMP_00 => {}
            IOMapAddr::TEC_00_DATASW => {}
            IOMapAddr::TEC_00_FUNCSW => {}
            IOMapAddr::TEC_STAT_CTRL => {}
            IOMapAddr::MMU_TLB0HIGH => {}
            IOMapAddr::MMU_TLB0LOW => {}
            IOMapAddr::MMU_TLB1HIGH => {}
            IOMapAddr::MMU_TLB1LOW => {}
            IOMapAddr::MMU_TLB2HIGH => {}
            IOMapAddr::MMU_TLB2LOW => {}
            IOMapAddr::MMU_TLB3HIGH => {}
            IOMapAddr::MMU_TLB3LOW => {}
            IOMapAddr::MMU_TLB4HIGH => {}
            IOMapAddr::MMU_TLB4LOW => {}
            IOMapAddr::MMU_TLB5HIGH => {}
            IOMapAddr::MMU_TLB5LOW => {}
            IOMapAddr::MMU_TLB6HIGH => {}
            IOMapAddr::MMU_TLB6LOW => {}
            IOMapAddr::MMU_TLB7HIGH => {}
            IOMapAddr::MMU_TLB7LOW => {}
            IOMapAddr::MMU_TLB8HIGH => {}
            IOMapAddr::MMU_TLB8LOW => {}
            IOMapAddr::MMU_TLB9HIGH => {}
            IOMapAddr::MMU_TLB9LOW => {}
            IOMapAddr::MMU_TLB10HIGH => {}
            IOMapAddr::MMU_TLB10LOW => {}
            IOMapAddr::MMU_TLB11HIGH => {}
            IOMapAddr::MMU_TLB11LOW => {}
            IOMapAddr::MMU_TLB12HIGH => {}
            IOMapAddr::MMU_TLB12LOW => {}
            IOMapAddr::MMU_TLB13HIGH => {}
            IOMapAddr::MMU_TLB13LOW => {}
            IOMapAddr::MMU_TLB14HIGH => {}
            IOMapAddr::MMU_TLB14LOW => {}
            IOMapAddr::MMU_TLB15HIGH => {}
            IOMapAddr::MMU_TLB15LOW => {}
            IOMapAddr::MMU_00_IPLBANK => {}
            IOMapAddr::MMU_ERRORADDR_MMUON => {}
            IOMapAddr::MMU_ERRORCAUSE_00 => {}
            IOMapAddr::MMU_PAGE_00 => {}
            IOMapAddr::CONSOLE_DATASW_DATAREG => {}
            IOMapAddr::CONSOLE_ADDRREG_00 => {}
            IOMapAddr::CONSOLE_ROTSW_00 => {}
            IOMapAddr::CONSOLE_FUNCREG_00 => {}
        }
    }

    fn get_timer_flag(&self, timer_num: TimerNum) -> u16 {
        let is_matched = match timer_num {
            TimerNum::TIMER0 => self.timers.timer0.lock().unwrap().is_matched(),
            TimerNum::TIMER1 => self.timers.timer1.lock().unwrap().is_matched(),
        };
        if is_matched {
            return 0x8000;
        }
        0u16
    }

    fn get_ft232rl_status(&self) -> u16 {
        let mut val = 0u16;
        if self.terminal.is_writable() {
            val |= 0x0080;
        }
        if self.terminal.is_readable() {
            val |= 0x0040;
        }
        val
    }

    fn get_rn4020_status(&self) -> u16 {
        let mut val = 0u16;
        if self.log.is_writable() {
            val |= 0x0080;
        }
        if self.log.is_readable() {
            val |= 0x0040;
        }
        val
    }

    fn get_micro_sd_status(&self) -> u16 {
        let mut val = 0u16;
        if self.sd_host.is_idle() {
            val |= 0x0080;
        }
        if self.sd_host.is_error_occurred() {
            val |= 0x0040;
        }
        // SDカードは基本的に挿入されているものとして動作する
        val
    }
}
