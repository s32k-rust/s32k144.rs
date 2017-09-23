#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCR {
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
#[doc = "Possible values of the field `SEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SENR {
    #[doc = "Slave mode is disabled."] _0,
    #[doc = "Slave mode is enabled."] _1,
}
impl SENR {
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
            SENR::_0 => false,
            SENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SENR {
        match value {
            false => SENR::_0,
            true => SENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SENR::_1
    }
}
#[doc = "Possible values of the field `RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTR {
    #[doc = "Slave logic is not reset."] _0,
    #[doc = "Slave logic is reset."] _1,
}
impl RSTR {
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
            RSTR::_0 => false,
            RSTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTR {
        match value {
            false => RSTR::_0,
            true => RSTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RSTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RSTR::_1
    }
}
#[doc = "Possible values of the field `FILTEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTENR {
    #[doc = "Disable digital filter and output delay counter for slave mode."] _0,
    #[doc = "Enable digital filter and output delay counter for slave mode."] _1,
}
impl FILTENR {
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
            FILTENR::_0 => false,
            FILTENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FILTENR {
        match value {
            false => FILTENR::_0,
            true => FILTENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FILTENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FILTENR::_1
    }
}
#[doc = "Possible values of the field `FILTDZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTDZR {
    #[doc = "Filter remains enabled in Doze mode."] _0,
    #[doc = "Filter is disabled in Doze mode."] _1,
}
impl FILTDZR {
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
            FILTDZR::_0 => false,
            FILTDZR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FILTDZR {
        match value {
            false => FILTDZR::_0,
            true => FILTDZR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FILTDZR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FILTDZR::_1
    }
}
#[doc = "Values that can be written to the field `SEN`"]
pub enum SENW {
    #[doc = "Slave mode is disabled."] _0,
    #[doc = "Slave mode is enabled."] _1,
}
impl SENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SENW::_0 => false,
            SENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SENW<'a> {
    w: &'a mut W,
}
impl<'a> _SENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave mode is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SENW::_0)
    }
    #[doc = "Slave mode is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SENW::_1)
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
#[doc = "Values that can be written to the field `RST`"]
pub enum RSTW {
    #[doc = "Slave logic is not reset."] _0,
    #[doc = "Slave logic is reset."] _1,
}
impl RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTW::_0 => false,
            RSTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave logic is not reset."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTW::_0)
    }
    #[doc = "Slave logic is reset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTW::_1)
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
#[doc = "Values that can be written to the field `FILTEN`"]
pub enum FILTENW {
    #[doc = "Disable digital filter and output delay counter for slave mode."] _0,
    #[doc = "Enable digital filter and output delay counter for slave mode."] _1,
}
impl FILTENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FILTENW::_0 => false,
            FILTENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTENW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable digital filter and output delay counter for slave mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FILTENW::_0)
    }
    #[doc = "Enable digital filter and output delay counter for slave mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FILTENW::_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FILTDZ`"]
pub enum FILTDZW {
    #[doc = "Filter remains enabled in Doze mode."] _0,
    #[doc = "Filter is disabled in Doze mode."] _1,
}
impl FILTDZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FILTDZW::_0 => false,
            FILTDZW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FILTDZW<'a> {
    w: &'a mut W,
}
impl<'a> _FILTDZW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FILTDZW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Filter remains enabled in Doze mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FILTDZW::_0)
    }
    #[doc = "Filter is disabled in Doze mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FILTDZW::_1)
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
    #[doc = "Bit 0 - Slave Enable"]
    #[inline]
    pub fn sen(&self) -> SENR {
        SENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline]
    pub fn rst(&self) -> RSTR {
        RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Filter Enable"]
    #[inline]
    pub fn filten(&self) -> FILTENR {
        FILTENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Filter Doze Enable"]
    #[inline]
    pub fn filtdz(&self) -> FILTDZR {
        FILTDZR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
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
    #[doc = "Bit 0 - Slave Enable"]
    #[inline]
    pub fn sen(&mut self) -> _SENW {
        _SENW { w: self }
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline]
    pub fn rst(&mut self) -> _RSTW {
        _RSTW { w: self }
    }
    #[doc = "Bit 4 - Filter Enable"]
    #[inline]
    pub fn filten(&mut self) -> _FILTENW {
        _FILTENW { w: self }
    }
    #[doc = "Bit 5 - Filter Doze Enable"]
    #[inline]
    pub fn filtdz(&mut self) -> _FILTDZW {
        _FILTDZW { w: self }
    }
}
