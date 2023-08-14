use crate::mos6502::Bus;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompositeBusError {
    #[error("mirror ranges have different magnitudes: {0} and {1}")]
    MirrorRangeMagnitude(u16, u16),
    #[error("mirror ranges `{0}`-`{1}` and `{2}`-`{3}` overlap")]
    MirrorRangeOverlap(u16, u16, u16, u16),
}

pub type MirrorRange = ((u16, u16), (u16, u16));

pub struct CompositeBus<T: Bus> {
    components: Vec<T>,
    mirror_address_ranges: Vec<MirrorRange>,
}

fn ranges_overlap(ranges: &[MirrorRange]) -> bool {
    for i in 0..ranges.len() {
        for j in (i + 1)..ranges.len() {
            if ranges[i].0 <= ranges[j].1 && ranges[i].1 >= ranges[j].0 {
                return true;
            }
        }
    }
    false
}

impl<T: Bus> CompositeBus<T> {
    pub fn new(
        components: Vec<T>,
        mirror_address_ranges: Vec<MirrorRange>,
    ) -> Result<CompositeBus<T>, CompositeBusError> {
        Ok(CompositeBus {
            components,
            mirror_address_ranges,
        })
    }
}
