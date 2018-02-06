#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SSR {
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
#[doc = "Possible values of the field `TDF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDFR {
    #[doc = "Transmit data not requested."]
    _0,
    #[doc = "Transmit data is requested."]
    _1,
}
impl TDFR {
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
            TDFR::_0 => false,
            TDFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDFR {
        match value {
            false => TDFR::_0,
            true => TDFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TDFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TDFR::_1
    }
}
#[doc = "Possible values of the field `RDF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDFR {
    #[doc = "Receive Data is not ready."]
    _0,
    #[doc = "Receive data is ready."]
    _1,
}
impl RDFR {
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
            RDFR::_0 => false,
            RDFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDFR {
        match value {
            false => RDFR::_0,
            true => RDFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RDFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RDFR::_1
    }
}
#[doc = "Possible values of the field `AVF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AVFR {
    #[doc = "Address Status Register is not valid."]
    _0,
    #[doc = "Address Status Register is valid."]
    _1,
}
impl AVFR {
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
            AVFR::_0 => false,
            AVFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AVFR {
        match value {
            false => AVFR::_0,
            true => AVFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AVFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AVFR::_1
    }
}
#[doc = "Possible values of the field `TAF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAFR {
    #[doc = "Transmit ACK/NACK is not required."]
    _0,
    #[doc = "Transmit ACK/NACK is required."]
    _1,
}
impl TAFR {
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
            TAFR::_0 => false,
            TAFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TAFR {
        match value {
            false => TAFR::_0,
            true => TAFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TAFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TAFR::_1
    }
}
#[doc = "Possible values of the field `RSF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSFR {
    #[doc = "Slave has not detected a Repeated START condition."]
    _0,
    #[doc = "Slave has detected a Repeated START condition."]
    _1,
}
impl RSFR {
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
            RSFR::_0 => false,
            RSFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSFR {
        match value {
            false => RSFR::_0,
            true => RSFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RSFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RSFR::_1
    }
}
#[doc = "Possible values of the field `SDF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDFR {
    #[doc = "Slave has not detected a STOP condition."]
    _0,
    #[doc = "Slave has detected a STOP condition."]
    _1,
}
impl SDFR {
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
            SDFR::_0 => false,
            SDFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SDFR {
        match value {
            false => SDFR::_0,
            true => SDFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SDFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SDFR::_1
    }
}
#[doc = "Possible values of the field `BEF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BEFR {
    #[doc = "Slave has not detected a bit error."]
    _0,
    #[doc = "Slave has detected a bit error."]
    _1,
}
impl BEFR {
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
            BEFR::_0 => false,
            BEFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BEFR {
        match value {
            false => BEFR::_0,
            true => BEFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BEFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BEFR::_1
    }
}
#[doc = "Possible values of the field `FEF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEFR {
    #[doc = "FIFO underflow or overflow not detected."]
    _0,
    #[doc = "FIFO underflow or overflow detected."]
    _1,
}
impl FEFR {
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
            FEFR::_0 => false,
            FEFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FEFR {
        match value {
            false => FEFR::_0,
            true => FEFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FEFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FEFR::_1
    }
}
#[doc = "Possible values of the field `AM0F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AM0FR {
    #[doc = "Have not received ADDR0 matching address."]
    _0,
    #[doc = "Have received ADDR0 matching address."]
    _1,
}
impl AM0FR {
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
            AM0FR::_0 => false,
            AM0FR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AM0FR {
        match value {
            false => AM0FR::_0,
            true => AM0FR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AM0FR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AM0FR::_1
    }
}
#[doc = "Possible values of the field `AM1F`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AM1FR {
    #[doc = "Have not received ADDR1 or ADDR0/ADDR1 range matching address."]
    _0,
    #[doc = "Have received ADDR1 or ADDR0/ADDR1 range matching address."]
    _1,
}
impl AM1FR {
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
            AM1FR::_0 => false,
            AM1FR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> AM1FR {
        match value {
            false => AM1FR::_0,
            true => AM1FR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == AM1FR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == AM1FR::_1
    }
}
#[doc = "Possible values of the field `GCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GCFR {
    #[doc = "Slave has not detected the General Call Address or General Call Address disabled."]
    _0,
    #[doc = "Slave has detected the General Call Address."]
    _1,
}
impl GCFR {
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
            GCFR::_0 => false,
            GCFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> GCFR {
        match value {
            false => GCFR::_0,
            true => GCFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == GCFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == GCFR::_1
    }
}
#[doc = "Possible values of the field `SARF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SARFR {
    #[doc = "SMBus Alert Response disabled or not detected."]
    _0,
    #[doc = "SMBus Alert Response enabled and detected."]
    _1,
}
impl SARFR {
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
            SARFR::_0 => false,
            SARFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SARFR {
        match value {
            false => SARFR::_0,
            true => SARFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SARFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SARFR::_1
    }
}
#[doc = "Possible values of the field `SBF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBFR {
    #[doc = "I2C Slave is idle."]
    _0,
    #[doc = "I2C Slave is busy."]
    _1,
}
impl SBFR {
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
            SBFR::_0 => false,
            SBFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBFR {
        match value {
            false => SBFR::_0,
            true => SBFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SBFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SBFR::_1
    }
}
#[doc = "Possible values of the field `BBF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BBFR {
    #[doc = "I2C Bus is idle."]
    _0,
    #[doc = "I2C Bus is busy."]
    _1,
}
impl BBFR {
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
            BBFR::_0 => false,
            BBFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BBFR {
        match value {
            false => BBFR::_0,
            true => BBFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BBFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BBFR::_1
    }
}
#[doc = "Values that can be written to the field `RSF`"]
pub enum RSFW {
    #[doc = "Slave has not detected a Repeated START condition."]
    _0,
    #[doc = "Slave has detected a Repeated START condition."]
    _1,
}
impl RSFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSFW::_0 => false,
            RSFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSFW<'a> {
    w: &'a mut W,
}
impl<'a> _RSFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave has not detected a Repeated START condition."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSFW::_0)
    }
    #[doc = "Slave has detected a Repeated START condition."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSFW::_1)
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
#[doc = "Values that can be written to the field `SDF`"]
pub enum SDFW {
    #[doc = "Slave has not detected a STOP condition."]
    _0,
    #[doc = "Slave has detected a STOP condition."]
    _1,
}
impl SDFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SDFW::_0 => false,
            SDFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SDFW<'a> {
    w: &'a mut W,
}
impl<'a> _SDFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SDFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave has not detected a STOP condition."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDFW::_0)
    }
    #[doc = "Slave has detected a STOP condition."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDFW::_1)
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
#[doc = "Values that can be written to the field `BEF`"]
pub enum BEFW {
    #[doc = "Slave has not detected a bit error."]
    _0,
    #[doc = "Slave has detected a bit error."]
    _1,
}
impl BEFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BEFW::_0 => false,
            BEFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BEFW<'a> {
    w: &'a mut W,
}
impl<'a> _BEFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BEFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Slave has not detected a bit error."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BEFW::_0)
    }
    #[doc = "Slave has detected a bit error."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BEFW::_1)
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
#[doc = "Values that can be written to the field `FEF`"]
pub enum FEFW {
    #[doc = "FIFO underflow or overflow not detected."]
    _0,
    #[doc = "FIFO underflow or overflow detected."]
    _1,
}
impl FEFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FEFW::_0 => false,
            FEFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FEFW<'a> {
    w: &'a mut W,
}
impl<'a> _FEFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FEFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FIFO underflow or overflow not detected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEFW::_0)
    }
    #[doc = "FIFO underflow or overflow detected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEFW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit Data Flag"]
    #[inline]
    pub fn tdf(&self) -> TDFR {
        TDFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Receive Data Flag"]
    #[inline]
    pub fn rdf(&self) -> RDFR {
        RDFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Address Valid Flag"]
    #[inline]
    pub fn avf(&self) -> AVFR {
        AVFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Transmit ACK Flag"]
    #[inline]
    pub fn taf(&self) -> TAFR {
        TAFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Repeated Start Flag"]
    #[inline]
    pub fn rsf(&self) -> RSFR {
        RSFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - STOP Detect Flag"]
    #[inline]
    pub fn sdf(&self) -> SDFR {
        SDFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Bit Error Flag"]
    #[inline]
    pub fn bef(&self) -> BEFR {
        BEFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - FIFO Error Flag"]
    #[inline]
    pub fn fef(&self) -> FEFR {
        FEFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Address Match 0 Flag"]
    #[inline]
    pub fn am0f(&self) -> AM0FR {
        AM0FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Address Match 1 Flag"]
    #[inline]
    pub fn am1f(&self) -> AM1FR {
        AM1FR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - General Call Flag"]
    #[inline]
    pub fn gcf(&self) -> GCFR {
        GCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - SMBus Alert Response Flag"]
    #[inline]
    pub fn sarf(&self) -> SARFR {
        SARFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Slave Busy Flag"]
    #[inline]
    pub fn sbf(&self) -> SBFR {
        SBFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Bus Busy Flag"]
    #[inline]
    pub fn bbf(&self) -> BBFR {
        BBFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
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
    #[doc = "Bit 8 - Repeated Start Flag"]
    #[inline]
    pub fn rsf(&mut self) -> _RSFW {
        _RSFW { w: self }
    }
    #[doc = "Bit 9 - STOP Detect Flag"]
    #[inline]
    pub fn sdf(&mut self) -> _SDFW {
        _SDFW { w: self }
    }
    #[doc = "Bit 10 - Bit Error Flag"]
    #[inline]
    pub fn bef(&mut self) -> _BEFW {
        _BEFW { w: self }
    }
    #[doc = "Bit 11 - FIFO Error Flag"]
    #[inline]
    pub fn fef(&mut self) -> _FEFW {
        _FEFW { w: self }
    }
}
