#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR0 {
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
#[doc = "Possible values of the field `ENCIE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENCIE1R {
    #[doc = "Interrupt notification of Memory 1 non-correctable error events is disabled."]
    _0,
    #[doc = "Interrupt notification of Memory 1 non-correctable error events is enabled."]
    _1,
}
impl ENCIE1R {
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
            ENCIE1R::_0 => false,
            ENCIE1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENCIE1R {
        match value {
            false => ENCIE1R::_0,
            true => ENCIE1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ENCIE1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ENCIE1R::_1
    }
}
#[doc = "Possible values of the field `ESCIE1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESCIE1R {
    #[doc = "Interrupt notification of Memory 1 single-bit correction events is disabled."]
    _0,
    #[doc = "Interrupt notification of Memory 1 single-bit correction events is enabled."]
    _1,
}
impl ESCIE1R {
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
            ESCIE1R::_0 => false,
            ESCIE1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESCIE1R {
        match value {
            false => ESCIE1R::_0,
            true => ESCIE1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ESCIE1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ESCIE1R::_1
    }
}
#[doc = "Possible values of the field `ENCIE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENCIE0R {
    #[doc = "Interrupt notification of Memory 0 non-correctable error events is disabled."]
    _0,
    #[doc = "Interrupt notification of Memory 0 non-correctable error events is enabled."]
    _1,
}
impl ENCIE0R {
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
            ENCIE0R::_0 => false,
            ENCIE0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ENCIE0R {
        match value {
            false => ENCIE0R::_0,
            true => ENCIE0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ENCIE0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ENCIE0R::_1
    }
}
#[doc = "Possible values of the field `ESCIE0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESCIE0R {
    #[doc = "Interrupt notification of Memory 0 single-bit correction events is disabled."]
    _0,
    #[doc = "Interrupt notification of Memory 0 single-bit correction events is enabled."]
    _1,
}
impl ESCIE0R {
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
            ESCIE0R::_0 => false,
            ESCIE0R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESCIE0R {
        match value {
            false => ESCIE0R::_0,
            true => ESCIE0R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ESCIE0R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ESCIE0R::_1
    }
}
#[doc = "Values that can be written to the field `ENCIE1`"]
pub enum ENCIE1W {
    #[doc = "Interrupt notification of Memory 1 non-correctable error events is disabled."]
    _0,
    #[doc = "Interrupt notification of Memory 1 non-correctable error events is enabled."]
    _1,
}
impl ENCIE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENCIE1W::_0 => false,
            ENCIE1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENCIE1W<'a> {
    w: &'a mut W,
}
impl<'a> _ENCIE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENCIE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt notification of Memory 1 non-correctable error events is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENCIE1W::_0)
    }
    #[doc = "Interrupt notification of Memory 1 non-correctable error events is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENCIE1W::_1)
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
#[doc = "Values that can be written to the field `ESCIE1`"]
pub enum ESCIE1W {
    #[doc = "Interrupt notification of Memory 1 single-bit correction events is disabled."]
    _0,
    #[doc = "Interrupt notification of Memory 1 single-bit correction events is enabled."]
    _1,
}
impl ESCIE1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ESCIE1W::_0 => false,
            ESCIE1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ESCIE1W<'a> {
    w: &'a mut W,
}
impl<'a> _ESCIE1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ESCIE1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt notification of Memory 1 single-bit correction events is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESCIE1W::_0)
    }
    #[doc = "Interrupt notification of Memory 1 single-bit correction events is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESCIE1W::_1)
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
#[doc = "Values that can be written to the field `ENCIE0`"]
pub enum ENCIE0W {
    #[doc = "Interrupt notification of Memory 0 non-correctable error events is disabled."]
    _0,
    #[doc = "Interrupt notification of Memory 0 non-correctable error events is enabled."]
    _1,
}
impl ENCIE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ENCIE0W::_0 => false,
            ENCIE0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENCIE0W<'a> {
    w: &'a mut W,
}
impl<'a> _ENCIE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENCIE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt notification of Memory 0 non-correctable error events is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENCIE0W::_0)
    }
    #[doc = "Interrupt notification of Memory 0 non-correctable error events is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENCIE0W::_1)
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
#[doc = "Values that can be written to the field `ESCIE0`"]
pub enum ESCIE0W {
    #[doc = "Interrupt notification of Memory 0 single-bit correction events is disabled."]
    _0,
    #[doc = "Interrupt notification of Memory 0 single-bit correction events is enabled."]
    _1,
}
impl ESCIE0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ESCIE0W::_0 => false,
            ESCIE0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ESCIE0W<'a> {
    w: &'a mut W,
}
impl<'a> _ESCIE0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ESCIE0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Interrupt notification of Memory 0 single-bit correction events is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESCIE0W::_0)
    }
    #[doc = "Interrupt notification of Memory 0 single-bit correction events is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESCIE0W::_1)
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
    #[doc = "Bit 26 - ENCIE1"]
    #[inline]
    pub fn encie1(&self) -> ENCIE1R {
        ENCIE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 27 - ESCIE1"]
    #[inline]
    pub fn escie1(&self) -> ESCIE1R {
        ESCIE1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - ENCIE0"]
    #[inline]
    pub fn encie0(&self) -> ENCIE0R {
        ENCIE0R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - ESCIE0"]
    #[inline]
    pub fn escie0(&self) -> ESCIE0R {
        ESCIE0R::_from({
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
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 26 - ENCIE1"]
    #[inline]
    pub fn encie1(&mut self) -> _ENCIE1W {
        _ENCIE1W { w: self }
    }
    #[doc = "Bit 27 - ESCIE1"]
    #[inline]
    pub fn escie1(&mut self) -> _ESCIE1W {
        _ESCIE1W { w: self }
    }
    #[doc = "Bit 30 - ENCIE0"]
    #[inline]
    pub fn encie0(&mut self) -> _ENCIE0W {
        _ENCIE0W { w: self }
    }
    #[doc = "Bit 31 - ESCIE0"]
    #[inline]
    pub fn escie0(&mut self) -> _ESCIE0W {
        _ESCIE0W { w: self }
    }
}
