use std::io::Read;
use std::io;
use crate::saves::FactorioVersion;

trait FactorioReader: Sized {
    fn read(version: &FactorioVersion, reader: &mut impl Read) -> io::Result<Self>;
    fn read_optimized_num(version: &FactorioVersion, reader: &mut impl Read) -> io::Result<Self> {
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
            
            fn read_optimized_num(_version: &FactorioVersion, reader: &mut impl Read) -> io::Result<Self> {
                let first = u8::read_num(reader)?;
                if first != u8::MAX {
                    return Ok(first.into());
                }
        
                // otherwise this reads the whole value
                Self::read_num(reader)
            }
        }
    };
    ($($int:ty),*) => {$(read_num_impl!($int);)*}
}

read_num_impl!(u8, u16, u32, u64);
