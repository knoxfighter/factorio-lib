use std::cmp::Ordering;
use std::io;
use std::io::Read;
use std::iter::Map;
use crate::reader::{FactorioNumber, read_u16};

// pub struct Version([u8;4]);
#[derive(PartialOrd, PartialEq)]
pub struct Version64([u16;4]); // parseable by parsing 4 x u16
pub struct Version48([u16;3]); // parseable by parsing 3 x optimized u16

pub struct SaveHeader {
    FactorioVersion: Version64,
    Campaign: String,
    Name: String,
    BaseMod: String,
    Difficulty: u8,
    Finished: bool,
    PlayerWon: bool,
    NextLevel: String,
    CanContinue: bool,
    FinishedButContinuing: bool,
    SavingReplay: bool,
    AllowNonAdminDebugOptions: bool,
    LoadedFrom: Version48,
    LoadedFromBuild: u16,
    AllowedCommands: u8,
    Stats: Map<u8, Vec<Map<u16, u32>>>,
    Mods: Vec<Mod>,
}

pub struct Mod {
    Name: String,
    Version: Version48,
    CRC: u32,
}

impl Version64 {
    fn read(reader: &mut impl Read) -> io::Result<Self> {
        Ok(Self {
            0: [u16::read_num(reader)?, u16::read_num(reader)?, u16::read_num(reader)?, u16::read_num(reader)?]
        })
    }
}

impl From<[u16;4]> for Version64 {
    fn from(value: [u16; 4]) -> Self {
        Self{0:value}
    }
}

pub fn get_save_header(reader: &mut impl Read) -> io::Result<SaveHeader> {
    let res = SaveHeader {
        FactorioVersion: Version64::read(reader)?,
    };

    res
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
