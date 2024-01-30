use crate::core::console::button::ButtonEventType::TacEvent;
use crate::core::console::button::ButtonName;
use crate::core::console::components::Components;
use crate::core::console::console_state::ConsoleState;
use crate::core::cpu::cpu::Cpu;
use crate::core::cpu::psw::Psw;
use crate::core::cpu::register::Register;
use crate::core::interrupt::intr_controller::IntrController;
use crate::core::io::device::logger::Logger;
use crate::core::io::device::sd_host_controller::SdHostController;
use crate::core::io::device::terminal_io::TerminalIO;
use crate::core::io::device::timer::Timer;
use crate::core::io::device::timer_core::TimerNum;
use crate::core::io::io_host_controller::IOHostController;
use crate::core::memory::memory::Memory;
use crate::core::memory::mmu::Mmu;
use crate::core::traits::console::console::ITacEvent;
use crate::util::timeout::{clear_timeout, Timeout};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::KeyboardEvent;
use yew::{NodeRef, Properties};

#[wasm_bindgen(js_namespace = Date)]
extern "C" {
    fn now() -> f64;
}

#[derive(PartialEq, Clone)]
pub struct Tac {
    memory: Rc<RefCell<Memory>>,
    mmu: Rc<RefCell<Mmu>>,
    intr_host: Rc<RefCell<IntrController>>,
    cpu: Cpu,
    psw: Rc<RefCell<Psw>>,
    register: Rc<RefCell<Register>>,
    io_host: Rc<RefCell<IOHostController>>,
    timers: Rc<RefCell<Timer>>,
    terminal_io: Rc<RefCell<TerminalIO>>,
    logger: Rc<RefCell<Logger>>,
    sd_host: Rc<RefCell<SdHostController>>,
    console_state: Rc<RefCell<ConsoleState>>,
    components: Rc<RefCell<Components>>,
    terminal: NodeRef,
    input: NodeRef,
    cpu_event: Option<i32>,
}

#[derive(Properties, PartialEq)]
pub struct TacProps {
    pub dmg: Rc<RefCell<Vec<u8>>>,
}

impl Tac {
    pub fn new(
        dmg: Rc<RefCell<Vec<u8>>>,
        memory: Rc<RefCell<Memory>>,
        psw: Rc<RefCell<Psw>>,
        register: Rc<RefCell<Register>>,
        console_state: Rc<RefCell<ConsoleState>>,
        components: Rc<RefCell<Components>>,
        terminal: NodeRef,
        input: NodeRef,
        logger_textarea: NodeRef,
    ) -> Self {
        let intr_host = IntrController::new();
        let intr_host_clone = Rc::new(RefCell::new(intr_host));
        let mmu = Rc::new(RefCell::new(Mmu::new(
            Rc::clone(&memory),
            Rc::clone(&intr_host_clone),
            Rc::clone(&psw),
        )));
        let logger = Rc::new(RefCell::new(Logger::new(
            Rc::clone(&intr_host_clone),
            input.clone(),
            logger_textarea.clone(),
        )));
        let timers = Rc::new(RefCell::new(Timer::new(Rc::clone(&intr_host_clone))));
        let terminal_io = Rc::new(RefCell::new(TerminalIO::new(
            terminal.clone(),
            Rc::clone(&intr_host_clone),
        )));
        let sd_host = Rc::new(RefCell::new(SdHostController::new(
            Rc::clone(&memory),
            Rc::clone(&intr_host_clone),
            dmg,
        )));
        let io_host = Rc::new(RefCell::new(IOHostController::new(
            Rc::clone(&timers),
            Rc::clone(&terminal_io),
            Rc::clone(&logger),
            Rc::clone(&mmu),
            Rc::clone(&sd_host),
            Rc::clone(&console_state),
            Rc::clone(&components),
        )));
        let cpu = Cpu::new(
            Rc::clone(&mmu),
            Rc::clone(&register),
            Rc::clone(&psw),
            Rc::clone(&intr_host_clone),
            Rc::clone(&io_host),
        );
        Self {
            memory,
            mmu,
            intr_host: intr_host_clone,
            cpu,
            psw,
            register,
            io_host,
            timers,
            terminal_io,
            logger,
            sd_host,
            console_state,
            components,
            terminal,
            input,
            cpu_event: None,
        }
    }

    pub fn init(&mut self) {
        self.mmu.borrow_mut().load_ipl();
        self.init_btn_action();
        self.reset();
    }

    pub fn run(&mut self) {
        self.timers.borrow_mut().restart_timer(TimerNum::TIMER0);
        self.timers.borrow_mut().restart_timer(TimerNum::TIMER1);

        let start_time = now();
        loop {
            let _inst = self.cpu.run();
            if self.components.borrow().get_break_switch()
            // && self.psw.borrow().get_pc() == self.break_addr
            {
                // self.update();
                self.stop();
                return;
            }

            if self.components.borrow().get_step_switch() {
                // self.update();
                self.stop();
                return;
            }
            let end_time = now();
            if end_time - start_time > 15.0 {
                self.stop();
                let mut clone = self.clone();
                let timeout = Timeout::new(
                    Closure::new(move || {
                        clone.run();
                    }),
                    0,
                );
                self.cpu_event = Some(timeout.id);
                return;
            }
        }
    }

    fn stop(&mut self) {
        self.timers.borrow_mut().pause_timer(TimerNum::TIMER0);
        self.timers.borrow_mut().pause_timer(TimerNum::TIMER1);
        if let Some(id) = self.cpu_event.take() {
            clear_timeout(id);
        }
    }

    fn reset(&mut self) {
        self.stop();
        self.cpu.reset();
        self.psw.borrow_mut().reset();
        self.register.borrow_mut().reset();
        self.mmu.borrow_mut().reset();
        self.intr_host.borrow_mut().reset();
        self.timers.borrow_mut().timer0.borrow_mut().reset();
        self.timers.borrow_mut().timer1.borrow_mut().reset();
        self.terminal_io.borrow_mut().reset();
        self.logger.borrow_mut().reset();
        self.sd_host.borrow_mut().reset();

        self.mmu.borrow_mut().load_ipl();
        // self.update();
    }

    fn init_btn_action(&mut self) {
        self.components.borrow_mut().buttons[ButtonName::RunBtn]
            .set_event(TacEvent(ITacEvent::run_btn_event));
        self.components.borrow_mut().buttons[ButtonName::ResetBtn]
            .set_event(TacEvent(ITacEvent::reset_btn_event));
        self.components.borrow_mut().buttons[ButtonName::StopBtn]
            .set_event(TacEvent(ITacEvent::stop_btn_event));
    }

    pub fn terminal_on_keydown(&self, e: KeyboardEvent) {
        self.terminal_io.borrow_mut().input_key_down(e);
    }

    pub fn is_halt(&self) -> bool {
        self.cpu.is_halt()
    }
}

impl ITacEvent for Tac {
    fn run_btn_event(&mut self) {
        self.console_state.borrow_mut().set_run_flag(true);
        self.run();
    }

    fn reset_btn_event(&mut self) {
        self.console_state.borrow_mut().set_run_flag(false);
        self.reset();
    }

    fn stop_btn_event(&mut self) {
        self.console_state.borrow_mut().set_run_flag(false);
        self.stop();
    }
}
