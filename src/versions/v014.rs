use std::{io, io::Read};
use crate::reader;
use crate::versions::v016::V016;

use super::{FactorioVersion, v013 as previous};

pub type Latest = V01414;

pub struct V014;

impl FactorioVersion for V014 {
    type PreviousVersion = previous::Latest;
}

pub struct V01414;

impl FactorioVersion for V01414 {
    type PreviousVersion = V014;

    fn read_mod_name(version: impl FactorioVersion, reader: &mut impl Read) -> io::Result<String> {
        reader::read_string::<V016>(reader)
    }
}
