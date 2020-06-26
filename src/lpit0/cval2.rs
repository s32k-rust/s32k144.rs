#[doc = "Reader of register CVAL2"]
pub type R = crate::R<u32, super::CVAL2>;
#[doc = "Reader of field `TMR_CUR_VAL`"]
pub type TMR_CUR_VAL_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current Timer Value"]
    #[inline(always)]
    pub fn tmr_cur_val(&self) -> TMR_CUR_VAL_R {
        TMR_CUR_VAL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
