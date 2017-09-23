#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::WMB0_ID {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct IDR {
    bits: u32,
}
impl IDR {
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
    #[doc = "Bits 0:28 - Received ID under Pretended Networking mode"]
    #[inline]
    pub fn id(&self) -> IDR {
        let bits = {
            const MASK: u32 = 536870911;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        IDR { bits }
    }
}
