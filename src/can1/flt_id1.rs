#[doc = "Reader of register FLT_ID1"]
pub type R = crate::R<u32, super::FLT_ID1>;
#[doc = "Writer for register FLT_ID1"]
pub type W = crate::W<u32, super::FLT_ID1>;
#[doc = "Register FLT_ID1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FLT_ID1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FLT_ID1`"]
pub type FLT_ID1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FLT_ID1`"]
pub struct FLT_ID1_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_ID1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff_ffff) | ((value as u32) & 0x1fff_ffff);
        self.w
    }
}
#[doc = "Remote Transmission Request Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT_RTR_A {
    #[doc = "0: Reject remote frame (accept data frame)"]
    _0 = 0,
    #[doc = "1: Accept remote frame"]
    _1 = 1,
}
impl From<FLT_RTR_A> for bool {
    #[inline(always)]
    fn from(variant: FLT_RTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLT_RTR`"]
pub type FLT_RTR_R = crate::R<bool, FLT_RTR_A>;
impl FLT_RTR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT_RTR_A {
        match self.bits {
            false => FLT_RTR_A::_0,
            true => FLT_RTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLT_RTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLT_RTR_A::_1
    }
}
#[doc = "Write proxy for field `FLT_RTR`"]
pub struct FLT_RTR_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_RTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT_RTR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Reject remote frame (accept data frame)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT_RTR_A::_0)
    }
    #[doc = "Accept remote frame"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLT_RTR_A::_1)
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
#[doc = "ID Extended Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLT_IDE_A {
    #[doc = "0: Accept standard frame format"]
    _0 = 0,
    #[doc = "1: Accept extended frame format"]
    _1 = 1,
}
impl From<FLT_IDE_A> for bool {
    #[inline(always)]
    fn from(variant: FLT_IDE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FLT_IDE`"]
pub type FLT_IDE_R = crate::R<bool, FLT_IDE_A>;
impl FLT_IDE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLT_IDE_A {
        match self.bits {
            false => FLT_IDE_A::_0,
            true => FLT_IDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLT_IDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLT_IDE_A::_1
    }
}
#[doc = "Write proxy for field `FLT_IDE`"]
pub struct FLT_IDE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLT_IDE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLT_IDE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Accept standard frame format"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLT_IDE_A::_0)
    }
    #[doc = "Accept extended frame format"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLT_IDE_A::_1)
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
    #[doc = "Bits 0:28 - ID Filter 1 for Pretended Networking filtering"]
    #[inline(always)]
    pub fn flt_id1(&self) -> FLT_ID1_R {
        FLT_ID1_R::new((self.bits & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 29 - Remote Transmission Request Filter"]
    #[inline(always)]
    pub fn flt_rtr(&self) -> FLT_RTR_R {
        FLT_RTR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - ID Extended Filter"]
    #[inline(always)]
    pub fn flt_ide(&self) -> FLT_IDE_R {
        FLT_IDE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - ID Filter 1 for Pretended Networking filtering"]
    #[inline(always)]
    pub fn flt_id1(&mut self) -> FLT_ID1_W {
        FLT_ID1_W { w: self }
    }
    #[doc = "Bit 29 - Remote Transmission Request Filter"]
    #[inline(always)]
    pub fn flt_rtr(&mut self) -> FLT_RTR_W {
        FLT_RTR_W { w: self }
    }
    #[doc = "Bit 30 - ID Extended Filter"]
    #[inline(always)]
    pub fn flt_ide(&mut self) -> FLT_IDE_W {
        FLT_IDE_W { w: self }
    }
}
