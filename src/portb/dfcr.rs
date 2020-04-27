#[doc = "Reader of register DFCR"]
pub type R = crate::R<u32, super::DFCR>;
#[doc = "Writer for register DFCR"]
pub type W = crate::W<u32, super::DFCR>;
#[doc = "Register DFCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DFCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CS_A {
    #[doc = "0: Digital filters are clocked by the bus clock."]
    _0 = 0,
    #[doc = "1: Digital filters are clocked by the LPO clock."]
    _1 = 1,
}
impl From<CS_A> for bool {
    #[inline(always)]
    fn from(variant: CS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CS`"]
pub type CS_R = crate::R<bool, CS_A>;
impl CS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS_A {
        match self.bits {
            false => CS_A::_0,
            true => CS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CS_A::_1
    }
}
#[doc = "Write proxy for field `CS`"]
pub struct CS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Digital filters are clocked by the bus clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CS_A::_0)
    }
    #[doc = "Digital filters are clocked by the LPO clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CS_A::_1)
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
    #[doc = "Bit 0 - Clock Source"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Source"]
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W {
        CS_W { w: self }
    }
}
