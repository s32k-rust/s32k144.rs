#[doc = "Reader of register UIDL"]
pub type R = crate::R<u32, super::UIDL>;
#[doc = "Reader of field `UID31_0`"]
pub type UID31_0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Unique Identification"]
    #[inline(always)]
    pub fn uid31_0(&self) -> UID31_0_R {
        UID31_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
