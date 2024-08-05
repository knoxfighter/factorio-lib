use std::{io, io::Read};

use super::{v013 as previous, FactorioVersion};
use crate::{reader, versions::v016::V016};

pub type Latest = V01414;

#[derive(Default)]
pub struct V014;

impl FactorioVersion for V014 {
    type PreviousVersion = previous::Latest;
}

#[derive(Default)]
pub struct V01414;

impl FactorioVersion for V01414 {
    type PreviousVersion = V014;

    fn read_mod_name(&self, reader: &mut impl Read) -> io::Result<String> {
        reader::read_string(&V016 {}, reader)
    }
}
