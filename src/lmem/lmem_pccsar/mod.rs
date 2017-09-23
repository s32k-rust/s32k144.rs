#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LMEM_PCCSAR {
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
#[doc = "Possible values of the field `LGO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LGOR {
    #[doc = "Write: no effect. Read: no line command active."]
    _0,
    #[doc = "Write: initiate line command indicated by bits CLCR[27:24]. Read: line command active."]
    _1,
}
impl LGOR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            LGOR::_0 => false,
            LGOR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LGOR {
        match value {
            false => LGOR::_0,
            true => LGOR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LGOR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LGOR::_1
    }
}
#[doc = r" Value of the field"]
pub struct PHYADDRR {
    bits: u32,
}
impl PHYADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `LGO`"]
pub enum LGOW {
    #[doc = "Write: no effect. Read: no line command active."]
    _0,
    #[doc = "Write: initiate line command indicated by bits CLCR[27:24]. Read: line command active."]
    _1,
}
impl LGOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LGOW::_0 => false,
            LGOW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LGOW<'a> {
    w: &'a mut W,
}
impl<'a> _LGOW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LGOW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write: no effect. Read: no line command active."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LGOW::_0)
    }
    #[doc = "Write: initiate line command indicated by bits CLCR[27:24]. Read: line command active."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LGOW::_1)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PHYADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _PHYADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        const MASK: u32 = 1073741823;
        const OFFSET: u8 = 2;
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
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline]
    pub fn lgo(&self) -> LGOR {
        LGOR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:31 - Physical Address"]
    #[inline]
    pub fn phyaddr(&self) -> PHYADDRR {
        let bits = {
            const MASK: u32 = 1073741823;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u32
        };
        PHYADDRR { bits }
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
    #[doc = "Bit 0 - Initiate Cache Line Command"]
    #[inline]
    pub fn lgo(&mut self) -> _LGOW {
        _LGOW { w: self }
    }
    #[doc = "Bits 2:31 - Physical Address"]
    #[inline]
    pub fn phyaddr(&mut self) -> _PHYADDRW {
        _PHYADDRW { w: self }
    }
}
