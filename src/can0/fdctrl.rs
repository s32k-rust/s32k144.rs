#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FDCTRL {
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
#[doc = r" Value of the field"]
pub struct TDCVALR {
    bits: u8,
}
impl TDCVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TDCOFFR {
    bits: u8,
}
impl TDCOFFR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TDCFAIL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDCFAILR {
    #[doc = "Measured loop delay is in range."]
    _0,
    #[doc = "Measured loop delay is out of range."]
    _1,
}
impl TDCFAILR {
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
            TDCFAILR::_0 => false,
            TDCFAILR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDCFAILR {
        match value {
            false => TDCFAILR::_0,
            true => TDCFAILR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TDCFAILR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TDCFAILR::_1
    }
}
#[doc = "Possible values of the field `TDCEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDCENR {
    #[doc = "TDC is disabled"]
    _0,
    #[doc = "TDC is enabled"]
    _1,
}
impl TDCENR {
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
            TDCENR::_0 => false,
            TDCENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDCENR {
        match value {
            false => TDCENR::_0,
            true => TDCENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TDCENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TDCENR::_1
    }
}
#[doc = "Possible values of the field `MBDSR0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBDSR0R {
    #[doc = "Selects 8 bytes per Message Buffer."]
    _00,
    #[doc = "Selects 16 bytes per Message Buffer."]
    _01,
    #[doc = "Selects 32 bytes per Message Buffer."]
    _10,
    #[doc = "Selects 64 bytes per Message Buffer."]
    _11,
}
impl MBDSR0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MBDSR0R::_00 => 0,
            MBDSR0R::_01 => 1,
            MBDSR0R::_10 => 2,
            MBDSR0R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MBDSR0R {
        match value {
            0 => MBDSR0R::_00,
            1 => MBDSR0R::_01,
            2 => MBDSR0R::_10,
            3 => MBDSR0R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == MBDSR0R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == MBDSR0R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == MBDSR0R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == MBDSR0R::_11
    }
}
#[doc = "Possible values of the field `FDRATE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FDRATER {
    #[doc = "Transmit a frame in nominal rate. The BRS bit in the Tx MB has no effect."]
    _0,
    #[doc = "Transmit a frame with bit rate switching if the BRS bit in the Tx MB is recessive."]
    _1,
}
impl FDRATER {
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
            FDRATER::_0 => false,
            FDRATER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FDRATER {
        match value {
            false => FDRATER::_0,
            true => FDRATER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FDRATER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FDRATER::_1
    }
}
#[doc = r" Proxy"]
pub struct _TDCOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _TDCOFFW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TDCFAIL`"]
pub enum TDCFAILW {
    #[doc = "Measured loop delay is in range."]
    _0,
    #[doc = "Measured loop delay is out of range."]
    _1,
}
impl TDCFAILW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDCFAILW::_0 => false,
            TDCFAILW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDCFAILW<'a> {
    w: &'a mut W,
}
impl<'a> _TDCFAILW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDCFAILW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Measured loop delay is in range."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDCFAILW::_0)
    }
    #[doc = "Measured loop delay is out of range."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDCFAILW::_1)
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
#[doc = "Values that can be written to the field `TDCEN`"]
pub enum TDCENW {
    #[doc = "TDC is disabled"]
    _0,
    #[doc = "TDC is enabled"]
    _1,
}
impl TDCENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDCENW::_0 => false,
            TDCENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDCENW<'a> {
    w: &'a mut W,
}
impl<'a> _TDCENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDCENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "TDC is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDCENW::_0)
    }
    #[doc = "TDC is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDCENW::_1)
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
#[doc = "Values that can be written to the field `MBDSR0`"]
pub enum MBDSR0W {
    #[doc = "Selects 8 bytes per Message Buffer."]
    _00,
    #[doc = "Selects 16 bytes per Message Buffer."]
    _01,
    #[doc = "Selects 32 bytes per Message Buffer."]
    _10,
    #[doc = "Selects 64 bytes per Message Buffer."]
    _11,
}
impl MBDSR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MBDSR0W::_00 => 0,
            MBDSR0W::_01 => 1,
            MBDSR0W::_10 => 2,
            MBDSR0W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MBDSR0W<'a> {
    w: &'a mut W,
}
impl<'a> _MBDSR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MBDSR0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Selects 8 bytes per Message Buffer."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(MBDSR0W::_00)
    }
    #[doc = "Selects 16 bytes per Message Buffer."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(MBDSR0W::_01)
    }
    #[doc = "Selects 32 bytes per Message Buffer."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(MBDSR0W::_10)
    }
    #[doc = "Selects 64 bytes per Message Buffer."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(MBDSR0W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FDRATE`"]
pub enum FDRATEW {
    #[doc = "Transmit a frame in nominal rate. The BRS bit in the Tx MB has no effect."]
    _0,
    #[doc = "Transmit a frame with bit rate switching if the BRS bit in the Tx MB is recessive."]
    _1,
}
impl FDRATEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FDRATEW::_0 => false,
            FDRATEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FDRATEW<'a> {
    w: &'a mut W,
}
impl<'a> _FDRATEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FDRATEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit a frame in nominal rate. The BRS bit in the Tx MB has no effect."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDRATEW::_0)
    }
    #[doc = "Transmit a frame with bit rate switching if the BRS bit in the Tx MB is recessive."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDRATEW::_1)
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
    #[doc = "Bits 0:5 - Transceiver Delay Compensation Value"]
    #[inline]
    pub fn tdcval(&self) -> TDCVALR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TDCVALR { bits }
    }
    #[doc = "Bits 8:12 - Transceiver Delay Compensation Offset"]
    #[inline]
    pub fn tdcoff(&self) -> TDCOFFR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TDCOFFR { bits }
    }
    #[doc = "Bit 14 - Transceiver Delay Compensation Fail"]
    #[inline]
    pub fn tdcfail(&self) -> TDCFAILR {
        TDCFAILR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - Transceiver Delay Compensation Enable"]
    #[inline]
    pub fn tdcen(&self) -> TDCENR {
        TDCENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:17 - Message Buffer Data Size for Region 0"]
    #[inline]
    pub fn mbdsr0(&self) -> MBDSR0R {
        MBDSR0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 31 - Bit Rate Switch Enable"]
    #[inline]
    pub fn fdrate(&self) -> FDRATER {
        FDRATER::_from({
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
        W { bits: 2147483904 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 8:12 - Transceiver Delay Compensation Offset"]
    #[inline]
    pub fn tdcoff(&mut self) -> _TDCOFFW {
        _TDCOFFW { w: self }
    }
    #[doc = "Bit 14 - Transceiver Delay Compensation Fail"]
    #[inline]
    pub fn tdcfail(&mut self) -> _TDCFAILW {
        _TDCFAILW { w: self }
    }
    #[doc = "Bit 15 - Transceiver Delay Compensation Enable"]
    #[inline]
    pub fn tdcen(&mut self) -> _TDCENW {
        _TDCENW { w: self }
    }
    #[doc = "Bits 16:17 - Message Buffer Data Size for Region 0"]
    #[inline]
    pub fn mbdsr0(&mut self) -> _MBDSR0W {
        _MBDSR0W { w: self }
    }
    #[doc = "Bit 31 - Bit Rate Switch Enable"]
    #[inline]
    pub fn fdrate(&mut self) -> _FDRATEW {
        _FDRATEW { w: self }
    }
}
