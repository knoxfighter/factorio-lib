use std::io::Read;

use super::{v015 as previous, FactorioVersion};
use crate::reader::{read_optimized_num, FactorioNumber};

pub type Latest = V016;

#[derive(Default)]
pub struct V016;

impl FactorioVersion for V016 {
    type PreviousVersion = previous::Latest;

    fn read_array_length(&self, reader: &mut impl Read) -> std::io::Result<u32> {
        read_optimized_num(reader)
    }

    fn read_allow_non_admin_debug_options(
        &self,
        reader: &mut impl Read,
    ) -> std::io::Result<Option<bool>> {
        let res = u8::read_num(reader)?;
        Ok(Some(res != 0))
    }
}
