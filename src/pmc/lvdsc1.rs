#[doc = r" Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::LVDSC1 {
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
#[doc = "Possible values of the field `LVDRE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDRER {
    #[doc = "No system resets on low voltage detect events."]
    _0,
    #[doc = r" Reserved"]
    _Reserved(bool),
}
impl LVDRER {
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
            LVDRER::_0 => false,
            LVDRER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LVDRER {
        match value {
            false => LVDRER::_0,
            i => LVDRER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LVDRER::_0
    }
}
#[doc = "Possible values of the field `LVDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDIER {
    #[doc = "Hardware interrupt disabled (use polling)"]
    _0,
    #[doc = "Request a hardware interrupt when LVDF = 1"]
    _1,
}
impl LVDIER {
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
            LVDIER::_0 => false,
            LVDIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LVDIER {
        match value {
            false => LVDIER::_0,
            true => LVDIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LVDIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LVDIER::_1
    }
}
#[doc = "Possible values of the field `LVDF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LVDFR {
    #[doc = "Low-voltage event not detected"]
    _0,
    #[doc = "Low-voltage event detected"]
    _1,
}
impl LVDFR {
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
            LVDFR::_0 => false,
            LVDFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LVDFR {
        match value {
            false => LVDFR::_0,
            true => LVDFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LVDFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LVDFR::_1
    }
}
#[doc = "Values that can be written to the field `LVDRE`"]
pub enum LVDREW {
    #[doc = "No system resets on low voltage detect events."]
    _0,
}
impl LVDREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LVDREW::_0 => false,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LVDREW<'a> {
    w: &'a mut W,
}
impl<'a> _LVDREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LVDREW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No system resets on low voltage detect events."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVDREW::_0)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LVDIE`"]
pub enum LVDIEW {
    #[doc = "Hardware interrupt disabled (use polling)"]
    _0,
    #[doc = "Request a hardware interrupt when LVDF = 1"]
    _1,
}
impl LVDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LVDIEW::_0 => false,
            LVDIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LVDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _LVDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LVDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware interrupt disabled (use polling)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVDIEW::_0)
    }
    #[doc = "Request a hardware interrupt when LVDF = 1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVDIEW::_1)
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LVDACKW<'a> {
    w: &'a mut W,
}
impl<'a> _LVDACKW<'a> {
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
        self.w.bits &= !((MASK as u8) << OFFSET);
        self.w.bits |= ((value & MASK) as u8) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 4 - Low Voltage Detect Reset Enable"]
    #[inline]
    pub fn lvdre(&self) -> LVDRER {
        LVDRER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 5 - Low Voltage Detect Interrupt Enable"]
    #[inline]
    pub fn lvdie(&self) -> LVDIER {
        LVDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u8) != 0
        })
    }
    #[doc = "Bit 7 - Low Voltage Detect Flag"]
    #[inline]
    pub fn lvdf(&self) -> LVDFR {
        LVDFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u8) != 0
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - Low Voltage Detect Reset Enable"]
    #[inline]
    pub fn lvdre(&mut self) -> _LVDREW {
        _LVDREW { w: self }
    }
    #[doc = "Bit 5 - Low Voltage Detect Interrupt Enable"]
    #[inline]
    pub fn lvdie(&mut self) -> _LVDIEW {
        _LVDIEW { w: self }
    }
    #[doc = "Bit 6 - Low Voltage Detect Acknowledge"]
    #[inline]
    pub fn lvdack(&mut self) -> _LVDACKW {
        _LVDACKW { w: self }
    }
}
