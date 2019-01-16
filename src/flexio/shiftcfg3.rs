#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SHIFTCFG3 {
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
#[doc = "Possible values of the field `SSTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSTARTR {
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on enable"]
    _0,
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on first shift"]
    _1,
    #[doc = "Transmitter outputs start bit value 0 before loading data on first shift, receiver/match store sets error flag if start bit is not 0"]
    _10,
    #[doc = "Transmitter outputs start bit value 1 before loading data on first shift, receiver/match store sets error flag if start bit is not 1"]
    _11,
}
impl SSTARTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SSTARTR::_0 => 0,
            SSTARTR::_1 => 1,
            SSTARTR::_10 => 2,
            SSTARTR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SSTARTR {
        match value {
            0 => SSTARTR::_0,
            1 => SSTARTR::_1,
            2 => SSTARTR::_10,
            3 => SSTARTR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SSTARTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SSTARTR::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == SSTARTR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == SSTARTR::_11
    }
}
#[doc = "Possible values of the field `SSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSTOPR {
    #[doc = "Stop bit disabled for transmitter/receiver/match store"]
    _0,
    #[doc = "Reserved for transmitter/receiver/match store"]
    _1,
    #[doc = "Transmitter outputs stop bit value 0 on store, receiver/match store sets error flag if stop bit is not 0"]
    _10,
    #[doc = "Transmitter outputs stop bit value 1 on store, receiver/match store sets error flag if stop bit is not 1"]
    _11,
}
impl SSTOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SSTOPR::_0 => 0,
            SSTOPR::_1 => 1,
            SSTOPR::_10 => 2,
            SSTOPR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SSTOPR {
        match value {
            0 => SSTOPR::_0,
            1 => SSTOPR::_1,
            2 => SSTOPR::_10,
            3 => SSTOPR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SSTOPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SSTOPR::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == SSTOPR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == SSTOPR::_11
    }
}
#[doc = "Possible values of the field `INSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INSRCR {
    #[doc = "Pin"]
    _0,
    #[doc = "Shifter N+1 Output"]
    _1,
}
impl INSRCR {
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
            INSRCR::_0 => false,
            INSRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INSRCR {
        match value {
            false => INSRCR::_0,
            true => INSRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INSRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INSRCR::_1
    }
}
#[doc = "Values that can be written to the field `SSTART`"]
pub enum SSTARTW {
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on enable"]
    _0,
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on first shift"]
    _1,
    #[doc = "Transmitter outputs start bit value 0 before loading data on first shift, receiver/match store sets error flag if start bit is not 0"]
    _10,
    #[doc = "Transmitter outputs start bit value 1 before loading data on first shift, receiver/match store sets error flag if start bit is not 1"]
    _11,
}
impl SSTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SSTARTW::_0 => 0,
            SSTARTW::_1 => 1,
            SSTARTW::_10 => 2,
            SSTARTW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _SSTARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSTARTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on enable"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSTARTW::_0)
    }
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on first shift"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSTARTW::_1)
    }
    #[doc = "Transmitter outputs start bit value 0 before loading data on first shift, receiver/match store sets error flag if start bit is not 0"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(SSTARTW::_10)
    }
    #[doc = "Transmitter outputs start bit value 1 before loading data on first shift, receiver/match store sets error flag if start bit is not 1"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(SSTARTW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SSTOP`"]
pub enum SSTOPW {
    #[doc = "Stop bit disabled for transmitter/receiver/match store"]
    _0,
    #[doc = "Reserved for transmitter/receiver/match store"]
    _1,
    #[doc = "Transmitter outputs stop bit value 0 on store, receiver/match store sets error flag if stop bit is not 0"]
    _10,
    #[doc = "Transmitter outputs stop bit value 1 on store, receiver/match store sets error flag if stop bit is not 1"]
    _11,
}
impl SSTOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SSTOPW::_0 => 0,
            SSTOPW::_1 => 1,
            SSTOPW::_10 => 2,
            SSTOPW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _SSTOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSTOPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Stop bit disabled for transmitter/receiver/match store"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSTOPW::_0)
    }
    #[doc = "Reserved for transmitter/receiver/match store"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSTOPW::_1)
    }
    #[doc = "Transmitter outputs stop bit value 0 on store, receiver/match store sets error flag if stop bit is not 0"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(SSTOPW::_10)
    }
    #[doc = "Transmitter outputs stop bit value 1 on store, receiver/match store sets error flag if stop bit is not 1"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(SSTOPW::_11)
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
#[doc = "Values that can be written to the field `INSRC`"]
pub enum INSRCW {
    #[doc = "Pin"]
    _0,
    #[doc = "Shifter N+1 Output"]
    _1,
}
impl INSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INSRCW::_0 => false,
            INSRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _INSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INSRCW::_0)
    }
    #[doc = "Shifter N+1 Output"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INSRCW::_1)
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
    #[doc = "Bits 0:1 - Shifter Start bit"]
    #[inline]
    pub fn sstart(&self) -> SSTARTR {
        SSTARTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Shifter Stop bit"]
    #[inline]
    pub fn sstop(&self) -> SSTOPR {
        SSTOPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 8 - Input Source"]
    #[inline]
    pub fn insrc(&self) -> INSRCR {
        INSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:1 - Shifter Start bit"]
    #[inline]
    pub fn sstart(&mut self) -> _SSTARTW {
        _SSTARTW { w: self }
    }
    #[doc = "Bits 4:5 - Shifter Stop bit"]
    #[inline]
    pub fn sstop(&mut self) -> _SSTOPW {
        _SSTOPW { w: self }
    }
    #[doc = "Bit 8 - Input Source"]
    #[inline]
    pub fn insrc(&mut self) -> _INSRCW {
        _INSRCW { w: self }
    }
}
