#[doc = "Reader of register FMS"]
pub type R = crate::R<u32, super::FMS>;
#[doc = "Writer for register FMS"]
pub type W = crate::W<u32, super::FMS>;
#[doc = "Register FMS `reset()`'s with value 0"]
impl crate::ResetValue for super::FMS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Fault Detection Flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTF0_A {
    #[doc = "0: No fault condition was detected at the fault input."]
    _0 = 0,
    #[doc = "1: A fault condition was detected at the fault input."]
    _1 = 1,
}
impl From<FAULTF0_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTF0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAULTF0`"]
pub type FAULTF0_R = crate::R<bool, FAULTF0_A>;
impl FAULTF0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTF0_A {
        match self.bits {
            false => FAULTF0_A::_0,
            true => FAULTF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTF0_A::_1
    }
}
#[doc = "Fault Detection Flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTF1_A {
    #[doc = "0: No fault condition was detected at the fault input."]
    _0 = 0,
    #[doc = "1: A fault condition was detected at the fault input."]
    _1 = 1,
}
impl From<FAULTF1_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTF1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAULTF1`"]
pub type FAULTF1_R = crate::R<bool, FAULTF1_A>;
impl FAULTF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTF1_A {
        match self.bits {
            false => FAULTF1_A::_0,
            true => FAULTF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTF1_A::_1
    }
}
#[doc = "Fault Detection Flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTF2_A {
    #[doc = "0: No fault condition was detected at the fault input."]
    _0 = 0,
    #[doc = "1: A fault condition was detected at the fault input."]
    _1 = 1,
}
impl From<FAULTF2_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTF2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAULTF2`"]
pub type FAULTF2_R = crate::R<bool, FAULTF2_A>;
impl FAULTF2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTF2_A {
        match self.bits {
            false => FAULTF2_A::_0,
            true => FAULTF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTF2_A::_1
    }
}
#[doc = "Fault Detection Flag 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTF3_A {
    #[doc = "0: No fault condition was detected at the fault input."]
    _0 = 0,
    #[doc = "1: A fault condition was detected at the fault input."]
    _1 = 1,
}
impl From<FAULTF3_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTF3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAULTF3`"]
pub type FAULTF3_R = crate::R<bool, FAULTF3_A>;
impl FAULTF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTF3_A {
        match self.bits {
            false => FAULTF3_A::_0,
            true => FAULTF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTF3_A::_1
    }
}
#[doc = "Fault Inputs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTIN_A {
    #[doc = "0: The logic OR of the enabled fault inputs is 0."]
    _0 = 0,
    #[doc = "1: The logic OR of the enabled fault inputs is 1."]
    _1 = 1,
}
impl From<FAULTIN_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTIN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAULTIN`"]
pub type FAULTIN_R = crate::R<bool, FAULTIN_A>;
impl FAULTIN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTIN_A {
        match self.bits {
            false => FAULTIN_A::_0,
            true => FAULTIN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTIN_A::_1
    }
}
#[doc = "Write Protection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPEN_A {
    #[doc = "0: Write protection is disabled. Write protected bits can be written."]
    _0 = 0,
    #[doc = "1: Write protection is enabled. Write protected bits cannot be written."]
    _1 = 1,
}
impl From<WPEN_A> for bool {
    #[inline(always)]
    fn from(variant: WPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WPEN`"]
pub type WPEN_R = crate::R<bool, WPEN_A>;
impl WPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPEN_A {
        match self.bits {
            false => WPEN_A::_0,
            true => WPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPEN_A::_1
    }
}
#[doc = "Write proxy for field `WPEN`"]
pub struct WPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write protection is disabled. Write protected bits can be written."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPEN_A::_0)
    }
    #[doc = "Write protection is enabled. Write protected bits cannot be written."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Fault Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FAULTF_A {
    #[doc = "0: No fault condition was detected."]
    _0 = 0,
    #[doc = "1: A fault condition was detected."]
    _1 = 1,
}
impl From<FAULTF_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FAULTF`"]
pub type FAULTF_R = crate::R<bool, FAULTF_A>;
impl FAULTF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTF_A {
        match self.bits {
            false => FAULTF_A::_0,
            true => FAULTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Fault Detection Flag 0"]
    #[inline(always)]
    pub fn faultf0(&self) -> FAULTF0_R {
        FAULTF0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fault Detection Flag 1"]
    #[inline(always)]
    pub fn faultf1(&self) -> FAULTF1_R {
        FAULTF1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fault Detection Flag 2"]
    #[inline(always)]
    pub fn faultf2(&self) -> FAULTF2_R {
        FAULTF2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fault Detection Flag 3"]
    #[inline(always)]
    pub fn faultf3(&self) -> FAULTF3_R {
        FAULTF3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fault Inputs"]
    #[inline(always)]
    pub fn faultin(&self) -> FAULTIN_R {
        FAULTIN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write Protection Enable"]
    #[inline(always)]
    pub fn wpen(&self) -> WPEN_R {
        WPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fault Detection Flag"]
    #[inline(always)]
    pub fn faultf(&self) -> FAULTF_R {
        FAULTF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Write Protection Enable"]
    #[inline(always)]
    pub fn wpen(&mut self) -> WPEN_W {
        WPEN_W { w: self }
    }
}
