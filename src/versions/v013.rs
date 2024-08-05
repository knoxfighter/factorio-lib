use std::{io, io::Read};

use super::FactorioVersion;
use crate::reader::{self, FactorioNumber};

pub type Latest = V013;

#[derive(Default)]
pub struct V013;

impl FactorioVersion for V013 {
    type PreviousVersion = Self;

    fn read_array_length(&self, reader: &mut impl Read) -> io::Result<u32> {
        u32::read_num(reader)
    }

    fn read_quality_version(&self, _reader: &mut impl Read) -> io::Result<Option<u8>> {
        Ok(None)
    }

    fn read_allow_non_admin_debug_options(
        &self,
        _reader: &mut impl Read,
    ) -> io::Result<Option<bool>> {
        Ok(None)
    }

    fn read_mod_name(&self, reader: &mut impl Read) -> io::Result<String> {
        reader::read_string(self, reader)
    }

    fn read_mod_crc(&self, _reader: &mut impl Read) -> io::Result<Option<u32>> {
        Ok(None)
    }
}
