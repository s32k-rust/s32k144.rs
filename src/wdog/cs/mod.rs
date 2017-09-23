#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CS {
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
#[doc = "Possible values of the field `STOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPR {
    #[doc = "Watchdog disabled in chip stop mode."]
    _0,
    #[doc = "Watchdog enabled in chip stop mode."]
    _1,
}
impl STOPR {
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
            STOPR::_0 => false,
            STOPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STOPR {
        match value {
            false => STOPR::_0,
            true => STOPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STOPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STOPR::_1
    }
}
#[doc = "Possible values of the field `WAIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAITR {
    #[doc = "Watchdog disabled in chip wait mode."]
    _0,
    #[doc = "Watchdog enabled in chip wait mode."]
    _1,
}
impl WAITR {
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
            WAITR::_0 => false,
            WAITR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAITR {
        match value {
            false => WAITR::_0,
            true => WAITR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WAITR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WAITR::_1
    }
}
#[doc = "Possible values of the field `DBG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGR {
    #[doc = "Watchdog disabled in chip debug mode."]
    _0,
    #[doc = "Watchdog enabled in chip debug mode."]
    _1,
}
impl DBGR {
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
            DBGR::_0 => false,
            DBGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBGR {
        match value {
            false => DBGR::_0,
            true => DBGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DBGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DBGR::_1
    }
}
#[doc = "Possible values of the field `TST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTR {
    #[doc = "Watchdog test mode disabled."]
    _00,
    #[doc = "Watchdog user mode enabled. (Watchdog test mode disabled.) After testing the watchdog, software should use this setting to indicate that the watchdog is functioning normally in user mode."]
    _01,
    #[doc = "Watchdog test mode enabled, only the low byte is used. CNT[CNTLOW] is compared with TOVAL[TOVALLOW]."]
    _10,
    #[doc = "Watchdog test mode enabled, only the high byte is used. CNT[CNTHIGH] is compared with TOVAL[TOVALHIGH]."]
    _11,
}
impl TSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSTR::_00 => 0,
            TSTR::_01 => 1,
            TSTR::_10 => 2,
            TSTR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSTR {
        match value {
            0 => TSTR::_00,
            1 => TSTR::_01,
            2 => TSTR::_10,
            3 => TSTR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TSTR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == TSTR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TSTR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TSTR::_11
    }
}
#[doc = "Possible values of the field `UPDATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UPDATER {
    #[doc = "Updates not allowed. After the initial configuration, the watchdog cannot be later modified without forcing a reset."]
    _0,
    #[doc = "Updates allowed. Software can modify the watchdog configuration registers within 8'd128 bus clocks after performing the unlock write sequence."]
    _1,
}
impl UPDATER {
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
            UPDATER::_0 => false,
            UPDATER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> UPDATER {
        match value {
            false => UPDATER::_0,
            true => UPDATER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == UPDATER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == UPDATER::_1
    }
}
#[doc = "Possible values of the field `INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTR {
    #[doc = "Watchdog interrupts are disabled. Watchdog resets are not delayed."]
    _0,
    #[doc = "Watchdog interrupts are enabled. Watchdog resets are delayed by 8'd128 bus clocks from the interrupt vector fetch."]
    _1,
}
impl INTR {
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
            INTR::_0 => false,
            INTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTR {
        match value {
            false => INTR::_0,
            true => INTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INTR::_1
    }
}
#[doc = "Possible values of the field `EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENR {
    #[doc = "Watchdog disabled."]
    _0,
    #[doc = "Watchdog enabled."]
    _1,
}
impl ENR {
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
            ENR::_0 => false,
            ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENR {
        match value {
            false => ENR::_0,
            true => ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ENR::_1
    }
}
#[doc = "Possible values of the field `CLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKR {
    #[doc = "Bus clock"]
    _00,
    #[doc = "LPO clock"]
    _01,
    #[doc = "INTCLK (internal clock)"]
    _10,
    #[doc = "ERCLK (external reference clock)"]
    _11,
}
impl CLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CLKR::_00 => 0,
            CLKR::_01 => 1,
            CLKR::_10 => 2,
            CLKR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CLKR {
        match value {
            0 => CLKR::_00,
            1 => CLKR::_01,
            2 => CLKR::_10,
            3 => CLKR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == CLKR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == CLKR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == CLKR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == CLKR::_11
    }
}
#[doc = "Possible values of the field `RCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RCSR {
    #[doc = "Reconfiguring WDOG."]
    _0,
    #[doc = "Reconfiguration is successful."]
    _1,
}
impl RCSR {
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
            RCSR::_0 => false,
            RCSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RCSR {
        match value {
            false => RCSR::_0,
            true => RCSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RCSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RCSR::_1
    }
}
#[doc = "Possible values of the field `ULK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULKR {
    #[doc = "WDOG is locked."]
    _0,
    #[doc = "WDOG is unlocked."]
    _1,
}
impl ULKR {
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
            ULKR::_0 => false,
            ULKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ULKR {
        match value {
            false => ULKR::_0,
            true => ULKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ULKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ULKR::_1
    }
}
#[doc = "Possible values of the field `PRES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESR {
    #[doc = "256 prescaler disabled."]
    _0,
    #[doc = "256 prescaler enabled."]
    _1,
}
impl PRESR {
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
            PRESR::_0 => false,
            PRESR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PRESR {
        match value {
            false => PRESR::_0,
            true => PRESR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PRESR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PRESR::_1
    }
}
#[doc = "Possible values of the field `CMD32EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD32ENR {
    #[doc = "Disables support for 32-bit refresh/unlock command write words. Only 16-bit or 8-bit is supported."]
    _0,
    #[doc = "Enables support for 32-bit refresh/unlock command write words. 16-bit or 8-bit is NOT supported."]
    _1,
}
impl CMD32ENR {
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
            CMD32ENR::_0 => false,
            CMD32ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CMD32ENR {
        match value {
            false => CMD32ENR::_0,
            true => CMD32ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CMD32ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CMD32ENR::_1
    }
}
#[doc = "Possible values of the field `FLG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLGR {
    #[doc = "No interrupt occurred."]
    _0,
    #[doc = "An interrupt occurred."]
    _1,
}
impl FLGR {
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
            FLGR::_0 => false,
            FLGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLGR {
        match value {
            false => FLGR::_0,
            true => FLGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FLGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FLGR::_1
    }
}
#[doc = "Possible values of the field `WIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WINR {
    #[doc = "Window mode disabled."]
    _0,
    #[doc = "Window mode enabled."]
    _1,
}
impl WINR {
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
            WINR::_0 => false,
            WINR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WINR {
        match value {
            false => WINR::_0,
            true => WINR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WINR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WINR::_1
    }
}
#[doc = "Values that can be written to the field `STOP`"]
pub enum STOPW {
    #[doc = "Watchdog disabled in chip stop mode."]
    _0,
    #[doc = "Watchdog enabled in chip stop mode."]
    _1,
}
impl STOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STOPW::_0 => false,
            STOPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _STOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STOPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Watchdog disabled in chip stop mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOPW::_0)
    }
    #[doc = "Watchdog enabled in chip stop mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOPW::_1)
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
#[doc = "Values that can be written to the field `WAIT`"]
pub enum WAITW {
    #[doc = "Watchdog disabled in chip wait mode."]
    _0,
    #[doc = "Watchdog enabled in chip wait mode."]
    _1,
}
impl WAITW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAITW::_0 => false,
            WAITW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAITW<'a> {
    w: &'a mut W,
}
impl<'a> _WAITW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAITW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Watchdog disabled in chip wait mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAITW::_0)
    }
    #[doc = "Watchdog enabled in chip wait mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAITW::_1)
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
#[doc = "Values that can be written to the field `DBG`"]
pub enum DBGW {
    #[doc = "Watchdog disabled in chip debug mode."]
    _0,
    #[doc = "Watchdog enabled in chip debug mode."]
    _1,
}
impl DBGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBGW::_0 => false,
            DBGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBGW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Watchdog disabled in chip debug mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGW::_0)
    }
    #[doc = "Watchdog enabled in chip debug mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGW::_1)
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
#[doc = "Values that can be written to the field `TST`"]
pub enum TSTW {
    #[doc = "Watchdog test mode disabled."]
    _00,
    #[doc = "Watchdog user mode enabled. (Watchdog test mode disabled.) After testing the watchdog, software should use this setting to indicate that the watchdog is functioning normally in user mode."]
    _01,
    #[doc = "Watchdog test mode enabled, only the low byte is used. CNT[CNTLOW] is compared with TOVAL[TOVALLOW]."]
    _10,
    #[doc = "Watchdog test mode enabled, only the high byte is used. CNT[CNTHIGH] is compared with TOVAL[TOVALHIGH]."]
    _11,
}
impl TSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSTW::_00 => 0,
            TSTW::_01 => 1,
            TSTW::_10 => 2,
            TSTW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Watchdog test mode disabled."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(TSTW::_00)
    }
    #[doc = "Watchdog user mode enabled. (Watchdog test mode disabled.) After testing the watchdog, software should use this setting to indicate that the watchdog is functioning normally in user mode."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(TSTW::_01)
    }
    #[doc = "Watchdog test mode enabled, only the low byte is used. CNT[CNTLOW] is compared with TOVAL[TOVALLOW]."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TSTW::_10)
    }
    #[doc = "Watchdog test mode enabled, only the high byte is used. CNT[CNTHIGH] is compared with TOVAL[TOVALHIGH]."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TSTW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `UPDATE`"]
pub enum UPDATEW {
    #[doc = "Updates not allowed. After the initial configuration, the watchdog cannot be later modified without forcing a reset."]
    _0,
    #[doc = "Updates allowed. Software can modify the watchdog configuration registers within 8'd128 bus clocks after performing the unlock write sequence."]
    _1,
}
impl UPDATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            UPDATEW::_0 => false,
            UPDATEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _UPDATEW<'a> {
    w: &'a mut W,
}
impl<'a> _UPDATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: UPDATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Updates not allowed. After the initial configuration, the watchdog cannot be later modified without forcing a reset."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(UPDATEW::_0)
    }
    #[doc = "Updates allowed. Software can modify the watchdog configuration registers within 8'd128 bus clocks after performing the unlock write sequence."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(UPDATEW::_1)
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
#[doc = "Values that can be written to the field `INT`"]
pub enum INTW {
    #[doc = "Watchdog interrupts are disabled. Watchdog resets are not delayed."]
    _0,
    #[doc = "Watchdog interrupts are enabled. Watchdog resets are delayed by 8'd128 bus clocks from the interrupt vector fetch."]
    _1,
}
impl INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTW::_0 => false,
            INTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTW<'a> {
    w: &'a mut W,
}
impl<'a> _INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Watchdog interrupts are disabled. Watchdog resets are not delayed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTW::_0)
    }
    #[doc = "Watchdog interrupts are enabled. Watchdog resets are delayed by 8'd128 bus clocks from the interrupt vector fetch."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTW::_1)
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
#[doc = "Values that can be written to the field `EN`"]
pub enum ENW {
    #[doc = "Watchdog disabled."]
    _0,
    #[doc = "Watchdog enabled."]
    _1,
}
impl ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENW::_0 => false,
            ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Watchdog disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENW::_0)
    }
    #[doc = "Watchdog enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENW::_1)
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
#[doc = "Values that can be written to the field `CLK`"]
pub enum CLKW {
    #[doc = "Bus clock"]
    _00,
    #[doc = "LPO clock"]
    _01,
    #[doc = "INTCLK (internal clock)"]
    _10,
    #[doc = "ERCLK (external reference clock)"]
    _11,
}
impl CLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CLKW::_00 => 0,
            CLKW::_01 => 1,
            CLKW::_10 => 2,
            CLKW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Bus clock"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(CLKW::_00)
    }
    #[doc = "LPO clock"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(CLKW::_01)
    }
    #[doc = "INTCLK (internal clock)"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(CLKW::_10)
    }
    #[doc = "ERCLK (external reference clock)"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(CLKW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRES`"]
pub enum PRESW {
    #[doc = "256 prescaler disabled."]
    _0,
    #[doc = "256 prescaler enabled."]
    _1,
}
impl PRESW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PRESW::_0 => false,
            PRESW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "256 prescaler disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRESW::_0)
    }
    #[doc = "256 prescaler enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRESW::_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CMD32EN`"]
pub enum CMD32ENW {
    #[doc = "Disables support for 32-bit refresh/unlock command write words. Only 16-bit or 8-bit is supported."]
    _0,
    #[doc = "Enables support for 32-bit refresh/unlock command write words. 16-bit or 8-bit is NOT supported."]
    _1,
}
impl CMD32ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CMD32ENW::_0 => false,
            CMD32ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMD32ENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMD32ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMD32ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables support for 32-bit refresh/unlock command write words. Only 16-bit or 8-bit is supported."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMD32ENW::_0)
    }
    #[doc = "Enables support for 32-bit refresh/unlock command write words. 16-bit or 8-bit is NOT supported."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMD32ENW::_1)
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
#[doc = "Values that can be written to the field `FLG`"]
pub enum FLGW {
    #[doc = "No interrupt occurred."]
    _0,
    #[doc = "An interrupt occurred."]
    _1,
}
impl FLGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLGW::_0 => false,
            FLGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLGW<'a> {
    w: &'a mut W,
}
impl<'a> _FLGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No interrupt occurred."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLGW::_0)
    }
    #[doc = "An interrupt occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLGW::_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WIN`"]
pub enum WINW {
    #[doc = "Window mode disabled."]
    _0,
    #[doc = "Window mode enabled."]
    _1,
}
impl WINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WINW::_0 => false,
            WINW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WINW<'a> {
    w: &'a mut W,
}
impl<'a> _WINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WINW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Window mode disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WINW::_0)
    }
    #[doc = "Window mode enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WINW::_1)
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
        const OFFSET: u8 = 15;
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
    #[doc = "Bit 0 - Stop Enable"]
    #[inline]
    pub fn stop(&self) -> STOPR {
        STOPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Wait Enable"]
    #[inline]
    pub fn wait(&self) -> WAITR {
        WAITR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Debug Enable"]
    #[inline]
    pub fn dbg(&self) -> DBGR {
        DBGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:4 - Watchdog Test"]
    #[inline]
    pub fn tst(&self) -> TSTR {
        TSTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - Allow updates"]
    #[inline]
    pub fn update(&self) -> UPDATER {
        UPDATER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Watchdog Interrupt"]
    #[inline]
    pub fn int(&self) -> INTR {
        INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Watchdog Enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:9 - Watchdog Clock"]
    #[inline]
    pub fn clk(&self) -> CLKR {
        CLKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 10 - Reconfiguration Success"]
    #[inline]
    pub fn rcs(&self) -> RCSR {
        RCSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Unlock status"]
    #[inline]
    pub fn ulk(&self) -> ULKR {
        ULKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Watchdog prescaler"]
    #[inline]
    pub fn pres(&self) -> PRESR {
        PRESR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words"]
    #[inline]
    pub fn cmd32en(&self) -> CMD32ENR {
        CMD32ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Watchdog Interrupt Flag"]
    #[inline]
    pub fn flg(&self) -> FLGR {
        FLGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Watchdog Window"]
    #[inline]
    pub fn win(&self) -> WINR {
        WINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 10624 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Stop Enable"]
    #[inline]
    pub fn stop(&mut self) -> _STOPW {
        _STOPW { w: self }
    }
    #[doc = "Bit 1 - Wait Enable"]
    #[inline]
    pub fn wait(&mut self) -> _WAITW {
        _WAITW { w: self }
    }
    #[doc = "Bit 2 - Debug Enable"]
    #[inline]
    pub fn dbg(&mut self) -> _DBGW {
        _DBGW { w: self }
    }
    #[doc = "Bits 3:4 - Watchdog Test"]
    #[inline]
    pub fn tst(&mut self) -> _TSTW {
        _TSTW { w: self }
    }
    #[doc = "Bit 5 - Allow updates"]
    #[inline]
    pub fn update(&mut self) -> _UPDATEW {
        _UPDATEW { w: self }
    }
    #[doc = "Bit 6 - Watchdog Interrupt"]
    #[inline]
    pub fn int(&mut self) -> _INTW {
        _INTW { w: self }
    }
    #[doc = "Bit 7 - Watchdog Enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bits 8:9 - Watchdog Clock"]
    #[inline]
    pub fn clk(&mut self) -> _CLKW {
        _CLKW { w: self }
    }
    #[doc = "Bit 12 - Watchdog prescaler"]
    #[inline]
    pub fn pres(&mut self) -> _PRESW {
        _PRESW { w: self }
    }
    #[doc = "Bit 13 - Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words"]
    #[inline]
    pub fn cmd32en(&mut self) -> _CMD32ENW {
        _CMD32ENW { w: self }
    }
    #[doc = "Bit 14 - Watchdog Interrupt Flag"]
    #[inline]
    pub fn flg(&mut self) -> _FLGW {
        _FLGW { w: self }
    }
    #[doc = "Bit 15 - Watchdog Window"]
    #[inline]
    pub fn win(&mut self) -> _WINW {
        _WINW { w: self }
    }
}
