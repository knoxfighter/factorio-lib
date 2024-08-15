use std::{io, io::Read};

use crate::saves::FactorioVersion;

pub(crate) trait FactorioReader: Sized {
    fn read(version: &FactorioVersion, reader: &mut impl Read) -> io::Result<Self>;
    fn read_optimized(_version: &FactorioVersion, _reader: &mut impl Read) -> io::Result<Self> {
        unimplemented!();
    }
}

macro_rules! read_num_impl {
    ($int:ty) => {
        impl FactorioReader for $int {
            fn read(_version: &FactorioVersion, reader: &mut impl Read) -> io::Result<Self> {
                let buf: $int = 0;
                let mut buf = buf.to_le_bytes();
                reader.read_exact(&mut buf)?;
                Ok(<$int>::from_le_bytes(buf))
            }

            fn read_optimized(version: &FactorioVersion, reader: &mut impl Read) -> io::Result<Self> {
                let first = u8::read(version, reader)?;
                if first != u8::MAX {
                    return Ok(first.into());
                }

                // otherwise this reads the whole value
                Self::read(version, reader)
            }
        }
    };
    ($($int:ty),*) => {$(read_num_impl!($int);)*}
}

read_num_impl!(u8, u16, u32, u64);

pub(crate) fn read_string(
    version: &FactorioVersion,
    reader: &mut impl Read,
    force_optimized: bool,
) -> io::Result<String> {
    let length = if version >= &[0, 16, 0, 0].into() || force_optimized {
        u32::read_optimized(version, reader)?
    } else {
        u32::read(version, reader)?
    };

    let mut buf = vec![0; length as _];
    reader.read_exact(&mut buf)?;

    String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}

pub(crate) fn read_quality_version(
    version: &FactorioVersion,
    reader: &mut impl Read,
) -> io::Result<Option<u8>> {
    if version >= &[0, 17, 0, 0].into() {
        return Ok(Some(u8::read(version, reader)?));
    }
    Ok(None)
}

pub(crate) fn read_allow_non_admin_debug_options(
    version: &FactorioVersion,
    reader: &mut impl Read,
) -> io::Result<Option<bool>> {
    if version >= &[0, 16, 0, 0].into() {
        return Ok(Some(u8::read(version, reader)? != 0));
    }
    Ok(None)
}

pub(crate) fn read_loaded_from(
    version: &FactorioVersion,
    reader: &mut impl Read,
) -> io::Result<[u16; 3]> {
    if version >= &[0, 14, 14, 0].into() {
        Ok([
            u16::read_optimized(version, reader)?,
            u16::read_optimized(version, reader)?,
            u16::read_optimized(version, reader)?,
        ])
    } else {
        Ok([
            u16::read(version, reader)?,
            u16::read(version, reader)?,
            u16::read(version, reader)?,
        ])
    }
}

pub(crate) fn read_array<T: FactorioReader>(
    version: &FactorioVersion,
    reader: &mut impl Read,
) -> io::Result<Vec<T>> {
    let length = if version >= &[0, 16, 0, 0].into() {
        u32::read_optimized(version, reader)?
    } else {
        u32::read(version, reader)?
    };

    let mut res = Vec::with_capacity(length as _);
    for _ in 0..length {
        let val = T::read(version, reader)?;
        res.push(val);
    }

    Ok(res)
}
