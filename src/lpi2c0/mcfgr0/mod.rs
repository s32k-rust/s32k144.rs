#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCFGR0 {
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
#[doc = "Possible values of the field `HREN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRENR {
    #[doc = "Host request input is disabled."]
    _0,
    #[doc = "Host request input is enabled."]
    _1,
}
impl HRENR {
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
            HRENR::_0 => false,
            HRENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRENR {
        match value {
            false => HRENR::_0,
            true => HRENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRENR::_1
    }
}
#[doc = "Possible values of the field `HRPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRPOLR {
    #[doc = "Active low."]
    _0,
    #[doc = "Active high."]
    _1,
}
impl HRPOLR {
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
            HRPOLR::_0 => false,
            HRPOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRPOLR {
        match value {
            false => HRPOLR::_0,
            true => HRPOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRPOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRPOLR::_1
    }
}
#[doc = "Possible values of the field `HRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRSELR {
    #[doc = "Host request input is pin HREQ."]
    _0,
    #[doc = "Host request input is input trigger."]
    _1,
}
impl HRSELR {
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
            HRSELR::_0 => false,
            HRSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> HRSELR {
        match value {
            false => HRSELR::_0,
            true => HRSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == HRSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == HRSELR::_1
    }
}
#[doc = "Possible values of the field `CIRFIFO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CIRFIFOR {
    #[doc = "Circular FIFO is disabled."]
    _0,
    #[doc = "Circular FIFO is enabled."]
    _1,
}
impl CIRFIFOR {
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
            CIRFIFOR::_0 => false,
            CIRFIFOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CIRFIFOR {
        match value {
            false => CIRFIFOR::_0,
            true => CIRFIFOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CIRFIFOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CIRFIFOR::_1
    }
}
#[doc = "Possible values of the field `RDMO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDMOR {
    #[doc = "Received data is stored in the receive FIFO as normal."]
    _0,
    #[doc = "Received data is discarded unless the RMF is set."]
    _1,
}
impl RDMOR {
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
            RDMOR::_0 => false,
            RDMOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDMOR {
        match value {
            false => RDMOR::_0,
            true => RDMOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RDMOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RDMOR::_1
    }
}
#[doc = "Values that can be written to the field `HREN`"]
pub enum HRENW {
    #[doc = "Host request input is disabled."]
    _0,
    #[doc = "Host request input is enabled."]
    _1,
}
impl HRENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRENW::_0 => false,
            HRENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRENW<'a> {
    w: &'a mut W,
}
impl<'a> _HRENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Host request input is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRENW::_0)
    }
    #[doc = "Host request input is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRENW::_1)
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
#[doc = "Values that can be written to the field `HRPOL`"]
pub enum HRPOLW {
    #[doc = "Active low."]
    _0,
    #[doc = "Active high."]
    _1,
}
impl HRPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRPOLW::_0 => false,
            HRPOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _HRPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Active low."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRPOLW::_0)
    }
    #[doc = "Active high."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRPOLW::_1)
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
#[doc = "Values that can be written to the field `HRSEL`"]
pub enum HRSELW {
    #[doc = "Host request input is pin HREQ."]
    _0,
    #[doc = "Host request input is input trigger."]
    _1,
}
impl HRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            HRSELW::_0 => false,
            HRSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _HRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _HRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: HRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Host request input is pin HREQ."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(HRSELW::_0)
    }
    #[doc = "Host request input is input trigger."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(HRSELW::_1)
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
#[doc = "Values that can be written to the field `CIRFIFO`"]
pub enum CIRFIFOW {
    #[doc = "Circular FIFO is disabled."]
    _0,
    #[doc = "Circular FIFO is enabled."]
    _1,
}
impl CIRFIFOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CIRFIFOW::_0 => false,
            CIRFIFOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CIRFIFOW<'a> {
    w: &'a mut W,
}
impl<'a> _CIRFIFOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CIRFIFOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Circular FIFO is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CIRFIFOW::_0)
    }
    #[doc = "Circular FIFO is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CIRFIFOW::_1)
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
#[doc = "Values that can be written to the field `RDMO`"]
pub enum RDMOW {
    #[doc = "Received data is stored in the receive FIFO as normal."]
    _0,
    #[doc = "Received data is discarded unless the RMF is set."]
    _1,
}
impl RDMOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDMOW::_0 => false,
            RDMOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDMOW<'a> {
    w: &'a mut W,
}
impl<'a> _RDMOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDMOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Received data is stored in the receive FIFO as normal."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDMOW::_0)
    }
    #[doc = "Received data is discarded unless the RMF is set."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDMOW::_1)
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
    #[doc = "Bit 0 - Host Request Enable"]
    #[inline]
    pub fn hren(&self) -> HRENR {
        HRENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Host Request Polarity"]
    #[inline]
    pub fn hrpol(&self) -> HRPOLR {
        HRPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Host Request Select"]
    #[inline]
    pub fn hrsel(&self) -> HRSELR {
        HRSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Circular FIFO Enable"]
    #[inline]
    pub fn cirfifo(&self) -> CIRFIFOR {
        CIRFIFOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Receive Data Match Only"]
    #[inline]
    pub fn rdmo(&self) -> RDMOR {
        RDMOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
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
    #[doc = "Bit 0 - Host Request Enable"]
    #[inline]
    pub fn hren(&mut self) -> _HRENW {
        _HRENW { w: self }
    }
    #[doc = "Bit 1 - Host Request Polarity"]
    #[inline]
    pub fn hrpol(&mut self) -> _HRPOLW {
        _HRPOLW { w: self }
    }
    #[doc = "Bit 2 - Host Request Select"]
    #[inline]
    pub fn hrsel(&mut self) -> _HRSELW {
        _HRSELW { w: self }
    }
    #[doc = "Bit 8 - Circular FIFO Enable"]
    #[inline]
    pub fn cirfifo(&mut self) -> _CIRFIFOW {
        _CIRFIFOW { w: self }
    }
    #[doc = "Bit 9 - Receive Data Match Only"]
    #[inline]
    pub fn rdmo(&mut self) -> _RDMOW {
        _RDMOW { w: self }
    }
}
