use std::io::Read;
use crate::reader::FactorioNumber;
use super::{v016 as previous, FactorioVersion};

pub type Latest = V017;

pub struct V017;

impl FactorioVersion for V017 {
    type PreviousVersion = previous::Latest;

    fn read_quality_version(reader: &mut impl Read) -> std::io::Result<Option<u8>> {
        let res = u8::read_num(reader)?;
        Ok(Some(res))
    }
}