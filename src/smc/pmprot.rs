#[doc = "Reader of register PMPROT"]
pub type R = crate::R<u32, super::PMPROT>;
#[doc = "Writer for register PMPROT"]
pub type W = crate::W<u32, super::PMPROT>;
#[doc = "Register PMPROT `reset()`'s with value 0"]
impl crate::ResetValue for super::PMPROT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Allow Very-Low-Power Modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVLP_A {
    #[doc = "0: VLPR and VLPS are not allowed."]
    _0 = 0,
    #[doc = "1: VLPR and VLPS are allowed."]
    _1 = 1,
}
impl From<AVLP_A> for bool {
    #[inline(always)]
    fn from(variant: AVLP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AVLP`"]
pub type AVLP_R = crate::R<bool, AVLP_A>;
impl AVLP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVLP_A {
        match self.bits {
            false => AVLP_A::_0,
            true => AVLP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AVLP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AVLP_A::_1
    }
}
#[doc = "Write proxy for field `AVLP`"]
pub struct AVLP_W<'a> {
    w: &'a mut W,
}
impl<'a> AVLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AVLP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "VLPR and VLPS are not allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVLP_A::_0)
    }
    #[doc = "VLPR and VLPS are allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVLP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Allow High Speed Run mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHSRUN_A {
    #[doc = "0: HSRUN is not allowed"]
    _0 = 0,
    #[doc = "1: HSRUN is allowed"]
    _1 = 1,
}
impl From<AHSRUN_A> for bool {
    #[inline(always)]
    fn from(variant: AHSRUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `AHSRUN`"]
pub type AHSRUN_R = crate::R<bool, AHSRUN_A>;
impl AHSRUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHSRUN_A {
        match self.bits {
            false => AHSRUN_A::_0,
            true => AHSRUN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AHSRUN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AHSRUN_A::_1
    }
}
#[doc = "Write proxy for field `AHSRUN`"]
pub struct AHSRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHSRUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHSRUN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HSRUN is not allowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AHSRUN_A::_0)
    }
    #[doc = "HSRUN is allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AHSRUN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - Allow Very-Low-Power Modes"]
    #[inline(always)]
    pub fn avlp(&self) -> AVLP_R {
        AVLP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Allow High Speed Run mode"]
    #[inline(always)]
    pub fn ahsrun(&self) -> AHSRUN_R {
        AHSRUN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Allow Very-Low-Power Modes"]
    #[inline(always)]
    pub fn avlp(&mut self) -> AVLP_W {
        AVLP_W { w: self }
    }
    #[doc = "Bit 7 - Allow High Speed Run mode"]
    #[inline(always)]
    pub fn ahsrun(&mut self) -> AHSRUN_W {
        AHSRUN_W { w: self }
    }
}
