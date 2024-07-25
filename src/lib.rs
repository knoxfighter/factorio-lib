pub mod saves;
mod reader;


enum  FactorioVersion {
    #[cfg(feature = "legacy")]
    Legacy,
    V01414,
    V0160,
}
