#[doc = "Reader of register WMB0_D47"]
pub type R = crate::R<u32, super::WMB0_D47>;
#[doc = "Reader of field `Data_byte_7`"]
pub type DATA_BYTE_7_R = crate::R<u8, u8>;
#[doc = "Reader of field `Data_byte_6`"]
pub type DATA_BYTE_6_R = crate::R<u8, u8>;
#[doc = "Reader of field `Data_byte_5`"]
pub type DATA_BYTE_5_R = crate::R<u8, u8>;
#[doc = "Reader of field `Data_byte_4`"]
pub type DATA_BYTE_4_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Received payload corresponding to the data byte 7 under Pretended Networking mode"]
    #[inline(always)]
    pub fn data_byte_7(&self) -> DATA_BYTE_7_R {
        DATA_BYTE_7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Received payload corresponding to the data byte 6 under Pretended Networking mode"]
    #[inline(always)]
    pub fn data_byte_6(&self) -> DATA_BYTE_6_R {
        DATA_BYTE_6_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Received payload corresponding to the data byte 5 under Pretended Networking mode"]
    #[inline(always)]
    pub fn data_byte_5(&self) -> DATA_BYTE_5_R {
        DATA_BYTE_5_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Received payload corresponding to the data byte 4 under Pretended Networking mode"]
    #[inline(always)]
    pub fn data_byte_4(&self) -> DATA_BYTE_4_R {
        DATA_BYTE_4_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
