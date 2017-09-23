#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `PT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PTR {
    #[doc = "Even parity."] _0,
    #[doc = "Odd parity."] _1,
}
impl PTR {
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
            PTR::_0 => false,
            PTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PTR {
        match value {
            false => PTR::_0,
            true => PTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PTR::_1
    }
}
#[doc = "Possible values of the field `PE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER {
    #[doc = "No hardware parity generation or checking."] _0,
    #[doc = "Parity enabled."] _1,
}
impl PER {
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
            PER::_0 => false,
            PER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PER {
        match value {
            false => PER::_0,
            true => PER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PER::_1
    }
}
#[doc = "Possible values of the field `ILT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILTR {
    #[doc = "Idle character bit count starts after start bit."] _0,
    #[doc = "Idle character bit count starts after stop bit."] _1,
}
impl ILTR {
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
            ILTR::_0 => false,
            ILTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ILTR {
        match value {
            false => ILTR::_0,
            true => ILTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ILTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ILTR::_1
    }
}
#[doc = "Possible values of the field `WAKE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKER {
    #[doc = "Configures RWU for idle-line wakeup."] _0,
    #[doc = "Configures RWU with address-mark wakeup."] _1,
}
impl WAKER {
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
            WAKER::_0 => false,
            WAKER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WAKER {
        match value {
            false => WAKER::_0,
            true => WAKER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WAKER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WAKER::_1
    }
}
#[doc = "Possible values of the field `M`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MR {
    #[doc = "Receiver and transmitter use 8-bit data characters."] _0,
    #[doc = "Receiver and transmitter use 9-bit data characters."] _1,
}
impl MR {
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
            MR::_0 => false,
            MR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MR {
        match value {
            false => MR::_0,
            true => MR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MR::_1
    }
}
#[doc = "Possible values of the field `RSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSRCR {
    #[doc = "Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the LPUART does not use the RXD pin."]
    _0,
    #[doc = "Single-wire LPUART mode where the TXD pin is connected to the transmitter output and receiver input."]
    _1,
}
impl RSRCR {
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
            RSRCR::_0 => false,
            RSRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSRCR {
        match value {
            false => RSRCR::_0,
            true => RSRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RSRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RSRCR::_1
    }
}
#[doc = "Possible values of the field `DOZEEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZEENR {
    #[doc = "LPUART is enabled in Doze mode."] _0,
    #[doc = "LPUART is disabled in Doze mode."] _1,
}
impl DOZEENR {
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
            DOZEENR::_0 => false,
            DOZEENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOZEENR {
        match value {
            false => DOZEENR::_0,
            true => DOZEENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DOZEENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DOZEENR::_1
    }
}
#[doc = "Possible values of the field `LOOPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPSR {
    #[doc = "Normal operation - RXD and TXD use separate pins."] _0,
    #[doc = "Loop mode or single-wire mode where transmitter outputs are internally connected to receiver input (see RSRC bit)."]
    _1,
}
impl LOOPSR {
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
            LOOPSR::_0 => false,
            LOOPSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOOPSR {
        match value {
            false => LOOPSR::_0,
            true => LOOPSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOOPSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOOPSR::_1
    }
}
#[doc = "Possible values of the field `IDLECFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECFGR {
    #[doc = "1 idle character"] _000,
    #[doc = "2 idle characters"] _001,
    #[doc = "4 idle characters"] _010,
    #[doc = "8 idle characters"] _011,
    #[doc = "16 idle characters"] _100,
    #[doc = "32 idle characters"] _101,
    #[doc = "64 idle characters"] _110,
    #[doc = "128 idle characters"] _111,
}
impl IDLECFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            IDLECFGR::_000 => 0,
            IDLECFGR::_001 => 1,
            IDLECFGR::_010 => 2,
            IDLECFGR::_011 => 3,
            IDLECFGR::_100 => 4,
            IDLECFGR::_101 => 5,
            IDLECFGR::_110 => 6,
            IDLECFGR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> IDLECFGR {
        match value {
            0 => IDLECFGR::_000,
            1 => IDLECFGR::_001,
            2 => IDLECFGR::_010,
            3 => IDLECFGR::_011,
            4 => IDLECFGR::_100,
            5 => IDLECFGR::_101,
            6 => IDLECFGR::_110,
            7 => IDLECFGR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == IDLECFGR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == IDLECFGR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == IDLECFGR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == IDLECFGR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == IDLECFGR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == IDLECFGR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == IDLECFGR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == IDLECFGR::_111
    }
}
#[doc = "Possible values of the field `M7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M7R {
    #[doc = "Receiver and transmitter use 8-bit to 10-bit data characters."] _0,
    #[doc = "Receiver and transmitter use 7-bit data characters."] _1,
}
impl M7R {
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
            M7R::_0 => false,
            M7R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M7R {
        match value {
            false => M7R::_0,
            true => M7R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M7R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M7R::_1
    }
}
#[doc = "Possible values of the field `MA2IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MA2IER {
    #[doc = "MA2F interrupt disabled"] _0,
    #[doc = "MA2F interrupt enabled"] _1,
}
impl MA2IER {
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
            MA2IER::_0 => false,
            MA2IER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MA2IER {
        match value {
            false => MA2IER::_0,
            true => MA2IER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MA2IER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MA2IER::_1
    }
}
#[doc = "Possible values of the field `MA1IE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MA1IER {
    #[doc = "MA1F interrupt disabled"] _0,
    #[doc = "MA1F interrupt enabled"] _1,
}
impl MA1IER {
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
            MA1IER::_0 => false,
            MA1IER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MA1IER {
        match value {
            false => MA1IER::_0,
            true => MA1IER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MA1IER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MA1IER::_1
    }
}
#[doc = "Possible values of the field `SBK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBKR {
    #[doc = "Normal transmitter operation."] _0,
    #[doc = "Queue break character(s) to be sent."] _1,
}
impl SBKR {
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
            SBKR::_0 => false,
            SBKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBKR {
        match value {
            false => SBKR::_0,
            true => SBKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SBKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SBKR::_1
    }
}
#[doc = "Possible values of the field `RWU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWUR {
    #[doc = "Normal receiver operation."] _0,
    #[doc = "LPUART receiver in standby waiting for wakeup condition."] _1,
}
impl RWUR {
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
            RWUR::_0 => false,
            RWUR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWUR {
        match value {
            false => RWUR::_0,
            true => RWUR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWUR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWUR::_1
    }
}
#[doc = "Possible values of the field `RE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RER {
    #[doc = "Receiver disabled."] _0,
    #[doc = "Receiver enabled."] _1,
}
impl RER {
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
            RER::_0 => false,
            RER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RER {
        match value {
            false => RER::_0,
            true => RER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RER::_1
    }
}
#[doc = "Possible values of the field `TE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TER {
    #[doc = "Transmitter disabled."] _0,
    #[doc = "Transmitter enabled."] _1,
}
impl TER {
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
            TER::_0 => false,
            TER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TER {
        match value {
            false => TER::_0,
            true => TER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TER::_1
    }
}
#[doc = "Possible values of the field `ILIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILIER {
    #[doc = "Hardware interrupts from IDLE disabled; use polling."] _0,
    #[doc = "Hardware interrupt requested when IDLE flag is 1."] _1,
}
impl ILIER {
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
            ILIER::_0 => false,
            ILIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ILIER {
        match value {
            false => ILIER::_0,
            true => ILIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ILIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ILIER::_1
    }
}
#[doc = "Possible values of the field `RIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIER {
    #[doc = "Hardware interrupts from RDRF disabled; use polling."] _0,
    #[doc = "Hardware interrupt requested when RDRF flag is 1."] _1,
}
impl RIER {
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
            RIER::_0 => false,
            RIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RIER {
        match value {
            false => RIER::_0,
            true => RIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RIER::_1
    }
}
#[doc = "Possible values of the field `TCIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCIER {
    #[doc = "Hardware interrupts from TC disabled; use polling."] _0,
    #[doc = "Hardware interrupt requested when TC flag is 1."] _1,
}
impl TCIER {
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
            TCIER::_0 => false,
            TCIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCIER {
        match value {
            false => TCIER::_0,
            true => TCIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TCIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TCIER::_1
    }
}
#[doc = "Possible values of the field `TIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIER {
    #[doc = "Hardware interrupts from TDRE disabled; use polling."] _0,
    #[doc = "Hardware interrupt requested when TDRE flag is 1."] _1,
}
impl TIER {
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
            TIER::_0 => false,
            TIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIER {
        match value {
            false => TIER::_0,
            true => TIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIER::_1
    }
}
#[doc = "Possible values of the field `PEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PEIER {
    #[doc = "PF interrupts disabled; use polling)."] _0,
    #[doc = "Hardware interrupt requested when PF is set."] _1,
}
impl PEIER {
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
            PEIER::_0 => false,
            PEIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PEIER {
        match value {
            false => PEIER::_0,
            true => PEIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PEIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PEIER::_1
    }
}
#[doc = "Possible values of the field `FEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEIER {
    #[doc = "FE interrupts disabled; use polling."] _0,
    #[doc = "Hardware interrupt requested when FE is set."] _1,
}
impl FEIER {
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
            FEIER::_0 => false,
            FEIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FEIER {
        match value {
            false => FEIER::_0,
            true => FEIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FEIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FEIER::_1
    }
}
#[doc = "Possible values of the field `NEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NEIER {
    #[doc = "NF interrupts disabled; use polling."] _0,
    #[doc = "Hardware interrupt requested when NF is set."] _1,
}
impl NEIER {
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
            NEIER::_0 => false,
            NEIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NEIER {
        match value {
            false => NEIER::_0,
            true => NEIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NEIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NEIER::_1
    }
}
#[doc = "Possible values of the field `ORIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORIER {
    #[doc = "OR interrupts disabled; use polling."] _0,
    #[doc = "Hardware interrupt requested when OR is set."] _1,
}
impl ORIER {
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
            ORIER::_0 => false,
            ORIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ORIER {
        match value {
            false => ORIER::_0,
            true => ORIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ORIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ORIER::_1
    }
}
#[doc = "Possible values of the field `TXINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXINVR {
    #[doc = "Transmit data not inverted."] _0,
    #[doc = "Transmit data inverted."] _1,
}
impl TXINVR {
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
            TXINVR::_0 => false,
            TXINVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXINVR {
        match value {
            false => TXINVR::_0,
            true => TXINVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXINVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXINVR::_1
    }
}
#[doc = "Possible values of the field `TXDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDIRR {
    #[doc = "TXD pin is an input in single-wire mode."] _0,
    #[doc = "TXD pin is an output in single-wire mode."] _1,
}
impl TXDIRR {
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
            TXDIRR::_0 => false,
            TXDIRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXDIRR {
        match value {
            false => TXDIRR::_0,
            true => TXDIRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXDIRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXDIRR::_1
    }
}
#[doc = r" Value of the field"]
pub struct R9T8R {
    bits: bool,
}
impl R9T8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct R8T9R {
    bits: bool,
}
impl R8T9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Values that can be written to the field `PT`"]
pub enum PTW {
    #[doc = "Even parity."] _0,
    #[doc = "Odd parity."] _1,
}
impl PTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PTW::_0 => false,
            PTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PTW<'a> {
    w: &'a mut W,
}
impl<'a> _PTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Even parity."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTW::_0)
    }
    #[doc = "Odd parity."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTW::_1)
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
#[doc = "Values that can be written to the field `PE`"]
pub enum PEW {
    #[doc = "No hardware parity generation or checking."] _0,
    #[doc = "Parity enabled."] _1,
}
impl PEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEW::_0 => false,
            PEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No hardware parity generation or checking."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEW::_0)
    }
    #[doc = "Parity enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEW::_1)
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
#[doc = "Values that can be written to the field `ILT`"]
pub enum ILTW {
    #[doc = "Idle character bit count starts after start bit."] _0,
    #[doc = "Idle character bit count starts after stop bit."] _1,
}
impl ILTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ILTW::_0 => false,
            ILTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ILTW<'a> {
    w: &'a mut W,
}
impl<'a> _ILTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ILTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Idle character bit count starts after start bit."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILTW::_0)
    }
    #[doc = "Idle character bit count starts after stop bit."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILTW::_1)
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
#[doc = "Values that can be written to the field `WAKE`"]
pub enum WAKEW {
    #[doc = "Configures RWU for idle-line wakeup."] _0,
    #[doc = "Configures RWU with address-mark wakeup."] _1,
}
impl WAKEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WAKEW::_0 => false,
            WAKEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WAKEW<'a> {
    w: &'a mut W,
}
impl<'a> _WAKEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WAKEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configures RWU for idle-line wakeup."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKEW::_0)
    }
    #[doc = "Configures RWU with address-mark wakeup."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKEW::_1)
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
#[doc = "Values that can be written to the field `M`"]
pub enum MW {
    #[doc = "Receiver and transmitter use 8-bit data characters."] _0,
    #[doc = "Receiver and transmitter use 9-bit data characters."] _1,
}
impl MW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MW::_0 => false,
            MW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MW<'a> {
    w: &'a mut W,
}
impl<'a> _MW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver and transmitter use 8-bit data characters."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MW::_0)
    }
    #[doc = "Receiver and transmitter use 9-bit data characters."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MW::_1)
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
#[doc = "Values that can be written to the field `RSRC`"]
pub enum RSRCW {
    #[doc = "Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the LPUART does not use the RXD pin."]
    _0,
    #[doc = "Single-wire LPUART mode where the TXD pin is connected to the transmitter output and receiver input."]
    _1,
}
impl RSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSRCW::_0 => false,
            RSRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _RSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the LPUART does not use the RXD pin."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSRCW::_0)
    }
    #[doc = "Single-wire LPUART mode where the TXD pin is connected to the transmitter output and receiver input."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSRCW::_1)
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
#[doc = "Values that can be written to the field `DOZEEN`"]
pub enum DOZEENW {
    #[doc = "LPUART is enabled in Doze mode."] _0,
    #[doc = "LPUART is disabled in Doze mode."] _1,
}
impl DOZEENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOZEENW::_0 => false,
            DOZEENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOZEENW<'a> {
    w: &'a mut W,
}
impl<'a> _DOZEENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOZEENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LPUART is enabled in Doze mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOZEENW::_0)
    }
    #[doc = "LPUART is disabled in Doze mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOZEENW::_1)
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
#[doc = "Values that can be written to the field `LOOPS`"]
pub enum LOOPSW {
    #[doc = "Normal operation - RXD and TXD use separate pins."] _0,
    #[doc = "Loop mode or single-wire mode where transmitter outputs are internally connected to receiver input (see RSRC bit)."]
    _1,
}
impl LOOPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOOPSW::_0 => false,
            LOOPSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOOPSW<'a> {
    w: &'a mut W,
}
impl<'a> _LOOPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOOPSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation - RXD and TXD use separate pins."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOOPSW::_0)
    }
    #[doc = "Loop mode or single-wire mode where transmitter outputs are internally connected to receiver input (see RSRC bit)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOOPSW::_1)
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
#[doc = "Values that can be written to the field `IDLECFG`"]
pub enum IDLECFGW {
    #[doc = "1 idle character"] _000,
    #[doc = "2 idle characters"] _001,
    #[doc = "4 idle characters"] _010,
    #[doc = "8 idle characters"] _011,
    #[doc = "16 idle characters"] _100,
    #[doc = "32 idle characters"] _101,
    #[doc = "64 idle characters"] _110,
    #[doc = "128 idle characters"] _111,
}
impl IDLECFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IDLECFGW::_000 => 0,
            IDLECFGW::_001 => 1,
            IDLECFGW::_010 => 2,
            IDLECFGW::_011 => 3,
            IDLECFGW::_100 => 4,
            IDLECFGW::_101 => 5,
            IDLECFGW::_110 => 6,
            IDLECFGW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLECFGW<'a> {
    w: &'a mut W,
}
impl<'a> _IDLECFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLECFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "1 idle character"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(IDLECFGW::_000)
    }
    #[doc = "2 idle characters"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(IDLECFGW::_001)
    }
    #[doc = "4 idle characters"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(IDLECFGW::_010)
    }
    #[doc = "8 idle characters"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(IDLECFGW::_011)
    }
    #[doc = "16 idle characters"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(IDLECFGW::_100)
    }
    #[doc = "32 idle characters"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(IDLECFGW::_101)
    }
    #[doc = "64 idle characters"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(IDLECFGW::_110)
    }
    #[doc = "128 idle characters"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(IDLECFGW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M7`"]
pub enum M7W {
    #[doc = "Receiver and transmitter use 8-bit to 10-bit data characters."] _0,
    #[doc = "Receiver and transmitter use 7-bit data characters."] _1,
}
impl M7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M7W::_0 => false,
            M7W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M7W<'a> {
    w: &'a mut W,
}
impl<'a> _M7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver and transmitter use 8-bit to 10-bit data characters."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M7W::_0)
    }
    #[doc = "Receiver and transmitter use 7-bit data characters."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M7W::_1)
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
#[doc = "Values that can be written to the field `MA2IE`"]
pub enum MA2IEW {
    #[doc = "MA2F interrupt disabled"] _0,
    #[doc = "MA2F interrupt enabled"] _1,
}
impl MA2IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MA2IEW::_0 => false,
            MA2IEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MA2IEW<'a> {
    w: &'a mut W,
}
impl<'a> _MA2IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MA2IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MA2F interrupt disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MA2IEW::_0)
    }
    #[doc = "MA2F interrupt enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MA2IEW::_1)
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
#[doc = "Values that can be written to the field `MA1IE`"]
pub enum MA1IEW {
    #[doc = "MA1F interrupt disabled"] _0,
    #[doc = "MA1F interrupt enabled"] _1,
}
impl MA1IEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MA1IEW::_0 => false,
            MA1IEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MA1IEW<'a> {
    w: &'a mut W,
}
impl<'a> _MA1IEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MA1IEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "MA1F interrupt disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MA1IEW::_0)
    }
    #[doc = "MA1F interrupt enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MA1IEW::_1)
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
#[doc = "Values that can be written to the field `SBK`"]
pub enum SBKW {
    #[doc = "Normal transmitter operation."] _0,
    #[doc = "Queue break character(s) to be sent."] _1,
}
impl SBKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBKW::_0 => false,
            SBKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBKW<'a> {
    w: &'a mut W,
}
impl<'a> _SBKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal transmitter operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBKW::_0)
    }
    #[doc = "Queue break character(s) to be sent."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBKW::_1)
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
#[doc = "Values that can be written to the field `RWU`"]
pub enum RWUW {
    #[doc = "Normal receiver operation."] _0,
    #[doc = "LPUART receiver in standby waiting for wakeup condition."] _1,
}
impl RWUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWUW::_0 => false,
            RWUW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWUW<'a> {
    w: &'a mut W,
}
impl<'a> _RWUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWUW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal receiver operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWUW::_0)
    }
    #[doc = "LPUART receiver in standby waiting for wakeup condition."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWUW::_1)
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
#[doc = "Values that can be written to the field `RE`"]
pub enum REW {
    #[doc = "Receiver disabled."] _0,
    #[doc = "Receiver enabled."] _1,
}
impl REW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REW::_0 => false,
            REW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REW<'a> {
    w: &'a mut W,
}
impl<'a> _REW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(REW::_0)
    }
    #[doc = "Receiver enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(REW::_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TE`"]
pub enum TEW {
    #[doc = "Transmitter disabled."] _0,
    #[doc = "Transmitter enabled."] _1,
}
impl TEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEW::_0 => false,
            TEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEW<'a> {
    w: &'a mut W,
}
impl<'a> _TEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmitter disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEW::_0)
    }
    #[doc = "Transmitter enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEW::_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ILIE`"]
pub enum ILIEW {
    #[doc = "Hardware interrupts from IDLE disabled; use polling."] _0,
    #[doc = "Hardware interrupt requested when IDLE flag is 1."] _1,
}
impl ILIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ILIEW::_0 => false,
            ILIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ILIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ILIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ILIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware interrupts from IDLE disabled; use polling."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILIEW::_0)
    }
    #[doc = "Hardware interrupt requested when IDLE flag is 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILIEW::_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RIE`"]
pub enum RIEW {
    #[doc = "Hardware interrupts from RDRF disabled; use polling."] _0,
    #[doc = "Hardware interrupt requested when RDRF flag is 1."] _1,
}
impl RIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RIEW::_0 => false,
            RIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware interrupts from RDRF disabled; use polling."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RIEW::_0)
    }
    #[doc = "Hardware interrupt requested when RDRF flag is 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RIEW::_1)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TCIE`"]
pub enum TCIEW {
    #[doc = "Hardware interrupts from TC disabled; use polling."] _0,
    #[doc = "Hardware interrupt requested when TC flag is 1."] _1,
}
impl TCIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCIEW::_0 => false,
            TCIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TCIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware interrupts from TC disabled; use polling."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCIEW::_0)
    }
    #[doc = "Hardware interrupt requested when TC flag is 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCIEW::_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIE`"]
pub enum TIEW {
    #[doc = "Hardware interrupts from TDRE disabled; use polling."] _0,
    #[doc = "Hardware interrupt requested when TDRE flag is 1."] _1,
}
impl TIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIEW::_0 => false,
            TIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware interrupts from TDRE disabled; use polling."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIEW::_0)
    }
    #[doc = "Hardware interrupt requested when TDRE flag is 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIEW::_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PEIE`"]
pub enum PEIEW {
    #[doc = "PF interrupts disabled; use polling)."] _0,
    #[doc = "Hardware interrupt requested when PF is set."] _1,
}
impl PEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PEIEW::_0 => false,
            PEIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _PEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PF interrupts disabled; use polling)."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEIEW::_0)
    }
    #[doc = "Hardware interrupt requested when PF is set."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEIEW::_1)
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
#[doc = "Values that can be written to the field `FEIE`"]
pub enum FEIEW {
    #[doc = "FE interrupts disabled; use polling."] _0,
    #[doc = "Hardware interrupt requested when FE is set."] _1,
}
impl FEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEIEW::_0 => false,
            FEIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _FEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FE interrupts disabled; use polling."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEIEW::_0)
    }
    #[doc = "Hardware interrupt requested when FE is set."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEIEW::_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `NEIE`"]
pub enum NEIEW {
    #[doc = "NF interrupts disabled; use polling."] _0,
    #[doc = "Hardware interrupt requested when NF is set."] _1,
}
impl NEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NEIEW::_0 => false,
            NEIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _NEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "NF interrupts disabled; use polling."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NEIEW::_0)
    }
    #[doc = "Hardware interrupt requested when NF is set."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NEIEW::_1)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ORIE`"]
pub enum ORIEW {
    #[doc = "OR interrupts disabled; use polling."] _0,
    #[doc = "Hardware interrupt requested when OR is set."] _1,
}
impl ORIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ORIEW::_0 => false,
            ORIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ORIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ORIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ORIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "OR interrupts disabled; use polling."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ORIEW::_0)
    }
    #[doc = "Hardware interrupt requested when OR is set."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ORIEW::_1)
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXINV`"]
pub enum TXINVW {
    #[doc = "Transmit data not inverted."] _0,
    #[doc = "Transmit data inverted."] _1,
}
impl TXINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXINVW::_0 => false,
            TXINVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXINVW<'a> {
    w: &'a mut W,
}
impl<'a> _TXINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit data not inverted."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXINVW::_0)
    }
    #[doc = "Transmit data inverted."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXINVW::_1)
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXDIR`"]
pub enum TXDIRW {
    #[doc = "TXD pin is an input in single-wire mode."] _0,
    #[doc = "TXD pin is an output in single-wire mode."] _1,
}
impl TXDIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDIRW::_0 => false,
            TXDIRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TXD pin is an input in single-wire mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXDIRW::_0)
    }
    #[doc = "TXD pin is an output in single-wire mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXDIRW::_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _R9T8W<'a> {
    w: &'a mut W,
}
impl<'a> _R9T8W<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _R8T9W<'a> {
    w: &'a mut W,
}
impl<'a> _R8T9W<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - Parity Type"]
    #[inline]
    pub fn pt(&self) -> PTR {
        PTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline]
    pub fn pe(&self) -> PER {
        PER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline]
    pub fn ilt(&self) -> ILTR {
        ILTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline]
    pub fn wake(&self) -> WAKER {
        WAKER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - 9-Bit or 8-Bit Mode Select"]
    #[inline]
    pub fn m(&self) -> MR {
        MR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline]
    pub fn rsrc(&self) -> RSRCR {
        RSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Doze Enable"]
    #[inline]
    pub fn dozeen(&self) -> DOZEENR {
        DOZEENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline]
    pub fn loops(&self) -> LOOPSR {
        LOOPSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:10 - Idle Configuration"]
    #[inline]
    pub fn idlecfg(&self) -> IDLECFGR {
        IDLECFGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 11 - 7-Bit Mode Select"]
    #[inline]
    pub fn m7(&self) -> M7R {
        M7R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Match 2 Interrupt Enable"]
    #[inline]
    pub fn ma2ie(&self) -> MA2IER {
        MA2IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Match 1 Interrupt Enable"]
    #[inline]
    pub fn ma1ie(&self) -> MA1IER {
        MA1IER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Send Break"]
    #[inline]
    pub fn sbk(&self) -> SBKR {
        SBKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Receiver Wakeup Control"]
    #[inline]
    pub fn rwu(&self) -> RWUR {
        RWUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Receiver Enable"]
    #[inline]
    pub fn re(&self) -> RER {
        RER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Transmitter Enable"]
    #[inline]
    pub fn te(&self) -> TER {
        TER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Idle Line Interrupt Enable"]
    #[inline]
    pub fn ilie(&self) -> ILIER {
        ILIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Receiver Interrupt Enable"]
    #[inline]
    pub fn rie(&self) -> RIER {
        RIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Transmission Complete Interrupt Enable for"]
    #[inline]
    pub fn tcie(&self) -> TCIER {
        TCIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Transmit Interrupt Enable"]
    #[inline]
    pub fn tie(&self) -> TIER {
        TIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Parity Error Interrupt Enable"]
    #[inline]
    pub fn peie(&self) -> PEIER {
        PEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Framing Error Interrupt Enable"]
    #[inline]
    pub fn feie(&self) -> FEIER {
        FEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Noise Error Interrupt Enable"]
    #[inline]
    pub fn neie(&self) -> NEIER {
        NEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Overrun Interrupt Enable"]
    #[inline]
    pub fn orie(&self) -> ORIER {
        ORIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Transmit Data Inversion"]
    #[inline]
    pub fn txinv(&self) -> TXINVR {
        TXINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - TXD Pin Direction in Single-Wire Mode"]
    #[inline]
    pub fn txdir(&self) -> TXDIRR {
        TXDIRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Receive Bit 9 / Transmit Bit 8"]
    #[inline]
    pub fn r9t8(&self) -> R9T8R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        R9T8R { bits }
    }
    #[doc = "Bit 31 - Receive Bit 8 / Transmit Bit 9"]
    #[inline]
    pub fn r8t9(&self) -> R8T9R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        R8T9R { bits }
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
    #[doc = "Bit 0 - Parity Type"]
    #[inline]
    pub fn pt(&mut self) -> _PTW {
        _PTW { w: self }
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline]
    pub fn pe(&mut self) -> _PEW {
        _PEW { w: self }
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline]
    pub fn ilt(&mut self) -> _ILTW {
        _ILTW { w: self }
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline]
    pub fn wake(&mut self) -> _WAKEW {
        _WAKEW { w: self }
    }
    #[doc = "Bit 4 - 9-Bit or 8-Bit Mode Select"]
    #[inline]
    pub fn m(&mut self) -> _MW {
        _MW { w: self }
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline]
    pub fn rsrc(&mut self) -> _RSRCW {
        _RSRCW { w: self }
    }
    #[doc = "Bit 6 - Doze Enable"]
    #[inline]
    pub fn dozeen(&mut self) -> _DOZEENW {
        _DOZEENW { w: self }
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline]
    pub fn loops(&mut self) -> _LOOPSW {
        _LOOPSW { w: self }
    }
    #[doc = "Bits 8:10 - Idle Configuration"]
    #[inline]
    pub fn idlecfg(&mut self) -> _IDLECFGW {
        _IDLECFGW { w: self }
    }
    #[doc = "Bit 11 - 7-Bit Mode Select"]
    #[inline]
    pub fn m7(&mut self) -> _M7W {
        _M7W { w: self }
    }
    #[doc = "Bit 14 - Match 2 Interrupt Enable"]
    #[inline]
    pub fn ma2ie(&mut self) -> _MA2IEW {
        _MA2IEW { w: self }
    }
    #[doc = "Bit 15 - Match 1 Interrupt Enable"]
    #[inline]
    pub fn ma1ie(&mut self) -> _MA1IEW {
        _MA1IEW { w: self }
    }
    #[doc = "Bit 16 - Send Break"]
    #[inline]
    pub fn sbk(&mut self) -> _SBKW {
        _SBKW { w: self }
    }
    #[doc = "Bit 17 - Receiver Wakeup Control"]
    #[inline]
    pub fn rwu(&mut self) -> _RWUW {
        _RWUW { w: self }
    }
    #[doc = "Bit 18 - Receiver Enable"]
    #[inline]
    pub fn re(&mut self) -> _REW {
        _REW { w: self }
    }
    #[doc = "Bit 19 - Transmitter Enable"]
    #[inline]
    pub fn te(&mut self) -> _TEW {
        _TEW { w: self }
    }
    #[doc = "Bit 20 - Idle Line Interrupt Enable"]
    #[inline]
    pub fn ilie(&mut self) -> _ILIEW {
        _ILIEW { w: self }
    }
    #[doc = "Bit 21 - Receiver Interrupt Enable"]
    #[inline]
    pub fn rie(&mut self) -> _RIEW {
        _RIEW { w: self }
    }
    #[doc = "Bit 22 - Transmission Complete Interrupt Enable for"]
    #[inline]
    pub fn tcie(&mut self) -> _TCIEW {
        _TCIEW { w: self }
    }
    #[doc = "Bit 23 - Transmit Interrupt Enable"]
    #[inline]
    pub fn tie(&mut self) -> _TIEW {
        _TIEW { w: self }
    }
    #[doc = "Bit 24 - Parity Error Interrupt Enable"]
    #[inline]
    pub fn peie(&mut self) -> _PEIEW {
        _PEIEW { w: self }
    }
    #[doc = "Bit 25 - Framing Error Interrupt Enable"]
    #[inline]
    pub fn feie(&mut self) -> _FEIEW {
        _FEIEW { w: self }
    }
    #[doc = "Bit 26 - Noise Error Interrupt Enable"]
    #[inline]
    pub fn neie(&mut self) -> _NEIEW {
        _NEIEW { w: self }
    }
    #[doc = "Bit 27 - Overrun Interrupt Enable"]
    #[inline]
    pub fn orie(&mut self) -> _ORIEW {
        _ORIEW { w: self }
    }
    #[doc = "Bit 28 - Transmit Data Inversion"]
    #[inline]
    pub fn txinv(&mut self) -> _TXINVW {
        _TXINVW { w: self }
    }
    #[doc = "Bit 29 - TXD Pin Direction in Single-Wire Mode"]
    #[inline]
    pub fn txdir(&mut self) -> _TXDIRW {
        _TXDIRW { w: self }
    }
    #[doc = "Bit 30 - Receive Bit 9 / Transmit Bit 8"]
    #[inline]
    pub fn r9t8(&mut self) -> _R9T8W {
        _R9T8W { w: self }
    }
    #[doc = "Bit 31 - Receive Bit 8 / Transmit Bit 9"]
    #[inline]
    pub fn r8t9(&mut self) -> _R8T9W {
        _R8T9W { w: self }
    }
}
