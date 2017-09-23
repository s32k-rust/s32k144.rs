#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::FSTAT {
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
#[doc = r" Value of the field"]
pub struct MGSTAT0R {
    bits: bool,
}
impl MGSTAT0R {
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
#[doc = "Possible values of the field `FPVIOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FPVIOLR {
    #[doc = "No protection violation detected"] _0,
    #[doc = "Protection violation detected"] _1,
}
impl FPVIOLR {
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
            FPVIOLR::_0 => false,
            FPVIOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FPVIOLR {
        match value {
            false => FPVIOLR::_0,
            true => FPVIOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FPVIOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FPVIOLR::_1
    }
}
#[doc = "Possible values of the field `ACCERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACCERRR {
    #[doc = "No access error detected"] _0,
    #[doc = "Access error detected"] _1,
}
impl ACCERRR {
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
            ACCERRR::_0 => false,
            ACCERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACCERRR {
        match value {
            false => ACCERRR::_0,
            true => ACCERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ACCERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ACCERRR::_1
    }
}
#[doc = "Possible values of the field `RDCOLERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDCOLERRR {
    #[doc = "No collision error detected"] _0,
    #[doc = "Collision error detected"] _1,
}
impl RDCOLERRR {
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
            RDCOLERRR::_0 => false,
            RDCOLERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDCOLERRR {
        match value {
            false => RDCOLERRR::_0,
            true => RDCOLERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RDCOLERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RDCOLERRR::_1
    }
}
#[doc = r" Value of the field"]
pub struct CCIFR {
    bits: bool,
}
impl CCIFR {
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
#[doc = "Values that can be written to the field `FPVIOL`"]
pub enum FPVIOLW {
    #[doc = "No protection violation detected"] _0,
    #[doc = "Protection violation detected"] _1,
}
impl FPVIOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FPVIOLW::_0 => false,
            FPVIOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FPVIOLW<'a> {
    w: &'a mut W,
}
impl<'a> _FPVIOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FPVIOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No protection violation detected"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPVIOLW::_0)
    }
    #[doc = "Protection violation detected"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPVIOLW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ACCERR`"]
pub enum ACCERRW {
    #[doc = "No access error detected"] _0,
    #[doc = "Access error detected"] _1,
}
impl ACCERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACCERRW::_0 => false,
            ACCERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACCERRW<'a> {
    w: &'a mut W,
}
impl<'a> _ACCERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACCERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No access error detected"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACCERRW::_0)
    }
    #[doc = "Access error detected"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACCERRW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RDCOLERR`"]
pub enum RDCOLERRW {
    #[doc = "No collision error detected"] _0,
    #[doc = "Collision error detected"] _1,
}
impl RDCOLERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDCOLERRW::_0 => false,
            RDCOLERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDCOLERRW<'a> {
    w: &'a mut W,
}
impl<'a> _RDCOLERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDCOLERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No collision error detected"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDCOLERRW::_0)
    }
    #[doc = "Collision error detected"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDCOLERRW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CCIFW<'a> {
    w: &'a mut W,
}
impl<'a> _CCIFW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Memory Controller Command Completion Status Flag"]
    #[inline]
    pub fn mgstat0(&self) -> MGSTAT0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        MGSTAT0R { bits }
    }
    #[doc = "Bit 4 - Flash Protection Violation Flag"]
    #[inline]
    pub fn fpviol(&self) -> FPVIOLR {
        FPVIOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Flash Access Error Flag"]
    #[inline]
    pub fn accerr(&self) -> ACCERRR {
        ACCERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 6 - FTFC Read Collision Error Flag"]
    #[inline]
    pub fn rdcolerr(&self) -> RDCOLERRR {
        RDCOLERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Command Complete Interrupt Flag"]
    #[inline]
    pub fn ccif(&self) -> CCIFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        };
        CCIFR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 128 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - Flash Protection Violation Flag"]
    #[inline]
    pub fn fpviol(&mut self) -> _FPVIOLW {
        _FPVIOLW { w: self }
    }
    #[doc = "Bit 5 - Flash Access Error Flag"]
    #[inline]
    pub fn accerr(&mut self) -> _ACCERRW {
        _ACCERRW { w: self }
    }
    #[doc = "Bit 6 - FTFC Read Collision Error Flag"]
    #[inline]
    pub fn rdcolerr(&mut self) -> _RDCOLERRW {
        _RDCOLERRW { w: self }
    }
    #[doc = "Bit 7 - Command Complete Interrupt Flag"]
    #[inline]
    pub fn ccif(&mut self) -> _CCIFW {
        _CCIFW { w: self }
    }
}
