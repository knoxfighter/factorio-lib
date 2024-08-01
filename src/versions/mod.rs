use std::io::{self, Read};

use crate::{
    reader::{FactorioNumber, FactorioReader},
    saves::Mod,
};

pub mod v013;
pub mod v014;
pub mod v015;
pub mod v016;
pub mod v017;

pub trait FactorioVersion {
    type PreviousVersion: FactorioVersion + Default;

    fn read_array_length(&self, reader: &mut impl Read) -> io::Result<u32> {
        Self::PreviousVersion::read_array_length(&Default::default(), reader)
    }

    fn read_quality_version(version: impl FactorioVersion, reader: &mut impl Read) -> io::Result<Option<u8>> {
        Self::PreviousVersion::read_quality_version(version, reader)
    }

    fn read_allow_non_admin_debug_options(version: impl FactorioVersion, reader: &mut impl Read) -> io::Result<Option<bool>> {
        Self::PreviousVersion::read_allow_non_admin_debug_options(version, reader)
    }

    fn read_mod_name(version: impl FactorioVersion, reader: &mut impl Read) -> io::Result<String> {
        Self::PreviousVersion::read_mod_name(version, reader)
    }
    fn read_mod_crc(version: impl FactorioVersion, reader: &mut impl Read) -> io::Result<Option<u32>> {
        Self::PreviousVersion::read_mod_crc(version, reader)
    }
}

pub enum RuntimeVersion {
    V017,
    V016,
    V015,
    V01414,
    V014,
    V013,
}

macro_rules! dispatch {
    ($version:expr, $func:ident, $($params:expr),*) => {
        match $version {
            RuntimeVersion::V017 => <v017::V017 as FactorioVersion>::$func($($params),*),
            RuntimeVersion::V016 => <v016::V016 as FactorioVersion>::$func($($params),*),
            RuntimeVersion::V015 => <v015::V015 as FactorioVersion>::$func($($params),*),
            RuntimeVersion::V01414 => <v014::V01414 as FactorioVersion>::$func($($params),*),
            RuntimeVersion::V014 => <v014::V014 as FactorioVersion>::$func($($params),*),
            RuntimeVersion::V013 => <v013::V013 as FactorioVersion>::$func($($params),*),
        }
    };
}

impl RuntimeVersion {
    pub fn parse_version(version: &[u16; 3]) -> Self {
        match version {
            [1, 1, _] => RuntimeVersion::V017,
            [1, 0, _] => RuntimeVersion::V017,
            [0, 18, _] => RuntimeVersion::V017,
            [0, 17, _] => RuntimeVersion::V017,
            [0, 16, _] => RuntimeVersion::V016,
            [0, 15, _] => RuntimeVersion::V015,
            [0, 14, x] if *x >= 14 => RuntimeVersion::V01414,
            [0, 14, _] => RuntimeVersion::V014,
            [0, 13, _] => RuntimeVersion::V013,
            _ => unimplemented!(),
        }
    }
}

impl FactorioVersion for RuntimeVersion {
    type PreviousVersion = Self;
    
    fn read_quality_version(version: impl FactorioVersion, reader: &mut impl Read) -> io::Result<Option<u8>> {
        dispatch!(self, read_quality_version, version, reader)
    }

    fn read_allow_non_admin_debug_options(
        &self,
        reader: &mut impl Read,
    ) -> io::Result<Option<bool>> {
        dispatch!(self, read_allow_non_admin_debug_options, reader)
    }

    fn read_mod_crc(&self, reader: &mut impl Read) -> io::Result<Option<u32>> {
        dispatch!(self, read_mod_crc, reader)
    }

    fn read_mod_name(&self, reader: &mut impl Read) -> io::Result<String> {
        dispatch!(self, read_mod_name, reader)
    }
}
