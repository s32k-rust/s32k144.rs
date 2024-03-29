#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WU_MTC {
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
#[doc = r" Value of the field"]
pub struct MCOUNTERR {
    bits: u8,
}
impl MCOUNTERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `WUMF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUMFR {
    #[doc = "No wake up by match event detected"]
    _0,
    #[doc = "Wake up by match event detected"]
    _1,
}
impl WUMFR {
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
            WUMFR::_0 => false,
            WUMFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WUMFR {
        match value {
            false => WUMFR::_0,
            true => WUMFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WUMFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WUMFR::_1
    }
}
#[doc = "Possible values of the field `WTOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTOFR {
    #[doc = "No wake up by timeout event detected"]
    _0,
    #[doc = "Wake up by timeout event detected"]
    _1,
}
impl WTOFR {
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
            WTOFR::_0 => false,
            WTOFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WTOFR {
        match value {
            false => WTOFR::_0,
            true => WTOFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WTOFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WTOFR::_1
    }
}
#[doc = "Values that can be written to the field `WUMF`"]
pub enum WUMFW {
    #[doc = "No wake up by match event detected"]
    _0,
    #[doc = "Wake up by match event detected"]
    _1,
}
impl WUMFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WUMFW::_0 => false,
            WUMFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WUMFW<'a> {
    w: &'a mut W,
}
impl<'a> _WUMFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WUMFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No wake up by match event detected"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WUMFW::_0)
    }
    #[doc = "Wake up by match event detected"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WUMFW::_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WTOF`"]
pub enum WTOFW {
    #[doc = "No wake up by timeout event detected"]
    _0,
    #[doc = "Wake up by timeout event detected"]
    _1,
}
impl WTOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WTOFW::_0 => false,
            WTOFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WTOFW<'a> {
    w: &'a mut W,
}
impl<'a> _WTOFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WTOFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No wake up by timeout event detected"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WTOFW::_0)
    }
    #[doc = "Wake up by timeout event detected"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WTOFW::_1)
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
        const OFFSET: u8 = 17;
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
    #[doc = "Bits 8:15 - Number of Matches while in Pretended Networking"]
    #[inline]
    pub fn mcounter(&self) -> MCOUNTERR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MCOUNTERR { bits }
    }
    #[doc = "Bit 16 - Wake Up by Match Flag Bit"]
    #[inline]
    pub fn wumf(&self) -> WUMFR {
        WUMFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Wake Up by Timeout Flag Bit"]
    #[inline]
    pub fn wtof(&self) -> WTOFR {
        WTOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
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
    #[doc = "Bit 16 - Wake Up by Match Flag Bit"]
    #[inline]
    pub fn wumf(&mut self) -> _WUMFW {
        _WUMFW { w: self }
    }
    #[doc = "Bit 17 - Wake Up by Timeout Flag Bit"]
    #[inline]
    pub fn wtof(&mut self) -> _WTOFW {
        _WTOFW { w: self }
    }
}
