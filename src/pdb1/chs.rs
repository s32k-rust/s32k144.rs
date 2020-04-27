#[doc = "Reader of register CH%sS"]
pub type R = crate::R<u32, super::CHS>;
#[doc = "Writer for register CH%sS"]
pub type W = crate::W<u32, super::CHS>;
#[doc = "Register CH%sS `reset()`'s with value 0"]
impl crate::ResetValue for super::CHS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "PDB Channel Sequence Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ERR_A {
    #[doc = "0: Sequence error not detected on PDB channel's corresponding pre-trigger."]
    _0 = 0,
    #[doc = "1: Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    _1 = 1,
}
impl From<ERR_A> for u8 {
    #[inline(always)]
    fn from(variant: ERR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ERR`"]
pub type ERR_R = crate::R<u8, ERR_A>;
impl ERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ERR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ERR_A::_0),
            1 => Val(ERR_A::_1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR_A::_1
    }
}
#[doc = "Write proxy for field `ERR`"]
pub struct ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Sequence error not detected on PDB channel's corresponding pre-trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR_A::_0)
    }
    #[doc = "Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR_A::_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `CF`"]
pub type CF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CF`"]
pub struct CF_W<'a> {
    w: &'a mut W,
}
impl<'a> CF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PDB Channel Flags"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
    #[doc = "Bits 16:23 - PDB Channel Flags"]
    #[inline(always)]
    pub fn cf(&mut self) -> CF_W {
        CF_W { w: self }
    }
}
