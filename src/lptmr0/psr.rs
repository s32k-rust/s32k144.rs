#[doc = "Reader of register PSR"]
pub type R = crate::R<u32, super::PSR>;
#[doc = "Writer for register PSR"]
pub type W = crate::W<u32, super::PSR>;
#[doc = "Register PSR `reset()`'s with value 0"]
impl crate::ResetValue for super::PSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Prescaler Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCS_A {
    #[doc = "0: Prescaler/glitch filter clock 0 selected."]
    _00 = 0,
    #[doc = "1: Prescaler/glitch filter clock 1 selected."]
    _01 = 1,
    #[doc = "2: Prescaler/glitch filter clock 2 selected."]
    _10 = 2,
    #[doc = "3: Prescaler/glitch filter clock 3 selected."]
    _11 = 3,
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
            0 => PCS_A::_00,
            1 => PCS_A::_01,
            2 => PCS_A::_10,
            3 => PCS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PCS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PCS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PCS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PCS_A::_11
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
    #[doc = "Prescaler/glitch filter clock 0 selected."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PCS_A::_00)
    }
    #[doc = "Prescaler/glitch filter clock 1 selected."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PCS_A::_01)
    }
    #[doc = "Prescaler/glitch filter clock 2 selected."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PCS_A::_10)
    }
    #[doc = "Prescaler/glitch filter clock 3 selected."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PCS_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Prescaler Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBYP_A {
    #[doc = "0: Prescaler/glitch filter is enabled."]
    _0 = 0,
    #[doc = "1: Prescaler/glitch filter is bypassed."]
    _1 = 1,
}
impl From<PBYP_A> for bool {
    #[inline(always)]
    fn from(variant: PBYP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PBYP`"]
pub type PBYP_R = crate::R<bool, PBYP_A>;
impl PBYP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PBYP_A {
        match self.bits {
            false => PBYP_A::_0,
            true => PBYP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PBYP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PBYP_A::_1
    }
}
#[doc = "Write proxy for field `PBYP`"]
pub struct PBYP_W<'a> {
    w: &'a mut W,
}
impl<'a> PBYP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PBYP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Prescaler/glitch filter is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PBYP_A::_0)
    }
    #[doc = "Prescaler/glitch filter is bypassed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PBYP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Prescale Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALE_A {
    #[doc = "0: Prescaler divides the prescaler clock by 2; glitch filter does not support this configuration."]
    _0000 = 0,
    #[doc = "1: Prescaler divides the prescaler clock by 4; glitch filter recognizes change on input pin after 2 rising clock edges."]
    _0001 = 1,
    #[doc = "2: Prescaler divides the prescaler clock by 8; glitch filter recognizes change on input pin after 4 rising clock edges."]
    _0010 = 2,
    #[doc = "3: Prescaler divides the prescaler clock by 16; glitch filter recognizes change on input pin after 8 rising clock edges."]
    _0011 = 3,
    #[doc = "4: Prescaler divides the prescaler clock by 32; glitch filter recognizes change on input pin after 16 rising clock edges."]
    _0100 = 4,
    #[doc = "5: Prescaler divides the prescaler clock by 64; glitch filter recognizes change on input pin after 32 rising clock edges."]
    _0101 = 5,
    #[doc = "6: Prescaler divides the prescaler clock by 128; glitch filter recognizes change on input pin after 64 rising clock edges."]
    _0110 = 6,
    #[doc = "7: Prescaler divides the prescaler clock by 256; glitch filter recognizes change on input pin after 128 rising clock edges."]
    _0111 = 7,
    #[doc = "8: Prescaler divides the prescaler clock by 512; glitch filter recognizes change on input pin after 256 rising clock edges."]
    _1000 = 8,
    #[doc = "9: Prescaler divides the prescaler clock by 1024; glitch filter recognizes change on input pin after 512 rising clock edges."]
    _1001 = 9,
    #[doc = "10: Prescaler divides the prescaler clock by 2048; glitch filter recognizes change on input pin after 1024 rising clock edges."]
    _1010 = 10,
    #[doc = "11: Prescaler divides the prescaler clock by 4096; glitch filter recognizes change on input pin after 2048 rising clock edges."]
    _1011 = 11,
    #[doc = "12: Prescaler divides the prescaler clock by 8192; glitch filter recognizes change on input pin after 4096 rising clock edges."]
    _1100 = 12,
    #[doc = "13: Prescaler divides the prescaler clock by 16,384; glitch filter recognizes change on input pin after 8192 rising clock edges."]
    _1101 = 13,
    #[doc = "14: Prescaler divides the prescaler clock by 32,768; glitch filter recognizes change on input pin after 16,384 rising clock edges."]
    _1110 = 14,
    #[doc = "15: Prescaler divides the prescaler clock by 65,536; glitch filter recognizes change on input pin after 32,768 rising clock edges."]
    _1111 = 15,
}
impl From<PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESCALE`"]
pub type PRESCALE_R = crate::R<u8, PRESCALE_A>;
impl PRESCALE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCALE_A {
        match self.bits {
            0 => PRESCALE_A::_0000,
            1 => PRESCALE_A::_0001,
            2 => PRESCALE_A::_0010,
            3 => PRESCALE_A::_0011,
            4 => PRESCALE_A::_0100,
            5 => PRESCALE_A::_0101,
            6 => PRESCALE_A::_0110,
            7 => PRESCALE_A::_0111,
            8 => PRESCALE_A::_1000,
            9 => PRESCALE_A::_1001,
            10 => PRESCALE_A::_1010,
            11 => PRESCALE_A::_1011,
            12 => PRESCALE_A::_1100,
            13 => PRESCALE_A::_1101,
            14 => PRESCALE_A::_1110,
            15 => PRESCALE_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == PRESCALE_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == PRESCALE_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == PRESCALE_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == PRESCALE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == PRESCALE_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == PRESCALE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == PRESCALE_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == PRESCALE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == PRESCALE_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == PRESCALE_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == PRESCALE_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == PRESCALE_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == PRESCALE_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == PRESCALE_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == PRESCALE_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == PRESCALE_A::_1111
    }
}
#[doc = "Write proxy for field `PRESCALE`"]
pub struct PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Prescaler divides the prescaler clock by 2; glitch filter does not support this configuration."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(PRESCALE_A::_0000)
    }
    #[doc = "Prescaler divides the prescaler clock by 4; glitch filter recognizes change on input pin after 2 rising clock edges."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(PRESCALE_A::_0001)
    }
    #[doc = "Prescaler divides the prescaler clock by 8; glitch filter recognizes change on input pin after 4 rising clock edges."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(PRESCALE_A::_0010)
    }
    #[doc = "Prescaler divides the prescaler clock by 16; glitch filter recognizes change on input pin after 8 rising clock edges."]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(PRESCALE_A::_0011)
    }
    #[doc = "Prescaler divides the prescaler clock by 32; glitch filter recognizes change on input pin after 16 rising clock edges."]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(PRESCALE_A::_0100)
    }
    #[doc = "Prescaler divides the prescaler clock by 64; glitch filter recognizes change on input pin after 32 rising clock edges."]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(PRESCALE_A::_0101)
    }
    #[doc = "Prescaler divides the prescaler clock by 128; glitch filter recognizes change on input pin after 64 rising clock edges."]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(PRESCALE_A::_0110)
    }
    #[doc = "Prescaler divides the prescaler clock by 256; glitch filter recognizes change on input pin after 128 rising clock edges."]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(PRESCALE_A::_0111)
    }
    #[doc = "Prescaler divides the prescaler clock by 512; glitch filter recognizes change on input pin after 256 rising clock edges."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(PRESCALE_A::_1000)
    }
    #[doc = "Prescaler divides the prescaler clock by 1024; glitch filter recognizes change on input pin after 512 rising clock edges."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(PRESCALE_A::_1001)
    }
    #[doc = "Prescaler divides the prescaler clock by 2048; glitch filter recognizes change on input pin after 1024 rising clock edges."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(PRESCALE_A::_1010)
    }
    #[doc = "Prescaler divides the prescaler clock by 4096; glitch filter recognizes change on input pin after 2048 rising clock edges."]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(PRESCALE_A::_1011)
    }
    #[doc = "Prescaler divides the prescaler clock by 8192; glitch filter recognizes change on input pin after 4096 rising clock edges."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(PRESCALE_A::_1100)
    }
    #[doc = "Prescaler divides the prescaler clock by 16,384; glitch filter recognizes change on input pin after 8192 rising clock edges."]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(PRESCALE_A::_1101)
    }
    #[doc = "Prescaler divides the prescaler clock by 32,768; glitch filter recognizes change on input pin after 16,384 rising clock edges."]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(PRESCALE_A::_1110)
    }
    #[doc = "Prescaler divides the prescaler clock by 65,536; glitch filter recognizes change on input pin after 32,768 rising clock edges."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(PRESCALE_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Prescaler Clock Select"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Prescaler Bypass"]
    #[inline(always)]
    pub fn pbyp(&self) -> PBYP_R {
        PBYP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - Prescale Value"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Prescaler Clock Select"]
    #[inline(always)]
    pub fn pcs(&mut self) -> PCS_W {
        PCS_W { w: self }
    }
    #[doc = "Bit 2 - Prescaler Bypass"]
    #[inline(always)]
    pub fn pbyp(&mut self) -> PBYP_W {
        PBYP_W { w: self }
    }
    #[doc = "Bits 3:6 - Prescale Value"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W {
        PRESCALE_W { w: self }
    }
}
