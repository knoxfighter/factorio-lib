use std::{io, io::Read};

use crate::versions::RuntimeVersion;

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
    fn read(runtime_version: &RuntimeVersion, reader: &mut impl Read) -> io::Result<Self>;
}

pub fn read_string(reader: &mut impl Read, length: usize) -> io::Result<String> {
    let mut buf = vec![0; length];
    reader.read_exact(&mut buf)?;

    String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))
}
