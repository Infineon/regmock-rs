/*
Test license
 
*/
#![cfg_attr(not(feature = "tracing"), no_std)]
#![doc = "SVD Test for Rust PAC generator"]
pub mod common;
pub use common::*;

#[cfg(feature = "tracing")]
pub mod reg_name;
#[cfg(feature = "tracing")]
pub mod tracing;

#[cfg(feature = "timer")]
pub mod timer;
#[cfg(feature = "uart")]
pub mod uart;
#[cfg(feature = "timer")]
pub const TIMER: timer::Timer = timer::Timer(0x40010000u32 as _);
#[cfg(feature = "uart")]
pub const UART:[uart::Uart;3] = [  uart::Uart(0x50000000u32 as _),   uart::Uart(0x50001000u32 as _),   uart::Uart(0x50002000u32 as _), ];