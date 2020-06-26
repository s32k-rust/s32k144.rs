#[doc = "Reader of register LMFDHR"]
pub type R = crate::R<u32, super::LMFDHR>;
#[doc = "Reader of field `PEFDH`"]
pub type PEFDH_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Parity or ECC Fault Data High"]
    #[inline(always)]
    pub fn pefdh(&self) -> PEFDH_R {
        PEFDH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
