#[doc = "Reader of register PMSTAT"]
pub type R = crate::R<u32, super::PMSTAT>;
#[doc = "Reader of field `PMSTAT`"]
pub type PMSTAT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Power Mode Status"]
    #[inline(always)]
    pub fn pmstat(&self) -> PMSTAT_R {
        PMSTAT_R::new((self.bits & 0xff) as u8)
    }
}
