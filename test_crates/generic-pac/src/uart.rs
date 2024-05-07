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
#[doc = "Test cluster"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart(pub(super) *mut u8);
unsafe impl core::marker::Send for Uart {}
unsafe impl core::marker::Sync for Uart {}
impl Uart {
#[doc = "read write reg enum"]
#[inline(always)]
pub const fn reg16bitenum(&self) -> crate::common::Reg<self::Reg16BitEnum, crate::common::RW> {
    unsafe { crate::common::Reg::from_ptr(self.0.add(260usize)) }
}

#[doc = "Read write without enum"]
#[inline(always)]
pub const fn reg16bitraw(&self) -> crate::common::Reg<self::Reg16BitRaw, crate::common::RW> {
    unsafe { crate::common::Reg::from_ptr(self.0.add(263usize)) }
}

#[doc = "read-write reg"]
#[inline(always)]
pub const fn reg1_(&self) -> [crate::common::Reg<self::Reg1, crate::common::RW>;2] {
    unsafe {  [crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x0usize )),
    crate::common::Reg::from_ptr(self.0.add(0x0usize + 0x100usize )),
    ] }
}

#[doc = "Read write without enum"]
#[inline(always)]
pub const fn reg32bitraw(&self) -> crate::common::Reg<self::Reg32BitRaw, crate::common::RW> {
    unsafe { crate::common::Reg::from_ptr(self.0.add(265usize)) }
}

#[doc = "Read write whithout enum"]
#[inline(always)]
pub const fn reg8bitraw(&self) -> crate::common::Reg<self::Reg8BitRaw, crate::common::RW> {
    unsafe { crate::common::Reg::from_ptr(self.0.add(262usize)) }
}

#[doc = "Register with bitfields without enumeration"]
#[inline(always)]
pub const fn regbitfieldraw(&self) -> crate::common::Reg<self::RegBitfieldRaw, crate::common::RW> {
    unsafe { crate::common::Reg::from_ptr(self.0.add(256usize)) }
}

}
#[derive(Copy,Clone,Eq, PartialEq)]
pub struct Reg16BitEnum(u16,u16);
impl hidden::RegValue for Reg16BitEnum {
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
        Reg16BitEnum(data,write_mask)
    }
}impl Reg16BitEnum {
    #[doc = "Check when bitfield size is not standard"]
    #[inline(always)]
    pub fn bitfield9bitsenum(self) -> crate::common::RegisterField<0,0x1ff,1,0,reg16bitenum::Bitfield9BitsEnum, Reg16BitEnum,crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,reg16bitenum::Bitfield9BitsEnum, Reg16BitEnum,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Boolean with enum"]
    #[inline(always)]
    pub fn boolenum(self) -> crate::common::RegisterField<9,0xff,1,0,reg16bitenum::Boolenum, Reg16BitEnum,crate::common::RW> {
        crate::common::RegisterField::<9,0xff,1,0,reg16bitenum::Boolenum, Reg16BitEnum,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for Reg16BitEnum {
    #[inline(always)]
    fn default() -> Reg16BitEnum {
        Reg16BitEnum(0,0)
    }
}
pub mod reg16bitenum {
    use crate::hidden::CastFrom;
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bitfield9BitsEnum(pub u16); 
    impl Bitfield9BitsEnum {
        #[doc = ""]
        pub const VAL_0:Self =Self(0);
        #[doc = ""]
        pub const VAL_256:Self =Self(256);
    }
    impl From<Bitfield9BitsEnum> for u64 {
        #[inline(always)]
        fn from(value: Bitfield9BitsEnum) -> Self {
            value.0 as Self
        }
    }
    impl CastFrom<u64> for Bitfield9BitsEnum {
        #[inline(always)]
        fn cast_from(val:u64) -> Self {
            Self(val as u16)
        }
    }#[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Boolenum(pub u8); 
    impl Boolenum {
        #[doc = ""]
        pub const BOOL_0:Self =Self(0);
        #[doc = ""]
        pub const BOOL_1:Self =Self(1);
    }
    impl From<Boolenum> for u64 {
        #[inline(always)]
        fn from(value: Boolenum) -> Self {
            value.0 as Self
        }
    }
    impl CastFrom<u64> for Boolenum {
        #[inline(always)]
        fn cast_from(val:u64) -> Self {
            Self(val as u8)
        }
    }
}
#[derive(Copy,Clone,Eq, PartialEq)]
pub struct Reg16BitRaw(u16,u16);
impl hidden::RegValue for Reg16BitRaw {
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
        Reg16BitRaw(data,write_mask)
    }
}
impl NoBitfieldReg for Reg16BitRaw {}
impl core::default::Default for Reg16BitRaw {
    #[inline(always)]
    fn default() -> Reg16BitRaw {
        Reg16BitRaw(0,0)
    }
}

#[derive(Copy,Clone,Eq, PartialEq)]
pub struct Reg1(u32,u32);
impl hidden::RegValue for Reg1 {
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
        Reg1(data,write_mask)
    }
}
impl NoBitfieldReg for Reg1 {}
impl core::default::Default for Reg1 {
    #[inline(always)]
    fn default() -> Reg1 {
        Reg1(0,0)
    }
}

#[derive(Copy,Clone,Eq, PartialEq)]
pub struct Reg32BitRaw(u32,u32);
impl hidden::RegValue for Reg32BitRaw {
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
        Reg32BitRaw(data,write_mask)
    }
}
impl NoBitfieldReg for Reg32BitRaw {}
impl core::default::Default for Reg32BitRaw {
    #[inline(always)]
    fn default() -> Reg32BitRaw {
        Reg32BitRaw(0,0)
    }
}

#[derive(Copy,Clone,Eq, PartialEq)]
pub struct Reg8BitRaw(u8,u8);
impl hidden::RegValue for Reg8BitRaw {
    type DataType = u8;
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
        Reg8BitRaw(data,write_mask)
    }
}
impl NoBitfieldReg for Reg8BitRaw {}
impl core::default::Default for Reg8BitRaw {
    #[inline(always)]
    fn default() -> Reg8BitRaw {
        Reg8BitRaw(0,0)
    }
}

#[derive(Copy,Clone,Eq, PartialEq)]
pub struct RegBitfieldRaw(u32,u32);
impl hidden::RegValue for RegBitfieldRaw {
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
        RegBitfieldRaw(data,write_mask)
    }
}impl RegBitfieldRaw {
    #[doc = ""]
    #[inline(always)]
    pub fn bitfield9bits(self) -> crate::common::RegisterField<0,0x1ff,1,0,u16, RegBitfieldRaw,crate::common::RW> {
        crate::common::RegisterField::<0,0x1ff,1,0,u16, RegBitfieldRaw,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn bitfield17bits(self) -> crate::common::RegisterField<9,0x3ffff,1,0,u32, RegBitfieldRaw,crate::common::RW> {
        crate::common::RegisterField::<9,0x3ffff,1,0,u32, RegBitfieldRaw,crate::common::RW>::from_register(self,0)
    }
    #[doc = ""]
    #[inline(always)]
    pub fn bool(self) -> 
    crate::common::RegisterFieldBool<27,1,0,RegBitfieldRaw,crate::common::RW> {
        
    crate::common::RegisterFieldBool::<27,1,0,RegBitfieldRaw,crate::common::RW>::from_register(self,0)
    }
}
impl core::default::Default for RegBitfieldRaw {
    #[inline(always)]
    fn default() -> RegBitfieldRaw {
        RegBitfieldRaw(0,0)
    }
}








