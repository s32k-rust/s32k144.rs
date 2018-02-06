#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCR {
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
#[doc = "Possible values of the field `TCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCRR {
    #[doc = "Time Prescaler Register overflows every 32896 clock cycles."]
    _10000000,
    #[doc = "Time Prescaler Register overflows every 32895 clock cycles."]
    _10000001,
    #[doc = "Time Prescaler Register overflows every 32769 clock cycles."]
    _11111111,
    #[doc = "Time Prescaler Register overflows every 32768 clock cycles."]
    _00000000,
    #[doc = "Time Prescaler Register overflows every 32767 clock cycles."]
    _00000001,
    #[doc = "Time Prescaler Register overflows every 32642 clock cycles."]
    _01111110,
    #[doc = "Time Prescaler Register overflows every 32641 clock cycles."]
    _01111111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TCRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TCRR::_10000000 => 128,
            TCRR::_10000001 => 129,
            TCRR::_11111111 => 255,
            TCRR::_00000000 => 0,
            TCRR::_00000001 => 1,
            TCRR::_01111110 => 126,
            TCRR::_01111111 => 127,
            TCRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TCRR {
        match value {
            128 => TCRR::_10000000,
            129 => TCRR::_10000001,
            255 => TCRR::_11111111,
            0 => TCRR::_00000000,
            1 => TCRR::_00000001,
            126 => TCRR::_01111110,
            127 => TCRR::_01111111,
            i => TCRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_10000000`"]
    #[inline]
    pub fn is_10000000(&self) -> bool {
        *self == TCRR::_10000000
    }
    #[doc = "Checks if the value of the field is `_10000001`"]
    #[inline]
    pub fn is_10000001(&self) -> bool {
        *self == TCRR::_10000001
    }
    #[doc = "Checks if the value of the field is `_11111111`"]
    #[inline]
    pub fn is_11111111(&self) -> bool {
        *self == TCRR::_11111111
    }
    #[doc = "Checks if the value of the field is `_00000000`"]
    #[inline]
    pub fn is_00000000(&self) -> bool {
        *self == TCRR::_00000000
    }
    #[doc = "Checks if the value of the field is `_00000001`"]
    #[inline]
    pub fn is_00000001(&self) -> bool {
        *self == TCRR::_00000001
    }
    #[doc = "Checks if the value of the field is `_01111110`"]
    #[inline]
    pub fn is_01111110(&self) -> bool {
        *self == TCRR::_01111110
    }
    #[doc = "Checks if the value of the field is `_01111111`"]
    #[inline]
    pub fn is_01111111(&self) -> bool {
        *self == TCRR::_01111111
    }
}
#[doc = r" Value of the field"]
pub struct CIRR {
    bits: u8,
}
impl CIRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TCVR {
    bits: u8,
}
impl TCVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CICR {
    bits: u8,
}
impl CICR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `TCR`"]
pub enum TCRW {
    #[doc = "Time Prescaler Register overflows every 32896 clock cycles."]
    _10000000,
    #[doc = "Time Prescaler Register overflows every 32895 clock cycles."]
    _10000001,
    #[doc = "Time Prescaler Register overflows every 32769 clock cycles."]
    _11111111,
    #[doc = "Time Prescaler Register overflows every 32768 clock cycles."]
    _00000000,
    #[doc = "Time Prescaler Register overflows every 32767 clock cycles."]
    _00000001,
    #[doc = "Time Prescaler Register overflows every 32642 clock cycles."]
    _01111110,
    #[doc = "Time Prescaler Register overflows every 32641 clock cycles."]
    _01111111,
}
impl TCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TCRW::_10000000 => 128,
            TCRW::_10000001 => 129,
            TCRW::_11111111 => 255,
            TCRW::_00000000 => 0,
            TCRW::_00000001 => 1,
            TCRW::_01111110 => 126,
            TCRW::_01111111 => 127,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCRW<'a> {
    w: &'a mut W,
}
impl<'a> _TCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Time Prescaler Register overflows every 32896 clock cycles."]
    #[inline]
    pub fn _10000000(self) -> &'a mut W {
        self.variant(TCRW::_10000000)
    }
    #[doc = "Time Prescaler Register overflows every 32895 clock cycles."]
    #[inline]
    pub fn _10000001(self) -> &'a mut W {
        self.variant(TCRW::_10000001)
    }
    #[doc = "Time Prescaler Register overflows every 32769 clock cycles."]
    #[inline]
    pub fn _11111111(self) -> &'a mut W {
        self.variant(TCRW::_11111111)
    }
    #[doc = "Time Prescaler Register overflows every 32768 clock cycles."]
    #[inline]
    pub fn _00000000(self) -> &'a mut W {
        self.variant(TCRW::_00000000)
    }
    #[doc = "Time Prescaler Register overflows every 32767 clock cycles."]
    #[inline]
    pub fn _00000001(self) -> &'a mut W {
        self.variant(TCRW::_00000001)
    }
    #[doc = "Time Prescaler Register overflows every 32642 clock cycles."]
    #[inline]
    pub fn _01111110(self) -> &'a mut W {
        self.variant(TCRW::_01111110)
    }
    #[doc = "Time Prescaler Register overflows every 32641 clock cycles."]
    #[inline]
    pub fn _01111111(self) -> &'a mut W {
        self.variant(TCRW::_01111111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CIRW<'a> {
    w: &'a mut W,
}
impl<'a> _CIRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:7 - Time Compensation Register"]
    #[inline]
    pub fn tcr(&self) -> TCRR {
        TCRR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - Compensation Interval Register"]
    #[inline]
    pub fn cir(&self) -> CIRR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CIRR { bits }
    }
    #[doc = "Bits 16:23 - Time Compensation Value"]
    #[inline]
    pub fn tcv(&self) -> TCVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TCVR { bits }
    }
    #[doc = "Bits 24:31 - Compensation Interval Counter"]
    #[inline]
    pub fn cic(&self) -> CICR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CICR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Time Compensation Register"]
    #[inline]
    pub fn tcr(&mut self) -> _TCRW {
        _TCRW { w: self }
    }
    #[doc = "Bits 8:15 - Compensation Interval Register"]
    #[inline]
    pub fn cir(&mut self) -> _CIRW {
        _CIRW { w: self }
    }
}
