#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SPLLCSR {
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
#[doc = "Possible values of the field `SPLLEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLLENR {
    #[doc = "System PLL is disabled"]
    _0,
    #[doc = "System PLL is enabled"]
    _1,
}
impl SPLLENR {
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
            SPLLENR::_0 => false,
            SPLLENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPLLENR {
        match value {
            false => SPLLENR::_0,
            true => SPLLENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPLLENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPLLENR::_1
    }
}
#[doc = "Possible values of the field `SPLLCM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLLCMR {
    #[doc = "System PLL Clock Monitor is disabled"]
    _0,
    #[doc = "System PLL Clock Monitor is enabled"]
    _1,
}
impl SPLLCMR {
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
            SPLLCMR::_0 => false,
            SPLLCMR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPLLCMR {
        match value {
            false => SPLLCMR::_0,
            true => SPLLCMR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPLLCMR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPLLCMR::_1
    }
}
#[doc = "Possible values of the field `SPLLCMRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLLCMRER {
    #[doc = "Clock Monitor generates interrupt when error detected"]
    _0,
    #[doc = "Clock Monitor generates reset when error detected"]
    _1,
}
impl SPLLCMRER {
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
            SPLLCMRER::_0 => false,
            SPLLCMRER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPLLCMRER {
        match value {
            false => SPLLCMRER::_0,
            true => SPLLCMRER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPLLCMRER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPLLCMRER::_1
    }
}
#[doc = "Possible values of the field `LK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LKR {
    #[doc = "Control Status Register can be written."]
    _0,
    #[doc = "Control Status Register cannot be written."]
    _1,
}
impl LKR {
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
            LKR::_0 => false,
            LKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LKR {
        match value {
            false => LKR::_0,
            true => LKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LKR::_1
    }
}
#[doc = "Possible values of the field `SPLLVLD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLLVLDR {
    #[doc = "System PLL is not enabled or clock is not valid"]
    _0,
    #[doc = "System PLL is enabled and output clock is valid"]
    _1,
}
impl SPLLVLDR {
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
            SPLLVLDR::_0 => false,
            SPLLVLDR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPLLVLDR {
        match value {
            false => SPLLVLDR::_0,
            true => SPLLVLDR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPLLVLDR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPLLVLDR::_1
    }
}
#[doc = "Possible values of the field `SPLLSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLLSELR {
    #[doc = "System PLL is not the system clock source"]
    _0,
    #[doc = "System PLL is the system clock source"]
    _1,
}
impl SPLLSELR {
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
            SPLLSELR::_0 => false,
            SPLLSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPLLSELR {
        match value {
            false => SPLLSELR::_0,
            true => SPLLSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPLLSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPLLSELR::_1
    }
}
#[doc = "Possible values of the field `SPLLERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPLLERRR {
    #[doc = "System PLL Clock Monitor is disabled or has not detected an error"]
    _0,
    #[doc = "System PLL Clock Monitor is enabled and detected an error. System PLL Clock Error flag will not set when System OSC is selected as its source and SOSCERR has set."]
    _1,
}
impl SPLLERRR {
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
            SPLLERRR::_0 => false,
            SPLLERRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SPLLERRR {
        match value {
            false => SPLLERRR::_0,
            true => SPLLERRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SPLLERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SPLLERRR::_1
    }
}
#[doc = "Values that can be written to the field `SPLLEN`"]
pub enum SPLLENW {
    #[doc = "System PLL is disabled"]
    _0,
    #[doc = "System PLL is enabled"]
    _1,
}
impl SPLLENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPLLENW::_0 => false,
            SPLLENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPLLENW<'a> {
    w: &'a mut W,
}
impl<'a> _SPLLENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPLLENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "System PLL is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLLENW::_0)
    }
    #[doc = "System PLL is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLLENW::_1)
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
#[doc = "Values that can be written to the field `SPLLCM`"]
pub enum SPLLCMW {
    #[doc = "System PLL Clock Monitor is disabled"]
    _0,
    #[doc = "System PLL Clock Monitor is enabled"]
    _1,
}
impl SPLLCMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPLLCMW::_0 => false,
            SPLLCMW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPLLCMW<'a> {
    w: &'a mut W,
}
impl<'a> _SPLLCMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPLLCMW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "System PLL Clock Monitor is disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLLCMW::_0)
    }
    #[doc = "System PLL Clock Monitor is enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLLCMW::_1)
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
#[doc = "Values that can be written to the field `SPLLCMRE`"]
pub enum SPLLCMREW {
    #[doc = "Clock Monitor generates interrupt when error detected"]
    _0,
    #[doc = "Clock Monitor generates reset when error detected"]
    _1,
}
impl SPLLCMREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPLLCMREW::_0 => false,
            SPLLCMREW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPLLCMREW<'a> {
    w: &'a mut W,
}
impl<'a> _SPLLCMREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPLLCMREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Clock Monitor generates interrupt when error detected"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLLCMREW::_0)
    }
    #[doc = "Clock Monitor generates reset when error detected"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLLCMREW::_1)
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
#[doc = "Values that can be written to the field `LK`"]
pub enum LKW {
    #[doc = "Control Status Register can be written."]
    _0,
    #[doc = "Control Status Register cannot be written."]
    _1,
}
impl LKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LKW::_0 => false,
            LKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LKW<'a> {
    w: &'a mut W,
}
impl<'a> _LKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Control Status Register can be written."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LKW::_0)
    }
    #[doc = "Control Status Register cannot be written."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LKW::_1)
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
#[doc = "Values that can be written to the field `SPLLERR`"]
pub enum SPLLERRW {
    #[doc = "System PLL Clock Monitor is disabled or has not detected an error"]
    _0,
    #[doc = "System PLL Clock Monitor is enabled and detected an error. System PLL Clock Error flag will not set when System OSC is selected as its source and SOSCERR has set."]
    _1,
}
impl SPLLERRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SPLLERRW::_0 => false,
            SPLLERRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPLLERRW<'a> {
    w: &'a mut W,
}
impl<'a> _SPLLERRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPLLERRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "System PLL Clock Monitor is disabled or has not detected an error"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPLLERRW::_0)
    }
    #[doc = "System PLL Clock Monitor is enabled and detected an error. System PLL Clock Error flag will not set when System OSC is selected as its source and SOSCERR has set."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPLLERRW::_1)
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
        const OFFSET: u8 = 26;
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
    #[doc = "Bit 0 - System PLL Enable"]
    #[inline]
    pub fn spllen(&self) -> SPLLENR {
        SPLLENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - System PLL Clock Monitor"]
    #[inline]
    pub fn spllcm(&self) -> SPLLCMR {
        SPLLCMR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - System PLL Clock Monitor Reset Enable"]
    #[inline]
    pub fn spllcmre(&self) -> SPLLCMRER {
        SPLLCMRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Lock Register"]
    #[inline]
    pub fn lk(&self) -> LKR {
        LKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - System PLL Valid"]
    #[inline]
    pub fn spllvld(&self) -> SPLLVLDR {
        SPLLVLDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - System PLL Selected"]
    #[inline]
    pub fn spllsel(&self) -> SPLLSELR {
        SPLLSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - System PLL Clock Error"]
    #[inline]
    pub fn spllerr(&self) -> SPLLERRR {
        SPLLERRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
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
    #[doc = "Bit 0 - System PLL Enable"]
    #[inline]
    pub fn spllen(&mut self) -> _SPLLENW {
        _SPLLENW { w: self }
    }
    #[doc = "Bit 16 - System PLL Clock Monitor"]
    #[inline]
    pub fn spllcm(&mut self) -> _SPLLCMW {
        _SPLLCMW { w: self }
    }
    #[doc = "Bit 17 - System PLL Clock Monitor Reset Enable"]
    #[inline]
    pub fn spllcmre(&mut self) -> _SPLLCMREW {
        _SPLLCMREW { w: self }
    }
    #[doc = "Bit 23 - Lock Register"]
    #[inline]
    pub fn lk(&mut self) -> _LKW {
        _LKW { w: self }
    }
    #[doc = "Bit 26 - System PLL Clock Error"]
    #[inline]
    pub fn spllerr(&mut self) -> _SPLLERRW {
        _SPLLERRW { w: self }
    }
}
