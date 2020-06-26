#[doc = "Reader of register EICHEN"]
pub type R = crate::R<u32, super::EICHEN>;
#[doc = "Writer for register EICHEN"]
pub type W = crate::W<u32, super::EICHEN>;
#[doc = "Register EICHEN `reset()`'s with value 0"]
impl crate::ResetValue for super::EICHEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Error Injection Channel 1 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EICH1EN_A {
    #[doc = "0: Error injection is disabled on Error Injection Channel 1"]
    _0 = 0,
    #[doc = "1: Error injection is enabled on Error Injection Channel 1"]
    _1 = 1,
}
impl From<EICH1EN_A> for bool {
    #[inline(always)]
    fn from(variant: EICH1EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EICH1EN`"]
pub type EICH1EN_R = crate::R<bool, EICH1EN_A>;
impl EICH1EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EICH1EN_A {
        match self.bits {
            false => EICH1EN_A::_0,
            true => EICH1EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EICH1EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EICH1EN_A::_1
    }
}
#[doc = "Write proxy for field `EICH1EN`"]
pub struct EICH1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EICH1EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EICH1EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Error injection is disabled on Error Injection Channel 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EICH1EN_A::_0)
    }
    #[doc = "Error injection is enabled on Error Injection Channel 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EICH1EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Error Injection Channel 0 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EICH0EN_A {
    #[doc = "0: Error injection is disabled on Error Injection Channel 0"]
    _0 = 0,
    #[doc = "1: Error injection is enabled on Error Injection Channel 0"]
    _1 = 1,
}
impl From<EICH0EN_A> for bool {
    #[inline(always)]
    fn from(variant: EICH0EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EICH0EN`"]
pub type EICH0EN_R = crate::R<bool, EICH0EN_A>;
impl EICH0EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EICH0EN_A {
        match self.bits {
            false => EICH0EN_A::_0,
            true => EICH0EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EICH0EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EICH0EN_A::_1
    }
}
#[doc = "Write proxy for field `EICH0EN`"]
pub struct EICH0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EICH0EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EICH0EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Error injection is disabled on Error Injection Channel 0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EICH0EN_A::_0)
    }
    #[doc = "Error injection is enabled on Error Injection Channel 0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EICH0EN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Error Injection Channel 1 Enable"]
    #[inline(always)]
    pub fn eich1en(&self) -> EICH1EN_R {
        EICH1EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Error Injection Channel 0 Enable"]
    #[inline(always)]
    pub fn eich0en(&self) -> EICH0EN_R {
        EICH0EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Error Injection Channel 1 Enable"]
    #[inline(always)]
    pub fn eich1en(&mut self) -> EICH1EN_W {
        EICH1EN_W { w: self }
    }
    #[doc = "Bit 31 - Error Injection Channel 0 Enable"]
    #[inline(always)]
    pub fn eich0en(&mut self) -> EICH0EN_W {
        EICH0EN_W { w: self }
    }
}
