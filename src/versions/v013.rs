use std::{io, io::Read};

use crate::reader::{self, FactorioNumber};

use super::FactorioVersion;

pub type Latest = V013;

pub struct V013;

impl FactorioVersion for V013 {
    type PreviousVersion = Self;

    fn read_array_length(version: impl FactorioVersion, reader: &mut impl Read) -> io::Result<u32> {
        u32::read_num(reader)
    }

    fn read_quality_version(version: impl FactorioVersion, _reader: &mut impl Read) -> io::Result<Option<u8>> {
        Ok(None)
    }

    fn read_allow_non_admin_debug_options(version: impl FactorioVersion, _reader: &mut impl Read) -> io::Result<Option<bool>> {
        Ok(None)
    }

    fn read_mod_name(version: impl FactorioVersion, reader: &mut impl Read) -> io::Result<String> {
        reader::read_string::<Self>(reader)
    }

    fn read_mod_crc(version: impl FactorioVersion, _reader: &mut impl Read) -> io::Result<Option<u32>> {
        Ok(None)
    }
}
