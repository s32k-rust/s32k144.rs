#[doc = "Reader of register EIMCR"]
pub type R = crate::R<u32, super::EIMCR>;
#[doc = "Writer for register EIMCR"]
pub type W = crate::W<u32, super::EIMCR>;
#[doc = "Register EIMCR `reset()`'s with value 0"]
impl crate::ResetValue for super::EIMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Global Error Injection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GEIEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<GEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: GEIEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GEIEN`"]
pub type GEIEN_R = crate::R<bool, GEIEN_A>;
impl GEIEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GEIEN_A {
        match self.bits {
            false => GEIEN_A::_0,
            true => GEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GEIEN_A::_1
    }
}
#[doc = "Write proxy for field `GEIEN`"]
pub struct GEIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GEIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GEIEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GEIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GEIEN_A::_1)
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
impl R {
    #[doc = "Bit 0 - Global Error Injection Enable"]
    #[inline(always)]
    pub fn geien(&self) -> GEIEN_R {
        GEIEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Global Error Injection Enable"]
    #[inline(always)]
    pub fn geien(&mut self) -> GEIEN_W {
        GEIEN_W { w: self }
    }
}
