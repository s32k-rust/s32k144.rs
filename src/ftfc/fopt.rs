#[doc = "Reader of register FOPT"]
pub type R = crate::R<u8, super::FOPT>;
#[doc = "Reader of field `OPT`"]
pub type OPT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Nonvolatile Option"]
    #[inline(always)]
    pub fn opt(&self) -> OPT_R {
        OPT_R::new((self.bits & 0xff) as u8)
    }
}
