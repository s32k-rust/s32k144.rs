#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG1 {
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
#[doc = "Possible values of the field `ADICLK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADICLKR {
    #[doc = "Alternate clock 1 (ADC_ALTCLK1)"]
    _00,
    #[doc = "Alternate clock 2 (ADC_ALTCLK2)"]
    _01,
    #[doc = "Alternate clock 3 (ADC_ALTCLK3)"]
    _10,
    #[doc = "Alternate clock 4 (ADC_ALTCLK4)"]
    _11,
}
impl ADICLKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADICLKR::_00 => 0,
            ADICLKR::_01 => 1,
            ADICLKR::_10 => 2,
            ADICLKR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADICLKR {
        match value {
            0 => ADICLKR::_00,
            1 => ADICLKR::_01,
            2 => ADICLKR::_10,
            3 => ADICLKR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == ADICLKR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == ADICLKR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == ADICLKR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == ADICLKR::_11
    }
}
#[doc = "Possible values of the field `MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODER {
    #[doc = "8-bit conversion."]
    _00,
    #[doc = "12-bit conversion."]
    _01,
    #[doc = "10-bit conversion."]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl MODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MODER::_00 => 0,
            MODER::_01 => 1,
            MODER::_10 => 2,
            MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MODER {
        match value {
            0 => MODER::_00,
            1 => MODER::_01,
            2 => MODER::_10,
            i => MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == MODER::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == MODER::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == MODER::_10
    }
}
#[doc = "Possible values of the field `ADIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADIVR {
    #[doc = "The divide ratio is 1 and the clock rate is input clock."]
    _00,
    #[doc = "The divide ratio is 2 and the clock rate is (input clock)/2."]
    _01,
    #[doc = "The divide ratio is 4 and the clock rate is (input clock)/4."]
    _10,
    #[doc = "The divide ratio is 8 and the clock rate is (input clock)/8."]
    _11,
}
impl ADIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            ADIVR::_00 => 0,
            ADIVR::_01 => 1,
            ADIVR::_10 => 2,
            ADIVR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> ADIVR {
        match value {
            0 => ADIVR::_00,
            1 => ADIVR::_01,
            2 => ADIVR::_10,
            3 => ADIVR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == ADIVR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == ADIVR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == ADIVR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == ADIVR::_11
    }
}
#[doc = "Values that can be written to the field `ADICLK`"]
pub enum ADICLKW {
    #[doc = "Alternate clock 1 (ADC_ALTCLK1)"]
    _00,
    #[doc = "Alternate clock 2 (ADC_ALTCLK2)"]
    _01,
    #[doc = "Alternate clock 3 (ADC_ALTCLK3)"]
    _10,
    #[doc = "Alternate clock 4 (ADC_ALTCLK4)"]
    _11,
}
impl ADICLKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADICLKW::_00 => 0,
            ADICLKW::_01 => 1,
            ADICLKW::_10 => 2,
            ADICLKW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADICLKW<'a> {
    w: &'a mut W,
}
impl<'a> _ADICLKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADICLKW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Alternate clock 1 (ADC_ALTCLK1)"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADICLKW::_00)
    }
    #[doc = "Alternate clock 2 (ADC_ALTCLK2)"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADICLKW::_01)
    }
    #[doc = "Alternate clock 3 (ADC_ALTCLK3)"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADICLKW::_10)
    }
    #[doc = "Alternate clock 4 (ADC_ALTCLK4)"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADICLKW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MODE`"]
pub enum MODEW {
    #[doc = "8-bit conversion."]
    _00,
    #[doc = "12-bit conversion."]
    _01,
    #[doc = "10-bit conversion."]
    _10,
}
impl MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MODEW::_00 => 0,
            MODEW::_01 => 1,
            MODEW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8-bit conversion."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(MODEW::_00)
    }
    #[doc = "12-bit conversion."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(MODEW::_01)
    }
    #[doc = "10-bit conversion."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(MODEW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ADIV`"]
pub enum ADIVW {
    #[doc = "The divide ratio is 1 and the clock rate is input clock."]
    _00,
    #[doc = "The divide ratio is 2 and the clock rate is (input clock)/2."]
    _01,
    #[doc = "The divide ratio is 4 and the clock rate is (input clock)/4."]
    _10,
    #[doc = "The divide ratio is 8 and the clock rate is (input clock)/8."]
    _11,
}
impl ADIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADIVW::_00 => 0,
            ADIVW::_01 => 1,
            ADIVW::_10 => 2,
            ADIVW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ADIVW<'a> {
    w: &'a mut W,
}
impl<'a> _ADIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ADIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The divide ratio is 1 and the clock rate is input clock."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADIVW::_00)
    }
    #[doc = "The divide ratio is 2 and the clock rate is (input clock)/2."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADIVW::_01)
    }
    #[doc = "The divide ratio is 4 and the clock rate is (input clock)/4."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADIVW::_10)
    }
    #[doc = "The divide ratio is 8 and the clock rate is (input clock)/8."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADIVW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CLRLTRGW<'a> {
    w: &'a mut W,
}
impl<'a> _CLRLTRGW<'a> {
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
        const OFFSET: u8 = 8;
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
    #[doc = "Bits 0:1 - Input Clock Select"]
    #[inline]
    pub fn adiclk(&self) -> ADICLKR {
        ADICLKR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Conversion mode selection"]
    #[inline]
    pub fn mode(&self) -> MODER {
        MODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 5:6 - Clock Divide Select"]
    #[inline]
    pub fn adiv(&self) -> ADIVR {
        ADIVR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bits 0:1 - Input Clock Select"]
    #[inline]
    pub fn adiclk(&mut self) -> _ADICLKW {
        _ADICLKW { w: self }
    }
    #[doc = "Bits 2:3 - Conversion mode selection"]
    #[inline]
    pub fn mode(&mut self) -> _MODEW {
        _MODEW { w: self }
    }
    #[doc = "Bits 5:6 - Clock Divide Select"]
    #[inline]
    pub fn adiv(&mut self) -> _ADIVW {
        _ADIVW { w: self }
    }
    #[doc = "Bit 8 - Clear Latch Trigger in Trigger Handler Block"]
    #[inline]
    pub fn clrltrg(&mut self) -> _CLRLTRGW {
        _CLRLTRGW { w: self }
    }
}
