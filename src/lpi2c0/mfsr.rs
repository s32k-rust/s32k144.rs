#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::MFSR {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct TXCOUNTR {
    bits: u8,
}
impl TXCOUNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RXCOUNTR {
    bits: u8,
}
impl RXCOUNTR {
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
    #[doc = "Bits 0:2 - Transmit FIFO Count"]
    #[inline]
    pub fn txcount(&self) -> TXCOUNTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXCOUNTR { bits }
    }
    #[doc = "Bits 16:18 - Receive FIFO Count"]
    #[inline]
    pub fn rxcount(&self) -> RXCOUNTR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXCOUNTR { bits }
    }
}
