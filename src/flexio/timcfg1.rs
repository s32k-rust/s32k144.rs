#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TIMCFG1 {
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
#[doc = "Possible values of the field `TSTART`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTARTR {
    #[doc = "Start bit disabled"]
    _0,
    #[doc = "Start bit enabled"]
    _1,
}
impl TSTARTR {
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
            TSTARTR::_0 => false,
            TSTARTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TSTARTR {
        match value {
            false => TSTARTR::_0,
            true => TSTARTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSTARTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSTARTR::_1
    }
}
#[doc = "Possible values of the field `TSTOP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSTOPR {
    #[doc = "Stop bit disabled"]
    _0,
    #[doc = "Stop bit is enabled on timer compare"]
    _1,
    #[doc = "Stop bit is enabled on timer disable"]
    _10,
    #[doc = "Stop bit is enabled on timer compare and timer disable"]
    _11,
}
impl TSTOPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TSTOPR::_0 => 0,
            TSTOPR::_1 => 1,
            TSTOPR::_10 => 2,
            TSTOPR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TSTOPR {
        match value {
            0 => TSTOPR::_0,
            1 => TSTOPR::_1,
            2 => TSTOPR::_10,
            3 => TSTOPR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TSTOPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TSTOPR::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TSTOPR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TSTOPR::_11
    }
}
#[doc = "Possible values of the field `TIMENA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMENAR {
    #[doc = "Timer always enabled"]
    _0,
    #[doc = "Timer enabled on Timer N-1 enable"]
    _1,
    #[doc = "Timer enabled on Trigger high"]
    _10,
    #[doc = "Timer enabled on Trigger high and Pin high"]
    _11,
    #[doc = "Timer enabled on Pin rising edge"]
    _100,
    #[doc = "Timer enabled on Pin rising edge and Trigger high"]
    _101,
    #[doc = "Timer enabled on Trigger rising edge"]
    _110,
    #[doc = "Timer enabled on Trigger rising or falling edge"]
    _111,
}
impl TIMENAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMENAR::_0 => 0,
            TIMENAR::_1 => 1,
            TIMENAR::_10 => 2,
            TIMENAR::_11 => 3,
            TIMENAR::_100 => 4,
            TIMENAR::_101 => 5,
            TIMENAR::_110 => 6,
            TIMENAR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMENAR {
        match value {
            0 => TIMENAR::_0,
            1 => TIMENAR::_1,
            2 => TIMENAR::_10,
            3 => TIMENAR::_11,
            4 => TIMENAR::_100,
            5 => TIMENAR::_101,
            6 => TIMENAR::_110,
            7 => TIMENAR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIMENAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIMENAR::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TIMENAR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TIMENAR::_11
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == TIMENAR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == TIMENAR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == TIMENAR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == TIMENAR::_111
    }
}
#[doc = "Possible values of the field `TIMDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMDISR {
    #[doc = "Timer never disabled"]
    _0,
    #[doc = "Timer disabled on Timer N-1 disable"]
    _1,
    #[doc = "Timer disabled on Timer compare"]
    _10,
    #[doc = "Timer disabled on Timer compare and Trigger Low"]
    _11,
    #[doc = "Timer disabled on Pin rising or falling edge"]
    _100,
    #[doc = "Timer disabled on Pin rising or falling edge provided Trigger is high"]
    _101,
    #[doc = "Timer disabled on Trigger falling edge"]
    _110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TIMDISR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMDISR::_0 => 0,
            TIMDISR::_1 => 1,
            TIMDISR::_10 => 2,
            TIMDISR::_11 => 3,
            TIMDISR::_100 => 4,
            TIMDISR::_101 => 5,
            TIMDISR::_110 => 6,
            TIMDISR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMDISR {
        match value {
            0 => TIMDISR::_0,
            1 => TIMDISR::_1,
            2 => TIMDISR::_10,
            3 => TIMDISR::_11,
            4 => TIMDISR::_100,
            5 => TIMDISR::_101,
            6 => TIMDISR::_110,
            i => TIMDISR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIMDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIMDISR::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TIMDISR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TIMDISR::_11
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == TIMDISR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == TIMDISR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == TIMDISR::_110
    }
}
#[doc = "Possible values of the field `TIMRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMRSTR {
    #[doc = "Timer never reset"]
    _0,
    #[doc = "Timer reset on Timer Pin equal to Timer Output"]
    _10,
    #[doc = "Timer reset on Timer Trigger equal to Timer Output"]
    _11,
    #[doc = "Timer reset on Timer Pin rising edge"]
    _100,
    #[doc = "Timer reset on Trigger rising edge"]
    _110,
    #[doc = "Timer reset on Trigger rising or falling edge"]
    _111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TIMRSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMRSTR::_0 => 0,
            TIMRSTR::_10 => 2,
            TIMRSTR::_11 => 3,
            TIMRSTR::_100 => 4,
            TIMRSTR::_110 => 6,
            TIMRSTR::_111 => 7,
            TIMRSTR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMRSTR {
        match value {
            0 => TIMRSTR::_0,
            2 => TIMRSTR::_10,
            3 => TIMRSTR::_11,
            4 => TIMRSTR::_100,
            6 => TIMRSTR::_110,
            7 => TIMRSTR::_111,
            i => TIMRSTR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIMRSTR::_0
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TIMRSTR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TIMRSTR::_11
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == TIMRSTR::_100
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == TIMRSTR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == TIMRSTR::_111
    }
}
#[doc = "Possible values of the field `TIMDEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMDECR {
    #[doc = "Decrement counter on FlexIO clock, Shift clock equals Timer output."]
    _0,
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Timer output."]
    _1,
    #[doc = "Decrement counter on Pin input (both edges), Shift clock equals Pin input."]
    _10,
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Trigger input."]
    _11,
}
impl TIMDECR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMDECR::_0 => 0,
            TIMDECR::_1 => 1,
            TIMDECR::_10 => 2,
            TIMDECR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMDECR {
        match value {
            0 => TIMDECR::_0,
            1 => TIMDECR::_1,
            2 => TIMDECR::_10,
            3 => TIMDECR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIMDECR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIMDECR::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TIMDECR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TIMDECR::_11
    }
}
#[doc = "Possible values of the field `TIMOUT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMOUTR {
    #[doc = "Timer output is logic one when enabled and is not affected by timer reset"]
    _0,
    #[doc = "Timer output is logic zero when enabled and is not affected by timer reset"]
    _1,
    #[doc = "Timer output is logic one when enabled and on timer reset"]
    _10,
    #[doc = "Timer output is logic zero when enabled and on timer reset"]
    _11,
}
impl TIMOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TIMOUTR::_0 => 0,
            TIMOUTR::_1 => 1,
            TIMOUTR::_10 => 2,
            TIMOUTR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TIMOUTR {
        match value {
            0 => TIMOUTR::_0,
            1 => TIMOUTR::_1,
            2 => TIMOUTR::_10,
            3 => TIMOUTR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TIMOUTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TIMOUTR::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == TIMOUTR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == TIMOUTR::_11
    }
}
#[doc = "Values that can be written to the field `TSTART`"]
pub enum TSTARTW {
    #[doc = "Start bit disabled"]
    _0,
    #[doc = "Start bit enabled"]
    _1,
}
impl TSTARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TSTARTW::_0 => false,
            TSTARTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSTARTW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSTARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Start bit disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSTARTW::_0)
    }
    #[doc = "Start bit enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSTARTW::_1)
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
#[doc = "Values that can be written to the field `TSTOP`"]
pub enum TSTOPW {
    #[doc = "Stop bit disabled"]
    _0,
    #[doc = "Stop bit is enabled on timer compare"]
    _1,
    #[doc = "Stop bit is enabled on timer disable"]
    _10,
    #[doc = "Stop bit is enabled on timer compare and timer disable"]
    _11,
}
impl TSTOPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TSTOPW::_0 => 0,
            TSTOPW::_1 => 1,
            TSTOPW::_10 => 2,
            TSTOPW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TSTOPW<'a> {
    w: &'a mut W,
}
impl<'a> _TSTOPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TSTOPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Stop bit disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSTOPW::_0)
    }
    #[doc = "Stop bit is enabled on timer compare"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSTOPW::_1)
    }
    #[doc = "Stop bit is enabled on timer disable"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TSTOPW::_10)
    }
    #[doc = "Stop bit is enabled on timer compare and timer disable"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TSTOPW::_11)
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
#[doc = "Values that can be written to the field `TIMENA`"]
pub enum TIMENAW {
    #[doc = "Timer always enabled"]
    _0,
    #[doc = "Timer enabled on Timer N-1 enable"]
    _1,
    #[doc = "Timer enabled on Trigger high"]
    _10,
    #[doc = "Timer enabled on Trigger high and Pin high"]
    _11,
    #[doc = "Timer enabled on Pin rising edge"]
    _100,
    #[doc = "Timer enabled on Pin rising edge and Trigger high"]
    _101,
    #[doc = "Timer enabled on Trigger rising edge"]
    _110,
    #[doc = "Timer enabled on Trigger rising or falling edge"]
    _111,
}
impl TIMENAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMENAW::_0 => 0,
            TIMENAW::_1 => 1,
            TIMENAW::_10 => 2,
            TIMENAW::_11 => 3,
            TIMENAW::_100 => 4,
            TIMENAW::_101 => 5,
            TIMENAW::_110 => 6,
            TIMENAW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMENAW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMENAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMENAW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer always enabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMENAW::_0)
    }
    #[doc = "Timer enabled on Timer N-1 enable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMENAW::_1)
    }
    #[doc = "Timer enabled on Trigger high"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TIMENAW::_10)
    }
    #[doc = "Timer enabled on Trigger high and Pin high"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TIMENAW::_11)
    }
    #[doc = "Timer enabled on Pin rising edge"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(TIMENAW::_100)
    }
    #[doc = "Timer enabled on Pin rising edge and Trigger high"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(TIMENAW::_101)
    }
    #[doc = "Timer enabled on Trigger rising edge"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(TIMENAW::_110)
    }
    #[doc = "Timer enabled on Trigger rising or falling edge"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(TIMENAW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMDIS`"]
pub enum TIMDISW {
    #[doc = "Timer never disabled"]
    _0,
    #[doc = "Timer disabled on Timer N-1 disable"]
    _1,
    #[doc = "Timer disabled on Timer compare"]
    _10,
    #[doc = "Timer disabled on Timer compare and Trigger Low"]
    _11,
    #[doc = "Timer disabled on Pin rising or falling edge"]
    _100,
    #[doc = "Timer disabled on Pin rising or falling edge provided Trigger is high"]
    _101,
    #[doc = "Timer disabled on Trigger falling edge"]
    _110,
}
impl TIMDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMDISW::_0 => 0,
            TIMDISW::_1 => 1,
            TIMDISW::_10 => 2,
            TIMDISW::_11 => 3,
            TIMDISW::_100 => 4,
            TIMDISW::_101 => 5,
            TIMDISW::_110 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMDISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMDISW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timer never disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMDISW::_0)
    }
    #[doc = "Timer disabled on Timer N-1 disable"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMDISW::_1)
    }
    #[doc = "Timer disabled on Timer compare"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TIMDISW::_10)
    }
    #[doc = "Timer disabled on Timer compare and Trigger Low"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TIMDISW::_11)
    }
    #[doc = "Timer disabled on Pin rising or falling edge"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(TIMDISW::_100)
    }
    #[doc = "Timer disabled on Pin rising or falling edge provided Trigger is high"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(TIMDISW::_101)
    }
    #[doc = "Timer disabled on Trigger falling edge"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(TIMDISW::_110)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMRST`"]
pub enum TIMRSTW {
    #[doc = "Timer never reset"]
    _0,
    #[doc = "Timer reset on Timer Pin equal to Timer Output"]
    _10,
    #[doc = "Timer reset on Timer Trigger equal to Timer Output"]
    _11,
    #[doc = "Timer reset on Timer Pin rising edge"]
    _100,
    #[doc = "Timer reset on Trigger rising edge"]
    _110,
    #[doc = "Timer reset on Trigger rising or falling edge"]
    _111,
}
impl TIMRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMRSTW::_0 => 0,
            TIMRSTW::_10 => 2,
            TIMRSTW::_11 => 3,
            TIMRSTW::_100 => 4,
            TIMRSTW::_110 => 6,
            TIMRSTW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMRSTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Timer never reset"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMRSTW::_0)
    }
    #[doc = "Timer reset on Timer Pin equal to Timer Output"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TIMRSTW::_10)
    }
    #[doc = "Timer reset on Timer Trigger equal to Timer Output"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TIMRSTW::_11)
    }
    #[doc = "Timer reset on Timer Pin rising edge"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(TIMRSTW::_100)
    }
    #[doc = "Timer reset on Trigger rising edge"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(TIMRSTW::_110)
    }
    #[doc = "Timer reset on Trigger rising or falling edge"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(TIMRSTW::_111)
    }
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
#[doc = "Values that can be written to the field `TIMDEC`"]
pub enum TIMDECW {
    #[doc = "Decrement counter on FlexIO clock, Shift clock equals Timer output."]
    _0,
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Timer output."]
    _1,
    #[doc = "Decrement counter on Pin input (both edges), Shift clock equals Pin input."]
    _10,
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Trigger input."]
    _11,
}
impl TIMDECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMDECW::_0 => 0,
            TIMDECW::_1 => 1,
            TIMDECW::_10 => 2,
            TIMDECW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMDECW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMDECW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMDECW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Decrement counter on FlexIO clock, Shift clock equals Timer output."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMDECW::_0)
    }
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Timer output."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMDECW::_1)
    }
    #[doc = "Decrement counter on Pin input (both edges), Shift clock equals Pin input."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TIMDECW::_10)
    }
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Trigger input."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TIMDECW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TIMOUT`"]
pub enum TIMOUTW {
    #[doc = "Timer output is logic one when enabled and is not affected by timer reset"]
    _0,
    #[doc = "Timer output is logic zero when enabled and is not affected by timer reset"]
    _1,
    #[doc = "Timer output is logic one when enabled and on timer reset"]
    _10,
    #[doc = "Timer output is logic zero when enabled and on timer reset"]
    _11,
}
impl TIMOUTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TIMOUTW::_0 => 0,
            TIMOUTW::_1 => 1,
            TIMOUTW::_10 => 2,
            TIMOUTW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TIMOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMOUTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TIMOUTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Timer output is logic one when enabled and is not affected by timer reset"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIMOUTW::_0)
    }
    #[doc = "Timer output is logic zero when enabled and is not affected by timer reset"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIMOUTW::_1)
    }
    #[doc = "Timer output is logic one when enabled and on timer reset"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(TIMOUTW::_10)
    }
    #[doc = "Timer output is logic zero when enabled and on timer reset"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(TIMOUTW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bit 1 - Timer Start Bit"]
    #[inline]
    pub fn tstart(&self) -> TSTARTR {
        TSTARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:5 - Timer Stop Bit"]
    #[inline]
    pub fn tstop(&self) -> TSTOPR {
        TSTOPR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:10 - Timer Enable"]
    #[inline]
    pub fn timena(&self) -> TIMENAR {
        TIMENAR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - Timer Disable"]
    #[inline]
    pub fn timdis(&self) -> TIMDISR {
        TIMDISR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:18 - Timer Reset"]
    #[inline]
    pub fn timrst(&self) -> TIMRSTR {
        TIMRSTR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Timer Decrement"]
    #[inline]
    pub fn timdec(&self) -> TIMDECR {
        TIMDECR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Timer Output"]
    #[inline]
    pub fn timout(&self) -> TIMOUTR {
        TIMOUTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
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
    #[doc = "Bit 1 - Timer Start Bit"]
    #[inline]
    pub fn tstart(&mut self) -> _TSTARTW {
        _TSTARTW { w: self }
    }
    #[doc = "Bits 4:5 - Timer Stop Bit"]
    #[inline]
    pub fn tstop(&mut self) -> _TSTOPW {
        _TSTOPW { w: self }
    }
    #[doc = "Bits 8:10 - Timer Enable"]
    #[inline]
    pub fn timena(&mut self) -> _TIMENAW {
        _TIMENAW { w: self }
    }
    #[doc = "Bits 12:14 - Timer Disable"]
    #[inline]
    pub fn timdis(&mut self) -> _TIMDISW {
        _TIMDISW { w: self }
    }
    #[doc = "Bits 16:18 - Timer Reset"]
    #[inline]
    pub fn timrst(&mut self) -> _TIMRSTW {
        _TIMRSTW { w: self }
    }
    #[doc = "Bits 20:21 - Timer Decrement"]
    #[inline]
    pub fn timdec(&mut self) -> _TIMDECW {
        _TIMDECW { w: self }
    }
    #[doc = "Bits 24:25 - Timer Output"]
    #[inline]
    pub fn timout(&mut self) -> _TIMOUTW {
        _TIMOUTW { w: self }
    }
}
