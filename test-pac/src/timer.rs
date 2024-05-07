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
#[doc = r"Timer Peripheral"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer(pub(super) *mut u8);
unsafe impl core::marker::Send for Timer {}
unsafe impl core::marker::Sync for Timer {}
impl Timer {
    #[doc = ""]
    #[inline(always)]
    pub fn timercluster(self) -> [self::Timercluster; 2] {
        unsafe {
            [
                self::Timercluster(self.0.add(0x0usize + 0x0usize)),
                self::Timercluster(self.0.add(0x0usize + 0xcusize)),
            ]
        }
    }
}

#[doc = ""]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timercluster(pub(super) *mut u8);
unsafe impl core::marker::Send for Timercluster {}
unsafe impl core::marker::Sync for Timercluster {}
impl Timercluster {
    #[doc = ""]
    #[inline(always)]
    pub const fn count(&self) -> crate::common::Reg<timercluster::Count_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = ""]
    #[inline(always)]
    pub const fn ctrlstat(
        &self,
    ) -> crate::common::Reg<timercluster::Ctrlstat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = ""]
    #[inline(always)]
    pub const fn max(&self) -> crate::common::Reg<timercluster::Max_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
pub mod timercluster {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Count_SPEC;
    impl crate::sealed::RegSpec for Count_SPEC {
        type DataType = u32;
    }
    #[doc = ""]
    pub type Count = crate::RegValueT<Count_SPEC>;

    impl Count {
        #[doc = "The count register stores the actual counter value"]
        #[inline(always)]
        pub fn value(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Count_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, Count_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Count {
        #[inline(always)]
        fn default() -> Count {
            <crate::RegValueT<Count_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrlstat_SPEC;
    impl crate::sealed::RegSpec for Ctrlstat_SPEC {
        type DataType = u32;
    }
    #[doc = ""]
    pub type Ctrlstat = crate::RegValueT<Ctrlstat_SPEC>;

    impl Ctrlstat {
        #[doc = "Enable Timer / Counter"]
        #[inline(always)]
        pub fn enable(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, Ctrlstat_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,Ctrlstat_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Asynchronous signals get synchronized with the\n                                    Timer/Counter clock"]
        #[inline(always)]
        pub fn reset_in(
            self,
        ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, Ctrlstat_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0x7,1,0,u8, Ctrlstat_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Select external clock source"]
        #[inline(always)]
        pub fn clock(
            self,
        ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, Ctrlstat_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x7,1,0,u8, Ctrlstat_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "If set, an interrupt will be triggered if an underflow\n                                    occurs"]
        #[inline(always)]
        pub fn underflow_irq(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, Ctrlstat_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,Ctrlstat_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Ctrlstat {
        #[inline(always)]
        fn default() -> Ctrlstat {
            <crate::RegValueT<Ctrlstat_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Max_SPEC;
    impl crate::sealed::RegSpec for Max_SPEC {
        type DataType = u32;
    }
    #[doc = ""]
    pub type Max = crate::RegValueT<Max_SPEC>;

    impl Max {
        #[doc = "The max register contains the maximal value of the\n                                    counter (count). On a Timer/Counter reset this value is copied\n                                    into count."]
        #[inline(always)]
        pub fn value(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Max_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, Max_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl core::default::Default for Max {
        #[inline(always)]
        fn default() -> Max {
            <crate::RegValueT<Max_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
