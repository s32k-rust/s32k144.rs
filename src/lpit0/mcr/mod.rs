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
#[doc = "Possible values of the field `M_CEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M_CENR {
    #[doc = "Peripheral clock to timers is disabled"]
    _0,
    #[doc = "Peripheral clock to timers is enabled"]
    _1,
}
impl M_CENR {
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
            M_CENR::_0 => false,
            M_CENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M_CENR {
        match value {
            false => M_CENR::_0,
            true => M_CENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M_CENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M_CENR::_1
    }
}
#[doc = "Possible values of the field `SW_RST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SW_RSTR {
    #[doc = "Timer channels and registers are not reset"]
    _0,
    #[doc = "Timer channels and registers are reset"]
    _1,
}
impl SW_RSTR {
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
            SW_RSTR::_0 => false,
            SW_RSTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SW_RSTR {
        match value {
            false => SW_RSTR::_0,
            true => SW_RSTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SW_RSTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SW_RSTR::_1
    }
}
#[doc = "Possible values of the field `DOZE_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZE_ENR {
    #[doc = "Timer channels are stopped in DOZE mode"]
    _0,
    #[doc = "Timer channels continue to run in DOZE mode"]
    _1,
}
impl DOZE_ENR {
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
            DOZE_ENR::_0 => false,
            DOZE_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DOZE_ENR {
        match value {
            false => DOZE_ENR::_0,
            true => DOZE_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DOZE_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DOZE_ENR::_1
    }
}
#[doc = "Possible values of the field `DBG_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBG_ENR {
    #[doc = "Timer channels are stopped in Debug mode"]
    _0,
    #[doc = "Timer channels continue to run in Debug mode"]
    _1,
}
impl DBG_ENR {
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
            DBG_ENR::_0 => false,
            DBG_ENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DBG_ENR {
        match value {
            false => DBG_ENR::_0,
            true => DBG_ENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DBG_ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DBG_ENR::_1
    }
}
#[doc = "Values that can be written to the field `M_CEN`"]
pub enum M_CENW {
    #[doc = "Peripheral clock to timers is disabled"]
    _0,
    #[doc = "Peripheral clock to timers is enabled"]
    _1,
}
impl M_CENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M_CENW::_0 => false,
            M_CENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M_CENW<'a> {
    w: &'a mut W,
}
impl<'a> _M_CENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M_CENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Peripheral clock to timers is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M_CENW::_0)
    }
    #[doc = "Peripheral clock to timers is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M_CENW::_1)
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
#[doc = "Values that can be written to the field `SW_RST`"]
pub enum SW_RSTW {
    #[doc = "Timer channels and registers are not reset"]
    _0,
    #[doc = "Timer channels and registers are reset"]
    _1,
}
impl SW_RSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SW_RSTW::_0 => false,
            SW_RSTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SW_RSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SW_RSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SW_RSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer channels and registers are not reset"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SW_RSTW::_0)
    }
    #[doc = "Timer channels and registers are reset"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SW_RSTW::_1)
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
#[doc = "Values that can be written to the field `DOZE_EN`"]
pub enum DOZE_ENW {
    #[doc = "Timer channels are stopped in DOZE mode"]
    _0,
    #[doc = "Timer channels continue to run in DOZE mode"]
    _1,
}
impl DOZE_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DOZE_ENW::_0 => false,
            DOZE_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DOZE_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DOZE_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DOZE_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer channels are stopped in DOZE mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOZE_ENW::_0)
    }
    #[doc = "Timer channels continue to run in DOZE mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOZE_ENW::_1)
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
#[doc = "Values that can be written to the field `DBG_EN`"]
pub enum DBG_ENW {
    #[doc = "Timer channels are stopped in Debug mode"]
    _0,
    #[doc = "Timer channels continue to run in Debug mode"]
    _1,
}
impl DBG_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DBG_ENW::_0 => false,
            DBG_ENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DBG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _DBG_ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DBG_ENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Timer channels are stopped in Debug mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBG_ENW::_0)
    }
    #[doc = "Timer channels continue to run in Debug mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBG_ENW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Module Clock Enable"]
    #[inline]
    pub fn m_cen(&self) -> M_CENR {
        M_CENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Software Reset Bit"]
    #[inline]
    pub fn sw_rst(&self) -> SW_RSTR {
        SW_RSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - DOZE Mode Enable Bit"]
    #[inline]
    pub fn doze_en(&self) -> DOZE_ENR {
        DOZE_ENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Debug Enable Bit"]
    #[inline]
    pub fn dbg_en(&self) -> DBG_ENR {
        DBG_ENR::_from({
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
    #[doc = "Bit 0 - Module Clock Enable"]
    #[inline]
    pub fn m_cen(&mut self) -> _M_CENW {
        _M_CENW { w: self }
    }
    #[doc = "Bit 1 - Software Reset Bit"]
    #[inline]
    pub fn sw_rst(&mut self) -> _SW_RSTW {
        _SW_RSTW { w: self }
    }
    #[doc = "Bit 2 - DOZE Mode Enable Bit"]
    #[inline]
    pub fn doze_en(&mut self) -> _DOZE_ENW {
        _DOZE_ENW { w: self }
    }
    #[doc = "Bit 3 - Debug Enable Bit"]
    #[inline]
    pub fn dbg_en(&mut self) -> _DBG_ENW {
        _DBG_ENW { w: self }
    }
}
