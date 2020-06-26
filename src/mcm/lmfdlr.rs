#[doc = "Reader of register LMFDLR"]
pub type R = crate::R<u32, super::LMFDLR>;
#[doc = "Reader of field `PEFDL`"]
pub type PEFDL_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Parity or ECC Fault Data Low"]
    #[inline(always)]
    pub fn pefdl(&self) -> PEFDL_R {
        PEFDL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
