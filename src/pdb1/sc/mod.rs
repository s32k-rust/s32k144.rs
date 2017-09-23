#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SC {
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
pub struct LDOKR {
    bits: bool,
}
impl LDOKR {
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
#[doc = "Possible values of the field `CONT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTR {
    #[doc = "PDB operation in One-Shot mode"] _0,
    #[doc = "PDB operation in Continuous mode"] _1,
}
impl CONTR {
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
            CONTR::_0 => false,
            CONTR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONTR {
        match value {
            false => CONTR::_0,
            true => CONTR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CONTR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CONTR::_1
    }
}
#[doc = "Possible values of the field `MULT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MULTR {
    #[doc = "Multiplication factor is 1."] _00,
    #[doc = "Multiplication factor is 10."] _01,
    #[doc = "Multiplication factor is 20."] _10,
    #[doc = "Multiplication factor is 40."] _11,
}
impl MULTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MULTR::_00 => 0,
            MULTR::_01 => 1,
            MULTR::_10 => 2,
            MULTR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MULTR {
        match value {
            0 => MULTR::_00,
            1 => MULTR::_01,
            2 => MULTR::_10,
            3 => MULTR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == MULTR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == MULTR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == MULTR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == MULTR::_11
    }
}
#[doc = "Possible values of the field `PDBIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDBIER {
    #[doc = "PDB interrupt disabled."] _0,
    #[doc = "PDB interrupt enabled."] _1,
}
impl PDBIER {
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
            PDBIER::_0 => false,
            PDBIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDBIER {
        match value {
            false => PDBIER::_0,
            true => PDBIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDBIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDBIER::_1
    }
}
#[doc = r" Value of the field"]
pub struct PDBIFR {
    bits: bool,
}
impl PDBIFR {
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
#[doc = "Possible values of the field `PDBEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDBENR {
    #[doc = "PDB disabled. Counter is off."] _0,
    #[doc = "PDB enabled."] _1,
}
impl PDBENR {
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
            PDBENR::_0 => false,
            PDBENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDBENR {
        match value {
            false => PDBENR::_0,
            true => PDBENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDBENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDBENR::_1
    }
}
#[doc = "Possible values of the field `TRGSEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGSELR {
    #[doc = "Trigger-In 0 is selected."] _0000,
    #[doc = "Trigger-In 1 is selected."] _0001,
    #[doc = "Trigger-In 2 is selected."] _0010,
    #[doc = "Trigger-In 3 is selected."] _0011,
    #[doc = "Trigger-In 4 is selected."] _0100,
    #[doc = "Trigger-In 5 is selected."] _0101,
    #[doc = "Trigger-In 6 is selected."] _0110,
    #[doc = "Trigger-In 7 is selected."] _0111,
    #[doc = "Trigger-In 8 is selected."] _1000,
    #[doc = "Trigger-In 9 is selected."] _1001,
    #[doc = "Trigger-In 10 is selected."] _1010,
    #[doc = "Trigger-In 11 is selected."] _1011,
    #[doc = "Trigger-In 12 is selected."] _1100,
    #[doc = "Trigger-In 13 is selected."] _1101,
    #[doc = "Trigger-In 14 is selected."] _1110,
    #[doc = "Software trigger is selected."] _1111,
}
impl TRGSELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TRGSELR::_0000 => 0,
            TRGSELR::_0001 => 1,
            TRGSELR::_0010 => 2,
            TRGSELR::_0011 => 3,
            TRGSELR::_0100 => 4,
            TRGSELR::_0101 => 5,
            TRGSELR::_0110 => 6,
            TRGSELR::_0111 => 7,
            TRGSELR::_1000 => 8,
            TRGSELR::_1001 => 9,
            TRGSELR::_1010 => 10,
            TRGSELR::_1011 => 11,
            TRGSELR::_1100 => 12,
            TRGSELR::_1101 => 13,
            TRGSELR::_1110 => 14,
            TRGSELR::_1111 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TRGSELR {
        match value {
            0 => TRGSELR::_0000,
            1 => TRGSELR::_0001,
            2 => TRGSELR::_0010,
            3 => TRGSELR::_0011,
            4 => TRGSELR::_0100,
            5 => TRGSELR::_0101,
            6 => TRGSELR::_0110,
            7 => TRGSELR::_0111,
            8 => TRGSELR::_1000,
            9 => TRGSELR::_1001,
            10 => TRGSELR::_1010,
            11 => TRGSELR::_1011,
            12 => TRGSELR::_1100,
            13 => TRGSELR::_1101,
            14 => TRGSELR::_1110,
            15 => TRGSELR::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == TRGSELR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == TRGSELR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == TRGSELR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == TRGSELR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == TRGSELR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == TRGSELR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == TRGSELR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == TRGSELR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == TRGSELR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == TRGSELR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == TRGSELR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == TRGSELR::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == TRGSELR::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == TRGSELR::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == TRGSELR::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == TRGSELR::_1111
    }
}
#[doc = "Possible values of the field `PRESCALER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALERR {
    #[doc = "Counting uses the peripheral clock divided by MULT (the multiplication factor)."] _000,
    #[doc = "Counting uses the peripheral clock divided by 2 x MULT (the multiplication factor)."]
    _001,
    #[doc = "Counting uses the peripheral clock divided by 4 x MULT (the multiplication factor)."]
    _010,
    #[doc = "Counting uses the peripheral clock divided by 8 x MULT (the multiplication factor)."]
    _011,
    #[doc = "Counting uses the peripheral clock divided by 16 x MULT (the multiplication factor)."]
    _100,
    #[doc = "Counting uses the peripheral clock divided by 32 x MULT (the multiplication factor)."]
    _101,
    #[doc = "Counting uses the peripheral clock divided by 64 x MULT (the multiplication factor)."]
    _110,
    #[doc = "Counting uses the peripheral clock divided by 128 x MULT (the multiplication factor)."]
    _111,
}
impl PRESCALERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCALERR::_000 => 0,
            PRESCALERR::_001 => 1,
            PRESCALERR::_010 => 2,
            PRESCALERR::_011 => 3,
            PRESCALERR::_100 => 4,
            PRESCALERR::_101 => 5,
            PRESCALERR::_110 => 6,
            PRESCALERR::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCALERR {
        match value {
            0 => PRESCALERR::_000,
            1 => PRESCALERR::_001,
            2 => PRESCALERR::_010,
            3 => PRESCALERR::_011,
            4 => PRESCALERR::_100,
            5 => PRESCALERR::_101,
            6 => PRESCALERR::_110,
            7 => PRESCALERR::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == PRESCALERR::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == PRESCALERR::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == PRESCALERR::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == PRESCALERR::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == PRESCALERR::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == PRESCALERR::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == PRESCALERR::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == PRESCALERR::_111
    }
}
#[doc = "Possible values of the field `DMAEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAENR {
    #[doc = "DMA disabled."] _0,
    #[doc = "DMA enabled."] _1,
}
impl DMAENR {
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
            DMAENR::_0 => false,
            DMAENR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DMAENR {
        match value {
            false => DMAENR::_0,
            true => DMAENR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == DMAENR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == DMAENR::_1
    }
}
#[doc = "Possible values of the field `PDBEIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDBEIER {
    #[doc = "PDB sequence error interrupt disabled."] _0,
    #[doc = "PDB sequence error interrupt enabled."] _1,
}
impl PDBEIER {
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
            PDBEIER::_0 => false,
            PDBEIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> PDBEIER {
        match value {
            false => PDBEIER::_0,
            true => PDBEIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == PDBEIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == PDBEIER::_1
    }
}
#[doc = "Possible values of the field `LDMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDMODR {
    #[doc = "The internal registers are loaded with the values from their buffers, immediately after 1 is written to LDOK."]
    _00,
    #[doc = "The internal registers are loaded with the values from their buffers when the PDB counter (CNT) = MOD + 1 CNT delay elapsed, after 1 is written to LDOK."]
    _01,
    #[doc = "The internal registers are loaded with the values from their buffers when a trigger input event is detected, after 1 is written to LDOK."]
    _10,
    #[doc = "The internal registers are loaded with the values from their buffers when either the PDB counter (CNT) = MOD + 1 CNT delay elapsed, or a trigger input event is detected, after 1 is written to LDOK."]
    _11,
}
impl LDMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LDMODR::_00 => 0,
            LDMODR::_01 => 1,
            LDMODR::_10 => 2,
            LDMODR::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LDMODR {
        match value {
            0 => LDMODR::_00,
            1 => LDMODR::_01,
            2 => LDMODR::_10,
            3 => LDMODR::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == LDMODR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == LDMODR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == LDMODR::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == LDMODR::_11
    }
}
#[doc = r" Proxy"]
pub struct _LDOKW<'a> {
    w: &'a mut W,
}
impl<'a> _LDOKW<'a> {
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
#[doc = "Values that can be written to the field `CONT`"]
pub enum CONTW {
    #[doc = "PDB operation in One-Shot mode"] _0,
    #[doc = "PDB operation in Continuous mode"] _1,
}
impl CONTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CONTW::_0 => false,
            CONTW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONTW<'a> {
    w: &'a mut W,
}
impl<'a> _CONTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB operation in One-Shot mode"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CONTW::_0)
    }
    #[doc = "PDB operation in Continuous mode"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CONTW::_1)
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
#[doc = "Values that can be written to the field `MULT`"]
pub enum MULTW {
    #[doc = "Multiplication factor is 1."] _00,
    #[doc = "Multiplication factor is 10."] _01,
    #[doc = "Multiplication factor is 20."] _10,
    #[doc = "Multiplication factor is 40."] _11,
}
impl MULTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MULTW::_00 => 0,
            MULTW::_01 => 1,
            MULTW::_10 => 2,
            MULTW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MULTW<'a> {
    w: &'a mut W,
}
impl<'a> _MULTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MULTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Multiplication factor is 1."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(MULTW::_00)
    }
    #[doc = "Multiplication factor is 10."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(MULTW::_01)
    }
    #[doc = "Multiplication factor is 20."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(MULTW::_10)
    }
    #[doc = "Multiplication factor is 40."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(MULTW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDBIE`"]
pub enum PDBIEW {
    #[doc = "PDB interrupt disabled."] _0,
    #[doc = "PDB interrupt enabled."] _1,
}
impl PDBIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDBIEW::_0 => false,
            PDBIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDBIEW<'a> {
    w: &'a mut W,
}
impl<'a> _PDBIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDBIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDBIEW::_0)
    }
    #[doc = "PDB interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDBIEW::_1)
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
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PDBIFW<'a> {
    w: &'a mut W,
}
impl<'a> _PDBIFW<'a> {
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PDBEN`"]
pub enum PDBENW {
    #[doc = "PDB disabled. Counter is off."] _0,
    #[doc = "PDB enabled."] _1,
}
impl PDBENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDBENW::_0 => false,
            PDBENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDBENW<'a> {
    w: &'a mut W,
}
impl<'a> _PDBENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDBENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB disabled. Counter is off."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDBENW::_0)
    }
    #[doc = "PDB enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDBENW::_1)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRGSEL`"]
pub enum TRGSELW {
    #[doc = "Trigger-In 0 is selected."] _0000,
    #[doc = "Trigger-In 1 is selected."] _0001,
    #[doc = "Trigger-In 2 is selected."] _0010,
    #[doc = "Trigger-In 3 is selected."] _0011,
    #[doc = "Trigger-In 4 is selected."] _0100,
    #[doc = "Trigger-In 5 is selected."] _0101,
    #[doc = "Trigger-In 6 is selected."] _0110,
    #[doc = "Trigger-In 7 is selected."] _0111,
    #[doc = "Trigger-In 8 is selected."] _1000,
    #[doc = "Trigger-In 9 is selected."] _1001,
    #[doc = "Trigger-In 10 is selected."] _1010,
    #[doc = "Trigger-In 11 is selected."] _1011,
    #[doc = "Trigger-In 12 is selected."] _1100,
    #[doc = "Trigger-In 13 is selected."] _1101,
    #[doc = "Trigger-In 14 is selected."] _1110,
    #[doc = "Software trigger is selected."] _1111,
}
impl TRGSELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRGSELW::_0000 => 0,
            TRGSELW::_0001 => 1,
            TRGSELW::_0010 => 2,
            TRGSELW::_0011 => 3,
            TRGSELW::_0100 => 4,
            TRGSELW::_0101 => 5,
            TRGSELW::_0110 => 6,
            TRGSELW::_0111 => 7,
            TRGSELW::_1000 => 8,
            TRGSELW::_1001 => 9,
            TRGSELW::_1010 => 10,
            TRGSELW::_1011 => 11,
            TRGSELW::_1100 => 12,
            TRGSELW::_1101 => 13,
            TRGSELW::_1110 => 14,
            TRGSELW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRGSELW<'a> {
    w: &'a mut W,
}
impl<'a> _TRGSELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRGSELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Trigger-In 0 is selected."]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(TRGSELW::_0000)
    }
    #[doc = "Trigger-In 1 is selected."]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(TRGSELW::_0001)
    }
    #[doc = "Trigger-In 2 is selected."]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(TRGSELW::_0010)
    }
    #[doc = "Trigger-In 3 is selected."]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(TRGSELW::_0011)
    }
    #[doc = "Trigger-In 4 is selected."]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(TRGSELW::_0100)
    }
    #[doc = "Trigger-In 5 is selected."]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(TRGSELW::_0101)
    }
    #[doc = "Trigger-In 6 is selected."]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(TRGSELW::_0110)
    }
    #[doc = "Trigger-In 7 is selected."]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(TRGSELW::_0111)
    }
    #[doc = "Trigger-In 8 is selected."]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(TRGSELW::_1000)
    }
    #[doc = "Trigger-In 9 is selected."]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(TRGSELW::_1001)
    }
    #[doc = "Trigger-In 10 is selected."]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(TRGSELW::_1010)
    }
    #[doc = "Trigger-In 11 is selected."]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(TRGSELW::_1011)
    }
    #[doc = "Trigger-In 12 is selected."]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(TRGSELW::_1100)
    }
    #[doc = "Trigger-In 13 is selected."]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(TRGSELW::_1101)
    }
    #[doc = "Trigger-In 14 is selected."]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(TRGSELW::_1110)
    }
    #[doc = "Software trigger is selected."]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(TRGSELW::_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESCALER`"]
pub enum PRESCALERW {
    #[doc = "Counting uses the peripheral clock divided by MULT (the multiplication factor)."] _000,
    #[doc = "Counting uses the peripheral clock divided by 2 x MULT (the multiplication factor)."]
    _001,
    #[doc = "Counting uses the peripheral clock divided by 4 x MULT (the multiplication factor)."]
    _010,
    #[doc = "Counting uses the peripheral clock divided by 8 x MULT (the multiplication factor)."]
    _011,
    #[doc = "Counting uses the peripheral clock divided by 16 x MULT (the multiplication factor)."]
    _100,
    #[doc = "Counting uses the peripheral clock divided by 32 x MULT (the multiplication factor)."]
    _101,
    #[doc = "Counting uses the peripheral clock divided by 64 x MULT (the multiplication factor)."]
    _110,
    #[doc = "Counting uses the peripheral clock divided by 128 x MULT (the multiplication factor)."]
    _111,
}
impl PRESCALERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCALERW::_000 => 0,
            PRESCALERW::_001 => 1,
            PRESCALERW::_010 => 2,
            PRESCALERW::_011 => 3,
            PRESCALERW::_100 => 4,
            PRESCALERW::_101 => 5,
            PRESCALERW::_110 => 6,
            PRESCALERW::_111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCALERW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCALERW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Counting uses the peripheral clock divided by MULT (the multiplication factor)."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(PRESCALERW::_000)
    }
    #[doc = "Counting uses the peripheral clock divided by 2 x MULT (the multiplication factor)."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(PRESCALERW::_001)
    }
    #[doc = "Counting uses the peripheral clock divided by 4 x MULT (the multiplication factor)."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(PRESCALERW::_010)
    }
    #[doc = "Counting uses the peripheral clock divided by 8 x MULT (the multiplication factor)."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(PRESCALERW::_011)
    }
    #[doc = "Counting uses the peripheral clock divided by 16 x MULT (the multiplication factor)."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(PRESCALERW::_100)
    }
    #[doc = "Counting uses the peripheral clock divided by 32 x MULT (the multiplication factor)."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(PRESCALERW::_101)
    }
    #[doc = "Counting uses the peripheral clock divided by 64 x MULT (the multiplication factor)."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(PRESCALERW::_110)
    }
    #[doc = "Counting uses the peripheral clock divided by 128 x MULT (the multiplication factor)."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(PRESCALERW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DMAEN`"]
pub enum DMAENW {
    #[doc = "DMA disabled."] _0,
    #[doc = "DMA enabled."] _1,
}
impl DMAENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DMAENW::_0 => false,
            DMAENW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DMAENW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAENW::_0)
    }
    #[doc = "DMA enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAENW::_1)
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
pub struct _SWTRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _SWTRIGW<'a> {
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
#[doc = "Values that can be written to the field `PDBEIE`"]
pub enum PDBEIEW {
    #[doc = "PDB sequence error interrupt disabled."] _0,
    #[doc = "PDB sequence error interrupt enabled."] _1,
}
impl PDBEIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            PDBEIEW::_0 => false,
            PDBEIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PDBEIEW<'a> {
    w: &'a mut W,
}
impl<'a> _PDBEIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PDBEIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "PDB sequence error interrupt disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDBEIEW::_0)
    }
    #[doc = "PDB sequence error interrupt enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDBEIEW::_1)
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LDMOD`"]
pub enum LDMODW {
    #[doc = "The internal registers are loaded with the values from their buffers, immediately after 1 is written to LDOK."]
    _00,
    #[doc = "The internal registers are loaded with the values from their buffers when the PDB counter (CNT) = MOD + 1 CNT delay elapsed, after 1 is written to LDOK."]
    _01,
    #[doc = "The internal registers are loaded with the values from their buffers when a trigger input event is detected, after 1 is written to LDOK."]
    _10,
    #[doc = "The internal registers are loaded with the values from their buffers when either the PDB counter (CNT) = MOD + 1 CNT delay elapsed, or a trigger input event is detected, after 1 is written to LDOK."]
    _11,
}
impl LDMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LDMODW::_00 => 0,
            LDMODW::_01 => 1,
            LDMODW::_10 => 2,
            LDMODW::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LDMODW<'a> {
    w: &'a mut W,
}
impl<'a> _LDMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LDMODW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The internal registers are loaded with the values from their buffers, immediately after 1 is written to LDOK."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(LDMODW::_00)
    }
    #[doc = "The internal registers are loaded with the values from their buffers when the PDB counter (CNT) = MOD + 1 CNT delay elapsed, after 1 is written to LDOK."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(LDMODW::_01)
    }
    #[doc = "The internal registers are loaded with the values from their buffers when a trigger input event is detected, after 1 is written to LDOK."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(LDMODW::_10)
    }
    #[doc = "The internal registers are loaded with the values from their buffers when either the PDB counter (CNT) = MOD + 1 CNT delay elapsed, or a trigger input event is detected, after 1 is written to LDOK."]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(LDMODW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - Load OK"]
    #[inline]
    pub fn ldok(&self) -> LDOKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LDOKR { bits }
    }
    #[doc = "Bit 1 - Continuous Mode Enable"]
    #[inline]
    pub fn cont(&self) -> CONTR {
        CONTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 2:3 - Multiplication Factor Select for Prescaler"]
    #[inline]
    pub fn mult(&self) -> MULTR {
        MULTR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 5 - PDB Interrupt Enable"]
    #[inline]
    pub fn pdbie(&self) -> PDBIER {
        PDBIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - PDB Interrupt Flag"]
    #[inline]
    pub fn pdbif(&self) -> PDBIFR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PDBIFR { bits }
    }
    #[doc = "Bit 7 - PDB Enable"]
    #[inline]
    pub fn pdben(&self) -> PDBENR {
        PDBENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:11 - Trigger Input Source Select"]
    #[inline]
    pub fn trgsel(&self) -> TRGSELR {
        TRGSELR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - Prescaler Divider Select"]
    #[inline]
    pub fn prescaler(&self) -> PRESCALERR {
        PRESCALERR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - DMA Enable"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        DMAENR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - PDB Sequence Error Interrupt Enable"]
    #[inline]
    pub fn pdbeie(&self) -> PDBEIER {
        PDBEIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 18:19 - Load Mode Select"]
    #[inline]
    pub fn ldmod(&self) -> LDMODR {
        LDMODR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - Load OK"]
    #[inline]
    pub fn ldok(&mut self) -> _LDOKW {
        _LDOKW { w: self }
    }
    #[doc = "Bit 1 - Continuous Mode Enable"]
    #[inline]
    pub fn cont(&mut self) -> _CONTW {
        _CONTW { w: self }
    }
    #[doc = "Bits 2:3 - Multiplication Factor Select for Prescaler"]
    #[inline]
    pub fn mult(&mut self) -> _MULTW {
        _MULTW { w: self }
    }
    #[doc = "Bit 5 - PDB Interrupt Enable"]
    #[inline]
    pub fn pdbie(&mut self) -> _PDBIEW {
        _PDBIEW { w: self }
    }
    #[doc = "Bit 6 - PDB Interrupt Flag"]
    #[inline]
    pub fn pdbif(&mut self) -> _PDBIFW {
        _PDBIFW { w: self }
    }
    #[doc = "Bit 7 - PDB Enable"]
    #[inline]
    pub fn pdben(&mut self) -> _PDBENW {
        _PDBENW { w: self }
    }
    #[doc = "Bits 8:11 - Trigger Input Source Select"]
    #[inline]
    pub fn trgsel(&mut self) -> _TRGSELW {
        _TRGSELW { w: self }
    }
    #[doc = "Bits 12:14 - Prescaler Divider Select"]
    #[inline]
    pub fn prescaler(&mut self) -> _PRESCALERW {
        _PRESCALERW { w: self }
    }
    #[doc = "Bit 15 - DMA Enable"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
    #[doc = "Bit 16 - Software Trigger"]
    #[inline]
    pub fn swtrig(&mut self) -> _SWTRIGW {
        _SWTRIGW { w: self }
    }
    #[doc = "Bit 17 - PDB Sequence Error Interrupt Enable"]
    #[inline]
    pub fn pdbeie(&mut self) -> _PDBEIEW {
        _PDBEIEW { w: self }
    }
    #[doc = "Bits 18:19 - Load Mode Select"]
    #[inline]
    pub fn ldmod(&mut self) -> _LDMODW {
        _LDMODW { w: self }
    }
}
