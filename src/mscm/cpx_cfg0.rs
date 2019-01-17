#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CPXCFG0 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct DCWYR {
    bits: u8,
}
impl DCWYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DCSZR {
    bits: u8,
}
impl DCSZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ICWYR {
    bits: u8,
}
impl ICWYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ICSZR {
    bits: u8,
}
impl ICSZR {
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
    #[doc = "Bits 0:7 - Level 1 Data Cache Ways"]
    #[inline]
    pub fn dcwy(&self) -> DCWYR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCWYR { bits }
    }
    #[doc = "Bits 8:15 - Level 1 Data Cache Size"]
    #[inline]
    pub fn dcsz(&self) -> DCSZR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DCSZR { bits }
    }
    #[doc = "Bits 16:23 - Level 1 Instruction Cache Ways"]
    #[inline]
    pub fn icwy(&self) -> ICWYR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ICWYR { bits }
    }
    #[doc = "Bits 24:31 - Level 1 Instruction Cache Size"]
    #[inline]
    pub fn icsz(&self) -> ICSZR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ICSZR { bits }
    }
}
