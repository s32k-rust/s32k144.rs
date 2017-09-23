#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL1 {
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
pub struct PROPSEGR {
    bits: u8,
}
impl PROPSEGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `LOM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOMR {
    #[doc = "Listen-Only mode is deactivated."] _0,
    #[doc = "FlexCAN module operates in Listen-Only mode."] _1,
}
impl LOMR {
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
            LOMR::_0 => false,
            LOMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LOMR {
        match value {
            false => LOMR::_0,
            true => LOMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LOMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LOMR::_1
    }
}
#[doc = "Possible values of the field `LBUF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBUFR {
    #[doc = "Buffer with highest priority is transmitted first."] _0,
    #[doc = "Lowest number buffer is transmitted first."] _1,
}
impl LBUFR {
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
            LBUFR::_0 => false,
            LBUFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBUFR {
        match value {
            false => LBUFR::_0,
            true => LBUFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LBUFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LBUFR::_1
    }
}
#[doc = "Possible values of the field `TSYN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSYNR {
    #[doc = "Timer Sync feature disabled"] _0,
    #[doc = "Timer Sync feature enabled"] _1,
}
impl TSYNR {
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
            TSYNR::_0 => false,
            TSYNR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSYNR {
        match value {
            false => TSYNR::_0,
            true => TSYNR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSYNR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSYNR::_1
    }
}
#[doc = "Possible values of the field `BOFFREC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFRECR {
    #[doc = "Automatic recovering from Bus Off state enabled."] _0,
    #[doc = "Automatic recovering from Bus Off state disabled."] _1,
}
impl BOFFRECR {
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
            BOFFRECR::_0 => false,
            BOFFRECR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOFFRECR {
        match value {
            false => BOFFRECR::_0,
            true => BOFFRECR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BOFFRECR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BOFFRECR::_1
    }
}
#[doc = "Possible values of the field `SMP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMPR {
    #[doc = "Just one sample is used to determine the bit value."] _0,
    #[doc = "Three samples are used to determine the value of the received bit: the regular one (sample point) and 2 preceding samples; a majority rule is used."]
    _1,
}
impl SMPR {
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
            SMPR::_0 => false,
            SMPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SMPR {
        match value {
            false => SMPR::_0,
            true => SMPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SMPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SMPR::_1
    }
}
#[doc = "Possible values of the field `RWRNMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RWRNMSKR {
    #[doc = "Rx Warning Interrupt disabled."] _0,
    #[doc = "Rx Warning Interrupt enabled."] _1,
}
impl RWRNMSKR {
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
            RWRNMSKR::_0 => false,
            RWRNMSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RWRNMSKR {
        match value {
            false => RWRNMSKR::_0,
            true => RWRNMSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RWRNMSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RWRNMSKR::_1
    }
}
#[doc = "Possible values of the field `TWRNMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TWRNMSKR {
    #[doc = "Tx Warning Interrupt disabled."] _0,
    #[doc = "Tx Warning Interrupt enabled."] _1,
}
impl TWRNMSKR {
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
            TWRNMSKR::_0 => false,
            TWRNMSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TWRNMSKR {
        match value {
            false => TWRNMSKR::_0,
            true => TWRNMSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TWRNMSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TWRNMSKR::_1
    }
}
#[doc = "Possible values of the field `LPB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPBR {
    #[doc = "Loop Back disabled."] _0,
    #[doc = "Loop Back enabled."] _1,
}
impl LPBR {
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
            LPBR::_0 => false,
            LPBR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LPBR {
        match value {
            false => LPBR::_0,
            true => LPBR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LPBR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LPBR::_1
    }
}
#[doc = "Possible values of the field `CLKSRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSRCR {
    #[doc = "The CAN engine clock source is the oscillator clock. Under this condition, the oscillator clock frequency must be lower than the bus clock."]
    _0,
    #[doc = "The CAN engine clock source is the peripheral clock."] _1,
}
impl CLKSRCR {
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
            CLKSRCR::_0 => false,
            CLKSRCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CLKSRCR {
        match value {
            false => CLKSRCR::_0,
            true => CLKSRCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CLKSRCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CLKSRCR::_1
    }
}
#[doc = "Possible values of the field `ERRMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRMSKR {
    #[doc = "Error interrupt disabled."] _0,
    #[doc = "Error interrupt enabled."] _1,
}
impl ERRMSKR {
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
            ERRMSKR::_0 => false,
            ERRMSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERRMSKR {
        match value {
            false => ERRMSKR::_0,
            true => ERRMSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERRMSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERRMSKR::_1
    }
}
#[doc = "Possible values of the field `BOFFMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOFFMSKR {
    #[doc = "Bus Off interrupt disabled."] _0,
    #[doc = "Bus Off interrupt enabled."] _1,
}
impl BOFFMSKR {
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
            BOFFMSKR::_0 => false,
            BOFFMSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOFFMSKR {
        match value {
            false => BOFFMSKR::_0,
            true => BOFFMSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BOFFMSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BOFFMSKR::_1
    }
}
#[doc = r" Value of the field"]
pub struct PSEG2R {
    bits: u8,
}
impl PSEG2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PSEG1R {
    bits: u8,
}
impl PSEG1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RJWR {
    bits: u8,
}
impl RJWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PRESDIVR {
    bits: u8,
}
impl PRESDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _PROPSEGW<'a> {
    w: &'a mut W,
}
impl<'a> _PROPSEGW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LOM`"]
pub enum LOMW {
    #[doc = "Listen-Only mode is deactivated."] _0,
    #[doc = "FlexCAN module operates in Listen-Only mode."] _1,
}
impl LOMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LOMW::_0 => false,
            LOMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LOMW<'a> {
    w: &'a mut W,
}
impl<'a> _LOMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LOMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Listen-Only mode is deactivated."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOMW::_0)
    }
    #[doc = "FlexCAN module operates in Listen-Only mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOMW::_1)
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
#[doc = "Values that can be written to the field `LBUF`"]
pub enum LBUFW {
    #[doc = "Buffer with highest priority is transmitted first."] _0,
    #[doc = "Lowest number buffer is transmitted first."] _1,
}
impl LBUFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBUFW::_0 => false,
            LBUFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBUFW<'a> {
    w: &'a mut W,
}
impl<'a> _LBUFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBUFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Buffer with highest priority is transmitted first."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBUFW::_0)
    }
    #[doc = "Lowest number buffer is transmitted first."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBUFW::_1)
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
#[doc = "Values that can be written to the field `TSYN`"]
pub enum TSYNW {
    #[doc = "Timer Sync feature disabled"] _0,
    #[doc = "Timer Sync feature enabled"] _1,
}
impl TSYNW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSYNW::_0 => false,
            TSYNW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSYNW<'a> {
    w: &'a mut W,
}
impl<'a> _TSYNW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSYNW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer Sync feature disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSYNW::_0)
    }
    #[doc = "Timer Sync feature enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSYNW::_1)
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
#[doc = "Values that can be written to the field `BOFFREC`"]
pub enum BOFFRECW {
    #[doc = "Automatic recovering from Bus Off state enabled."] _0,
    #[doc = "Automatic recovering from Bus Off state disabled."] _1,
}
impl BOFFRECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOFFRECW::_0 => false,
            BOFFRECW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOFFRECW<'a> {
    w: &'a mut W,
}
impl<'a> _BOFFRECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOFFRECW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Automatic recovering from Bus Off state enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOFFRECW::_0)
    }
    #[doc = "Automatic recovering from Bus Off state disabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOFFRECW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SMP`"]
pub enum SMPW {
    #[doc = "Just one sample is used to determine the bit value."] _0,
    #[doc = "Three samples are used to determine the value of the received bit: the regular one (sample point) and 2 preceding samples; a majority rule is used."]
    _1,
}
impl SMPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SMPW::_0 => false,
            SMPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMPW<'a> {
    w: &'a mut W,
}
impl<'a> _SMPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Just one sample is used to determine the bit value."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMPW::_0)
    }
    #[doc = "Three samples are used to determine the value of the received bit: the regular one (sample point) and 2 preceding samples; a majority rule is used."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SMPW::_1)
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
#[doc = "Values that can be written to the field `RWRNMSK`"]
pub enum RWRNMSKW {
    #[doc = "Rx Warning Interrupt disabled."] _0,
    #[doc = "Rx Warning Interrupt enabled."] _1,
}
impl RWRNMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RWRNMSKW::_0 => false,
            RWRNMSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RWRNMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _RWRNMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RWRNMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Rx Warning Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWRNMSKW::_0)
    }
    #[doc = "Rx Warning Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWRNMSKW::_1)
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
#[doc = "Values that can be written to the field `TWRNMSK`"]
pub enum TWRNMSKW {
    #[doc = "Tx Warning Interrupt disabled."] _0,
    #[doc = "Tx Warning Interrupt enabled."] _1,
}
impl TWRNMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TWRNMSKW::_0 => false,
            TWRNMSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TWRNMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _TWRNMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TWRNMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Tx Warning Interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TWRNMSKW::_0)
    }
    #[doc = "Tx Warning Interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TWRNMSKW::_1)
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
#[doc = "Values that can be written to the field `LPB`"]
pub enum LPBW {
    #[doc = "Loop Back disabled."] _0,
    #[doc = "Loop Back enabled."] _1,
}
impl LPBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LPBW::_0 => false,
            LPBW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LPBW<'a> {
    w: &'a mut W,
}
impl<'a> _LPBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LPBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Loop Back disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPBW::_0)
    }
    #[doc = "Loop Back enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPBW::_1)
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
#[doc = "Values that can be written to the field `CLKSRC`"]
pub enum CLKSRCW {
    #[doc = "The CAN engine clock source is the oscillator clock. Under this condition, the oscillator clock frequency must be lower than the bus clock."]
    _0,
    #[doc = "The CAN engine clock source is the peripheral clock."] _1,
}
impl CLKSRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLKSRCW::_0 => false,
            CLKSRCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLKSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _CLKSRCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLKSRCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The CAN engine clock source is the oscillator clock. Under this condition, the oscillator clock frequency must be lower than the bus clock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKSRCW::_0)
    }
    #[doc = "The CAN engine clock source is the peripheral clock."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKSRCW::_1)
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
#[doc = "Values that can be written to the field `ERRMSK`"]
pub enum ERRMSKW {
    #[doc = "Error interrupt disabled."] _0,
    #[doc = "Error interrupt enabled."] _1,
}
impl ERRMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERRMSKW::_0 => false,
            ERRMSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERRMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _ERRMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERRMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Error interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERRMSKW::_0)
    }
    #[doc = "Error interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERRMSKW::_1)
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
#[doc = "Values that can be written to the field `BOFFMSK`"]
pub enum BOFFMSKW {
    #[doc = "Bus Off interrupt disabled."] _0,
    #[doc = "Bus Off interrupt enabled."] _1,
}
impl BOFFMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOFFMSKW::_0 => false,
            BOFFMSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOFFMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _BOFFMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOFFMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Bus Off interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOFFMSKW::_0)
    }
    #[doc = "Bus Off interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOFFMSKW::_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PSEG2W<'a> {
    w: &'a mut W,
}
impl<'a> _PSEG2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PSEG1W<'a> {
    w: &'a mut W,
}
impl<'a> _PSEG1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RJWW<'a> {
    w: &'a mut W,
}
impl<'a> _RJWW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PRESDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESDIVW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:2 - Propagation Segment"]
    #[inline]
    pub fn propseg(&self) -> PROPSEGR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PROPSEGR { bits }
    }
    #[doc = "Bit 3 - Listen-Only Mode"]
    #[inline]
    pub fn lom(&self) -> LOMR {
        LOMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - Lowest Buffer Transmitted First"]
    #[inline]
    pub fn lbuf(&self) -> LBUFR {
        LBUFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Timer Sync"]
    #[inline]
    pub fn tsyn(&self) -> TSYNR {
        TSYNR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Bus Off Recovery"]
    #[inline]
    pub fn boffrec(&self) -> BOFFRECR {
        BOFFRECR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - CAN Bit Sampling"]
    #[inline]
    pub fn smp(&self) -> SMPR {
        SMPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Rx Warning Interrupt Mask"]
    #[inline]
    pub fn rwrnmsk(&self) -> RWRNMSKR {
        RWRNMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Tx Warning Interrupt Mask"]
    #[inline]
    pub fn twrnmsk(&self) -> TWRNMSKR {
        TWRNMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Loop Back Mode"]
    #[inline]
    pub fn lpb(&self) -> LPBR {
        LPBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - CAN Engine Clock Source"]
    #[inline]
    pub fn clksrc(&self) -> CLKSRCR {
        CLKSRCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Error Interrupt Mask"]
    #[inline]
    pub fn errmsk(&self) -> ERRMSKR {
        ERRMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Bus Off Interrupt Mask"]
    #[inline]
    pub fn boffmsk(&self) -> BOFFMSKR {
        BOFFMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:18 - Phase Segment 2"]
    #[inline]
    pub fn pseg2(&self) -> PSEG2R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PSEG2R { bits }
    }
    #[doc = "Bits 19:21 - Phase Segment 1"]
    #[inline]
    pub fn pseg1(&self) -> PSEG1R {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PSEG1R { bits }
    }
    #[doc = "Bits 22:23 - Resync Jump Width"]
    #[inline]
    pub fn rjw(&self) -> RJWR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RJWR { bits }
    }
    #[doc = "Bits 24:31 - Prescaler Division Factor"]
    #[inline]
    pub fn presdiv(&self) -> PRESDIVR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PRESDIVR { bits }
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
    #[doc = "Bits 0:2 - Propagation Segment"]
    #[inline]
    pub fn propseg(&mut self) -> _PROPSEGW {
        _PROPSEGW { w: self }
    }
    #[doc = "Bit 3 - Listen-Only Mode"]
    #[inline]
    pub fn lom(&mut self) -> _LOMW {
        _LOMW { w: self }
    }
    #[doc = "Bit 4 - Lowest Buffer Transmitted First"]
    #[inline]
    pub fn lbuf(&mut self) -> _LBUFW {
        _LBUFW { w: self }
    }
    #[doc = "Bit 5 - Timer Sync"]
    #[inline]
    pub fn tsyn(&mut self) -> _TSYNW {
        _TSYNW { w: self }
    }
    #[doc = "Bit 6 - Bus Off Recovery"]
    #[inline]
    pub fn boffrec(&mut self) -> _BOFFRECW {
        _BOFFRECW { w: self }
    }
    #[doc = "Bit 7 - CAN Bit Sampling"]
    #[inline]
    pub fn smp(&mut self) -> _SMPW {
        _SMPW { w: self }
    }
    #[doc = "Bit 10 - Rx Warning Interrupt Mask"]
    #[inline]
    pub fn rwrnmsk(&mut self) -> _RWRNMSKW {
        _RWRNMSKW { w: self }
    }
    #[doc = "Bit 11 - Tx Warning Interrupt Mask"]
    #[inline]
    pub fn twrnmsk(&mut self) -> _TWRNMSKW {
        _TWRNMSKW { w: self }
    }
    #[doc = "Bit 12 - Loop Back Mode"]
    #[inline]
    pub fn lpb(&mut self) -> _LPBW {
        _LPBW { w: self }
    }
    #[doc = "Bit 13 - CAN Engine Clock Source"]
    #[inline]
    pub fn clksrc(&mut self) -> _CLKSRCW {
        _CLKSRCW { w: self }
    }
    #[doc = "Bit 14 - Error Interrupt Mask"]
    #[inline]
    pub fn errmsk(&mut self) -> _ERRMSKW {
        _ERRMSKW { w: self }
    }
    #[doc = "Bit 15 - Bus Off Interrupt Mask"]
    #[inline]
    pub fn boffmsk(&mut self) -> _BOFFMSKW {
        _BOFFMSKW { w: self }
    }
    #[doc = "Bits 16:18 - Phase Segment 2"]
    #[inline]
    pub fn pseg2(&mut self) -> _PSEG2W {
        _PSEG2W { w: self }
    }
    #[doc = "Bits 19:21 - Phase Segment 1"]
    #[inline]
    pub fn pseg1(&mut self) -> _PSEG1W {
        _PSEG1W { w: self }
    }
    #[doc = "Bits 22:23 - Resync Jump Width"]
    #[inline]
    pub fn rjw(&mut self) -> _RJWW {
        _RJWW { w: self }
    }
    #[doc = "Bits 24:31 - Prescaler Division Factor"]
    #[inline]
    pub fn presdiv(&mut self) -> _PRESDIVW {
        _PRESDIVW { w: self }
    }
}
