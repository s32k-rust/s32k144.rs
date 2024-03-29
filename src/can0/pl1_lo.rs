#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PL1_LO {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R { bits: self.register.get() }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
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
#[doc = r" Proxy"]
pub struct _DATA_BYTE_3W<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_BYTE_3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATA_BYTE_2W<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_BYTE_2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATA_BYTE_1W<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_BYTE_1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATA_BYTE_0W<'a> {
    w: &'a mut W,
}
impl<'a> _DATA_BYTE_0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to the data byte 3."]
    #[inline]
    pub fn data_byte_3(&self) -> DATA_BYTE_3R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATA_BYTE_3R { bits }
    }
    #[doc = "Bits 8:15 - Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to the data byte 2."]
    #[inline]
    pub fn data_byte_2(&self) -> DATA_BYTE_2R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATA_BYTE_2R { bits }
    }
    #[doc = "Bits 16:23 - Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to the data byte 1."]
    #[inline]
    pub fn data_byte_1(&self) -> DATA_BYTE_1R {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATA_BYTE_1R { bits }
    }
    #[doc = "Bits 24:31 - Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to the data byte 0."]
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
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to the data byte 3."]
    #[inline]
    pub fn data_byte_3(&mut self) -> _DATA_BYTE_3W {
        _DATA_BYTE_3W { w: self }
    }
    #[doc = "Bits 8:15 - Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to the data byte 2."]
    #[inline]
    pub fn data_byte_2(&mut self) -> _DATA_BYTE_2W {
        _DATA_BYTE_2W { w: self }
    }
    #[doc = "Bits 16:23 - Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to the data byte 1."]
    #[inline]
    pub fn data_byte_1(&mut self) -> _DATA_BYTE_1W {
        _DATA_BYTE_1W { w: self }
    }
    #[doc = "Bits 24:31 - Payload Filter 1 low order bits for Pretended Networking payload filtering corresponding to the data byte 0."]
    #[inline]
    pub fn data_byte_0(&mut self) -> _DATA_BYTE_0W {
        _DATA_BYTE_0W { w: self }
    }
}
