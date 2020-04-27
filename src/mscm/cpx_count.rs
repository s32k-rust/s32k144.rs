#[doc = "Reader of register CPxCOUNT"]
pub type R = crate::R<u32, super::CPXCOUNT>;
#[doc = "Reader of field `PCNT`"]
pub type PCNT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Processor Count"]
    #[inline(always)]
    pub fn pcnt(&self) -> PCNT_R {
        PCNT_R::new((self.bits & 0x03) as u8)
    }
}
