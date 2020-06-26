#[doc = "Reader of register PARAM"]
pub type R = crate::R<u32, super::PARAM>;
#[doc = "Reader of field `CHANNEL`"]
pub type CHANNEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `EXT_TRIG`"]
pub type EXT_TRIG_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of Timer Channels"]
    #[inline(always)]
    pub fn channel(&self) -> CHANNEL_R {
        CHANNEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of External Trigger Inputs"]
    #[inline(always)]
    pub fn ext_trig(&self) -> EXT_TRIG_R {
        EXT_TRIG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
