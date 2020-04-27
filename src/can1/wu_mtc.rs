#[doc = "Reader of register WU_MTC"]
pub type R = crate::R<u32, super::WU_MTC>;
#[doc = "Writer for register WU_MTC"]
pub type W = crate::W<u32, super::WU_MTC>;
#[doc = "Register WU_MTC `reset()`'s with value 0"]
impl crate::ResetValue for super::WU_MTC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCOUNTER`"]
pub type MCOUNTER_R = crate::R<u8, u8>;
#[doc = "Wake Up by Match Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUMF_A {
    #[doc = "0: No wake up by match event detected"]
    _0 = 0,
    #[doc = "1: Wake up by match event detected"]
    _1 = 1,
}
impl From<WUMF_A> for bool {
    #[inline(always)]
    fn from(variant: WUMF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WUMF`"]
pub type WUMF_R = crate::R<bool, WUMF_A>;
impl WUMF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUMF_A {
        match self.bits {
            false => WUMF_A::_0,
            true => WUMF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WUMF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WUMF_A::_1
    }
}
#[doc = "Write proxy for field `WUMF`"]
pub struct WUMF_W<'a> {
    w: &'a mut W,
}
impl<'a> WUMF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WUMF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No wake up by match event detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUMF_A::_0)
    }
    #[doc = "Wake up by match event detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUMF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Wake Up by Timeout Flag Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTOF_A {
    #[doc = "0: No wake up by timeout event detected"]
    _0 = 0,
    #[doc = "1: Wake up by timeout event detected"]
    _1 = 1,
}
impl From<WTOF_A> for bool {
    #[inline(always)]
    fn from(variant: WTOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WTOF`"]
pub type WTOF_R = crate::R<bool, WTOF_A>;
impl WTOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTOF_A {
        match self.bits {
            false => WTOF_A::_0,
            true => WTOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WTOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WTOF_A::_1
    }
}
#[doc = "Write proxy for field `WTOF`"]
pub struct WTOF_W<'a> {
    w: &'a mut W,
}
impl<'a> WTOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WTOF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No wake up by timeout event detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WTOF_A::_0)
    }
    #[doc = "Wake up by timeout event detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WTOF_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Number of Matches while in Pretended Networking"]
    #[inline(always)]
    pub fn mcounter(&self) -> MCOUNTER_R {
        MCOUNTER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Wake Up by Match Flag Bit"]
    #[inline(always)]
    pub fn wumf(&self) -> WUMF_R {
        WUMF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Wake Up by Timeout Flag Bit"]
    #[inline(always)]
    pub fn wtof(&self) -> WTOF_R {
        WTOF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Wake Up by Match Flag Bit"]
    #[inline(always)]
    pub fn wumf(&mut self) -> WUMF_W {
        WUMF_W { w: self }
    }
    #[doc = "Bit 17 - Wake Up by Timeout Flag Bit"]
    #[inline(always)]
    pub fn wtof(&mut self) -> WTOF_W {
        WTOF_W { w: self }
    }
}
