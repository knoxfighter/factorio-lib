use std::{io, io::Read};

use crate::versions::FactorioVersion;

pub trait FactorioNumber: Sized + From<u8> {
    fn read_num(reader: &mut impl Read) -> io::Result<Self>;
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

pub trait FactorioReader: Sized {
    fn read(version: &impl FactorioVersion, reader: &mut impl Read) -> io::Result<Self>;
}

pub fn read_array<T: FactorioReader>(
    version: &impl FactorioVersion,
    reader: &mut impl Read,
) -> io::Result<Vec<T>> {
    let amount = version.read_array_length(reader)?;

    let mut res = Vec::with_capacity(amount as _);
    for _ in 0..amount {
        let val = T::read(version, reader)?;
        res.push(val);
    }

    Ok(res)
}

pub fn read_string(version: &impl FactorioVersion, reader: &mut impl Read) -> io::Result<String> {
    let length = version.read_array_length(reader)?;

    let mut buf = vec![0; length as _];
    reader.read_exact(&mut buf)?;

    String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}

pub fn read_optimized_num<T: FactorioNumber>(reader: &mut impl Read) -> io::Result<T> {
    let false = std::mem::size_of::<T>() == std::mem::size_of::<u8>() else {
        panic!("function called on invalid type");
    };

    let first = <u8 as FactorioNumber>::read_num(reader)?;
    if first != u8::MAX {
        return Ok(first.into());
    }

    // otherwise this reads the whole value
    T::read_num(reader)
}
