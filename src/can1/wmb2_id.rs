#[doc = "Reader of register WMB2_ID"]
pub type R = crate::R<u32, super::WMB2_ID>;
#[doc = "Reader of field `ID`"]
pub type ID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:28 - Received ID under Pretended Networking mode"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x1fff_ffff) as u32)
    }
}
