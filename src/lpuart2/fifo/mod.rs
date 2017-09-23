#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFO {
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
#[doc = "Possible values of the field `RXFIFOSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFIFOSIZER {
    #[doc = "Receive FIFO/Buffer depth = 1 dataword."]
    _000,
    #[doc = "Receive FIFO/Buffer depth = 4 datawords."]
    _001,
    #[doc = "Receive FIFO/Buffer depth = 8 datawords."]
    _010,
    #[doc = "Receive FIFO/Buffer depth = 16 datawords."]
    _011,
    #[doc = "Receive FIFO/Buffer depth = 32 datawords."]
    _100,
    #[doc = "Receive FIFO/Buffer depth = 64 datawords."]
    _101,
    #[doc = "Receive FIFO/Buffer depth = 128 datawords."]
    _110,
    #[doc = "Receive FIFO/Buffer depth = 256 datawords."]
    _111,
}
impl RXFIFOSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXFIFOSIZER::_000 => 0,
            RXFIFOSIZER::_001 => 1,
            RXFIFOSIZER::_010 => 2,
            RXFIFOSIZER::_011 => 3,
            RXFIFOSIZER::_100 => 4,
            RXFIFOSIZER::_101 => 5,
            RXFIFOSIZER::_110 => 6,
            RXFIFOSIZER::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXFIFOSIZER {
        match value {
            0 => RXFIFOSIZER::_000,
            1 => RXFIFOSIZER::_001,
            2 => RXFIFOSIZER::_010,
            3 => RXFIFOSIZER::_011,
            4 => RXFIFOSIZER::_100,
            5 => RXFIFOSIZER::_101,
            6 => RXFIFOSIZER::_110,
            7 => RXFIFOSIZER::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == RXFIFOSIZER::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == RXFIFOSIZER::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == RXFIFOSIZER::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == RXFIFOSIZER::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == RXFIFOSIZER::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == RXFIFOSIZER::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == RXFIFOSIZER::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == RXFIFOSIZER::_111
    }
}
#[doc = "Possible values of the field `RXFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXFER {
    #[doc = "Receive FIFO is not enabled. Buffer is depth 1. (Legacy support)"]
    _0,
    #[doc = "Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE."]
    _1,
}
impl RXFER {
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
            RXFER::_0 => false,
            RXFER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXFER {
        match value {
            false => RXFER::_0,
            true => RXFER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXFER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXFER::_1
    }
}
#[doc = "Possible values of the field `TXFIFOSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIFOSIZER {
    #[doc = "Transmit FIFO/Buffer depth = 1 dataword."]
    _000,
    #[doc = "Transmit FIFO/Buffer depth = 4 datawords."]
    _001,
    #[doc = "Transmit FIFO/Buffer depth = 8 datawords."]
    _010,
    #[doc = "Transmit FIFO/Buffer depth = 16 datawords."]
    _011,
    #[doc = "Transmit FIFO/Buffer depth = 32 datawords."]
    _100,
    #[doc = "Transmit FIFO/Buffer depth = 64 datawords."]
    _101,
    #[doc = "Transmit FIFO/Buffer depth = 128 datawords."]
    _110,
    #[doc = "Transmit FIFO/Buffer depth = 256 datawords"]
    _111,
}
impl TXFIFOSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXFIFOSIZER::_000 => 0,
            TXFIFOSIZER::_001 => 1,
            TXFIFOSIZER::_010 => 2,
            TXFIFOSIZER::_011 => 3,
            TXFIFOSIZER::_100 => 4,
            TXFIFOSIZER::_101 => 5,
            TXFIFOSIZER::_110 => 6,
            TXFIFOSIZER::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXFIFOSIZER {
        match value {
            0 => TXFIFOSIZER::_000,
            1 => TXFIFOSIZER::_001,
            2 => TXFIFOSIZER::_010,
            3 => TXFIFOSIZER::_011,
            4 => TXFIFOSIZER::_100,
            5 => TXFIFOSIZER::_101,
            6 => TXFIFOSIZER::_110,
            7 => TXFIFOSIZER::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == TXFIFOSIZER::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == TXFIFOSIZER::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == TXFIFOSIZER::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == TXFIFOSIZER::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == TXFIFOSIZER::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == TXFIFOSIZER::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == TXFIFOSIZER::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == TXFIFOSIZER::_111
    }
}
#[doc = "Possible values of the field `TXFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFER {
    #[doc = "Transmit FIFO is not enabled. Buffer is depth 1. (Legacy support)."]
    _0,
    #[doc = "Transmit FIFO is enabled. Buffer is depth indicated by TXFIFOSIZE."]
    _1,
}
impl TXFER {
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
            TXFER::_0 => false,
            TXFER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXFER {
        match value {
            false => TXFER::_0,
            true => TXFER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXFER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXFER::_1
    }
}
#[doc = "Possible values of the field `RXUFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXUFER {
    #[doc = "RXUF flag does not generate an interrupt to the host."]
    _0,
    #[doc = "RXUF flag generates an interrupt to the host."]
    _1,
}
impl RXUFER {
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
            RXUFER::_0 => false,
            RXUFER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXUFER {
        match value {
            false => RXUFER::_0,
            true => RXUFER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXUFER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXUFER::_1
    }
}
#[doc = "Possible values of the field `TXOFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOFER {
    #[doc = "TXOF flag does not generate an interrupt to the host."]
    _0,
    #[doc = "TXOF flag generates an interrupt to the host."]
    _1,
}
impl TXOFER {
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
            TXOFER::_0 => false,
            TXOFER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXOFER {
        match value {
            false => TXOFER::_0,
            true => TXOFER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXOFER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXOFER::_1
    }
}
#[doc = "Possible values of the field `RXIDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXIDENR {
    #[doc = "Disable RDRF assertion due to partially filled FIFO when receiver is idle."]
    _000,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 1 character."]
    _001,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 2 characters."]
    _010,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 4 characters."]
    _011,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 8 characters."]
    _100,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 16 characters."]
    _101,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 32 characters."]
    _110,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 64 characters."]
    _111,
}
impl RXIDENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RXIDENR::_000 => 0,
            RXIDENR::_001 => 1,
            RXIDENR::_010 => 2,
            RXIDENR::_011 => 3,
            RXIDENR::_100 => 4,
            RXIDENR::_101 => 5,
            RXIDENR::_110 => 6,
            RXIDENR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RXIDENR {
        match value {
            0 => RXIDENR::_000,
            1 => RXIDENR::_001,
            2 => RXIDENR::_010,
            3 => RXIDENR::_011,
            4 => RXIDENR::_100,
            5 => RXIDENR::_101,
            6 => RXIDENR::_110,
            7 => RXIDENR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == RXIDENR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == RXIDENR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == RXIDENR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == RXIDENR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == RXIDENR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == RXIDENR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == RXIDENR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == RXIDENR::_111
    }
}
#[doc = "Possible values of the field `RXUF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXUFR {
    #[doc = "No receive buffer underflow has occurred since the last time the flag was cleared."]
    _0,
    #[doc = "At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    _1,
}
impl RXUFR {
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
            RXUFR::_0 => false,
            RXUFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXUFR {
        match value {
            false => RXUFR::_0,
            true => RXUFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXUFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXUFR::_1
    }
}
#[doc = "Possible values of the field `TXOF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXOFR {
    #[doc = "No transmit buffer overflow has occurred since the last time the flag was cleared."]
    _0,
    #[doc = "At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    _1,
}
impl TXOFR {
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
            TXOFR::_0 => false,
            TXOFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXOFR {
        match value {
            false => TXOFR::_0,
            true => TXOFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXOFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXOFR::_1
    }
}
#[doc = "Possible values of the field `RXEMPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEMPTR {
    #[doc = "Receive buffer is not empty."]
    _0,
    #[doc = "Receive buffer is empty."]
    _1,
}
impl RXEMPTR {
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
            RXEMPTR::_0 => false,
            RXEMPTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXEMPTR {
        match value {
            false => RXEMPTR::_0,
            true => RXEMPTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXEMPTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXEMPTR::_1
    }
}
#[doc = "Possible values of the field `TXEMPT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEMPTR {
    #[doc = "Transmit buffer is not empty."]
    _0,
    #[doc = "Transmit buffer is empty."]
    _1,
}
impl TXEMPTR {
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
            TXEMPTR::_0 => false,
            TXEMPTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXEMPTR {
        match value {
            false => TXEMPTR::_0,
            true => TXEMPTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXEMPTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXEMPTR::_1
    }
}
#[doc = "Values that can be written to the field `RXFE`"]
pub enum RXFEW {
    #[doc = "Receive FIFO is not enabled. Buffer is depth 1. (Legacy support)"]
    _0,
    #[doc = "Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE."]
    _1,
}
impl RXFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFEW::_0 => false,
            RXFEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive FIFO is not enabled. Buffer is depth 1. (Legacy support)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXFEW::_0)
    }
    #[doc = "Receive FIFO is enabled. Buffer is depth indicted by RXFIFOSIZE."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXFEW::_1)
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
#[doc = "Values that can be written to the field `TXFE`"]
pub enum TXFEW {
    #[doc = "Transmit FIFO is not enabled. Buffer is depth 1. (Legacy support)."]
    _0,
    #[doc = "Transmit FIFO is enabled. Buffer is depth indicated by TXFIFOSIZE."]
    _1,
}
impl TXFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFEW::_0 => false,
            TXFEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit FIFO is not enabled. Buffer is depth 1. (Legacy support)."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXFEW::_0)
    }
    #[doc = "Transmit FIFO is enabled. Buffer is depth indicated by TXFIFOSIZE."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXFEW::_1)
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
#[doc = "Values that can be written to the field `RXUFE`"]
pub enum RXUFEW {
    #[doc = "RXUF flag does not generate an interrupt to the host."]
    _0,
    #[doc = "RXUF flag generates an interrupt to the host."]
    _1,
}
impl RXUFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXUFEW::_0 => false,
            RXUFEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXUFEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXUFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXUFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "RXUF flag does not generate an interrupt to the host."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXUFEW::_0)
    }
    #[doc = "RXUF flag generates an interrupt to the host."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXUFEW::_1)
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
#[doc = "Values that can be written to the field `TXOFE`"]
pub enum TXOFEW {
    #[doc = "TXOF flag does not generate an interrupt to the host."]
    _0,
    #[doc = "TXOF flag generates an interrupt to the host."]
    _1,
}
impl TXOFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXOFEW::_0 => false,
            TXOFEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXOFEW<'a> {
    w: &'a mut W,
}
impl<'a> _TXOFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXOFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TXOF flag does not generate an interrupt to the host."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXOFEW::_0)
    }
    #[doc = "TXOF flag generates an interrupt to the host."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXOFEW::_1)
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
#[doc = "Values that can be written to the field `RXIDEN`"]
pub enum RXIDENW {
    #[doc = "Disable RDRF assertion due to partially filled FIFO when receiver is idle."]
    _000,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 1 character."]
    _001,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 2 characters."]
    _010,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 4 characters."]
    _011,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 8 characters."]
    _100,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 16 characters."]
    _101,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 32 characters."]
    _110,
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 64 characters."]
    _111,
}
impl RXIDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RXIDENW::_000 => 0,
            RXIDENW::_001 => 1,
            RXIDENW::_010 => 2,
            RXIDENW::_011 => 3,
            RXIDENW::_100 => 4,
            RXIDENW::_101 => 5,
            RXIDENW::_110 => 6,
            RXIDENW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXIDENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXIDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXIDENW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Disable RDRF assertion due to partially filled FIFO when receiver is idle."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(RXIDENW::_000)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 1 character."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(RXIDENW::_001)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 2 characters."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(RXIDENW::_010)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 4 characters."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(RXIDENW::_011)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 8 characters."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(RXIDENW::_100)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 16 characters."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(RXIDENW::_101)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 32 characters."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(RXIDENW::_110)
    }
    #[doc = "Enable RDRF assertion due to partially filled FIFO when receiver is idle for 64 characters."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(RXIDENW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXFLUSH`"]
pub enum RXFLUSHW {
    #[doc = "No flush operation occurs."]
    _0,
    #[doc = "All data in the receive FIFO/buffer is cleared out."]
    _1,
}
impl RXFLUSHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXFLUSHW::_0 => false,
            RXFLUSHW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXFLUSHW<'a> {
    w: &'a mut W,
}
impl<'a> _RXFLUSHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXFLUSHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No flush operation occurs."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXFLUSHW::_0)
    }
    #[doc = "All data in the receive FIFO/buffer is cleared out."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXFLUSHW::_1)
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
#[doc = "Values that can be written to the field `TXFLUSH`"]
pub enum TXFLUSHW {
    #[doc = "No flush operation occurs."]
    _0,
    #[doc = "All data in the transmit FIFO/Buffer is cleared out."]
    _1,
}
impl TXFLUSHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXFLUSHW::_0 => false,
            TXFLUSHW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXFLUSHW<'a> {
    w: &'a mut W,
}
impl<'a> _TXFLUSHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXFLUSHW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No flush operation occurs."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXFLUSHW::_0)
    }
    #[doc = "All data in the transmit FIFO/Buffer is cleared out."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXFLUSHW::_1)
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
#[doc = "Values that can be written to the field `RXUF`"]
pub enum RXUFW {
    #[doc = "No receive buffer underflow has occurred since the last time the flag was cleared."]
    _0,
    #[doc = "At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    _1,
}
impl RXUFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXUFW::_0 => false,
            RXUFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXUFW<'a> {
    w: &'a mut W,
}
impl<'a> _RXUFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXUFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No receive buffer underflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXUFW::_0)
    }
    #[doc = "At least one receive buffer underflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXUFW::_1)
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
#[doc = "Values that can be written to the field `TXOF`"]
pub enum TXOFW {
    #[doc = "No transmit buffer overflow has occurred since the last time the flag was cleared."]
    _0,
    #[doc = "At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    _1,
}
impl TXOFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXOFW::_0 => false,
            TXOFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXOFW<'a> {
    w: &'a mut W,
}
impl<'a> _TXOFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXOFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No transmit buffer overflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXOFW::_0)
    }
    #[doc = "At least one transmit buffer overflow has occurred since the last time the flag was cleared."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXOFW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Receive FIFO. Buffer Depth"]
    #[inline]
    pub fn rxfifosize(&self) -> RXFIFOSIZER {
        RXFIFOSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Receive FIFO Enable"]
    #[inline]
    pub fn rxfe(&self) -> RXFER {
        RXFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:6 - Transmit FIFO. Buffer Depth"]
    #[inline]
    pub fn txfifosize(&self) -> TXFIFOSIZER {
        TXFIFOSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Transmit FIFO Enable"]
    #[inline]
    pub fn txfe(&self) -> TXFER {
        TXFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Receive FIFO Underflow Interrupt Enable"]
    #[inline]
    pub fn rxufe(&self) -> RXUFER {
        RXUFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Transmit FIFO Overflow Interrupt Enable"]
    #[inline]
    pub fn txofe(&self) -> TXOFER {
        TXOFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 10:12 - Receiver Idle Empty Enable"]
    #[inline]
    pub fn rxiden(&self) -> RXIDENR {
        RXIDENR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Receiver Buffer Underflow Flag"]
    #[inline]
    pub fn rxuf(&self) -> RXUFR {
        RXUFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Transmitter Buffer Overflow Flag"]
    #[inline]
    pub fn txof(&self) -> TXOFR {
        TXOFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Receive Buffer/FIFO Empty"]
    #[inline]
    pub fn rxempt(&self) -> RXEMPTR {
        RXEMPTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Transmit Buffer/FIFO Empty"]
    #[inline]
    pub fn txempt(&self) -> TXEMPTR {
        TXEMPTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 12582929 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 3 - Receive FIFO Enable"]
    #[inline]
    pub fn rxfe(&mut self) -> _RXFEW {
        _RXFEW { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO Enable"]
    #[inline]
    pub fn txfe(&mut self) -> _TXFEW {
        _TXFEW { w: self }
    }
    #[doc = "Bit 8 - Receive FIFO Underflow Interrupt Enable"]
    #[inline]
    pub fn rxufe(&mut self) -> _RXUFEW {
        _RXUFEW { w: self }
    }
    #[doc = "Bit 9 - Transmit FIFO Overflow Interrupt Enable"]
    #[inline]
    pub fn txofe(&mut self) -> _TXOFEW {
        _TXOFEW { w: self }
    }
    #[doc = "Bits 10:12 - Receiver Idle Empty Enable"]
    #[inline]
    pub fn rxiden(&mut self) -> _RXIDENW {
        _RXIDENW { w: self }
    }
    #[doc = "Bit 14 - Receive FIFO/Buffer Flush"]
    #[inline]
    pub fn rxflush(&mut self) -> _RXFLUSHW {
        _RXFLUSHW { w: self }
    }
    #[doc = "Bit 15 - Transmit FIFO/Buffer Flush"]
    #[inline]
    pub fn txflush(&mut self) -> _TXFLUSHW {
        _TXFLUSHW { w: self }
    }
    #[doc = "Bit 16 - Receiver Buffer Underflow Flag"]
    #[inline]
    pub fn rxuf(&mut self) -> _RXUFW {
        _RXUFW { w: self }
    }
    #[doc = "Bit 17 - Transmitter Buffer Overflow Flag"]
    #[inline]
    pub fn txof(&mut self) -> _TXOFW {
        _TXOFW { w: self }
    }
}
