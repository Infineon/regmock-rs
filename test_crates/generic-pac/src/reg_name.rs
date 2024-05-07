/*
Test license
 
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
  0x40010000u64 => "
      TIMER.bitfield_reg_alt_group(),
      TIMER.bitfield_reg(),
      TIMER.cluster1().cr(),
      TIMER.cluster1().cluster1().nestedreg(),
      TIMER.clusterdim()[0].cr(),
    ",
  0x40010004u64 => "
      TIMER.sr(),
    ",
  0x40010010u64 => "
      TIMER.int(),
    ",
  0x40010020u64 => "
      TIMER.nobitfield_reg(),
    ",
  0x40010024u64 => "
      TIMER.r#match(),
    ",
  0x40010028u64 => "
      TIMER.prescale_wr(),
      TIMER.prescale_rd(),
    ",
  0x40010050u64 => "
      TIMER.arrayreg()[0],
    ",
  0x40010054u64 => "
      TIMER.arrayreg()[1],
    ",
  0x40010058u64 => "
      TIMER.arrayreg()[2],
    ",
  0x4001005cu64 => "
      TIMER.arrayreg()[3],
    ",
  0x40010100u64 => "
      TIMER.clusterdim()[1].cr(),
    ",
  0x40010200u64 => "
      TIMER.clusterdim()[2].cr(),
    ",
  0x40010300u64 => "
      TIMER.clusterdim()[3].cr(),
    ",
  0x50000000u64 => "
      UART[0].reg1_()[0],
    ",
  0x50000100u64 => "
      UART[0].regbitfieldraw(),
      UART[0].reg1_()[1],
    ",
  0x50000104u64 => "
      UART[0].reg16bitenum(),
    ",
  0x50000106u64 => "
      UART[0].reg8bitraw(),
    ",
  0x50000107u64 => "
      UART[0].reg16bitraw(),
    ",
  0x50000109u64 => "
      UART[0].reg32bitraw(),
    ",
  0x50001000u64 => "
      UART[1].reg1_()[0],
    ",
  0x50001100u64 => "
      UART[1].regbitfieldraw(),
      UART[1].reg1_()[1],
    ",
  0x50001104u64 => "
      UART[1].reg16bitenum(),
    ",
  0x50001106u64 => "
      UART[1].reg8bitraw(),
    ",
  0x50001107u64 => "
      UART[1].reg16bitraw(),
    ",
  0x50001109u64 => "
      UART[1].reg32bitraw(),
    ",
  0x50002000u64 => "
      UART[2].reg1_()[0],
    ",
  0x50002100u64 => "
      UART[2].reg1_()[1],
      UART[2].regbitfieldraw(),
    ",
  0x50002104u64 => "
      UART[2].reg16bitenum(),
    ",
  0x50002106u64 => "
      UART[2].reg8bitraw(),
    ",
  0x50002107u64 => "
      UART[2].reg16bitraw(),
    ",
  0x50002109u64 => "
      UART[2].reg32bitraw(),
    ",
};
