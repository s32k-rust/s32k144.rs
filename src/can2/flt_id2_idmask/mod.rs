#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLT_ID2_IDMASK {
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
pub struct FLT_ID2_IDMASKR {
    bits: u32,
}
impl FLT_ID2_IDMASKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Possible values of the field `RTR_MSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTR_MSKR {
    #[doc = "The corresponding bit in the filter is \"don't care\""] _0,
    #[doc = "The corresponding bit in the filter is checked"] _1,
}
impl RTR_MSKR {
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
            RTR_MSKR::_0 => false,
            RTR_MSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RTR_MSKR {
        match value {
            false => RTR_MSKR::_0,
            true => RTR_MSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RTR_MSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RTR_MSKR::_1
    }
}
#[doc = "Possible values of the field `IDE_MSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDE_MSKR {
    #[doc = "The corresponding bit in the filter is \"don't care\""] _0,
    #[doc = "The corresponding bit in the filter is checked"] _1,
}
impl IDE_MSKR {
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
            IDE_MSKR::_0 => false,
            IDE_MSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> IDE_MSKR {
        match value {
            false => IDE_MSKR::_0,
            true => IDE_MSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == IDE_MSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == IDE_MSKR::_1
    }
}
#[doc = r" Proxy"]
pub struct _FLT_ID2_IDMASKW<'a> {
    w: &'a mut W,
}
impl<'a> _FLT_ID2_IDMASKW<'a> {
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
#[doc = "Values that can be written to the field `RTR_MSK`"]
pub enum RTR_MSKW {
    #[doc = "The corresponding bit in the filter is \"don't care\""] _0,
    #[doc = "The corresponding bit in the filter is checked"] _1,
}
impl RTR_MSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RTR_MSKW::_0 => false,
            RTR_MSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RTR_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _RTR_MSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RTR_MSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTR_MSKW::_0)
    }
    #[doc = "The corresponding bit in the filter is checked"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTR_MSKW::_1)
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
#[doc = "Values that can be written to the field `IDE_MSK`"]
pub enum IDE_MSKW {
    #[doc = "The corresponding bit in the filter is \"don't care\""] _0,
    #[doc = "The corresponding bit in the filter is checked"] _1,
}
impl IDE_MSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            IDE_MSKW::_0 => false,
            IDE_MSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IDE_MSKW<'a> {
    w: &'a mut W,
}
impl<'a> _IDE_MSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IDE_MSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care\""]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDE_MSKW::_0)
    }
    #[doc = "The corresponding bit in the filter is checked"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDE_MSKW::_1)
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
    #[doc = "Bits 0:28 - ID Filter 2 for Pretended Networking Filtering / ID Mask Bits for Pretended Networking ID Filtering"]
    #[inline]
    pub fn flt_id2_idmask(&self) -> FLT_ID2_IDMASKR {
        let bits = {
            const MASK: u32 = 536870911;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        FLT_ID2_IDMASKR { bits }
    }
    #[doc = "Bit 29 - Remote Transmission Request Mask Bit"]
    #[inline]
    pub fn rtr_msk(&self) -> RTR_MSKR {
        RTR_MSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - ID Extended Mask Bit"]
    #[inline]
    pub fn ide_msk(&self) -> IDE_MSKR {
        IDE_MSKR::_from({
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
    #[doc = "Bits 0:28 - ID Filter 2 for Pretended Networking Filtering / ID Mask Bits for Pretended Networking ID Filtering"]
    #[inline]
    pub fn flt_id2_idmask(&mut self) -> _FLT_ID2_IDMASKW {
        _FLT_ID2_IDMASKW { w: self }
    }
    #[doc = "Bit 29 - Remote Transmission Request Mask Bit"]
    #[inline]
    pub fn rtr_msk(&mut self) -> _RTR_MSKW {
        _RTR_MSKW { w: self }
    }
    #[doc = "Bit 30 - ID Extended Mask Bit"]
    #[inline]
    pub fn ide_msk(&mut self) -> _IDE_MSKW {
        _IDE_MSKW { w: self }
    }
}
