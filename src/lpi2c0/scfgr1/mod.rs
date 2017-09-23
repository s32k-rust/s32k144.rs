#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCFGR1 {
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
#[doc = "Possible values of the field `ADRSTALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADRSTALLR {
    #[doc = "Clock stretching disabled."] _0,
    #[doc = "Clock stretching enabled."] _1,
}
impl ADRSTALLR {
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
            ADRSTALLR::_0 => false,
            ADRSTALLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADRSTALLR {
        match value {
            false => ADRSTALLR::_0,
            true => ADRSTALLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADRSTALLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADRSTALLR::_1
    }
}
#[doc = "Possible values of the field `RXSTALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXSTALLR {
    #[doc = "Clock stretching disabled."] _0,
    #[doc = "Clock stretching enabled."] _1,
}
impl RXSTALLR {
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
            RXSTALLR::_0 => false,
            RXSTALLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXSTALLR {
        match value {
            false => RXSTALLR::_0,
            true => RXSTALLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXSTALLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXSTALLR::_1
    }
}
#[doc = "Possible values of the field `TXDSTALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDSTALLR {
    #[doc = "Clock stretching disabled."] _0,
    #[doc = "Clock stretching enabled."] _1,
}
impl TXDSTALLR {
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
            TXDSTALLR::_0 => false,
            TXDSTALLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXDSTALLR {
        match value {
            false => TXDSTALLR::_0,
            true => TXDSTALLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXDSTALLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXDSTALLR::_1
    }
}
#[doc = "Possible values of the field `ACKSTALL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACKSTALLR {
    #[doc = "Clock stretching disabled."] _0,
    #[doc = "Clock stretching enabled."] _1,
}
impl ACKSTALLR {
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
            ACKSTALLR::_0 => false,
            ACKSTALLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACKSTALLR {
        match value {
            false => ACKSTALLR::_0,
            true => ACKSTALLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ACKSTALLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ACKSTALLR::_1
    }
}
#[doc = "Possible values of the field `GCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCENR {
    #[doc = "General Call address is disabled."] _0,
    #[doc = "General call address is enabled."] _1,
}
impl GCENR {
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
            GCENR::_0 => false,
            GCENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GCENR {
        match value {
            false => GCENR::_0,
            true => GCENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GCENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GCENR::_1
    }
}
#[doc = "Possible values of the field `SAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAENR {
    #[doc = "Disables match on SMBus Alert."] _0,
    #[doc = "Enables match on SMBus Alert."] _1,
}
impl SAENR {
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
            SAENR::_0 => false,
            SAENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SAENR {
        match value {
            false => SAENR::_0,
            true => SAENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SAENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SAENR::_1
    }
}
#[doc = "Possible values of the field `TXCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXCFGR {
    #[doc = "Transmit Data Flag will only assert during a slave-transmit transfer when the transmit data register is empty."]
    _0,
    #[doc = "Transmit Data Flag will assert whenever the transmit data register is empty."] _1,
}
impl TXCFGR {
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
            TXCFGR::_0 => false,
            TXCFGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXCFGR {
        match value {
            false => TXCFGR::_0,
            true => TXCFGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXCFGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXCFGR::_1
    }
}
#[doc = "Possible values of the field `RXCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXCFGR {
    #[doc = "Reading the receive data register will return receive data and clear the receive data flag."]
    _0,
    #[doc = "Reading the receive data register when the address valid flag is set will return the address status register and clear the address valid flag. Reading the receive data register when the address valid flag is clear will return receive data and clear the receive data flag."]
    _1,
}
impl RXCFGR {
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
            RXCFGR::_0 => false,
            RXCFGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXCFGR {
        match value {
            false => RXCFGR::_0,
            true => RXCFGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXCFGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXCFGR::_1
    }
}
#[doc = "Possible values of the field `IGNACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IGNACKR {
    #[doc = "Slave will end transfer when NACK detected."] _0,
    #[doc = "Slave will not end transfer when NACK detected."] _1,
}
impl IGNACKR {
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
            IGNACKR::_0 => false,
            IGNACKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IGNACKR {
        match value {
            false => IGNACKR::_0,
            true => IGNACKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IGNACKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IGNACKR::_1
    }
}
#[doc = "Possible values of the field `HSMEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSMENR {
    #[doc = "Disables detection of Hs-mode master code."] _0,
    #[doc = "Enables detection of Hs-mode master code."] _1,
}
impl HSMENR {
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
            HSMENR::_0 => false,
            HSMENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HSMENR {
        match value {
            false => HSMENR::_0,
            true => HSMENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HSMENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HSMENR::_1
    }
}
#[doc = "Possible values of the field `ADDRCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRCFGR {
    #[doc = "Address match 0 (7-bit)."] _000,
    #[doc = "Address match 0 (10-bit)."] _001,
    #[doc = "Address match 0 (7-bit) or Address match 1 (7-bit)."] _010,
    #[doc = "Address match 0 (10-bit) or Address match 1 (10-bit)."] _011,
    #[doc = "Address match 0 (7-bit) or Address match 1 (10-bit)."] _100,
    #[doc = "Address match 0 (10-bit) or Address match 1 (7-bit)."] _101,
    #[doc = "From Address match 0 (7-bit) to Address match 1 (7-bit)."] _110,
    #[doc = "From Address match 0 (10-bit) to Address match 1 (10-bit)."] _111,
}
impl ADDRCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADDRCFGR::_000 => 0,
            ADDRCFGR::_001 => 1,
            ADDRCFGR::_010 => 2,
            ADDRCFGR::_011 => 3,
            ADDRCFGR::_100 => 4,
            ADDRCFGR::_101 => 5,
            ADDRCFGR::_110 => 6,
            ADDRCFGR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADDRCFGR {
        match value {
            0 => ADDRCFGR::_000,
            1 => ADDRCFGR::_001,
            2 => ADDRCFGR::_010,
            3 => ADDRCFGR::_011,
            4 => ADDRCFGR::_100,
            5 => ADDRCFGR::_101,
            6 => ADDRCFGR::_110,
            7 => ADDRCFGR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == ADDRCFGR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == ADDRCFGR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == ADDRCFGR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == ADDRCFGR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == ADDRCFGR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == ADDRCFGR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == ADDRCFGR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == ADDRCFGR::_111
    }
}
#[doc = "Values that can be written to the field `ADRSTALL`"]
pub enum ADRSTALLW {
    #[doc = "Clock stretching disabled."] _0,
    #[doc = "Clock stretching enabled."] _1,
}
impl ADRSTALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADRSTALLW::_0 => false,
            ADRSTALLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADRSTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _ADRSTALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADRSTALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock stretching disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADRSTALLW::_0)
    }
    #[doc = "Clock stretching enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADRSTALLW::_1)
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
#[doc = "Values that can be written to the field `RXSTALL`"]
pub enum RXSTALLW {
    #[doc = "Clock stretching disabled."] _0,
    #[doc = "Clock stretching enabled."] _1,
}
impl RXSTALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXSTALLW::_0 => false,
            RXSTALLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXSTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _RXSTALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXSTALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock stretching disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXSTALLW::_0)
    }
    #[doc = "Clock stretching enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXSTALLW::_1)
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
#[doc = "Values that can be written to the field `TXDSTALL`"]
pub enum TXDSTALLW {
    #[doc = "Clock stretching disabled."] _0,
    #[doc = "Clock stretching enabled."] _1,
}
impl TXDSTALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXDSTALLW::_0 => false,
            TXDSTALLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXDSTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _TXDSTALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXDSTALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock stretching disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXDSTALLW::_0)
    }
    #[doc = "Clock stretching enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXDSTALLW::_1)
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
#[doc = "Values that can be written to the field `ACKSTALL`"]
pub enum ACKSTALLW {
    #[doc = "Clock stretching disabled."] _0,
    #[doc = "Clock stretching enabled."] _1,
}
impl ACKSTALLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACKSTALLW::_0 => false,
            ACKSTALLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACKSTALLW<'a> {
    w: &'a mut W,
}
impl<'a> _ACKSTALLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACKSTALLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock stretching disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACKSTALLW::_0)
    }
    #[doc = "Clock stretching enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACKSTALLW::_1)
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
#[doc = "Values that can be written to the field `GCEN`"]
pub enum GCENW {
    #[doc = "General Call address is disabled."] _0,
    #[doc = "General call address is enabled."] _1,
}
impl GCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            GCENW::_0 => false,
            GCENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _GCENW<'a> {
    w: &'a mut W,
}
impl<'a> _GCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: GCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "General Call address is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(GCENW::_0)
    }
    #[doc = "General call address is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(GCENW::_1)
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
#[doc = "Values that can be written to the field `SAEN`"]
pub enum SAENW {
    #[doc = "Disables match on SMBus Alert."] _0,
    #[doc = "Enables match on SMBus Alert."] _1,
}
impl SAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SAENW::_0 => false,
            SAENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAENW<'a> {
    w: &'a mut W,
}
impl<'a> _SAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables match on SMBus Alert."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAENW::_0)
    }
    #[doc = "Enables match on SMBus Alert."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAENW::_1)
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
#[doc = "Values that can be written to the field `TXCFG`"]
pub enum TXCFGW {
    #[doc = "Transmit Data Flag will only assert during a slave-transmit transfer when the transmit data register is empty."]
    _0,
    #[doc = "Transmit Data Flag will assert whenever the transmit data register is empty."] _1,
}
impl TXCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXCFGW::_0 => false,
            TXCFGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _TXCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit Data Flag will only assert during a slave-transmit transfer when the transmit data register is empty."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXCFGW::_0)
    }
    #[doc = "Transmit Data Flag will assert whenever the transmit data register is empty."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXCFGW::_1)
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
#[doc = "Values that can be written to the field `RXCFG`"]
pub enum RXCFGW {
    #[doc = "Reading the receive data register will return receive data and clear the receive data flag."]
    _0,
    #[doc = "Reading the receive data register when the address valid flag is set will return the address status register and clear the address valid flag. Reading the receive data register when the address valid flag is clear will return receive data and clear the receive data flag."]
    _1,
}
impl RXCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXCFGW::_0 => false,
            RXCFGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXCFGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reading the receive data register will return receive data and clear the receive data flag."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXCFGW::_0)
    }
    #[doc = "Reading the receive data register when the address valid flag is set will return the address status register and clear the address valid flag. Reading the receive data register when the address valid flag is clear will return receive data and clear the receive data flag."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXCFGW::_1)
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
#[doc = "Values that can be written to the field `IGNACK`"]
pub enum IGNACKW {
    #[doc = "Slave will end transfer when NACK detected."] _0,
    #[doc = "Slave will not end transfer when NACK detected."] _1,
}
impl IGNACKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IGNACKW::_0 => false,
            IGNACKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IGNACKW<'a> {
    w: &'a mut W,
}
impl<'a> _IGNACKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IGNACKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave will end transfer when NACK detected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IGNACKW::_0)
    }
    #[doc = "Slave will not end transfer when NACK detected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IGNACKW::_1)
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
#[doc = "Values that can be written to the field `HSMEN`"]
pub enum HSMENW {
    #[doc = "Disables detection of Hs-mode master code."] _0,
    #[doc = "Enables detection of Hs-mode master code."] _1,
}
impl HSMENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HSMENW::_0 => false,
            HSMENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HSMENW<'a> {
    w: &'a mut W,
}
impl<'a> _HSMENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HSMENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disables detection of Hs-mode master code."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HSMENW::_0)
    }
    #[doc = "Enables detection of Hs-mode master code."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HSMENW::_1)
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
#[doc = "Values that can be written to the field `ADDRCFG`"]
pub enum ADDRCFGW {
    #[doc = "Address match 0 (7-bit)."] _000,
    #[doc = "Address match 0 (10-bit)."] _001,
    #[doc = "Address match 0 (7-bit) or Address match 1 (7-bit)."] _010,
    #[doc = "Address match 0 (10-bit) or Address match 1 (10-bit)."] _011,
    #[doc = "Address match 0 (7-bit) or Address match 1 (10-bit)."] _100,
    #[doc = "Address match 0 (10-bit) or Address match 1 (7-bit)."] _101,
    #[doc = "From Address match 0 (7-bit) to Address match 1 (7-bit)."] _110,
    #[doc = "From Address match 0 (10-bit) to Address match 1 (10-bit)."] _111,
}
impl ADDRCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADDRCFGW::_000 => 0,
            ADDRCFGW::_001 => 1,
            ADDRCFGW::_010 => 2,
            ADDRCFGW::_011 => 3,
            ADDRCFGW::_100 => 4,
            ADDRCFGW::_101 => 5,
            ADDRCFGW::_110 => 6,
            ADDRCFGW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADDRCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADDRCFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Address match 0 (7-bit)."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(ADDRCFGW::_000)
    }
    #[doc = "Address match 0 (10-bit)."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(ADDRCFGW::_001)
    }
    #[doc = "Address match 0 (7-bit) or Address match 1 (7-bit)."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(ADDRCFGW::_010)
    }
    #[doc = "Address match 0 (10-bit) or Address match 1 (10-bit)."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(ADDRCFGW::_011)
    }
    #[doc = "Address match 0 (7-bit) or Address match 1 (10-bit)."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(ADDRCFGW::_100)
    }
    #[doc = "Address match 0 (10-bit) or Address match 1 (7-bit)."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(ADDRCFGW::_101)
    }
    #[doc = "From Address match 0 (7-bit) to Address match 1 (7-bit)."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(ADDRCFGW::_110)
    }
    #[doc = "From Address match 0 (10-bit) to Address match 1 (10-bit)."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(ADDRCFGW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 0 - Address SCL Stall"]
    #[inline]
    pub fn adrstall(&self) -> ADRSTALLR {
        ADRSTALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - RX SCL Stall"]
    #[inline]
    pub fn rxstall(&self) -> RXSTALLR {
        RXSTALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - TX Data SCL Stall"]
    #[inline]
    pub fn txdstall(&self) -> TXDSTALLR {
        TXDSTALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - ACK SCL Stall"]
    #[inline]
    pub fn ackstall(&self) -> ACKSTALLR {
        ACKSTALLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - General Call Enable"]
    #[inline]
    pub fn gcen(&self) -> GCENR {
        GCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - SMBus Alert Enable"]
    #[inline]
    pub fn saen(&self) -> SAENR {
        SAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Transmit Flag Configuration"]
    #[inline]
    pub fn txcfg(&self) -> TXCFGR {
        TXCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Receive Data Configuration"]
    #[inline]
    pub fn rxcfg(&self) -> RXCFGR {
        RXCFGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Ignore NACK"]
    #[inline]
    pub fn ignack(&self) -> IGNACKR {
        IGNACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - High Speed Mode Enable"]
    #[inline]
    pub fn hsmen(&self) -> HSMENR {
        HSMENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:18 - Address Configuration"]
    #[inline]
    pub fn addrcfg(&self) -> ADDRCFGR {
        ADDRCFGR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 0 - Address SCL Stall"]
    #[inline]
    pub fn adrstall(&mut self) -> _ADRSTALLW {
        _ADRSTALLW { w: self }
    }
    #[doc = "Bit 1 - RX SCL Stall"]
    #[inline]
    pub fn rxstall(&mut self) -> _RXSTALLW {
        _RXSTALLW { w: self }
    }
    #[doc = "Bit 2 - TX Data SCL Stall"]
    #[inline]
    pub fn txdstall(&mut self) -> _TXDSTALLW {
        _TXDSTALLW { w: self }
    }
    #[doc = "Bit 3 - ACK SCL Stall"]
    #[inline]
    pub fn ackstall(&mut self) -> _ACKSTALLW {
        _ACKSTALLW { w: self }
    }
    #[doc = "Bit 8 - General Call Enable"]
    #[inline]
    pub fn gcen(&mut self) -> _GCENW {
        _GCENW { w: self }
    }
    #[doc = "Bit 9 - SMBus Alert Enable"]
    #[inline]
    pub fn saen(&mut self) -> _SAENW {
        _SAENW { w: self }
    }
    #[doc = "Bit 10 - Transmit Flag Configuration"]
    #[inline]
    pub fn txcfg(&mut self) -> _TXCFGW {
        _TXCFGW { w: self }
    }
    #[doc = "Bit 11 - Receive Data Configuration"]
    #[inline]
    pub fn rxcfg(&mut self) -> _RXCFGW {
        _RXCFGW { w: self }
    }
    #[doc = "Bit 12 - Ignore NACK"]
    #[inline]
    pub fn ignack(&mut self) -> _IGNACKW {
        _IGNACKW { w: self }
    }
    #[doc = "Bit 13 - High Speed Mode Enable"]
    #[inline]
    pub fn hsmen(&mut self) -> _HSMENW {
        _HSMENW { w: self }
    }
    #[doc = "Bits 16:18 - Address Configuration"]
    #[inline]
    pub fn addrcfg(&mut self) -> _ADDRCFGW {
        _ADDRCFGW { w: self }
    }
}
