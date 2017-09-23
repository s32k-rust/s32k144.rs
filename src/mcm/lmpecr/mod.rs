#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LMPECR {
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
#[doc = "Possible values of the field `ERNCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERNCRR {
    #[doc = "Reporting disabled"] _0,
    #[doc = "Reporting enabled"] _1,
}
impl ERNCRR {
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
            ERNCRR::_0 => false,
            ERNCRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ERNCRR {
        match value {
            false => ERNCRR::_0,
            true => ERNCRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ERNCRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ERNCRR::_1
    }
}
#[doc = "Possible values of the field `ER1BR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ER1BRR {
    #[doc = "Reporting disabled"] _0,
    #[doc = "Reporting enabled"] _1,
}
impl ER1BRR {
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
            ER1BRR::_0 => false,
            ER1BRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ER1BRR {
        match value {
            false => ER1BRR::_0,
            true => ER1BRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ER1BRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ER1BRR::_1
    }
}
#[doc = "Possible values of the field `ECPR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ECPRR {
    #[doc = "Reporting disabled"] _0,
    #[doc = "Reporting enabled"] _1,
}
impl ECPRR {
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
            ECPRR::_0 => false,
            ECPRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> ECPRR {
        match value {
            false => ECPRR::_0,
            true => ECPRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == ECPRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == ECPRR::_1
    }
}
#[doc = "Values that can be written to the field `ERNCR`"]
pub enum ERNCRW {
    #[doc = "Reporting disabled"] _0,
    #[doc = "Reporting enabled"] _1,
}
impl ERNCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ERNCRW::_0 => false,
            ERNCRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ERNCRW<'a> {
    w: &'a mut W,
}
impl<'a> _ERNCRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ERNCRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reporting disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERNCRW::_0)
    }
    #[doc = "Reporting enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERNCRW::_1)
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
#[doc = "Values that can be written to the field `ER1BR`"]
pub enum ER1BRW {
    #[doc = "Reporting disabled"] _0,
    #[doc = "Reporting enabled"] _1,
}
impl ER1BRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ER1BRW::_0 => false,
            ER1BRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ER1BRW<'a> {
    w: &'a mut W,
}
impl<'a> _ER1BRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ER1BRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reporting disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ER1BRW::_0)
    }
    #[doc = "Reporting enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ER1BRW::_1)
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
#[doc = "Values that can be written to the field `ECPR`"]
pub enum ECPRW {
    #[doc = "Reporting disabled"] _0,
    #[doc = "Reporting enabled"] _1,
}
impl ECPRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ECPRW::_0 => false,
            ECPRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ECPRW<'a> {
    w: &'a mut W,
}
impl<'a> _ECPRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ECPRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reporting disabled"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECPRW::_0)
    }
    #[doc = "Reporting enabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECPRW::_1)
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
        const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - Enable RAM ECC Noncorrectable Reporting"]
    #[inline]
    pub fn erncr(&self) -> ERNCRR {
        ERNCRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Enable RAM ECC 1 Bit Reporting"]
    #[inline]
    pub fn er1br(&self) -> ER1BRR {
        ER1BRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Enable Cache Parity Reporting"]
    #[inline]
    pub fn ecpr(&self) -> ECPRR {
        ECPRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
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
    #[doc = "Bit 0 - Enable RAM ECC Noncorrectable Reporting"]
    #[inline]
    pub fn erncr(&mut self) -> _ERNCRW {
        _ERNCRW { w: self }
    }
    #[doc = "Bit 8 - Enable RAM ECC 1 Bit Reporting"]
    #[inline]
    pub fn er1br(&mut self) -> _ER1BRW {
        _ER1BRW { w: self }
    }
    #[doc = "Bit 20 - Enable Cache Parity Reporting"]
    #[inline]
    pub fn ecpr(&mut self) -> _ECPRW {
        _ECPRW { w: self }
    }
}
