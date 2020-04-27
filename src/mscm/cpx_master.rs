#[doc = "Reader of register CPxMASTER"]
pub type R = crate::R<u32, super::CPXMASTER>;
#[doc = "Reader of field `PPMN`"]
pub type PPMN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Processor x Physical Master Number"]
    #[inline(always)]
    pub fn ppmn(&self) -> PPMN_R {
        PPMN_R::new((self.bits & 0x3f) as u8)
    }
}
