use std::{io, io::Read};

use super::{v014 as previous, FactorioVersion};
use crate::reader::FactorioNumber;

pub type Latest = V015;

#[derive(Default)]
pub struct V015;

impl FactorioVersion for V015 {
    type PreviousVersion = previous::Latest;

    fn read_mod_crc(&self, reader: &mut impl Read) -> io::Result<Option<u32>> {
        Ok(Some(u32::read_num(reader)?))
    }
}
