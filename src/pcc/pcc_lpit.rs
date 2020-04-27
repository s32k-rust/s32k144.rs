#[doc = "Reader of register PCC_LPIT"]
pub type R = crate::R<u32, super::PCC_LPIT>;
#[doc = "Writer for register PCC_LPIT"]
pub type W = crate::W<u32, super::PCC_LPIT>;
#[doc = "Register PCC_LPIT `reset()`'s with value 0x8000_0000"]
impl crate::ResetValue for super::PCC_LPIT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8000_0000
    }
}
#[doc = "Peripheral Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCS_A {
    #[doc = "0: Clock is off."]
    _000 = 0,
    #[doc = "1: Clock option 1"]
    _001 = 1,
    #[doc = "2: Clock option 2"]
    _010 = 2,
    #[doc = "3: Clock option 3"]
    _011 = 3,
    #[doc = "4: Clock option 4"]
    _100 = 4,
    #[doc = "5: Clock option 5"]
    _101 = 5,
    #[doc = "6: Clock option 6"]
    _110 = 6,
    #[doc = "7: Clock option 7"]
    _111 = 7,
}
impl From<PCS_A> for u8 {
    #[inline(always)]
    fn from(variant: PCS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCS`"]
pub type PCS_R = crate::R<u8, PCS_A>;
impl PCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCS_A {
        match self.bits {
            0 => PCS_A::_000,
            1 => PCS_A::_001,
            2 => PCS_A::_010,
            3 => PCS_A::_011,
            4 => PCS_A::_100,
            5 => PCS_A::_101,
            6 => PCS_A::_110,
            7 => PCS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PCS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PCS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PCS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PCS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PCS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PCS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PCS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PCS_A::_111
    }
}
#[doc = "Write proxy for field `PCS`"]
pub struct PCS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock is off."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PCS_A::_000)
    }
    #[doc = "Clock option 1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PCS_A::_001)
    }
    #[doc = "Clock option 2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PCS_A::_010)
    }
    #[doc = "Clock option 3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PCS_A::_011)
    }
    #[doc = "Clock option 4"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PCS_A::_100)
    }
    #[doc = "Clock option 5"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PCS_A::_101)
    }
    #[doc = "Clock option 6"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PCS_A::_110)
    }
    #[doc = "Clock option 7"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PCS_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGC_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled. The current clock selection and divider options are locked."]
    _1 = 1,
}
impl From<CGC_A> for bool {
    #[inline(always)]
    fn from(variant: CGC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CGC`"]
pub type CGC_R = crate::R<bool, CGC_A>;
impl CGC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CGC_A {
        match self.bits {
            false => CGC_A::_0,
            true => CGC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CGC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CGC_A::_1
    }
}
#[doc = "Write proxy for field `CGC`"]
pub struct CGC_W<'a> {
    w: &'a mut W,
}
impl<'a> CGC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CGC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CGC_A::_0)
    }
    #[doc = "Clock enabled. The current clock selection and divider options are locked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CGC_A::_1)
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
#[doc = "Present\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PR_A {
    #[doc = "0: Peripheral is not present."]
    _0 = 0,
    #[doc = "1: Peripheral is present."]
    _1 = 1,
}
impl From<PR_A> for bool {
    #[inline(always)]
    fn from(variant: PR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PR`"]
pub type PR_R = crate::R<bool, PR_A>;
impl PR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PR_A {
        match self.bits {
            false => PR_A::_0,
            true => PR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PR_A::_1
    }
}
impl R {
    #[doc = "Bits 24:26 - Peripheral Clock Source Select"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 30 - Clock Gate Control"]
    #[inline(always)]
    pub fn cgc(&self) -> CGC_R {
        CGC_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Present"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:26 - Peripheral Clock Source Select"]
    #[inline(always)]
    pub fn pcs(&mut self) -> PCS_W {
        PCS_W { w: self }
    }
    #[doc = "Bit 30 - Clock Gate Control"]
    #[inline(always)]
    pub fn cgc(&mut self) -> CGC_W {
        CGC_W { w: self }
    }
}
