use std::{
    io,
    io::{Read, Seek},
};

use flate2::read::ZlibDecoder;

use crate::reader::{read_allow_non_admin_debug_options, read_array, read_build_version, read_large_blueprint_size, read_loaded_from, read_quality_version, read_string, FactorioReader};

#[repr(u8)]
#[derive(PartialEq, Debug)]
pub enum Difficulty {
    Easy    = 0x0,
    Normal  = 0x1,
    Hard    = 0x2,
    Nothing = 0x3,
}

impl From<u8> for Difficulty {
    fn from(value: u8) -> Self {
        match value {
            0 => Difficulty::Easy,
            1 => Difficulty::Normal,
            2 => Difficulty::Hard,
            3 => Difficulty::Nothing,
            _ => {
                panic!("invalid value: {value}")
            }
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Debug)]
pub enum AllowedCommands {
    True,
    False,
    AdminsOnly,
}

impl From<u8> for AllowedCommands {
    fn from(value: u8) -> Self {
        match value {
            1 => AllowedCommands::True,
            2 => AllowedCommands::False,
            3 => AllowedCommands::AdminsOnly,
            _ => {
                panic!("invalid value: {value}")
            }
        }
    }
}

#[derive(PartialOrd, PartialEq, Debug, Copy, Clone)]
pub struct FactorioVersion([u16; 4]);

impl From<[u16; 4]> for FactorioVersion {
    fn from(value: [u16; 4]) -> Self {
        Self(value)
    }
}

#[derive(PartialEq, Debug)]
pub struct SaveHeader {
    pub factorio_version: FactorioVersion,
    pub quality_version: Option<u8>,
    pub campaign_name: String, // can be ":/tutorials"
    pub level_name: String,
    pub base_mod_name: String,
    pub difficulty: Difficulty,
    pub finished: bool,
    pub player_won: bool,
    pub next_level: String,
    pub can_continue: bool,            // since in 0.12
    pub finished_but_continuing: bool, // since in 0.12
    pub saving_replay: bool,
    pub allow_non_admin_debug_options: Option<bool>,
    pub loaded_from: [u16; 3], // called "application-version" in factorio
    pub loaded_from_build: u32,
    pub allowed_commands: AllowedCommands,
    pub lange_blueprint_size: Option<u32>,
    pub mods: Vec<Mod>, /* called "active-mods" in factorio */

                        /* the following is untracked and just a first glance in 1.1 factorio
                         * pub ModSettingsChecksum: u32,
                         * pub ModSettings: SomeMap,
                         * pub MapData: ??, */
}

#[derive(PartialEq, Debug)]
pub struct Mod {
    pub name: String,
    pub version: [u16; 3],
    pub crc: Option<u32>, // since 0.15.0
}

impl FactorioReader for Mod {
    fn read(factorio_version: &FactorioVersion, reader: &mut impl Read) -> io::Result<Self> {
        let name = if factorio_version >= &[0, 14, 0, 0].into() {
            read_string(factorio_version, reader, true)?
        } else {
            read_string(factorio_version, reader, false)?
        };

        let version = if factorio_version >= &[0, 14, 14, 0].into() {
            [
                u16::read_optimized(factorio_version, reader)?,
                u16::read_optimized(factorio_version, reader)?,
                u16::read_optimized(factorio_version, reader)?,
            ]
        } else {
            [
                u16::read(factorio_version, reader)?,
                u16::read(factorio_version, reader)?,
                u16::read(factorio_version, reader)?,
            ]
        };

        let crc = if factorio_version >= &[0, 15, 0, 91].into() {
            Some(u32::read(factorio_version, reader)?)
        } else {
            None
        };

        Ok(Mod { name, version, crc })
    }
}

/// Get the header of a savefile by a reader to a `level.dat` file.
/// In Factorio >= 1.1.14 that has to be the zlib decompressed `level.dat0` file
///
/// # Arguments
///
/// * `reader`:
///
/// returns: Result<SaveHeader, Error>
///
/// # Examples
///
/// ```
/// use factorio::saves::get_save_header;
///
/// let file = std::fs::File::open("test/test_1_1.zip").unwrap();
/// let mut archive = zip::ZipArchive::new(file).unwrap();
/// let mut level_init = archive.by_name("test_1_1/level.dat").unwrap();
/// let header = get_save_header(&mut level_init).unwrap();
/// ```
/// ```
/// use factorio::saves::get_save_header;
///
/// let file = std::fs::File::open("test/test_1_1_14.zip").unwrap();
/// let mut archive = zip::ZipArchive::new(file).unwrap();
///
/// // read level.dat0 and deflate (flate2)
/// let level_init = archive.by_name("test1_1_14/level.dat0").unwrap();
/// let mut decoder = flate2::read::ZlibDecoder::new(level_init);
/// let header = get_save_header(&mut decoder).unwrap();
/// ```
pub fn get_save_header(reader: &mut impl Read) -> io::Result<SaveHeader> {
    let save_version: FactorioVersion = [
        u16::read(&[0, 0, 0, 0].into(), reader)?,
        u16::read(&[0, 0, 0, 0].into(), reader)?,
        u16::read(&[0, 0, 0, 0].into(), reader)?,
        u16::read(&[0, 0, 0, 0].into(), reader)?,
    ]
    .into();

    let res = SaveHeader {
        factorio_version: save_version,
        quality_version: read_quality_version(&save_version, reader)?,
        campaign_name: read_string(&save_version, reader, false)?,
        level_name: read_string(&save_version, reader, false)?,
        base_mod_name: read_string(&save_version, reader, false)?,
        difficulty: u8::read(&save_version, reader)?.into(),
        finished: u8::read(&save_version, reader)? != 0,
        player_won: u8::read(&save_version, reader)? != 0,
        next_level: read_string(&save_version, reader, false)?,
        can_continue: u8::read(&save_version, reader)? != 0,
        finished_but_continuing: u8::read(&save_version, reader)? != 0,
        saving_replay: u8::read(&save_version, reader)? != 0,
        allow_non_admin_debug_options: read_allow_non_admin_debug_options(&save_version, reader)?,
        loaded_from: read_loaded_from(&save_version, reader)?,
        loaded_from_build: read_build_version(&save_version, reader)?,
        allowed_commands: u8::read(&save_version, reader)?.into(),
        lange_blueprint_size: read_large_blueprint_size(&save_version, reader)?,
        mods: read_array::<Mod>(&save_version, reader)?,
    };

    Ok(res)
}

pub fn get_save_header_by_path(reader: impl Read + Seek) -> io::Result<SaveHeader> {
    let mut archive = zip::ZipArchive::new(reader)?;

    let dat_info = archive
        .file_names()
        .enumerate()
        .find_map(move |(i, filename)| {
            if filename.ends_with("level.dat") {
                return Some((i, false));
            } else if filename.ends_with("level.dat0") {
                return Some((i, true));
            }
            None
        })
        .ok_or(io::Error::from(io::ErrorKind::NotFound))?;

    let mut file = archive.by_index(dat_info.0)?;

    // if 1 is true, it is zlib compressed
    if dat_info.1 {
        let mut file = ZlibDecoder::new(file);
        get_save_header(&mut file)
    } else {
        get_save_header(&mut file)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::Path};

    use super::*;

    #[test]
    fn test_2_0_13_ext() {
        let header = SaveHeader {
            factorio_version: [2, 0, 13, 1].into(),
            quality_version: Some(0),
            campaign_name: "transport-belt-madness".to_string(),
            level_name: "level-01".to_string(),
            base_mod_name: "base".to_string(),
            difficulty: Difficulty::Normal,
            finished: false,
            player_won: false,
            next_level: "".to_string(),
            can_continue: false,
            finished_but_continuing: false,
            saving_replay: false,
            allow_non_admin_debug_options: Some(true),
            loaded_from: [2, 0, 13],
            loaded_from_build: 79912,
            allowed_commands: AllowedCommands::True,
            lange_blueprint_size: Some(10485760),
            mods: vec![
                Mod {
                    name: "base".to_string(),
                    version: [2, 0, 13],
                    crc: Some(2691306720),
                },
                Mod {
                    name: "AutoDeconstruct".to_string(),
                    version: [1, 0, 2],
                    crc: Some(3603960023),
                },
                Mod {
                    name: "elevated-rails".to_string(),
                    version: [2, 0, 13],
                    crc: Some(2288905443),
                },
                Mod {
                    name: "flib".to_string(),
                    version: [0, 15, 0],
                    crc: Some(2235140407),
                },
                Mod {
                    name: "quality".to_string(),
                    version: [2, 0, 13],
                    crc: Some(4264147466),
                },
                Mod {
                    name: "RateCalculator".to_string(),
                    version: [3, 3, 2],
                    crc: Some(2191928673),
                },
                Mod {
                    name: "space-age".to_string(),
                    version: [2, 0, 13],
                    crc: Some(542481865),
                },
            ],
        };

        let path = Path::new("test/test_2_0_13_ext.zip");
        let file = File::open(path).unwrap();
        let test = get_save_header_by_path(file).unwrap();
        assert_eq!(header, test);
    }
    
    #[test]
    fn test_2_0_13() {
        let header = SaveHeader {
            factorio_version: [2, 0, 13, 1].into(),
            quality_version: Some(0),
            campaign_name: "transport-belt-madness".to_string(),
            level_name: "level-01".to_string(),
            base_mod_name: "base".to_string(),
            difficulty: Difficulty::Normal,
            finished: false,
            player_won: false,
            next_level: "".to_string(),
            can_continue: false,
            finished_but_continuing: false,
            saving_replay: false,
            allow_non_admin_debug_options: Some(true),
            loaded_from: [2, 0, 13],
            loaded_from_build: 79912,
            allowed_commands: AllowedCommands::True,
            lange_blueprint_size: Some(10485760),
            mods: vec![
                Mod {
                    name: "base".to_string(),
                    version: [2, 0, 13],
                    crc: Some(2691306720),
                },
                Mod {
                    name: "AutoDeconstruct".to_string(),
                    version: [1, 0, 2],
                    crc: Some(3603960023),
                },
                Mod {
                    name: "flib".to_string(),
                    version: [0, 15, 0],
                    crc: Some(2235140407),
                },
                Mod {
                    name: "RateCalculator".to_string(),
                    version: [3, 3, 2],
                    crc: Some(2191928673),
                },
            ],
        };

        let path = Path::new("test/test_2_0_13.zip");
        let file = File::open(path).unwrap();
        let test = get_save_header_by_path(file).unwrap();
        assert_eq!(header, test);
    }

    #[test]
    fn test_1_1_14() {
        let header = SaveHeader {
            factorio_version: [1, 1, 19, 0].into(),
            quality_version: Some(0),
            campaign_name: "transport-belt-madness".to_string(),
            level_name: "level-01".to_string(),
            base_mod_name: "base".to_string(),
            difficulty: Difficulty::Normal,
            finished: false,
            player_won: false,
            next_level: "".to_string(),
            can_continue: false,
            finished_but_continuing: false,
            saving_replay: false,
            allow_non_admin_debug_options: Some(true),
            loaded_from: [1, 1, 19],
            loaded_from_build: 57957,
            allowed_commands: AllowedCommands::True,
            lange_blueprint_size: None,
            mods: vec![
                Mod {
                    name: "base".to_string(),
                    version: [1, 1, 19],
                    crc: Some(1503457007),
                },
                Mod {
                    name: "belt-balancer".to_string(),
                    version: [3, 0, 0],
                    crc: Some(1386746210),
                },
                Mod {
                    name: "train-station-overview".to_string(),
                    version: [3, 0, 0],
                    crc: Some(442638023),
                },
            ],
        };

        let path = Path::new("test/test_1_1_14.zip");
        let file = File::open(path).unwrap();
        let test = get_save_header_by_path(file).unwrap();
        assert_eq!(header, test);
    }

    #[test]
    fn test_1_1() {
        let header = SaveHeader {
            factorio_version: [1, 1, 6, 4].into(),
            quality_version: Some(0),
            campaign_name: "transport-belt-madness".to_string(),
            level_name: "level-01".to_string(),
            base_mod_name: "base".to_string(),
            difficulty: Difficulty::Normal,
            finished: false,
            player_won: false,
            next_level: "".to_string(),
            can_continue: false,
            finished_but_continuing: false,
            saving_replay: false,
            allow_non_admin_debug_options: Some(true),
            loaded_from: [1, 1, 6],
            loaded_from_build: 57355,
            allowed_commands: AllowedCommands::True,
            lange_blueprint_size: None,
            mods: vec![
                Mod {
                    name: "base".to_string(),
                    version: [1, 1, 6],
                    crc: Some(2570609689),
                },
                Mod {
                    name: "belt-balancer".to_string(),
                    version: [3, 0, 0],
                    crc: Some(1386746210),
                },
                Mod {
                    name: "train-station-overview".to_string(),
                    version: [3, 0, 0],
                    crc: Some(442638023),
                },
            ],
        };

        let path = Path::new("test/test_1_1.zip");
        let file = File::open(path).unwrap();
        let test = get_save_header_by_path(file).unwrap();

        assert_eq!(header, test);
    }

    #[test]
    fn test_0_18() {
        let header = SaveHeader {
            factorio_version: [0, 18, 2, 2].into(),
            quality_version: Some(0),
            campaign_name: "transport-belt-madness".to_string(),
            level_name: "level-01".to_string(),
            base_mod_name: "base".to_string(),
            difficulty: Difficulty::Normal,
            finished: false,
            player_won: false,
            next_level: "".to_string(),
            can_continue: false,
            finished_but_continuing: false,
            saving_replay: false,
            allow_non_admin_debug_options: Some(true),
            loaded_from: [0, 18, 2],
            loaded_from_build: 49204,
            allowed_commands: AllowedCommands::True,
            lange_blueprint_size: None,
            mods: vec![
                Mod {
                    name: "base".to_string(),
                    version: [0, 18, 2],
                    crc: Some(3303078020),
                },
                Mod {
                    name: "belt-balancer".to_string(),
                    version: [2, 0, 0],
                    crc: Some(3388930795),
                },
                Mod {
                    name: "train-station-overview".to_string(),
                    version: [2, 0, 1],
                    crc: Some(442638023),
                },
            ],
        };

        let path = Path::new("test/test_0_18.zip");
        let file = File::open(path).unwrap();
        let test = get_save_header_by_path(file).unwrap();

        assert_eq!(header, test);
    }

    #[test]
    fn test_0_17() {
        let header = SaveHeader {
            factorio_version: [0, 17, 1, 1].into(),
            quality_version: Some(0),
            campaign_name: "transport-belt-madness".to_string(),
            level_name: "level-01".to_string(),
            base_mod_name: "base".to_string(),
            difficulty: Difficulty::Easy,
            finished: false,
            player_won: false,
            next_level: "".to_string(),
            can_continue: false,
            finished_but_continuing: false,
            saving_replay: true,
            allow_non_admin_debug_options: Some(true),
            loaded_from: [0, 17, 1],
            loaded_from_build: 43001,
            allowed_commands: AllowedCommands::True,
            lange_blueprint_size: None,
            mods: vec![
                Mod {
                    name: "Warehousing".to_string(),
                    version: [0, 2, 0],
                    crc: Some(2309169136),
                },
                Mod {
                    name: "base".to_string(),
                    version: [0, 17, 1],
                    crc: Some(819815259),
                },
            ],
        };

        let path = Path::new("test/test_0_17.zip");
        let file = File::open(path).unwrap();
        let test = get_save_header_by_path(file).unwrap();

        assert_eq!(header, test);
    }

    #[test]
    fn test_0_16() {
        let header = SaveHeader {
            factorio_version: [0, 16, 51, 0].into(),
            quality_version: None,
            campaign_name: "transport-belt-madness".to_string(),
            level_name: "level-01".to_string(),
            base_mod_name: "base".to_string(),
            difficulty: Difficulty::Easy,
            finished: false,
            player_won: false,
            next_level: "".to_string(),
            can_continue: false,
            finished_but_continuing: false,
            saving_replay: true,
            allow_non_admin_debug_options: Some(true),
            loaded_from: [0, 16, 51],
            loaded_from_build: 36654,
            allowed_commands: AllowedCommands::True,
            lange_blueprint_size: None,
            mods: vec![
                Mod {
                    name: "Warehousing".to_string(),
                    version: [0, 1, 3],
                    crc: Some(4151823552),
                },
                Mod {
                    name: "base".to_string(),
                    version: [0, 16, 51],
                    crc: Some(3323233190),
                },
            ],
        };

        let path = Path::new("test/test_0_16.zip");
        let file = File::open(path).unwrap();
        let test = get_save_header_by_path(file).unwrap();

        assert_eq!(header, test);
    }

    #[test]
    fn test_0_15() {
        let header = SaveHeader {
            factorio_version: [0, 15, 40, 0].into(),
            quality_version: None,
            campaign_name: "transport-belt-madness".to_string(),
            level_name: "level-01".to_string(),
            base_mod_name: "base".to_string(),
            difficulty: Difficulty::Easy,
            finished: false,
            player_won: false,
            next_level: "".to_string(),
            can_continue: false,
            finished_but_continuing: false,
            saving_replay: true,
            allow_non_admin_debug_options: None,
            loaded_from: [0, 15, 40],
            loaded_from_build: 30950,
            allowed_commands: AllowedCommands::True,
            lange_blueprint_size: None,
            mods: vec![
                Mod {
                    name: "Warehousing".to_string(),
                    version: [0, 0, 13],
                    crc: Some(606039864),
                },
                Mod {
                    name: "base".to_string(),
                    version: [0, 15, 40],
                    crc: Some(1503927233),
                },
            ],
        };

        let path = Path::new("test/test_0_15.zip");
        let file = File::open(path).unwrap();
        let test = get_save_header_by_path(file).unwrap();

        assert_eq!(header, test);
    }

    #[test]
    fn test_0_14() {
        let header = SaveHeader {
            factorio_version: [0, 14, 23, 0].into(),
            quality_version: None,
            campaign_name: "transport-belt-madness".to_string(),
            level_name: "level-01".to_string(),
            base_mod_name: "base".to_string(),
            difficulty: Difficulty::Normal,
            finished: false,
            player_won: false,
            next_level: "".to_string(),
            can_continue: false,
            finished_but_continuing: false,
            saving_replay: true,
            allow_non_admin_debug_options: None,
            loaded_from: [0, 14, 23],
            loaded_from_build: 25374,
            allowed_commands: AllowedCommands::True,
            lange_blueprint_size: None,
            mods: vec![
                Mod {
                    name: "Warehousing".to_string(),
                    version: [0, 0, 11],
                    crc: None,
                },
                Mod {
                    name: "base".to_string(),
                    version: [0, 14, 23],
                    crc: None,
                },
            ],
        };

        let path = Path::new("test/test_0_14.zip");
        let file = File::open(path).unwrap();
        let test = get_save_header_by_path(file).unwrap();

        assert_eq!(header, test);
    }

    #[test]
    fn test_0_13() {
        let header = SaveHeader {
            factorio_version: [0, 13, 20, 0].into(),
            quality_version: None,
            campaign_name: "transport-belt-madness".to_string(),
            level_name: "level-01".to_string(),
            base_mod_name: "base".to_string(),
            difficulty: Difficulty::Normal,
            finished: false,
            player_won: false,
            next_level: "".to_string(),
            can_continue: false,
            finished_but_continuing: false,
            saving_replay: true,
            allow_non_admin_debug_options: None,
            loaded_from: [0, 13, 20],
            loaded_from_build: 24011,
            allowed_commands: AllowedCommands::True,
            lange_blueprint_size: None,
            mods: vec![
                Mod {
                    name: "Extra-Virtual-Signals".to_string(),
                    version: [1, 1, 0],
                    crc: None,
                },
                Mod {
                    name: "base".to_string(),
                    version: [0, 13, 20],
                    crc: None,
                },
            ],
        };

        let path = Path::new("test/test_0_13.zip");
        let file = File::open(path).unwrap();
        let test = get_save_header_by_path(file).unwrap();

        assert_eq!(header, test);
    }
}
