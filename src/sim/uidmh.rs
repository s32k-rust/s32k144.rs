#[doc = "Reader of register UIDMH"]
pub type R = crate::R<u32, super::UIDMH>;
#[doc = "Reader of field `UID95_64`"]
pub type UID95_64_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Unique Identification"]
    #[inline(always)]
    pub fn uid95_64(&self) -> UID95_64_R {
        UID95_64_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
