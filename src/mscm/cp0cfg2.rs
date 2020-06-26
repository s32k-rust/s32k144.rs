#[doc = "Reader of register CP0CFG2"]
pub type R = crate::R<u32, super::CP0CFG2>;
#[doc = "Reader of field `TMUSZ`"]
pub type TMUSZ_R = crate::R<u8, u8>;
#[doc = "Reader of field `TMLSZ`"]
pub type TMLSZ_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 8:15 - Tightly-coupled Memory Upper Size"]
    #[inline(always)]
    pub fn tmusz(&self) -> TMUSZ_R {
        TMUSZ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Tightly-coupled Memory Lower Size"]
    #[inline(always)]
    pub fn tmlsz(&self) -> TMLSZ_R {
        TMLSZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
