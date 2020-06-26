#[doc = "Reader of register UIDML"]
pub type R = crate::R<u32, super::UIDML>;
#[doc = "Reader of field `UID63_32`"]
pub type UID63_32_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Unique Identification"]
    #[inline(always)]
    pub fn uid63_32(&self) -> UID63_32_R {
        UID63_32_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
