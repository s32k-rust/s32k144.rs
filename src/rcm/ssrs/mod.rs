#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SSRS {
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
#[doc = "Possible values of the field `SLVD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLVDR {
    #[doc = "Reset not caused by LVD trip or POR"]
    _0,
    #[doc = "Reset caused by LVD trip or POR"]
    _1,
}
impl SLVDR {
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
            SLVDR::_0 => false,
            SLVDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLVDR {
        match value {
            false => SLVDR::_0,
            true => SLVDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SLVDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SLVDR::_1
    }
}
#[doc = "Possible values of the field `SLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOCR {
    #[doc = "Reset not caused by a loss of external clock."]
    _0,
    #[doc = "Reset caused by a loss of external clock."]
    _1,
}
impl SLOCR {
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
            SLOCR::_0 => false,
            SLOCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLOCR {
        match value {
            false => SLOCR::_0,
            true => SLOCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SLOCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SLOCR::_1
    }
}
#[doc = "Possible values of the field `SLOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOLR {
    #[doc = "Reset not caused by a loss of lock in the PLL/FLL"]
    _0,
    #[doc = "Reset caused by a loss of lock in the PLL/FLL"]
    _1,
}
impl SLOLR {
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
            SLOLR::_0 => false,
            SLOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLOLR {
        match value {
            false => SLOLR::_0,
            true => SLOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SLOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SLOLR::_1
    }
}
#[doc = "Possible values of the field `SWDOG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWDOGR {
    #[doc = "Reset not caused by watchdog timeout"]
    _0,
    #[doc = "Reset caused by watchdog timeout"]
    _1,
}
impl SWDOGR {
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
            SWDOGR::_0 => false,
            SWDOGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWDOGR {
        match value {
            false => SWDOGR::_0,
            true => SWDOGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWDOGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWDOGR::_1
    }
}
#[doc = "Possible values of the field `SPIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPINR {
    #[doc = "Reset not caused by external reset pin"]
    _0,
    #[doc = "Reset caused by external reset pin"]
    _1,
}
impl SPINR {
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
            SPINR::_0 => false,
            SPINR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPINR {
        match value {
            false => SPINR::_0,
            true => SPINR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPINR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPINR::_1
    }
}
#[doc = "Possible values of the field `SPOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPORR {
    #[doc = "Reset not caused by POR"]
    _0,
    #[doc = "Reset caused by POR"]
    _1,
}
impl SPORR {
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
            SPORR::_0 => false,
            SPORR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPORR {
        match value {
            false => SPORR::_0,
            true => SPORR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPORR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPORR::_1
    }
}
#[doc = "Possible values of the field `SJTAG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SJTAGR {
    #[doc = "Reset not caused by JTAG"]
    _0,
    #[doc = "Reset caused by JTAG"]
    _1,
}
impl SJTAGR {
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
            SJTAGR::_0 => false,
            SJTAGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SJTAGR {
        match value {
            false => SJTAGR::_0,
            true => SJTAGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SJTAGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SJTAGR::_1
    }
}
#[doc = "Possible values of the field `SLOCKUP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOCKUPR {
    #[doc = "Reset not caused by core LOCKUP event"]
    _0,
    #[doc = "Reset caused by core LOCKUP event"]
    _1,
}
impl SLOCKUPR {
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
            SLOCKUPR::_0 => false,
            SLOCKUPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SLOCKUPR {
        match value {
            false => SLOCKUPR::_0,
            true => SLOCKUPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SLOCKUPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SLOCKUPR::_1
    }
}
#[doc = "Possible values of the field `SSW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSWR {
    #[doc = "Reset not caused by software setting of SYSRESETREQ bit"]
    _0,
    #[doc = "Reset caused by software setting of SYSRESETREQ bit"]
    _1,
}
impl SSWR {
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
            SSWR::_0 => false,
            SSWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSWR {
        match value {
            false => SSWR::_0,
            true => SSWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SSWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SSWR::_1
    }
}
#[doc = "Possible values of the field `SMDM_AP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMDM_APR {
    #[doc = "Reset was not caused by host debugger system setting of the System Reset Request bit"]
    _0,
    #[doc = "Reset was caused by host debugger system setting of the System Reset Request bit"]
    _1,
}
impl SMDM_APR {
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
            SMDM_APR::_0 => false,
            SMDM_APR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMDM_APR {
        match value {
            false => SMDM_APR::_0,
            true => SMDM_APR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SMDM_APR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SMDM_APR::_1
    }
}
#[doc = "Possible values of the field `SSACKERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSACKERRR {
    #[doc = "Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _0,
    #[doc = "Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _1,
}
impl SSACKERRR {
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
            SSACKERRR::_0 => false,
            SSACKERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SSACKERRR {
        match value {
            false => SSACKERRR::_0,
            true => SSACKERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SSACKERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SSACKERRR::_1
    }
}
#[doc = "Values that can be written to the field `SLVD`"]
pub enum SLVDW {
    #[doc = "Reset not caused by LVD trip or POR"]
    _0,
    #[doc = "Reset caused by LVD trip or POR"]
    _1,
}
impl SLVDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLVDW::_0 => false,
            SLVDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLVDW<'a> {
    w: &'a mut W,
}
impl<'a> _SLVDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLVDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by LVD trip or POR"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVDW::_0)
    }
    #[doc = "Reset caused by LVD trip or POR"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVDW::_1)
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
#[doc = "Values that can be written to the field `SLOC`"]
pub enum SLOCW {
    #[doc = "Reset not caused by a loss of external clock."]
    _0,
    #[doc = "Reset caused by a loss of external clock."]
    _1,
}
impl SLOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLOCW::_0 => false,
            SLOCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLOCW<'a> {
    w: &'a mut W,
}
impl<'a> _SLOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLOCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by a loss of external clock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLOCW::_0)
    }
    #[doc = "Reset caused by a loss of external clock."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLOCW::_1)
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
#[doc = "Values that can be written to the field `SLOL`"]
pub enum SLOLW {
    #[doc = "Reset not caused by a loss of lock in the PLL/FLL"]
    _0,
    #[doc = "Reset caused by a loss of lock in the PLL/FLL"]
    _1,
}
impl SLOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLOLW::_0 => false,
            SLOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLOLW<'a> {
    w: &'a mut W,
}
impl<'a> _SLOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by a loss of lock in the PLL/FLL"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLOLW::_0)
    }
    #[doc = "Reset caused by a loss of lock in the PLL/FLL"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLOLW::_1)
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
#[doc = "Values that can be written to the field `SWDOG`"]
pub enum SWDOGW {
    #[doc = "Reset not caused by watchdog timeout"]
    _0,
    #[doc = "Reset caused by watchdog timeout"]
    _1,
}
impl SWDOGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWDOGW::_0 => false,
            SWDOGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWDOGW<'a> {
    w: &'a mut W,
}
impl<'a> _SWDOGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWDOGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by watchdog timeout"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWDOGW::_0)
    }
    #[doc = "Reset caused by watchdog timeout"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWDOGW::_1)
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
#[doc = "Values that can be written to the field `SPIN`"]
pub enum SPINW {
    #[doc = "Reset not caused by external reset pin"]
    _0,
    #[doc = "Reset caused by external reset pin"]
    _1,
}
impl SPINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPINW::_0 => false,
            SPINW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPINW<'a> {
    w: &'a mut W,
}
impl<'a> _SPINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPINW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by external reset pin"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPINW::_0)
    }
    #[doc = "Reset caused by external reset pin"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPINW::_1)
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
#[doc = "Values that can be written to the field `SPOR`"]
pub enum SPORW {
    #[doc = "Reset not caused by POR"]
    _0,
    #[doc = "Reset caused by POR"]
    _1,
}
impl SPORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPORW::_0 => false,
            SPORW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPORW<'a> {
    w: &'a mut W,
}
impl<'a> _SPORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by POR"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPORW::_0)
    }
    #[doc = "Reset caused by POR"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPORW::_1)
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
#[doc = "Values that can be written to the field `SJTAG`"]
pub enum SJTAGW {
    #[doc = "Reset not caused by JTAG"]
    _0,
    #[doc = "Reset caused by JTAG"]
    _1,
}
impl SJTAGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SJTAGW::_0 => false,
            SJTAGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SJTAGW<'a> {
    w: &'a mut W,
}
impl<'a> _SJTAGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SJTAGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by JTAG"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SJTAGW::_0)
    }
    #[doc = "Reset caused by JTAG"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SJTAGW::_1)
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
#[doc = "Values that can be written to the field `SLOCKUP`"]
pub enum SLOCKUPW {
    #[doc = "Reset not caused by core LOCKUP event"]
    _0,
    #[doc = "Reset caused by core LOCKUP event"]
    _1,
}
impl SLOCKUPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SLOCKUPW::_0 => false,
            SLOCKUPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLOCKUPW<'a> {
    w: &'a mut W,
}
impl<'a> _SLOCKUPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLOCKUPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by core LOCKUP event"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLOCKUPW::_0)
    }
    #[doc = "Reset caused by core LOCKUP event"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLOCKUPW::_1)
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
#[doc = "Values that can be written to the field `SSW`"]
pub enum SSWW {
    #[doc = "Reset not caused by software setting of SYSRESETREQ bit"]
    _0,
    #[doc = "Reset caused by software setting of SYSRESETREQ bit"]
    _1,
}
impl SSWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSWW::_0 => false,
            SSWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSWW<'a> {
    w: &'a mut W,
}
impl<'a> _SSWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by software setting of SYSRESETREQ bit"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSWW::_0)
    }
    #[doc = "Reset caused by software setting of SYSRESETREQ bit"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSWW::_1)
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
#[doc = "Values that can be written to the field `SMDM_AP`"]
pub enum SMDM_APW {
    #[doc = "Reset was not caused by host debugger system setting of the System Reset Request bit"]
    _0,
    #[doc = "Reset was caused by host debugger system setting of the System Reset Request bit"]
    _1,
}
impl SMDM_APW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMDM_APW::_0 => false,
            SMDM_APW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMDM_APW<'a> {
    w: &'a mut W,
}
impl<'a> _SMDM_APW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMDM_APW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset was not caused by host debugger system setting of the System Reset Request bit"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMDM_APW::_0)
    }
    #[doc = "Reset was caused by host debugger system setting of the System Reset Request bit"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMDM_APW::_1)
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
#[doc = "Values that can be written to the field `SSACKERR`"]
pub enum SSACKERRW {
    #[doc = "Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _0,
    #[doc = "Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _1,
}
impl SSACKERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SSACKERRW::_0 => false,
            SSACKERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SSACKERRW<'a> {
    w: &'a mut W,
}
impl<'a> _SSACKERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SSACKERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SSACKERRW::_0)
    }
    #[doc = "Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SSACKERRW::_1)
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
    #[doc = "Bit 1 - Sticky Low-Voltage Detect Reset"]
    #[inline]
    pub fn slvd(&self) -> SLVDR {
        SLVDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Sticky Loss-of-Clock Reset"]
    #[inline]
    pub fn sloc(&self) -> SLOCR {
        SLOCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Sticky Loss-of-Lock Reset"]
    #[inline]
    pub fn slol(&self) -> SLOLR {
        SLOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Sticky Watchdog"]
    #[inline]
    pub fn swdog(&self) -> SWDOGR {
        SWDOGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Sticky External Reset Pin"]
    #[inline]
    pub fn spin(&self) -> SPINR {
        SPINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Sticky Power-On Reset"]
    #[inline]
    pub fn spor(&self) -> SPORR {
        SPORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Sticky JTAG generated reset"]
    #[inline]
    pub fn sjtag(&self) -> SJTAGR {
        SJTAGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Sticky Core Lockup"]
    #[inline]
    pub fn slockup(&self) -> SLOCKUPR {
        SLOCKUPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Sticky Software"]
    #[inline]
    pub fn ssw(&self) -> SSWR {
        SSWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Sticky MDM-AP System Reset Request"]
    #[inline]
    pub fn smdm_ap(&self) -> SMDM_APR {
        SMDM_APR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Sticky Stop Acknowledge Error"]
    #[inline]
    pub fn ssackerr(&self) -> SSACKERRR {
        SSACKERRR::_from({
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
        W { bits: 130 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - Sticky Low-Voltage Detect Reset"]
    #[inline]
    pub fn slvd(&mut self) -> _SLVDW {
        _SLVDW { w: self }
    }
    #[doc = "Bit 2 - Sticky Loss-of-Clock Reset"]
    #[inline]
    pub fn sloc(&mut self) -> _SLOCW {
        _SLOCW { w: self }
    }
    #[doc = "Bit 3 - Sticky Loss-of-Lock Reset"]
    #[inline]
    pub fn slol(&mut self) -> _SLOLW {
        _SLOLW { w: self }
    }
    #[doc = "Bit 5 - Sticky Watchdog"]
    #[inline]
    pub fn swdog(&mut self) -> _SWDOGW {
        _SWDOGW { w: self }
    }
    #[doc = "Bit 6 - Sticky External Reset Pin"]
    #[inline]
    pub fn spin(&mut self) -> _SPINW {
        _SPINW { w: self }
    }
    #[doc = "Bit 7 - Sticky Power-On Reset"]
    #[inline]
    pub fn spor(&mut self) -> _SPORW {
        _SPORW { w: self }
    }
    #[doc = "Bit 8 - Sticky JTAG generated reset"]
    #[inline]
    pub fn sjtag(&mut self) -> _SJTAGW {
        _SJTAGW { w: self }
    }
    #[doc = "Bit 9 - Sticky Core Lockup"]
    #[inline]
    pub fn slockup(&mut self) -> _SLOCKUPW {
        _SLOCKUPW { w: self }
    }
    #[doc = "Bit 10 - Sticky Software"]
    #[inline]
    pub fn ssw(&mut self) -> _SSWW {
        _SSWW { w: self }
    }
    #[doc = "Bit 11 - Sticky MDM-AP System Reset Request"]
    #[inline]
    pub fn smdm_ap(&mut self) -> _SMDM_APW {
        _SMDM_APW { w: self }
    }
    #[doc = "Bit 13 - Sticky Stop Acknowledge Error"]
    #[inline]
    pub fn ssackerr(&mut self) -> _SSACKERRW {
        _SSACKERRW { w: self }
    }
}
