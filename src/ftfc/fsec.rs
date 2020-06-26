#[doc = "Reader of register FSEC"]
pub type R = crate::R<u8, super::FSEC>;
#[doc = "Flash Security\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SEC_A {
    #[doc = "2: MCU security status is unsecure (The standard shipping condition of the FTFC is unsecure.)"]
    _10 = 2,
}
impl From<SEC_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SEC`"]
pub type SEC_R = crate::R<u8, SEC_A>;
impl SEC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SEC_A> {
        use crate::Variant::*;
        match self.bits {
            2 => Val(SEC_A::_10),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SEC_A::_10
    }
}
#[doc = "Factory Failure Analysis Access Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FSLACC_A {
    #[doc = "0: Factory access granted"]
    _00 = 0,
    #[doc = "3: Factory access granted"]
    _11 = 3,
}
impl From<FSLACC_A> for u8 {
    #[inline(always)]
    fn from(variant: FSLACC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FSLACC`"]
pub type FSLACC_R = crate::R<u8, FSLACC_A>;
impl FSLACC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FSLACC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FSLACC_A::_00),
            3 => Val(FSLACC_A::_11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FSLACC_A::_00
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FSLACC_A::_11
    }
}
#[doc = "Mass Erase Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MEEN_A {
    #[doc = "0: Mass erase is enabled"]
    _00 = 0,
    #[doc = "1: Mass erase is enabled"]
    _01 = 1,
    #[doc = "3: Mass erase is enabled"]
    _11 = 3,
}
impl From<MEEN_A> for u8 {
    #[inline(always)]
    fn from(variant: MEEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MEEN`"]
pub type MEEN_R = crate::R<u8, MEEN_A>;
impl MEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MEEN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MEEN_A::_00),
            1 => Val(MEEN_A::_01),
            3 => Val(MEEN_A::_11),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MEEN_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MEEN_A::_01
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MEEN_A::_11
    }
}
#[doc = "Backdoor Key Security Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEYEN_A {
    #[doc = "0: Backdoor key access disabled"]
    _00 = 0,
    #[doc = "1: Backdoor key access disabled (preferred KEYEN state to disable backdoor key access)"]
    _01 = 1,
    #[doc = "2: Backdoor key access enabled"]
    _10 = 2,
    #[doc = "3: Backdoor key access disabled"]
    _11 = 3,
}
impl From<KEYEN_A> for u8 {
    #[inline(always)]
    fn from(variant: KEYEN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `KEYEN`"]
pub type KEYEN_R = crate::R<u8, KEYEN_A>;
impl KEYEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> KEYEN_A {
        match self.bits {
            0 => KEYEN_A::_00,
            1 => KEYEN_A::_01,
            2 => KEYEN_A::_10,
            3 => KEYEN_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == KEYEN_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == KEYEN_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == KEYEN_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == KEYEN_A::_11
    }
}
impl R {
    #[doc = "Bits 0:1 - Flash Security"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Factory Failure Analysis Access Code"]
    #[inline(always)]
    pub fn fslacc(&self) -> FSLACC_R {
        FSLACC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Mass Erase Enable Bits"]
    #[inline(always)]
    pub fn meen(&self) -> MEEN_R {
        MEEN_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Backdoor Key Security Enable"]
    #[inline(always)]
    pub fn keyen(&self) -> KEYEN_R {
        KEYEN_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
