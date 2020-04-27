#[doc = "Reader of register SIRCDIV"]
pub type R = crate::R<u32, super::SIRCDIV>;
#[doc = "Writer for register SIRCDIV"]
pub type W = crate::W<u32, super::SIRCDIV>;
#[doc = "Register SIRCDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::SIRCDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Slow IRC Clock Divide 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIRCDIV1_A {
    #[doc = "0: Output disabled"]
    _000 = 0,
    #[doc = "1: Divide by 1"]
    _001 = 1,
    #[doc = "2: Divide by 2"]
    _010 = 2,
    #[doc = "3: Divide by 4"]
    _011 = 3,
    #[doc = "4: Divide by 8"]
    _100 = 4,
    #[doc = "5: Divide by 16"]
    _101 = 5,
    #[doc = "6: Divide by 32"]
    _110 = 6,
    #[doc = "7: Divide by 64"]
    _111 = 7,
}
impl From<SIRCDIV1_A> for u8 {
    #[inline(always)]
    fn from(variant: SIRCDIV1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SIRCDIV1`"]
pub type SIRCDIV1_R = crate::R<u8, SIRCDIV1_A>;
impl SIRCDIV1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIRCDIV1_A {
        match self.bits {
            0 => SIRCDIV1_A::_000,
            1 => SIRCDIV1_A::_001,
            2 => SIRCDIV1_A::_010,
            3 => SIRCDIV1_A::_011,
            4 => SIRCDIV1_A::_100,
            5 => SIRCDIV1_A::_101,
            6 => SIRCDIV1_A::_110,
            7 => SIRCDIV1_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SIRCDIV1_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SIRCDIV1_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SIRCDIV1_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SIRCDIV1_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SIRCDIV1_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SIRCDIV1_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SIRCDIV1_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SIRCDIV1_A::_111
    }
}
#[doc = "Write proxy for field `SIRCDIV1`"]
pub struct SIRCDIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SIRCDIV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIRCDIV1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SIRCDIV1_A::_000)
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SIRCDIV1_A::_001)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SIRCDIV1_A::_010)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SIRCDIV1_A::_011)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SIRCDIV1_A::_100)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SIRCDIV1_A::_101)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SIRCDIV1_A::_110)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(SIRCDIV1_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Slow IRC Clock Divide 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIRCDIV2_A {
    #[doc = "0: Output disabled"]
    _000 = 0,
    #[doc = "1: Divide by 1"]
    _001 = 1,
    #[doc = "2: Divide by 2"]
    _010 = 2,
    #[doc = "3: Divide by 4"]
    _011 = 3,
    #[doc = "4: Divide by 8"]
    _100 = 4,
    #[doc = "5: Divide by 16"]
    _101 = 5,
    #[doc = "6: Divide by 32"]
    _110 = 6,
    #[doc = "7: Divide by 64"]
    _111 = 7,
}
impl From<SIRCDIV2_A> for u8 {
    #[inline(always)]
    fn from(variant: SIRCDIV2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SIRCDIV2`"]
pub type SIRCDIV2_R = crate::R<u8, SIRCDIV2_A>;
impl SIRCDIV2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIRCDIV2_A {
        match self.bits {
            0 => SIRCDIV2_A::_000,
            1 => SIRCDIV2_A::_001,
            2 => SIRCDIV2_A::_010,
            3 => SIRCDIV2_A::_011,
            4 => SIRCDIV2_A::_100,
            5 => SIRCDIV2_A::_101,
            6 => SIRCDIV2_A::_110,
            7 => SIRCDIV2_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SIRCDIV2_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SIRCDIV2_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SIRCDIV2_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SIRCDIV2_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SIRCDIV2_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SIRCDIV2_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SIRCDIV2_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SIRCDIV2_A::_111
    }
}
#[doc = "Write proxy for field `SIRCDIV2`"]
pub struct SIRCDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> SIRCDIV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIRCDIV2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SIRCDIV2_A::_000)
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SIRCDIV2_A::_001)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SIRCDIV2_A::_010)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SIRCDIV2_A::_011)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SIRCDIV2_A::_100)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SIRCDIV2_A::_101)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SIRCDIV2_A::_110)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(SIRCDIV2_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Slow IRC Clock Divide 1"]
    #[inline(always)]
    pub fn sircdiv1(&self) -> SIRCDIV1_R {
        SIRCDIV1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Slow IRC Clock Divide 2"]
    #[inline(always)]
    pub fn sircdiv2(&self) -> SIRCDIV2_R {
        SIRCDIV2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slow IRC Clock Divide 1"]
    #[inline(always)]
    pub fn sircdiv1(&mut self) -> SIRCDIV1_W {
        SIRCDIV1_W { w: self }
    }
    #[doc = "Bits 8:10 - Slow IRC Clock Divide 2"]
    #[inline(always)]
    pub fn sircdiv2(&mut self) -> SIRCDIV2_W {
        SIRCDIV2_W { w: self }
    }
}
