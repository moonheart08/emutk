use crate::mcbus::Device;

pub struct SerialOut {
    
}

impl Device for SerialOut {
    fn read_u32(&mut self, addr: usize) -> (Cycles, u32)
    {
        
    }

    fn write_u32(&mut self, addr: usize, data: u32) -> Cycles
    {

    }

    fn read_u16(&mut self, addr: usize) -> (Cycles, u16)
    {

    }

    fn write_u16(&mut self, addr: usize, data: u16) -> Cycles
    {

    }

    fn read_u8(&mut self, addr: usize) -> (Cycles, u8)
    {

    }

    fn write_u8(&mut self, addr: usize, data: u8) -> Cycles 
    {

    }

    fn device_active(&mut self) -> bool {
        true
    }

    fn device_origin(&mut self) -> DeviceOrigin {
        DeviceOrigin::Native
    }

    fn interrupt_pending(&mut self) -> bool {
        // nothing yet.
    }

    fn tick(&mut self) {
        // nothing yet
    }

    fn serialize(&mut self, space: &mut [u8]) {
        unimplemented!();
    }
}