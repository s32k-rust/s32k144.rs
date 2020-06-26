#[doc = "Reader of register EAR1"]
pub type R = crate::R<u32, super::EAR1>;
#[doc = "Reader of field `EAR`"]
pub type EAR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - EAR"]
    #[inline(always)]
    pub fn ear(&self) -> EAR_R {
        EAR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
