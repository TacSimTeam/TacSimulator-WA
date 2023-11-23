pub trait IConsoleState {
    fn push_sw_value_to_reg(&mut self, val: u8);
    fn write_mem_data(&mut self, data: u16);
    fn get_mem_addr(&self) -> u16;
    fn get_mem_data(&self) -> u16;
    fn read_mem_data(&self) -> u16;
    fn read_reg(&self) -> u16;
    fn get_rot_sw(&self) -> u8;
}

pub trait IConsoleStateAction {
    fn left_allow_btn_event(&mut self, _val: u8);
    fn right_allow_btn_event(&mut self, _val: u8);
    fn seta_btn_event(&mut self, val: u8);
    fn inca_btn_event(&mut self, _val: u8);
    fn deca_btn_event(&mut self, _val: u8);
    fn write_btn_event(&mut self, val: u8);
}

pub trait ITacEvent {
    fn run_btn_event(&mut self);
    fn reset_btn_event(&mut self);
    fn stop_btn_event(&mut self);
}
