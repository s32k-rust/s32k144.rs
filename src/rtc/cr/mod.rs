#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR {
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
#[doc = "Possible values of the field `SWR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRR {
    #[doc = "No effect."] _0,
    #[doc = r" Reserved"] _Reserved(bool),
}
impl SWRR {
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
            SWRR::_0 => false,
            SWRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWRR {
        match value {
            false => SWRR::_0,
            i => SWRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWRR::_0
    }
}
#[doc = "Possible values of the field `SUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUPR {
    #[doc = "Non-supervisor mode write accesses are not supported and generate a bus error."] _0,
    #[doc = "Non-supervisor mode write accesses are supported."] _1,
}
impl SUPR {
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
            SUPR::_0 => false,
            SUPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SUPR {
        match value {
            false => SUPR::_0,
            true => SUPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SUPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SUPR::_1
    }
}
#[doc = "Possible values of the field `UM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UMR {
    #[doc = "Registers cannot be written when locked."] _0,
    #[doc = "Registers can be written when locked under limited conditions."] _1,
}
impl UMR {
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
            UMR::_0 => false,
            UMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UMR {
        match value {
            false => UMR::_0,
            true => UMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == UMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == UMR::_1
    }
}
#[doc = "Possible values of the field `CPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPSR {
    #[doc = "The prescaler output clock (as configured by TSIC) is output on RTC_CLKOUT."] _0,
    #[doc = "The RTC 32kHz crystal clock is output on RTC_CLKOUT, provided it is output to other peripherals."]
    _1,
}
impl CPSR {
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
            CPSR::_0 => false,
            CPSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPSR {
        match value {
            false => CPSR::_0,
            true => CPSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPSR::_1
    }
}
#[doc = "Possible values of the field `LPOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPOSR {
    #[doc = "RTC prescaler increments using 32kHz crystal."] _0,
    #[doc = "RTC prescaler increments using 1kHz LPO, bits [4:0] of the prescaler are ignored."] _1,
}
impl LPOSR {
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
            LPOSR::_0 => false,
            LPOSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPOSR {
        match value {
            false => LPOSR::_0,
            true => LPOSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LPOSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LPOSR::_1
    }
}
#[doc = "Possible values of the field `CPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPER {
    #[doc = "Disable RTC_CLKOUT pin."] _0,
    #[doc = "Enable RTC_CLKOUT pin."] _1,
}
impl CPER {
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
            CPER::_0 => false,
            CPER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPER {
        match value {
            false => CPER::_0,
            true => CPER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPER::_1
    }
}
#[doc = "Values that can be written to the field `SWR`"]
pub enum SWRW {
    #[doc = "No effect."] _0,
}
impl SWRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWRW::_0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWRW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWRW::_0)
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
#[doc = "Values that can be written to the field `SUP`"]
pub enum SUPW {
    #[doc = "Non-supervisor mode write accesses are not supported and generate a bus error."] _0,
    #[doc = "Non-supervisor mode write accesses are supported."] _1,
}
impl SUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SUPW::_0 => false,
            SUPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SUPW<'a> {
    w: &'a mut W,
}
impl<'a> _SUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Non-supervisor mode write accesses are not supported and generate a bus error."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUPW::_0)
    }
    #[doc = "Non-supervisor mode write accesses are supported."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUPW::_1)
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
#[doc = "Values that can be written to the field `UM`"]
pub enum UMW {
    #[doc = "Registers cannot be written when locked."] _0,
    #[doc = "Registers can be written when locked under limited conditions."] _1,
}
impl UMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UMW::_0 => false,
            UMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UMW<'a> {
    w: &'a mut W,
}
impl<'a> _UMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Registers cannot be written when locked."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(UMW::_0)
    }
    #[doc = "Registers can be written when locked under limited conditions."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(UMW::_1)
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
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CPS`"]
pub enum CPSW {
    #[doc = "The prescaler output clock (as configured by TSIC) is output on RTC_CLKOUT."] _0,
    #[doc = "The RTC 32kHz crystal clock is output on RTC_CLKOUT, provided it is output to other peripherals."]
    _1,
}
impl CPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPSW::_0 => false,
            CPSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPSW<'a> {
    w: &'a mut W,
}
impl<'a> _CPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The prescaler output clock (as configured by TSIC) is output on RTC_CLKOUT."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPSW::_0)
    }
    #[doc = "The RTC 32kHz crystal clock is output on RTC_CLKOUT, provided it is output to other peripherals."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPSW::_1)
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
#[doc = "Values that can be written to the field `LPOS`"]
pub enum LPOSW {
    #[doc = "RTC prescaler increments using 32kHz crystal."] _0,
    #[doc = "RTC prescaler increments using 1kHz LPO, bits [4:0] of the prescaler are ignored."] _1,
}
impl LPOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPOSW::_0 => false,
            LPOSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPOSW<'a> {
    w: &'a mut W,
}
impl<'a> _LPOSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPOSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RTC prescaler increments using 32kHz crystal."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPOSW::_0)
    }
    #[doc = "RTC prescaler increments using 1kHz LPO, bits [4:0] of the prescaler are ignored."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPOSW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CPE`"]
pub enum CPEW {
    #[doc = "Disable RTC_CLKOUT pin."] _0,
    #[doc = "Enable RTC_CLKOUT pin."] _1,
}
impl CPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPEW::_0 => false,
            CPEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPEW<'a> {
    w: &'a mut W,
}
impl<'a> _CPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable RTC_CLKOUT pin."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPEW::_0)
    }
    #[doc = "Enable RTC_CLKOUT pin."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPEW::_1)
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
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Software Reset"]
    #[inline]
    pub fn swr(&self) -> SWRR {
        SWRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Supervisor Access"]
    #[inline]
    pub fn sup(&self) -> SUPR {
        SUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Update Mode"]
    #[inline]
    pub fn um(&self) -> UMR {
        UMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Clock Pin Select"]
    #[inline]
    pub fn cps(&self) -> CPSR {
        CPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - LPO Select"]
    #[inline]
    pub fn lpos(&self) -> LPOSR {
        LPOSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Clock Pin Enable"]
    #[inline]
    pub fn cpe(&self) -> CPER {
        CPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Software Reset"]
    #[inline]
    pub fn swr(&mut self) -> _SWRW {
        _SWRW { w: self }
    }
    #[doc = "Bit 2 - Supervisor Access"]
    #[inline]
    pub fn sup(&mut self) -> _SUPW {
        _SUPW { w: self }
    }
    #[doc = "Bit 3 - Update Mode"]
    #[inline]
    pub fn um(&mut self) -> _UMW {
        _UMW { w: self }
    }
    #[doc = "Bit 5 - Clock Pin Select"]
    #[inline]
    pub fn cps(&mut self) -> _CPSW {
        _CPSW { w: self }
    }
    #[doc = "Bit 7 - LPO Select"]
    #[inline]
    pub fn lpos(&mut self) -> _LPOSW {
        _LPOSW { w: self }
    }
    #[doc = "Bit 24 - Clock Pin Enable"]
    #[inline]
    pub fn cpe(&mut self) -> _CPEW {
        _CPEW { w: self }
    }
}
