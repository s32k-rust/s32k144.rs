#[doc = "Reader of register SR0"]
pub type R = crate::R<u32, super::SR0>;
#[doc = "Writer for register SR0"]
pub type W = crate::W<u32, super::SR0>;
#[doc = "Register SR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "NCE1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCE1_A {
    #[doc = "0: No non-correctable error event on Memory 1 detected"]
    _0 = 0,
    #[doc = "1: Non-correctable error event on Memory 1 detected"]
    _1 = 1,
}
impl From<NCE1_A> for bool {
    #[inline(always)]
    fn from(variant: NCE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NCE1`"]
pub type NCE1_R = crate::R<bool, NCE1_A>;
impl NCE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCE1_A {
        match self.bits {
            false => NCE1_A::_0,
            true => NCE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NCE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NCE1_A::_1
    }
}
#[doc = "Write proxy for field `NCE1`"]
pub struct NCE1_W<'a> {
    w: &'a mut W,
}
impl<'a> NCE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NCE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No non-correctable error event on Memory 1 detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NCE1_A::_0)
    }
    #[doc = "Non-correctable error event on Memory 1 detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NCE1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "SBC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBC1_A {
    #[doc = "0: No single-bit correction event on Memory 1 detected"]
    _0 = 0,
    #[doc = "1: Single-bit correction event on Memory 1 detected"]
    _1 = 1,
}
impl From<SBC1_A> for bool {
    #[inline(always)]
    fn from(variant: SBC1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SBC1`"]
pub type SBC1_R = crate::R<bool, SBC1_A>;
impl SBC1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBC1_A {
        match self.bits {
            false => SBC1_A::_0,
            true => SBC1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBC1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBC1_A::_1
    }
}
#[doc = "Write proxy for field `SBC1`"]
pub struct SBC1_W<'a> {
    w: &'a mut W,
}
impl<'a> SBC1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBC1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No single-bit correction event on Memory 1 detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBC1_A::_0)
    }
    #[doc = "Single-bit correction event on Memory 1 detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBC1_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "NCE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NCE0_A {
    #[doc = "0: No non-correctable error event on Memory 0 detected"]
    _0 = 0,
    #[doc = "1: Non-correctable error event on Memory 0 detected"]
    _1 = 1,
}
impl From<NCE0_A> for bool {
    #[inline(always)]
    fn from(variant: NCE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NCE0`"]
pub type NCE0_R = crate::R<bool, NCE0_A>;
impl NCE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCE0_A {
        match self.bits {
            false => NCE0_A::_0,
            true => NCE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NCE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NCE0_A::_1
    }
}
#[doc = "Write proxy for field `NCE0`"]
pub struct NCE0_W<'a> {
    w: &'a mut W,
}
impl<'a> NCE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NCE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No non-correctable error event on Memory 0 detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NCE0_A::_0)
    }
    #[doc = "Non-correctable error event on Memory 0 detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NCE0_A::_1)
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
#[doc = "SBC0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBC0_A {
    #[doc = "0: No single-bit correction event on Memory 0 detected"]
    _0 = 0,
    #[doc = "1: Single-bit correction event on Memory 0 detected"]
    _1 = 1,
}
impl From<SBC0_A> for bool {
    #[inline(always)]
    fn from(variant: SBC0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SBC0`"]
pub type SBC0_R = crate::R<bool, SBC0_A>;
impl SBC0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBC0_A {
        match self.bits {
            false => SBC0_A::_0,
            true => SBC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBC0_A::_1
    }
}
#[doc = "Write proxy for field `SBC0`"]
pub struct SBC0_W<'a> {
    w: &'a mut W,
}
impl<'a> SBC0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SBC0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No single-bit correction event on Memory 0 detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBC0_A::_0)
    }
    #[doc = "Single-bit correction event on Memory 0 detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBC0_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 26 - NCE1"]
    #[inline(always)]
    pub fn nce1(&self) -> NCE1_R {
        NCE1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - SBC1"]
    #[inline(always)]
    pub fn sbc1(&self) -> SBC1_R {
        SBC1_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 30 - NCE0"]
    #[inline(always)]
    pub fn nce0(&self) -> NCE0_R {
        NCE0_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SBC0"]
    #[inline(always)]
    pub fn sbc0(&self) -> SBC0_R {
        SBC0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 26 - NCE1"]
    #[inline(always)]
    pub fn nce1(&mut self) -> NCE1_W {
        NCE1_W { w: self }
    }
    #[doc = "Bit 27 - SBC1"]
    #[inline(always)]
    pub fn sbc1(&mut self) -> SBC1_W {
        SBC1_W { w: self }
    }
    #[doc = "Bit 30 - NCE0"]
    #[inline(always)]
    pub fn nce0(&mut self) -> NCE0_W {
        NCE0_W { w: self }
    }
    #[doc = "Bit 31 - SBC0"]
    #[inline(always)]
    pub fn sbc0(&mut self) -> SBC0_W {
        SBC0_W { w: self }
    }
}
