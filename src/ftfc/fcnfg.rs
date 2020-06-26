#[doc = "Reader of register FCNFG"]
pub type R = crate::R<u8, super::FCNFG>;
#[doc = "Writer for register FCNFG"]
pub type W = crate::W<u8, super::FCNFG>;
#[doc = "Register FCNFG `reset()`'s with value 0x02"]
impl crate::ResetValue for super::FCNFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `EEERDY`"]
pub type EEERDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAMRDY`"]
pub type RAMRDY_R = crate::R<bool, bool>;
#[doc = "Erase Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERSSUSP_A {
    #[doc = "0: No suspend requested"]
    _0 = 0,
    #[doc = "1: Suspend the current Erase Flash Sector command execution"]
    _1 = 1,
}
impl From<ERSSUSP_A> for bool {
    #[inline(always)]
    fn from(variant: ERSSUSP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERSSUSP`"]
pub type ERSSUSP_R = crate::R<bool, ERSSUSP_A>;
impl ERSSUSP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERSSUSP_A {
        match self.bits {
            false => ERSSUSP_A::_0,
            true => ERSSUSP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERSSUSP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERSSUSP_A::_1
    }
}
#[doc = "Write proxy for field `ERSSUSP`"]
pub struct ERSSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> ERSSUSP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERSSUSP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No suspend requested"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERSSUSP_A::_0)
    }
    #[doc = "Suspend the current Erase Flash Sector command execution"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERSSUSP_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Erase All Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERSAREQ_A {
    #[doc = "0: No request or request complete"]
    _0 = 0,
}
impl From<ERSAREQ_A> for bool {
    #[inline(always)]
    fn from(variant: ERSAREQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERSAREQ`"]
pub type ERSAREQ_R = crate::R<bool, ERSAREQ_A>;
impl ERSAREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, ERSAREQ_A> {
        use crate::Variant::*;
        match self.bits {
            false => Val(ERSAREQ_A::_0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERSAREQ_A::_0
    }
}
#[doc = "Read Collision Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDCOLLIE_A {
    #[doc = "0: Read collision error interrupt disabled"]
    _0 = 0,
    #[doc = "1: Read collision error interrupt enabled. An interrupt request is generated whenever an FTFC read collision error is detected (see the description of FSTAT\\[RDCOLERR\\])."]
    _1 = 1,
}
impl From<RDCOLLIE_A> for bool {
    #[inline(always)]
    fn from(variant: RDCOLLIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDCOLLIE`"]
pub type RDCOLLIE_R = crate::R<bool, RDCOLLIE_A>;
impl RDCOLLIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDCOLLIE_A {
        match self.bits {
            false => RDCOLLIE_A::_0,
            true => RDCOLLIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDCOLLIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDCOLLIE_A::_1
    }
}
#[doc = "Write proxy for field `RDCOLLIE`"]
pub struct RDCOLLIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDCOLLIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDCOLLIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read collision error interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDCOLLIE_A::_0)
    }
    #[doc = "Read collision error interrupt enabled. An interrupt request is generated whenever an FTFC read collision error is detected (see the description of FSTAT\\[RDCOLERR\\])."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDCOLLIE_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Command Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCIE_A {
    #[doc = "0: Command complete interrupt disabled"]
    _0 = 0,
    #[doc = "1: Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\]
flag is set."]
    _1 = 1,
}
impl From<CCIE_A> for bool {
    #[inline(always)]
    fn from(variant: CCIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CCIE`"]
pub type CCIE_R = crate::R<bool, CCIE_A>;
impl CCIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIE_A {
        match self.bits {
            false => CCIE_A::_0,
            true => CCIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCIE_A::_1
    }
}
#[doc = "Write proxy for field `CCIE`"]
pub struct CCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Command complete interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCIE_A::_0)
    }
    #[doc = "Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\]
flag is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCIE_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - EEERDY"]
    #[inline(always)]
    pub fn eeerdy(&self) -> EEERDY_R {
        EEERDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RAM Ready"]
    #[inline(always)]
    pub fn ramrdy(&self) -> RAMRDY_R {
        RAMRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Erase Suspend"]
    #[inline(always)]
    pub fn erssusp(&self) -> ERSSUSP_R {
        ERSSUSP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Erase All Request"]
    #[inline(always)]
    pub fn ersareq(&self) -> ERSAREQ_R {
        ERSAREQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Read Collision Error Interrupt Enable"]
    #[inline(always)]
    pub fn rdcollie(&self) -> RDCOLLIE_R {
        RDCOLLIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ccie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Erase Suspend"]
    #[inline(always)]
    pub fn erssusp(&mut self) -> ERSSUSP_W {
        ERSSUSP_W { w: self }
    }
    #[doc = "Bit 6 - Read Collision Error Interrupt Enable"]
    #[inline(always)]
    pub fn rdcollie(&mut self) -> RDCOLLIE_W {
        RDCOLLIE_W { w: self }
    }
    #[doc = "Bit 7 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ccie(&mut self) -> CCIE_W {
        CCIE_W { w: self }
    }
}
