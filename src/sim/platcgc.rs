#[doc = "Reader of register PLATCGC"]
pub type R = crate::R<u32, super::PLATCGC>;
#[doc = "Writer for register PLATCGC"]
pub type W = crate::W<u32, super::PLATCGC>;
#[doc = "Register PLATCGC `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::PLATCGC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "MSCM Clock Gating Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGCMSCM_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<CGCMSCM_A> for bool {
    #[inline(always)]
    fn from(variant: CGCMSCM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CGCMSCM`"]
pub type CGCMSCM_R = crate::R<bool, CGCMSCM_A>;
impl CGCMSCM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CGCMSCM_A {
        match self.bits {
            false => CGCMSCM_A::_0,
            true => CGCMSCM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CGCMSCM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CGCMSCM_A::_1
    }
}
#[doc = "Write proxy for field `CGCMSCM`"]
pub struct CGCMSCM_W<'a> {
    w: &'a mut W,
}
impl<'a> CGCMSCM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGCMSCM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CGCMSCM_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CGCMSCM_A::_1)
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
#[doc = "MPU Clock Gating Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGCMPU_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<CGCMPU_A> for bool {
    #[inline(always)]
    fn from(variant: CGCMPU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CGCMPU`"]
pub type CGCMPU_R = crate::R<bool, CGCMPU_A>;
impl CGCMPU_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CGCMPU_A {
        match self.bits {
            false => CGCMPU_A::_0,
            true => CGCMPU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CGCMPU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CGCMPU_A::_1
    }
}
#[doc = "Write proxy for field `CGCMPU`"]
pub struct CGCMPU_W<'a> {
    w: &'a mut W,
}
impl<'a> CGCMPU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGCMPU_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CGCMPU_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CGCMPU_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "DMA Clock Gating Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGCDMA_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<CGCDMA_A> for bool {
    #[inline(always)]
    fn from(variant: CGCDMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CGCDMA`"]
pub type CGCDMA_R = crate::R<bool, CGCDMA_A>;
impl CGCDMA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CGCDMA_A {
        match self.bits {
            false => CGCDMA_A::_0,
            true => CGCDMA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CGCDMA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CGCDMA_A::_1
    }
}
#[doc = "Write proxy for field `CGCDMA`"]
pub struct CGCDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> CGCDMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGCDMA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CGCDMA_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CGCDMA_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "ERM Clock Gating Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGCERM_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<CGCERM_A> for bool {
    #[inline(always)]
    fn from(variant: CGCERM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CGCERM`"]
pub type CGCERM_R = crate::R<bool, CGCERM_A>;
impl CGCERM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CGCERM_A {
        match self.bits {
            false => CGCERM_A::_0,
            true => CGCERM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CGCERM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CGCERM_A::_1
    }
}
#[doc = "Write proxy for field `CGCERM`"]
pub struct CGCERM_W<'a> {
    w: &'a mut W,
}
impl<'a> CGCERM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGCERM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CGCERM_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CGCERM_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "EIM Clock Gating Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGCEIM_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<CGCEIM_A> for bool {
    #[inline(always)]
    fn from(variant: CGCEIM_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CGCEIM`"]
pub type CGCEIM_R = crate::R<bool, CGCEIM_A>;
impl CGCEIM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CGCEIM_A {
        match self.bits {
            false => CGCEIM_A::_0,
            true => CGCEIM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CGCEIM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CGCEIM_A::_1
    }
}
#[doc = "Write proxy for field `CGCEIM`"]
pub struct CGCEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CGCEIM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGCEIM_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CGCEIM_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CGCEIM_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MSCM Clock Gating Control"]
    #[inline(always)]
    pub fn cgcmscm(&self) -> CGCMSCM_R {
        CGCMSCM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MPU Clock Gating Control"]
    #[inline(always)]
    pub fn cgcmpu(&self) -> CGCMPU_R {
        CGCMPU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA Clock Gating Control"]
    #[inline(always)]
    pub fn cgcdma(&self) -> CGCDMA_R {
        CGCDMA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ERM Clock Gating Control"]
    #[inline(always)]
    pub fn cgcerm(&self) -> CGCERM_R {
        CGCERM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EIM Clock Gating Control"]
    #[inline(always)]
    pub fn cgceim(&self) -> CGCEIM_R {
        CGCEIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MSCM Clock Gating Control"]
    #[inline(always)]
    pub fn cgcmscm(&mut self) -> CGCMSCM_W {
        CGCMSCM_W { w: self }
    }
    #[doc = "Bit 1 - MPU Clock Gating Control"]
    #[inline(always)]
    pub fn cgcmpu(&mut self) -> CGCMPU_W {
        CGCMPU_W { w: self }
    }
    #[doc = "Bit 2 - DMA Clock Gating Control"]
    #[inline(always)]
    pub fn cgcdma(&mut self) -> CGCDMA_W {
        CGCDMA_W { w: self }
    }
    #[doc = "Bit 3 - ERM Clock Gating Control"]
    #[inline(always)]
    pub fn cgcerm(&mut self) -> CGCERM_W {
        CGCERM_W { w: self }
    }
    #[doc = "Bit 4 - EIM Clock Gating Control"]
    #[inline(always)]
    pub fn cgceim(&mut self) -> CGCEIM_W {
        CGCEIM_W { w: self }
    }
}
