#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ISCR {
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
#[doc = "Possible values of the field `FIOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIOCR {
    #[doc = "No interrupt"]
    _0,
    #[doc = "Interrupt occurred"]
    _1,
}
impl FIOCR {
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
            FIOCR::_0 => false,
            FIOCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIOCR {
        match value {
            false => FIOCR::_0,
            true => FIOCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FIOCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FIOCR::_1
    }
}
#[doc = "Possible values of the field `FDZC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDZCR {
    #[doc = "No interrupt"]
    _0,
    #[doc = "Interrupt occurred"]
    _1,
}
impl FDZCR {
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
            FDZCR::_0 => false,
            FDZCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FDZCR {
        match value {
            false => FDZCR::_0,
            true => FDZCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FDZCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FDZCR::_1
    }
}
#[doc = "Possible values of the field `FOFC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFCR {
    #[doc = "No interrupt"]
    _0,
    #[doc = "Interrupt occurred"]
    _1,
}
impl FOFCR {
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
            FOFCR::_0 => false,
            FOFCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FOFCR {
        match value {
            false => FOFCR::_0,
            true => FOFCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FOFCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FOFCR::_1
    }
}
#[doc = "Possible values of the field `FUFC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUFCR {
    #[doc = "No interrupt"]
    _0,
    #[doc = "Interrupt occurred"]
    _1,
}
impl FUFCR {
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
            FUFCR::_0 => false,
            FUFCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FUFCR {
        match value {
            false => FUFCR::_0,
            true => FUFCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FUFCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FUFCR::_1
    }
}
#[doc = "Possible values of the field `FIXC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIXCR {
    #[doc = "No interrupt"]
    _0,
    #[doc = "Interrupt occurred"]
    _1,
}
impl FIXCR {
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
            FIXCR::_0 => false,
            FIXCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIXCR {
        match value {
            false => FIXCR::_0,
            true => FIXCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FIXCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FIXCR::_1
    }
}
#[doc = "Possible values of the field `FIDC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIDCR {
    #[doc = "No interrupt"]
    _0,
    #[doc = "Interrupt occurred"]
    _1,
}
impl FIDCR {
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
            FIDCR::_0 => false,
            FIDCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIDCR {
        match value {
            false => FIDCR::_0,
            true => FIDCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FIDCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FIDCR::_1
    }
}
#[doc = "Possible values of the field `FIOCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIOCER {
    #[doc = "Disable interrupt"]
    _0,
    #[doc = "Enable interrupt"]
    _1,
}
impl FIOCER {
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
            FIOCER::_0 => false,
            FIOCER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIOCER {
        match value {
            false => FIOCER::_0,
            true => FIOCER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FIOCER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FIOCER::_1
    }
}
#[doc = "Possible values of the field `FDZCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDZCER {
    #[doc = "Disable interrupt"]
    _0,
    #[doc = "Enable interrupt"]
    _1,
}
impl FDZCER {
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
            FDZCER::_0 => false,
            FDZCER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FDZCER {
        match value {
            false => FDZCER::_0,
            true => FDZCER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FDZCER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FDZCER::_1
    }
}
#[doc = "Possible values of the field `FOFCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FOFCER {
    #[doc = "Disable interrupt"]
    _0,
    #[doc = "Enable interrupt"]
    _1,
}
impl FOFCER {
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
            FOFCER::_0 => false,
            FOFCER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FOFCER {
        match value {
            false => FOFCER::_0,
            true => FOFCER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FOFCER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FOFCER::_1
    }
}
#[doc = "Possible values of the field `FUFCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FUFCER {
    #[doc = "Disable interrupt"]
    _0,
    #[doc = "Enable interrupt"]
    _1,
}
impl FUFCER {
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
            FUFCER::_0 => false,
            FUFCER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FUFCER {
        match value {
            false => FUFCER::_0,
            true => FUFCER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FUFCER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FUFCER::_1
    }
}
#[doc = "Possible values of the field `FIXCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIXCER {
    #[doc = "Disable interrupt"]
    _0,
    #[doc = "Enable interrupt"]
    _1,
}
impl FIXCER {
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
            FIXCER::_0 => false,
            FIXCER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIXCER {
        match value {
            false => FIXCER::_0,
            true => FIXCER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FIXCER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FIXCER::_1
    }
}
#[doc = "Possible values of the field `FIDCE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIDCER {
    #[doc = "Disable interrupt"]
    _0,
    #[doc = "Enable interrupt"]
    _1,
}
impl FIDCER {
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
            FIDCER::_0 => false,
            FIDCER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FIDCER {
        match value {
            false => FIDCER::_0,
            true => FIDCER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FIDCER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FIDCER::_1
    }
}
#[doc = "Values that can be written to the field `FIOCE`"]
pub enum FIOCEW {
    #[doc = "Disable interrupt"]
    _0,
    #[doc = "Enable interrupt"]
    _1,
}
impl FIOCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIOCEW::_0 => false,
            FIOCEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIOCEW<'a> {
    w: &'a mut W,
}
impl<'a> _FIOCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIOCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIOCEW::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIOCEW::_1)
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
#[doc = "Values that can be written to the field `FDZCE`"]
pub enum FDZCEW {
    #[doc = "Disable interrupt"]
    _0,
    #[doc = "Enable interrupt"]
    _1,
}
impl FDZCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FDZCEW::_0 => false,
            FDZCEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FDZCEW<'a> {
    w: &'a mut W,
}
impl<'a> _FDZCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FDZCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDZCEW::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDZCEW::_1)
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
#[doc = "Values that can be written to the field `FOFCE`"]
pub enum FOFCEW {
    #[doc = "Disable interrupt"]
    _0,
    #[doc = "Enable interrupt"]
    _1,
}
impl FOFCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FOFCEW::_0 => false,
            FOFCEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FOFCEW<'a> {
    w: &'a mut W,
}
impl<'a> _FOFCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FOFCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FOFCEW::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FOFCEW::_1)
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
#[doc = "Values that can be written to the field `FUFCE`"]
pub enum FUFCEW {
    #[doc = "Disable interrupt"]
    _0,
    #[doc = "Enable interrupt"]
    _1,
}
impl FUFCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FUFCEW::_0 => false,
            FUFCEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FUFCEW<'a> {
    w: &'a mut W,
}
impl<'a> _FUFCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FUFCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FUFCEW::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FUFCEW::_1)
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
#[doc = "Values that can be written to the field `FIXCE`"]
pub enum FIXCEW {
    #[doc = "Disable interrupt"]
    _0,
    #[doc = "Enable interrupt"]
    _1,
}
impl FIXCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIXCEW::_0 => false,
            FIXCEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIXCEW<'a> {
    w: &'a mut W,
}
impl<'a> _FIXCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIXCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIXCEW::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIXCEW::_1)
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
#[doc = "Values that can be written to the field `FIDCE`"]
pub enum FIDCEW {
    #[doc = "Disable interrupt"]
    _0,
    #[doc = "Enable interrupt"]
    _1,
}
impl FIDCEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FIDCEW::_0 => false,
            FIDCEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FIDCEW<'a> {
    w: &'a mut W,
}
impl<'a> _FIDCEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FIDCEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable interrupt"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIDCEW::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIDCEW::_1)
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
    #[doc = "Bit 8 - FPU Invalid Operation Interrupt Status"]
    #[inline]
    pub fn fioc(&self) -> FIOCR {
        FIOCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - FPU Divide-by-Zero Interrupt Status"]
    #[inline]
    pub fn fdzc(&self) -> FDZCR {
        FDZCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - FPU Overflow Interrupt Status"]
    #[inline]
    pub fn fofc(&self) -> FOFCR {
        FOFCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - FPU Underflow Interrupt Status"]
    #[inline]
    pub fn fufc(&self) -> FUFCR {
        FUFCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - FPU Inexact Interrupt Status"]
    #[inline]
    pub fn fixc(&self) -> FIXCR {
        FIXCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - FPU Input Denormal Interrupt Status"]
    #[inline]
    pub fn fidc(&self) -> FIDCR {
        FIDCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - FPU Invalid Operation Interrupt Enable"]
    #[inline]
    pub fn fioce(&self) -> FIOCER {
        FIOCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - FPU Divide-by-Zero Interrupt Enable"]
    #[inline]
    pub fn fdzce(&self) -> FDZCER {
        FDZCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - FPU Overflow Interrupt Enable"]
    #[inline]
    pub fn fofce(&self) -> FOFCER {
        FOFCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - FPU Underflow Interrupt Enable"]
    #[inline]
    pub fn fufce(&self) -> FUFCER {
        FUFCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 28 - FPU Inexact Interrupt Enable"]
    #[inline]
    pub fn fixce(&self) -> FIXCER {
        FIXCER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - FPU Input Denormal Interrupt Enable"]
    #[inline]
    pub fn fidce(&self) -> FIDCER {
        FIDCER::_from({
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
        W { bits: 131072 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 24 - FPU Invalid Operation Interrupt Enable"]
    #[inline]
    pub fn fioce(&mut self) -> _FIOCEW {
        _FIOCEW { w: self }
    }
    #[doc = "Bit 25 - FPU Divide-by-Zero Interrupt Enable"]
    #[inline]
    pub fn fdzce(&mut self) -> _FDZCEW {
        _FDZCEW { w: self }
    }
    #[doc = "Bit 26 - FPU Overflow Interrupt Enable"]
    #[inline]
    pub fn fofce(&mut self) -> _FOFCEW {
        _FOFCEW { w: self }
    }
    #[doc = "Bit 27 - FPU Underflow Interrupt Enable"]
    #[inline]
    pub fn fufce(&mut self) -> _FUFCEW {
        _FUFCEW { w: self }
    }
    #[doc = "Bit 28 - FPU Inexact Interrupt Enable"]
    #[inline]
    pub fn fixce(&mut self) -> _FIXCEW {
        _FIXCEW { w: self }
    }
    #[doc = "Bit 31 - FPU Input Denormal Interrupt Enable"]
    #[inline]
    pub fn fidce(&mut self) -> _FIDCEW {
        _FIDCEW { w: self }
    }
}
