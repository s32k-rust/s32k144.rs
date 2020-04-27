#[doc = "Reader of register CP0CFG1"]
pub type R = crate::R<u32, super::CP0CFG1>;
#[doc = "Reader of field `L2WY`"]
pub type L2WY_R = crate::R<u8, u8>;
#[doc = "Reader of field `L2SZ`"]
pub type L2SZ_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 16:23 - Level 2 Instruction Cache Ways"]
    #[inline(always)]
    pub fn l2wy(&self) -> L2WY_R {
        L2WY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Level 2 Instruction Cache Size"]
    #[inline(always)]
    pub fn l2sz(&self) -> L2SZ_R {
        L2SZ_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
