#[doc = "Reader of register LMFAR"]
pub type R = crate::R<u32, super::LMFAR>;
#[doc = "Reader of field `EFADD`"]
pub type EFADD_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC Fault Address"]
    #[inline(always)]
    pub fn efadd(&self) -> EFADD_R {
        EFADD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
