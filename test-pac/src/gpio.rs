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
#[doc = r"GPIO"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio(pub(super) *mut u8);
unsafe impl core::marker::Send for Gpio {}
unsafe impl core::marker::Sync for Gpio {}
impl Gpio {
    #[doc = ""]
    #[inline(always)]
    pub const fn r#in(&self) -> crate::common::Reg<self::In_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn out(&self) -> crate::common::Reg<self::Out_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }

    #[doc = ""]
    #[inline(always)]
    pub const fn we(&self) -> crate::common::Reg<self::We_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct In_SPEC;
impl crate::sealed::RegSpec for In_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type In = crate::RegValueT<In_SPEC>;

impl In {
    #[doc = "Level of GPIO0"]
    #[inline(always)]
    pub fn gpio0(self) -> crate::common::RegisterFieldBool<0, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Level of GPIO1"]
    #[inline(always)]
    pub fn gpio1(self) -> crate::common::RegisterFieldBool<1, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Level of GPIO2"]
    #[inline(always)]
    pub fn gpio2(self) -> crate::common::RegisterFieldBool<2, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Level of GPIO3"]
    #[inline(always)]
    pub fn gpio3(self) -> crate::common::RegisterFieldBool<3, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Level of GPIO4"]
    #[inline(always)]
    pub fn gpio4(self) -> crate::common::RegisterFieldBool<4, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Level of GPIO5"]
    #[inline(always)]
    pub fn gpio5(self) -> crate::common::RegisterFieldBool<5, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Level of GPIO6"]
    #[inline(always)]
    pub fn gpio6(self) -> crate::common::RegisterFieldBool<6, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Level of GPIO7"]
    #[inline(always)]
    pub fn gpio7(self) -> crate::common::RegisterFieldBool<7, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Level of GPIO8"]
    #[inline(always)]
    pub fn gpio8(self) -> crate::common::RegisterFieldBool<8, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Level of GPIO9"]
    #[inline(always)]
    pub fn gpio9(self) -> crate::common::RegisterFieldBool<9, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Level of GPIO10"]
    #[inline(always)]
    pub fn gpio10(self) -> crate::common::RegisterFieldBool<10, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Level of GPIO11"]
    #[inline(always)]
    pub fn gpio11(self) -> crate::common::RegisterFieldBool<11, 1, 0, In_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, In_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for In {
    #[inline(always)]
    fn default() -> In {
        <crate::RegValueT<In_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Out_SPEC;
impl crate::sealed::RegSpec for Out_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type Out = crate::RegValueT<Out_SPEC>;

impl Out {
    #[doc = "Level to drive on GPIO0 if selected and enabled"]
    #[inline(always)]
    pub fn gpio0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Level to drive on GPIO1 if selected and enabled"]
    #[inline(always)]
    pub fn gpio1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Level to drive on GPIO2 if selected and enabled"]
    #[inline(always)]
    pub fn gpio2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Level to drive on GPIO3 if selected and enabled"]
    #[inline(always)]
    pub fn gpio3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Level to drive on GPIO4 if selected and enabled"]
    #[inline(always)]
    pub fn gpio4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Level to drive on GPIO5 if selected and enabled"]
    #[inline(always)]
    pub fn gpio5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Level to drive on GPIO6 if selected and enabled"]
    #[inline(always)]
    pub fn gpio6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Level to drive on GPIO7 if selected and enabled"]
    #[inline(always)]
    pub fn gpio7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Level to drive on GPIO8 if selected and enabled"]
    #[inline(always)]
    pub fn gpio8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Level to drive on GPIO9 if selected and enabled"]
    #[inline(always)]
    pub fn gpio9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Level to drive on GPIO10 if selected and enabled"]
    #[inline(always)]
    pub fn gpio10(self) -> crate::common::RegisterFieldBool<10, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Level to drive on GPIO11 if selected and enabled"]
    #[inline(always)]
    pub fn gpio11(self) -> crate::common::RegisterFieldBool<11, 1, 0, Out_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Out_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl core::default::Default for Out {
    #[inline(always)]
    fn default() -> Out {
        <crate::RegValueT<Out_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct We_SPEC;
impl crate::sealed::RegSpec for We_SPEC {
    type DataType = u32;
}
#[doc = ""]
pub type We = crate::RegValueT<We_SPEC>;

impl We {
    #[doc = "Direction of GPIO0"]
    #[inline(always)]
    pub fn gpio0(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, we::Gpio0, We_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,we::Gpio0, We_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Direction of GPIO1"]
    #[inline(always)]
    pub fn gpio1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, we::Gpio1, We_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,we::Gpio1, We_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Direction of GPIO2"]
    #[inline(always)]
    pub fn gpio2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, we::Gpio2, We_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,we::Gpio2, We_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Direction of GPIO3"]
    #[inline(always)]
    pub fn gpio3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, we::Gpio3, We_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,we::Gpio3, We_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Direction of GPIO4"]
    #[inline(always)]
    pub fn gpio4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, we::Gpio4, We_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x1,1,0,we::Gpio4, We_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Direction of GPIO5"]
    #[inline(always)]
    pub fn gpio5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, we::Gpio5, We_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x1,1,0,we::Gpio5, We_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Direction of GPIO6"]
    #[inline(always)]
    pub fn gpio6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, we::Gpio6, We_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x1,1,0,we::Gpio6, We_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Direction of GPIO7"]
    #[inline(always)]
    pub fn gpio7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, we::Gpio7, We_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x1,1,0,we::Gpio7, We_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Direction of GPIO8"]
    #[inline(always)]
    pub fn gpio8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, we::Gpio8, We_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,we::Gpio8, We_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Direction of GPIO9"]
    #[inline(always)]
    pub fn gpio9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, we::Gpio9, We_SPEC, crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,we::Gpio9, We_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Direction of GPIO10"]
    #[inline(always)]
    pub fn gpio10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, we::Gpio10, We_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x1,1,0,we::Gpio10, We_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Direction of GPIO11"]
    #[inline(always)]
    pub fn gpio11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, we::Gpio11, We_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x1,1,0,we::Gpio11, We_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for We {
    #[inline(always)]
    fn default() -> We {
        <crate::RegValueT<We_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod we {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpio0_SPEC;
    pub type Gpio0 = crate::EnumBitfieldStruct<u8, Gpio0_SPEC>;
    impl Gpio0 {
        #[doc = ""]
        pub const IN: Self = Self::new(0);
        #[doc = ""]
        pub const OUT: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpio1_SPEC;
    pub type Gpio1 = crate::EnumBitfieldStruct<u8, Gpio1_SPEC>;
    impl Gpio1 {
        #[doc = ""]
        pub const IN: Self = Self::new(0);
        #[doc = ""]
        pub const OUT: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpio2_SPEC;
    pub type Gpio2 = crate::EnumBitfieldStruct<u8, Gpio2_SPEC>;
    impl Gpio2 {
        #[doc = ""]
        pub const IN: Self = Self::new(0);
        #[doc = ""]
        pub const OUT: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpio3_SPEC;
    pub type Gpio3 = crate::EnumBitfieldStruct<u8, Gpio3_SPEC>;
    impl Gpio3 {
        #[doc = ""]
        pub const IN: Self = Self::new(0);
        #[doc = ""]
        pub const OUT: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpio4_SPEC;
    pub type Gpio4 = crate::EnumBitfieldStruct<u8, Gpio4_SPEC>;
    impl Gpio4 {
        #[doc = ""]
        pub const IN: Self = Self::new(0);
        #[doc = ""]
        pub const OUT: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpio5_SPEC;
    pub type Gpio5 = crate::EnumBitfieldStruct<u8, Gpio5_SPEC>;
    impl Gpio5 {
        #[doc = ""]
        pub const IN: Self = Self::new(0);
        #[doc = ""]
        pub const OUT: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpio6_SPEC;
    pub type Gpio6 = crate::EnumBitfieldStruct<u8, Gpio6_SPEC>;
    impl Gpio6 {
        #[doc = ""]
        pub const IN: Self = Self::new(0);
        #[doc = ""]
        pub const OUT: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpio7_SPEC;
    pub type Gpio7 = crate::EnumBitfieldStruct<u8, Gpio7_SPEC>;
    impl Gpio7 {
        #[doc = ""]
        pub const IN: Self = Self::new(0);
        #[doc = ""]
        pub const OUT: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpio8_SPEC;
    pub type Gpio8 = crate::EnumBitfieldStruct<u8, Gpio8_SPEC>;
    impl Gpio8 {
        #[doc = ""]
        pub const IN: Self = Self::new(0);
        #[doc = ""]
        pub const OUT: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpio9_SPEC;
    pub type Gpio9 = crate::EnumBitfieldStruct<u8, Gpio9_SPEC>;
    impl Gpio9 {
        #[doc = ""]
        pub const IN: Self = Self::new(0);
        #[doc = ""]
        pub const OUT: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpio10_SPEC;
    pub type Gpio10 = crate::EnumBitfieldStruct<u8, Gpio10_SPEC>;
    impl Gpio10 {
        #[doc = ""]
        pub const IN: Self = Self::new(0);
        #[doc = ""]
        pub const OUT: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Gpio11_SPEC;
    pub type Gpio11 = crate::EnumBitfieldStruct<u8, Gpio11_SPEC>;
    impl Gpio11 {
        #[doc = ""]
        pub const IN: Self = Self::new(0);
        #[doc = ""]
        pub const OUT: Self = Self::new(1);
    }
}
