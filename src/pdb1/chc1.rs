#[doc = "Reader of register CH%sC1"]
pub type R = crate::R<u32, super::CHC1>;
#[doc = "Writer for register CH%sC1"]
pub type W = crate::W<u32, super::CHC1>;
#[doc = "Register CH%sC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CHC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EN_A {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    _1 = 1,
}
impl From<EN_A> for u8 {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<u8, EN_A>;
impl EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EN_A::_0),
            1 => Val(EN_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN_A::_1
    }
}
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TOS_A {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1 = 1,
}
impl From<TOS_A> for u8 {
    #[inline(always)]
    fn from(variant: TOS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TOS`"]
pub type TOS_R = crate::R<u8, TOS_A>;
impl TOS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TOS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TOS_A::_0),
            1 => Val(TOS_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOS_A::_1
    }
}
#[doc = "Write proxy for field `TOS`"]
pub struct TOS_W<'a> {
    w: &'a mut W,
}
impl<'a> TOS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TOS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BB_A {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1 = 1,
}
impl From<BB_A> for u8 {
    #[inline(always)]
    fn from(variant: BB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BB`"]
pub type BB_R = crate::R<u8, BB_A>;
impl BB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BB_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BB_A::_0),
            1 => Val(BB_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BB_A::_1
    }
}
#[doc = "Write proxy for field `BB`"]
pub struct BB_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb(&self) -> BB_R {
        BB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bits 8:15 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos(&mut self) -> TOS_W {
        TOS_W { w: self }
    }
    #[doc = "Bits 16:23 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb(&mut self) -> BB_W {
        BB_W { w: self }
    }
}
