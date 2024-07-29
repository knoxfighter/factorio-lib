use std::io::Read;

use super::{v014 as previous, FactorioVersion, RuntimeVersion};
use crate::{reader::FactorioNumber, saves::Mod};

pub type Latest = V015;

pub struct V015;

impl FactorioVersion for V015 {
    type PreviousVersion = previous::Latest;

    fn read_mod(runtime_version: &RuntimeVersion, reader: &mut impl Read) -> std::io::Result<Mod> {
        let mut m = Self::PreviousVersion::read_mod(runtime_version, reader)?;
        m.crc = Some(u32::read_num(reader)?);
        Ok(m)
    }
}
