#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::FDCRC {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct FD_TXCRCR {
    bits: u32,
}
impl FD_TXCRCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct FD_MBCRCR {
    bits: u8,
}
impl FD_MBCRCR {
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
    #[doc = "Bits 0:20 - Extended Transmitted CRC value"]
    #[inline]
    pub fn fd_txcrc(&self) -> FD_TXCRCR {
        let bits = {
            const MASK: u32 = 2097151;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        FD_TXCRCR { bits }
    }
    #[doc = "Bits 24:30 - CRC Mailbox Number for FD_TXCRC"]
    #[inline]
    pub fn fd_mbcrc(&self) -> FD_MBCRCR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        FD_MBCRCR { bits }
    }
}
