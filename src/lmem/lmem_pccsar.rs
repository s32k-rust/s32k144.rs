#[doc = "Reader of register LMEM_PCCSAR"]
pub type R = crate::R<u32, super::LMEM_PCCSAR>;
#[doc = "Writer for register LMEM_PCCSAR"]
pub type W = crate::W<u32, super::LMEM_PCCSAR>;
#[doc = "Register LMEM_PCCSAR `reset()`'s with value 0"]
impl crate::ResetValue for super::LMEM_PCCSAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Initiate Cache Line Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LGO_A {
    #[doc = "0: Write: no effect. Read: no line command active."]
    _0 = 0,
    #[doc = "1: Write: initiate line command indicated by bits CLCR\\[27:24\\]. Read: line command active."]
    _1 = 1,
}
impl From<LGO_A> for bool {
    #[inline(always)]
    fn from(variant: LGO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LGO`"]
pub type LGO_R = crate::R<bool, LGO_A>;
impl LGO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LGO_A {
        match self.bits {
            false => LGO_A::_0,
            true => LGO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LGO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LGO_A::_1
    }
}
#[doc = "Write proxy for field `LGO`"]
pub struct LGO_W<'a> {
    w: &'a mut W,
}
impl<'a> LGO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LGO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Write: no effect. Read: no line command active."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LGO_A::_0)
    }
    #[doc = "Write: initiate line command indicated by bits CLCR\\[27:24\\]. Read: line command active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LGO_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `PHYADDR`"]
pub type PHYADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PHYADDR`"]
pub struct PHYADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff_ffff << 2)) | (((value as u32) & 0x3fff_ffff) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline(always)]
    pub fn lgo(&self) -> LGO_R {
        LGO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:31 - Physical Address"]
    #[inline(always)]
    pub fn phyaddr(&self) -> PHYADDR_R {
        PHYADDR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline(always)]
    pub fn lgo(&mut self) -> LGO_W {
        LGO_W { w: self }
    }
    #[doc = "Bits 2:31 - Physical Address"]
    #[inline(always)]
    pub fn phyaddr(&mut self) -> PHYADDR_W {
        PHYADDR_W { w: self }
    }
}
