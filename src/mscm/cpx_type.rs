#[doc = "Reader of register CPxTYPE"]
pub type R = crate::R<u32, super::CPXTYPE>;
#[doc = "Reader of field `RYPZ`"]
pub type RYPZ_R = crate::R<u8, u8>;
#[doc = "Reader of field `PERSONALITY`"]
pub type PERSONALITY_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:7 - Processor x Revision"]
    #[inline(always)]
    pub fn rypz(&self) -> RYPZ_R {
        RYPZ_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Processor x Personality"]
    #[inline(always)]
    pub fn personality(&self) -> PERSONALITY_R {
        PERSONALITY_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
