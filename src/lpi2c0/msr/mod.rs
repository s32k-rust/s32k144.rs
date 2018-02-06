#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MSR {
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
#[doc = "Possible values of the field `EPF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPFR {
    #[doc = "Master has not generated a STOP or Repeated START condition."]
    _0,
    #[doc = "Master has generated a STOP or Repeated START condition."]
    _1,
}
impl EPFR {
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
            EPFR::_0 => false,
            EPFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> EPFR {
        match value {
            false => EPFR::_0,
            true => EPFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == EPFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == EPFR::_1
    }
}
#[doc = "Possible values of the field `SDF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDFR {
    #[doc = "Master has not generated a STOP condition."]
    _0,
    #[doc = "Master has generated a STOP condition."]
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
#[doc = "Possible values of the field `NDF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NDFR {
    #[doc = "Unexpected NACK not detected."]
    _0,
    #[doc = "Unexpected NACK was detected."]
    _1,
}
impl NDFR {
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
            NDFR::_0 => false,
            NDFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> NDFR {
        match value {
            false => NDFR::_0,
            true => NDFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == NDFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == NDFR::_1
    }
}
#[doc = "Possible values of the field `ALF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALFR {
    #[doc = "Master has not lost arbitration."]
    _0,
    #[doc = "Master has lost arbitration."]
    _1,
}
impl ALFR {
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
            ALFR::_0 => false,
            ALFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ALFR {
        match value {
            false => ALFR::_0,
            true => ALFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ALFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ALFR::_1
    }
}
#[doc = "Possible values of the field `FEF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FEFR {
    #[doc = "No error."]
    _0,
    #[doc = "Master sending or receiving data without START condition."]
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
#[doc = "Possible values of the field `PLTF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLTFR {
    #[doc = "Pin low timeout has not occurred or is disabled."]
    _0,
    #[doc = "Pin low timeout has occurred."]
    _1,
}
impl PLTFR {
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
            PLTFR::_0 => false,
            PLTFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PLTFR {
        match value {
            false => PLTFR::_0,
            true => PLTFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PLTFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PLTFR::_1
    }
}
#[doc = "Possible values of the field `DMF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMFR {
    #[doc = "Have not received matching data."]
    _0,
    #[doc = "Have received matching data."]
    _1,
}
impl DMFR {
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
            DMFR::_0 => false,
            DMFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMFR {
        match value {
            false => DMFR::_0,
            true => DMFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMFR::_1
    }
}
#[doc = "Possible values of the field `MBF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBFR {
    #[doc = "I2C Master is idle."]
    _0,
    #[doc = "I2C Master is busy."]
    _1,
}
impl MBFR {
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
            MBFR::_0 => false,
            MBFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MBFR {
        match value {
            false => MBFR::_0,
            true => MBFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MBFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MBFR::_1
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
#[doc = "Values that can be written to the field `EPF`"]
pub enum EPFW {
    #[doc = "Master has not generated a STOP or Repeated START condition."]
    _0,
    #[doc = "Master has generated a STOP or Repeated START condition."]
    _1,
}
impl EPFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            EPFW::_0 => false,
            EPFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EPFW<'a> {
    w: &'a mut W,
}
impl<'a> _EPFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EPFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Master has not generated a STOP or Repeated START condition."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(EPFW::_0)
    }
    #[doc = "Master has generated a STOP or Repeated START condition."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(EPFW::_1)
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
    #[doc = "Master has not generated a STOP condition."]
    _0,
    #[doc = "Master has generated a STOP condition."]
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
    #[doc = "Master has not generated a STOP condition."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDFW::_0)
    }
    #[doc = "Master has generated a STOP condition."]
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
#[doc = "Values that can be written to the field `NDF`"]
pub enum NDFW {
    #[doc = "Unexpected NACK not detected."]
    _0,
    #[doc = "Unexpected NACK was detected."]
    _1,
}
impl NDFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            NDFW::_0 => false,
            NDFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _NDFW<'a> {
    w: &'a mut W,
}
impl<'a> _NDFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: NDFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Unexpected NACK not detected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(NDFW::_0)
    }
    #[doc = "Unexpected NACK was detected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(NDFW::_1)
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
#[doc = "Values that can be written to the field `ALF`"]
pub enum ALFW {
    #[doc = "Master has not lost arbitration."]
    _0,
    #[doc = "Master has lost arbitration."]
    _1,
}
impl ALFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ALFW::_0 => false,
            ALFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ALFW<'a> {
    w: &'a mut W,
}
impl<'a> _ALFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ALFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Master has not lost arbitration."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALFW::_0)
    }
    #[doc = "Master has lost arbitration."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALFW::_1)
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
#[doc = "Values that can be written to the field `FEF`"]
pub enum FEFW {
    #[doc = "No error."]
    _0,
    #[doc = "Master sending or receiving data without START condition."]
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
    #[doc = "No error."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEFW::_0)
    }
    #[doc = "Master sending or receiving data without START condition."]
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PLTF`"]
pub enum PLTFW {
    #[doc = "Pin low timeout has not occurred or is disabled."]
    _0,
    #[doc = "Pin low timeout has occurred."]
    _1,
}
impl PLTFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PLTFW::_0 => false,
            PLTFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PLTFW<'a> {
    w: &'a mut W,
}
impl<'a> _PLTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PLTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pin low timeout has not occurred or is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PLTFW::_0)
    }
    #[doc = "Pin low timeout has occurred."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PLTFW::_1)
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
#[doc = "Values that can be written to the field `DMF`"]
pub enum DMFW {
    #[doc = "Have not received matching data."]
    _0,
    #[doc = "Have received matching data."]
    _1,
}
impl DMFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMFW::_0 => false,
            DMFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMFW<'a> {
    w: &'a mut W,
}
impl<'a> _DMFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Have not received matching data."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMFW::_0)
    }
    #[doc = "Have received matching data."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMFW::_1)
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
    #[doc = "Bit 8 - End Packet Flag"]
    #[inline]
    pub fn epf(&self) -> EPFR {
        EPFR::_from({
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
    #[doc = "Bit 10 - NACK Detect Flag"]
    #[inline]
    pub fn ndf(&self) -> NDFR {
        NDFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Arbitration Lost Flag"]
    #[inline]
    pub fn alf(&self) -> ALFR {
        ALFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - FIFO Error Flag"]
    #[inline]
    pub fn fef(&self) -> FEFR {
        FEFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Pin Low Timeout Flag"]
    #[inline]
    pub fn pltf(&self) -> PLTFR {
        PLTFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Data Match Flag"]
    #[inline]
    pub fn dmf(&self) -> DMFR {
        DMFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Master Busy Flag"]
    #[inline]
    pub fn mbf(&self) -> MBFR {
        MBFR::_from({
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
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 8 - End Packet Flag"]
    #[inline]
    pub fn epf(&mut self) -> _EPFW {
        _EPFW { w: self }
    }
    #[doc = "Bit 9 - STOP Detect Flag"]
    #[inline]
    pub fn sdf(&mut self) -> _SDFW {
        _SDFW { w: self }
    }
    #[doc = "Bit 10 - NACK Detect Flag"]
    #[inline]
    pub fn ndf(&mut self) -> _NDFW {
        _NDFW { w: self }
    }
    #[doc = "Bit 11 - Arbitration Lost Flag"]
    #[inline]
    pub fn alf(&mut self) -> _ALFW {
        _ALFW { w: self }
    }
    #[doc = "Bit 12 - FIFO Error Flag"]
    #[inline]
    pub fn fef(&mut self) -> _FEFW {
        _FEFW { w: self }
    }
    #[doc = "Bit 13 - Pin Low Timeout Flag"]
    #[inline]
    pub fn pltf(&mut self) -> _PLTFW {
        _PLTFW { w: self }
    }
    #[doc = "Bit 14 - Data Match Flag"]
    #[inline]
    pub fn dmf(&mut self) -> _DMFW {
        _DMFW { w: self }
    }
}
