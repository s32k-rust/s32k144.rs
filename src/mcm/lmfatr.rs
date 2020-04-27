#[doc = "Reader of register LMFATR"]
pub type R = crate::R<u32, super::LMFATR>;
#[doc = "Reader of field `PEFPRT`"]
pub type PEFPRT_R = crate::R<u8, u8>;
#[doc = "Parity/ECC Fault Master Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PEFSIZE_A {
    #[doc = "0: 8-bit access"]
    _000 = 0,
    #[doc = "1: 16-bit access"]
    _001 = 1,
    #[doc = "2: 32-bit access"]
    _010 = 2,
    #[doc = "3: 64-bit access"]
    _011 = 3,
}
impl From<PEFSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PEFSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PEFSIZE`"]
pub type PEFSIZE_R = crate::R<u8, PEFSIZE_A>;
impl PEFSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PEFSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PEFSIZE_A::_000),
            1 => Val(PEFSIZE_A::_001),
            2 => Val(PEFSIZE_A::_010),
            3 => Val(PEFSIZE_A::_011),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PEFSIZE_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PEFSIZE_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PEFSIZE_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PEFSIZE_A::_011
    }
}
#[doc = "Reader of field `PEFW`"]
pub type PEFW_R = crate::R<bool, bool>;
#[doc = "Reader of field `PEFMST`"]
pub type PEFMST_R = crate::R<u8, u8>;
#[doc = "Reader of field `OVR`"]
pub type OVR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - Parity/ECC Fault Protection"]
    #[inline(always)]
    pub fn pefprt(&self) -> PEFPRT_R {
        PEFPRT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Parity/ECC Fault Master Size"]
    #[inline(always)]
    pub fn pefsize(&self) -> PEFSIZE_R {
        PEFSIZE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Parity/ECC Fault Write"]
    #[inline(always)]
    pub fn pefw(&self) -> PEFW_R {
        PEFW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Parity/ECC Fault Master Number"]
    #[inline(always)]
    pub fn pefmst(&self) -> PEFMST_R {
        PEFMST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Overrun"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
