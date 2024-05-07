/*
CC0 1.0 Universal

CREATIVE COMMONS CORPORATION IS NOT A LAW FIRM AND DOES NOT PROVIDE LEGAL SERVICES.
DISTRIBUTION OF THIS DOCUMENT DOES NOT CREATE AN ATTORNEY-CLIENT RELATIONSHIP. CREATIVE
COMMONS PROVIDES THIS INFORMATION ON AN "AS-IS" BASIS. CREATIVE COMMONS MAKES NO WARRANTIES
REGARDING THE USE OF THIS DOCUMENT OR THE INFORMATION OR WORKS PROVIDED HEREUNDER, AND
DISCLAIMS LIABILITY FOR DAMAGES RESULTING FROM THE USE OF THIS DOCUMENT OR THE INFORMATION
OR WORKS PROVIDED HEREUNDER.
*/
#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"SPI Slave peripheral"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi(pub(super) *mut u8);
unsafe impl core::marker::Send for Spi {}
unsafe impl core::marker::Sync for Spi {}
impl Spi {
    #[doc = "Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> crate::common::Reg<self::Ctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }

    #[doc = "Receive FIFO"]
    #[inline(always)]
    pub const fn rx(&self) -> crate::common::Reg<self::Rx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }

    #[doc = "Status register"]
    #[inline(always)]
    pub const fn status(&self) -> crate::common::Reg<self::Status_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }

    #[doc = "Transmit FIFO"]
    #[inline(always)]
    pub const fn tx(&self) -> crate::common::Reg<self::Tx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl_SPEC;
impl crate::sealed::RegSpec for Ctrl_SPEC {
    type DataType = u32;
}
#[doc = "Control register"]
pub type Ctrl = crate::RegValueT<Ctrl_SPEC>;

impl Ctrl {
    #[doc = "Global module enable"]
    #[inline(always)]
    pub fn en(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "0 to latch on first edge, 1 to shift"]
    #[inline(always)]
    pub fn cpha(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ctrl::Cpha, Ctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,ctrl::Cpha, Ctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock idle polarity"]
    #[inline(always)]
    pub fn cpol(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        <crate::RegValueT<Ctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctrl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cpha_SPEC;
    pub type Cpha = crate::EnumBitfieldStruct<u8, Cpha_SPEC>;
    impl Cpha {
        #[doc = "The first edge of the clock latches data, first bit\n                                        is driven from CS"]
        pub const CPHA_FIRST_LATCHES: Self = Self::new(0);
        #[doc = "The first edge shifts data, CS assertion does not\n                                        change data output"]
        pub const CPHA_FIRST_SHIFTS: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rx_SPEC;
impl crate::sealed::RegSpec for Rx_SPEC {
    type DataType = u32;
}
#[doc = "Receive FIFO"]
pub type Rx = crate::RegValueT<Rx_SPEC>;

impl Rx {
    #[doc = "Read data from RX FIFO"]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Rx_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Rx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "A byte was available when the read occured"]
    #[inline(always)]
    pub fn valid(self) -> crate::common::RegisterFieldBool<31, 1, 0, Rx_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Rx_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Rx {
    #[inline(always)]
    fn default() -> Rx {
        <crate::RegValueT<Rx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status_SPEC;
impl crate::sealed::RegSpec for Status_SPEC {
    type DataType = u32;
}
#[doc = "Status register"]
pub type Status = crate::RegValueT<Status_SPEC>;

impl Status {
    #[doc = "A byte is being transmitted"]
    #[inline(always)]
    pub fn busy(self) -> crate::common::RegisterFieldBool<0, 1, 0, Status_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Status_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "The device is selected via CS"]
    #[inline(always)]
    pub fn selected(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Status_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Status_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "The RX FIFO is empty"]
    #[inline(always)]
    pub fn rxe(self) -> crate::common::RegisterFieldBool<2, 1, 0, Status_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Status_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "The RX FIFO is not full"]
    #[inline(always)]
    pub fn rxnf(self) -> crate::common::RegisterFieldBool<3, 1, 0, Status_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Status_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "The RX FIFO overflowed since the last clear"]
    #[inline(always)]
    pub fn rxovfl(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Status_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Status_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "The TX FIFO is full"]
    #[inline(always)]
    pub fn txf(self) -> crate::common::RegisterFieldBool<5, 1, 0, Status_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Status_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "The TX FIFO is not empty"]
    #[inline(always)]
    pub fn txne(self) -> crate::common::RegisterFieldBool<6, 1, 0, Status_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Status_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Number of bytes to be read from the RX FIFO"]
    #[inline(always)]
    pub fn rx_fill(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Status_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Status_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Number of bytes to be sent from the TX FIFO"]
    #[inline(always)]
    pub fn tx_fill(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Status_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Status_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Write 1 to clear the rxovfl bit"]
    #[inline(always)]
    pub fn clear_rxovfl(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Status_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Status_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Write 1 to empty TX and RX FIFO"]
    #[inline(always)]
    pub fn flush(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Status_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Status_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Status {
    #[inline(always)]
    fn default() -> Status {
        <crate::RegValueT<Status_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tx_SPEC;
impl crate::sealed::RegSpec for Tx_SPEC {
    type DataType = u32;
}
#[doc = "Transmit FIFO"]
pub type Tx = crate::RegValueT<Tx_SPEC>;

impl Tx {
    #[doc = "Write to TX FIFO"]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Tx_SPEC, crate::common::W> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Tx_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Tx {
    #[inline(always)]
    fn default() -> Tx {
        <crate::RegValueT<Tx_SPEC> as RegisterValue<_>>::new(0)
    }
}
