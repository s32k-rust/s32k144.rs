#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCR {
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
#[doc = "Possible values of the field `MEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MENR {
    #[doc = "Master logic is disabled."]
    _0,
    #[doc = "Master logic is enabled."]
    _1,
}
impl MENR {
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
            MENR::_0 => false,
            MENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MENR {
        match value {
            false => MENR::_0,
            true => MENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MENR::_1
    }
}
#[doc = "Possible values of the field `RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSTR {
    #[doc = "Master logic is not reset."]
    _0,
    #[doc = "Master logic is reset."]
    _1,
}
impl RSTR {
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
            RSTR::_0 => false,
            RSTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RSTR {
        match value {
            false => RSTR::_0,
            true => RSTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RSTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RSTR::_1
    }
}
#[doc = "Possible values of the field `DOZEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZENR {
    #[doc = "Master is enabled in Doze mode."]
    _0,
    #[doc = "Master is disabled in Doze mode."]
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
#[doc = "Possible values of the field `DBGEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGENR {
    #[doc = "Master is disabled in debug mode."]
    _0,
    #[doc = "Master is enabled in debug mode."]
    _1,
}
impl DBGENR {
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
            DBGENR::_0 => false,
            DBGENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBGENR {
        match value {
            false => DBGENR::_0,
            true => DBGENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DBGENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DBGENR::_1
    }
}
#[doc = "Values that can be written to the field `MEN`"]
pub enum MENW {
    #[doc = "Master logic is disabled."]
    _0,
    #[doc = "Master logic is enabled."]
    _1,
}
impl MENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MENW::_0 => false,
            MENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MENW<'a> {
    w: &'a mut W,
}
impl<'a> _MENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Master logic is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MENW::_0)
    }
    #[doc = "Master logic is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MENW::_1)
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
#[doc = "Values that can be written to the field `RST`"]
pub enum RSTW {
    #[doc = "Master logic is not reset."]
    _0,
    #[doc = "Master logic is reset."]
    _1,
}
impl RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RSTW::_0 => false,
            RSTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Master logic is not reset."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTW::_0)
    }
    #[doc = "Master logic is reset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTW::_1)
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
#[doc = "Values that can be written to the field `DOZEN`"]
pub enum DOZENW {
    #[doc = "Master is enabled in Doze mode."]
    _0,
    #[doc = "Master is disabled in Doze mode."]
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
    #[doc = "Master is enabled in Doze mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOZENW::_0)
    }
    #[doc = "Master is disabled in Doze mode."]
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DBGEN`"]
pub enum DBGENW {
    #[doc = "Master is disabled in debug mode."]
    _0,
    #[doc = "Master is enabled in debug mode."]
    _1,
}
impl DBGENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBGENW::_0 => false,
            DBGENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBGENW<'a> {
    w: &'a mut W,
}
impl<'a> _DBGENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBGENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Master is disabled in debug mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGENW::_0)
    }
    #[doc = "Master is enabled in debug mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGENW::_1)
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
#[doc = "Values that can be written to the field `RTF`"]
pub enum RTFW {
    #[doc = "No effect."]
    _0,
    #[doc = "Transmit FIFO is reset."]
    _1,
}
impl RTFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTFW::_0 => false,
            RTFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTFW<'a> {
    w: &'a mut W,
}
impl<'a> _RTFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTFW::_0)
    }
    #[doc = "Transmit FIFO is reset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTFW::_1)
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
#[doc = "Values that can be written to the field `RRF`"]
pub enum RRFW {
    #[doc = "No effect."]
    _0,
    #[doc = "Receive FIFO is reset."]
    _1,
}
impl RRFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RRFW::_0 => false,
            RRFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RRFW<'a> {
    w: &'a mut W,
}
impl<'a> _RRFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RRFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RRFW::_0)
    }
    #[doc = "Receive FIFO is reset."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RRFW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Master Enable"]
    #[inline]
    pub fn men(&self) -> MENR {
        MENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline]
    pub fn rst(&self) -> RSTR {
        RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Doze mode enable"]
    #[inline]
    pub fn dozen(&self) -> DOZENR {
        DOZENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Debug Enable"]
    #[inline]
    pub fn dbgen(&self) -> DBGENR {
        DBGENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Master Enable"]
    #[inline]
    pub fn men(&mut self) -> _MENW {
        _MENW { w: self }
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline]
    pub fn rst(&mut self) -> _RSTW {
        _RSTW { w: self }
    }
    #[doc = "Bit 2 - Doze mode enable"]
    #[inline]
    pub fn dozen(&mut self) -> _DOZENW {
        _DOZENW { w: self }
    }
    #[doc = "Bit 3 - Debug Enable"]
    #[inline]
    pub fn dbgen(&mut self) -> _DBGENW {
        _DBGENW { w: self }
    }
    #[doc = "Bit 8 - Reset Transmit FIFO"]
    #[inline]
    pub fn rtf(&mut self) -> _RTFW {
        _RTFW { w: self }
    }
    #[doc = "Bit 9 - Reset Receive FIFO"]
    #[inline]
    pub fn rrf(&mut self) -> _RRFW {
        _RRFW { w: self }
    }
}
