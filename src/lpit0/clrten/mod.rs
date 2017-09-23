#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CLRTEN {
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
        R {
            bits: self.register.get(),
        }
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
#[doc = "Values that can be written to the field `CLR_T_EN_0`"]
pub enum CLR_T_EN_0W {
    #[doc = "No action"] _0,
    #[doc = "Clear T_EN bit for Timer Channel 0"] _1,
}
impl CLR_T_EN_0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR_T_EN_0W::_0 => false,
            CLR_T_EN_0W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR_T_EN_0W<'a> {
    w: &'a mut W,
}
impl<'a> _CLR_T_EN_0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR_T_EN_0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_T_EN_0W::_0)
    }
    #[doc = "Clear T_EN bit for Timer Channel 0"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_T_EN_0W::_1)
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
#[doc = "Values that can be written to the field `CLR_T_EN_1`"]
pub enum CLR_T_EN_1W {
    #[doc = "No Action"] _0,
    #[doc = "Clear T_EN bit for Timer Channel 1"] _1,
}
impl CLR_T_EN_1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR_T_EN_1W::_0 => false,
            CLR_T_EN_1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR_T_EN_1W<'a> {
    w: &'a mut W,
}
impl<'a> _CLR_T_EN_1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR_T_EN_1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Action"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_T_EN_1W::_0)
    }
    #[doc = "Clear T_EN bit for Timer Channel 1"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_T_EN_1W::_1)
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
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLR_T_EN_2`"]
pub enum CLR_T_EN_2W {
    #[doc = "No Action"] _0,
    #[doc = "Clear T_EN bit for Timer Channel 2"] _1,
}
impl CLR_T_EN_2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR_T_EN_2W::_0 => false,
            CLR_T_EN_2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR_T_EN_2W<'a> {
    w: &'a mut W,
}
impl<'a> _CLR_T_EN_2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR_T_EN_2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Action"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_T_EN_2W::_0)
    }
    #[doc = "Clear T_EN bit for Timer Channel 2"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_T_EN_2W::_1)
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
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CLR_T_EN_3`"]
pub enum CLR_T_EN_3W {
    #[doc = "No Action"] _0,
    #[doc = "Clear T_EN bit for Timer Channel 3"] _1,
}
impl CLR_T_EN_3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CLR_T_EN_3W::_0 => false,
            CLR_T_EN_3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CLR_T_EN_3W<'a> {
    w: &'a mut W,
}
impl<'a> _CLR_T_EN_3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CLR_T_EN_3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Action"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLR_T_EN_3W::_0)
    }
    #[doc = "Clear T_EN bit for Timer Channel 3"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLR_T_EN_3W::_1)
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
        const OFFSET: u8 = 3;
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
    #[doc = "Bit 0 - Clear Timer 0 Enable"]
    #[inline]
    pub fn clr_t_en_0(&mut self) -> _CLR_T_EN_0W {
        _CLR_T_EN_0W { w: self }
    }
    #[doc = "Bit 1 - Clear Timer 1 Enable"]
    #[inline]
    pub fn clr_t_en_1(&mut self) -> _CLR_T_EN_1W {
        _CLR_T_EN_1W { w: self }
    }
    #[doc = "Bit 2 - Clear Timer 2 Enable"]
    #[inline]
    pub fn clr_t_en_2(&mut self) -> _CLR_T_EN_2W {
        _CLR_T_EN_2W { w: self }
    }
    #[doc = "Bit 3 - Clear Timer 3 Enable"]
    #[inline]
    pub fn clr_t_en_3(&mut self) -> _CLR_T_EN_3W {
        _CLR_T_EN_3W { w: self }
    }
}
