#[doc = "Reader of register PIN"]
pub type R = crate::R<u32, super::PIN>;
#[doc = "Reader of field `PDI`"]
pub type PDI_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Pin Data Input"]
    #[inline(always)]
    pub fn pdi(&self) -> PDI_R {
        PDI_R::new((self.bits & 0xff) as u8)
    }
}
