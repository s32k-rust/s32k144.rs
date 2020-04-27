#[doc = "Reader of register PDIR"]
pub type R = crate::R<u32, super::PDIR>;
#[doc = "Reader of field `PDI`"]
pub type PDI_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Port Data Input"]
    #[inline(always)]
    pub fn pdi(&self) -> PDI_R {
        PDI_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
