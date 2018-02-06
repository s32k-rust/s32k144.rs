#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PSR {
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
#[doc = "Possible values of the field `PCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSR {
    #[doc = "Prescaler/glitch filter clock 0 selected."]
    _00,
    #[doc = "Prescaler/glitch filter clock 1 selected."]
    _01,
    #[doc = "Prescaler/glitch filter clock 2 selected."]
    _10,
    #[doc = "Prescaler/glitch filter clock 3 selected."]
    _11,
}
impl PCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PCSR::_00 => 0,
            PCSR::_01 => 1,
            PCSR::_10 => 2,
            PCSR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PCSR {
        match value {
            0 => PCSR::_00,
            1 => PCSR::_01,
            2 => PCSR::_10,
            3 => PCSR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == PCSR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == PCSR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == PCSR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == PCSR::_11
    }
}
#[doc = "Possible values of the field `PBYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PBYPR {
    #[doc = "Prescaler/glitch filter is enabled."]
    _0,
    #[doc = "Prescaler/glitch filter is bypassed."]
    _1,
}
impl PBYPR {
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
            PBYPR::_0 => false,
            PBYPR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PBYPR {
        match value {
            false => PBYPR::_0,
            true => PBYPR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PBYPR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PBYPR::_1
    }
}
#[doc = "Possible values of the field `PRESCALE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALER {
    #[doc = "Prescaler divides the prescaler clock by 2; glitch filter does not support this configuration."]
    _0000,
    #[doc = "Prescaler divides the prescaler clock by 4; glitch filter recognizes change on input pin after 2 rising clock edges."]
    _0001,
    #[doc = "Prescaler divides the prescaler clock by 8; glitch filter recognizes change on input pin after 4 rising clock edges."]
    _0010,
    #[doc = "Prescaler divides the prescaler clock by 16; glitch filter recognizes change on input pin after 8 rising clock edges."]
    _0011,
    #[doc = "Prescaler divides the prescaler clock by 32; glitch filter recognizes change on input pin after 16 rising clock edges."]
    _0100,
    #[doc = "Prescaler divides the prescaler clock by 64; glitch filter recognizes change on input pin after 32 rising clock edges."]
    _0101,
    #[doc = "Prescaler divides the prescaler clock by 128; glitch filter recognizes change on input pin after 64 rising clock edges."]
    _0110,
    #[doc = "Prescaler divides the prescaler clock by 256; glitch filter recognizes change on input pin after 128 rising clock edges."]
    _0111,
    #[doc = "Prescaler divides the prescaler clock by 512; glitch filter recognizes change on input pin after 256 rising clock edges."]
    _1000,
    #[doc = "Prescaler divides the prescaler clock by 1024; glitch filter recognizes change on input pin after 512 rising clock edges."]
    _1001,
    #[doc = "Prescaler divides the prescaler clock by 2048; glitch filter recognizes change on input pin after 1024 rising clock edges."]
    _1010,
    #[doc = "Prescaler divides the prescaler clock by 4096; glitch filter recognizes change on input pin after 2048 rising clock edges."]
    _1011,
    #[doc = "Prescaler divides the prescaler clock by 8192; glitch filter recognizes change on input pin after 4096 rising clock edges."]
    _1100,
    #[doc = "Prescaler divides the prescaler clock by 16,384; glitch filter recognizes change on input pin after 8192 rising clock edges."]
    _1101,
    #[doc = "Prescaler divides the prescaler clock by 32,768; glitch filter recognizes change on input pin after 16,384 rising clock edges."]
    _1110,
    #[doc = "Prescaler divides the prescaler clock by 65,536; glitch filter recognizes change on input pin after 32,768 rising clock edges."]
    _1111,
}
impl PRESCALER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCALER::_0000 => 0,
            PRESCALER::_0001 => 1,
            PRESCALER::_0010 => 2,
            PRESCALER::_0011 => 3,
            PRESCALER::_0100 => 4,
            PRESCALER::_0101 => 5,
            PRESCALER::_0110 => 6,
            PRESCALER::_0111 => 7,
            PRESCALER::_1000 => 8,
            PRESCALER::_1001 => 9,
            PRESCALER::_1010 => 10,
            PRESCALER::_1011 => 11,
            PRESCALER::_1100 => 12,
            PRESCALER::_1101 => 13,
            PRESCALER::_1110 => 14,
            PRESCALER::_1111 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCALER {
        match value {
            0 => PRESCALER::_0000,
            1 => PRESCALER::_0001,
            2 => PRESCALER::_0010,
            3 => PRESCALER::_0011,
            4 => PRESCALER::_0100,
            5 => PRESCALER::_0101,
            6 => PRESCALER::_0110,
            7 => PRESCALER::_0111,
            8 => PRESCALER::_1000,
            9 => PRESCALER::_1001,
            10 => PRESCALER::_1010,
            11 => PRESCALER::_1011,
            12 => PRESCALER::_1100,
            13 => PRESCALER::_1101,
            14 => PRESCALER::_1110,
            15 => PRESCALER::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == PRESCALER::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == PRESCALER::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == PRESCALER::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == PRESCALER::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == PRESCALER::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == PRESCALER::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == PRESCALER::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == PRESCALER::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == PRESCALER::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == PRESCALER::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == PRESCALER::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == PRESCALER::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == PRESCALER::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == PRESCALER::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == PRESCALER::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == PRESCALER::_1111
    }
}
#[doc = "Values that can be written to the field `PCS`"]
pub enum PCSW {
    #[doc = "Prescaler/glitch filter clock 0 selected."]
    _00,
    #[doc = "Prescaler/glitch filter clock 1 selected."]
    _01,
    #[doc = "Prescaler/glitch filter clock 2 selected."]
    _10,
    #[doc = "Prescaler/glitch filter clock 3 selected."]
    _11,
}
impl PCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PCSW::_00 => 0,
            PCSW::_01 => 1,
            PCSW::_10 => 2,
            PCSW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PCSW<'a> {
    w: &'a mut W,
}
impl<'a> _PCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PCSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Prescaler/glitch filter clock 0 selected."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(PCSW::_00)
    }
    #[doc = "Prescaler/glitch filter clock 1 selected."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(PCSW::_01)
    }
    #[doc = "Prescaler/glitch filter clock 2 selected."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(PCSW::_10)
    }
    #[doc = "Prescaler/glitch filter clock 3 selected."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(PCSW::_11)
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
#[doc = "Values that can be written to the field `PBYP`"]
pub enum PBYPW {
    #[doc = "Prescaler/glitch filter is enabled."]
    _0,
    #[doc = "Prescaler/glitch filter is bypassed."]
    _1,
}
impl PBYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PBYPW::_0 => false,
            PBYPW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PBYPW<'a> {
    w: &'a mut W,
}
impl<'a> _PBYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PBYPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Prescaler/glitch filter is enabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PBYPW::_0)
    }
    #[doc = "Prescaler/glitch filter is bypassed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PBYPW::_1)
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
#[doc = "Values that can be written to the field `PRESCALE`"]
pub enum PRESCALEW {
    #[doc = "Prescaler divides the prescaler clock by 2; glitch filter does not support this configuration."]
    _0000,
    #[doc = "Prescaler divides the prescaler clock by 4; glitch filter recognizes change on input pin after 2 rising clock edges."]
    _0001,
    #[doc = "Prescaler divides the prescaler clock by 8; glitch filter recognizes change on input pin after 4 rising clock edges."]
    _0010,
    #[doc = "Prescaler divides the prescaler clock by 16; glitch filter recognizes change on input pin after 8 rising clock edges."]
    _0011,
    #[doc = "Prescaler divides the prescaler clock by 32; glitch filter recognizes change on input pin after 16 rising clock edges."]
    _0100,
    #[doc = "Prescaler divides the prescaler clock by 64; glitch filter recognizes change on input pin after 32 rising clock edges."]
    _0101,
    #[doc = "Prescaler divides the prescaler clock by 128; glitch filter recognizes change on input pin after 64 rising clock edges."]
    _0110,
    #[doc = "Prescaler divides the prescaler clock by 256; glitch filter recognizes change on input pin after 128 rising clock edges."]
    _0111,
    #[doc = "Prescaler divides the prescaler clock by 512; glitch filter recognizes change on input pin after 256 rising clock edges."]
    _1000,
    #[doc = "Prescaler divides the prescaler clock by 1024; glitch filter recognizes change on input pin after 512 rising clock edges."]
    _1001,
    #[doc = "Prescaler divides the prescaler clock by 2048; glitch filter recognizes change on input pin after 1024 rising clock edges."]
    _1010,
    #[doc = "Prescaler divides the prescaler clock by 4096; glitch filter recognizes change on input pin after 2048 rising clock edges."]
    _1011,
    #[doc = "Prescaler divides the prescaler clock by 8192; glitch filter recognizes change on input pin after 4096 rising clock edges."]
    _1100,
    #[doc = "Prescaler divides the prescaler clock by 16,384; glitch filter recognizes change on input pin after 8192 rising clock edges."]
    _1101,
    #[doc = "Prescaler divides the prescaler clock by 32,768; glitch filter recognizes change on input pin after 16,384 rising clock edges."]
    _1110,
    #[doc = "Prescaler divides the prescaler clock by 65,536; glitch filter recognizes change on input pin after 32,768 rising clock edges."]
    _1111,
}
impl PRESCALEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCALEW::_0000 => 0,
            PRESCALEW::_0001 => 1,
            PRESCALEW::_0010 => 2,
            PRESCALEW::_0011 => 3,
            PRESCALEW::_0100 => 4,
            PRESCALEW::_0101 => 5,
            PRESCALEW::_0110 => 6,
            PRESCALEW::_0111 => 7,
            PRESCALEW::_1000 => 8,
            PRESCALEW::_1001 => 9,
            PRESCALEW::_1010 => 10,
            PRESCALEW::_1011 => 11,
            PRESCALEW::_1100 => 12,
            PRESCALEW::_1101 => 13,
            PRESCALEW::_1110 => 14,
            PRESCALEW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCALEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Prescaler divides the prescaler clock by 2; glitch filter does not support this configuration."]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(PRESCALEW::_0000)
    }
    #[doc = "Prescaler divides the prescaler clock by 4; glitch filter recognizes change on input pin after 2 rising clock edges."]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(PRESCALEW::_0001)
    }
    #[doc = "Prescaler divides the prescaler clock by 8; glitch filter recognizes change on input pin after 4 rising clock edges."]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(PRESCALEW::_0010)
    }
    #[doc = "Prescaler divides the prescaler clock by 16; glitch filter recognizes change on input pin after 8 rising clock edges."]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(PRESCALEW::_0011)
    }
    #[doc = "Prescaler divides the prescaler clock by 32; glitch filter recognizes change on input pin after 16 rising clock edges."]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(PRESCALEW::_0100)
    }
    #[doc = "Prescaler divides the prescaler clock by 64; glitch filter recognizes change on input pin after 32 rising clock edges."]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(PRESCALEW::_0101)
    }
    #[doc = "Prescaler divides the prescaler clock by 128; glitch filter recognizes change on input pin after 64 rising clock edges."]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(PRESCALEW::_0110)
    }
    #[doc = "Prescaler divides the prescaler clock by 256; glitch filter recognizes change on input pin after 128 rising clock edges."]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(PRESCALEW::_0111)
    }
    #[doc = "Prescaler divides the prescaler clock by 512; glitch filter recognizes change on input pin after 256 rising clock edges."]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(PRESCALEW::_1000)
    }
    #[doc = "Prescaler divides the prescaler clock by 1024; glitch filter recognizes change on input pin after 512 rising clock edges."]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(PRESCALEW::_1001)
    }
    #[doc = "Prescaler divides the prescaler clock by 2048; glitch filter recognizes change on input pin after 1024 rising clock edges."]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(PRESCALEW::_1010)
    }
    #[doc = "Prescaler divides the prescaler clock by 4096; glitch filter recognizes change on input pin after 2048 rising clock edges."]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(PRESCALEW::_1011)
    }
    #[doc = "Prescaler divides the prescaler clock by 8192; glitch filter recognizes change on input pin after 4096 rising clock edges."]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(PRESCALEW::_1100)
    }
    #[doc = "Prescaler divides the prescaler clock by 16,384; glitch filter recognizes change on input pin after 8192 rising clock edges."]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(PRESCALEW::_1101)
    }
    #[doc = "Prescaler divides the prescaler clock by 32,768; glitch filter recognizes change on input pin after 16,384 rising clock edges."]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(PRESCALEW::_1110)
    }
    #[doc = "Prescaler divides the prescaler clock by 65,536; glitch filter recognizes change on input pin after 32,768 rising clock edges."]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(PRESCALEW::_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:1 - Prescaler Clock Select"]
    #[inline]
    pub fn pcs(&self) -> PCSR {
        PCSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 2 - Prescaler Bypass"]
    #[inline]
    pub fn pbyp(&self) -> PBYPR {
        PBYPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 3:6 - Prescale Value"]
    #[inline]
    pub fn prescale(&self) -> PRESCALER {
        PRESCALER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 3;
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
    #[doc = "Bits 0:1 - Prescaler Clock Select"]
    #[inline]
    pub fn pcs(&mut self) -> _PCSW {
        _PCSW { w: self }
    }
    #[doc = "Bit 2 - Prescaler Bypass"]
    #[inline]
    pub fn pbyp(&mut self) -> _PBYPW {
        _PBYPW { w: self }
    }
    #[doc = "Bits 3:6 - Prescale Value"]
    #[inline]
    pub fn prescale(&mut self) -> _PRESCALEW {
        _PRESCALEW { w: self }
    }
}
