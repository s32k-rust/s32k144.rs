#[doc = "Reader of register CP0NUM"]
pub type R = crate::R<u32, super::CP0NUM>;
#[doc = "Reader of field `CPN`"]
pub type CPN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Processor 0 Number"]
    #[inline(always)]
    pub fn cpn(&self) -> CPN_R {
        CPN_R::new((self.bits & 0x01) != 0)
    }
}
