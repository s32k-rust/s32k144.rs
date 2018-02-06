#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PLATCGC {
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
#[doc = "Possible values of the field `CGCMSCM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGCMSCMR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl CGCMSCMR {
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
            CGCMSCMR::_0 => false,
            CGCMSCMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CGCMSCMR {
        match value {
            false => CGCMSCMR::_0,
            true => CGCMSCMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CGCMSCMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CGCMSCMR::_1
    }
}
#[doc = "Possible values of the field `CGCMPU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGCMPUR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl CGCMPUR {
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
            CGCMPUR::_0 => false,
            CGCMPUR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CGCMPUR {
        match value {
            false => CGCMPUR::_0,
            true => CGCMPUR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CGCMPUR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CGCMPUR::_1
    }
}
#[doc = "Possible values of the field `CGCDMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGCDMAR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl CGCDMAR {
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
            CGCDMAR::_0 => false,
            CGCDMAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CGCDMAR {
        match value {
            false => CGCDMAR::_0,
            true => CGCDMAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CGCDMAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CGCDMAR::_1
    }
}
#[doc = "Possible values of the field `CGCERM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGCERMR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl CGCERMR {
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
            CGCERMR::_0 => false,
            CGCERMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CGCERMR {
        match value {
            false => CGCERMR::_0,
            true => CGCERMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CGCERMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CGCERMR::_1
    }
}
#[doc = "Possible values of the field `CGCEIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CGCEIMR {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl CGCEIMR {
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
            CGCEIMR::_0 => false,
            CGCEIMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CGCEIMR {
        match value {
            false => CGCEIMR::_0,
            true => CGCEIMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CGCEIMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CGCEIMR::_1
    }
}
#[doc = "Values that can be written to the field `CGCMSCM`"]
pub enum CGCMSCMW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl CGCMSCMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CGCMSCMW::_0 => false,
            CGCMSCMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CGCMSCMW<'a> {
    w: &'a mut W,
}
impl<'a> _CGCMSCMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGCMSCMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CGCMSCMW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CGCMSCMW::_1)
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
#[doc = "Values that can be written to the field `CGCMPU`"]
pub enum CGCMPUW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl CGCMPUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CGCMPUW::_0 => false,
            CGCMPUW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CGCMPUW<'a> {
    w: &'a mut W,
}
impl<'a> _CGCMPUW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGCMPUW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CGCMPUW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CGCMPUW::_1)
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
#[doc = "Values that can be written to the field `CGCDMA`"]
pub enum CGCDMAW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl CGCDMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CGCDMAW::_0 => false,
            CGCDMAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CGCDMAW<'a> {
    w: &'a mut W,
}
impl<'a> _CGCDMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGCDMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CGCDMAW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CGCDMAW::_1)
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
#[doc = "Values that can be written to the field `CGCERM`"]
pub enum CGCERMW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl CGCERMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CGCERMW::_0 => false,
            CGCERMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CGCERMW<'a> {
    w: &'a mut W,
}
impl<'a> _CGCERMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGCERMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CGCERMW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CGCERMW::_1)
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
#[doc = "Values that can be written to the field `CGCEIM`"]
pub enum CGCEIMW {
    #[doc = "Clock disabled"]
    _0,
    #[doc = "Clock enabled"]
    _1,
}
impl CGCEIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CGCEIMW::_0 => false,
            CGCEIMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CGCEIMW<'a> {
    w: &'a mut W,
}
impl<'a> _CGCEIMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CGCEIMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CGCEIMW::_0)
    }
    #[doc = "Clock enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CGCEIMW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - MSCM Clock Gating Control"]
    #[inline]
    pub fn cgcmscm(&self) -> CGCMSCMR {
        CGCMSCMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - MPU Clock Gating Control"]
    #[inline]
    pub fn cgcmpu(&self) -> CGCMPUR {
        CGCMPUR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - DMA Clock Gating Control"]
    #[inline]
    pub fn cgcdma(&self) -> CGCDMAR {
        CGCDMAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - ERM Clock Gating Control"]
    #[inline]
    pub fn cgcerm(&self) -> CGCERMR {
        CGCERMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 4 - EIM Clock Gating Control"]
    #[inline]
    pub fn cgceim(&self) -> CGCEIMR {
        CGCEIMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 31 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - MSCM Clock Gating Control"]
    #[inline]
    pub fn cgcmscm(&mut self) -> _CGCMSCMW {
        _CGCMSCMW { w: self }
    }
    #[doc = "Bit 1 - MPU Clock Gating Control"]
    #[inline]
    pub fn cgcmpu(&mut self) -> _CGCMPUW {
        _CGCMPUW { w: self }
    }
    #[doc = "Bit 2 - DMA Clock Gating Control"]
    #[inline]
    pub fn cgcdma(&mut self) -> _CGCDMAW {
        _CGCDMAW { w: self }
    }
    #[doc = "Bit 3 - ERM Clock Gating Control"]
    #[inline]
    pub fn cgcerm(&mut self) -> _CGCERMW {
        _CGCERMW { w: self }
    }
    #[doc = "Bit 4 - EIM Clock Gating Control"]
    #[inline]
    pub fn cgceim(&mut self) -> _CGCEIMW {
        _CGCEIMW { w: self }
    }
}
