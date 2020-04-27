#[doc = "Reader of register WMB2_D03"]
pub type R = crate::R<u32, super::WMB2_D03>;
#[doc = "Reader of field `Data_byte_3`"]
pub type DATA_BYTE_3_R = crate::R<u8, u8>;
#[doc = "Reader of field `Data_byte_2`"]
pub type DATA_BYTE_2_R = crate::R<u8, u8>;
#[doc = "Reader of field `Data_byte_1`"]
pub type DATA_BYTE_1_R = crate::R<u8, u8>;
#[doc = "Reader of field `Data_byte_0`"]
pub type DATA_BYTE_0_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Received payload corresponding to the data byte 3 under Pretended Networking mode"]
    #[inline(always)]
    pub fn data_byte_3(&self) -> DATA_BYTE_3_R {
        DATA_BYTE_3_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Received payload corresponding to the data byte 2 under Pretended Networking mode"]
    #[inline(always)]
    pub fn data_byte_2(&self) -> DATA_BYTE_2_R {
        DATA_BYTE_2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Received payload corresponding to the data byte 1 under Pretended Networking mode"]
    #[inline(always)]
    pub fn data_byte_1(&self) -> DATA_BYTE_1_R {
        DATA_BYTE_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Received payload corresponding to the data byte 0 under Pretended Networking mode"]
    #[inline(always)]
    pub fn data_byte_0(&self) -> DATA_BYTE_0_R {
        DATA_BYTE_0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
