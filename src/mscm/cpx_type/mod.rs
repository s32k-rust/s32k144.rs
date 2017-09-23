#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CPXTYPE {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r" Value of the field"]
pub struct RYPZR {
    bits: u8,
}
impl RYPZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PERSONALITYR {
    bits: u32,
}
impl PERSONALITYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Processor x Revision"]
    #[inline]
    pub fn rypz(&self) -> RYPZR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RYPZR { bits }
    }
    #[doc = "Bits 8:31 - Processor x Personality"]
    #[inline]
    pub fn personality(&self) -> PERSONALITYR {
        let bits = {
            const MASK: u32 = 16777215;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        PERSONALITYR { bits }
    }
}
