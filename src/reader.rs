use std::io;
use std::io::Read;
use crate::FactorioVersion;
use crate::saves::Version64;

pub trait FactorioNumber : Sized {
    fn read_num(reader: &mut impl Read) -> io::Result<Self>;
}

pub fn read_optimized_num<T: FactorioNumber + From<u8>>(reader: &mut impl Read, _save_version: FactorioVersion) -> io::Result<T> {
    // since factorio 0.14.14 this is an "optimized" number, which only reads the first byte, if it is smaller than 255
    // if save_version >= [0, 14, 14, 0].into() {
    match _save_version {
        #[cfg(feature = "legacy")]
        FactorioVersion::Legacy => {}
        FactorioVersion::V01414 => {
            let first = <u8 as FactorioNumber>::read_num(reader)?;
            if first != u8::MAX {
                return Ok(first.into());
            }
        }
    }

    // otherwise this reads the whole value
    <T as FactorioNumber>::read_num(reader)
}


macro_rules! read_num_impl {
    ($int:ty) => {
        impl FactorioNumber for $int {
            fn read_num(reader: &mut impl Read) -> io::Result<Self> {
                let buf: $int = 0;
                let mut buf = buf.to_le_bytes();
                reader.read_exact(&mut buf)?;
                Ok(<$int>::from_le_bytes(buf))
            }
        }
    };
    ($($int:ty),*) => {$(read_num_impl!($int);)*}
}

read_num_impl!(u8, u16, u32, u64);

pub fn read_string(reader: &mut impl Read, save_version: Version64, force_optimized: bool) -> io::Result<String> {
    if save_version >= [0, 16, 0, 0].into() {
        let size = read_optimized_num::<u32>(reader, save_version)?;
    } else {
        let size = 
    }
    
    Ok("t".into())
}
