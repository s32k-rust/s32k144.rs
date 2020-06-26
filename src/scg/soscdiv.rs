#[doc = "Reader of register SOSCDIV"]
pub type R = crate::R<u32, super::SOSCDIV>;
#[doc = "Writer for register SOSCDIV"]
pub type W = crate::W<u32, super::SOSCDIV>;
#[doc = "Register SOSCDIV `reset()`'s with value 0"]
impl crate::ResetValue for super::SOSCDIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "System OSC Clock Divide 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SOSCDIV1_A {
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
impl From<SOSCDIV1_A> for u8 {
    #[inline(always)]
    fn from(variant: SOSCDIV1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SOSCDIV1`"]
pub type SOSCDIV1_R = crate::R<u8, SOSCDIV1_A>;
impl SOSCDIV1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCDIV1_A {
        match self.bits {
            0 => SOSCDIV1_A::_000,
            1 => SOSCDIV1_A::_001,
            2 => SOSCDIV1_A::_010,
            3 => SOSCDIV1_A::_011,
            4 => SOSCDIV1_A::_100,
            5 => SOSCDIV1_A::_101,
            6 => SOSCDIV1_A::_110,
            7 => SOSCDIV1_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SOSCDIV1_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SOSCDIV1_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SOSCDIV1_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SOSCDIV1_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SOSCDIV1_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SOSCDIV1_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SOSCDIV1_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SOSCDIV1_A::_111
    }
}
#[doc = "Write proxy for field `SOSCDIV1`"]
pub struct SOSCDIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCDIV1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOSCDIV1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SOSCDIV1_A::_000)
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SOSCDIV1_A::_001)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SOSCDIV1_A::_010)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SOSCDIV1_A::_011)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SOSCDIV1_A::_100)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SOSCDIV1_A::_101)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SOSCDIV1_A::_110)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(SOSCDIV1_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "System OSC Clock Divide 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SOSCDIV2_A {
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
impl From<SOSCDIV2_A> for u8 {
    #[inline(always)]
    fn from(variant: SOSCDIV2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SOSCDIV2`"]
pub type SOSCDIV2_R = crate::R<u8, SOSCDIV2_A>;
impl SOSCDIV2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOSCDIV2_A {
        match self.bits {
            0 => SOSCDIV2_A::_000,
            1 => SOSCDIV2_A::_001,
            2 => SOSCDIV2_A::_010,
            3 => SOSCDIV2_A::_011,
            4 => SOSCDIV2_A::_100,
            5 => SOSCDIV2_A::_101,
            6 => SOSCDIV2_A::_110,
            7 => SOSCDIV2_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SOSCDIV2_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SOSCDIV2_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SOSCDIV2_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SOSCDIV2_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SOSCDIV2_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SOSCDIV2_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SOSCDIV2_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SOSCDIV2_A::_111
    }
}
#[doc = "Write proxy for field `SOSCDIV2`"]
pub struct SOSCDIV2_W<'a> {
    w: &'a mut W,
}
impl<'a> SOSCDIV2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOSCDIV2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SOSCDIV2_A::_000)
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SOSCDIV2_A::_001)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SOSCDIV2_A::_010)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(SOSCDIV2_A::_011)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SOSCDIV2_A::_100)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SOSCDIV2_A::_101)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(SOSCDIV2_A::_110)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(SOSCDIV2_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - System OSC Clock Divide 1"]
    #[inline(always)]
    pub fn soscdiv1(&self) -> SOSCDIV1_R {
        SOSCDIV1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - System OSC Clock Divide 2"]
    #[inline(always)]
    pub fn soscdiv2(&self) -> SOSCDIV2_R {
        SOSCDIV2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - System OSC Clock Divide 1"]
    #[inline(always)]
    pub fn soscdiv1(&mut self) -> SOSCDIV1_W {
        SOSCDIV1_W { w: self }
    }
    #[doc = "Bits 8:10 - System OSC Clock Divide 2"]
    #[inline(always)]
    pub fn soscdiv2(&mut self) -> SOSCDIV2_W {
        SOSCDIV2_W { w: self }
    }
}
