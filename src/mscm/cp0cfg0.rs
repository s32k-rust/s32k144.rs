#[doc = "Reader of register CP0CFG0"]
pub type R = crate::R<u32, super::CP0CFG0>;
#[doc = "Reader of field `DCWY`"]
pub type DCWY_R = crate::R<u8, u8>;
#[doc = "Reader of field `DCSZ`"]
pub type DCSZ_R = crate::R<u8, u8>;
#[doc = "Reader of field `ICWY`"]
pub type ICWY_R = crate::R<u8, u8>;
#[doc = "Reader of field `ICSZ`"]
pub type ICSZ_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Level 1 Data Cache Ways"]
    #[inline(always)]
    pub fn dcwy(&self) -> DCWY_R {
        DCWY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Level 1 Data Cache Size"]
    #[inline(always)]
    pub fn dcsz(&self) -> DCSZ_R {
        DCSZ_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Level 1 Instruction Cache Ways"]
    #[inline(always)]
    pub fn icwy(&self) -> ICWY_R {
        ICWY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Level 1 Instruction Cache Size"]
    #[inline(always)]
    pub fn icsz(&self) -> ICSZ_R {
        ICSZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
