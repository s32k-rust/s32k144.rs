#[doc = "Reader of register CPxNUM"]
pub type R = crate::R<u32, super::CPXNUM>;
#[doc = "Reader of field `CPN`"]
pub type CPN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Processor x Number"]
    #[inline(always)]
    pub fn cpn(&self) -> CPN_R {
        CPN_R::new((self.bits & 0x01) != 0)
    }
}
