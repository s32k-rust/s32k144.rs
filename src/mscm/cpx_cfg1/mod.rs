#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CPXCFG1 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct L2WYR {
    bits: u8,
}
impl L2WYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct L2SZR {
    bits: u8,
}
impl L2SZR {
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
    #[doc = "Bits 16:23 - Level 2 Instruction Cache Ways"]
    #[inline]
    pub fn l2wy(&self) -> L2WYR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        L2WYR { bits }
    }
    #[doc = "Bits 24:31 - Level 2 Instruction Cache Size"]
    #[inline]
    pub fn l2sz(&self) -> L2SZR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        L2SZR { bits }
    }
}
