/*
Test license
 
*/
#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::{*};
#[allow(unused_imports)]
use crate::common::hidden;
#[doc = "Description of peripheral"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer(pub(super) *mut u8);
unsafe impl core::marker::Send for Timer {}
unsafe impl core::marker::Sync for Timer {}
impl Timer {
#[doc = "Array of register"]
#[inline(always)]
pub const fn arrayreg(&self) -> [crate::common::Reg<self::Arrayreg, crate::common::RW>;4] {
    unsafe {  [crate::common::Reg::from_ptr(self.0.add(0x50usize + 0x0usize )),
    crate::common::Reg::from_ptr(self.0.add(0x50usize + 0x4usize )),
    crate::common::Reg::from_ptr(self.0.add(0x50usize + 0x8usize )),
    crate::common::Reg::from_ptr(self.0.add(0x50usize + 0xcusize )),
    ] }
}

#[doc = "Register to test basic bitfield features"]
#[inline(always)]
pub const fn bitfield_reg(&self) -> crate::common::Reg<self::BitfieldReg, crate::common::RW> {
    unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
}

#[doc = "Another defintion register using alternate group"]
#[inline(always)]
pub const fn bitfield_reg_alt_group(&self) -> crate::common::Reg<self::BitfieldRegAltGroup, crate::common::RW> {
    unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
}

#[doc = "Interrupt Register"]
#[inline(always)]
pub const fn int(&self) -> crate::common::Reg<self::Int, crate::common::W> {
    unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
}

#[doc = "The Match Register stores the compare Value for the MATCH condition"]
#[inline(always)]
pub const fn r#match(&self) -> crate::common::Reg<self::Match, crate::common::RW> {
    unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
}

#[doc = "The Counter Register reflects the actual Value of the Timer/Counter"]
#[inline(always)]
pub const fn nobitfield_reg(&self) -> crate::common::Reg<self::NobitfieldReg, crate::common::RW> {
    unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
}

#[doc = "The Prescale Register stores the Value for the prescaler. The cont event gets divided by this value"]
#[inline(always)]
pub const fn prescale_rd(&self) -> crate::common::Reg<self::PrescaleRd, crate::common::R> {
    unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
}

#[doc = "The Prescale Register stores the Value for the prescaler. The cont event gets divided by this value"]
#[inline(always)]
pub const fn prescale_wr(&self) -> crate::common::Reg<self::PrescaleWr, crate::common::W> {
    unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
}

#[doc = "Status Register"]
#[inline(always)]
pub const fn sr(&self) -> crate::common::Reg<self::Sr, crate::common::R> {
    unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
}
#[doc = "Test Cluster"]
#[inline(always)]
pub fn cluster1(self) -> self::Cluster1{
    unsafe {   self::Cluster1(self.0.add(256usize)) }
}
#[doc = "Test Cluster array "]
#[inline(always)]
pub fn clusterdim(self) -> [self::ClusterDim;4] {
    unsafe {  [self::ClusterDim(self.0.add(0x200usize + 0x0usize)),
        self::ClusterDim(self.0.add(0x200usize + 0x100usize)),
        self::ClusterDim(self.0.add(0x200usize + 0x200usize)),
        self::ClusterDim(self.0.add(0x200usize + 0x300usize)),
        ] }
}

}
#[derive(Copy,Clone,Eq, PartialEq)]
pub struct Arrayreg(u32,u32);
impl hidden::RegValue for Arrayreg {
    type DataType = u32;
    #[inline(always)]
    fn data_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.0
    }
    #[inline(always)]
    fn data(&self) -> Self::DataType {
        self.0
    }
    #[inline(always)]
    fn get_mask_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.1
    }
    #[inline(always)]
    fn new(data: Self::DataType, write_mask: Self::DataType) -> Self {
        Arrayreg(data,write_mask)
    }
}
impl NoBitfieldReg for Arrayreg {}
impl core::default::Default for Arrayreg {
    #[inline(always)]
    fn default() -> Arrayreg {
        Arrayreg(0,0)
    }
}

#[derive(Copy,Clone,Eq, PartialEq)]
pub struct BitfieldReg(u32,u32);
impl hidden::RegValue for BitfieldReg {
    type DataType = u32;
    #[inline(always)]
    fn data_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.0
    }
    #[inline(always)]
    fn data(&self) -> Self::DataType {
        self.0
    }
    #[inline(always)]
    fn get_mask_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.1
    }
    #[inline(always)]
    fn new(data: Self::DataType, write_mask: Self::DataType) -> Self {
        BitfieldReg(data,write_mask)
    }
}impl BitfieldReg {
    #[doc = "Boolean Bitfield Read Only"]
    #[inline(always)]
    pub fn boolr(self) -> 
    crate::common::RegisterFieldBool<0,1,0,BitfieldReg,crate::common::R> {
        
    crate::common::RegisterFieldBool::<0,1,0,BitfieldReg,crate::common::R>::from_register(self,0)
    }
    #[doc = "Boolean Bitfield Write Only"]
    #[inline(always)]
    pub fn boolw(self) -> 
    crate::common::RegisterFieldBool<1,1,0,BitfieldReg,crate::common::W> {
        
    crate::common::RegisterFieldBool::<1,1,0,BitfieldReg,crate::common::W>::from_register(self,0)
    }
    #[doc = "Boolean bitfield Read Write"]
    #[inline(always)]
    pub fn boolrw(self) -> 
    crate::common::RegisterFieldBool<2,1,0,BitfieldReg,crate::common::RW> {
        
    crate::common::RegisterFieldBool::<2,1,0,BitfieldReg,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Raw Bitfield Read Only"]
    #[inline(always)]
    pub fn bitfieldr(self) -> crate::common::RegisterField<3,0x7,1,0,u8, BitfieldReg,crate::common::R> {
        crate::common::RegisterField::<3,0x7,1,0,u8, BitfieldReg,crate::common::R>::from_register(self,0)
    }
    #[doc = "Bitfield Raw Write Only"]
    #[inline(always)]
    pub fn bitfieldw(self) -> crate::common::RegisterField<6,0x3,1,0,u8, BitfieldReg,crate::common::W> {
        crate::common::RegisterField::<6,0x3,1,0,u8, BitfieldReg,crate::common::W>::from_register(self,0)
    }
    #[doc = "BitField Raw Read Write"]
    #[inline(always)]
    pub fn bitfieldrw(self) -> crate::common::RegisterField<8,0xf,1,0,u8, BitfieldReg,crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, BitfieldReg,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bitfield with enumerated field"]
    #[inline(always)]
    pub fn bitfieldenumerated(self) -> crate::common::RegisterField<12,0xf,1,0,bitfield_reg::BitfieldEnumerated, BitfieldReg,crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,bitfield_reg::BitfieldEnumerated, BitfieldReg,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Array of bitfields"]
    #[inline(always)]pub fn fieldarray(self,index:u8) -> crate::common::RegisterField<16,0x3,8,1,bitfield_reg::FieldArray, BitfieldReg,crate::common::RW> {
        assert!(index < 8);
        crate::common::RegisterField::<16,0x3,8,1,bitfield_reg::FieldArray, BitfieldReg,crate::common::RW>::from_register(self,index)
     
    }
}
impl core::default::Default for BitfieldReg {
    #[inline(always)]
    fn default() -> BitfieldReg {
        BitfieldReg(0,0)
    }
}
pub mod bitfield_reg {
    use crate::hidden::CastFrom;
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct BitfieldEnumerated(pub u8); 
    impl BitfieldEnumerated {
        #[doc = "Core Clock"]
        pub const C_CLK:Self =Self(0);
        #[doc = "GPIO A, PIN 0"]
        pub const GPIOA_0:Self =Self(1);
        #[doc = "GPIO A, PIN 1"]
        pub const GPIOA_1:Self =Self(2);
        #[doc = "GPIO A, PIN 2"]
        pub const GPIOA_2:Self =Self(3);
        #[doc = "GPIO A, PIN 3"]
        pub const GPIOA_3:Self =Self(4);
        #[doc = "GPIO A, PIN 4"]
        pub const GPIOA_4:Self =Self(5);
        #[doc = "GPIO A, PIN 5"]
        pub const GPIOA_5:Self =Self(6);
        #[doc = "GPIO A, PIN 6"]
        pub const GPIOA_6:Self =Self(7);
        #[doc = "GPIO A, PIN 7"]
        pub const GPIOA_7:Self =Self(8);
        #[doc = "GPIO B, PIN 0"]
        pub const GPIOB_0:Self =Self(9);
        #[doc = "GPIO B, PIN 1"]
        pub const GPIOB_1:Self =Self(10);
        #[doc = "GPIO B, PIN 2"]
        pub const GPIOB_2:Self =Self(11);
        #[doc = "GPIO B, PIN 3"]
        pub const GPIOB_3:Self =Self(12);
        #[doc = "GPIO C, PIN 0"]
        pub const GPIOC_0:Self =Self(13);
        #[doc = "GPIO C, PIN 1"]
        pub const GPIOC_5:Self =Self(14);
        #[doc = "GPIO C, PIN 2"]
        pub const GPIOC_6:Self =Self(15);
    }
    impl From<BitfieldEnumerated> for u64 {
        #[inline(always)]
        fn from(value: BitfieldEnumerated) -> Self {
            value.0 as Self
        }
    }
    impl CastFrom<u64> for BitfieldEnumerated {
        #[inline(always)]
        fn cast_from(val:u64) -> Self {
            Self(val as u8)
        }
    }#[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct FieldArray(pub u8); 
    impl FieldArray {
        #[doc = "Only rising edges result in a counter increment or decrement"]
        pub const RISING:Self =Self(0);
        #[doc = "Only falling edges result in a counter increment or decrement"]
        pub const FALLING:Self =Self(1);
        #[doc = "Rising and falling edges result in a counter increment or decrement"]
        pub const BOTH:Self =Self(2);
    }
    impl From<FieldArray> for u64 {
        #[inline(always)]
        fn from(value: FieldArray) -> Self {
            value.0 as Self
        }
    }
    impl CastFrom<u64> for FieldArray {
        #[inline(always)]
        fn cast_from(val:u64) -> Self {
            Self(val as u8)
        }
    }
}
#[derive(Copy,Clone,Eq, PartialEq)]
pub struct BitfieldRegAltGroup(u32,u32);
impl hidden::RegValue for BitfieldRegAltGroup {
    type DataType = u32;
    #[inline(always)]
    fn data_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.0
    }
    #[inline(always)]
    fn data(&self) -> Self::DataType {
        self.0
    }
    #[inline(always)]
    fn get_mask_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.1
    }
    #[inline(always)]
    fn new(data: Self::DataType, write_mask: Self::DataType) -> Self {
        BitfieldRegAltGroup(data,write_mask)
    }
}
impl NoBitfieldReg for BitfieldRegAltGroup {}
impl core::default::Default for BitfieldRegAltGroup {
    #[inline(always)]
    fn default() -> BitfieldRegAltGroup {
        BitfieldRegAltGroup(0,0)
    }
}

#[derive(Copy,Clone,Eq, PartialEq)]
pub struct Int(u16,u16);
impl hidden::RegValue for Int {
    type DataType = u16;
    #[inline(always)]
    fn data_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.0
    }
    #[inline(always)]
    fn data(&self) -> Self::DataType {
        self.0
    }
    #[inline(always)]
    fn get_mask_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.1
    }
    #[inline(always)]
    fn new(data: Self::DataType, write_mask: Self::DataType) -> Self {
        Int(data,write_mask)
    }
}impl Int {
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn en(self) -> crate::common::RegisterField<0,0x1,1,0,int::En, Int,crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,int::En, Int,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Mode, selects on which condition the Timer should generate an Interrupt"]
    #[inline(always)]
    pub fn mode(self) -> crate::common::RegisterField<4,0x7,1,0,int::Mode, Int,crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,int::Mode, Int,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Int {
    #[inline(always)]
    fn default() -> Int {
        Int(0,0)
    }
}
pub mod int {
    use crate::hidden::CastFrom;
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En(pub u8); 
    impl En {
        #[doc = "Timer does not generate Interrupts"]
        pub const DISABLED:Self =Self(0);
        #[doc = "Timer triggers the TIMERn Interrupt"]
        pub const ENABLE:Self =Self(1);
    }
    impl From<En> for u64 {
        #[inline(always)]
        fn from(value: En) -> Self {
            value.0 as Self
        }
    }
    impl CastFrom<u64> for En {
        #[inline(always)]
        fn cast_from(val:u64) -> Self {
            Self(val as u8)
        }
    }#[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mode(pub u8); 
    impl Mode {
        #[doc = "Timer generates an Interrupt when the MATCH condition is hit"]
        pub const MATCH:Self =Self(0);
        #[doc = "Timer generates an Interrupt when it underflows"]
        pub const UNDERFLOW:Self =Self(1);
        #[doc = "Timer generates an Interrupt when it overflows"]
        pub const OVERFLOW:Self =Self(2);
    }
    impl From<Mode> for u64 {
        #[inline(always)]
        fn from(value: Mode) -> Self {
            value.0 as Self
        }
    }
    impl CastFrom<u64> for Mode {
        #[inline(always)]
        fn cast_from(val:u64) -> Self {
            Self(val as u8)
        }
    }
}
#[derive(Copy,Clone,Eq, PartialEq)]
pub struct Match(u32,u32);
impl hidden::RegValue for Match {
    type DataType = u32;
    #[inline(always)]
    fn data_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.0
    }
    #[inline(always)]
    fn data(&self) -> Self::DataType {
        self.0
    }
    #[inline(always)]
    fn get_mask_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.1
    }
    #[inline(always)]
    fn new(data: Self::DataType, write_mask: Self::DataType) -> Self {
        Match(data,write_mask)
    }
}
impl NoBitfieldReg for Match {}
impl core::default::Default for Match {
    #[inline(always)]
    fn default() -> Match {
        Match(0,0)
    }
}

#[derive(Copy,Clone,Eq, PartialEq)]
pub struct NobitfieldReg(u32,u32);
impl hidden::RegValue for NobitfieldReg {
    type DataType = u32;
    #[inline(always)]
    fn data_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.0
    }
    #[inline(always)]
    fn data(&self) -> Self::DataType {
        self.0
    }
    #[inline(always)]
    fn get_mask_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.1
    }
    #[inline(always)]
    fn new(data: Self::DataType, write_mask: Self::DataType) -> Self {
        NobitfieldReg(data,write_mask)
    }
}
impl NoBitfieldReg for NobitfieldReg {}
impl core::default::Default for NobitfieldReg {
    #[inline(always)]
    fn default() -> NobitfieldReg {
        NobitfieldReg(0,0)
    }
}

#[derive(Copy,Clone,Eq, PartialEq)]
pub struct PrescaleRd(u32,u32);
impl hidden::RegValue for PrescaleRd {
    type DataType = u32;
    #[inline(always)]
    fn data_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.0
    }
    #[inline(always)]
    fn data(&self) -> Self::DataType {
        self.0
    }
    #[inline(always)]
    fn get_mask_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.1
    }
    #[inline(always)]
    fn new(data: Self::DataType, write_mask: Self::DataType) -> Self {
        PrescaleRd(data,write_mask)
    }
}
impl NoBitfieldReg for PrescaleRd {}
impl core::default::Default for PrescaleRd {
    #[inline(always)]
    fn default() -> PrescaleRd {
        PrescaleRd(0,0)
    }
}

#[derive(Copy,Clone,Eq, PartialEq)]
pub struct PrescaleWr(u32,u32);
impl hidden::RegValue for PrescaleWr {
    type DataType = u32;
    #[inline(always)]
    fn data_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.0
    }
    #[inline(always)]
    fn data(&self) -> Self::DataType {
        self.0
    }
    #[inline(always)]
    fn get_mask_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.1
    }
    #[inline(always)]
    fn new(data: Self::DataType, write_mask: Self::DataType) -> Self {
        PrescaleWr(data,write_mask)
    }
}
impl NoBitfieldReg for PrescaleWr {}
impl core::default::Default for PrescaleWr {
    #[inline(always)]
    fn default() -> PrescaleWr {
        PrescaleWr(0,0)
    }
}

#[derive(Copy,Clone,Eq, PartialEq)]
pub struct Sr(u16,u16);
impl hidden::RegValue for Sr {
    type DataType = u16;
    #[inline(always)]
    fn data_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.0
    }
    #[inline(always)]
    fn data(&self) -> Self::DataType {
        self.0
    }
    #[inline(always)]
    fn get_mask_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.1
    }
    #[inline(always)]
    fn new(data: Self::DataType, write_mask: Self::DataType) -> Self {
        Sr(data,write_mask)
    }
}impl Sr {
    #[doc = "Shows if Timer is running or not"]
    #[inline(always)]
    pub fn run(self) -> crate::common::RegisterField<0,0x1,1,0,sr::Run, Sr,crate::common::R> {
        crate::common::RegisterField::<0,0x1,1,0,sr::Run, Sr,crate::common::R>::from_register(self,0)
    }
    #[doc = "Shows if the MATCH was hit"]
    #[inline(always)]
    pub fn r#match(self) -> crate::common::RegisterField<8,0x1,1,0,sr::Match, Sr,crate::common::RW> {
        crate::common::RegisterField::<8,0x1,1,0,sr::Match, Sr,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Shows if an underflow occured. This flag is sticky"]
    #[inline(always)]
    pub fn un(self) -> crate::common::RegisterField<9,0x1,1,0,sr::Un, Sr,crate::common::RW> {
        crate::common::RegisterField::<9,0x1,1,0,sr::Un, Sr,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Shows if an overflow occured. This flag is sticky"]
    #[inline(always)]
    pub fn ov(self) -> crate::common::RegisterField<10,0x1,1,0,sr::Ov, Sr,crate::common::RW> {
        crate::common::RegisterField::<10,0x1,1,0,sr::Ov, Sr,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Shows if Timer is in RESET state"]
    #[inline(always)]
    pub fn rst(self) -> crate::common::RegisterField<12,0x1,1,0,sr::Rst, Sr,crate::common::R> {
        crate::common::RegisterField::<12,0x1,1,0,sr::Rst, Sr,crate::common::R>::from_register(self,0)
    }
    #[doc = "Shows the currently active RELOAD Register"]
    #[inline(always)]
    pub fn reload(self) -> crate::common::RegisterField<14,0x3,1,0,sr::Reload, Sr,crate::common::R> {
        crate::common::RegisterField::<14,0x3,1,0,sr::Reload, Sr,crate::common::R>::from_register(self,0)
    }
}
impl core::default::Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0,0)
    }
}
pub mod sr {
    use crate::hidden::CastFrom;
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Run(pub u8); 
    impl Run {
        #[doc = "Timer is not running"]
        pub const STOPPED:Self =Self(0);
        #[doc = "Timer is running"]
        pub const RUNNING:Self =Self(1);
    }
    impl From<Run> for u64 {
        #[inline(always)]
        fn from(value: Run) -> Self {
            value.0 as Self
        }
    }
    impl CastFrom<u64> for Run {
        #[inline(always)]
        fn cast_from(val:u64) -> Self {
            Self(val as u8)
        }
    }#[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Match(pub u8); 
    impl Match {
        #[doc = "The MATCH condition was not hit"]
        pub const NO_MATCH:Self =Self(0);
        #[doc = "The MATCH condition was hit"]
        pub const MATCH_HIT:Self =Self(1);
    }
    impl From<Match> for u64 {
        #[inline(always)]
        fn from(value: Match) -> Self {
            value.0 as Self
        }
    }
    impl CastFrom<u64> for Match {
        #[inline(always)]
        fn cast_from(val:u64) -> Self {
            Self(val as u8)
        }
    }#[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Un(pub u8); 
    impl Un {
        #[doc = "No underflow occured since last clear"]
        pub const NO_UNDERFLOW:Self =Self(0);
        #[doc = "A minimum of one underflow occured since last clear"]
        pub const UNDERFLOW:Self =Self(1);
    }
    impl From<Un> for u64 {
        #[inline(always)]
        fn from(value: Un) -> Self {
            value.0 as Self
        }
    }
    impl CastFrom<u64> for Un {
        #[inline(always)]
        fn cast_from(val:u64) -> Self {
            Self(val as u8)
        }
    }#[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ov(pub u8); 
    impl Ov {
        #[doc = "No overflow occured since last clear"]
        pub const NO_OVERFLOW:Self =Self(0);
        #[doc = "A minimum of one overflow occured since last clear"]
        pub const OVERFLOW_OCCURED:Self =Self(1);
    }
    impl From<Ov> for u64 {
        #[inline(always)]
        fn from(value: Ov) -> Self {
            value.0 as Self
        }
    }
    impl CastFrom<u64> for Ov {
        #[inline(always)]
        fn cast_from(val:u64) -> Self {
            Self(val as u8)
        }
    }#[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rst(pub u8); 
    impl Rst {
        #[doc = "Timer is not in RESET state and can operate"]
        pub const READY:Self =Self(0);
        #[doc = "Timer is in RESET state and can not operate"]
        pub const IN_RESET:Self =Self(1);
    }
    impl From<Rst> for u64 {
        #[inline(always)]
        fn from(value: Rst) -> Self {
            value.0 as Self
        }
    }
    impl CastFrom<u64> for Rst {
        #[inline(always)]
        fn cast_from(val:u64) -> Self {
            Self(val as u8)
        }
    }#[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Reload(pub u8); 
    impl Reload {
        #[doc = "Reload Register number 0 is active"]
        pub const RELOAD_0:Self =Self(0);
        #[doc = "Reload Register number 1 is active"]
        pub const RELOAD_1:Self =Self(1);
        #[doc = "Reload Register number 2 is active"]
        pub const RELOAD_2:Self =Self(2);
        #[doc = "Reload Register number 3 is active"]
        pub const RELOAD_3:Self =Self(3);
    }
    impl From<Reload> for u64 {
        #[inline(always)]
        fn from(value: Reload) -> Self {
            value.0 as Self
        }
    }
    impl CastFrom<u64> for Reload {
        #[inline(always)]
        fn cast_from(val:u64) -> Self {
            Self(val as u8)
        }
    }
}

#[doc = "Test Cluster"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cluster1(pub(super) *mut u8);
unsafe impl core::marker::Send for Cluster1 {}
unsafe impl core::marker::Sync for Cluster1 {}
impl Cluster1 {
    #[doc = ""]
#[inline(always)]
pub const fn cr(&self) -> crate::common::Reg<cluster1::Cr, crate::common::RW> {
    unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
}
    #[doc = "A cluster inside another cluster"]
#[inline(always)]
pub fn cluster1(self) -> cluster1::Cluster1{
    unsafe {   cluster1::Cluster1(self.0.add(256usize)) }
}
    }
pub mod cluster1 {
    #[allow(unused_imports)]
    use crate::common::{*};
    #[derive(Copy,Clone,Eq, PartialEq)]
pub struct Cr(u32,u32);
impl hidden::RegValue for Cr {
    type DataType = u32;
    #[inline(always)]
    fn data_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.0
    }
    #[inline(always)]
    fn data(&self) -> Self::DataType {
        self.0
    }
    #[inline(always)]
    fn get_mask_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.1
    }
    #[inline(always)]
    fn new(data: Self::DataType, write_mask: Self::DataType) -> Self {
        Cr(data,write_mask)
    }
}impl Cr {
    #[doc = ""]
    #[inline(always)]
    pub fn filed1(self) -> crate::common::RegisterField<0,0x7,1,0,u8, Cr,crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Cr,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn psc(self) -> crate::common::RegisterField<3,0x3,1,0,cr::Psc, Cr,crate::common::RW> {
        crate::common::RegisterField::<3,0x3,1,0,cr::Psc, Cr,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(0,0)
    }
}
pub mod cr {
    use crate::hidden::CastFrom;
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psc(pub u8); 
    impl Psc {
        #[doc = ""]
        pub const VAL_1:Self =Self(1);
    }
    impl From<Psc> for u64 {
        #[inline(always)]
        fn from(value: Psc) -> Self {
            value.0 as Self
        }
    }
    impl CastFrom<u64> for Psc {
        #[inline(always)]
        fn cast_from(val:u64) -> Self {
            Self(val as u8)
        }
    }
}
    #[doc = "A cluster inside another cluster"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cluster1(pub(super) *mut u8);
unsafe impl core::marker::Send for Cluster1 {}
unsafe impl core::marker::Sync for Cluster1 {}
impl Cluster1 {
    #[doc = ""]
#[inline(always)]
pub const fn nestedreg(&self) -> crate::common::Reg<cluster1::NestedReg, crate::common::RW> {
    unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
}
    }
pub mod cluster1 {
    #[allow(unused_imports)]
    use crate::common::{*};
    #[derive(Copy,Clone,Eq, PartialEq)]
pub struct NestedReg(u32,u32);
impl hidden::RegValue for NestedReg {
    type DataType = u32;
    #[inline(always)]
    fn data_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.0
    }
    #[inline(always)]
    fn data(&self) -> Self::DataType {
        self.0
    }
    #[inline(always)]
    fn get_mask_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.1
    }
    #[inline(always)]
    fn new(data: Self::DataType, write_mask: Self::DataType) -> Self {
        NestedReg(data,write_mask)
    }
}
impl NoBitfieldReg for NestedReg {}
impl core::default::Default for NestedReg {
    #[inline(always)]
    fn default() -> NestedReg {
        NestedReg(74565,0)
    }
}

    }
    }
#[doc = "Test Cluster array "]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClusterDim(pub(super) *mut u8);
unsafe impl core::marker::Send for ClusterDim {}
unsafe impl core::marker::Sync for ClusterDim {}
impl ClusterDim {
    #[doc = ""]
#[inline(always)]
pub const fn cr(&self) -> crate::common::Reg<clusterdim::Cr, crate::common::RW> {
    unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
}
    }
pub mod clusterdim {
    #[allow(unused_imports)]
    use crate::common::{*};
    #[derive(Copy,Clone,Eq, PartialEq)]
pub struct Cr(u32,u32);
impl hidden::RegValue for Cr {
    type DataType = u32;
    #[inline(always)]
    fn data_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.0
    }
    #[inline(always)]
    fn data(&self) -> Self::DataType {
        self.0
    }
    #[inline(always)]
    fn get_mask_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.1
    }
    #[inline(always)]
    fn new(data: Self::DataType, write_mask: Self::DataType) -> Self {
        Cr(data,write_mask)
    }
}impl Cr {
    #[doc = ""]
    #[inline(always)]
    pub fn filed1(self) -> crate::common::RegisterField<0,0x7,1,0,u8, Cr,crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Cr,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn psc(self) -> crate::common::RegisterField<3,0x3,1,0,cr::Psc, Cr,crate::common::RW> {
        crate::common::RegisterField::<3,0x3,1,0,cr::Psc, Cr,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Cr {
    #[inline(always)]
    fn default() -> Cr {
        Cr(4096,0)
    }
}
pub mod cr {
    use crate::hidden::CastFrom;
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Psc(pub u8); 
    impl Psc {
        #[doc = ""]
        pub const VAL_1:Self =Self(1);
    }
    impl From<Psc> for u64 {
        #[inline(always)]
        fn from(value: Psc) -> Self {
            value.0 as Self
        }
    }
    impl CastFrom<u64> for Psc {
        #[inline(always)]
        fn cast_from(val:u64) -> Self {
            Self(val as u8)
        }
    }
}
    }






