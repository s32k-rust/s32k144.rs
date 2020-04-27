#[doc = "Reader of register SPLLDIV"]
pub type R = crate::R<u32, super::SPLLDIV>;
#[doc = "Writer for register SPLLDIV"]
pub type W = crate::W<u32, super::SPLLDIV>;
#[doc = "Register SPLLDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::SPLLDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "System PLL Clock Divide 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPLLDIV1_A {
    #[doc = "0: Clock disabled"]
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
impl From<SPLLDIV1_A> for u8 {
    #[inline(always)]
    fn from(variant: SPLLDIV1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPLLDIV1`"]
pub type SPLLDIV1_R = crate::R<u8, SPLLDIV1_A>;
impl SPLLDIV1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLLDIV1_A {
        match self.bits {
            0 => SPLLDIV1_A::_000,
            1 => SPLLDIV1_A::_001,
            2 => SPLLDIV1_A::_010,
            3 => SPLLDIV1_A::_011,
            4 => SPLLDIV1_A::_100,
            5 => SPLLDIV1_A::_101,
            6 => SPLLDIV1_A::_110,
            7 => SPLLDIV1_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SPLLDIV1_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SPLLDIV1_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SPLLDIV1_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SPLLDIV1_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SPLLDIV1_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SPLLDIV1_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SPLLDIV1_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SPLLDIV1_A::_111
    }
}
#[doc = "Write proxy for field `SPLLDIV1`"]
pub struct SPLLDIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLLDIV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLLDIV1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SPLLDIV1_A::_000)
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SPLLDIV1_A::_001)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SPLLDIV1_A::_010)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SPLLDIV1_A::_011)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SPLLDIV1_A::_100)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SPLLDIV1_A::_101)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SPLLDIV1_A::_110)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(SPLLDIV1_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "System PLL Clock Divide 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPLLDIV2_A {
    #[doc = "0: Clock disabled"]
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
impl From<SPLLDIV2_A> for u8 {
    #[inline(always)]
    fn from(variant: SPLLDIV2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPLLDIV2`"]
pub type SPLLDIV2_R = crate::R<u8, SPLLDIV2_A>;
impl SPLLDIV2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPLLDIV2_A {
        match self.bits {
            0 => SPLLDIV2_A::_000,
            1 => SPLLDIV2_A::_001,
            2 => SPLLDIV2_A::_010,
            3 => SPLLDIV2_A::_011,
            4 => SPLLDIV2_A::_100,
            5 => SPLLDIV2_A::_101,
            6 => SPLLDIV2_A::_110,
            7 => SPLLDIV2_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SPLLDIV2_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SPLLDIV2_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SPLLDIV2_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SPLLDIV2_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SPLLDIV2_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SPLLDIV2_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SPLLDIV2_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SPLLDIV2_A::_111
    }
}
#[doc = "Write proxy for field `SPLLDIV2`"]
pub struct SPLLDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> SPLLDIV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPLLDIV2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SPLLDIV2_A::_000)
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SPLLDIV2_A::_001)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SPLLDIV2_A::_010)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SPLLDIV2_A::_011)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SPLLDIV2_A::_100)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SPLLDIV2_A::_101)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SPLLDIV2_A::_110)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(SPLLDIV2_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - System PLL Clock Divide 1"]
    #[inline(always)]
    pub fn splldiv1(&self) -> SPLLDIV1_R {
        SPLLDIV1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - System PLL Clock Divide 2"]
    #[inline(always)]
    pub fn splldiv2(&self) -> SPLLDIV2_R {
        SPLLDIV2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - System PLL Clock Divide 1"]
    #[inline(always)]
    pub fn splldiv1(&mut self) -> SPLLDIV1_W {
        SPLLDIV1_W { w: self }
    }
    #[doc = "Bits 8:10 - System PLL Clock Divide 2"]
    #[inline(always)]
    pub fn splldiv2(&mut self) -> SPLLDIV2_W {
        SPLLDIV2_W { w: self }
    }
}
