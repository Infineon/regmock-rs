/*
CC0 1.0 Universal

CREATIVE COMMONS CORPORATION IS NOT A LAW FIRM AND DOES NOT PROVIDE LEGAL SERVICES.
DISTRIBUTION OF THIS DOCUMENT DOES NOT CREATE AN ATTORNEY-CLIENT RELATIONSHIP. CREATIVE
COMMONS PROVIDES THIS INFORMATION ON AN "AS-IS" BASIS. CREATIVE COMMONS MAKES NO WARRANTIES
REGARDING THE USE OF THIS DOCUMENT OR THE INFORMATION OR WORKS PROVIDED HEREUNDER, AND
DISCLAIMS LIABILITY FOR DAMAGES RESULTING FROM THE USE OF THIS DOCUMENT OR THE INFORMATION
OR WORKS PROVIDED HEREUNDER.
*/
#![cfg_attr(not(feature = "tracing"), no_std)]
#![allow(non_camel_case_types)]
#![doc = "Example for regmock test and demonstration purposes"]
pub mod common;
pub use common::*;

#[cfg(feature = "tracing")]
pub mod reg_name;
#[cfg(feature = "tracing")]
pub mod tracing;

#[cfg(feature = "gpio")]
pub mod gpio;
#[cfg(feature = "spi")]
pub mod spi;
#[cfg(feature = "timer")]
pub mod timer;

#[cfg(feature = "gpio")]
pub const GPIO: gpio::Gpio = gpio::Gpio(0x8400u32 as _);
#[cfg(feature = "spi")]
pub const SPI: spi::Spi = spi::Spi(0x8200u32 as _);
#[cfg(feature = "timer")]
pub const TIMER: timer::Timer = timer::Timer(0x8000u32 as _);
