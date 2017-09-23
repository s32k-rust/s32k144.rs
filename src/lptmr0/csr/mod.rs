#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSR {
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
#[doc = "Possible values of the field `TEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TENR {
    #[doc = "LPTMR is disabled and internal logic is reset."] _0,
    #[doc = "LPTMR is enabled."] _1,
}
impl TENR {
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
            TENR::_0 => false,
            TENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TENR {
        match value {
            false => TENR::_0,
            true => TENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TENR::_1
    }
}
#[doc = "Possible values of the field `TMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMSR {
    #[doc = "Time Counter mode."] _0,
    #[doc = "Pulse Counter mode."] _1,
}
impl TMSR {
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
            TMSR::_0 => false,
            TMSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TMSR {
        match value {
            false => TMSR::_0,
            true => TMSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TMSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TMSR::_1
    }
}
#[doc = "Possible values of the field `TFC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFCR {
    #[doc = "CNR is reset whenever TCF is set."] _0,
    #[doc = "CNR is reset on overflow."] _1,
}
impl TFCR {
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
            TFCR::_0 => false,
            TFCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TFCR {
        match value {
            false => TFCR::_0,
            true => TFCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TFCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TFCR::_1
    }
}
#[doc = "Possible values of the field `TPP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPPR {
    #[doc = "Pulse Counter input source is active-high, and the CNR will increment on the rising-edge."]
    _0,
    #[doc = "Pulse Counter input source is active-low, and the CNR will increment on the falling-edge."]
    _1,
}
impl TPPR {
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
            TPPR::_0 => false,
            TPPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TPPR {
        match value {
            false => TPPR::_0,
            true => TPPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TPPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TPPR::_1
    }
}
#[doc = "Possible values of the field `TPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TPSR {
    #[doc = "Pulse counter input 0 is selected."] _00,
    #[doc = "Pulse counter input 1 is selected."] _01,
    #[doc = "Pulse counter input 2 is selected."] _10,
    #[doc = "Pulse counter input 3 is selected."] _11,
}
impl TPSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TPSR::_00 => 0,
            TPSR::_01 => 1,
            TPSR::_10 => 2,
            TPSR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TPSR {
        match value {
            0 => TPSR::_00,
            1 => TPSR::_01,
            2 => TPSR::_10,
            3 => TPSR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == TPSR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == TPSR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TPSR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TPSR::_11
    }
}
#[doc = "Possible values of the field `TIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIER {
    #[doc = "Timer interrupt disabled."] _0,
    #[doc = "Timer interrupt enabled."] _1,
}
impl TIER {
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
            TIER::_0 => false,
            TIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TIER {
        match value {
            false => TIER::_0,
            true => TIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIER::_1
    }
}
#[doc = "Possible values of the field `TCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCFR {
    #[doc = "The value of CNR is not equal to CMR and increments."] _0,
    #[doc = "The value of CNR is equal to CMR and increments."] _1,
}
impl TCFR {
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
            TCFR::_0 => false,
            TCFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCFR {
        match value {
            false => TCFR::_0,
            true => TCFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TCFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TCFR::_1
    }
}
#[doc = "Possible values of the field `TDRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRER {
    #[doc = "Timer DMA Request disabled."] _0,
    #[doc = "Timer DMA Request enabled."] _1,
}
impl TDRER {
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
            TDRER::_0 => false,
            TDRER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDRER {
        match value {
            false => TDRER::_0,
            true => TDRER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TDRER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TDRER::_1
    }
}
#[doc = "Values that can be written to the field `TEN`"]
pub enum TENW {
    #[doc = "LPTMR is disabled and internal logic is reset."] _0,
    #[doc = "LPTMR is enabled."] _1,
}
impl TENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TENW::_0 => false,
            TENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TENW<'a> {
    w: &'a mut W,
}
impl<'a> _TENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "LPTMR is disabled and internal logic is reset."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TENW::_0)
    }
    #[doc = "LPTMR is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TENW::_1)
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
#[doc = "Values that can be written to the field `TMS`"]
pub enum TMSW {
    #[doc = "Time Counter mode."] _0,
    #[doc = "Pulse Counter mode."] _1,
}
impl TMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TMSW::_0 => false,
            TMSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TMSW<'a> {
    w: &'a mut W,
}
impl<'a> _TMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TMSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Time Counter mode."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMSW::_0)
    }
    #[doc = "Pulse Counter mode."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMSW::_1)
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
#[doc = "Values that can be written to the field `TFC`"]
pub enum TFCW {
    #[doc = "CNR is reset whenever TCF is set."] _0,
    #[doc = "CNR is reset on overflow."] _1,
}
impl TFCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TFCW::_0 => false,
            TFCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TFCW<'a> {
    w: &'a mut W,
}
impl<'a> _TFCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TFCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "CNR is reset whenever TCF is set."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFCW::_0)
    }
    #[doc = "CNR is reset on overflow."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFCW::_1)
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
#[doc = "Values that can be written to the field `TPP`"]
pub enum TPPW {
    #[doc = "Pulse Counter input source is active-high, and the CNR will increment on the rising-edge."]
    _0,
    #[doc = "Pulse Counter input source is active-low, and the CNR will increment on the falling-edge."]
    _1,
}
impl TPPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TPPW::_0 => false,
            TPPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPPW<'a> {
    w: &'a mut W,
}
impl<'a> _TPPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Pulse Counter input source is active-high, and the CNR will increment on the rising-edge."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPPW::_0)
    }
    #[doc = "Pulse Counter input source is active-low, and the CNR will increment on the falling-edge."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPPW::_1)
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
#[doc = "Values that can be written to the field `TPS`"]
pub enum TPSW {
    #[doc = "Pulse counter input 0 is selected."] _00,
    #[doc = "Pulse counter input 1 is selected."] _01,
    #[doc = "Pulse counter input 2 is selected."] _10,
    #[doc = "Pulse counter input 3 is selected."] _11,
}
impl TPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TPSW::_00 => 0,
            TPSW::_01 => 1,
            TPSW::_10 => 2,
            TPSW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TPSW<'a> {
    w: &'a mut W,
}
impl<'a> _TPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TPSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pulse counter input 0 is selected."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(TPSW::_00)
    }
    #[doc = "Pulse counter input 1 is selected."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(TPSW::_01)
    }
    #[doc = "Pulse counter input 2 is selected."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TPSW::_10)
    }
    #[doc = "Pulse counter input 3 is selected."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TPSW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIE`"]
pub enum TIEW {
    #[doc = "Timer interrupt disabled."] _0,
    #[doc = "Timer interrupt enabled."] _1,
}
impl TIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TIEW::_0 => false,
            TIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIEW<'a> {
    w: &'a mut W,
}
impl<'a> _TIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIEW::_0)
    }
    #[doc = "Timer interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIEW::_1)
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
#[doc = "Values that can be written to the field `TCF`"]
pub enum TCFW {
    #[doc = "The value of CNR is not equal to CMR and increments."] _0,
    #[doc = "The value of CNR is equal to CMR and increments."] _1,
}
impl TCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCFW::_0 => false,
            TCFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCFW<'a> {
    w: &'a mut W,
}
impl<'a> _TCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The value of CNR is not equal to CMR and increments."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCFW::_0)
    }
    #[doc = "The value of CNR is equal to CMR and increments."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCFW::_1)
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
#[doc = "Values that can be written to the field `TDRE`"]
pub enum TDREW {
    #[doc = "Timer DMA Request disabled."] _0,
    #[doc = "Timer DMA Request enabled."] _1,
}
impl TDREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDREW::_0 => false,
            TDREW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDREW<'a> {
    w: &'a mut W,
}
impl<'a> _TDREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer DMA Request disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDREW::_0)
    }
    #[doc = "Timer DMA Request enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDREW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Timer Enable"]
    #[inline]
    pub fn ten(&self) -> TENR {
        TENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Timer Mode Select"]
    #[inline]
    pub fn tms(&self) -> TMSR {
        TMSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Timer Free-Running Counter"]
    #[inline]
    pub fn tfc(&self) -> TFCR {
        TFCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Timer Pin Polarity"]
    #[inline]
    pub fn tpp(&self) -> TPPR {
        TPPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Timer Pin Select"]
    #[inline]
    pub fn tps(&self) -> TPSR {
        TPSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline]
    pub fn tie(&self) -> TIER {
        TIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Timer Compare Flag"]
    #[inline]
    pub fn tcf(&self) -> TCFR {
        TCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Timer DMA Request Enable"]
    #[inline]
    pub fn tdre(&self) -> TDRER {
        TDRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - Timer Enable"]
    #[inline]
    pub fn ten(&mut self) -> _TENW {
        _TENW { w: self }
    }
    #[doc = "Bit 1 - Timer Mode Select"]
    #[inline]
    pub fn tms(&mut self) -> _TMSW {
        _TMSW { w: self }
    }
    #[doc = "Bit 2 - Timer Free-Running Counter"]
    #[inline]
    pub fn tfc(&mut self) -> _TFCW {
        _TFCW { w: self }
    }
    #[doc = "Bit 3 - Timer Pin Polarity"]
    #[inline]
    pub fn tpp(&mut self) -> _TPPW {
        _TPPW { w: self }
    }
    #[doc = "Bits 4:5 - Timer Pin Select"]
    #[inline]
    pub fn tps(&mut self) -> _TPSW {
        _TPSW { w: self }
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline]
    pub fn tie(&mut self) -> _TIEW {
        _TIEW { w: self }
    }
    #[doc = "Bit 7 - Timer Compare Flag"]
    #[inline]
    pub fn tcf(&mut self) -> _TCFW {
        _TCFW { w: self }
    }
    #[doc = "Bit 8 - Timer DMA Request Enable"]
    #[inline]
    pub fn tdre(&mut self) -> _TDREW {
        _TDREW { w: self }
    }
}
