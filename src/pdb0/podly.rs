#[doc = "Reader of register PODLY"]
pub type R = crate::R<u32, super::PODLY>;
#[doc = "Writer for register PODLY"]
pub type W = crate::W<u32, super::PODLY>;
#[doc = "Register PODLY `reset()`'s with value 0"]
impl crate::ResetValue for super::PODLY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DLY2`"]
pub type DLY2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DLY2`"]
pub struct DLY2_W<'a> {
    w: &'a mut W,
}
impl<'a> DLY2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `DLY1`"]
pub type DLY1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DLY1`"]
pub struct DLY1_W<'a> {
    w: &'a mut W,
}
impl<'a> DLY1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - PDB Pulse-Out Delay 2"]
    #[inline(always)]
    pub fn dly2(&self) -> DLY2_R {
        DLY2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PDB Pulse-Out Delay 1"]
    #[inline(always)]
    pub fn dly1(&self) -> DLY1_R {
        DLY1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PDB Pulse-Out Delay 2"]
    #[inline(always)]
    pub fn dly2(&mut self) -> DLY2_W {
        DLY2_W { w: self }
    }
    #[doc = "Bits 16:31 - PDB Pulse-Out Delay 1"]
    #[inline(always)]
    pub fn dly1(&mut self) -> DLY1_W {
        DLY1_W { w: self }
    }
}
