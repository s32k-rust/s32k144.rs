#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::CRCR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct TXCRCR {
    bits: u16,
}
impl TXCRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MBCRCR {
    bits: u8,
}
impl MBCRCR {
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
    #[doc = "Bits 0:14 - Transmitted CRC value"]
    #[inline]
    pub fn txcrc(&self) -> TXCRCR {
        let bits = {
            const MASK: u16 = 32767;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        TXCRCR { bits }
    }
    #[doc = "Bits 16:22 - CRC Mailbox"]
    #[inline]
    pub fn mbcrc(&self) -> MBCRCR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MBCRCR { bits }
    }
}
