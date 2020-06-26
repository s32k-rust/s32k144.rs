#[doc = "Reader of register LPOCLKS"]
pub type R = crate::R<u32, super::LPOCLKS>;
#[doc = "Writer for register LPOCLKS"]
pub type W = crate::W<u32, super::LPOCLKS>;
#[doc = "Register LPOCLKS `reset()`'s with value 0x03"]
impl crate::ResetValue for super::LPOCLKS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "1 kHz LPO_CLK enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPO1KCLKEN_A {
    #[doc = "0: Disable 1 kHz LPO_CLK output"]
    _0 = 0,
    #[doc = "1: Enable 1 kHz LPO_CLK output"]
    _1 = 1,
}
impl From<LPO1KCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPO1KCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPO1KCLKEN`"]
pub type LPO1KCLKEN_R = crate::R<bool, LPO1KCLKEN_A>;
impl LPO1KCLKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPO1KCLKEN_A {
        match self.bits {
            false => LPO1KCLKEN_A::_0,
            true => LPO1KCLKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPO1KCLKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPO1KCLKEN_A::_1
    }
}
#[doc = "Write proxy for field `LPO1KCLKEN`"]
pub struct LPO1KCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPO1KCLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPO1KCLKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable 1 kHz LPO_CLK output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPO1KCLKEN_A::_0)
    }
    #[doc = "Enable 1 kHz LPO_CLK output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPO1KCLKEN_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "32 kHz LPO_CLK enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPO32KCLKEN_A {
    #[doc = "0: Disable 32 kHz LPO_CLK output"]
    _0 = 0,
    #[doc = "1: Enable 32 kHz LPO_CLK output"]
    _1 = 1,
}
impl From<LPO32KCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPO32KCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LPO32KCLKEN`"]
pub type LPO32KCLKEN_R = crate::R<bool, LPO32KCLKEN_A>;
impl LPO32KCLKEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPO32KCLKEN_A {
        match self.bits {
            false => LPO32KCLKEN_A::_0,
            true => LPO32KCLKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPO32KCLKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPO32KCLKEN_A::_1
    }
}
#[doc = "Write proxy for field `LPO32KCLKEN`"]
pub struct LPO32KCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPO32KCLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPO32KCLKEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable 32 kHz LPO_CLK output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPO32KCLKEN_A::_0)
    }
    #[doc = "Enable 32 kHz LPO_CLK output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPO32KCLKEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "LPO clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPOCLKSEL_A {
    #[doc = "0: 128 kHz LPO_CLK"]
    _00 = 0,
    #[doc = "1: No clock"]
    _01 = 1,
    #[doc = "2: 32 kHz LPO_CLK which is derived from the 128 kHz LPO_CLK"]
    _10 = 2,
    #[doc = "3: 1 kHz LPO_CLK which is derived from the 128 kHz LPO_CLK"]
    _11 = 3,
}
impl From<LPOCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPOCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPOCLKSEL`"]
pub type LPOCLKSEL_R = crate::R<u8, LPOCLKSEL_A>;
impl LPOCLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPOCLKSEL_A {
        match self.bits {
            0 => LPOCLKSEL_A::_00,
            1 => LPOCLKSEL_A::_01,
            2 => LPOCLKSEL_A::_10,
            3 => LPOCLKSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LPOCLKSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LPOCLKSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LPOCLKSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == LPOCLKSEL_A::_11
    }
}
#[doc = "Write proxy for field `LPOCLKSEL`"]
pub struct LPOCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPOCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPOCLKSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "128 kHz LPO_CLK"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPOCLKSEL_A::_00)
    }
    #[doc = "No clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPOCLKSEL_A::_01)
    }
    #[doc = "32 kHz LPO_CLK which is derived from the 128 kHz LPO_CLK"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPOCLKSEL_A::_10)
    }
    #[doc = "1 kHz LPO_CLK which is derived from the 128 kHz LPO_CLK"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(LPOCLKSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "32 kHz clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCCLKSEL_A {
    #[doc = "0: SOSCDIV1_CLK"]
    _00 = 0,
    #[doc = "1: 32 kHz LPO_CLK"]
    _01 = 1,
    #[doc = "2: RTC_CLKIN clock"]
    _10 = 2,
    #[doc = "3: FIRCDIV1_CLK"]
    _11 = 3,
}
impl From<RTCCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `RTCCLKSEL`"]
pub type RTCCLKSEL_R = crate::R<u8, RTCCLKSEL_A>;
impl RTCCLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCLKSEL_A {
        match self.bits {
            0 => RTCCLKSEL_A::_00,
            1 => RTCCLKSEL_A::_01,
            2 => RTCCLKSEL_A::_10,
            3 => RTCCLKSEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RTCCLKSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RTCCLKSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RTCCLKSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RTCCLKSEL_A::_11
    }
}
#[doc = "Write proxy for field `RTCCLKSEL`"]
pub struct RTCCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCCLKSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "SOSCDIV1_CLK"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RTCCLKSEL_A::_00)
    }
    #[doc = "32 kHz LPO_CLK"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RTCCLKSEL_A::_01)
    }
    #[doc = "RTC_CLKIN clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RTCCLKSEL_A::_10)
    }
    #[doc = "FIRCDIV1_CLK"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RTCCLKSEL_A::_11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1 kHz LPO_CLK enable"]
    #[inline(always)]
    pub fn lpo1kclken(&self) -> LPO1KCLKEN_R {
        LPO1KCLKEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 32 kHz LPO_CLK enable"]
    #[inline(always)]
    pub fn lpo32kclken(&self) -> LPO32KCLKEN_R {
        LPO32KCLKEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - LPO clock source select"]
    #[inline(always)]
    pub fn lpoclksel(&self) -> LPOCLKSEL_R {
        LPOCLKSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - 32 kHz clock source select"]
    #[inline(always)]
    pub fn rtcclksel(&self) -> RTCCLKSEL_R {
        RTCCLKSEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1 kHz LPO_CLK enable"]
    #[inline(always)]
    pub fn lpo1kclken(&mut self) -> LPO1KCLKEN_W {
        LPO1KCLKEN_W { w: self }
    }
    #[doc = "Bit 1 - 32 kHz LPO_CLK enable"]
    #[inline(always)]
    pub fn lpo32kclken(&mut self) -> LPO32KCLKEN_W {
        LPO32KCLKEN_W { w: self }
    }
    #[doc = "Bits 2:3 - LPO clock source select"]
    #[inline(always)]
    pub fn lpoclksel(&mut self) -> LPOCLKSEL_W {
        LPOCLKSEL_W { w: self }
    }
    #[doc = "Bits 4:5 - 32 kHz clock source select"]
    #[inline(always)]
    pub fn rtcclksel(&mut self) -> RTCCLKSEL_W {
        RTCCLKSEL_W { w: self }
    }
}
