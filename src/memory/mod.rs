use crate::mos6502::Bus;

pub struct CompositeBus<T: Bus> {
    components: Vec<T>,
    cached_size: u16,
}

pub struct MirrorBus<T: Bus> {
    components: Vec<T>,
    mirrors: u16,
}

impl<T: Bus> CompositeBus<T> {
    pub fn new(components: Vec<T>) -> CompositeBus<T> {
        CompositeBus {
            components,
            cached_size: 0,
        }
    }
}
