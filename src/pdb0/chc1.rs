#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CHC1 {
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
#[doc = "Possible values of the field `EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENR {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl ENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ENR::_0 => 0,
            ENR::_1 => 1,
            ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ENR {
        match value {
            0 => ENR::_0,
            1 => ENR::_1,
            i => ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ENR::_1
    }
}
#[doc = "Possible values of the field `TOS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TOSR {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TOSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TOSR::_0 => 0,
            TOSR::_1 => 1,
            TOSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TOSR {
        match value {
            0 => TOSR::_0,
            1 => TOSR::_1,
            i => TOSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TOSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TOSR::_1
    }
}
#[doc = "Possible values of the field `BB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BBR {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BBR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BBR::_0 => 0,
            BBR::_1 => 1,
            BBR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BBR {
        match value {
            0 => BBR::_0,
            1 => BBR::_1,
            i => BBR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BBR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BBR::_1
    }
}
#[doc = "Values that can be written to the field `EN`"]
pub enum ENW {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    _1,
}
impl ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ENW::_0 => 0,
            ENW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ENW<'a> {
    w: &'a mut W,
}
impl<'a> _ENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENW::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TOS`"]
pub enum TOSW {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1,
}
impl TOSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TOSW::_0 => 0,
            TOSW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TOSW<'a> {
    w: &'a mut W,
}
impl<'a> _TOSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TOSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOSW::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOSW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BB`"]
pub enum BBW {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0,
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1,
}
impl BBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BBW::_0 => 0,
            BBW::_1 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BBW<'a> {
    w: &'a mut W,
}
impl<'a> _BBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BBW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BBW::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BBW::_1)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bits 0:7 - PDB Channel Pre-Trigger Enable"]
    #[inline]
    pub fn en(&self) -> ENR {
        ENR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:15 - PDB Channel Pre-Trigger Output Select"]
    #[inline]
    pub fn tos(&self) -> TOSR {
        TOSR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:23 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline]
    pub fn bb(&self) -> BBR {
        BBR::_from({
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:7 - PDB Channel Pre-Trigger Enable"]
    #[inline]
    pub fn en(&mut self) -> _ENW {
        _ENW { w: self }
    }
    #[doc = "Bits 8:15 - PDB Channel Pre-Trigger Output Select"]
    #[inline]
    pub fn tos(&mut self) -> _TOSW {
        _TOSW { w: self }
    }
    #[doc = "Bits 16:23 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline]
    pub fn bb(&mut self) -> _BBW {
        _BBW { w: self }
    }
}
