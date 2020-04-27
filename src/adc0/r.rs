#[doc = "Reader of register R%s"]
pub type R = crate::R<u32, super::R>;
#[doc = "Reader of field `D`"]
pub type D_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Data result"]
    #[inline(always)]
    pub fn d(&self) -> D_R {
        D_R::new((self.bits & 0x0fff) as u16)
    }
}
