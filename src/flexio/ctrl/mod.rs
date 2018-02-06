#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
#[doc = "Possible values of the field `FLEXEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLEXENR {
    #[doc = "FlexIO module is disabled."]
    _0,
    #[doc = "FlexIO module is enabled."]
    _1,
}
impl FLEXENR {
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
            FLEXENR::_0 => false,
            FLEXENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLEXENR {
        match value {
            false => FLEXENR::_0,
            true => FLEXENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FLEXENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FLEXENR::_1
    }
}
#[doc = "Possible values of the field `SWRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRSTR {
    #[doc = "Software reset is disabled"]
    _0,
    #[doc = "Software reset is enabled, all FlexIO registers except the Control Register are reset."]
    _1,
}
impl SWRSTR {
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
            SWRSTR::_0 => false,
            SWRSTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWRSTR {
        match value {
            false => SWRSTR::_0,
            true => SWRSTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SWRSTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SWRSTR::_1
    }
}
#[doc = "Possible values of the field `FASTACC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FASTACCR {
    #[doc = "Configures for normal register accesses to FlexIO"]
    _0,
    #[doc = "Configures for fast register accesses to FlexIO"]
    _1,
}
impl FASTACCR {
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
            FASTACCR::_0 => false,
            FASTACCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FASTACCR {
        match value {
            false => FASTACCR::_0,
            true => FASTACCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FASTACCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FASTACCR::_1
    }
}
#[doc = "Possible values of the field `DBGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGER {
    #[doc = "FlexIO is disabled in debug modes."]
    _0,
    #[doc = "FlexIO is enabled in debug modes"]
    _1,
}
impl DBGER {
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
            DBGER::_0 => false,
            DBGER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBGER {
        match value {
            false => DBGER::_0,
            true => DBGER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DBGER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DBGER::_1
    }
}
#[doc = "Possible values of the field `DOZEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZENR {
    #[doc = "FlexIO enabled in Doze modes."]
    _0,
    #[doc = "FlexIO disabled in Doze modes."]
    _1,
}
impl DOZENR {
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
            DOZENR::_0 => false,
            DOZENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOZENR {
        match value {
            false => DOZENR::_0,
            true => DOZENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DOZENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DOZENR::_1
    }
}
#[doc = "Values that can be written to the field `FLEXEN`"]
pub enum FLEXENW {
    #[doc = "FlexIO module is disabled."]
    _0,
    #[doc = "FlexIO module is enabled."]
    _1,
}
impl FLEXENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLEXENW::_0 => false,
            FLEXENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLEXENW<'a> {
    w: &'a mut W,
}
impl<'a> _FLEXENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLEXENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FlexIO module is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLEXENW::_0)
    }
    #[doc = "FlexIO module is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLEXENW::_1)
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
#[doc = "Values that can be written to the field `SWRST`"]
pub enum SWRSTW {
    #[doc = "Software reset is disabled"]
    _0,
    #[doc = "Software reset is enabled, all FlexIO registers except the Control Register are reset."]
    _1,
}
impl SWRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWRSTW::_0 => false,
            SWRSTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software reset is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWRSTW::_0)
    }
    #[doc = "Software reset is enabled, all FlexIO registers except the Control Register are reset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWRSTW::_1)
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
#[doc = "Values that can be written to the field `FASTACC`"]
pub enum FASTACCW {
    #[doc = "Configures for normal register accesses to FlexIO"]
    _0,
    #[doc = "Configures for fast register accesses to FlexIO"]
    _1,
}
impl FASTACCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FASTACCW::_0 => false,
            FASTACCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FASTACCW<'a> {
    w: &'a mut W,
}
impl<'a> _FASTACCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FASTACCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Configures for normal register accesses to FlexIO"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FASTACCW::_0)
    }
    #[doc = "Configures for fast register accesses to FlexIO"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FASTACCW::_1)
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
#[doc = "Values that can be written to the field `DBGE`"]
pub enum DBGEW {
    #[doc = "FlexIO is disabled in debug modes."]
    _0,
    #[doc = "FlexIO is enabled in debug modes"]
    _1,
}
impl DBGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBGEW::_0 => false,
            DBGEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBGEW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FlexIO is disabled in debug modes."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGEW::_0)
    }
    #[doc = "FlexIO is enabled in debug modes"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGEW::_1)
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
#[doc = "Values that can be written to the field `DOZEN`"]
pub enum DOZENW {
    #[doc = "FlexIO enabled in Doze modes."]
    _0,
    #[doc = "FlexIO disabled in Doze modes."]
    _1,
}
impl DOZENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOZENW::_0 => false,
            DOZENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOZENW<'a> {
    w: &'a mut W,
}
impl<'a> _DOZENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOZENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FlexIO enabled in Doze modes."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOZENW::_0)
    }
    #[doc = "FlexIO disabled in Doze modes."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOZENW::_1)
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
    #[doc = "Bit 0 - FlexIO Enable"]
    #[inline]
    pub fn flexen(&self) -> FLEXENR {
        FLEXENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline]
    pub fn swrst(&self) -> SWRSTR {
        SWRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Fast Access"]
    #[inline]
    pub fn fastacc(&self) -> FASTACCR {
        FASTACCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Debug Enable"]
    #[inline]
    pub fn dbge(&self) -> DBGER {
        DBGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Doze Enable"]
    #[inline]
    pub fn dozen(&self) -> DOZENR {
        DOZENR::_from({
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
    #[doc = "Bit 0 - FlexIO Enable"]
    #[inline]
    pub fn flexen(&mut self) -> _FLEXENW {
        _FLEXENW { w: self }
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline]
    pub fn swrst(&mut self) -> _SWRSTW {
        _SWRSTW { w: self }
    }
    #[doc = "Bit 2 - Fast Access"]
    #[inline]
    pub fn fastacc(&mut self) -> _FASTACCW {
        _FASTACCW { w: self }
    }
    #[doc = "Bit 30 - Debug Enable"]
    #[inline]
    pub fn dbge(&mut self) -> _DBGEW {
        _DBGEW { w: self }
    }
    #[doc = "Bit 31 - Doze Enable"]
    #[inline]
    pub fn dozen(&mut self) -> _DOZENW {
        _DOZENW { w: self }
    }
}
