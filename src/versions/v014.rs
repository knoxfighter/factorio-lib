use std::{io, io::Read};

use super::{v013 as previous, FactorioVersion, RuntimeVersion};
use crate::reader::FactorioNumber;

pub type Latest = V01414;

pub struct V014;

impl FactorioVersion for V014 {
    type PreviousVersion = previous::Latest;
}

pub struct V01414;

impl FactorioVersion for V01414 {
    type PreviousVersion = V014;

    fn read_optimized_number<T: FactorioNumber>(reader: &mut impl Read) -> io::Result<T> {
        // since factorio 0.14.14 this is an "optimized" number, which only reads the
        // first byte, if it is smaller than 255
        let first = <u8 as FactorioNumber>::read_num(reader)?;
        if first != u8::MAX {
            return Ok(first.into());
        }

        // otherwise this reads the whole value
        <T as FactorioNumber>::read_num(reader)
    }

    fn read_mod_name(reader: &mut impl Read) -> io::Result<String> {
        RuntimeVersion::V016.read_string(reader)
    }
}
