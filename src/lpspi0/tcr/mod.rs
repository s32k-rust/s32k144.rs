#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TCR {
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
pub struct FRAMESZR {
    bits: u16,
}
impl FRAMESZR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `WIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WIDTHR {
    #[doc = "Single bit transfer."]
    _00,
    #[doc = "Two bit transfer."]
    _01,
    #[doc = "Four bit transfer."]
    _10,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl WIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            WIDTHR::_00 => 0,
            WIDTHR::_01 => 1,
            WIDTHR::_10 => 2,
            WIDTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> WIDTHR {
        match value {
            0 => WIDTHR::_00,
            1 => WIDTHR::_01,
            2 => WIDTHR::_10,
            i => WIDTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == WIDTHR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == WIDTHR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == WIDTHR::_10
    }
}
#[doc = "Possible values of the field `TXMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXMSKR {
    #[doc = "Normal transfer."]
    _0,
    #[doc = "Mask transmit data."]
    _1,
}
impl TXMSKR {
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
            TXMSKR::_0 => false,
            TXMSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TXMSKR {
        match value {
            false => TXMSKR::_0,
            true => TXMSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TXMSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TXMSKR::_1
    }
}
#[doc = "Possible values of the field `RXMSK`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXMSKR {
    #[doc = "Normal transfer."]
    _0,
    #[doc = "Receive data is masked."]
    _1,
}
impl RXMSKR {
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
            RXMSKR::_0 => false,
            RXMSKR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXMSKR {
        match value {
            false => RXMSKR::_0,
            true => RXMSKR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXMSKR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXMSKR::_1
    }
}
#[doc = "Possible values of the field `CONTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTCR {
    #[doc = "Command word for start of new transfer."]
    _0,
    #[doc = "Command word for continuing transfer."]
    _1,
}
impl CONTCR {
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
            CONTCR::_0 => false,
            CONTCR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CONTCR {
        match value {
            false => CONTCR::_0,
            true => CONTCR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CONTCR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CONTCR::_1
    }
}
#[doc = "Possible values of the field `CONT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONTR {
    #[doc = "Continuous transfer disabled."]
    _0,
    #[doc = "Continuous transfer enabled."]
    _1,
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
#[doc = "Possible values of the field `BYSW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYSWR {
    #[doc = "Byte swap disabled."]
    _0,
    #[doc = "Byte swap enabled."]
    _1,
}
impl BYSWR {
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
            BYSWR::_0 => false,
            BYSWR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BYSWR {
        match value {
            false => BYSWR::_0,
            true => BYSWR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BYSWR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BYSWR::_1
    }
}
#[doc = "Possible values of the field `LSBF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSBFR {
    #[doc = "Data is transferred MSB first."]
    _0,
    #[doc = "Data is transferred LSB first."]
    _1,
}
impl LSBFR {
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
            LSBFR::_0 => false,
            LSBFR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LSBFR {
        match value {
            false => LSBFR::_0,
            true => LSBFR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LSBFR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LSBFR::_1
    }
}
#[doc = "Possible values of the field `PCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSR {
    #[doc = "Transfer using LPSPI_PCS[0]"]
    _00,
    #[doc = "Transfer using LPSPI_PCS[1]"]
    _01,
    #[doc = "Transfer using LPSPI_PCS[2]"]
    _10,
    #[doc = "Transfer using LPSPI_PCS[3]"]
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
#[doc = "Possible values of the field `PRESCALE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALER {
    #[doc = "Divide by 1."]
    _000,
    #[doc = "Divide by 2."]
    _001,
    #[doc = "Divide by 4."]
    _010,
    #[doc = "Divide by 8."]
    _011,
    #[doc = "Divide by 16."]
    _100,
    #[doc = "Divide by 32."]
    _101,
    #[doc = "Divide by 64."]
    _110,
    #[doc = "Divide by 128."]
    _111,
}
impl PRESCALER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCALER::_000 => 0,
            PRESCALER::_001 => 1,
            PRESCALER::_010 => 2,
            PRESCALER::_011 => 3,
            PRESCALER::_100 => 4,
            PRESCALER::_101 => 5,
            PRESCALER::_110 => 6,
            PRESCALER::_111 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCALER {
        match value {
            0 => PRESCALER::_000,
            1 => PRESCALER::_001,
            2 => PRESCALER::_010,
            3 => PRESCALER::_011,
            4 => PRESCALER::_100,
            5 => PRESCALER::_101,
            6 => PRESCALER::_110,
            7 => PRESCALER::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline]
    pub fn is_000(&self) -> bool {
        *self == PRESCALER::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline]
    pub fn is_001(&self) -> bool {
        *self == PRESCALER::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline]
    pub fn is_010(&self) -> bool {
        *self == PRESCALER::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline]
    pub fn is_011(&self) -> bool {
        *self == PRESCALER::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline]
    pub fn is_100(&self) -> bool {
        *self == PRESCALER::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline]
    pub fn is_101(&self) -> bool {
        *self == PRESCALER::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline]
    pub fn is_110(&self) -> bool {
        *self == PRESCALER::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline]
    pub fn is_111(&self) -> bool {
        *self == PRESCALER::_111
    }
}
#[doc = "Possible values of the field `CPHA`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHAR {
    #[doc = "Data is captured on the leading edge of SCK and changed on the following edge."]
    _0,
    #[doc = "Data is changed on the leading edge of SCK and captured on the following edge."]
    _1,
}
impl CPHAR {
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
            CPHAR::_0 => false,
            CPHAR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPHAR {
        match value {
            false => CPHAR::_0,
            true => CPHAR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPHAR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPHAR::_1
    }
}
#[doc = "Possible values of the field `CPOL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOLR {
    #[doc = "The inactive state value of SCK is low."]
    _0,
    #[doc = "The inactive state value of SCK is high."]
    _1,
}
impl CPOLR {
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
            CPOLR::_0 => false,
            CPOLR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> CPOLR {
        match value {
            false => CPOLR::_0,
            true => CPOLR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == CPOLR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == CPOLR::_1
    }
}
#[doc = r" Proxy"]
pub struct _FRAMESZW<'a> {
    w: &'a mut W,
}
impl<'a> _FRAMESZW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WIDTH`"]
pub enum WIDTHW {
    #[doc = "Single bit transfer."]
    _00,
    #[doc = "Two bit transfer."]
    _01,
    #[doc = "Four bit transfer."]
    _10,
}
impl WIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            WIDTHW::_00 => 0,
            WIDTHW::_01 => 1,
            WIDTHW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _WIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Single bit transfer."]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(WIDTHW::_00)
    }
    #[doc = "Two bit transfer."]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(WIDTHW::_01)
    }
    #[doc = "Four bit transfer."]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(WIDTHW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TXMSK`"]
pub enum TXMSKW {
    #[doc = "Normal transfer."]
    _0,
    #[doc = "Mask transmit data."]
    _1,
}
impl TXMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TXMSKW::_0 => false,
            TXMSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _TXMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal transfer."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXMSKW::_0)
    }
    #[doc = "Mask transmit data."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXMSKW::_1)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXMSK`"]
pub enum RXMSKW {
    #[doc = "Normal transfer."]
    _0,
    #[doc = "Receive data is masked."]
    _1,
}
impl RXMSKW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXMSKW::_0 => false,
            RXMSKW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXMSKW<'a> {
    w: &'a mut W,
}
impl<'a> _RXMSKW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXMSKW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal transfer."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXMSKW::_0)
    }
    #[doc = "Receive data is masked."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXMSKW::_1)
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CONTC`"]
pub enum CONTCW {
    #[doc = "Command word for start of new transfer."]
    _0,
    #[doc = "Command word for continuing transfer."]
    _1,
}
impl CONTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CONTCW::_0 => false,
            CONTCW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CONTCW<'a> {
    w: &'a mut W,
}
impl<'a> _CONTCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CONTCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Command word for start of new transfer."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CONTCW::_0)
    }
    #[doc = "Command word for continuing transfer."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CONTCW::_1)
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CONT`"]
pub enum CONTW {
    #[doc = "Continuous transfer disabled."]
    _0,
    #[doc = "Continuous transfer enabled."]
    _1,
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
    #[doc = "Continuous transfer disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CONTW::_0)
    }
    #[doc = "Continuous transfer enabled."]
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BYSW`"]
pub enum BYSWW {
    #[doc = "Byte swap disabled."]
    _0,
    #[doc = "Byte swap enabled."]
    _1,
}
impl BYSWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BYSWW::_0 => false,
            BYSWW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BYSWW<'a> {
    w: &'a mut W,
}
impl<'a> _BYSWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BYSWW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Byte swap disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BYSWW::_0)
    }
    #[doc = "Byte swap enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BYSWW::_1)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `LSBF`"]
pub enum LSBFW {
    #[doc = "Data is transferred MSB first."]
    _0,
    #[doc = "Data is transferred LSB first."]
    _1,
}
impl LSBFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LSBFW::_0 => false,
            LSBFW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LSBFW<'a> {
    w: &'a mut W,
}
impl<'a> _LSBFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LSBFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data is transferred MSB first."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LSBFW::_0)
    }
    #[doc = "Data is transferred LSB first."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LSBFW::_1)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PCS`"]
pub enum PCSW {
    #[doc = "Transfer using LPSPI_PCS[0]"]
    _00,
    #[doc = "Transfer using LPSPI_PCS[1]"]
    _01,
    #[doc = "Transfer using LPSPI_PCS[2]"]
    _10,
    #[doc = "Transfer using LPSPI_PCS[3]"]
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
    #[doc = "Transfer using LPSPI_PCS[0]"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(PCSW::_00)
    }
    #[doc = "Transfer using LPSPI_PCS[1]"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(PCSW::_01)
    }
    #[doc = "Transfer using LPSPI_PCS[2]"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(PCSW::_10)
    }
    #[doc = "Transfer using LPSPI_PCS[3]"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(PCSW::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESCALE`"]
pub enum PRESCALEW {
    #[doc = "Divide by 1."]
    _000,
    #[doc = "Divide by 2."]
    _001,
    #[doc = "Divide by 4."]
    _010,
    #[doc = "Divide by 8."]
    _011,
    #[doc = "Divide by 16."]
    _100,
    #[doc = "Divide by 32."]
    _101,
    #[doc = "Divide by 64."]
    _110,
    #[doc = "Divide by 128."]
    _111,
}
impl PRESCALEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCALEW::_000 => 0,
            PRESCALEW::_001 => 1,
            PRESCALEW::_010 => 2,
            PRESCALEW::_011 => 3,
            PRESCALEW::_100 => 4,
            PRESCALEW::_101 => 5,
            PRESCALEW::_110 => 6,
            PRESCALEW::_111 => 7,
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
    #[doc = "Divide by 1."]
    #[inline]
    pub fn _000(self) -> &'a mut W {
        self.variant(PRESCALEW::_000)
    }
    #[doc = "Divide by 2."]
    #[inline]
    pub fn _001(self) -> &'a mut W {
        self.variant(PRESCALEW::_001)
    }
    #[doc = "Divide by 4."]
    #[inline]
    pub fn _010(self) -> &'a mut W {
        self.variant(PRESCALEW::_010)
    }
    #[doc = "Divide by 8."]
    #[inline]
    pub fn _011(self) -> &'a mut W {
        self.variant(PRESCALEW::_011)
    }
    #[doc = "Divide by 16."]
    #[inline]
    pub fn _100(self) -> &'a mut W {
        self.variant(PRESCALEW::_100)
    }
    #[doc = "Divide by 32."]
    #[inline]
    pub fn _101(self) -> &'a mut W {
        self.variant(PRESCALEW::_101)
    }
    #[doc = "Divide by 64."]
    #[inline]
    pub fn _110(self) -> &'a mut W {
        self.variant(PRESCALEW::_110)
    }
    #[doc = "Divide by 128."]
    #[inline]
    pub fn _111(self) -> &'a mut W {
        self.variant(PRESCALEW::_111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CPHA`"]
pub enum CPHAW {
    #[doc = "Data is captured on the leading edge of SCK and changed on the following edge."]
    _0,
    #[doc = "Data is changed on the leading edge of SCK and captured on the following edge."]
    _1,
}
impl CPHAW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPHAW::_0 => false,
            CPHAW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPHAW<'a> {
    w: &'a mut W,
}
impl<'a> _CPHAW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPHAW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Data is captured on the leading edge of SCK and changed on the following edge."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPHAW::_0)
    }
    #[doc = "Data is changed on the leading edge of SCK and captured on the following edge."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPHAW::_1)
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `CPOL`"]
pub enum CPOLW {
    #[doc = "The inactive state value of SCK is low."]
    _0,
    #[doc = "The inactive state value of SCK is high."]
    _1,
}
impl CPOLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            CPOLW::_0 => false,
            CPOLW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CPOLW<'a> {
    w: &'a mut W,
}
impl<'a> _CPOLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CPOLW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The inactive state value of SCK is low."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPOLW::_0)
    }
    #[doc = "The inactive state value of SCK is high."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPOLW::_1)
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bits 0:11 - Frame Size"]
    #[inline]
    pub fn framesz(&self) -> FRAMESZR {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        FRAMESZR { bits }
    }
    #[doc = "Bits 16:17 - Transfer Width"]
    #[inline]
    pub fn width(&self) -> WIDTHR {
        WIDTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 18 - Transmit Data Mask"]
    #[inline]
    pub fn txmsk(&self) -> TXMSKR {
        TXMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 19 - Receive Data Mask"]
    #[inline]
    pub fn rxmsk(&self) -> RXMSKR {
        RXMSKR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 20 - Continuing Command"]
    #[inline]
    pub fn contc(&self) -> CONTCR {
        CONTCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Continuous Transfer"]
    #[inline]
    pub fn cont(&self) -> CONTR {
        CONTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Byte Swap"]
    #[inline]
    pub fn bysw(&self) -> BYSWR {
        BYSWR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - LSB First"]
    #[inline]
    pub fn lsbf(&self) -> LSBFR {
        LSBFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:25 - Peripheral Chip Select"]
    #[inline]
    pub fn pcs(&self) -> PCSR {
        PCSR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 27:29 - Prescaler Value"]
    #[inline]
    pub fn prescale(&self) -> PRESCALER {
        PRESCALER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - Clock Phase"]
    #[inline]
    pub fn cpha(&self) -> CPHAR {
        CPHAR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Clock Polarity"]
    #[inline]
    pub fn cpol(&self) -> CPOLR {
        CPOLR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 31 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:11 - Frame Size"]
    #[inline]
    pub fn framesz(&mut self) -> _FRAMESZW {
        _FRAMESZW { w: self }
    }
    #[doc = "Bits 16:17 - Transfer Width"]
    #[inline]
    pub fn width(&mut self) -> _WIDTHW {
        _WIDTHW { w: self }
    }
    #[doc = "Bit 18 - Transmit Data Mask"]
    #[inline]
    pub fn txmsk(&mut self) -> _TXMSKW {
        _TXMSKW { w: self }
    }
    #[doc = "Bit 19 - Receive Data Mask"]
    #[inline]
    pub fn rxmsk(&mut self) -> _RXMSKW {
        _RXMSKW { w: self }
    }
    #[doc = "Bit 20 - Continuing Command"]
    #[inline]
    pub fn contc(&mut self) -> _CONTCW {
        _CONTCW { w: self }
    }
    #[doc = "Bit 21 - Continuous Transfer"]
    #[inline]
    pub fn cont(&mut self) -> _CONTW {
        _CONTW { w: self }
    }
    #[doc = "Bit 22 - Byte Swap"]
    #[inline]
    pub fn bysw(&mut self) -> _BYSWW {
        _BYSWW { w: self }
    }
    #[doc = "Bit 23 - LSB First"]
    #[inline]
    pub fn lsbf(&mut self) -> _LSBFW {
        _LSBFW { w: self }
    }
    #[doc = "Bits 24:25 - Peripheral Chip Select"]
    #[inline]
    pub fn pcs(&mut self) -> _PCSW {
        _PCSW { w: self }
    }
    #[doc = "Bits 27:29 - Prescaler Value"]
    #[inline]
    pub fn prescale(&mut self) -> _PRESCALEW {
        _PRESCALEW { w: self }
    }
    #[doc = "Bit 30 - Clock Phase"]
    #[inline]
    pub fn cpha(&mut self) -> _CPHAW {
        _CPHAW { w: self }
    }
    #[doc = "Bit 31 - Clock Polarity"]
    #[inline]
    pub fn cpol(&mut self) -> _CPOLW {
        _CPOLW { w: self }
    }
}
