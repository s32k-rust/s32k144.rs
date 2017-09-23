#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLT_ID1 {
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
pub struct FLT_ID1R {
    bits: u32,
}
impl FLT_ID1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `FLT_RTR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT_RTRR {
    #[doc = "Reject remote frame (accept data frame)"] _0,
    #[doc = "Accept remote frame"] _1,
}
impl FLT_RTRR {
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
            FLT_RTRR::_0 => false,
            FLT_RTRR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLT_RTRR {
        match value {
            false => FLT_RTRR::_0,
            true => FLT_RTRR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FLT_RTRR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FLT_RTRR::_1
    }
}
#[doc = "Possible values of the field `FLT_IDE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT_IDER {
    #[doc = "Accept standard frame format"] _0,
    #[doc = "Accept extended frame format"] _1,
}
impl FLT_IDER {
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
            FLT_IDER::_0 => false,
            FLT_IDER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> FLT_IDER {
        match value {
            false => FLT_IDER::_0,
            true => FLT_IDER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == FLT_IDER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == FLT_IDER::_1
    }
}
#[doc = r" Proxy"]
pub struct _FLT_ID1W<'a> {
    w: &'a mut W,
}
impl<'a> _FLT_ID1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 536870911;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLT_RTR`"]
pub enum FLT_RTRW {
    #[doc = "Reject remote frame (accept data frame)"] _0,
    #[doc = "Accept remote frame"] _1,
}
impl FLT_RTRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLT_RTRW::_0 => false,
            FLT_RTRW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLT_RTRW<'a> {
    w: &'a mut W,
}
impl<'a> _FLT_RTRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLT_RTRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Reject remote frame (accept data frame)"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT_RTRW::_0)
    }
    #[doc = "Accept remote frame"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLT_RTRW::_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `FLT_IDE`"]
pub enum FLT_IDEW {
    #[doc = "Accept standard frame format"] _0,
    #[doc = "Accept extended frame format"] _1,
}
impl FLT_IDEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            FLT_IDEW::_0 => false,
            FLT_IDEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _FLT_IDEW<'a> {
    w: &'a mut W,
}
impl<'a> _FLT_IDEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: FLT_IDEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accept standard frame format"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT_IDEW::_0)
    }
    #[doc = "Accept extended frame format"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLT_IDEW::_1)
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
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:28 - ID Filter 1 for Pretended Networking filtering"]
    #[inline]
    pub fn flt_id1(&self) -> FLT_ID1R {
        let bits = {
            const MASK: u32 = 536870911;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        FLT_ID1R { bits }
    }
    #[doc = "Bit 29 - Remote Transmission Request Filter"]
    #[inline]
    pub fn flt_rtr(&self) -> FLT_RTRR {
        FLT_RTRR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - ID Extended Filter"]
    #[inline]
    pub fn flt_ide(&self) -> FLT_IDER {
        FLT_IDER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:28 - ID Filter 1 for Pretended Networking filtering"]
    #[inline]
    pub fn flt_id1(&mut self) -> _FLT_ID1W {
        _FLT_ID1W { w: self }
    }
    #[doc = "Bit 29 - Remote Transmission Request Filter"]
    #[inline]
    pub fn flt_rtr(&mut self) -> _FLT_RTRW {
        _FLT_RTRW { w: self }
    }
    #[doc = "Bit 30 - ID Extended Filter"]
    #[inline]
    pub fn flt_ide(&mut self) -> _FLT_IDEW {
        _FLT_IDEW { w: self }
    }
}
