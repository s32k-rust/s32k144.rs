#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CPO {
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
#[doc = "Possible values of the field `CPOREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOREQR {
    #[doc = "Request is cleared."]
    _0,
    #[doc = "Request Compute Operation."]
    _1,
}
impl CPOREQR {
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
            CPOREQR::_0 => false,
            CPOREQR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPOREQR {
        match value {
            false => CPOREQR::_0,
            true => CPOREQR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPOREQR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPOREQR::_1
    }
}
#[doc = "Possible values of the field `CPOACK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOACKR {
    #[doc = "Compute operation entry has not completed or compute operation exit has completed."]
    _0,
    #[doc = "Compute operation entry has completed or compute operation exit has not completed."]
    _1,
}
impl CPOACKR {
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
            CPOACKR::_0 => false,
            CPOACKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPOACKR {
        match value {
            false => CPOACKR::_0,
            true => CPOACKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPOACKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPOACKR::_1
    }
}
#[doc = "Possible values of the field `CPOWOI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOWOIR {
    #[doc = "No effect."]
    _0,
    #[doc = "When set, the CPOREQ is cleared on any interrupt or exception vector fetch."]
    _1,
}
impl CPOWOIR {
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
            CPOWOIR::_0 => false,
            CPOWOIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPOWOIR {
        match value {
            false => CPOWOIR::_0,
            true => CPOWOIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPOWOIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPOWOIR::_1
    }
}
#[doc = "Values that can be written to the field `CPOREQ`"]
pub enum CPOREQW {
    #[doc = "Request is cleared."]
    _0,
    #[doc = "Request Compute Operation."]
    _1,
}
impl CPOREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPOREQW::_0 => false,
            CPOREQW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPOREQW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPOREQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Request is cleared."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPOREQW::_0)
    }
    #[doc = "Request Compute Operation."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPOREQW::_1)
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
#[doc = "Values that can be written to the field `CPOWOI`"]
pub enum CPOWOIW {
    #[doc = "No effect."]
    _0,
    #[doc = "When set, the CPOREQ is cleared on any interrupt or exception vector fetch."]
    _1,
}
impl CPOWOIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPOWOIW::_0 => false,
            CPOWOIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPOWOIW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOWOIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPOWOIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPOWOIW::_0)
    }
    #[doc = "When set, the CPOREQ is cleared on any interrupt or exception vector fetch."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPOWOIW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Compute Operation Request"]
    #[inline]
    pub fn cporeq(&self) -> CPOREQR {
        CPOREQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Compute Operation Acknowledge"]
    #[inline]
    pub fn cpoack(&self) -> CPOACKR {
        CPOACKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Compute Operation Wakeup On Interrupt"]
    #[inline]
    pub fn cpowoi(&self) -> CPOWOIR {
        CPOWOIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Compute Operation Request"]
    #[inline]
    pub fn cporeq(&mut self) -> _CPOREQW {
        _CPOREQW { w: self }
    }
    #[doc = "Bit 2 - Compute Operation Wakeup On Interrupt"]
    #[inline]
    pub fn cpowoi(&mut self) -> _CPOWOIW {
        _CPOWOIW { w: self }
    }
}
