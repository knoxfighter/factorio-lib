use std::io;
use std::io::Read;

use crate::reader::FactorioNumber;

use super::{FactorioVersion, v014 as previous};

pub type Latest = V015;

pub struct V015;

impl FactorioVersion for V015 {
    type PreviousVersion = previous::Latest;
    
    fn read_mod_crc(reader: &mut impl Read) -> io::Result<Option<u32>> {
        Ok(Some(u32::read_num(reader)?))
    }
}
