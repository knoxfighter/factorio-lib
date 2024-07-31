use std::{io, io::Read};

use crate::reader::{self, FactorioNumber};

use super::FactorioVersion;

pub type Latest = V013;

pub struct V013;

impl FactorioVersion for V013 {
    type PreviousVersion = Self;

    fn read_array_length(reader: &mut impl Read) -> io::Result<u32> {
        u32::read_num(reader)
    }

    fn read_quality_version(_reader: &mut impl Read) -> io::Result<Option<u8>> {
        Ok(None)
    }

    fn read_allow_non_admin_debug_options(_reader: &mut impl Read) -> io::Result<Option<bool>> {
        Ok(None)
    }

    fn read_mod_name(reader: &mut impl Read) -> io::Result<String> {
        reader::read_string::<Self>(reader)
    }

    fn read_mod_crc(_reader: &mut impl Read) -> io::Result<Option<u32>> {
        Ok(None)
    }
}
