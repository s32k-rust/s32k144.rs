#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CONF {
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
pub struct LDFQR {
    bits: u8,
}
impl LDFQR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BDMMODER {
    bits: u8,
}
impl BDMMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GTBEENR {
    bits: bool,
}
impl GTBEENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct GTBEOUTR {
    bits: bool,
}
impl GTBEOUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `ITRIGR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITRIGRR {
    #[doc = "Initialization trigger is generated on counter wrap events."]
    _0,
    #[doc = "Initialization trigger is generated when a reload point is reached."]
    _1,
}
impl ITRIGRR {
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
            ITRIGRR::_0 => false,
            ITRIGRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ITRIGRR {
        match value {
            false => ITRIGRR::_0,
            true => ITRIGRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ITRIGRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ITRIGRR::_1
    }
}
#[doc = r" Proxy"]
pub struct _LDFQW<'a> {
    w: &'a mut W,
}
impl<'a> _LDFQW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BDMMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _BDMMODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GTBEENW<'a> {
    w: &'a mut W,
}
impl<'a> _GTBEENW<'a> {
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
#[doc = r" Proxy"]
pub struct _GTBEOUTW<'a> {
    w: &'a mut W,
}
impl<'a> _GTBEOUTW<'a> {
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
#[doc = "Values that can be written to the field `ITRIGR`"]
pub enum ITRIGRW {
    #[doc = "Initialization trigger is generated on counter wrap events."]
    _0,
    #[doc = "Initialization trigger is generated when a reload point is reached."]
    _1,
}
impl ITRIGRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ITRIGRW::_0 => false,
            ITRIGRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ITRIGRW<'a> {
    w: &'a mut W,
}
impl<'a> _ITRIGRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ITRIGRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Initialization trigger is generated on counter wrap events."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ITRIGRW::_0)
    }
    #[doc = "Initialization trigger is generated when a reload point is reached."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ITRIGRW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - Frequency of the Reload Opportunities"]
    #[inline]
    pub fn ldfq(&self) -> LDFQR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        LDFQR { bits }
    }
    #[doc = "Bits 6:7 - Debug Mode"]
    #[inline]
    pub fn bdmmode(&self) -> BDMMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BDMMODER { bits }
    }
    #[doc = "Bit 9 - Global Time Base Enable"]
    #[inline]
    pub fn gtbeen(&self) -> GTBEENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GTBEENR { bits }
    }
    #[doc = "Bit 10 - Global Time Base Output"]
    #[inline]
    pub fn gtbeout(&self) -> GTBEOUTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GTBEOUTR { bits }
    }
    #[doc = "Bit 11 - Initialization trigger on Reload Point"]
    #[inline]
    pub fn itrigr(&self) -> ITRIGRR {
        ITRIGRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
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
    #[doc = "Bits 0:4 - Frequency of the Reload Opportunities"]
    #[inline]
    pub fn ldfq(&mut self) -> _LDFQW {
        _LDFQW { w: self }
    }
    #[doc = "Bits 6:7 - Debug Mode"]
    #[inline]
    pub fn bdmmode(&mut self) -> _BDMMODEW {
        _BDMMODEW { w: self }
    }
    #[doc = "Bit 9 - Global Time Base Enable"]
    #[inline]
    pub fn gtbeen(&mut self) -> _GTBEENW {
        _GTBEENW { w: self }
    }
    #[doc = "Bit 10 - Global Time Base Output"]
    #[inline]
    pub fn gtbeout(&mut self) -> _GTBEOUTW {
        _GTBEOUTW { w: self }
    }
    #[doc = "Bit 11 - Initialization trigger on Reload Point"]
    #[inline]
    pub fn itrigr(&mut self) -> _ITRIGRW {
        _ITRIGRW { w: self }
    }
}
