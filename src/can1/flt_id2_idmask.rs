#[doc = "Reader of register FLT_ID2_IDMASK"]
pub type R = crate::R<u32, super::FLT_ID2_IDMASK>;
#[doc = "Writer for register FLT_ID2_IDMASK"]
pub type W = crate::W<u32, super::FLT_ID2_IDMASK>;
#[doc = "Register FLT_ID2_IDMASK `reset()`'s with value 0"]
impl crate::ResetValue for super::FLT_ID2_IDMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLT_ID2_IDMASK`"]
pub type FLT_ID2_IDMASK_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FLT_ID2_IDMASK`"]
pub struct FLT_ID2_IDMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_ID2_IDMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
#[doc = "Remote Transmission Request Mask Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTR_MSK_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked"]
    _1 = 1,
}
impl From<RTR_MSK_A> for bool {
    #[inline(always)]
    fn from(variant: RTR_MSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RTR_MSK`"]
pub type RTR_MSK_R = crate::R<bool, RTR_MSK_A>;
impl RTR_MSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTR_MSK_A {
        match self.bits {
            false => RTR_MSK_A::_0,
            true => RTR_MSK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTR_MSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTR_MSK_A::_1
    }
}
#[doc = "Write proxy for field `RTR_MSK`"]
pub struct RTR_MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> RTR_MSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTR_MSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTR_MSK_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTR_MSK_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "ID Extended Mask Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDE_MSK_A {
    #[doc = "0: The corresponding bit in the filter is \"don't care\""]
    _0 = 0,
    #[doc = "1: The corresponding bit in the filter is checked"]
    _1 = 1,
}
impl From<IDE_MSK_A> for bool {
    #[inline(always)]
    fn from(variant: IDE_MSK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IDE_MSK`"]
pub type IDE_MSK_R = crate::R<bool, IDE_MSK_A>;
impl IDE_MSK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDE_MSK_A {
        match self.bits {
            false => IDE_MSK_A::_0,
            true => IDE_MSK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDE_MSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDE_MSK_A::_1
    }
}
#[doc = "Write proxy for field `IDE_MSK`"]
pub struct IDE_MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> IDE_MSK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDE_MSK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The corresponding bit in the filter is \"don't care\""]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDE_MSK_A::_0)
    }
    #[doc = "The corresponding bit in the filter is checked"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDE_MSK_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:28 - ID Filter 2 for Pretended Networking Filtering / ID Mask Bits for Pretended Networking ID Filtering"]
    #[inline(always)]
    pub fn flt_id2_idmask(&self) -> FLT_ID2_IDMASK_R {
        FLT_ID2_IDMASK_R::new((self.bits & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 29 - Remote Transmission Request Mask Bit"]
    #[inline(always)]
    pub fn rtr_msk(&self) -> RTR_MSK_R {
        RTR_MSK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ID Extended Mask Bit"]
    #[inline(always)]
    pub fn ide_msk(&self) -> IDE_MSK_R {
        IDE_MSK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - ID Filter 2 for Pretended Networking Filtering / ID Mask Bits for Pretended Networking ID Filtering"]
    #[inline(always)]
    pub fn flt_id2_idmask(&mut self) -> FLT_ID2_IDMASK_W {
        FLT_ID2_IDMASK_W { w: self }
    }
    #[doc = "Bit 29 - Remote Transmission Request Mask Bit"]
    #[inline(always)]
    pub fn rtr_msk(&mut self) -> RTR_MSK_W {
        RTR_MSK_W { w: self }
    }
    #[doc = "Bit 30 - ID Extended Mask Bit"]
    #[inline(always)]
    pub fn ide_msk(&mut self) -> IDE_MSK_W {
        IDE_MSK_W { w: self }
    }
}
