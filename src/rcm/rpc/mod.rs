#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RPC {
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
#[doc = "Possible values of the field `RSTFLTSRW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTFLTSRWR {
    #[doc = "All filtering disabled"]
    _00,
    #[doc = "Bus clock filter enabled for normal operation"]
    _01,
    #[doc = "LPO clock filter enabled for normal operation"]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RSTFLTSRWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RSTFLTSRWR::_00 => 0,
            RSTFLTSRWR::_01 => 1,
            RSTFLTSRWR::_10 => 2,
            RSTFLTSRWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RSTFLTSRWR {
        match value {
            0 => RSTFLTSRWR::_00,
            1 => RSTFLTSRWR::_01,
            2 => RSTFLTSRWR::_10,
            i => RSTFLTSRWR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == RSTFLTSRWR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == RSTFLTSRWR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == RSTFLTSRWR::_10
    }
}
#[doc = "Possible values of the field `RSTFLTSS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTFLTSSR {
    #[doc = "All filtering disabled"]
    _0,
    #[doc = "LPO clock filter enabled"]
    _1,
}
impl RSTFLTSSR {
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
            RSTFLTSSR::_0 => false,
            RSTFLTSSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTFLTSSR {
        match value {
            false => RSTFLTSSR::_0,
            true => RSTFLTSSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RSTFLTSSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RSTFLTSSR::_1
    }
}
#[doc = r" Value of the field"]
pub struct RSTFLTSELR {
    bits: u8,
}
impl RSTFLTSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `RSTFLTSRW`"]
pub enum RSTFLTSRWW {
    #[doc = "All filtering disabled"]
    _00,
    #[doc = "Bus clock filter enabled for normal operation"]
    _01,
    #[doc = "LPO clock filter enabled for normal operation"]
    _10,
}
impl RSTFLTSRWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RSTFLTSRWW::_00 => 0,
            RSTFLTSRWW::_01 => 1,
            RSTFLTSRWW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTFLTSRWW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTFLTSRWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTFLTSRWW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "All filtering disabled"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(RSTFLTSRWW::_00)
    }
    #[doc = "Bus clock filter enabled for normal operation"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(RSTFLTSRWW::_01)
    }
    #[doc = "LPO clock filter enabled for normal operation"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(RSTFLTSRWW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSTFLTSS`"]
pub enum RSTFLTSSW {
    #[doc = "All filtering disabled"]
    _0,
    #[doc = "LPO clock filter enabled"]
    _1,
}
impl RSTFLTSSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTFLTSSW::_0 => false,
            RSTFLTSSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTFLTSSW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTFLTSSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTFLTSSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "All filtering disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTFLTSSW::_0)
    }
    #[doc = "LPO clock filter enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTFLTSSW::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RSTFLTSELW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTFLTSELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
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
    #[doc = "Bits 0:1 - Reset Pin Filter Select in Run and Wait Modes"]
    #[inline]
    pub fn rstfltsrw(&self) -> RSTFLTSRWR {
        RSTFLTSRWR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Reset Pin Filter Select in Stop Mode"]
    #[inline]
    pub fn rstfltss(&self) -> RSTFLTSSR {
        RSTFLTSSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:12 - Reset Pin Filter Bus Clock Select"]
    #[inline]
    pub fn rstfltsel(&self) -> RSTFLTSELR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RSTFLTSELR { bits }
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
    #[doc = "Bits 0:1 - Reset Pin Filter Select in Run and Wait Modes"]
    #[inline]
    pub fn rstfltsrw(&mut self) -> _RSTFLTSRWW {
        _RSTFLTSRWW { w: self }
    }
    #[doc = "Bit 2 - Reset Pin Filter Select in Stop Mode"]
    #[inline]
    pub fn rstfltss(&mut self) -> _RSTFLTSSW {
        _RSTFLTSSW { w: self }
    }
    #[doc = "Bits 8:12 - Reset Pin Filter Bus Clock Select"]
    #[inline]
    pub fn rstfltsel(&mut self) -> _RSTFLTSELW {
        _RSTFLTSELW { w: self }
    }
}
