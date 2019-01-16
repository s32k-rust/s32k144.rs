#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::WMB0_D03 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct DATA_BYTE_3R {
    bits: u8,
}
impl DATA_BYTE_3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATA_BYTE_2R {
    bits: u8,
}
impl DATA_BYTE_2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATA_BYTE_1R {
    bits: u8,
}
impl DATA_BYTE_1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATA_BYTE_0R {
    bits: u8,
}
impl DATA_BYTE_0R {
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
    #[doc = "Bits 0:7 - Received payload corresponding to the data byte 3 under Pretended Networking mode"]
    #[inline]
    pub fn data_byte_3(&self) -> DATA_BYTE_3R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATA_BYTE_3R { bits }
    }
    #[doc = "Bits 8:15 - Received payload corresponding to the data byte 2 under Pretended Networking mode"]
    #[inline]
    pub fn data_byte_2(&self) -> DATA_BYTE_2R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATA_BYTE_2R { bits }
    }
    #[doc = "Bits 16:23 - Received payload corresponding to the data byte 1 under Pretended Networking mode"]
    #[inline]
    pub fn data_byte_1(&self) -> DATA_BYTE_1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATA_BYTE_1R { bits }
    }
    #[doc = "Bits 24:31 - Received payload corresponding to the data byte 0 under Pretended Networking mode"]
    #[inline]
    pub fn data_byte_0(&self) -> DATA_BYTE_0R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATA_BYTE_0R { bits }
    }
}
