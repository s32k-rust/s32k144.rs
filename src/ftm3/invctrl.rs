#[doc = "Reader of register INVCTRL"]
pub type R = crate::R<u32, super::INVCTRL>;
#[doc = "Writer for register INVCTRL"]
pub type W = crate::W<u32, super::INVCTRL>;
#[doc = "Register INVCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::INVCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pair Channels 0 Inverting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV0EN_A {
    #[doc = "0: Inverting is disabled."]
    _0 = 0,
    #[doc = "1: Inverting is enabled."]
    _1 = 1,
}
impl From<INV0EN_A> for bool {
    #[inline(always)]
    fn from(variant: INV0EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INV0EN`"]
pub type INV0EN_R = crate::R<bool, INV0EN_A>;
impl INV0EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV0EN_A {
        match self.bits {
            false => INV0EN_A::_0,
            true => INV0EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INV0EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INV0EN_A::_1
    }
}
#[doc = "Write proxy for field `INV0EN`"]
pub struct INV0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INV0EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INV0EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Inverting is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV0EN_A::_0)
    }
    #[doc = "Inverting is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV0EN_A::_1)
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
#[doc = "Pair Channels 1 Inverting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV1EN_A {
    #[doc = "0: Inverting is disabled."]
    _0 = 0,
    #[doc = "1: Inverting is enabled."]
    _1 = 1,
}
impl From<INV1EN_A> for bool {
    #[inline(always)]
    fn from(variant: INV1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INV1EN`"]
pub type INV1EN_R = crate::R<bool, INV1EN_A>;
impl INV1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV1EN_A {
        match self.bits {
            false => INV1EN_A::_0,
            true => INV1EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INV1EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INV1EN_A::_1
    }
}
#[doc = "Write proxy for field `INV1EN`"]
pub struct INV1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INV1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INV1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Inverting is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV1EN_A::_0)
    }
    #[doc = "Inverting is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV1EN_A::_1)
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
#[doc = "Pair Channels 2 Inverting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV2EN_A {
    #[doc = "0: Inverting is disabled."]
    _0 = 0,
    #[doc = "1: Inverting is enabled."]
    _1 = 1,
}
impl From<INV2EN_A> for bool {
    #[inline(always)]
    fn from(variant: INV2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INV2EN`"]
pub type INV2EN_R = crate::R<bool, INV2EN_A>;
impl INV2EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV2EN_A {
        match self.bits {
            false => INV2EN_A::_0,
            true => INV2EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INV2EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INV2EN_A::_1
    }
}
#[doc = "Write proxy for field `INV2EN`"]
pub struct INV2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INV2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INV2EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Inverting is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV2EN_A::_0)
    }
    #[doc = "Inverting is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV2EN_A::_1)
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
#[doc = "Pair Channels 3 Inverting Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INV3EN_A {
    #[doc = "0: Inverting is disabled."]
    _0 = 0,
    #[doc = "1: Inverting is enabled."]
    _1 = 1,
}
impl From<INV3EN_A> for bool {
    #[inline(always)]
    fn from(variant: INV3EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INV3EN`"]
pub type INV3EN_R = crate::R<bool, INV3EN_A>;
impl INV3EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV3EN_A {
        match self.bits {
            false => INV3EN_A::_0,
            true => INV3EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INV3EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INV3EN_A::_1
    }
}
#[doc = "Write proxy for field `INV3EN`"]
pub struct INV3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INV3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INV3EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Inverting is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV3EN_A::_0)
    }
    #[doc = "Inverting is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV3EN_A::_1)
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
impl R {
    #[doc = "Bit 0 - Pair Channels 0 Inverting Enable"]
    #[inline(always)]
    pub fn inv0en(&self) -> INV0EN_R {
        INV0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pair Channels 1 Inverting Enable"]
    #[inline(always)]
    pub fn inv1en(&self) -> INV1EN_R {
        INV1EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pair Channels 2 Inverting Enable"]
    #[inline(always)]
    pub fn inv2en(&self) -> INV2EN_R {
        INV2EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pair Channels 3 Inverting Enable"]
    #[inline(always)]
    pub fn inv3en(&self) -> INV3EN_R {
        INV3EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pair Channels 0 Inverting Enable"]
    #[inline(always)]
    pub fn inv0en(&mut self) -> INV0EN_W {
        INV0EN_W { w: self }
    }
    #[doc = "Bit 1 - Pair Channels 1 Inverting Enable"]
    #[inline(always)]
    pub fn inv1en(&mut self) -> INV1EN_W {
        INV1EN_W { w: self }
    }
    #[doc = "Bit 2 - Pair Channels 2 Inverting Enable"]
    #[inline(always)]
    pub fn inv2en(&mut self) -> INV2EN_W {
        INV2EN_W { w: self }
    }
    #[doc = "Bit 3 - Pair Channels 3 Inverting Enable"]
    #[inline(always)]
    pub fn inv3en(&mut self) -> INV3EN_W {
        INV3EN_W { w: self }
    }
}
