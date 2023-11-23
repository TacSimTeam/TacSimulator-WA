use crate::core::console::components::Components;
use crate::core::console::console::ConsoleWrap as Console;
use crate::core::console::console_state::ConsoleState;
use crate::core::cpu::psw::Psw;
use crate::core::cpu::register::Register;
use crate::core::memory::memory::Memory;
use crate::core::tac::{Tac, TacProps};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use yew::{
    function_component, html, use_state, Component, Context, ContextProvider, Html, NodeRef,
    Reducible,
};

pub struct TacWrap {
    memory: Rc<RefCell<Memory>>,
    psw: Rc<RefCell<Psw>>,
    register: Rc<RefCell<Register>>,
    console_state: Rc<RefCell<ConsoleState>>,
    components: Rc<RefCell<Components>>,
    tac: Rc<RefCell<Tac>>,
    terminal: NodeRef,
    input: NodeRef,
}

impl TacWrap {
    fn new(dmg: Rc<RefCell<Vec<u8>>>) -> Self {
        let memory = Rc::new(RefCell::new(Memory::new()));
        let psw = Rc::new(RefCell::new(Psw::new()));
        let register = Rc::new(RefCell::new(Register::new(Rc::clone(&psw))));

        let console_state = Rc::new(RefCell::new(ConsoleState::new(
            Rc::clone(&memory),
            Rc::clone(&psw),
            Rc::clone(&register),
        )));
        let components = Rc::new(RefCell::new(Components::new()));
        let terminal = NodeRef::default();
        let input = NodeRef::default();
        let tac = Rc::new(RefCell::new(Tac::new(
            dmg,
            Rc::clone(&memory),
            Rc::clone(&psw),
            Rc::clone(&register),
            Rc::clone(&console_state),
            Rc::clone(&components),
            terminal.clone(),
            input.clone(),
        )));
        Self {
            memory,
            psw,
            register,
            console_state,
            components,
            tac,
            terminal,
            input,
        }
    }
}

impl Component for TacWrap {
    type Message = ();
    type Properties = TacProps;

    fn create(ctx: &Context<Self>) -> Self {
        let TacProps { dmg } = ctx.props();
        TacWrap::new(Rc::clone(dmg))
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let tac = &self.tac;
        return html! {
            <>
                <ContextProvider<Rc<RefCell<Tac>>> context={Rc::clone(tac)} >
                    <section class="layout">
                        <div class="console-area">
                            <Console state={Rc::clone(&self.console_state)} component={Rc::clone(&self.components)} />
                            <input ref={&self.input.clone()} type="checkbox"/>
                        </div>
                        <div class="terminal-area">
                            <textarea ref={&self.terminal.clone()}></textarea>
                        </div>
                    </section>
                </ContextProvider<Rc<RefCell<Tac>>>>
            </>
        };
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            gloo::console::log!("rendered");
        } else {
            self.tac.borrow_mut().init();
        }
    }
}
