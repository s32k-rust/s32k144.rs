#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CP0CFG2 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct TMUSZR {
    bits: u8,
}
impl TMUSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TMLSZR {
    bits: u8,
}
impl TMLSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 8:15 - Tightly-coupled Memory Upper Size"]
    #[inline]
    pub fn tmusz(&self) -> TMUSZR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMUSZR { bits }
    }
    #[doc = "Bits 24:31 - Tightly-coupled Memory Lower Size"]
    #[inline]
    pub fn tmlsz(&self) -> TMLSZR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TMLSZR { bits }
    }
}
