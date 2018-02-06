#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LPOCLKS {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `LPO1KCLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPO1KCLKENR {
    #[doc = "Disable 1 kHz LPO_CLK output"]
    _0,
    #[doc = "Enable 1 kHz LPO_CLK output"]
    _1,
}
impl LPO1KCLKENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LPO1KCLKENR::_0 => false,
            LPO1KCLKENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPO1KCLKENR {
        match value {
            false => LPO1KCLKENR::_0,
            true => LPO1KCLKENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LPO1KCLKENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LPO1KCLKENR::_1
    }
}
#[doc = "Possible values of the field `LPO32KCLKEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPO32KCLKENR {
    #[doc = "Disable 32 kHz LPO_CLK output"]
    _0,
    #[doc = "Enable 32 kHz LPO_CLK output"]
    _1,
}
impl LPO32KCLKENR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LPO32KCLKENR::_0 => false,
            LPO32KCLKENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPO32KCLKENR {
        match value {
            false => LPO32KCLKENR::_0,
            true => LPO32KCLKENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LPO32KCLKENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LPO32KCLKENR::_1
    }
}
#[doc = "Possible values of the field `LPOCLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOCLKSELR {
    #[doc = "128 kHz LPO_CLK"]
    _00,
    #[doc = "No clock"]
    _01,
    #[doc = "32 kHz LPO_CLK which is derived from the 128 kHz LPO_CLK"]
    _10,
    #[doc = "1 kHz LPO_CLK which is derived from the 128 kHz LPO_CLK"]
    _11,
}
impl LPOCLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LPOCLKSELR::_00 => 0,
            LPOCLKSELR::_01 => 1,
            LPOCLKSELR::_10 => 2,
            LPOCLKSELR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LPOCLKSELR {
        match value {
            0 => LPOCLKSELR::_00,
            1 => LPOCLKSELR::_01,
            2 => LPOCLKSELR::_10,
            3 => LPOCLKSELR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LPOCLKSELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LPOCLKSELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == LPOCLKSELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == LPOCLKSELR::_11
    }
}
#[doc = "Possible values of the field `RTCCLKSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCCLKSELR {
    #[doc = "SOSCDIV1_CLK"]
    _00,
    #[doc = "32 kHz LPO_CLK"]
    _01,
    #[doc = "RTC_CLKIN clock"]
    _10,
    #[doc = "FIRCDIV1_CLK"]
    _11,
}
impl RTCCLKSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RTCCLKSELR::_00 => 0,
            RTCCLKSELR::_01 => 1,
            RTCCLKSELR::_10 => 2,
            RTCCLKSELR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RTCCLKSELR {
        match value {
            0 => RTCCLKSELR::_00,
            1 => RTCCLKSELR::_01,
            2 => RTCCLKSELR::_10,
            3 => RTCCLKSELR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == RTCCLKSELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == RTCCLKSELR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == RTCCLKSELR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == RTCCLKSELR::_11
    }
}
#[doc = "Values that can be written to the field `LPO1KCLKEN`"]
pub enum LPO1KCLKENW {
    #[doc = "Disable 1 kHz LPO_CLK output"]
    _0,
    #[doc = "Enable 1 kHz LPO_CLK output"]
    _1,
}
impl LPO1KCLKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPO1KCLKENW::_0 => false,
            LPO1KCLKENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPO1KCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _LPO1KCLKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPO1KCLKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable 1 kHz LPO_CLK output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPO1KCLKENW::_0)
    }
    #[doc = "Enable 1 kHz LPO_CLK output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPO1KCLKENW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPO32KCLKEN`"]
pub enum LPO32KCLKENW {
    #[doc = "Disable 32 kHz LPO_CLK output"]
    _0,
    #[doc = "Enable 32 kHz LPO_CLK output"]
    _1,
}
impl LPO32KCLKENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPO32KCLKENW::_0 => false,
            LPO32KCLKENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPO32KCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _LPO32KCLKENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPO32KCLKENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable 32 kHz LPO_CLK output"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPO32KCLKENW::_0)
    }
    #[doc = "Enable 32 kHz LPO_CLK output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPO32KCLKENW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPOCLKSEL`"]
pub enum LPOCLKSELW {
    #[doc = "128 kHz LPO_CLK"]
    _00,
    #[doc = "No clock"]
    _01,
    #[doc = "32 kHz LPO_CLK which is derived from the 128 kHz LPO_CLK"]
    _10,
    #[doc = "1 kHz LPO_CLK which is derived from the 128 kHz LPO_CLK"]
    _11,
}
impl LPOCLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LPOCLKSELW::_00 => 0,
            LPOCLKSELW::_01 => 1,
            LPOCLKSELW::_10 => 2,
            LPOCLKSELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPOCLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _LPOCLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPOCLKSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "128 kHz LPO_CLK"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(LPOCLKSELW::_00)
    }
    #[doc = "No clock"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(LPOCLKSELW::_01)
    }
    #[doc = "32 kHz LPO_CLK which is derived from the 128 kHz LPO_CLK"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(LPOCLKSELW::_10)
    }
    #[doc = "1 kHz LPO_CLK which is derived from the 128 kHz LPO_CLK"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(LPOCLKSELW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RTCCLKSEL`"]
pub enum RTCCLKSELW {
    #[doc = "SOSCDIV1_CLK"]
    _00,
    #[doc = "32 kHz LPO_CLK"]
    _01,
    #[doc = "RTC_CLKIN clock"]
    _10,
    #[doc = "FIRCDIV1_CLK"]
    _11,
}
impl RTCCLKSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RTCCLKSELW::_00 => 0,
            RTCCLKSELW::_01 => 1,
            RTCCLKSELW::_10 => 2,
            RTCCLKSELW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTCCLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RTCCLKSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTCCLKSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "SOSCDIV1_CLK"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(RTCCLKSELW::_00)
    }
    #[doc = "32 kHz LPO_CLK"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(RTCCLKSELW::_01)
    }
    #[doc = "RTC_CLKIN clock"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(RTCCLKSELW::_10)
    }
    #[doc = "FIRCDIV1_CLK"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(RTCCLKSELW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - 1 kHz LPO_CLK enable"]
    #[inline]
    pub fn lpo1kclken(&self) -> LPO1KCLKENR {
        LPO1KCLKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - 32 kHz LPO_CLK enable"]
    #[inline]
    pub fn lpo32kclken(&self) -> LPO32KCLKENR {
        LPO32KCLKENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - LPO clock source select"]
    #[inline]
    pub fn lpoclksel(&self) -> LPOCLKSELR {
        LPOCLKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - 32 kHz clock source select"]
    #[inline]
    pub fn rtcclksel(&self) -> RTCCLKSELR {
        RTCCLKSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - 1 kHz LPO_CLK enable"]
    #[inline]
    pub fn lpo1kclken(&mut self) -> _LPO1KCLKENW {
        _LPO1KCLKENW { w: self }
    }
    #[doc = "Bit 1 - 32 kHz LPO_CLK enable"]
    #[inline]
    pub fn lpo32kclken(&mut self) -> _LPO32KCLKENW {
        _LPO32KCLKENW { w: self }
    }
    #[doc = "Bits 2:3 - LPO clock source select"]
    #[inline]
    pub fn lpoclksel(&mut self) -> _LPOCLKSELW {
        _LPOCLKSELW { w: self }
    }
    #[doc = "Bits 4:5 - 32 kHz clock source select"]
    #[inline]
    pub fn rtcclksel(&mut self) -> _RTCCLKSELW {
        _RTCCLKSELW { w: self }
    }
}
