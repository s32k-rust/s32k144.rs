#[doc = "Reader of register PARAM"]
pub type R = crate::R<u32, super::PARAM>;
#[doc = "Reader of field `CLKPRES`"]
pub type CLKPRES_R = crate::R<u8, u8>;
#[doc = "Reader of field `DIVPRES`"]
pub type DIVPRES_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Clock Present"]
    #[inline(always)]
    pub fn clkpres(&self) -> CLKPRES_R {
        CLKPRES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 27:31 - Divider Present"]
    #[inline(always)]
    pub fn divpres(&self) -> DIVPRES_R {
        DIVPRES_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
