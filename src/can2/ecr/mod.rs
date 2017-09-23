#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ECR {
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
pub struct TXERRCNTR {
    bits: u8,
}
impl TXERRCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RXERRCNTR {
    bits: u8,
}
impl RXERRCNTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TXERRCNT_FASTR {
    bits: u8,
}
impl TXERRCNT_FASTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct RXERRCNT_FASTR {
    bits: u8,
}
impl RXERRCNT_FASTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TXERRCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _TXERRCNTW<'a> {
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
pub struct _RXERRCNTW<'a> {
    w: &'a mut W,
}
impl<'a> _RXERRCNTW<'a> {
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
pub struct _TXERRCNT_FASTW<'a> {
    w: &'a mut W,
}
impl<'a> _TXERRCNT_FASTW<'a> {
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
pub struct _RXERRCNT_FASTW<'a> {
    w: &'a mut W,
}
impl<'a> _RXERRCNT_FASTW<'a> {
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
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline]
    pub fn txerrcnt(&self) -> TXERRCNTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXERRCNTR { bits }
    }
    #[doc = "Bits 8:15 - Receive Error Counter"]
    #[inline]
    pub fn rxerrcnt(&self) -> RXERRCNTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXERRCNTR { bits }
    }
    #[doc = "Bits 16:23 - Transmit Error Counter for fast bits"]
    #[inline]
    pub fn txerrcnt_fast(&self) -> TXERRCNT_FASTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TXERRCNT_FASTR { bits }
    }
    #[doc = "Bits 24:31 - Receive Error Counter for fast bits"]
    #[inline]
    pub fn rxerrcnt_fast(&self) -> RXERRCNT_FASTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXERRCNT_FASTR { bits }
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
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline]
    pub fn txerrcnt(&mut self) -> _TXERRCNTW {
        _TXERRCNTW { w: self }
    }
    #[doc = "Bits 8:15 - Receive Error Counter"]
    #[inline]
    pub fn rxerrcnt(&mut self) -> _RXERRCNTW {
        _RXERRCNTW { w: self }
    }
    #[doc = "Bits 16:23 - Transmit Error Counter for fast bits"]
    #[inline]
    pub fn txerrcnt_fast(&mut self) -> _TXERRCNT_FASTW {
        _TXERRCNT_FASTW { w: self }
    }
    #[doc = "Bits 24:31 - Receive Error Counter for fast bits"]
    #[inline]
    pub fn rxerrcnt_fast(&mut self) -> _RXERRCNT_FASTW {
        _RXERRCNT_FASTW { w: self }
    }
}
