mod reader;
//pub mod saves;
pub mod saves;
mod versions;

// Mostly based on https://forums.factorio.com/viewtopic.php?f=5&t=8568&p=277892&hilit=level.dat+python#p277892
// Function that writes the save in factorio: `Scenario::saveMap()`
// start with/within `MapSerialiser::MapSerialiser()`

// We support reading from 0.13 onwards, if you need something earlier, tell us
// and we might implement it.
