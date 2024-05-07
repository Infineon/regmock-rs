/*
CC0 1.0 Universal

CREATIVE COMMONS CORPORATION IS NOT A LAW FIRM AND DOES NOT PROVIDE LEGAL SERVICES.
DISTRIBUTION OF THIS DOCUMENT DOES NOT CREATE AN ATTORNEY-CLIENT RELATIONSHIP. CREATIVE
COMMONS PROVIDES THIS INFORMATION ON AN "AS-IS" BASIS. CREATIVE COMMONS MAKES NO WARRANTIES
REGARDING THE USE OF THIS DOCUMENT OR THE INFORMATION OR WORKS PROVIDED HEREUNDER, AND
DISCLAIMS LIABILITY FOR DAMAGES RESULTING FROM THE USE OF THIS DOCUMENT OR THE INFORMATION
OR WORKS PROVIDED HEREUNDER.
*/
//! Contains perfect hash function that maps form raw addresses to
//! a string containing the names of all registers that point to an address.
//!
//! When using tracing feature to record accesses to registers, the exact
//! API path, though which a specific address was accessed gets lost.
//! This poses a problem when recorded register accesses contain accesses
//! to unexpected registers. [`reg_name_from_addr`] can be used to make
//! logs of raw register accesses more readable to humans by providing a list
//! of names of registers that alias a specific physical address.
//!
use phf::phf_map;

/// Get a &str name of a register given it's address.
pub fn reg_name_from_addr(addr: u64) -> Option<&'static &'static str> {
    REGISTER_NAMES.get(&addr)
}

static REGISTER_NAMES: phf::Map<u64, &'static str> = phf_map! {
  0x8000u64 => "
      TIMER.timercluster()[0].ctrlstat(),
    ",
  0x8004u64 => "
      TIMER.timercluster()[0].count(),
    ",
  0x8008u64 => "
      TIMER.timercluster()[0].max(),
    ",
  0x800cu64 => "
      TIMER.timercluster()[1].ctrlstat(),
    ",
  0x8010u64 => "
      TIMER.timercluster()[1].count(),
    ",
  0x8014u64 => "
      TIMER.timercluster()[1].max(),
    ",
  0x8200u64 => "
      SPI.status(),
    ",
  0x8204u64 => "
      SPI.ctrl(),
    ",
  0x8208u64 => "
      SPI.tx(),
    ",
  0x820cu64 => "
      SPI.rx(),
    ",
  0x8420u64 => "
      GPIO.r#in(),
    ",
  0x8424u64 => "
      GPIO.we(),
    ",
  0x842cu64 => "
      GPIO.out(),
    ",
};
