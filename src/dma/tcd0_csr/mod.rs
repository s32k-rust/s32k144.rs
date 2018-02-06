#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::TCD0_CSR {
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
#[doc = "Possible values of the field `START`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTR {
    #[doc = "The channel is not explicitly started."]
    _0,
    #[doc = "The channel is explicitly started via a software initiated service request."]
    _1,
}
impl STARTR {
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
            STARTR::_0 => false,
            STARTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> STARTR {
        match value {
            false => STARTR::_0,
            true => STARTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == STARTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == STARTR::_1
    }
}
#[doc = "Possible values of the field `INTMAJOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTMAJORR {
    #[doc = "The end-of-major loop interrupt is disabled."]
    _0,
    #[doc = "The end-of-major loop interrupt is enabled."]
    _1,
}
impl INTMAJORR {
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
            INTMAJORR::_0 => false,
            INTMAJORR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTMAJORR {
        match value {
            false => INTMAJORR::_0,
            true => INTMAJORR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INTMAJORR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INTMAJORR::_1
    }
}
#[doc = "Possible values of the field `INTHALF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTHALFR {
    #[doc = "The half-point interrupt is disabled."]
    _0,
    #[doc = "The half-point interrupt is enabled."]
    _1,
}
impl INTHALFR {
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
            INTHALFR::_0 => false,
            INTHALFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTHALFR {
        match value {
            false => INTHALFR::_0,
            true => INTHALFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == INTHALFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == INTHALFR::_1
    }
}
#[doc = r" Value of the field"]
pub struct DREQR {
    bits: bool,
}
impl DREQR {
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
#[doc = "Possible values of the field `ESG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ESGR {
    #[doc = "The current channel's TCD is normal format."]
    _0,
    #[doc = "The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution."]
    _1,
}
impl ESGR {
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
            ESGR::_0 => false,
            ESGR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ESGR {
        match value {
            false => ESGR::_0,
            true => ESGR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ESGR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ESGR::_1
    }
}
#[doc = "Possible values of the field `MAJORELINK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAJORELINKR {
    #[doc = "The channel-to-channel linking is disabled."]
    _0,
    #[doc = "The channel-to-channel linking is enabled."]
    _1,
}
impl MAJORELINKR {
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
            MAJORELINKR::_0 => false,
            MAJORELINKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAJORELINKR {
        match value {
            false => MAJORELINKR::_0,
            true => MAJORELINKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MAJORELINKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MAJORELINKR::_1
    }
}
#[doc = r" Value of the field"]
pub struct ACTIVER {
    bits: bool,
}
impl ACTIVER {
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
pub struct DONER {
    bits: bool,
}
impl DONER {
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
pub struct MAJORLINKCHR {
    bits: u8,
}
impl MAJORLINKCHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `BWC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BWCR {
    #[doc = "No eDMA engine stalls."]
    _0,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W."]
    _10,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W."]
    _11,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl BWCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            BWCR::_0 => 0,
            BWCR::_10 => 2,
            BWCR::_11 => 3,
            BWCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> BWCR {
        match value {
            0 => BWCR::_0,
            2 => BWCR::_10,
            3 => BWCR::_11,
            i => BWCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BWCR::_0
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == BWCR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == BWCR::_11
    }
}
#[doc = "Values that can be written to the field `START`"]
pub enum STARTW {
    #[doc = "The channel is not explicitly started."]
    _0,
    #[doc = "The channel is explicitly started via a software initiated service request."]
    _1,
}
impl STARTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            STARTW::_0 => false,
            STARTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _STARTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: STARTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel is not explicitly started."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(STARTW::_0)
    }
    #[doc = "The channel is explicitly started via a software initiated service request."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(STARTW::_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INTMAJOR`"]
pub enum INTMAJORW {
    #[doc = "The end-of-major loop interrupt is disabled."]
    _0,
    #[doc = "The end-of-major loop interrupt is enabled."]
    _1,
}
impl INTMAJORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTMAJORW::_0 => false,
            INTMAJORW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTMAJORW<'a> {
    w: &'a mut W,
}
impl<'a> _INTMAJORW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTMAJORW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The end-of-major loop interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTMAJORW::_0)
    }
    #[doc = "The end-of-major loop interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTMAJORW::_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INTHALF`"]
pub enum INTHALFW {
    #[doc = "The half-point interrupt is disabled."]
    _0,
    #[doc = "The half-point interrupt is enabled."]
    _1,
}
impl INTHALFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTHALFW::_0 => false,
            INTHALFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTHALFW<'a> {
    w: &'a mut W,
}
impl<'a> _INTHALFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTHALFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The half-point interrupt is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTHALFW::_0)
    }
    #[doc = "The half-point interrupt is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTHALFW::_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DREQW<'a> {
    w: &'a mut W,
}
impl<'a> _DREQW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ESG`"]
pub enum ESGW {
    #[doc = "The current channel's TCD is normal format."]
    _0,
    #[doc = "The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution."]
    _1,
}
impl ESGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ESGW::_0 => false,
            ESGW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ESGW<'a> {
    w: &'a mut W,
}
impl<'a> _ESGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ESGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The current channel's TCD is normal format."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESGW::_0)
    }
    #[doc = "The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESGW::_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MAJORELINK`"]
pub enum MAJORELINKW {
    #[doc = "The channel-to-channel linking is disabled."]
    _0,
    #[doc = "The channel-to-channel linking is enabled."]
    _1,
}
impl MAJORELINKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAJORELINKW::_0 => false,
            MAJORELINKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAJORELINKW<'a> {
    w: &'a mut W,
}
impl<'a> _MAJORELINKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAJORELINKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The channel-to-channel linking is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAJORELINKW::_0)
    }
    #[doc = "The channel-to-channel linking is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAJORELINKW::_1)
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ACTIVEW<'a> {
    w: &'a mut W,
}
impl<'a> _ACTIVEW<'a> {
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
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DONEW<'a> {
    w: &'a mut W,
}
impl<'a> _DONEW<'a> {
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MAJORLINKCHW<'a> {
    w: &'a mut W,
}
impl<'a> _MAJORLINKCHW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BWC`"]
pub enum BWCW {
    #[doc = "No eDMA engine stalls."]
    _0,
    #[doc = "eDMA engine stalls for 4 cycles after each R/W."]
    _10,
    #[doc = "eDMA engine stalls for 8 cycles after each R/W."]
    _11,
}
impl BWCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            BWCW::_0 => 0,
            BWCW::_10 => 2,
            BWCW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BWCW<'a> {
    w: &'a mut W,
}
impl<'a> _BWCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BWCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No eDMA engine stalls."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWCW::_0)
    }
    #[doc = "eDMA engine stalls for 4 cycles after each R/W."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(BWCW::_10)
    }
    #[doc = "eDMA engine stalls for 8 cycles after each R/W."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(BWCW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bit 0 - Channel Start"]
    #[inline]
    pub fn start(&self) -> STARTR {
        STARTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 1 - Enable an interrupt when major iteration count completes."]
    #[inline]
    pub fn intmajor(&self) -> INTMAJORR {
        INTMAJORR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 2 - Enable an interrupt when major counter is half complete."]
    #[inline]
    pub fn inthalf(&self) -> INTHALFR {
        INTHALFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 3 - Disable Request"]
    #[inline]
    pub fn dreq(&self) -> DREQR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        DREQR { bits }
    }
    #[doc = "Bit 4 - Enable Scatter/Gather Processing"]
    #[inline]
    pub fn esg(&self) -> ESGR {
        ESGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 5 - Enable channel-to-channel linking on major loop complete"]
    #[inline]
    pub fn majorelink(&self) -> MAJORELINKR {
        MAJORELINKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        })
    }
    #[doc = "Bit 6 - Channel Active"]
    #[inline]
    pub fn active(&self) -> ACTIVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        ACTIVER { bits }
    }
    #[doc = "Bit 7 - Channel Done"]
    #[inline]
    pub fn done(&self) -> DONER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u16) != 0
        };
        DONER { bits }
    }
    #[doc = "Bits 8:11 - Major Loop Link Channel Number"]
    #[inline]
    pub fn majorlinkch(&self) -> MAJORLINKCHR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        MAJORLINKCHR { bits }
    }
    #[doc = "Bits 14:15 - Bandwidth Control"]
    #[inline]
    pub fn bwc(&self) -> BWCR {
        BWCR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u16) as u8
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Channel Start"]
    #[inline]
    pub fn start(&mut self) -> _STARTW {
        _STARTW { w: self }
    }
    #[doc = "Bit 1 - Enable an interrupt when major iteration count completes."]
    #[inline]
    pub fn intmajor(&mut self) -> _INTMAJORW {
        _INTMAJORW { w: self }
    }
    #[doc = "Bit 2 - Enable an interrupt when major counter is half complete."]
    #[inline]
    pub fn inthalf(&mut self) -> _INTHALFW {
        _INTHALFW { w: self }
    }
    #[doc = "Bit 3 - Disable Request"]
    #[inline]
    pub fn dreq(&mut self) -> _DREQW {
        _DREQW { w: self }
    }
    #[doc = "Bit 4 - Enable Scatter/Gather Processing"]
    #[inline]
    pub fn esg(&mut self) -> _ESGW {
        _ESGW { w: self }
    }
    #[doc = "Bit 5 - Enable channel-to-channel linking on major loop complete"]
    #[inline]
    pub fn majorelink(&mut self) -> _MAJORELINKW {
        _MAJORELINKW { w: self }
    }
    #[doc = "Bit 6 - Channel Active"]
    #[inline]
    pub fn active(&mut self) -> _ACTIVEW {
        _ACTIVEW { w: self }
    }
    #[doc = "Bit 7 - Channel Done"]
    #[inline]
    pub fn done(&mut self) -> _DONEW {
        _DONEW { w: self }
    }
    #[doc = "Bits 8:11 - Major Loop Link Channel Number"]
    #[inline]
    pub fn majorlinkch(&mut self) -> _MAJORLINKCHW {
        _MAJORLINKCHW { w: self }
    }
    #[doc = "Bits 14:15 - Bandwidth Control"]
    #[inline]
    pub fn bwc(&mut self) -> _BWCW {
        _BWCW { w: self }
    }
}
