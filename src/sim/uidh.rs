#[doc = "Reader of register UIDH"]
pub type R = crate::R<u32, super::UIDH>;
#[doc = "Reader of field `UID127_96`"]
pub type UID127_96_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Unique Identification"]
    #[inline(always)]
    pub fn uid127_96(&self) -> UID127_96_R {
        UID127_96_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
