pub trait TimerEvent {
    fn routine(&mut self);
    fn is_continue(&self) -> bool;
}
