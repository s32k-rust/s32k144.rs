#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SC2 {
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
#[doc = "Possible values of the field `REFSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSELR {
    #[doc = "Default voltage reference pin pair, that is, external pins VREFH and VREFL"]
    _00,
    #[doc = "Alternate reference voltage, that is, VALTH. This voltage may be additional external pin or internal source depending on the MCU configuration. See the chip configuration information for details specific to this MCU."]
    _01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl REFSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            REFSELR::_00 => 0,
            REFSELR::_01 => 1,
            REFSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> REFSELR {
        match value {
            0 => REFSELR::_00,
            1 => REFSELR::_01,
            i => REFSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == REFSELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == REFSELR::_01
    }
}
#[doc = "Possible values of the field `DMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENR {
    #[doc = "DMA is disabled."]
    _0,
    #[doc = "DMA is enabled and will assert the ADC DMA request during an ADC conversion complete event , which is indicated when any SC1n[COCO] flag is asserted."]
    _1,
}
impl DMAENR {
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
            DMAENR::_0 => false,
            DMAENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAENR {
        match value {
            false => DMAENR::_0,
            true => DMAENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMAENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMAENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct ACRENR {
    bits: bool,
}
impl ACRENR {
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
pub struct ACFGTR {
    bits: bool,
}
impl ACFGTR {
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
#[doc = "Possible values of the field `ACFE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACFER {
    #[doc = "Compare function disabled."]
    _0,
    #[doc = "Compare function enabled."]
    _1,
}
impl ACFER {
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
            ACFER::_0 => false,
            ACFER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ACFER {
        match value {
            false => ACFER::_0,
            true => ACFER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ACFER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ACFER::_1
    }
}
#[doc = "Possible values of the field `ADTRG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADTRGR {
    #[doc = "Software trigger selected."]
    _0,
    #[doc = "Hardware trigger selected."]
    _1,
}
impl ADTRGR {
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
            ADTRGR::_0 => false,
            ADTRGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADTRGR {
        match value {
            false => ADTRGR::_0,
            true => ADTRGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADTRGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADTRGR::_1
    }
}
#[doc = "Possible values of the field `ADACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADACTR {
    #[doc = "Conversion not in progress."]
    _0,
    #[doc = "Conversion in progress."]
    _1,
}
impl ADACTR {
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
            ADACTR::_0 => false,
            ADACTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ADACTR {
        match value {
            false => ADACTR::_0,
            true => ADACTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ADACTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ADACTR::_1
    }
}
#[doc = r" Value of the field"]
pub struct TRGPRNUMR {
    bits: u8,
}
impl TRGPRNUMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `TRGSTLAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSTLATR {
    #[doc = "No trigger request has been latched"]
    _0,
    #[doc = "A trigger request has been latched"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRGSTLATR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRGSTLATR::_0 => 0,
            TRGSTLATR::_1 => 1,
            TRGSTLATR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRGSTLATR {
        match value {
            0 => TRGSTLATR::_0,
            1 => TRGSTLATR::_1,
            i => TRGSTLATR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRGSTLATR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRGSTLATR::_1
    }
}
#[doc = "Possible values of the field `TRGSTERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSTERRR {
    #[doc = "No error has occurred"]
    _0,
    #[doc = "An error has occurred"]
    _1,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl TRGSTERRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRGSTERRR::_0 => 0,
            TRGSTERRR::_1 => 1,
            TRGSTERRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRGSTERRR {
        match value {
            0 => TRGSTERRR::_0,
            1 => TRGSTERRR::_1,
            i => TRGSTERRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRGSTERRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRGSTERRR::_1
    }
}
#[doc = "Values that can be written to the field `REFSEL`"]
pub enum REFSELW {
    #[doc = "Default voltage reference pin pair, that is, external pins VREFH and VREFL"]
    _00,
    #[doc = "Alternate reference voltage, that is, VALTH. This voltage may be additional external pin or internal source depending on the MCU configuration. See the chip configuration information for details specific to this MCU."]
    _01,
}
impl REFSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            REFSELW::_00 => 0,
            REFSELW::_01 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFSELW<'a> {
    w: &'a mut W,
}
impl<'a> _REFSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Default voltage reference pin pair, that is, external pins VREFH and VREFL"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(REFSELW::_00)
    }
    #[doc = "Alternate reference voltage, that is, VALTH. This voltage may be additional external pin or internal source depending on the MCU configuration. See the chip configuration information for details specific to this MCU."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(REFSELW::_01)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAEN`"]
pub enum DMAENW {
    #[doc = "DMA is disabled."]
    _0,
    #[doc = "DMA is enabled and will assert the ADC DMA request during an ADC conversion complete event , which is indicated when any SC1n[COCO] flag is asserted."]
    _1,
}
impl DMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAENW::_0 => false,
            DMAENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAENW::_0)
    }
    #[doc = "DMA is enabled and will assert the ADC DMA request during an ADC conversion complete event , which is indicated when any SC1n[COCO] flag is asserted."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAENW::_1)
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
#[doc = r" Proxy"]
pub struct _ACRENW<'a> {
    w: &'a mut W,
}
impl<'a> _ACRENW<'a> {
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
#[doc = r" Proxy"]
pub struct _ACFGTW<'a> {
    w: &'a mut W,
}
impl<'a> _ACFGTW<'a> {
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
#[doc = "Values that can be written to the field `ACFE`"]
pub enum ACFEW {
    #[doc = "Compare function disabled."]
    _0,
    #[doc = "Compare function enabled."]
    _1,
}
impl ACFEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ACFEW::_0 => false,
            ACFEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ACFEW<'a> {
    w: &'a mut W,
}
impl<'a> _ACFEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ACFEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Compare function disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACFEW::_0)
    }
    #[doc = "Compare function enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACFEW::_1)
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
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADTRG`"]
pub enum ADTRGW {
    #[doc = "Software trigger selected."]
    _0,
    #[doc = "Hardware trigger selected."]
    _1,
}
impl ADTRGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ADTRGW::_0 => false,
            ADTRGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADTRGW<'a> {
    w: &'a mut W,
}
impl<'a> _ADTRGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADTRGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Software trigger selected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTRGW::_0)
    }
    #[doc = "Hardware trigger selected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTRGW::_1)
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
        const OFFSET: u8 = 6;
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
    #[doc = "Bits 0:1 - Voltage Reference Selection"]
    #[inline]
    pub fn refsel(&self) -> REFSELR {
        REFSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - DMA Enable"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        DMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 3 - Compare Function Range Enable"]
    #[inline]
    pub fn acren(&self) -> ACRENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACRENR { bits }
    }
    #[doc = "Bit 4 - Compare Function Greater Than Enable"]
    #[inline]
    pub fn acfgt(&self) -> ACFGTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ACFGTR { bits }
    }
    #[doc = "Bit 5 - Compare Function Enable"]
    #[inline]
    pub fn acfe(&self) -> ACFER {
        ACFER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Conversion Trigger Select"]
    #[inline]
    pub fn adtrg(&self) -> ADTRGR {
        ADTRGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Conversion Active"]
    #[inline]
    pub fn adact(&self) -> ADACTR {
        ADACTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 13:14 - Trigger Process Number"]
    #[inline]
    pub fn trgprnum(&self) -> TRGPRNUMR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRGPRNUMR { bits }
    }
    #[doc = "Bits 16:19 - Trigger Status"]
    #[inline]
    pub fn trgstlat(&self) -> TRGSTLATR {
        TRGSTLATR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Error in Multiplexed Trigger Request"]
    #[inline]
    pub fn trgsterr(&self) -> TRGSTERRR {
        TRGSTERRR::_from({
            const MASK: u8 = 15;
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
    #[doc = "Bits 0:1 - Voltage Reference Selection"]
    #[inline]
    pub fn refsel(&mut self) -> _REFSELW {
        _REFSELW { w: self }
    }
    #[doc = "Bit 2 - DMA Enable"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
    #[doc = "Bit 3 - Compare Function Range Enable"]
    #[inline]
    pub fn acren(&mut self) -> _ACRENW {
        _ACRENW { w: self }
    }
    #[doc = "Bit 4 - Compare Function Greater Than Enable"]
    #[inline]
    pub fn acfgt(&mut self) -> _ACFGTW {
        _ACFGTW { w: self }
    }
    #[doc = "Bit 5 - Compare Function Enable"]
    #[inline]
    pub fn acfe(&mut self) -> _ACFEW {
        _ACFEW { w: self }
    }
    #[doc = "Bit 6 - Conversion Trigger Select"]
    #[inline]
    pub fn adtrg(&mut self) -> _ADTRGW {
        _ADTRGW { w: self }
    }
}
