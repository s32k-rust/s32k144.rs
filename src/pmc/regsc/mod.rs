#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::REGSC {
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
#[doc = "Possible values of the field `BIASEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIASENR {
    #[doc = "Biasing disabled, core logic can run in full performance"]
    _0,
    #[doc = "Biasing enabled, core logic is slower and there are restrictions in allowed system clock speed (see Data Sheet for details)"]
    _1,
}
impl BIASENR {
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
            BIASENR::_0 => false,
            BIASENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BIASENR {
        match value {
            false => BIASENR::_0,
            true => BIASENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BIASENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BIASENR::_1
    }
}
#[doc = "Possible values of the field `CLKBIASDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKBIASDISR {
    #[doc = "No effect"]
    _0,
    #[doc = "In VLPS mode, the bias currents and reference voltages for the following clock modules are disabled: SIRC, FIRC, PLL. (if available on device)"]
    _1,
}
impl CLKBIASDISR {
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
            CLKBIASDISR::_0 => false,
            CLKBIASDISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKBIASDISR {
        match value {
            false => CLKBIASDISR::_0,
            true => CLKBIASDISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CLKBIASDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CLKBIASDISR::_1
    }
}
#[doc = "Possible values of the field `REGFPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REGFPMR {
    #[doc = "Regulator is in low power mode or transition to/from"]
    _0,
    #[doc = "Regulator is in full performance mode"]
    _1,
}
impl REGFPMR {
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
            REGFPMR::_0 => false,
            REGFPMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REGFPMR {
        match value {
            false => REGFPMR::_0,
            true => REGFPMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == REGFPMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == REGFPMR::_1
    }
}
#[doc = "Possible values of the field `LPOSTAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOSTATR {
    #[doc = "Low power oscillator in low phase"]
    _0,
    #[doc = "Low power oscillator in high phase"]
    _1,
}
impl LPOSTATR {
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
            LPOSTATR::_0 => false,
            LPOSTATR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPOSTATR {
        match value {
            false => LPOSTATR::_0,
            true => LPOSTATR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LPOSTATR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LPOSTATR::_1
    }
}
#[doc = "Possible values of the field `LPODIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPODISR {
    #[doc = "Low power oscillator enabled"]
    _0,
    #[doc = "Low power oscillator disabled"]
    _1,
}
impl LPODISR {
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
            LPODISR::_0 => false,
            LPODISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPODISR {
        match value {
            false => LPODISR::_0,
            true => LPODISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LPODISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LPODISR::_1
    }
}
#[doc = "Values that can be written to the field `BIASEN`"]
pub enum BIASENW {
    #[doc = "Biasing disabled, core logic can run in full performance"]
    _0,
    #[doc = "Biasing enabled, core logic is slower and there are restrictions in allowed system clock speed (see Data Sheet for details)"]
    _1,
}
impl BIASENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BIASENW::_0 => false,
            BIASENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BIASENW<'a> {
    w: &'a mut W,
}
impl<'a> _BIASENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BIASENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Biasing disabled, core logic can run in full performance"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BIASENW::_0)
    }
    #[doc = "Biasing enabled, core logic is slower and there are restrictions in allowed system clock speed (see Data Sheet for details)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BIASENW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLKBIASDIS`"]
pub enum CLKBIASDISW {
    #[doc = "No effect"]
    _0,
    #[doc = "In VLPS mode, the bias currents and reference voltages for the following clock modules are disabled: SIRC, FIRC, PLL. (if available on device)"]
    _1,
}
impl CLKBIASDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKBIASDISW::_0 => false,
            CLKBIASDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKBIASDISW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKBIASDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKBIASDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKBIASDISW::_0)
    }
    #[doc = "In VLPS mode, the bias currents and reference voltages for the following clock modules are disabled: SIRC, FIRC, PLL. (if available on device)"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKBIASDISW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LPODIS`"]
pub enum LPODISW {
    #[doc = "Low power oscillator enabled"]
    _0,
    #[doc = "Low power oscillator disabled"]
    _1,
}
impl LPODISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPODISW::_0 => false,
            LPODISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPODISW<'a> {
    w: &'a mut W,
}
impl<'a> _LPODISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPODISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Low power oscillator enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPODISW::_0)
    }
    #[doc = "Low power oscillator disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPODISW::_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Bias Enable Bit"]
    #[inline]
    pub fn biasen(&self) -> BIASENR {
        BIASENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 1 - Clock Bias Disable Bit"]
    #[inline]
    pub fn clkbiasdis(&self) -> CLKBIASDISR {
        CLKBIASDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 2 - Regulator in Full Performance Mode Status Bit"]
    #[inline]
    pub fn regfpm(&self) -> REGFPMR {
        REGFPMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - LPO Status Bit"]
    #[inline]
    pub fn lpostat(&self) -> LPOSTATR {
        LPOSTATR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - LPO Disable Bit"]
    #[inline]
    pub fn lpodis(&self) -> LPODISR {
        LPODISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Bias Enable Bit"]
    #[inline]
    pub fn biasen(&mut self) -> _BIASENW {
        _BIASENW { w: self }
    }
    #[doc = "Bit 1 - Clock Bias Disable Bit"]
    #[inline]
    pub fn clkbiasdis(&mut self) -> _CLKBIASDISW {
        _CLKBIASDISW { w: self }
    }
    #[doc = "Bit 7 - LPO Disable Bit"]
    #[inline]
    pub fn lpodis(&mut self) -> _LPODISW {
        _LPODISW { w: self }
    }
}
