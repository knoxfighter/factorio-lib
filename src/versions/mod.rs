use std::io::{self, Read};

use crate::reader::{FactorioNumber, FactorioReader};
use crate::saves::Mod;

pub mod v013;
pub mod v014;
pub mod v015;
pub mod v016;
pub mod v017;

trait FactorioVersion {
    type PreviousVersion: FactorioVersion;

    fn read_optimized_number<T: FactorioNumber>(reader: &mut impl Read) -> io::Result<T> {
        Self::PreviousVersion::read_optimized_number(reader)
    }

    fn read_string(reader: &mut impl Read) -> io::Result<String> {
        Self::PreviousVersion::read_string(reader)
    }
    
    fn read_map<T: FactorioReader>(runtime_version: &RuntimeVersion, reader: &mut impl Read) -> io::Result<Vec<T>> {
        Self::PreviousVersion::read_map(runtime_version, reader)
    }

    fn read_quality_version(reader: &mut impl Read) -> io::Result<Option<u8>> {
        Self::PreviousVersion::read_quality_version(reader)
    }

    fn read_allow_non_admin_debug_options(reader: &mut impl Read) -> io::Result<Option<bool>> {
        Self::PreviousVersion::read_allow_non_admin_debug_options(reader)
    }
    
    fn read_mod(runtime_version: &RuntimeVersion, reader: &mut impl Read) -> io::Result<Mod> {
        Self::PreviousVersion::read_mod(runtime_version, reader)
    }
    
    fn read_mod_name(reader: &mut impl Read) -> io::Result<String> {
        Self::PreviousVersion::read_mod_name(reader)
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

    pub fn read_optimized_number<T: FactorioNumber>(&self, reader: &mut impl Read) -> io::Result<T> {
        dispatch!(self, read_optimized_number, reader)
    }

    pub fn read_string(&self, reader: &mut impl Read) -> io::Result<String> {
        dispatch!(self, read_string, reader)
    }
    
    pub fn read_array<T: FactorioReader>(&self, reader: &mut impl Read) -> io::Result<Vec<T>> {
        dispatch!(self, read_map, &self, reader)
    }

    pub fn read_quality_version(&self, reader: &mut impl Read) -> io::Result<Option<u8>> {
        dispatch!(self, read_quality_version, reader)
    }

    pub fn read_allow_non_admin_debug_options(&self, reader: &mut impl Read) -> io::Result<Option<bool>> {
        dispatch!(self, read_allow_non_admin_debug_options, reader)
    }
    
    pub fn read_mod(&self, reader: &mut impl Read) -> io::Result<Mod> {
        dispatch!(self, read_mod, self, reader)
    }
    
    pub fn read_mod_name(&self, reader: &mut impl Read) -> io::Result<String> {
        dispatch!(self, read_mod_name, reader)
    }
}
