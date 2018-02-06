#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SRIE {
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
#[doc = "Possible values of the field `DELAY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DELAYR {
    #[doc = "10 LPO cycles"]
    _00,
    #[doc = "34 LPO cycles"]
    _01,
    #[doc = "130 LPO cycles"]
    _10,
    #[doc = "514 LPO cycles"]
    _11,
}
impl DELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DELAYR::_00 => 0,
            DELAYR::_01 => 1,
            DELAYR::_10 => 2,
            DELAYR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DELAYR {
        match value {
            0 => DELAYR::_00,
            1 => DELAYR::_01,
            2 => DELAYR::_10,
            3 => DELAYR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == DELAYR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == DELAYR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == DELAYR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == DELAYR::_11
    }
}
#[doc = "Possible values of the field `LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCR {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl LOCR {
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
            LOCR::_0 => false,
            LOCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCR {
        match value {
            false => LOCR::_0,
            true => LOCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOCR::_1
    }
}
#[doc = "Possible values of the field `LOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOLR {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl LOLR {
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
            LOLR::_0 => false,
            LOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOLR {
        match value {
            false => LOLR::_0,
            true => LOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOLR::_1
    }
}
#[doc = "Possible values of the field `WDOG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDOGR {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl WDOGR {
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
            WDOGR::_0 => false,
            WDOGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WDOGR {
        match value {
            false => WDOGR::_0,
            true => WDOGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WDOGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WDOGR::_1
    }
}
#[doc = "Possible values of the field `PIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PINR {
    #[doc = "Reset not caused by external reset pin"]
    _0,
    #[doc = "Reset caused by external reset pin"]
    _1,
}
impl PINR {
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
            PINR::_0 => false,
            PINR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PINR {
        match value {
            false => PINR::_0,
            true => PINR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PINR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PINR::_1
    }
}
#[doc = "Possible values of the field `GIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GIER {
    #[doc = "All interrupt sources disabled."]
    _0,
    #[doc = "All interrupt sources enabled. Note that the individual interrupt-enable bits still need to be set to generate interrupts."]
    _1,
}
impl GIER {
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
            GIER::_0 => false,
            GIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GIER {
        match value {
            false => GIER::_0,
            true => GIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GIER::_1
    }
}
#[doc = "Possible values of the field `JTAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JTAGR {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl JTAGR {
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
            JTAGR::_0 => false,
            JTAGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> JTAGR {
        match value {
            false => JTAGR::_0,
            true => JTAGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == JTAGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == JTAGR::_1
    }
}
#[doc = "Possible values of the field `LOCKUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKUPR {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl LOCKUPR {
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
            LOCKUPR::_0 => false,
            LOCKUPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOCKUPR {
        match value {
            false => LOCKUPR::_0,
            true => LOCKUPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOCKUPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOCKUPR::_1
    }
}
#[doc = "Possible values of the field `SW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWR {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl SWR {
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
            SWR::_0 => false,
            SWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWR {
        match value {
            false => SWR::_0,
            true => SWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWR::_1
    }
}
#[doc = "Possible values of the field `MDM_AP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDM_APR {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl MDM_APR {
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
            MDM_APR::_0 => false,
            MDM_APR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MDM_APR {
        match value {
            false => MDM_APR::_0,
            true => MDM_APR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MDM_APR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MDM_APR::_1
    }
}
#[doc = "Possible values of the field `SACKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SACKERRR {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl SACKERRR {
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
            SACKERRR::_0 => false,
            SACKERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SACKERRR {
        match value {
            false => SACKERRR::_0,
            true => SACKERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SACKERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SACKERRR::_1
    }
}
#[doc = "Values that can be written to the field `DELAY`"]
pub enum DELAYW {
    #[doc = "10 LPO cycles"]
    _00,
    #[doc = "34 LPO cycles"]
    _01,
    #[doc = "130 LPO cycles"]
    _10,
    #[doc = "514 LPO cycles"]
    _11,
}
impl DELAYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DELAYW::_00 => 0,
            DELAYW::_01 => 1,
            DELAYW::_10 => 2,
            DELAYW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _DELAYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DELAYW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "10 LPO cycles"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(DELAYW::_00)
    }
    #[doc = "34 LPO cycles"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(DELAYW::_01)
    }
    #[doc = "130 LPO cycles"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(DELAYW::_10)
    }
    #[doc = "514 LPO cycles"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(DELAYW::_11)
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
#[doc = "Values that can be written to the field `LOC`"]
pub enum LOCW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl LOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCW::_0 => false,
            LOCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCW::_1)
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
#[doc = "Values that can be written to the field `LOL`"]
pub enum LOLW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl LOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOLW::_0 => false,
            LOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOLW<'a> {
    w: &'a mut W,
}
impl<'a> _LOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOLW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOLW::_1)
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
#[doc = "Values that can be written to the field `WDOG`"]
pub enum WDOGW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl WDOGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WDOGW::_0 => false,
            WDOGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WDOGW<'a> {
    w: &'a mut W,
}
impl<'a> _WDOGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WDOGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WDOGW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WDOGW::_1)
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
#[doc = "Values that can be written to the field `PIN`"]
pub enum PINW {
    #[doc = "Reset not caused by external reset pin"]
    _0,
    #[doc = "Reset caused by external reset pin"]
    _1,
}
impl PINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PINW::_0 => false,
            PINW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PINW<'a> {
    w: &'a mut W,
}
impl<'a> _PINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PINW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by external reset pin"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PINW::_0)
    }
    #[doc = "Reset caused by external reset pin"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PINW::_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `GIE`"]
pub enum GIEW {
    #[doc = "All interrupt sources disabled."]
    _0,
    #[doc = "All interrupt sources enabled. Note that the individual interrupt-enable bits still need to be set to generate interrupts."]
    _1,
}
impl GIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GIEW::_0 => false,
            GIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GIEW<'a> {
    w: &'a mut W,
}
impl<'a> _GIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "All interrupt sources disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GIEW::_0)
    }
    #[doc = "All interrupt sources enabled. Note that the individual interrupt-enable bits still need to be set to generate interrupts."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GIEW::_1)
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
#[doc = "Values that can be written to the field `JTAG`"]
pub enum JTAGW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl JTAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            JTAGW::_0 => false,
            JTAGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _JTAGW<'a> {
    w: &'a mut W,
}
impl<'a> _JTAGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: JTAGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(JTAGW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(JTAGW::_1)
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
#[doc = "Values that can be written to the field `LOCKUP`"]
pub enum LOCKUPW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl LOCKUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOCKUPW::_0 => false,
            LOCKUPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOCKUPW<'a> {
    w: &'a mut W,
}
impl<'a> _LOCKUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOCKUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCKUPW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCKUPW::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SW`"]
pub enum SWW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl SWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWW::_0 => false,
            SWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWW<'a> {
    w: &'a mut W,
}
impl<'a> _SWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWW::_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MDM_AP`"]
pub enum MDM_APW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl MDM_APW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MDM_APW::_0 => false,
            MDM_APW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDM_APW<'a> {
    w: &'a mut W,
}
impl<'a> _MDM_APW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDM_APW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MDM_APW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MDM_APW::_1)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SACKERR`"]
pub enum SACKERRW {
    #[doc = "Interrupt disabled."]
    _0,
    #[doc = "Interrupt enabled."]
    _1,
}
impl SACKERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SACKERRW::_0 => false,
            SACKERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SACKERRW<'a> {
    w: &'a mut W,
}
impl<'a> _SACKERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SACKERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SACKERRW::_0)
    }
    #[doc = "Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SACKERRW::_1)
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
        const OFFSET: u8 = 13;
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
    #[doc = "Bits 0:1 - Reset Delay Time"]
    #[inline]
    pub fn delay(&self) -> DELAYR {
        DELAYR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Loss-of-Clock Interrupt"]
    #[inline]
    pub fn loc(&self) -> LOCR {
        LOCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Loss-of-Lock Interrupt"]
    #[inline]
    pub fn lol(&self) -> LOLR {
        LOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Watchdog Interrupt"]
    #[inline]
    pub fn wdog(&self) -> WDOGR {
        WDOGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - External Reset Pin Interrupt"]
    #[inline]
    pub fn pin(&self) -> PINR {
        PINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Global Interrupt Enable"]
    #[inline]
    pub fn gie(&self) -> GIER {
        GIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - JTAG generated reset"]
    #[inline]
    pub fn jtag(&self) -> JTAGR {
        JTAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Core Lockup Interrupt"]
    #[inline]
    pub fn lockup(&self) -> LOCKUPR {
        LOCKUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Software Interrupt"]
    #[inline]
    pub fn sw(&self) -> SWR {
        SWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - MDM-AP System Reset Request"]
    #[inline]
    pub fn mdm_ap(&self) -> MDM_APR {
        MDM_APR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Stop Acknowledge Error Interrupt"]
    #[inline]
    pub fn sackerr(&self) -> SACKERRR {
        SACKERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
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
    #[doc = "Bits 0:1 - Reset Delay Time"]
    #[inline]
    pub fn delay(&mut self) -> _DELAYW {
        _DELAYW { w: self }
    }
    #[doc = "Bit 2 - Loss-of-Clock Interrupt"]
    #[inline]
    pub fn loc(&mut self) -> _LOCW {
        _LOCW { w: self }
    }
    #[doc = "Bit 3 - Loss-of-Lock Interrupt"]
    #[inline]
    pub fn lol(&mut self) -> _LOLW {
        _LOLW { w: self }
    }
    #[doc = "Bit 5 - Watchdog Interrupt"]
    #[inline]
    pub fn wdog(&mut self) -> _WDOGW {
        _WDOGW { w: self }
    }
    #[doc = "Bit 6 - External Reset Pin Interrupt"]
    #[inline]
    pub fn pin(&mut self) -> _PINW {
        _PINW { w: self }
    }
    #[doc = "Bit 7 - Global Interrupt Enable"]
    #[inline]
    pub fn gie(&mut self) -> _GIEW {
        _GIEW { w: self }
    }
    #[doc = "Bit 8 - JTAG generated reset"]
    #[inline]
    pub fn jtag(&mut self) -> _JTAGW {
        _JTAGW { w: self }
    }
    #[doc = "Bit 9 - Core Lockup Interrupt"]
    #[inline]
    pub fn lockup(&mut self) -> _LOCKUPW {
        _LOCKUPW { w: self }
    }
    #[doc = "Bit 10 - Software Interrupt"]
    #[inline]
    pub fn sw(&mut self) -> _SWW {
        _SWW { w: self }
    }
    #[doc = "Bit 11 - MDM-AP System Reset Request"]
    #[inline]
    pub fn mdm_ap(&mut self) -> _MDM_APW {
        _MDM_APW { w: self }
    }
    #[doc = "Bit 13 - Stop Acknowledge Error Interrupt"]
    #[inline]
    pub fn sackerr(&mut self) -> _SACKERRW {
        _SACKERRW { w: self }
    }
}
