#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RCCR {
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
#[doc = "Possible values of the field `DIVSLOW`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVSLOWR {
    #[doc = "Divide-by-1"]
    _0000,
    #[doc = "Divide-by-2"]
    _0001,
    #[doc = "Divide-by-3"]
    _0010,
    #[doc = "Divide-by-4"]
    _0011,
    #[doc = "Divide-by-5"]
    _0100,
    #[doc = "Divide-by-6"]
    _0101,
    #[doc = "Divide-by-7"]
    _0110,
    #[doc = "Divide-by-8"]
    _0111,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DIVSLOWR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIVSLOWR::_0000 => 0,
            DIVSLOWR::_0001 => 1,
            DIVSLOWR::_0010 => 2,
            DIVSLOWR::_0011 => 3,
            DIVSLOWR::_0100 => 4,
            DIVSLOWR::_0101 => 5,
            DIVSLOWR::_0110 => 6,
            DIVSLOWR::_0111 => 7,
            DIVSLOWR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIVSLOWR {
        match value {
            0 => DIVSLOWR::_0000,
            1 => DIVSLOWR::_0001,
            2 => DIVSLOWR::_0010,
            3 => DIVSLOWR::_0011,
            4 => DIVSLOWR::_0100,
            5 => DIVSLOWR::_0101,
            6 => DIVSLOWR::_0110,
            7 => DIVSLOWR::_0111,
            i => DIVSLOWR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == DIVSLOWR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == DIVSLOWR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == DIVSLOWR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == DIVSLOWR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == DIVSLOWR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == DIVSLOWR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == DIVSLOWR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == DIVSLOWR::_0111
    }
}
#[doc = "Possible values of the field `DIVBUS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVBUSR {
    #[doc = "Divide-by-1"]
    _0000,
    #[doc = "Divide-by-2"]
    _0001,
    #[doc = "Divide-by-3"]
    _0010,
    #[doc = "Divide-by-4"]
    _0011,
    #[doc = "Divide-by-5"]
    _0100,
    #[doc = "Divide-by-6"]
    _0101,
    #[doc = "Divide-by-7"]
    _0110,
    #[doc = "Divide-by-8"]
    _0111,
    #[doc = "Divide-by-9"]
    _1000,
    #[doc = "Divide-by-10"]
    _1001,
    #[doc = "Divide-by-11"]
    _1010,
    #[doc = "Divide-by-12"]
    _1011,
    #[doc = "Divide-by-13"]
    _1100,
    #[doc = "Divide-by-14"]
    _1101,
    #[doc = "Divide-by-15"]
    _1110,
    #[doc = "Divide-by-16"]
    _1111,
}
impl DIVBUSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIVBUSR::_0000 => 0,
            DIVBUSR::_0001 => 1,
            DIVBUSR::_0010 => 2,
            DIVBUSR::_0011 => 3,
            DIVBUSR::_0100 => 4,
            DIVBUSR::_0101 => 5,
            DIVBUSR::_0110 => 6,
            DIVBUSR::_0111 => 7,
            DIVBUSR::_1000 => 8,
            DIVBUSR::_1001 => 9,
            DIVBUSR::_1010 => 10,
            DIVBUSR::_1011 => 11,
            DIVBUSR::_1100 => 12,
            DIVBUSR::_1101 => 13,
            DIVBUSR::_1110 => 14,
            DIVBUSR::_1111 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIVBUSR {
        match value {
            0 => DIVBUSR::_0000,
            1 => DIVBUSR::_0001,
            2 => DIVBUSR::_0010,
            3 => DIVBUSR::_0011,
            4 => DIVBUSR::_0100,
            5 => DIVBUSR::_0101,
            6 => DIVBUSR::_0110,
            7 => DIVBUSR::_0111,
            8 => DIVBUSR::_1000,
            9 => DIVBUSR::_1001,
            10 => DIVBUSR::_1010,
            11 => DIVBUSR::_1011,
            12 => DIVBUSR::_1100,
            13 => DIVBUSR::_1101,
            14 => DIVBUSR::_1110,
            15 => DIVBUSR::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == DIVBUSR::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == DIVBUSR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == DIVBUSR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == DIVBUSR::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == DIVBUSR::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == DIVBUSR::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == DIVBUSR::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == DIVBUSR::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == DIVBUSR::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == DIVBUSR::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == DIVBUSR::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == DIVBUSR::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == DIVBUSR::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == DIVBUSR::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == DIVBUSR::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == DIVBUSR::_1111
    }
}
#[doc = "Possible values of the field `DIVCORE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVCORER {
    #[doc = "Divide-by-1"]
    _0000,
    #[doc = "Divide-by-2"]
    _0001,
    #[doc = "Divide-by-3"]
    _0010,
    #[doc = "Divide-by-4"]
    _0011,
    #[doc = "Divide-by-5"]
    _0100,
    #[doc = "Divide-by-6"]
    _0101,
    #[doc = "Divide-by-7"]
    _0110,
    #[doc = "Divide-by-8"]
    _0111,
    #[doc = "Divide-by-9"]
    _1000,
    #[doc = "Divide-by-10"]
    _1001,
    #[doc = "Divide-by-11"]
    _1010,
    #[doc = "Divide-by-12"]
    _1011,
    #[doc = "Divide-by-13"]
    _1100,
    #[doc = "Divide-by-14"]
    _1101,
    #[doc = "Divide-by-15"]
    _1110,
    #[doc = "Divide-by-16"]
    _1111,
}
impl DIVCORER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DIVCORER::_0000 => 0,
            DIVCORER::_0001 => 1,
            DIVCORER::_0010 => 2,
            DIVCORER::_0011 => 3,
            DIVCORER::_0100 => 4,
            DIVCORER::_0101 => 5,
            DIVCORER::_0110 => 6,
            DIVCORER::_0111 => 7,
            DIVCORER::_1000 => 8,
            DIVCORER::_1001 => 9,
            DIVCORER::_1010 => 10,
            DIVCORER::_1011 => 11,
            DIVCORER::_1100 => 12,
            DIVCORER::_1101 => 13,
            DIVCORER::_1110 => 14,
            DIVCORER::_1111 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DIVCORER {
        match value {
            0 => DIVCORER::_0000,
            1 => DIVCORER::_0001,
            2 => DIVCORER::_0010,
            3 => DIVCORER::_0011,
            4 => DIVCORER::_0100,
            5 => DIVCORER::_0101,
            6 => DIVCORER::_0110,
            7 => DIVCORER::_0111,
            8 => DIVCORER::_1000,
            9 => DIVCORER::_1001,
            10 => DIVCORER::_1010,
            11 => DIVCORER::_1011,
            12 => DIVCORER::_1100,
            13 => DIVCORER::_1101,
            14 => DIVCORER::_1110,
            15 => DIVCORER::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline]
    pub fn is_0000(&self) -> bool {
        *self == DIVCORER::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == DIVCORER::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == DIVCORER::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == DIVCORER::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline]
    pub fn is_0100(&self) -> bool {
        *self == DIVCORER::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline]
    pub fn is_0101(&self) -> bool {
        *self == DIVCORER::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == DIVCORER::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline]
    pub fn is_0111(&self) -> bool {
        *self == DIVCORER::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline]
    pub fn is_1000(&self) -> bool {
        *self == DIVCORER::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline]
    pub fn is_1001(&self) -> bool {
        *self == DIVCORER::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline]
    pub fn is_1010(&self) -> bool {
        *self == DIVCORER::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline]
    pub fn is_1011(&self) -> bool {
        *self == DIVCORER::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline]
    pub fn is_1100(&self) -> bool {
        *self == DIVCORER::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline]
    pub fn is_1101(&self) -> bool {
        *self == DIVCORER::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline]
    pub fn is_1110(&self) -> bool {
        *self == DIVCORER::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline]
    pub fn is_1111(&self) -> bool {
        *self == DIVCORER::_1111
    }
}
#[doc = "Possible values of the field `SCS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCSR {
    #[doc = "System OSC (SOSC_CLK)"]
    _0001,
    #[doc = "Slow IRC (SIRC_CLK)"]
    _0010,
    #[doc = "Fast IRC (FIRC_CLK)"]
    _0011,
    #[doc = "System PLL (SPLL_CLK)"]
    _0110,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SCSR::_0001 => 1,
            SCSR::_0010 => 2,
            SCSR::_0011 => 3,
            SCSR::_0110 => 6,
            SCSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SCSR {
        match value {
            1 => SCSR::_0001,
            2 => SCSR::_0010,
            3 => SCSR::_0011,
            6 => SCSR::_0110,
            i => SCSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline]
    pub fn is_0001(&self) -> bool {
        *self == SCSR::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline]
    pub fn is_0010(&self) -> bool {
        *self == SCSR::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline]
    pub fn is_0011(&self) -> bool {
        *self == SCSR::_0011
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline]
    pub fn is_0110(&self) -> bool {
        *self == SCSR::_0110
    }
}
#[doc = "Values that can be written to the field `DIVSLOW`"]
pub enum DIVSLOWW {
    #[doc = "Divide-by-1"]
    _0000,
    #[doc = "Divide-by-2"]
    _0001,
    #[doc = "Divide-by-3"]
    _0010,
    #[doc = "Divide-by-4"]
    _0011,
    #[doc = "Divide-by-5"]
    _0100,
    #[doc = "Divide-by-6"]
    _0101,
    #[doc = "Divide-by-7"]
    _0110,
    #[doc = "Divide-by-8"]
    _0111,
}
impl DIVSLOWW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIVSLOWW::_0000 => 0,
            DIVSLOWW::_0001 => 1,
            DIVSLOWW::_0010 => 2,
            DIVSLOWW::_0011 => 3,
            DIVSLOWW::_0100 => 4,
            DIVSLOWW::_0101 => 5,
            DIVSLOWW::_0110 => 6,
            DIVSLOWW::_0111 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVSLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVSLOWW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVSLOWW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Divide-by-1"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DIVSLOWW::_0000)
    }
    #[doc = "Divide-by-2"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(DIVSLOWW::_0001)
    }
    #[doc = "Divide-by-3"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(DIVSLOWW::_0010)
    }
    #[doc = "Divide-by-4"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(DIVSLOWW::_0011)
    }
    #[doc = "Divide-by-5"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(DIVSLOWW::_0100)
    }
    #[doc = "Divide-by-6"]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(DIVSLOWW::_0101)
    }
    #[doc = "Divide-by-7"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(DIVSLOWW::_0110)
    }
    #[doc = "Divide-by-8"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(DIVSLOWW::_0111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIVBUS`"]
pub enum DIVBUSW {
    #[doc = "Divide-by-1"]
    _0000,
    #[doc = "Divide-by-2"]
    _0001,
    #[doc = "Divide-by-3"]
    _0010,
    #[doc = "Divide-by-4"]
    _0011,
    #[doc = "Divide-by-5"]
    _0100,
    #[doc = "Divide-by-6"]
    _0101,
    #[doc = "Divide-by-7"]
    _0110,
    #[doc = "Divide-by-8"]
    _0111,
    #[doc = "Divide-by-9"]
    _1000,
    #[doc = "Divide-by-10"]
    _1001,
    #[doc = "Divide-by-11"]
    _1010,
    #[doc = "Divide-by-12"]
    _1011,
    #[doc = "Divide-by-13"]
    _1100,
    #[doc = "Divide-by-14"]
    _1101,
    #[doc = "Divide-by-15"]
    _1110,
    #[doc = "Divide-by-16"]
    _1111,
}
impl DIVBUSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIVBUSW::_0000 => 0,
            DIVBUSW::_0001 => 1,
            DIVBUSW::_0010 => 2,
            DIVBUSW::_0011 => 3,
            DIVBUSW::_0100 => 4,
            DIVBUSW::_0101 => 5,
            DIVBUSW::_0110 => 6,
            DIVBUSW::_0111 => 7,
            DIVBUSW::_1000 => 8,
            DIVBUSW::_1001 => 9,
            DIVBUSW::_1010 => 10,
            DIVBUSW::_1011 => 11,
            DIVBUSW::_1100 => 12,
            DIVBUSW::_1101 => 13,
            DIVBUSW::_1110 => 14,
            DIVBUSW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVBUSW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVBUSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVBUSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide-by-1"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DIVBUSW::_0000)
    }
    #[doc = "Divide-by-2"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(DIVBUSW::_0001)
    }
    #[doc = "Divide-by-3"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(DIVBUSW::_0010)
    }
    #[doc = "Divide-by-4"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(DIVBUSW::_0011)
    }
    #[doc = "Divide-by-5"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(DIVBUSW::_0100)
    }
    #[doc = "Divide-by-6"]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(DIVBUSW::_0101)
    }
    #[doc = "Divide-by-7"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(DIVBUSW::_0110)
    }
    #[doc = "Divide-by-8"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(DIVBUSW::_0111)
    }
    #[doc = "Divide-by-9"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(DIVBUSW::_1000)
    }
    #[doc = "Divide-by-10"]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(DIVBUSW::_1001)
    }
    #[doc = "Divide-by-11"]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(DIVBUSW::_1010)
    }
    #[doc = "Divide-by-12"]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(DIVBUSW::_1011)
    }
    #[doc = "Divide-by-13"]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(DIVBUSW::_1100)
    }
    #[doc = "Divide-by-14"]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(DIVBUSW::_1101)
    }
    #[doc = "Divide-by-15"]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(DIVBUSW::_1110)
    }
    #[doc = "Divide-by-16"]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(DIVBUSW::_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DIVCORE`"]
pub enum DIVCOREW {
    #[doc = "Divide-by-1"]
    _0000,
    #[doc = "Divide-by-2"]
    _0001,
    #[doc = "Divide-by-3"]
    _0010,
    #[doc = "Divide-by-4"]
    _0011,
    #[doc = "Divide-by-5"]
    _0100,
    #[doc = "Divide-by-6"]
    _0101,
    #[doc = "Divide-by-7"]
    _0110,
    #[doc = "Divide-by-8"]
    _0111,
    #[doc = "Divide-by-9"]
    _1000,
    #[doc = "Divide-by-10"]
    _1001,
    #[doc = "Divide-by-11"]
    _1010,
    #[doc = "Divide-by-12"]
    _1011,
    #[doc = "Divide-by-13"]
    _1100,
    #[doc = "Divide-by-14"]
    _1101,
    #[doc = "Divide-by-15"]
    _1110,
    #[doc = "Divide-by-16"]
    _1111,
}
impl DIVCOREW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DIVCOREW::_0000 => 0,
            DIVCOREW::_0001 => 1,
            DIVCOREW::_0010 => 2,
            DIVCOREW::_0011 => 3,
            DIVCOREW::_0100 => 4,
            DIVCOREW::_0101 => 5,
            DIVCOREW::_0110 => 6,
            DIVCOREW::_0111 => 7,
            DIVCOREW::_1000 => 8,
            DIVCOREW::_1001 => 9,
            DIVCOREW::_1010 => 10,
            DIVCOREW::_1011 => 11,
            DIVCOREW::_1100 => 12,
            DIVCOREW::_1101 => 13,
            DIVCOREW::_1110 => 14,
            DIVCOREW::_1111 => 15,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIVCOREW<'a> {
    w: &'a mut W,
}
impl<'a> _DIVCOREW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIVCOREW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Divide-by-1"]
    #[inline]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DIVCOREW::_0000)
    }
    #[doc = "Divide-by-2"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(DIVCOREW::_0001)
    }
    #[doc = "Divide-by-3"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(DIVCOREW::_0010)
    }
    #[doc = "Divide-by-4"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(DIVCOREW::_0011)
    }
    #[doc = "Divide-by-5"]
    #[inline]
    pub fn _0100(self) -> &'a mut W {
        self.variant(DIVCOREW::_0100)
    }
    #[doc = "Divide-by-6"]
    #[inline]
    pub fn _0101(self) -> &'a mut W {
        self.variant(DIVCOREW::_0101)
    }
    #[doc = "Divide-by-7"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(DIVCOREW::_0110)
    }
    #[doc = "Divide-by-8"]
    #[inline]
    pub fn _0111(self) -> &'a mut W {
        self.variant(DIVCOREW::_0111)
    }
    #[doc = "Divide-by-9"]
    #[inline]
    pub fn _1000(self) -> &'a mut W {
        self.variant(DIVCOREW::_1000)
    }
    #[doc = "Divide-by-10"]
    #[inline]
    pub fn _1001(self) -> &'a mut W {
        self.variant(DIVCOREW::_1001)
    }
    #[doc = "Divide-by-11"]
    #[inline]
    pub fn _1010(self) -> &'a mut W {
        self.variant(DIVCOREW::_1010)
    }
    #[doc = "Divide-by-12"]
    #[inline]
    pub fn _1011(self) -> &'a mut W {
        self.variant(DIVCOREW::_1011)
    }
    #[doc = "Divide-by-13"]
    #[inline]
    pub fn _1100(self) -> &'a mut W {
        self.variant(DIVCOREW::_1100)
    }
    #[doc = "Divide-by-14"]
    #[inline]
    pub fn _1101(self) -> &'a mut W {
        self.variant(DIVCOREW::_1101)
    }
    #[doc = "Divide-by-15"]
    #[inline]
    pub fn _1110(self) -> &'a mut W {
        self.variant(DIVCOREW::_1110)
    }
    #[doc = "Divide-by-16"]
    #[inline]
    pub fn _1111(self) -> &'a mut W {
        self.variant(DIVCOREW::_1111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SCS`"]
pub enum SCSW {
    #[doc = "System OSC (SOSC_CLK)"]
    _0001,
    #[doc = "Slow IRC (SIRC_CLK)"]
    _0010,
    #[doc = "Fast IRC (FIRC_CLK)"]
    _0011,
    #[doc = "System PLL (SPLL_CLK)"]
    _0110,
}
impl SCSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SCSW::_0001 => 1,
            SCSW::_0010 => 2,
            SCSW::_0011 => 3,
            SCSW::_0110 => 6,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SCSW<'a> {
    w: &'a mut W,
}
impl<'a> _SCSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SCSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "System OSC (SOSC_CLK)"]
    #[inline]
    pub fn _0001(self) -> &'a mut W {
        self.variant(SCSW::_0001)
    }
    #[doc = "Slow IRC (SIRC_CLK)"]
    #[inline]
    pub fn _0010(self) -> &'a mut W {
        self.variant(SCSW::_0010)
    }
    #[doc = "Fast IRC (FIRC_CLK)"]
    #[inline]
    pub fn _0011(self) -> &'a mut W {
        self.variant(SCSW::_0011)
    }
    #[doc = "System PLL (SPLL_CLK)"]
    #[inline]
    pub fn _0110(self) -> &'a mut W {
        self.variant(SCSW::_0110)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
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
    #[doc = "Bits 0:3 - Slow Clock Divide Ratio"]
    #[inline]
    pub fn divslow(&self) -> DIVSLOWR {
        DIVSLOWR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Bus Clock Divide Ratio"]
    #[inline]
    pub fn divbus(&self) -> DIVBUSR {
        DIVBUSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Core Clock Divide Ratio"]
    #[inline]
    pub fn divcore(&self) -> DIVCORER {
        DIVCORER::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - System Clock Source"]
    #[inline]
    pub fn scs(&self) -> SCSR {
        SCSR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 50331649 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Slow Clock Divide Ratio"]
    #[inline]
    pub fn divslow(&mut self) -> _DIVSLOWW {
        _DIVSLOWW { w: self }
    }
    #[doc = "Bits 4:7 - Bus Clock Divide Ratio"]
    #[inline]
    pub fn divbus(&mut self) -> _DIVBUSW {
        _DIVBUSW { w: self }
    }
    #[doc = "Bits 16:19 - Core Clock Divide Ratio"]
    #[inline]
    pub fn divcore(&mut self) -> _DIVCOREW {
        _DIVCOREW { w: self }
    }
    #[doc = "Bits 24:27 - System Clock Source"]
    #[inline]
    pub fn scs(&mut self) -> _SCSW {
        _SCSW { w: self }
    }
}
