#[doc = "Reader of register CHIPCTL"]
pub type R = crate::R<u32, super::CHIPCTL>;
#[doc = "Writer for register CHIPCTL"]
pub type W = crate::W<u32, super::CHIPCTL>;
#[doc = "Register CHIPCTL `reset()`'s with value 0x0030_0000"]
impl crate::ResetValue for super::CHIPCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0030_0000
    }
}
#[doc = "ADC interleave channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_INTERLEAVE_EN_A {
    #[doc = "0: Interleaving disabled. No channel pair interleaved. Interleaved channels are individually connected to pins. PTC0 is connected to ADC0_SE8. PTC1 is connected to ADC0_SE9. PTB15 is connected to ADC1_SE14. PTB16 is connected to ADC1_SE15. PTB0 is connected to ADC0_SE4. PTB1 is connected to ADC0_SE5. PTB13 is connected to ADC1_SE8. PTB14 is connected to ADC1_SE9."]
    _0000 = 0,
    #[doc = "8: PTB14 to ADC1_SE9 and ADC0_SE9"]
    _1XXX = 8,
    #[doc = "4: PTB13 to ADC1_SE8 and ADC0_SE8"]
    X1XX = 4,
    #[doc = "2: PTB1 to ADC0_SE5 and ADC1_SE15"]
    XX1X = 2,
    #[doc = "1: PTB0 to ADC0_SE4 and ADC1_SE14"]
    XXX1 = 1,
}
impl From<ADC_INTERLEAVE_EN_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_INTERLEAVE_EN_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC_INTERLEAVE_EN`"]
pub type ADC_INTERLEAVE_EN_R = crate::R<u8, ADC_INTERLEAVE_EN_A>;
impl ADC_INTERLEAVE_EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADC_INTERLEAVE_EN_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADC_INTERLEAVE_EN_A::_0000),
            8 => Val(ADC_INTERLEAVE_EN_A::_1XXX),
            4 => Val(ADC_INTERLEAVE_EN_A::X1XX),
            2 => Val(ADC_INTERLEAVE_EN_A::XX1X),
            1 => Val(ADC_INTERLEAVE_EN_A::XXX1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == ADC_INTERLEAVE_EN_A::_0000
    }
    #[doc = "Checks if the value of the field is `_1XXX`"]
    #[inline(always)]
    pub fn is_1xxx(&self) -> bool {
        *self == ADC_INTERLEAVE_EN_A::_1XXX
    }
    #[doc = "Checks if the value of the field is `X1XX`"]
    #[inline(always)]
    pub fn is_x1xx(&self) -> bool {
        *self == ADC_INTERLEAVE_EN_A::X1XX
    }
    #[doc = "Checks if the value of the field is `XX1X`"]
    #[inline(always)]
    pub fn is_xx1x(&self) -> bool {
        *self == ADC_INTERLEAVE_EN_A::XX1X
    }
    #[doc = "Checks if the value of the field is `XXX1`"]
    #[inline(always)]
    pub fn is_xxx1(&self) -> bool {
        *self == ADC_INTERLEAVE_EN_A::XXX1
    }
}
#[doc = "Write proxy for field `ADC_INTERLEAVE_EN`"]
pub struct ADC_INTERLEAVE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_INTERLEAVE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_INTERLEAVE_EN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interleaving disabled. No channel pair interleaved. Interleaved channels are individually connected to pins. PTC0 is connected to ADC0_SE8. PTC1 is connected to ADC0_SE9. PTB15 is connected to ADC1_SE14. PTB16 is connected to ADC1_SE15. PTB0 is connected to ADC0_SE4. PTB1 is connected to ADC0_SE5. PTB13 is connected to ADC1_SE8. PTB14 is connected to ADC1_SE9."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(ADC_INTERLEAVE_EN_A::_0000)
    }
    #[doc = "PTB14 to ADC1_SE9 and ADC0_SE9"]
    #[inline(always)]
    pub fn _1xxx(self) -> &'a mut W {
        self.variant(ADC_INTERLEAVE_EN_A::_1XXX)
    }
    #[doc = "PTB13 to ADC1_SE8 and ADC0_SE8"]
    #[inline(always)]
    pub fn x1xx(self) -> &'a mut W {
        self.variant(ADC_INTERLEAVE_EN_A::X1XX)
    }
    #[doc = "PTB1 to ADC0_SE5 and ADC1_SE15"]
    #[inline(always)]
    pub fn xx1x(self) -> &'a mut W {
        self.variant(ADC_INTERLEAVE_EN_A::XX1X)
    }
    #[doc = "PTB0 to ADC0_SE4 and ADC1_SE14"]
    #[inline(always)]
    pub fn xxx1(self) -> &'a mut W {
        self.variant(ADC_INTERLEAVE_EN_A::XXX1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "CLKOUT Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKOUTSEL_A {
    #[doc = "0: SCG CLKOUT"]
    _0000 = 0,
    #[doc = "2: SOSC DIV2 CLK"]
    _0010 = 2,
    #[doc = "4: SIRC DIV2 CLK"]
    _0100 = 4,
    #[doc = "5: For S32K148: QSPI SFIF_CLK_HYP: Divide by 2 clock (configured through SCLKCONFIG\\[5\\]) for HyperRAM going to sfif clock to QSPI; For others: Reserved"]
    _0101 = 5,
    #[doc = "6: FIRC DIV2 CLK"]
    _0110 = 6,
    #[doc = "7: HCLK"]
    _0111 = 7,
    #[doc = "8: SPLL DIV2 CLK"]
    _1000 = 8,
    #[doc = "9: BUS_CLK"]
    _1001 = 9,
    #[doc = "10: LPO128K_CLK"]
    _1010 = 10,
    #[doc = "11: For S32K148: QSPI IPG_CLK; For others: Reserved"]
    _1011 = 11,
    #[doc = "12: LPO_CLK as selected by SIM_LPOCLKS\\[LPOCLKSEL\\]"]
    _1100 = 12,
    #[doc = "13: For S32K148: QSPI IPG_CLK_SFIF; For others: Reserved"]
    _1101 = 13,
    #[doc = "14: RTC_CLK as selected by SIM_LPOCLKS\\[RTCCLKSEL\\]"]
    _1110 = 14,
    #[doc = "15: For S32K148: QSPI IPG_CLK_2XSFIF; For others: Reserved"]
    _1111 = 15,
}
impl From<CLKOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKOUTSEL`"]
pub type CLKOUTSEL_R = crate::R<u8, CLKOUTSEL_A>;
impl CLKOUTSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKOUTSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKOUTSEL_A::_0000),
            2 => Val(CLKOUTSEL_A::_0010),
            4 => Val(CLKOUTSEL_A::_0100),
            5 => Val(CLKOUTSEL_A::_0101),
            6 => Val(CLKOUTSEL_A::_0110),
            7 => Val(CLKOUTSEL_A::_0111),
            8 => Val(CLKOUTSEL_A::_1000),
            9 => Val(CLKOUTSEL_A::_1001),
            10 => Val(CLKOUTSEL_A::_1010),
            11 => Val(CLKOUTSEL_A::_1011),
            12 => Val(CLKOUTSEL_A::_1100),
            13 => Val(CLKOUTSEL_A::_1101),
            14 => Val(CLKOUTSEL_A::_1110),
            15 => Val(CLKOUTSEL_A::_1111),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == CLKOUTSEL_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == CLKOUTSEL_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == CLKOUTSEL_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == CLKOUTSEL_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == CLKOUTSEL_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == CLKOUTSEL_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == CLKOUTSEL_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == CLKOUTSEL_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == CLKOUTSEL_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == CLKOUTSEL_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == CLKOUTSEL_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == CLKOUTSEL_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == CLKOUTSEL_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == CLKOUTSEL_A::_1111
    }
}
#[doc = "Write proxy for field `CLKOUTSEL`"]
pub struct CLKOUTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SCG CLKOUT"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0000)
    }
    #[doc = "SOSC DIV2 CLK"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0010)
    }
    #[doc = "SIRC DIV2 CLK"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0100)
    }
    #[doc = "For S32K148: QSPI SFIF_CLK_HYP: Divide by 2 clock (configured through SCLKCONFIG\\[5\\]) for HyperRAM going to sfif clock to QSPI; For others: Reserved"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0101)
    }
    #[doc = "FIRC DIV2 CLK"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0110)
    }
    #[doc = "HCLK"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_0111)
    }
    #[doc = "SPLL DIV2 CLK"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_1000)
    }
    #[doc = "BUS_CLK"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_1001)
    }
    #[doc = "LPO128K_CLK"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_1010)
    }
    #[doc = "For S32K148: QSPI IPG_CLK; For others: Reserved"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_1011)
    }
    #[doc = "LPO_CLK as selected by SIM_LPOCLKS\\[LPOCLKSEL\\]"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_1100)
    }
    #[doc = "For S32K148: QSPI IPG_CLK_SFIF; For others: Reserved"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_1101)
    }
    #[doc = "RTC_CLK as selected by SIM_LPOCLKS\\[RTCCLKSEL\\]"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_1110)
    }
    #[doc = "For S32K148: QSPI IPG_CLK_2XSFIF; For others: Reserved"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_1111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "CLKOUT Divide Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKOUTDIV_A {
    #[doc = "0: Divide by 1"]
    _000 = 0,
    #[doc = "1: Divide by 2"]
    _001 = 1,
    #[doc = "2: Divide by 3"]
    _010 = 2,
    #[doc = "3: Divide by 4"]
    _011 = 3,
    #[doc = "4: Divide by 5"]
    _100 = 4,
    #[doc = "5: Divide by 6"]
    _101 = 5,
    #[doc = "6: Divide by 7"]
    _110 = 6,
    #[doc = "7: Divide by 8"]
    _111 = 7,
}
impl From<CLKOUTDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CLKOUTDIV`"]
pub type CLKOUTDIV_R = crate::R<u8, CLKOUTDIV_A>;
impl CLKOUTDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKOUTDIV_A {
        match self.bits {
            0 => CLKOUTDIV_A::_000,
            1 => CLKOUTDIV_A::_001,
            2 => CLKOUTDIV_A::_010,
            3 => CLKOUTDIV_A::_011,
            4 => CLKOUTDIV_A::_100,
            5 => CLKOUTDIV_A::_101,
            6 => CLKOUTDIV_A::_110,
            7 => CLKOUTDIV_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CLKOUTDIV_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CLKOUTDIV_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CLKOUTDIV_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CLKOUTDIV_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CLKOUTDIV_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CLKOUTDIV_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CLKOUTDIV_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CLKOUTDIV_A::_111
    }
}
#[doc = "Write proxy for field `CLKOUTDIV`"]
pub struct CLKOUTDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUTDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::_000)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::_001)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::_010)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::_011)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::_100)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::_101)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::_110)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CLKOUTDIV_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "CLKOUT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKOUTEN_A {
    #[doc = "0: Clockout disable"]
    _0 = 0,
    #[doc = "1: Clockout enable"]
    _1 = 1,
}
impl From<CLKOUTEN_A> for bool {
    #[inline(always)]
    fn from(variant: CLKOUTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKOUTEN`"]
pub type CLKOUTEN_R = crate::R<bool, CLKOUTEN_A>;
impl CLKOUTEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKOUTEN_A {
        match self.bits {
            false => CLKOUTEN_A::_0,
            true => CLKOUTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKOUTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKOUTEN_A::_1
    }
}
#[doc = "Write proxy for field `CLKOUTEN`"]
pub struct CLKOUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOUTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKOUTEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clockout disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKOUTEN_A::_0)
    }
    #[doc = "Clockout enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKOUTEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Debug trace clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRACECLK_SEL_A {
    #[doc = "0: Core clock"]
    _0 = 0,
    #[doc = "1: Platform clock"]
    _1 = 1,
}
impl From<TRACECLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: TRACECLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRACECLK_SEL`"]
pub type TRACECLK_SEL_R = crate::R<bool, TRACECLK_SEL_A>;
impl TRACECLK_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACECLK_SEL_A {
        match self.bits {
            false => TRACECLK_SEL_A::_0,
            true => TRACECLK_SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRACECLK_SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRACECLK_SEL_A::_1
    }
}
#[doc = "Write proxy for field `TRACECLK_SEL`"]
pub struct TRACECLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACECLK_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRACECLK_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Core clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRACECLK_SEL_A::_0)
    }
    #[doc = "Platform clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRACECLK_SEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "PDB back-to-back select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDB_BB_SEL_A {
    #[doc = "0: PDB0 channel 0 back-to-back operation with ADC0 COCO\\[7:0\\]
and PDB1 channel 0 back-to-back operation with ADC1 COCO\\[7:0\\]"]
    _0 = 0,
    #[doc = "1: Channel 0 of PDB0 and PDB1 back-to-back operation with COCO\\[7:0\\]
of ADC0 and ADC1."]
    _1 = 1,
}
impl From<PDB_BB_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: PDB_BB_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PDB_BB_SEL`"]
pub type PDB_BB_SEL_R = crate::R<bool, PDB_BB_SEL_A>;
impl PDB_BB_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDB_BB_SEL_A {
        match self.bits {
            false => PDB_BB_SEL_A::_0,
            true => PDB_BB_SEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDB_BB_SEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDB_BB_SEL_A::_1
    }
}
#[doc = "Write proxy for field `PDB_BB_SEL`"]
pub struct PDB_BB_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PDB_BB_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDB_BB_SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PDB0 channel 0 back-to-back operation with ADC0 COCO\\[7:0\\]
and PDB1 channel 0 back-to-back operation with ADC1 COCO\\[7:0\\]"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDB_BB_SEL_A::_0)
    }
    #[doc = "Channel 0 of PDB0 and PDB1 back-to-back operation with COCO\\[7:0\\]
of ADC0 and ADC1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDB_BB_SEL_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "ADC_SUPPLY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC_SUPPLY_A {
    #[doc = "0: 5 V input VDD supply (VDD)"]
    _000 = 0,
    #[doc = "1: 5 V input analog supply (VDDA)"]
    _001 = 1,
    #[doc = "2: ADC Reference Supply (VREFH)"]
    _010 = 2,
    #[doc = "3: 3.3 V Oscillator Regulator Output (VDD_3V)"]
    _011 = 3,
    #[doc = "4: 3.3 V flash regulator output (VDD_flash_3V)"]
    _100 = 4,
    #[doc = "5: 1.2 V core regulator output (VDD_LV)"]
    _101 = 5,
}
impl From<ADC_SUPPLY_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC_SUPPLY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADC_SUPPLY`"]
pub type ADC_SUPPLY_R = crate::R<u8, ADC_SUPPLY_A>;
impl ADC_SUPPLY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADC_SUPPLY_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADC_SUPPLY_A::_000),
            1 => Val(ADC_SUPPLY_A::_001),
            2 => Val(ADC_SUPPLY_A::_010),
            3 => Val(ADC_SUPPLY_A::_011),
            4 => Val(ADC_SUPPLY_A::_100),
            5 => Val(ADC_SUPPLY_A::_101),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == ADC_SUPPLY_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == ADC_SUPPLY_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == ADC_SUPPLY_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == ADC_SUPPLY_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == ADC_SUPPLY_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == ADC_SUPPLY_A::_101
    }
}
#[doc = "Write proxy for field `ADC_SUPPLY`"]
pub struct ADC_SUPPLY_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SUPPLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_SUPPLY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "5 V input VDD supply (VDD)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(ADC_SUPPLY_A::_000)
    }
    #[doc = "5 V input analog supply (VDDA)"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(ADC_SUPPLY_A::_001)
    }
    #[doc = "ADC Reference Supply (VREFH)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(ADC_SUPPLY_A::_010)
    }
    #[doc = "3.3 V Oscillator Regulator Output (VDD_3V)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(ADC_SUPPLY_A::_011)
    }
    #[doc = "3.3 V flash regulator output (VDD_flash_3V)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(ADC_SUPPLY_A::_100)
    }
    #[doc = "1.2 V core regulator output (VDD_LV)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(ADC_SUPPLY_A::_101)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "ADC_SUPPLYEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_SUPPLYEN_A {
    #[doc = "0: Disable internal supply monitoring"]
    _0 = 0,
    #[doc = "1: Enable internal supply monitoring"]
    _1 = 1,
}
impl From<ADC_SUPPLYEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC_SUPPLYEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ADC_SUPPLYEN`"]
pub type ADC_SUPPLYEN_R = crate::R<bool, ADC_SUPPLYEN_A>;
impl ADC_SUPPLYEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC_SUPPLYEN_A {
        match self.bits {
            false => ADC_SUPPLYEN_A::_0,
            true => ADC_SUPPLYEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADC_SUPPLYEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC_SUPPLYEN_A::_1
    }
}
#[doc = "Write proxy for field `ADC_SUPPLYEN`"]
pub struct ADC_SUPPLYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SUPPLYEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_SUPPLYEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable internal supply monitoring"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC_SUPPLYEN_A::_0)
    }
    #[doc = "Enable internal supply monitoring"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC_SUPPLYEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "SRAMU_RETEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMU_RETEN_A {
    #[doc = "0: SRAMU contents are retained across resets"]
    _0 = 0,
    #[doc = "1: No SRAMU retention"]
    _1 = 1,
}
impl From<SRAMU_RETEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRAMU_RETEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAMU_RETEN`"]
pub type SRAMU_RETEN_R = crate::R<bool, SRAMU_RETEN_A>;
impl SRAMU_RETEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMU_RETEN_A {
        match self.bits {
            false => SRAMU_RETEN_A::_0,
            true => SRAMU_RETEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRAMU_RETEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRAMU_RETEN_A::_1
    }
}
#[doc = "Write proxy for field `SRAMU_RETEN`"]
pub struct SRAMU_RETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMU_RETEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAMU_RETEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SRAMU contents are retained across resets"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRAMU_RETEN_A::_0)
    }
    #[doc = "No SRAMU retention"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRAMU_RETEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "SRAML_RETEN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAML_RETEN_A {
    #[doc = "0: SRAML contents are retained across resets"]
    _0 = 0,
    #[doc = "1: No SRAML retention"]
    _1 = 1,
}
impl From<SRAML_RETEN_A> for bool {
    #[inline(always)]
    fn from(variant: SRAML_RETEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SRAML_RETEN`"]
pub type SRAML_RETEN_R = crate::R<bool, SRAML_RETEN_A>;
impl SRAML_RETEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAML_RETEN_A {
        match self.bits {
            false => SRAML_RETEN_A::_0,
            true => SRAML_RETEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRAML_RETEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRAML_RETEN_A::_1
    }
}
#[doc = "Write proxy for field `SRAML_RETEN`"]
pub struct SRAML_RETEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAML_RETEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAML_RETEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "SRAML contents are retained across resets"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRAML_RETEN_A::_0)
    }
    #[doc = "No SRAML retention"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRAML_RETEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC interleave channel enable"]
    #[inline(always)]
    pub fn adc_interleave_en(&self) -> ADC_INTERLEAVE_EN_R {
        ADC_INTERLEAVE_EN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CLKOUT Select"]
    #[inline(always)]
    pub fn clkoutsel(&self) -> CLKOUTSEL_R {
        CLKOUTSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - CLKOUT Divide Ratio"]
    #[inline(always)]
    pub fn clkoutdiv(&self) -> CLKOUTDIV_R {
        CLKOUTDIV_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - CLKOUT enable"]
    #[inline(always)]
    pub fn clkouten(&self) -> CLKOUTEN_R {
        CLKOUTEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline(always)]
    pub fn traceclk_sel(&self) -> TRACECLK_SEL_R {
        TRACECLK_SEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PDB back-to-back select"]
    #[inline(always)]
    pub fn pdb_bb_sel(&self) -> PDB_BB_SEL_R {
        PDB_BB_SEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - ADC_SUPPLY"]
    #[inline(always)]
    pub fn adc_supply(&self) -> ADC_SUPPLY_R {
        ADC_SUPPLY_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 19 - ADC_SUPPLYEN"]
    #[inline(always)]
    pub fn adc_supplyen(&self) -> ADC_SUPPLYEN_R {
        ADC_SUPPLYEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - SRAMU_RETEN"]
    #[inline(always)]
    pub fn sramu_reten(&self) -> SRAMU_RETEN_R {
        SRAMU_RETEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - SRAML_RETEN"]
    #[inline(always)]
    pub fn sraml_reten(&self) -> SRAML_RETEN_R {
        SRAML_RETEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC interleave channel enable"]
    #[inline(always)]
    pub fn adc_interleave_en(&mut self) -> ADC_INTERLEAVE_EN_W {
        ADC_INTERLEAVE_EN_W { w: self }
    }
    #[doc = "Bits 4:7 - CLKOUT Select"]
    #[inline(always)]
    pub fn clkoutsel(&mut self) -> CLKOUTSEL_W {
        CLKOUTSEL_W { w: self }
    }
    #[doc = "Bits 8:10 - CLKOUT Divide Ratio"]
    #[inline(always)]
    pub fn clkoutdiv(&mut self) -> CLKOUTDIV_W {
        CLKOUTDIV_W { w: self }
    }
    #[doc = "Bit 11 - CLKOUT enable"]
    #[inline(always)]
    pub fn clkouten(&mut self) -> CLKOUTEN_W {
        CLKOUTEN_W { w: self }
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline(always)]
    pub fn traceclk_sel(&mut self) -> TRACECLK_SEL_W {
        TRACECLK_SEL_W { w: self }
    }
    #[doc = "Bit 13 - PDB back-to-back select"]
    #[inline(always)]
    pub fn pdb_bb_sel(&mut self) -> PDB_BB_SEL_W {
        PDB_BB_SEL_W { w: self }
    }
    #[doc = "Bits 16:18 - ADC_SUPPLY"]
    #[inline(always)]
    pub fn adc_supply(&mut self) -> ADC_SUPPLY_W {
        ADC_SUPPLY_W { w: self }
    }
    #[doc = "Bit 19 - ADC_SUPPLYEN"]
    #[inline(always)]
    pub fn adc_supplyen(&mut self) -> ADC_SUPPLYEN_W {
        ADC_SUPPLYEN_W { w: self }
    }
    #[doc = "Bit 20 - SRAMU_RETEN"]
    #[inline(always)]
    pub fn sramu_reten(&mut self) -> SRAMU_RETEN_W {
        SRAMU_RETEN_W { w: self }
    }
    #[doc = "Bit 21 - SRAML_RETEN"]
    #[inline(always)]
    pub fn sraml_reten(&mut self) -> SRAML_RETEN_W {
        SRAML_RETEN_W { w: self }
    }
}
