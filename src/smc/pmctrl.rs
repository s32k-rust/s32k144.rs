#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMCTRL {
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
        R { bits: self.register.get() }
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
#[doc = "Possible values of the field `STOPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPMR {
    #[doc = "Normal Stop (STOP)"]
    _000,
    #[doc = "Very-Low-Power Stop (VLPS)"]
    _010,
    #[doc = "Reseved"]
    _110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl STOPMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            STOPMR::_000 => 0,
            STOPMR::_010 => 2,
            STOPMR::_110 => 6,
            STOPMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> STOPMR {
        match value {
            0 => STOPMR::_000,
            2 => STOPMR::_010,
            6 => STOPMR::_110,
            i => STOPMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == STOPMR::_000
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == STOPMR::_010
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == STOPMR::_110
    }
}
#[doc = "Possible values of the field `VLPSA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLPSAR {
    #[doc = "The previous stop mode entry was successful."]
    _0,
    #[doc = "The previous stop mode entry was aborted."]
    _1,
}
impl VLPSAR {
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
            VLPSAR::_0 => false,
            VLPSAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VLPSAR {
        match value {
            false => VLPSAR::_0,
            true => VLPSAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == VLPSAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == VLPSAR::_1
    }
}
#[doc = "Possible values of the field `RUNM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNMR {
    #[doc = "Normal Run mode (RUN)"]
    _00,
    #[doc = "Very-Low-Power Run mode (VLPR)"]
    _10,
    #[doc = "High Speed Run mode (HSRUN)"]
    _11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RUNMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RUNMR::_00 => 0,
            RUNMR::_10 => 2,
            RUNMR::_11 => 3,
            RUNMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RUNMR {
        match value {
            0 => RUNMR::_00,
            2 => RUNMR::_10,
            3 => RUNMR::_11,
            i => RUNMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == RUNMR::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == RUNMR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == RUNMR::_11
    }
}
#[doc = "Values that can be written to the field `STOPM`"]
pub enum STOPMW {
    #[doc = "Normal Stop (STOP)"]
    _000,
    #[doc = "Very-Low-Power Stop (VLPS)"]
    _010,
    #[doc = "Reseved"]
    _110,
}
impl STOPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            STOPMW::_000 => 0,
            STOPMW::_010 => 2,
            STOPMW::_110 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPMW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal Stop (STOP)"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(STOPMW::_000)
    }
    #[doc = "Very-Low-Power Stop (VLPS)"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(STOPMW::_010)
    }
    #[doc = "Reseved"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(STOPMW::_110)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RUNM`"]
pub enum RUNMW {
    #[doc = "Normal Run mode (RUN)"]
    _00,
    #[doc = "Very-Low-Power Run mode (VLPR)"]
    _10,
    #[doc = "High Speed Run mode (HSRUN)"]
    _11,
}
impl RUNMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RUNMW::_00 => 0,
            RUNMW::_10 => 2,
            RUNMW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RUNMW<'a> {
    w: &'a mut W,
}
impl<'a> _RUNMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RUNMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Normal Run mode (RUN)"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(RUNMW::_00)
    }
    #[doc = "Very-Low-Power Run mode (VLPR)"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(RUNMW::_10)
    }
    #[doc = "High Speed Run mode (HSRUN)"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(RUNMW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
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
    #[doc = "Bits 0:2 - Stop Mode Control"]
    #[inline]
    pub fn stopm(&self) -> STOPMR {
        STOPMR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Very Low Power Stop Aborted"]
    #[inline]
    pub fn vlpsa(&self) -> VLPSAR {
        VLPSAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 5:6 - Run Mode Control"]
    #[inline]
    pub fn runm(&self) -> RUNMR {
        RUNMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:2 - Stop Mode Control"]
    #[inline]
    pub fn stopm(&mut self) -> _STOPMW {
        _STOPMW { w: self }
    }
    #[doc = "Bits 5:6 - Run Mode Control"]
    #[inline]
    pub fn runm(&mut self) -> _RUNMW {
        _RUNMW { w: self }
    }
}
