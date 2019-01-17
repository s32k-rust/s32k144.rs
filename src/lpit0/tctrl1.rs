#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCTRL1 {
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
#[doc = "Possible values of the field `T_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum T_ENR {
    #[doc = "Timer Channel is disabled"]
    _0,
    #[doc = "Timer Channel is enabled"]
    _1,
}
impl T_ENR {
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
            T_ENR::_0 => false,
            T_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> T_ENR {
        match value {
            false => T_ENR::_0,
            true => T_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == T_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == T_ENR::_1
    }
}
#[doc = "Possible values of the field `CHAIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHAINR {
    #[doc = "Channel Chaining is disabled. Channel Timer runs independently."]
    _0,
    #[doc = "Channel Chaining is enabled. Timer decrements on previous channel's timeout"]
    _1,
}
impl CHAINR {
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
            CHAINR::_0 => false,
            CHAINR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHAINR {
        match value {
            false => CHAINR::_0,
            true => CHAINR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CHAINR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CHAINR::_1
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "32-bit Periodic Counter"]
    _0,
    #[doc = "Dual 16-bit Periodic Counter"]
    _1,
    #[doc = "32-bit Trigger Accumulator"]
    _10,
    #[doc = "32-bit Trigger Input Capture"]
    _11,
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::_0 => 0,
            MODER::_1 => 1,
            MODER::_10 => 2,
            MODER::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::_0,
            1 => MODER::_1,
            2 => MODER::_10,
            3 => MODER::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MODER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MODER::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == MODER::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == MODER::_11
    }
}
#[doc = "Possible values of the field `TSOT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSOTR {
    #[doc = "Timer starts to decrement immediately based on restart condition (controlled by TSOI bit)"]
    _0,
    #[doc = "Timer starts to decrement when rising edge on selected trigger is detected"]
    _1,
}
impl TSOTR {
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
            TSOTR::_0 => false,
            TSOTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSOTR {
        match value {
            false => TSOTR::_0,
            true => TSOTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSOTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSOTR::_1
    }
}
#[doc = "Possible values of the field `TSOI`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSOIR {
    #[doc = "The channel timer does not stop after timeout."]
    _0,
    #[doc = "The channel timer will stop after a timeout, and the channel timer will restart based on TSOT. When TSOT = 0, the channel timer will restart after a rising edge on the T_EN bit is detected (which means that the timer channel is disabled and then enabled); when TSOT = 1, the channel timer will restart after a rising edge on the selected trigger is detected."]
    _1,
}
impl TSOIR {
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
            TSOIR::_0 => false,
            TSOIR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSOIR {
        match value {
            false => TSOIR::_0,
            true => TSOIR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSOIR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSOIR::_1
    }
}
#[doc = "Possible values of the field `TROT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TROTR {
    #[doc = "Timer will not reload on selected trigger"]
    _0,
    #[doc = "Timer will reload on selected trigger"]
    _1,
}
impl TROTR {
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
            TROTR::_0 => false,
            TROTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TROTR {
        match value {
            false => TROTR::_0,
            true => TROTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TROTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TROTR::_1
    }
}
#[doc = "Possible values of the field `TRG_SRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRG_SRCR {
    #[doc = "Trigger source selected in external"]
    _0,
    #[doc = "Trigger source selected is the internal trigger"]
    _1,
}
impl TRG_SRCR {
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
            TRG_SRCR::_0 => false,
            TRG_SRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRG_SRCR {
        match value {
            false => TRG_SRCR::_0,
            true => TRG_SRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRG_SRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRG_SRCR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TRG_SELR {
    bits: u8,
}
impl TRG_SELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `T_EN`"]
pub enum T_ENW {
    #[doc = "Timer Channel is disabled"]
    _0,
    #[doc = "Timer Channel is enabled"]
    _1,
}
impl T_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            T_ENW::_0 => false,
            T_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _T_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _T_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: T_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer Channel is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(T_ENW::_0)
    }
    #[doc = "Timer Channel is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(T_ENW::_1)
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
#[doc = "Values that can be written to the field `CHAIN`"]
pub enum CHAINW {
    #[doc = "Channel Chaining is disabled. Channel Timer runs independently."]
    _0,
    #[doc = "Channel Chaining is enabled. Timer decrements on previous channel's timeout"]
    _1,
}
impl CHAINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHAINW::_0 => false,
            CHAINW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHAINW<'a> {
    w: &'a mut W,
}
impl<'a> _CHAINW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHAINW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel Chaining is disabled. Channel Timer runs independently."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHAINW::_0)
    }
    #[doc = "Channel Chaining is enabled. Timer decrements on previous channel's timeout"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHAINW::_1)
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
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "32-bit Periodic Counter"]
    _0,
    #[doc = "Dual 16-bit Periodic Counter"]
    _1,
    #[doc = "32-bit Trigger Accumulator"]
    _10,
    #[doc = "32-bit Trigger Input Capture"]
    _11,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::_0 => 0,
            MODEW::_1 => 1,
            MODEW::_10 => 2,
            MODEW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "32-bit Periodic Counter"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODEW::_0)
    }
    #[doc = "Dual 16-bit Periodic Counter"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODEW::_1)
    }
    #[doc = "32-bit Trigger Accumulator"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(MODEW::_10)
    }
    #[doc = "32-bit Trigger Input Capture"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(MODEW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TSOT`"]
pub enum TSOTW {
    #[doc = "Timer starts to decrement immediately based on restart condition (controlled by TSOI bit)"]
    _0,
    #[doc = "Timer starts to decrement when rising edge on selected trigger is detected"]
    _1,
}
impl TSOTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSOTW::_0 => false,
            TSOTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSOTW<'a> {
    w: &'a mut W,
}
impl<'a> _TSOTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSOTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer starts to decrement immediately based on restart condition (controlled by TSOI bit)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSOTW::_0)
    }
    #[doc = "Timer starts to decrement when rising edge on selected trigger is detected"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSOTW::_1)
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
#[doc = "Values that can be written to the field `TSOI`"]
pub enum TSOIW {
    #[doc = "The channel timer does not stop after timeout."]
    _0,
    #[doc = "The channel timer will stop after a timeout, and the channel timer will restart based on TSOT. When TSOT = 0, the channel timer will restart after a rising edge on the T_EN bit is detected (which means that the timer channel is disabled and then enabled); when TSOT = 1, the channel timer will restart after a rising edge on the selected trigger is detected."]
    _1,
}
impl TSOIW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSOIW::_0 => false,
            TSOIW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSOIW<'a> {
    w: &'a mut W,
}
impl<'a> _TSOIW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSOIW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel timer does not stop after timeout."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSOIW::_0)
    }
    #[doc = "The channel timer will stop after a timeout, and the channel timer will restart based on TSOT. When TSOT = 0, the channel timer will restart after a rising edge on the T_EN bit is detected (which means that the timer channel is disabled and then enabled); when TSOT = 1, the channel timer will restart after a rising edge on the selected trigger is detected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSOIW::_1)
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
#[doc = "Values that can be written to the field `TROT`"]
pub enum TROTW {
    #[doc = "Timer will not reload on selected trigger"]
    _0,
    #[doc = "Timer will reload on selected trigger"]
    _1,
}
impl TROTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TROTW::_0 => false,
            TROTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TROTW<'a> {
    w: &'a mut W,
}
impl<'a> _TROTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TROTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer will not reload on selected trigger"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TROTW::_0)
    }
    #[doc = "Timer will reload on selected trigger"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TROTW::_1)
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
#[doc = "Values that can be written to the field `TRG_SRC`"]
pub enum TRG_SRCW {
    #[doc = "Trigger source selected in external"]
    _0,
    #[doc = "Trigger source selected is the internal trigger"]
    _1,
}
impl TRG_SRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRG_SRCW::_0 => false,
            TRG_SRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRG_SRCW<'a> {
    w: &'a mut W,
}
impl<'a> _TRG_SRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRG_SRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Trigger source selected in external"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRG_SRCW::_0)
    }
    #[doc = "Trigger source selected is the internal trigger"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRG_SRCW::_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TRG_SELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRG_SELW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Timer Enable"]
    #[inline]
    pub fn t_en(&self) -> T_ENR {
        T_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Chain Channel"]
    #[inline]
    pub fn chain(&self) -> CHAINR {
        CHAINR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - Timer Operation Mode"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 16 - Timer Start On Trigger"]
    #[inline]
    pub fn tsot(&self) -> TSOTR {
        TSOTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Timer Stop On Interrupt"]
    #[inline]
    pub fn tsoi(&self) -> TSOIR {
        TSOIR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Timer Reload On Trigger"]
    #[inline]
    pub fn trot(&self) -> TROTR {
        TROTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Trigger Source"]
    #[inline]
    pub fn trg_src(&self) -> TRG_SRCR {
        TRG_SRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline]
    pub fn trg_sel(&self) -> TRG_SELR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRG_SELR { bits }
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
    #[doc = "Bit 0 - Timer Enable"]
    #[inline]
    pub fn t_en(&mut self) -> _T_ENW {
        _T_ENW { w: self }
    }
    #[doc = "Bit 1 - Chain Channel"]
    #[inline]
    pub fn chain(&mut self) -> _CHAINW {
        _CHAINW { w: self }
    }
    #[doc = "Bits 2:3 - Timer Operation Mode"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bit 16 - Timer Start On Trigger"]
    #[inline]
    pub fn tsot(&mut self) -> _TSOTW {
        _TSOTW { w: self }
    }
    #[doc = "Bit 17 - Timer Stop On Interrupt"]
    #[inline]
    pub fn tsoi(&mut self) -> _TSOIW {
        _TSOIW { w: self }
    }
    #[doc = "Bit 18 - Timer Reload On Trigger"]
    #[inline]
    pub fn trot(&mut self) -> _TROTW {
        _TROTW { w: self }
    }
    #[doc = "Bit 23 - Trigger Source"]
    #[inline]
    pub fn trg_src(&mut self) -> _TRG_SRCW {
        _TRG_SRCW { w: self }
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline]
    pub fn trg_sel(&mut self) -> _TRG_SELW {
        _TRG_SELW { w: self }
    }
}
