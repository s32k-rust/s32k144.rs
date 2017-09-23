#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SYNCONF {
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
#[doc = "Possible values of the field `HWTRIGMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWTRIGMODER {
    #[doc = "FTM clears the TRIGj bit when the hardware trigger j is detected, where j = 0, 1,2."]
    _0,
    #[doc = "FTM does not clear the TRIGj bit when the hardware trigger j is detected, where j = 0, 1,2."]
    _1,
}
impl HWTRIGMODER {
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
            HWTRIGMODER::_0 => false,
            HWTRIGMODER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HWTRIGMODER {
        match value {
            false => HWTRIGMODER::_0,
            true => HWTRIGMODER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HWTRIGMODER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HWTRIGMODER::_1
    }
}
#[doc = "Possible values of the field `CNTINC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CNTINCR {
    #[doc = "CNTIN register is updated with its buffer value at all rising edges of FTM input clock."]
    _0,
    #[doc = "CNTIN register is updated with its buffer value by the PWM synchronization."] _1,
}
impl CNTINCR {
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
            CNTINCR::_0 => false,
            CNTINCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CNTINCR {
        match value {
            false => CNTINCR::_0,
            true => CNTINCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CNTINCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CNTINCR::_1
    }
}
#[doc = "Possible values of the field `INVC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVCR {
    #[doc = "INVCTRL register is updated with its buffer value at all rising edges of FTM input clock."]
    _0,
    #[doc = "INVCTRL register is updated with its buffer value by the PWM synchronization."] _1,
}
impl INVCR {
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
            INVCR::_0 => false,
            INVCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INVCR {
        match value {
            false => INVCR::_0,
            true => INVCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INVCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INVCR::_1
    }
}
#[doc = "Possible values of the field `SWOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWOCR {
    #[doc = "SWOCTRL register is updated with its buffer value at all rising edges of FTM input clock."]
    _0,
    #[doc = "SWOCTRL register is updated with its buffer value by the PWM synchronization."] _1,
}
impl SWOCR {
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
            SWOCR::_0 => false,
            SWOCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWOCR {
        match value {
            false => SWOCR::_0,
            true => SWOCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWOCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWOCR::_1
    }
}
#[doc = "Possible values of the field `SYNCMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYNCMODER {
    #[doc = "Legacy PWM synchronization is selected."] _0,
    #[doc = "Enhanced PWM synchronization is selected."] _1,
}
impl SYNCMODER {
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
            SYNCMODER::_0 => false,
            SYNCMODER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SYNCMODER {
        match value {
            false => SYNCMODER::_0,
            true => SYNCMODER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SYNCMODER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SYNCMODER::_1
    }
}
#[doc = "Possible values of the field `SWRSTCNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTCNTR {
    #[doc = "The software trigger does not activate the FTM counter synchronization."] _0,
    #[doc = "The software trigger activates the FTM counter synchronization."] _1,
}
impl SWRSTCNTR {
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
            SWRSTCNTR::_0 => false,
            SWRSTCNTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWRSTCNTR {
        match value {
            false => SWRSTCNTR::_0,
            true => SWRSTCNTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWRSTCNTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWRSTCNTR::_1
    }
}
#[doc = "Possible values of the field `SWWRBUF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWWRBUFR {
    #[doc = "The software trigger does not activate MOD, HCR, CNTIN, and CV registers synchronization."]
    _0,
    #[doc = "The software trigger activates MOD, HCR, CNTIN, and CV registers synchronization."] _1,
}
impl SWWRBUFR {
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
            SWWRBUFR::_0 => false,
            SWWRBUFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWWRBUFR {
        match value {
            false => SWWRBUFR::_0,
            true => SWWRBUFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWWRBUFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWWRBUFR::_1
    }
}
#[doc = "Possible values of the field `SWOM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWOMR {
    #[doc = "The software trigger does not activate the OUTMASK register synchronization."] _0,
    #[doc = "The software trigger activates the OUTMASK register synchronization."] _1,
}
impl SWOMR {
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
            SWOMR::_0 => false,
            SWOMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWOMR {
        match value {
            false => SWOMR::_0,
            true => SWOMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWOMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWOMR::_1
    }
}
#[doc = "Possible values of the field `SWINVC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWINVCR {
    #[doc = "The software trigger does not activate the INVCTRL register synchronization."] _0,
    #[doc = "The software trigger activates the INVCTRL register synchronization."] _1,
}
impl SWINVCR {
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
            SWINVCR::_0 => false,
            SWINVCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWINVCR {
        match value {
            false => SWINVCR::_0,
            true => SWINVCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWINVCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWINVCR::_1
    }
}
#[doc = "Possible values of the field `SWSOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWSOCR {
    #[doc = "The software trigger does not activate the SWOCTRL register synchronization."] _0,
    #[doc = "The software trigger activates the SWOCTRL register synchronization."] _1,
}
impl SWSOCR {
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
            SWSOCR::_0 => false,
            SWSOCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWSOCR {
        match value {
            false => SWSOCR::_0,
            true => SWSOCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWSOCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWSOCR::_1
    }
}
#[doc = "Possible values of the field `HWRSTCNT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWRSTCNTR {
    #[doc = "A hardware trigger does not activate the FTM counter synchronization."] _0,
    #[doc = "A hardware trigger activates the FTM counter synchronization."] _1,
}
impl HWRSTCNTR {
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
            HWRSTCNTR::_0 => false,
            HWRSTCNTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HWRSTCNTR {
        match value {
            false => HWRSTCNTR::_0,
            true => HWRSTCNTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HWRSTCNTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HWRSTCNTR::_1
    }
}
#[doc = "Possible values of the field `HWWRBUF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWWRBUFR {
    #[doc = "A hardware trigger does not activate MOD, HCR, CNTIN, and CV registers synchronization."]
    _0,
    #[doc = "A hardware trigger activates MOD, HCR, CNTIN, and CV registers synchronization."] _1,
}
impl HWWRBUFR {
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
            HWWRBUFR::_0 => false,
            HWWRBUFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HWWRBUFR {
        match value {
            false => HWWRBUFR::_0,
            true => HWWRBUFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HWWRBUFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HWWRBUFR::_1
    }
}
#[doc = "Possible values of the field `HWOM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWOMR {
    #[doc = "A hardware trigger does not activate the OUTMASK register synchronization."] _0,
    #[doc = "A hardware trigger activates the OUTMASK register synchronization."] _1,
}
impl HWOMR {
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
            HWOMR::_0 => false,
            HWOMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HWOMR {
        match value {
            false => HWOMR::_0,
            true => HWOMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HWOMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HWOMR::_1
    }
}
#[doc = "Possible values of the field `HWINVC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWINVCR {
    #[doc = "A hardware trigger does not activate the INVCTRL register synchronization."] _0,
    #[doc = "A hardware trigger activates the INVCTRL register synchronization."] _1,
}
impl HWINVCR {
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
            HWINVCR::_0 => false,
            HWINVCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HWINVCR {
        match value {
            false => HWINVCR::_0,
            true => HWINVCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HWINVCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HWINVCR::_1
    }
}
#[doc = "Possible values of the field `HWSOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HWSOCR {
    #[doc = "A hardware trigger does not activate the SWOCTRL register synchronization."] _0,
    #[doc = "A hardware trigger activates the SWOCTRL register synchronization."] _1,
}
impl HWSOCR {
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
            HWSOCR::_0 => false,
            HWSOCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HWSOCR {
        match value {
            false => HWSOCR::_0,
            true => HWSOCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HWSOCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HWSOCR::_1
    }
}
#[doc = "Values that can be written to the field `HWTRIGMODE`"]
pub enum HWTRIGMODEW {
    #[doc = "FTM clears the TRIGj bit when the hardware trigger j is detected, where j = 0, 1,2."]
    _0,
    #[doc = "FTM does not clear the TRIGj bit when the hardware trigger j is detected, where j = 0, 1,2."]
    _1,
}
impl HWTRIGMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HWTRIGMODEW::_0 => false,
            HWTRIGMODEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HWTRIGMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _HWTRIGMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HWTRIGMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM clears the TRIGj bit when the hardware trigger j is detected, where j = 0, 1,2."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HWTRIGMODEW::_0)
    }
    #[doc = "FTM does not clear the TRIGj bit when the hardware trigger j is detected, where j = 0, 1,2."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HWTRIGMODEW::_1)
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
#[doc = "Values that can be written to the field `CNTINC`"]
pub enum CNTINCW {
    #[doc = "CNTIN register is updated with its buffer value at all rising edges of FTM input clock."]
    _0,
    #[doc = "CNTIN register is updated with its buffer value by the PWM synchronization."] _1,
}
impl CNTINCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CNTINCW::_0 => false,
            CNTINCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CNTINCW<'a> {
    w: &'a mut W,
}
impl<'a> _CNTINCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CNTINCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CNTIN register is updated with its buffer value at all rising edges of FTM input clock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTINCW::_0)
    }
    #[doc = "CNTIN register is updated with its buffer value by the PWM synchronization."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTINCW::_1)
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
#[doc = "Values that can be written to the field `INVC`"]
pub enum INVCW {
    #[doc = "INVCTRL register is updated with its buffer value at all rising edges of FTM input clock."]
    _0,
    #[doc = "INVCTRL register is updated with its buffer value by the PWM synchronization."] _1,
}
impl INVCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INVCW::_0 => false,
            INVCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INVCW<'a> {
    w: &'a mut W,
}
impl<'a> _INVCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INVCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "INVCTRL register is updated with its buffer value at all rising edges of FTM input clock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INVCW::_0)
    }
    #[doc = "INVCTRL register is updated with its buffer value by the PWM synchronization."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INVCW::_1)
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
#[doc = "Values that can be written to the field `SWOC`"]
pub enum SWOCW {
    #[doc = "SWOCTRL register is updated with its buffer value at all rising edges of FTM input clock."]
    _0,
    #[doc = "SWOCTRL register is updated with its buffer value by the PWM synchronization."] _1,
}
impl SWOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWOCW::_0 => false,
            SWOCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWOCW<'a> {
    w: &'a mut W,
}
impl<'a> _SWOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWOCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "SWOCTRL register is updated with its buffer value at all rising edges of FTM input clock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWOCW::_0)
    }
    #[doc = "SWOCTRL register is updated with its buffer value by the PWM synchronization."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWOCW::_1)
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
#[doc = "Values that can be written to the field `SYNCMODE`"]
pub enum SYNCMODEW {
    #[doc = "Legacy PWM synchronization is selected."] _0,
    #[doc = "Enhanced PWM synchronization is selected."] _1,
}
impl SYNCMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SYNCMODEW::_0 => false,
            SYNCMODEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SYNCMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SYNCMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Legacy PWM synchronization is selected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCMODEW::_0)
    }
    #[doc = "Enhanced PWM synchronization is selected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCMODEW::_1)
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
#[doc = "Values that can be written to the field `SWRSTCNT`"]
pub enum SWRSTCNTW {
    #[doc = "The software trigger does not activate the FTM counter synchronization."] _0,
    #[doc = "The software trigger activates the FTM counter synchronization."] _1,
}
impl SWRSTCNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWRSTCNTW::_0 => false,
            SWRSTCNTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWRSTCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRSTCNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWRSTCNTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The software trigger does not activate the FTM counter synchronization."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWRSTCNTW::_0)
    }
    #[doc = "The software trigger activates the FTM counter synchronization."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWRSTCNTW::_1)
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
#[doc = "Values that can be written to the field `SWWRBUF`"]
pub enum SWWRBUFW {
    #[doc = "The software trigger does not activate MOD, HCR, CNTIN, and CV registers synchronization."]
    _0,
    #[doc = "The software trigger activates MOD, HCR, CNTIN, and CV registers synchronization."] _1,
}
impl SWWRBUFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWWRBUFW::_0 => false,
            SWWRBUFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWWRBUFW<'a> {
    w: &'a mut W,
}
impl<'a> _SWWRBUFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWWRBUFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The software trigger does not activate MOD, HCR, CNTIN, and CV registers synchronization."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWWRBUFW::_0)
    }
    #[doc = "The software trigger activates MOD, HCR, CNTIN, and CV registers synchronization."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWWRBUFW::_1)
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
#[doc = "Values that can be written to the field `SWOM`"]
pub enum SWOMW {
    #[doc = "The software trigger does not activate the OUTMASK register synchronization."] _0,
    #[doc = "The software trigger activates the OUTMASK register synchronization."] _1,
}
impl SWOMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWOMW::_0 => false,
            SWOMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWOMW<'a> {
    w: &'a mut W,
}
impl<'a> _SWOMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWOMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The software trigger does not activate the OUTMASK register synchronization."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWOMW::_0)
    }
    #[doc = "The software trigger activates the OUTMASK register synchronization."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWOMW::_1)
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
#[doc = "Values that can be written to the field `SWINVC`"]
pub enum SWINVCW {
    #[doc = "The software trigger does not activate the INVCTRL register synchronization."] _0,
    #[doc = "The software trigger activates the INVCTRL register synchronization."] _1,
}
impl SWINVCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWINVCW::_0 => false,
            SWINVCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWINVCW<'a> {
    w: &'a mut W,
}
impl<'a> _SWINVCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWINVCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The software trigger does not activate the INVCTRL register synchronization."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWINVCW::_0)
    }
    #[doc = "The software trigger activates the INVCTRL register synchronization."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWINVCW::_1)
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
#[doc = "Values that can be written to the field `SWSOC`"]
pub enum SWSOCW {
    #[doc = "The software trigger does not activate the SWOCTRL register synchronization."] _0,
    #[doc = "The software trigger activates the SWOCTRL register synchronization."] _1,
}
impl SWSOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWSOCW::_0 => false,
            SWSOCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWSOCW<'a> {
    w: &'a mut W,
}
impl<'a> _SWSOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWSOCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The software trigger does not activate the SWOCTRL register synchronization."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWSOCW::_0)
    }
    #[doc = "The software trigger activates the SWOCTRL register synchronization."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWSOCW::_1)
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
#[doc = "Values that can be written to the field `HWRSTCNT`"]
pub enum HWRSTCNTW {
    #[doc = "A hardware trigger does not activate the FTM counter synchronization."] _0,
    #[doc = "A hardware trigger activates the FTM counter synchronization."] _1,
}
impl HWRSTCNTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HWRSTCNTW::_0 => false,
            HWRSTCNTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HWRSTCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _HWRSTCNTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HWRSTCNTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware trigger does not activate the FTM counter synchronization."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HWRSTCNTW::_0)
    }
    #[doc = "A hardware trigger activates the FTM counter synchronization."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HWRSTCNTW::_1)
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
#[doc = "Values that can be written to the field `HWWRBUF`"]
pub enum HWWRBUFW {
    #[doc = "A hardware trigger does not activate MOD, HCR, CNTIN, and CV registers synchronization."]
    _0,
    #[doc = "A hardware trigger activates MOD, HCR, CNTIN, and CV registers synchronization."] _1,
}
impl HWWRBUFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HWWRBUFW::_0 => false,
            HWWRBUFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HWWRBUFW<'a> {
    w: &'a mut W,
}
impl<'a> _HWWRBUFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HWWRBUFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware trigger does not activate MOD, HCR, CNTIN, and CV registers synchronization."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HWWRBUFW::_0)
    }
    #[doc = "A hardware trigger activates MOD, HCR, CNTIN, and CV registers synchronization."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HWWRBUFW::_1)
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
#[doc = "Values that can be written to the field `HWOM`"]
pub enum HWOMW {
    #[doc = "A hardware trigger does not activate the OUTMASK register synchronization."] _0,
    #[doc = "A hardware trigger activates the OUTMASK register synchronization."] _1,
}
impl HWOMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HWOMW::_0 => false,
            HWOMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HWOMW<'a> {
    w: &'a mut W,
}
impl<'a> _HWOMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HWOMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware trigger does not activate the OUTMASK register synchronization."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HWOMW::_0)
    }
    #[doc = "A hardware trigger activates the OUTMASK register synchronization."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HWOMW::_1)
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
#[doc = "Values that can be written to the field `HWINVC`"]
pub enum HWINVCW {
    #[doc = "A hardware trigger does not activate the INVCTRL register synchronization."] _0,
    #[doc = "A hardware trigger activates the INVCTRL register synchronization."] _1,
}
impl HWINVCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HWINVCW::_0 => false,
            HWINVCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HWINVCW<'a> {
    w: &'a mut W,
}
impl<'a> _HWINVCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HWINVCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware trigger does not activate the INVCTRL register synchronization."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HWINVCW::_0)
    }
    #[doc = "A hardware trigger activates the INVCTRL register synchronization."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HWINVCW::_1)
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
#[doc = "Values that can be written to the field `HWSOC`"]
pub enum HWSOCW {
    #[doc = "A hardware trigger does not activate the SWOCTRL register synchronization."] _0,
    #[doc = "A hardware trigger activates the SWOCTRL register synchronization."] _1,
}
impl HWSOCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HWSOCW::_0 => false,
            HWSOCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HWSOCW<'a> {
    w: &'a mut W,
}
impl<'a> _HWSOCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HWSOCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "A hardware trigger does not activate the SWOCTRL register synchronization."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HWSOCW::_0)
    }
    #[doc = "A hardware trigger activates the SWOCTRL register synchronization."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HWSOCW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Hardware Trigger Mode"]
    #[inline]
    pub fn hwtrigmode(&self) -> HWTRIGMODER {
        HWTRIGMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - CNTIN Register Synchronization"]
    #[inline]
    pub fn cntinc(&self) -> CNTINCR {
        CNTINCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - INVCTRL Register Synchronization"]
    #[inline]
    pub fn invc(&self) -> INVCR {
        INVCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - SWOCTRL Register Synchronization"]
    #[inline]
    pub fn swoc(&self) -> SWOCR {
        SWOCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Synchronization Mode"]
    #[inline]
    pub fn syncmode(&self) -> SYNCMODER {
        SYNCMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - FTM counter synchronization is activated by the software trigger"]
    #[inline]
    pub fn swrstcnt(&self) -> SWRSTCNTR {
        SWRSTCNTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - MOD, HCR, CNTIN, and CV registers synchronization is activated by the software trigger"]
    #[inline]
    pub fn swwrbuf(&self) -> SWWRBUFR {
        SWWRBUFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Output mask synchronization is activated by the software trigger"]
    #[inline]
    pub fn swom(&self) -> SWOMR {
        SWOMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Inverting control synchronization is activated by the software trigger"]
    #[inline]
    pub fn swinvc(&self) -> SWINVCR {
        SWINVCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Software output control synchronization is activated by the software trigger"]
    #[inline]
    pub fn swsoc(&self) -> SWSOCR {
        SWSOCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - FTM counter synchronization is activated by a hardware trigger"]
    #[inline]
    pub fn hwrstcnt(&self) -> HWRSTCNTR {
        HWRSTCNTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - MOD, HCR, CNTIN, and CV registers synchronization is activated by a hardware trigger"]
    #[inline]
    pub fn hwwrbuf(&self) -> HWWRBUFR {
        HWWRBUFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Output mask synchronization is activated by a hardware trigger"]
    #[inline]
    pub fn hwom(&self) -> HWOMR {
        HWOMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Inverting control synchronization is activated by a hardware trigger"]
    #[inline]
    pub fn hwinvc(&self) -> HWINVCR {
        HWINVCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Software output control synchronization is activated by a hardware trigger"]
    #[inline]
    pub fn hwsoc(&self) -> HWSOCR {
        HWSOCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - Hardware Trigger Mode"]
    #[inline]
    pub fn hwtrigmode(&mut self) -> _HWTRIGMODEW {
        _HWTRIGMODEW { w: self }
    }
    #[doc = "Bit 2 - CNTIN Register Synchronization"]
    #[inline]
    pub fn cntinc(&mut self) -> _CNTINCW {
        _CNTINCW { w: self }
    }
    #[doc = "Bit 4 - INVCTRL Register Synchronization"]
    #[inline]
    pub fn invc(&mut self) -> _INVCW {
        _INVCW { w: self }
    }
    #[doc = "Bit 5 - SWOCTRL Register Synchronization"]
    #[inline]
    pub fn swoc(&mut self) -> _SWOCW {
        _SWOCW { w: self }
    }
    #[doc = "Bit 7 - Synchronization Mode"]
    #[inline]
    pub fn syncmode(&mut self) -> _SYNCMODEW {
        _SYNCMODEW { w: self }
    }
    #[doc = "Bit 8 - FTM counter synchronization is activated by the software trigger"]
    #[inline]
    pub fn swrstcnt(&mut self) -> _SWRSTCNTW {
        _SWRSTCNTW { w: self }
    }
    #[doc = "Bit 9 - MOD, HCR, CNTIN, and CV registers synchronization is activated by the software trigger"]
    #[inline]
    pub fn swwrbuf(&mut self) -> _SWWRBUFW {
        _SWWRBUFW { w: self }
    }
    #[doc = "Bit 10 - Output mask synchronization is activated by the software trigger"]
    #[inline]
    pub fn swom(&mut self) -> _SWOMW {
        _SWOMW { w: self }
    }
    #[doc = "Bit 11 - Inverting control synchronization is activated by the software trigger"]
    #[inline]
    pub fn swinvc(&mut self) -> _SWINVCW {
        _SWINVCW { w: self }
    }
    #[doc = "Bit 12 - Software output control synchronization is activated by the software trigger"]
    #[inline]
    pub fn swsoc(&mut self) -> _SWSOCW {
        _SWSOCW { w: self }
    }
    #[doc = "Bit 16 - FTM counter synchronization is activated by a hardware trigger"]
    #[inline]
    pub fn hwrstcnt(&mut self) -> _HWRSTCNTW {
        _HWRSTCNTW { w: self }
    }
    #[doc = "Bit 17 - MOD, HCR, CNTIN, and CV registers synchronization is activated by a hardware trigger"]
    #[inline]
    pub fn hwwrbuf(&mut self) -> _HWWRBUFW {
        _HWWRBUFW { w: self }
    }
    #[doc = "Bit 18 - Output mask synchronization is activated by a hardware trigger"]
    #[inline]
    pub fn hwom(&mut self) -> _HWOMW {
        _HWOMW { w: self }
    }
    #[doc = "Bit 19 - Inverting control synchronization is activated by a hardware trigger"]
    #[inline]
    pub fn hwinvc(&mut self) -> _HWINVCW {
        _HWINVCW { w: self }
    }
    #[doc = "Bit 20 - Software output control synchronization is activated by a hardware trigger"]
    #[inline]
    pub fn hwsoc(&mut self) -> _HWSOCW {
        _HWSOCW { w: self }
    }
}
