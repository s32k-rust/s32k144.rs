#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SR {
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
#[doc = "Possible values of the field `TDF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDFR {
    #[doc = "Transmit data not requested."]
    _0,
    #[doc = "Transmit data is requested."]
    _1,
}
impl TDFR {
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
            TDFR::_0 => false,
            TDFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDFR {
        match value {
            false => TDFR::_0,
            true => TDFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TDFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TDFR::_1
    }
}
#[doc = "Possible values of the field `RDF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDFR {
    #[doc = "Receive Data is not ready."]
    _0,
    #[doc = "Receive data is ready."]
    _1,
}
impl RDFR {
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
            RDFR::_0 => false,
            RDFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDFR {
        match value {
            false => RDFR::_0,
            true => RDFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RDFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RDFR::_1
    }
}
#[doc = "Possible values of the field `WCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCFR {
    #[doc = "Transfer word not completed."]
    _0,
    #[doc = "Transfer word completed."]
    _1,
}
impl WCFR {
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
            WCFR::_0 => false,
            WCFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WCFR {
        match value {
            false => WCFR::_0,
            true => WCFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WCFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WCFR::_1
    }
}
#[doc = "Possible values of the field `FCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCFR {
    #[doc = "Frame transfer has not completed."]
    _0,
    #[doc = "Frame transfer has completed."]
    _1,
}
impl FCFR {
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
            FCFR::_0 => false,
            FCFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FCFR {
        match value {
            false => FCFR::_0,
            true => FCFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FCFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FCFR::_1
    }
}
#[doc = "Possible values of the field `TCF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TCFR {
    #[doc = "All transfers have not completed."]
    _0,
    #[doc = "All transfers have completed."]
    _1,
}
impl TCFR {
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
            TCFR::_0 => false,
            TCFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TCFR {
        match value {
            false => TCFR::_0,
            true => TCFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TCFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TCFR::_1
    }
}
#[doc = "Possible values of the field `TEF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TEFR {
    #[doc = "Transmit FIFO underrun has not occurred."]
    _0,
    #[doc = "Transmit FIFO underrun has occurred"]
    _1,
}
impl TEFR {
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
            TEFR::_0 => false,
            TEFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TEFR {
        match value {
            false => TEFR::_0,
            true => TEFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TEFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TEFR::_1
    }
}
#[doc = "Possible values of the field `REF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFR {
    #[doc = "Receive FIFO has not overflowed."]
    _0,
    #[doc = "Receive FIFO has overflowed."]
    _1,
}
impl REFR {
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
            REFR::_0 => false,
            REFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> REFR {
        match value {
            false => REFR::_0,
            true => REFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == REFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == REFR::_1
    }
}
#[doc = "Possible values of the field `DMF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMFR {
    #[doc = "Have not received matching data."]
    _0,
    #[doc = "Have received matching data."]
    _1,
}
impl DMFR {
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
            DMFR::_0 => false,
            DMFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMFR {
        match value {
            false => DMFR::_0,
            true => DMFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMFR::_1
    }
}
#[doc = "Possible values of the field `MBF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBFR {
    #[doc = "LPSPI is idle."]
    _0,
    #[doc = "LPSPI is busy."]
    _1,
}
impl MBFR {
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
            MBFR::_0 => false,
            MBFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MBFR {
        match value {
            false => MBFR::_0,
            true => MBFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MBFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MBFR::_1
    }
}
#[doc = "Values that can be written to the field `WCF`"]
pub enum WCFW {
    #[doc = "Transfer word not completed."]
    _0,
    #[doc = "Transfer word completed."]
    _1,
}
impl WCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WCFW::_0 => false,
            WCFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WCFW<'a> {
    w: &'a mut W,
}
impl<'a> _WCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transfer word not completed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WCFW::_0)
    }
    #[doc = "Transfer word completed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WCFW::_1)
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
#[doc = "Values that can be written to the field `FCF`"]
pub enum FCFW {
    #[doc = "Frame transfer has not completed."]
    _0,
    #[doc = "Frame transfer has completed."]
    _1,
}
impl FCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FCFW::_0 => false,
            FCFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FCFW<'a> {
    w: &'a mut W,
}
impl<'a> _FCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Frame transfer has not completed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FCFW::_0)
    }
    #[doc = "Frame transfer has completed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FCFW::_1)
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
#[doc = "Values that can be written to the field `TCF`"]
pub enum TCFW {
    #[doc = "All transfers have not completed."]
    _0,
    #[doc = "All transfers have completed."]
    _1,
}
impl TCFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TCFW::_0 => false,
            TCFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TCFW<'a> {
    w: &'a mut W,
}
impl<'a> _TCFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TCFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "All transfers have not completed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCFW::_0)
    }
    #[doc = "All transfers have completed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCFW::_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TEF`"]
pub enum TEFW {
    #[doc = "Transmit FIFO underrun has not occurred."]
    _0,
    #[doc = "Transmit FIFO underrun has occurred"]
    _1,
}
impl TEFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TEFW::_0 => false,
            TEFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TEFW<'a> {
    w: &'a mut W,
}
impl<'a> _TEFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TEFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Transmit FIFO underrun has not occurred."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEFW::_0)
    }
    #[doc = "Transmit FIFO underrun has occurred"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEFW::_1)
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
#[doc = "Values that can be written to the field `REF`"]
pub enum REFW {
    #[doc = "Receive FIFO has not overflowed."]
    _0,
    #[doc = "Receive FIFO has overflowed."]
    _1,
}
impl REFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            REFW::_0 => false,
            REFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _REFW<'a> {
    w: &'a mut W,
}
impl<'a> _REFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: REFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receive FIFO has not overflowed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(REFW::_0)
    }
    #[doc = "Receive FIFO has overflowed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(REFW::_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMF`"]
pub enum DMFW {
    #[doc = "Have not received matching data."]
    _0,
    #[doc = "Have received matching data."]
    _1,
}
impl DMFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMFW::_0 => false,
            DMFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMFW<'a> {
    w: &'a mut W,
}
impl<'a> _DMFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Have not received matching data."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMFW::_0)
    }
    #[doc = "Have received matching data."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMFW::_1)
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
        const OFFSET: u8 = 13;
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
    #[doc = "Bit 0 - Transmit Data Flag"]
    #[inline]
    pub fn tdf(&self) -> TDFR {
        TDFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Receive Data Flag"]
    #[inline]
    pub fn rdf(&self) -> RDFR {
        RDFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Word Complete Flag"]
    #[inline]
    pub fn wcf(&self) -> WCFR {
        WCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Frame Complete Flag"]
    #[inline]
    pub fn fcf(&self) -> FCFR {
        FCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Transfer Complete Flag"]
    #[inline]
    pub fn tcf(&self) -> TCFR {
        TCFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 11 - Transmit Error Flag"]
    #[inline]
    pub fn tef(&self) -> TEFR {
        TEFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Receive Error Flag"]
    #[inline]
    pub fn ref_(&self) -> REFR {
        REFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Data Match Flag"]
    #[inline]
    pub fn dmf(&self) -> DMFR {
        DMFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Module Busy Flag"]
    #[inline]
    pub fn mbf(&self) -> MBFR {
        MBFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 8 - Word Complete Flag"]
    #[inline]
    pub fn wcf(&mut self) -> _WCFW {
        _WCFW { w: self }
    }
    #[doc = "Bit 9 - Frame Complete Flag"]
    #[inline]
    pub fn fcf(&mut self) -> _FCFW {
        _FCFW { w: self }
    }
    #[doc = "Bit 10 - Transfer Complete Flag"]
    #[inline]
    pub fn tcf(&mut self) -> _TCFW {
        _TCFW { w: self }
    }
    #[doc = "Bit 11 - Transmit Error Flag"]
    #[inline]
    pub fn tef(&mut self) -> _TEFW {
        _TEFW { w: self }
    }
    #[doc = "Bit 12 - Receive Error Flag"]
    #[inline]
    pub fn ref_(&mut self) -> _REFW {
        _REFW { w: self }
    }
    #[doc = "Bit 13 - Data Match Flag"]
    #[inline]
    pub fn dmf(&mut self) -> _DMFW {
        _DMFW { w: self }
    }
}
