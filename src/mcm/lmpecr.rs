#[doc = "Reader of register LMPECR"]
pub type R = crate::R<u32, super::LMPECR>;
#[doc = "Writer for register LMPECR"]
pub type W = crate::W<u32, super::LMPECR>;
#[doc = "Register LMPECR `reset()`'s with value 0"]
impl crate::ResetValue for super::LMPECR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable RAM ECC Noncorrectable Reporting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERNCR_A {
    #[doc = "0: Reporting disabled"]
    _0 = 0,
    #[doc = "1: Reporting enabled"]
    _1 = 1,
}
impl From<ERNCR_A> for bool {
    #[inline(always)]
    fn from(variant: ERNCR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ERNCR`"]
pub type ERNCR_R = crate::R<bool, ERNCR_A>;
impl ERNCR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERNCR_A {
        match self.bits {
            false => ERNCR_A::_0,
            true => ERNCR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERNCR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERNCR_A::_1
    }
}
#[doc = "Write proxy for field `ERNCR`"]
pub struct ERNCR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERNCR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERNCR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reporting disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERNCR_A::_0)
    }
    #[doc = "Reporting enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERNCR_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Enable RAM ECC 1 Bit Reporting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ER1BR_A {
    #[doc = "0: Reporting disabled"]
    _0 = 0,
    #[doc = "1: Reporting enabled"]
    _1 = 1,
}
impl From<ER1BR_A> for bool {
    #[inline(always)]
    fn from(variant: ER1BR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ER1BR`"]
pub type ER1BR_R = crate::R<bool, ER1BR_A>;
impl ER1BR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ER1BR_A {
        match self.bits {
            false => ER1BR_A::_0,
            true => ER1BR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ER1BR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ER1BR_A::_1
    }
}
#[doc = "Write proxy for field `ER1BR`"]
pub struct ER1BR_W<'a> {
    w: &'a mut W,
}
impl<'a> ER1BR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ER1BR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reporting disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ER1BR_A::_0)
    }
    #[doc = "Reporting enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ER1BR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Enable Cache Parity Reporting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECPR_A {
    #[doc = "0: Reporting disabled"]
    _0 = 0,
    #[doc = "1: Reporting enabled"]
    _1 = 1,
}
impl From<ECPR_A> for bool {
    #[inline(always)]
    fn from(variant: ECPR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ECPR`"]
pub type ECPR_R = crate::R<bool, ECPR_A>;
impl ECPR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECPR_A {
        match self.bits {
            false => ECPR_A::_0,
            true => ECPR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECPR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECPR_A::_1
    }
}
#[doc = "Write proxy for field `ECPR`"]
pub struct ECPR_W<'a> {
    w: &'a mut W,
}
impl<'a> ECPR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ECPR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reporting disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECPR_A::_0)
    }
    #[doc = "Reporting enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECPR_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable RAM ECC Noncorrectable Reporting"]
    #[inline(always)]
    pub fn erncr(&self) -> ERNCR_R {
        ERNCR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable RAM ECC 1 Bit Reporting"]
    #[inline(always)]
    pub fn er1br(&self) -> ER1BR_R {
        ER1BR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable Cache Parity Reporting"]
    #[inline(always)]
    pub fn ecpr(&self) -> ECPR_R {
        ECPR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable RAM ECC Noncorrectable Reporting"]
    #[inline(always)]
    pub fn erncr(&mut self) -> ERNCR_W {
        ERNCR_W { w: self }
    }
    #[doc = "Bit 8 - Enable RAM ECC 1 Bit Reporting"]
    #[inline(always)]
    pub fn er1br(&mut self) -> ER1BR_W {
        ER1BR_W { w: self }
    }
    #[doc = "Bit 20 - Enable Cache Parity Reporting"]
    #[inline(always)]
    pub fn ecpr(&mut self) -> ECPR_W {
        ECPR_W { w: self }
    }
}
