#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::C0SC {
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
#[doc = "Possible values of the field `DMA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAR {
    #[doc = "Disable DMA transfers."] _0,
    #[doc = "Enable DMA transfers."] _1,
}
impl DMAR {
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
            DMAR::_0 => false,
            DMAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAR {
        match value {
            false => DMAR::_0,
            true => DMAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMAR::_1
    }
}
#[doc = "Possible values of the field `ICRST`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICRSTR {
    #[doc = "FTM counter is not reset when the selected channel (n) input event is detected."] _0,
    #[doc = "FTM counter is reset when the selected channel (n) input event is detected."] _1,
}
impl ICRSTR {
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
            ICRSTR::_0 => false,
            ICRSTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ICRSTR {
        match value {
            false => ICRSTR::_0,
            true => ICRSTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ICRSTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ICRSTR::_1
    }
}
#[doc = r" Value of the field"]
pub struct ELSAR {
    bits: bool,
}
impl ELSAR {
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
pub struct ELSBR {
    bits: bool,
}
impl ELSBR {
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
pub struct MSAR {
    bits: bool,
}
impl MSAR {
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
pub struct MSBR {
    bits: bool,
}
impl MSBR {
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
#[doc = "Possible values of the field `CHIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHIER {
    #[doc = "Disable channel (n) interrupt. Use software polling."] _0,
    #[doc = "Enable channel (n) interrupt."] _1,
}
impl CHIER {
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
            CHIER::_0 => false,
            CHIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHIER {
        match value {
            false => CHIER::_0,
            true => CHIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CHIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CHIER::_1
    }
}
#[doc = "Possible values of the field `CHF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHFR {
    #[doc = "No channel (n) event has occurred."] _0,
    #[doc = "A channel (n) event has occurred."] _1,
}
impl CHFR {
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
            CHFR::_0 => false,
            CHFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHFR {
        match value {
            false => CHFR::_0,
            true => CHFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CHFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CHFR::_1
    }
}
#[doc = "Possible values of the field `TRIGMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGMODER {
    #[doc = "Channel outputs will generate the normal PWM outputs without generating a pulse."] _0,
    #[doc = "If a match in the channel occurs, a trigger generation on channel output will happen. The trigger pulse width has one FTM clock cycle."]
    _1,
}
impl TRIGMODER {
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
            TRIGMODER::_0 => false,
            TRIGMODER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGMODER {
        match value {
            false => TRIGMODER::_0,
            true => TRIGMODER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TRIGMODER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TRIGMODER::_1
    }
}
#[doc = "Possible values of the field `CHIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHISR {
    #[doc = "The channel (n) input is zero."] _0,
    #[doc = "The channel (n) input is one."] _1,
}
impl CHISR {
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
            CHISR::_0 => false,
            CHISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHISR {
        match value {
            false => CHISR::_0,
            true => CHISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CHISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CHISR::_1
    }
}
#[doc = "Possible values of the field `CHOV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHOVR {
    #[doc = "The channel (n) output is zero."] _0,
    #[doc = "The channel (n) output is one."] _1,
}
impl CHOVR {
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
            CHOVR::_0 => false,
            CHOVR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CHOVR {
        match value {
            false => CHOVR::_0,
            true => CHOVR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CHOVR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CHOVR::_1
    }
}
#[doc = "Values that can be written to the field `DMA`"]
pub enum DMAW {
    #[doc = "Disable DMA transfers."] _0,
    #[doc = "Enable DMA transfers."] _1,
}
impl DMAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAW::_0 => false,
            DMAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable DMA transfers."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAW::_0)
    }
    #[doc = "Enable DMA transfers."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAW::_1)
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
#[doc = "Values that can be written to the field `ICRST`"]
pub enum ICRSTW {
    #[doc = "FTM counter is not reset when the selected channel (n) input event is detected."] _0,
    #[doc = "FTM counter is reset when the selected channel (n) input event is detected."] _1,
}
impl ICRSTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ICRSTW::_0 => false,
            ICRSTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ICRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _ICRSTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ICRSTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "FTM counter is not reset when the selected channel (n) input event is detected."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICRSTW::_0)
    }
    #[doc = "FTM counter is reset when the selected channel (n) input event is detected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICRSTW::_1)
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
#[doc = r" Proxy"]
pub struct _ELSAW<'a> {
    w: &'a mut W,
}
impl<'a> _ELSAW<'a> {
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
pub struct _ELSBW<'a> {
    w: &'a mut W,
}
impl<'a> _ELSBW<'a> {
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
pub struct _MSAW<'a> {
    w: &'a mut W,
}
impl<'a> _MSAW<'a> {
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
#[doc = r" Proxy"]
pub struct _MSBW<'a> {
    w: &'a mut W,
}
impl<'a> _MSBW<'a> {
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
#[doc = "Values that can be written to the field `CHIE`"]
pub enum CHIEW {
    #[doc = "Disable channel (n) interrupt. Use software polling."] _0,
    #[doc = "Enable channel (n) interrupt."] _1,
}
impl CHIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CHIEW::_0 => false,
            CHIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CHIEW<'a> {
    w: &'a mut W,
}
impl<'a> _CHIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CHIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Disable channel (n) interrupt. Use software polling."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHIEW::_0)
    }
    #[doc = "Enable channel (n) interrupt."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHIEW::_1)
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
#[doc = "Values that can be written to the field `TRIGMODE`"]
pub enum TRIGMODEW {
    #[doc = "Channel outputs will generate the normal PWM outputs without generating a pulse."] _0,
    #[doc = "If a match in the channel occurs, a trigger generation on channel output will happen. The trigger pulse width has one FTM clock cycle."]
    _1,
}
impl TRIGMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGMODEW::_0 => false,
            TRIGMODEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGMODEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel outputs will generate the normal PWM outputs without generating a pulse."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRIGMODEW::_0)
    }
    #[doc = "If a match in the channel occurs, a trigger generation on channel output will happen. The trigger pulse width has one FTM clock cycle."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRIGMODEW::_1)
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - DMA Enable"]
    #[inline]
    pub fn dma(&self) -> DMAR {
        DMAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - FTM counter reset by the selected input capture event."]
    #[inline]
    pub fn icrst(&self) -> ICRSTR {
        ICRSTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Channel (n) Edge or Level Select"]
    #[inline]
    pub fn elsa(&self) -> ELSAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ELSAR { bits }
    }
    #[doc = "Bit 3 - Channel (n) Edge or Level Select"]
    #[inline]
    pub fn elsb(&self) -> ELSBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ELSBR { bits }
    }
    #[doc = "Bit 4 - Channel (n) Mode Select"]
    #[inline]
    pub fn msa(&self) -> MSAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MSAR { bits }
    }
    #[doc = "Bit 5 - Channel (n) Mode Select"]
    #[inline]
    pub fn msb(&self) -> MSBR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MSBR { bits }
    }
    #[doc = "Bit 6 - Channel (n) Interrupt Enable"]
    #[inline]
    pub fn chie(&self) -> CHIER {
        CHIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channel (n) Flag"]
    #[inline]
    pub fn chf(&self) -> CHFR {
        CHFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Trigger mode control"]
    #[inline]
    pub fn trigmode(&self) -> TRIGMODER {
        TRIGMODER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Channel (n) Input State"]
    #[inline]
    pub fn chis(&self) -> CHISR {
        CHISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Channel (n) Output Value"]
    #[inline]
    pub fn chov(&self) -> CHOVR {
        CHOVR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
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
    #[doc = "Bit 0 - DMA Enable"]
    #[inline]
    pub fn dma(&mut self) -> _DMAW {
        _DMAW { w: self }
    }
    #[doc = "Bit 1 - FTM counter reset by the selected input capture event."]
    #[inline]
    pub fn icrst(&mut self) -> _ICRSTW {
        _ICRSTW { w: self }
    }
    #[doc = "Bit 2 - Channel (n) Edge or Level Select"]
    #[inline]
    pub fn elsa(&mut self) -> _ELSAW {
        _ELSAW { w: self }
    }
    #[doc = "Bit 3 - Channel (n) Edge or Level Select"]
    #[inline]
    pub fn elsb(&mut self) -> _ELSBW {
        _ELSBW { w: self }
    }
    #[doc = "Bit 4 - Channel (n) Mode Select"]
    #[inline]
    pub fn msa(&mut self) -> _MSAW {
        _MSAW { w: self }
    }
    #[doc = "Bit 5 - Channel (n) Mode Select"]
    #[inline]
    pub fn msb(&mut self) -> _MSBW {
        _MSBW { w: self }
    }
    #[doc = "Bit 6 - Channel (n) Interrupt Enable"]
    #[inline]
    pub fn chie(&mut self) -> _CHIEW {
        _CHIEW { w: self }
    }
    #[doc = "Bit 8 - Trigger mode control"]
    #[inline]
    pub fn trigmode(&mut self) -> _TRIGMODEW {
        _TRIGMODEW { w: self }
    }
}
