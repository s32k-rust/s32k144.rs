#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEADTIME {
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
pub struct DTVALR {
    bits: u8,
}
impl DTVALR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `DTPS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTPSR {
    #[doc = "Divide the FTM input clock by 1."]
    _0X,
    #[doc = "Divide the FTM input clock by 4."]
    _10,
    #[doc = "Divide the FTM input clock by 16."]
    _11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DTPSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DTPSR::_0X => 0,
            DTPSR::_10 => 2,
            DTPSR::_11 => 3,
            DTPSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DTPSR {
        match value {
            0 => DTPSR::_0X,
            2 => DTPSR::_10,
            3 => DTPSR::_11,
            i => DTPSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0X`"]
    #[inline]
    pub fn is_0x(&self) -> bool {
        *self == DTPSR::_0X
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == DTPSR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == DTPSR::_11
    }
}
#[doc = r" Value of the field"]
pub struct DTVALEXR {
    bits: u8,
}
impl DTVALEXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DTVALW<'a> {
    w: &'a mut W,
}
impl<'a> _DTVALW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DTPS`"]
pub enum DTPSW {
    #[doc = "Divide the FTM input clock by 1."]
    _0X,
    #[doc = "Divide the FTM input clock by 4."]
    _10,
    #[doc = "Divide the FTM input clock by 16."]
    _11,
}
impl DTPSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DTPSW::_0X => 0,
            DTPSW::_10 => 2,
            DTPSW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DTPSW<'a> {
    w: &'a mut W,
}
impl<'a> _DTPSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DTPSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divide the FTM input clock by 1."]
    #[inline]
    pub fn _0x(self) -> &'a mut W {
        self.variant(DTPSW::_0X)
    }
    #[doc = "Divide the FTM input clock by 4."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(DTPSW::_10)
    }
    #[doc = "Divide the FTM input clock by 16."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(DTPSW::_11)
    }
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
pub struct _DTVALEXW<'a> {
    w: &'a mut W,
}
impl<'a> _DTVALEXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:5 - Deadtime Value"]
    #[inline]
    pub fn dtval(&self) -> DTVALR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DTVALR { bits }
    }
    #[doc = "Bits 6:7 - Deadtime Prescaler Value"]
    #[inline]
    pub fn dtps(&self) -> DTPSR {
        DTPSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Extended Deadtime Value"]
    #[inline]
    pub fn dtvalex(&self) -> DTVALEXR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DTVALEXR { bits }
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
    #[doc = "Bits 0:5 - Deadtime Value"]
    #[inline]
    pub fn dtval(&mut self) -> _DTVALW {
        _DTVALW { w: self }
    }
    #[doc = "Bits 6:7 - Deadtime Prescaler Value"]
    #[inline]
    pub fn dtps(&mut self) -> _DTPSW {
        _DTPSW { w: self }
    }
    #[doc = "Bits 16:19 - Extended Deadtime Value"]
    #[inline]
    pub fn dtvalex(&mut self) -> _DTVALEXW {
        _DTVALEXW { w: self }
    }
}
