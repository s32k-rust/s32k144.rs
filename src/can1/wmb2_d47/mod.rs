#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::WMB2_D47 {
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
}
#[doc = r" Value of the field"]
pub struct DATA_BYTE_7R {
    bits: u8,
}
impl DATA_BYTE_7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATA_BYTE_6R {
    bits: u8,
}
impl DATA_BYTE_6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATA_BYTE_5R {
    bits: u8,
}
impl DATA_BYTE_5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATA_BYTE_4R {
    bits: u8,
}
impl DATA_BYTE_4R {
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
    #[doc = "Bits 0:7 - Received payload corresponding to the data byte 7 under Pretended Networking mode"]
    #[inline]
    pub fn data_byte_7(&self) -> DATA_BYTE_7R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATA_BYTE_7R { bits }
    }
    #[doc = "Bits 8:15 - Received payload corresponding to the data byte 6 under Pretended Networking mode"]
    #[inline]
    pub fn data_byte_6(&self) -> DATA_BYTE_6R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATA_BYTE_6R { bits }
    }
    #[doc = "Bits 16:23 - Received payload corresponding to the data byte 5 under Pretended Networking mode"]
    #[inline]
    pub fn data_byte_5(&self) -> DATA_BYTE_5R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATA_BYTE_5R { bits }
    }
    #[doc = "Bits 24:31 - Received payload corresponding to the data byte 4 under Pretended Networking mode"]
    #[inline]
    pub fn data_byte_4(&self) -> DATA_BYTE_4R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATA_BYTE_4R { bits }
    }
}
