use std::{io, io::Read};

use super::{FactorioVersion, RuntimeVersion};
use crate::{
    reader::{self, FactorioNumber, FactorioReader},
    saves::Mod,
};

pub type Latest = V013;

pub struct V013;

impl FactorioVersion for V013 {
    type PreviousVersion = Self;

    fn read_optimized_number<T: FactorioNumber>(reader: &mut impl Read) -> io::Result<T> {
        <T as FactorioNumber>::read_num(reader)
    }

    fn read_string(reader: &mut impl Read) -> io::Result<String> {
        let length: u32 = Self::read_optimized_number(reader)?;

        reader::read_string(reader, length as _)
    }

    fn read_map<T: FactorioReader>(
        runtime_version: &RuntimeVersion,
        reader: &mut impl Read,
    ) -> std::io::Result<Vec<T>> {
        let amount = u32::read_num(reader)?;

        let mut res = Vec::with_capacity(amount as _);
        for _ in 0..amount {
            let val = T::read(runtime_version, reader)?;
            res.push(val);
        }

        Ok(res)
    }

    fn read_quality_version(_reader: &mut impl Read) -> io::Result<Option<u8>> {
        Ok(None)
    }

    fn read_allow_non_admin_debug_options(_reader: &mut impl Read) -> io::Result<Option<bool>> {
        Ok(None)
    }

    fn read_mod(runtime_version: &RuntimeVersion, reader: &mut impl Read) -> io::Result<Mod> {
        Ok(Mod {
            name: runtime_version.read_mod_name(reader)?,
            version: [
                runtime_version.read_optimized_number::<u16>(reader)?,
                runtime_version.read_optimized_number::<u16>(reader)?,
                runtime_version.read_optimized_number::<u16>(reader)?,
            ],
            crc: None,
        })
    }

    fn read_mod_name(reader: &mut impl Read) -> io::Result<String> {
        Self::read_string(reader)
    }
}
