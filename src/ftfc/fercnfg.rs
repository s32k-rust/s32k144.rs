#[doc = "Reader of register FERCNFG"]
pub type R = crate::R<u8, super::FERCNFG>;
#[doc = "Writer for register FERCNFG"]
pub type W = crate::W<u8, super::FERCNFG>;
#[doc = "Register FERCNFG `reset()`'s with value 0"]
impl crate::ResetValue for super::FERCNFG {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Double Bit Fault Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFDIE_A {
    #[doc = "0: Double bit fault detect interrupt disabled"]
    _0 = 0,
    #[doc = "1: Double bit fault detect interrupt enabled. An interrupt request is generated whenever the FERSTAT\\[DFDIF\\]
flag is set."]
    _1 = 1,
}
impl From<DFDIE_A> for bool {
    #[inline(always)]
    fn from(variant: DFDIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DFDIE`"]
pub type DFDIE_R = crate::R<bool, DFDIE_A>;
impl DFDIE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFDIE_A {
        match self.bits {
            false => DFDIE_A::_0,
            true => DFDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFDIE_A::_1
    }
}
#[doc = "Write proxy for field `DFDIE`"]
pub struct DFDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DFDIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFDIE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Double bit fault detect interrupt disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFDIE_A::_0)
    }
    #[doc = "Double bit fault detect interrupt enabled. An interrupt request is generated whenever the FERSTAT\\[DFDIF\\]
flag is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFDIE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Force Double Bit Fault Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDFD_A {
    #[doc = "0: FERSTAT\\[DFDIF\\]
sets only if a double bit fault is detected during read access from the platform flash controller"]
    _0 = 0,
    #[doc = "1: FERSTAT\\[DFDIF\\]
sets during any valid flash read access from the platform flash controller. An interrupt request is generated if the DFDIE bit is set."]
    _1 = 1,
}
impl From<FDFD_A> for bool {
    #[inline(always)]
    fn from(variant: FDFD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FDFD`"]
pub type FDFD_R = crate::R<bool, FDFD_A>;
impl FDFD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDFD_A {
        match self.bits {
            false => FDFD_A::_0,
            true => FDFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FDFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FDFD_A::_1
    }
}
#[doc = "Write proxy for field `FDFD`"]
pub struct FDFD_W<'a> {
    w: &'a mut W,
}
impl<'a> FDFD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDFD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "FERSTAT\\[DFDIF\\]
sets only if a double bit fault is detected during read access from the platform flash controller"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDFD_A::_0)
    }
    #[doc = "FERSTAT\\[DFDIF\\]
sets during any valid flash read access from the platform flash controller. An interrupt request is generated if the DFDIE bit is set."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDFD_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Double Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub fn dfdie(&self) -> DFDIE_R {
        DFDIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Force Double Bit Fault Detect"]
    #[inline(always)]
    pub fn fdfd(&self) -> FDFD_R {
        FDFD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Double Bit Fault Detect Interrupt Enable"]
    #[inline(always)]
    pub fn dfdie(&mut self) -> DFDIE_W {
        DFDIE_W { w: self }
    }
    #[doc = "Bit 5 - Force Double Bit Fault Detect"]
    #[inline(always)]
    pub fn fdfd(&mut self) -> FDFD_W {
        FDFD_W { w: self }
    }
}
