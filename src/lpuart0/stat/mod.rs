#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STAT {
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
#[doc = "Possible values of the field `MA2F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MA2FR {
    #[doc = "Received data is not equal to MA2"]
    _0,
    #[doc = "Received data is equal to MA2"]
    _1,
}
impl MA2FR {
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
            MA2FR::_0 => false,
            MA2FR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MA2FR {
        match value {
            false => MA2FR::_0,
            true => MA2FR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MA2FR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MA2FR::_1
    }
}
#[doc = "Possible values of the field `MA1F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MA1FR {
    #[doc = "Received data is not equal to MA1"]
    _0,
    #[doc = "Received data is equal to MA1"]
    _1,
}
impl MA1FR {
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
            MA1FR::_0 => false,
            MA1FR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MA1FR {
        match value {
            false => MA1FR::_0,
            true => MA1FR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MA1FR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MA1FR::_1
    }
}
#[doc = "Possible values of the field `PF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFR {
    #[doc = "No parity error."]
    _0,
    #[doc = "Parity error."]
    _1,
}
impl PFR {
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
            PFR::_0 => false,
            PFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PFR {
        match value {
            false => PFR::_0,
            true => PFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PFR::_1
    }
}
#[doc = "Possible values of the field `FE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FER {
    #[doc = "No framing error detected. This does not guarantee the framing is correct."]
    _0,
    #[doc = "Framing error."]
    _1,
}
impl FER {
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
            FER::_0 => false,
            FER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FER {
        match value {
            false => FER::_0,
            true => FER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FER::_1
    }
}
#[doc = "Possible values of the field `NF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NFR {
    #[doc = "No noise detected."]
    _0,
    #[doc = "Noise detected in the received character in LPUART_DATA."]
    _1,
}
impl NFR {
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
            NFR::_0 => false,
            NFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NFR {
        match value {
            false => NFR::_0,
            true => NFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NFR::_1
    }
}
#[doc = "Possible values of the field `OR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ORR {
    #[doc = "No overrun."]
    _0,
    #[doc = "Receive overrun (new LPUART data lost)."]
    _1,
}
impl ORR {
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
            ORR::_0 => false,
            ORR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ORR {
        match value {
            false => ORR::_0,
            true => ORR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ORR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ORR::_1
    }
}
#[doc = "Possible values of the field `IDLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLER {
    #[doc = "No idle line detected."]
    _0,
    #[doc = "Idle line was detected."]
    _1,
}
impl IDLER {
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
            IDLER::_0 => false,
            IDLER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDLER {
        match value {
            false => IDLER::_0,
            true => IDLER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IDLER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IDLER::_1
    }
}
#[doc = "Possible values of the field `RDRF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDRFR {
    #[doc = "Receive data buffer empty."]
    _0,
    #[doc = "Receive data buffer full."]
    _1,
}
impl RDRFR {
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
            RDRFR::_0 => false,
            RDRFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDRFR {
        match value {
            false => RDRFR::_0,
            true => RDRFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RDRFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RDRFR::_1
    }
}
#[doc = "Possible values of the field `TC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCR {
    #[doc = "Transmitter active (sending data, a preamble, or a break)."]
    _0,
    #[doc = "Transmitter idle (transmission activity complete)."]
    _1,
}
impl TCR {
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
            TCR::_0 => false,
            TCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCR {
        match value {
            false => TCR::_0,
            true => TCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TCR::_1
    }
}
#[doc = "Possible values of the field `TDRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRER {
    #[doc = "Transmit data buffer full."]
    _0,
    #[doc = "Transmit data buffer empty."]
    _1,
}
impl TDRER {
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
            TDRER::_0 => false,
            TDRER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDRER {
        match value {
            false => TDRER::_0,
            true => TDRER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TDRER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TDRER::_1
    }
}
#[doc = "Possible values of the field `RAF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAFR {
    #[doc = "LPUART receiver idle waiting for a start bit."]
    _0,
    #[doc = "LPUART receiver active (RXD input not idle)."]
    _1,
}
impl RAFR {
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
            RAFR::_0 => false,
            RAFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RAFR {
        match value {
            false => RAFR::_0,
            true => RAFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RAFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RAFR::_1
    }
}
#[doc = "Possible values of the field `LBKDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDER {
    #[doc = "LIN break detect is disabled, normal break character can be detected."]
    _0,
    #[doc = "LIN break detect is enabled. LIN break character is detected at length of 11 bit times (if M = 0) or 12 (if M = 1) or 13 (M10 = 1)."]
    _1,
}
impl LBKDER {
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
            LBKDER::_0 => false,
            LBKDER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBKDER {
        match value {
            false => LBKDER::_0,
            true => LBKDER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LBKDER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LBKDER::_1
    }
}
#[doc = "Possible values of the field `BRK13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BRK13R {
    #[doc = "Break character is transmitted with length of 9 to 13 bit times."]
    _0,
    #[doc = "Break character is transmitted with length of 12 to 15 bit times."]
    _1,
}
impl BRK13R {
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
            BRK13R::_0 => false,
            BRK13R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BRK13R {
        match value {
            false => BRK13R::_0,
            true => BRK13R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BRK13R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BRK13R::_1
    }
}
#[doc = "Possible values of the field `RWUID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWUIDR {
    #[doc = "During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character. During address match wakeup, the IDLE bit does not set when an address does not match."]
    _0,
    #[doc = "During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character. During address match wakeup, the IDLE bit does set when an address does not match."]
    _1,
}
impl RWUIDR {
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
            RWUIDR::_0 => false,
            RWUIDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWUIDR {
        match value {
            false => RWUIDR::_0,
            true => RWUIDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWUIDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWUIDR::_1
    }
}
#[doc = "Possible values of the field `RXINV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXINVR {
    #[doc = "Receive data not inverted."]
    _0,
    #[doc = "Receive data inverted."]
    _1,
}
impl RXINVR {
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
            RXINVR::_0 => false,
            RXINVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXINVR {
        match value {
            false => RXINVR::_0,
            true => RXINVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXINVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXINVR::_1
    }
}
#[doc = "Possible values of the field `MSBF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSBFR {
    #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    _0,
    #[doc = "MSB (bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of CTRL[M], CTRL[PE] and BAUD[M10]. Further, the first bit received after the start bit is identified as bit9, bit8, bit7 or bit6 depending on the setting of CTRL[M] and CTRL[PE]."]
    _1,
}
impl MSBFR {
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
            MSBFR::_0 => false,
            MSBFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MSBFR {
        match value {
            false => MSBFR::_0,
            true => MSBFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MSBFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MSBFR::_1
    }
}
#[doc = "Possible values of the field `RXEDGIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEDGIFR {
    #[doc = "No active edge on the receive pin has occurred."]
    _0,
    #[doc = "An active edge on the receive pin has occurred."]
    _1,
}
impl RXEDGIFR {
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
            RXEDGIFR::_0 => false,
            RXEDGIFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXEDGIFR {
        match value {
            false => RXEDGIFR::_0,
            true => RXEDGIFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXEDGIFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXEDGIFR::_1
    }
}
#[doc = "Possible values of the field `LBKDIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDIFR {
    #[doc = "No LIN break character has been detected."]
    _0,
    #[doc = "LIN break character has been detected."]
    _1,
}
impl LBKDIFR {
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
            LBKDIFR::_0 => false,
            LBKDIFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBKDIFR {
        match value {
            false => LBKDIFR::_0,
            true => LBKDIFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LBKDIFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LBKDIFR::_1
    }
}
#[doc = "Values that can be written to the field `MA2F`"]
pub enum MA2FW {
    #[doc = "Received data is not equal to MA2"]
    _0,
    #[doc = "Received data is equal to MA2"]
    _1,
}
impl MA2FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MA2FW::_0 => false,
            MA2FW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MA2FW<'a> {
    w: &'a mut W,
}
impl<'a> _MA2FW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MA2FW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Received data is not equal to MA2"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MA2FW::_0)
    }
    #[doc = "Received data is equal to MA2"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MA2FW::_1)
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
#[doc = "Values that can be written to the field `MA1F`"]
pub enum MA1FW {
    #[doc = "Received data is not equal to MA1"]
    _0,
    #[doc = "Received data is equal to MA1"]
    _1,
}
impl MA1FW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MA1FW::_0 => false,
            MA1FW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MA1FW<'a> {
    w: &'a mut W,
}
impl<'a> _MA1FW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MA1FW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Received data is not equal to MA1"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MA1FW::_0)
    }
    #[doc = "Received data is equal to MA1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MA1FW::_1)
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
#[doc = "Values that can be written to the field `PF`"]
pub enum PFW {
    #[doc = "No parity error."]
    _0,
    #[doc = "Parity error."]
    _1,
}
impl PFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PFW::_0 => false,
            PFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PFW<'a> {
    w: &'a mut W,
}
impl<'a> _PFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No parity error."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PFW::_0)
    }
    #[doc = "Parity error."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PFW::_1)
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
#[doc = "Values that can be written to the field `FE`"]
pub enum FEW {
    #[doc = "No framing error detected. This does not guarantee the framing is correct."]
    _0,
    #[doc = "Framing error."]
    _1,
}
impl FEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEW::_0 => false,
            FEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEW<'a> {
    w: &'a mut W,
}
impl<'a> _FEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No framing error detected. This does not guarantee the framing is correct."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEW::_0)
    }
    #[doc = "Framing error."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEW::_1)
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
#[doc = "Values that can be written to the field `NF`"]
pub enum NFW {
    #[doc = "No noise detected."]
    _0,
    #[doc = "Noise detected in the received character in LPUART_DATA."]
    _1,
}
impl NFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NFW::_0 => false,
            NFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NFW<'a> {
    w: &'a mut W,
}
impl<'a> _NFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No noise detected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NFW::_0)
    }
    #[doc = "Noise detected in the received character in LPUART_DATA."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NFW::_1)
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
#[doc = "Values that can be written to the field `OR`"]
pub enum ORW {
    #[doc = "No overrun."]
    _0,
    #[doc = "Receive overrun (new LPUART data lost)."]
    _1,
}
impl ORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ORW::_0 => false,
            ORW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ORW<'a> {
    w: &'a mut W,
}
impl<'a> _ORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No overrun."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ORW::_0)
    }
    #[doc = "Receive overrun (new LPUART data lost)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ORW::_1)
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
#[doc = "Values that can be written to the field `IDLE`"]
pub enum IDLEW {
    #[doc = "No idle line detected."]
    _0,
    #[doc = "Idle line was detected."]
    _1,
}
impl IDLEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDLEW::_0 => false,
            IDLEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDLEW<'a> {
    w: &'a mut W,
}
impl<'a> _IDLEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDLEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No idle line detected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDLEW::_0)
    }
    #[doc = "Idle line was detected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDLEW::_1)
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
#[doc = "Values that can be written to the field `LBKDE`"]
pub enum LBKDEW {
    #[doc = "LIN break detect is disabled, normal break character can be detected."]
    _0,
    #[doc = "LIN break detect is enabled. LIN break character is detected at length of 11 bit times (if M = 0) or 12 (if M = 1) or 13 (M10 = 1)."]
    _1,
}
impl LBKDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBKDEW::_0 => false,
            LBKDEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBKDEW<'a> {
    w: &'a mut W,
}
impl<'a> _LBKDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBKDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LIN break detect is disabled, normal break character can be detected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDEW::_0)
    }
    #[doc = "LIN break detect is enabled. LIN break character is detected at length of 11 bit times (if M = 0) or 12 (if M = 1) or 13 (M10 = 1)."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDEW::_1)
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
#[doc = "Values that can be written to the field `BRK13`"]
pub enum BRK13W {
    #[doc = "Break character is transmitted with length of 9 to 13 bit times."]
    _0,
    #[doc = "Break character is transmitted with length of 12 to 15 bit times."]
    _1,
}
impl BRK13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BRK13W::_0 => false,
            BRK13W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BRK13W<'a> {
    w: &'a mut W,
}
impl<'a> _BRK13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BRK13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Break character is transmitted with length of 9 to 13 bit times."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRK13W::_0)
    }
    #[doc = "Break character is transmitted with length of 12 to 15 bit times."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRK13W::_1)
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
#[doc = "Values that can be written to the field `RWUID`"]
pub enum RWUIDW {
    #[doc = "During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character. During address match wakeup, the IDLE bit does not set when an address does not match."]
    _0,
    #[doc = "During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character. During address match wakeup, the IDLE bit does set when an address does not match."]
    _1,
}
impl RWUIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWUIDW::_0 => false,
            RWUIDW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWUIDW<'a> {
    w: &'a mut W,
}
impl<'a> _RWUIDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWUIDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character. During address match wakeup, the IDLE bit does not set when an address does not match."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWUIDW::_0)
    }
    #[doc = "During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character. During address match wakeup, the IDLE bit does set when an address does not match."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWUIDW::_1)
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
#[doc = "Values that can be written to the field `RXINV`"]
pub enum RXINVW {
    #[doc = "Receive data not inverted."]
    _0,
    #[doc = "Receive data inverted."]
    _1,
}
impl RXINVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXINVW::_0 => false,
            RXINVW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXINVW<'a> {
    w: &'a mut W,
}
impl<'a> _RXINVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXINVW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive data not inverted."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXINVW::_0)
    }
    #[doc = "Receive data inverted."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXINVW::_1)
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
#[doc = "Values that can be written to the field `MSBF`"]
pub enum MSBFW {
    #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    _0,
    #[doc = "MSB (bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of CTRL[M], CTRL[PE] and BAUD[M10]. Further, the first bit received after the start bit is identified as bit9, bit8, bit7 or bit6 depending on the setting of CTRL[M] and CTRL[PE]."]
    _1,
}
impl MSBFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MSBFW::_0 => false,
            MSBFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSBFW<'a> {
    w: &'a mut W,
}
impl<'a> _MSBFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSBFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSBFW::_0)
    }
    #[doc = "MSB (bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of CTRL[M], CTRL[PE] and BAUD[M10]. Further, the first bit received after the start bit is identified as bit9, bit8, bit7 or bit6 depending on the setting of CTRL[M] and CTRL[PE]."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSBFW::_1)
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
#[doc = "Values that can be written to the field `RXEDGIF`"]
pub enum RXEDGIFW {
    #[doc = "No active edge on the receive pin has occurred."]
    _0,
    #[doc = "An active edge on the receive pin has occurred."]
    _1,
}
impl RXEDGIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXEDGIFW::_0 => false,
            RXEDGIFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXEDGIFW<'a> {
    w: &'a mut W,
}
impl<'a> _RXEDGIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXEDGIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No active edge on the receive pin has occurred."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXEDGIFW::_0)
    }
    #[doc = "An active edge on the receive pin has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXEDGIFW::_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LBKDIF`"]
pub enum LBKDIFW {
    #[doc = "No LIN break character has been detected."]
    _0,
    #[doc = "LIN break character has been detected."]
    _1,
}
impl LBKDIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBKDIFW::_0 => false,
            LBKDIFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBKDIFW<'a> {
    w: &'a mut W,
}
impl<'a> _LBKDIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBKDIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No LIN break character has been detected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDIFW::_0)
    }
    #[doc = "LIN break character has been detected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDIFW::_1)
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
    #[doc = "Bit 14 - Match 2 Flag"]
    #[inline]
    pub fn ma2f(&self) -> MA2FR {
        MA2FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Match 1 Flag"]
    #[inline]
    pub fn ma1f(&self) -> MA1FR {
        MA1FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Parity Error Flag"]
    #[inline]
    pub fn pf(&self) -> PFR {
        PFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Framing Error Flag"]
    #[inline]
    pub fn fe(&self) -> FER {
        FER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Noise Flag"]
    #[inline]
    pub fn nf(&self) -> NFR {
        NFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Receiver Overrun Flag"]
    #[inline]
    pub fn or(&self) -> ORR {
        ORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Idle Line Flag"]
    #[inline]
    pub fn idle(&self) -> IDLER {
        IDLER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Receive Data Register Full Flag"]
    #[inline]
    pub fn rdrf(&self) -> RDRFR {
        RDRFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Transmission Complete Flag"]
    #[inline]
    pub fn tc(&self) -> TCR {
        TCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Transmit Data Register Empty Flag"]
    #[inline]
    pub fn tdre(&self) -> TDRER {
        TDRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Receiver Active Flag"]
    #[inline]
    pub fn raf(&self) -> RAFR {
        RAFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - LIN Break Detection Enable"]
    #[inline]
    pub fn lbkde(&self) -> LBKDER {
        LBKDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Break Character Generation Length"]
    #[inline]
    pub fn brk13(&self) -> BRK13R {
        BRK13R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - Receive Wake Up Idle Detect"]
    #[inline]
    pub fn rwuid(&self) -> RWUIDR {
        RWUIDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - Receive Data Inversion"]
    #[inline]
    pub fn rxinv(&self) -> RXINVR {
        RXINVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 29 - MSB First"]
    #[inline]
    pub fn msbf(&self) -> MSBFR {
        MSBFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - RXD Pin Active Edge Interrupt Flag"]
    #[inline]
    pub fn rxedgif(&self) -> RXEDGIFR {
        RXEDGIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - LIN Break Detect Interrupt Flag"]
    #[inline]
    pub fn lbkdif(&self) -> LBKDIFR {
        LBKDIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 12582912 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 14 - Match 2 Flag"]
    #[inline]
    pub fn ma2f(&mut self) -> _MA2FW {
        _MA2FW { w: self }
    }
    #[doc = "Bit 15 - Match 1 Flag"]
    #[inline]
    pub fn ma1f(&mut self) -> _MA1FW {
        _MA1FW { w: self }
    }
    #[doc = "Bit 16 - Parity Error Flag"]
    #[inline]
    pub fn pf(&mut self) -> _PFW {
        _PFW { w: self }
    }
    #[doc = "Bit 17 - Framing Error Flag"]
    #[inline]
    pub fn fe(&mut self) -> _FEW {
        _FEW { w: self }
    }
    #[doc = "Bit 18 - Noise Flag"]
    #[inline]
    pub fn nf(&mut self) -> _NFW {
        _NFW { w: self }
    }
    #[doc = "Bit 19 - Receiver Overrun Flag"]
    #[inline]
    pub fn or(&mut self) -> _ORW {
        _ORW { w: self }
    }
    #[doc = "Bit 20 - Idle Line Flag"]
    #[inline]
    pub fn idle(&mut self) -> _IDLEW {
        _IDLEW { w: self }
    }
    #[doc = "Bit 25 - LIN Break Detection Enable"]
    #[inline]
    pub fn lbkde(&mut self) -> _LBKDEW {
        _LBKDEW { w: self }
    }
    #[doc = "Bit 26 - Break Character Generation Length"]
    #[inline]
    pub fn brk13(&mut self) -> _BRK13W {
        _BRK13W { w: self }
    }
    #[doc = "Bit 27 - Receive Wake Up Idle Detect"]
    #[inline]
    pub fn rwuid(&mut self) -> _RWUIDW {
        _RWUIDW { w: self }
    }
    #[doc = "Bit 28 - Receive Data Inversion"]
    #[inline]
    pub fn rxinv(&mut self) -> _RXINVW {
        _RXINVW { w: self }
    }
    #[doc = "Bit 29 - MSB First"]
    #[inline]
    pub fn msbf(&mut self) -> _MSBFW {
        _MSBFW { w: self }
    }
    #[doc = "Bit 30 - RXD Pin Active Edge Interrupt Flag"]
    #[inline]
    pub fn rxedgif(&mut self) -> _RXEDGIFW {
        _RXEDGIFW { w: self }
    }
    #[doc = "Bit 31 - LIN Break Detect Interrupt Flag"]
    #[inline]
    pub fn lbkdif(&mut self) -> _LBKDIFW {
        _LBKDIFW { w: self }
    }
}
