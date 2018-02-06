#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::C1 {
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
#[doc = r" Value of the field"]
pub struct VOSELR {
    bits: u8,
}
impl VOSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Possible values of the field `MSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSELR {
    #[doc = "IN0"]
    _000,
    #[doc = "IN1"]
    _001,
    #[doc = "IN2"]
    _010,
    #[doc = "IN3"]
    _011,
    #[doc = "IN4"]
    _100,
    #[doc = "IN5"]
    _101,
    #[doc = "IN6"]
    _110,
    #[doc = "IN7"]
    _111,
}
impl MSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MSELR::_000 => 0,
            MSELR::_001 => 1,
            MSELR::_010 => 2,
            MSELR::_011 => 3,
            MSELR::_100 => 4,
            MSELR::_101 => 5,
            MSELR::_110 => 6,
            MSELR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MSELR {
        match value {
            0 => MSELR::_000,
            1 => MSELR::_001,
            2 => MSELR::_010,
            3 => MSELR::_011,
            4 => MSELR::_100,
            5 => MSELR::_101,
            6 => MSELR::_110,
            7 => MSELR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == MSELR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == MSELR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == MSELR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == MSELR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == MSELR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == MSELR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == MSELR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == MSELR::_111
    }
}
#[doc = "Possible values of the field `PSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSELR {
    #[doc = "IN0"]
    _000,
    #[doc = "IN1"]
    _001,
    #[doc = "IN2"]
    _010,
    #[doc = "IN3"]
    _011,
    #[doc = "IN4"]
    _100,
    #[doc = "IN5"]
    _101,
    #[doc = "IN6"]
    _110,
    #[doc = "IN7"]
    _111,
}
impl PSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PSELR::_000 => 0,
            PSELR::_001 => 1,
            PSELR::_010 => 2,
            PSELR::_011 => 3,
            PSELR::_100 => 4,
            PSELR::_101 => 5,
            PSELR::_110 => 6,
            PSELR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PSELR {
        match value {
            0 => PSELR::_000,
            1 => PSELR::_001,
            2 => PSELR::_010,
            3 => PSELR::_011,
            4 => PSELR::_100,
            5 => PSELR::_101,
            6 => PSELR::_110,
            7 => PSELR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == PSELR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == PSELR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == PSELR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == PSELR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == PSELR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == PSELR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == PSELR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == PSELR::_111
    }
}
#[doc = "Possible values of the field `VRSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VRSELR {
    #[doc = "Vin1 is selected as resistor ladder network supply reference Vin."]
    _0,
    #[doc = "Vin2 is selected as resistor ladder network supply reference Vin."]
    _1,
}
impl VRSELR {
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
            VRSELR::_0 => false,
            VRSELR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> VRSELR {
        match value {
            false => VRSELR::_0,
            true => VRSELR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == VRSELR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == VRSELR::_1
    }
}
#[doc = "Possible values of the field `DACEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DACENR {
    #[doc = "DAC is disabled."]
    _0,
    #[doc = "DAC is enabled."]
    _1,
}
impl DACENR {
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
            DACENR::_0 => false,
            DACENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DACENR {
        match value {
            false => DACENR::_0,
            true => DACENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DACENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DACENR::_1
    }
}
#[doc = r" Value of the field"]
pub struct CHN0R {
    bits: bool,
}
impl CHN0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CHN1R {
    bits: bool,
}
impl CHN1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CHN2R {
    bits: bool,
}
impl CHN2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CHN3R {
    bits: bool,
}
impl CHN3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CHN4R {
    bits: bool,
}
impl CHN4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CHN5R {
    bits: bool,
}
impl CHN5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CHN6R {
    bits: bool,
}
impl CHN6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = r" Value of the field"]
pub struct CHN7R {
    bits: bool,
}
impl CHN7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
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
}
#[doc = "Possible values of the field `INNSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INNSELR {
    #[doc = "IN0, from the 8-bit DAC output"]
    _00,
    #[doc = "IN1, from the analog 8-1 mux"]
    _01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INNSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INNSELR::_00 => 0,
            INNSELR::_01 => 1,
            INNSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INNSELR {
        match value {
            0 => INNSELR::_00,
            1 => INNSELR::_01,
            i => INNSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == INNSELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == INNSELR::_01
    }
}
#[doc = "Possible values of the field `INPSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPSELR {
    #[doc = "IN0, from the 8-bit DAC output"]
    _00,
    #[doc = "IN1, from the analog 8-1 mux"]
    _01,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INPSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPSELR::_00 => 0,
            INPSELR::_01 => 1,
            INPSELR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPSELR {
        match value {
            0 => INPSELR::_00,
            1 => INPSELR::_01,
            i => INPSELR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == INPSELR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == INPSELR::_01
    }
}
#[doc = r" Proxy"]
pub struct _VOSELW<'a> {
    w: &'a mut W,
}
impl<'a> _VOSELW<'a> {
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
#[doc = "Values that can be written to the field `MSEL`"]
pub enum MSELW {
    #[doc = "IN0"]
    _000,
    #[doc = "IN1"]
    _001,
    #[doc = "IN2"]
    _010,
    #[doc = "IN3"]
    _011,
    #[doc = "IN4"]
    _100,
    #[doc = "IN5"]
    _101,
    #[doc = "IN6"]
    _110,
    #[doc = "IN7"]
    _111,
}
impl MSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MSELW::_000 => 0,
            MSELW::_001 => 1,
            MSELW::_010 => 2,
            MSELW::_011 => 3,
            MSELW::_100 => 4,
            MSELW::_101 => 5,
            MSELW::_110 => 6,
            MSELW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "IN0"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(MSELW::_000)
    }
    #[doc = "IN1"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(MSELW::_001)
    }
    #[doc = "IN2"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(MSELW::_010)
    }
    #[doc = "IN3"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(MSELW::_011)
    }
    #[doc = "IN4"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(MSELW::_100)
    }
    #[doc = "IN5"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(MSELW::_101)
    }
    #[doc = "IN6"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(MSELW::_110)
    }
    #[doc = "IN7"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(MSELW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PSEL`"]
pub enum PSELW {
    #[doc = "IN0"]
    _000,
    #[doc = "IN1"]
    _001,
    #[doc = "IN2"]
    _010,
    #[doc = "IN3"]
    _011,
    #[doc = "IN4"]
    _100,
    #[doc = "IN5"]
    _101,
    #[doc = "IN6"]
    _110,
    #[doc = "IN7"]
    _111,
}
impl PSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PSELW::_000 => 0,
            PSELW::_001 => 1,
            PSELW::_010 => 2,
            PSELW::_011 => 3,
            PSELW::_100 => 4,
            PSELW::_101 => 5,
            PSELW::_110 => 6,
            PSELW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PSELW<'a> {
    w: &'a mut W,
}
impl<'a> _PSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "IN0"]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(PSELW::_000)
    }
    #[doc = "IN1"]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(PSELW::_001)
    }
    #[doc = "IN2"]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(PSELW::_010)
    }
    #[doc = "IN3"]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(PSELW::_011)
    }
    #[doc = "IN4"]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(PSELW::_100)
    }
    #[doc = "IN5"]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(PSELW::_101)
    }
    #[doc = "IN6"]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(PSELW::_110)
    }
    #[doc = "IN7"]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(PSELW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `VRSEL`"]
pub enum VRSELW {
    #[doc = "Vin1 is selected as resistor ladder network supply reference Vin."]
    _0,
    #[doc = "Vin2 is selected as resistor ladder network supply reference Vin."]
    _1,
}
impl VRSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            VRSELW::_0 => false,
            VRSELW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _VRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _VRSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: VRSELW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Vin1 is selected as resistor ladder network supply reference Vin."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(VRSELW::_0)
    }
    #[doc = "Vin2 is selected as resistor ladder network supply reference Vin."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(VRSELW::_1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DACEN`"]
pub enum DACENW {
    #[doc = "DAC is disabled."]
    _0,
    #[doc = "DAC is enabled."]
    _1,
}
impl DACENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DACENW::_0 => false,
            DACENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DACENW<'a> {
    w: &'a mut W,
}
impl<'a> _DACENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DACENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DAC is disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACENW::_0)
    }
    #[doc = "DAC is enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACENW::_1)
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CHN0W<'a> {
    w: &'a mut W,
}
impl<'a> _CHN0W<'a> {
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CHN1W<'a> {
    w: &'a mut W,
}
impl<'a> _CHN1W<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CHN2W<'a> {
    w: &'a mut W,
}
impl<'a> _CHN2W<'a> {
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CHN3W<'a> {
    w: &'a mut W,
}
impl<'a> _CHN3W<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CHN4W<'a> {
    w: &'a mut W,
}
impl<'a> _CHN4W<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CHN5W<'a> {
    w: &'a mut W,
}
impl<'a> _CHN5W<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CHN6W<'a> {
    w: &'a mut W,
}
impl<'a> _CHN6W<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CHN7W<'a> {
    w: &'a mut W,
}
impl<'a> _CHN7W<'a> {
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INNSEL`"]
pub enum INNSELW {
    #[doc = "IN0, from the 8-bit DAC output"]
    _00,
    #[doc = "IN1, from the analog 8-1 mux"]
    _01,
}
impl INNSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INNSELW::_00 => 0,
            INNSELW::_01 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INNSELW<'a> {
    w: &'a mut W,
}
impl<'a> _INNSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INNSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "IN0, from the 8-bit DAC output"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(INNSELW::_00)
    }
    #[doc = "IN1, from the analog 8-1 mux"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(INNSELW::_01)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `INPSEL`"]
pub enum INPSELW {
    #[doc = "IN0, from the 8-bit DAC output"]
    _00,
    #[doc = "IN1, from the analog 8-1 mux"]
    _01,
}
impl INPSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPSELW::_00 => 0,
            INPSELW::_01 => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPSELW<'a> {
    w: &'a mut W,
}
impl<'a> _INPSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPSELW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "IN0, from the 8-bit DAC output"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(INPSELW::_00)
    }
    #[doc = "IN1, from the analog 8-1 mux"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(INPSELW::_01)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 27;
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
    #[doc = "Bits 0:7 - DAC Output Voltage Select"]
    #[inline]
    pub fn vosel(&self) -> VOSELR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VOSELR { bits }
    }
    #[doc = "Bits 8:10 - Minus Input MUX Control"]
    #[inline]
    pub fn msel(&self) -> MSELR {
        MSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:13 - Plus Input MUX Control"]
    #[inline]
    pub fn psel(&self) -> PSELR {
        PSELR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 14 - Supply Voltage Reference Source Select"]
    #[inline]
    pub fn vrsel(&self) -> VRSELR {
        VRSELR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - DAC Enable"]
    #[inline]
    pub fn dacen(&self) -> DACENR {
        DACENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Channel 0 input enable"]
    #[inline]
    pub fn chn0(&self) -> CHN0R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHN0R { bits }
    }
    #[doc = "Bit 17 - Channel 1 input enable"]
    #[inline]
    pub fn chn1(&self) -> CHN1R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHN1R { bits }
    }
    #[doc = "Bit 18 - Channel 2 input enable"]
    #[inline]
    pub fn chn2(&self) -> CHN2R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHN2R { bits }
    }
    #[doc = "Bit 19 - Channel 3 input enable"]
    #[inline]
    pub fn chn3(&self) -> CHN3R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHN3R { bits }
    }
    #[doc = "Bit 20 - Channel 4 input enable"]
    #[inline]
    pub fn chn4(&self) -> CHN4R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHN4R { bits }
    }
    #[doc = "Bit 21 - Channel 5 input enable"]
    #[inline]
    pub fn chn5(&self) -> CHN5R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHN5R { bits }
    }
    #[doc = "Bit 22 - Channel 6 input enable"]
    #[inline]
    pub fn chn6(&self) -> CHN6R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHN6R { bits }
    }
    #[doc = "Bit 23 - Channel 7 input enable"]
    #[inline]
    pub fn chn7(&self) -> CHN7R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CHN7R { bits }
    }
    #[doc = "Bits 24:25 - Selection of the input to the negative port of the comparator"]
    #[inline]
    pub fn innsel(&self) -> INNSELR {
        INNSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 27:28 - Selection of the input to the positive port of the comparator"]
    #[inline]
    pub fn inpsel(&self) -> INPSELR {
        INPSELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 27;
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
    #[doc = "Bits 0:7 - DAC Output Voltage Select"]
    #[inline]
    pub fn vosel(&mut self) -> _VOSELW {
        _VOSELW { w: self }
    }
    #[doc = "Bits 8:10 - Minus Input MUX Control"]
    #[inline]
    pub fn msel(&mut self) -> _MSELW {
        _MSELW { w: self }
    }
    #[doc = "Bits 11:13 - Plus Input MUX Control"]
    #[inline]
    pub fn psel(&mut self) -> _PSELW {
        _PSELW { w: self }
    }
    #[doc = "Bit 14 - Supply Voltage Reference Source Select"]
    #[inline]
    pub fn vrsel(&mut self) -> _VRSELW {
        _VRSELW { w: self }
    }
    #[doc = "Bit 15 - DAC Enable"]
    #[inline]
    pub fn dacen(&mut self) -> _DACENW {
        _DACENW { w: self }
    }
    #[doc = "Bit 16 - Channel 0 input enable"]
    #[inline]
    pub fn chn0(&mut self) -> _CHN0W {
        _CHN0W { w: self }
    }
    #[doc = "Bit 17 - Channel 1 input enable"]
    #[inline]
    pub fn chn1(&mut self) -> _CHN1W {
        _CHN1W { w: self }
    }
    #[doc = "Bit 18 - Channel 2 input enable"]
    #[inline]
    pub fn chn2(&mut self) -> _CHN2W {
        _CHN2W { w: self }
    }
    #[doc = "Bit 19 - Channel 3 input enable"]
    #[inline]
    pub fn chn3(&mut self) -> _CHN3W {
        _CHN3W { w: self }
    }
    #[doc = "Bit 20 - Channel 4 input enable"]
    #[inline]
    pub fn chn4(&mut self) -> _CHN4W {
        _CHN4W { w: self }
    }
    #[doc = "Bit 21 - Channel 5 input enable"]
    #[inline]
    pub fn chn5(&mut self) -> _CHN5W {
        _CHN5W { w: self }
    }
    #[doc = "Bit 22 - Channel 6 input enable"]
    #[inline]
    pub fn chn6(&mut self) -> _CHN6W {
        _CHN6W { w: self }
    }
    #[doc = "Bit 23 - Channel 7 input enable"]
    #[inline]
    pub fn chn7(&mut self) -> _CHN7W {
        _CHN7W { w: self }
    }
    #[doc = "Bits 24:25 - Selection of the input to the negative port of the comparator"]
    #[inline]
    pub fn innsel(&mut self) -> _INNSELW {
        _INNSELW { w: self }
    }
    #[doc = "Bits 27:28 - Selection of the input to the positive port of the comparator"]
    #[inline]
    pub fn inpsel(&mut self) -> _INPSELW {
        _INPSELW { w: self }
    }
}
