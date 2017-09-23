#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BAUD {
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
pub struct SBRR {
    bits: u16,
}
impl SBRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = "Possible values of the field `SBNS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SBNSR {
    #[doc = "One stop bit."] _0,
    #[doc = "Two stop bits."] _1,
}
impl SBNSR {
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
            SBNSR::_0 => false,
            SBNSR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SBNSR {
        match value {
            false => SBNSR::_0,
            true => SBNSR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SBNSR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SBNSR::_1
    }
}
#[doc = "Possible values of the field `RXEDGIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXEDGIER {
    #[doc = "Hardware interrupts from LPUART_STAT[RXEDGIF] disabled."] _0,
    #[doc = "Hardware interrupt requested when LPUART_STAT[RXEDGIF] flag is 1."] _1,
}
impl RXEDGIER {
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
            RXEDGIER::_0 => false,
            RXEDGIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RXEDGIER {
        match value {
            false => RXEDGIER::_0,
            true => RXEDGIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RXEDGIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RXEDGIER::_1
    }
}
#[doc = "Possible values of the field `LBKDIE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LBKDIER {
    #[doc = "Hardware interrupts from LPUART_STAT[LBKDIF] disabled (use polling)."] _0,
    #[doc = "Hardware interrupt requested when LPUART_STAT[LBKDIF] flag is 1."] _1,
}
impl LBKDIER {
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
            LBKDIER::_0 => false,
            LBKDIER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> LBKDIER {
        match value {
            false => LBKDIER::_0,
            true => LBKDIER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == LBKDIER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == LBKDIER::_1
    }
}
#[doc = "Possible values of the field `RESYNCDIS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESYNCDISR {
    #[doc = "Resynchronization during received data word is supported"] _0,
    #[doc = "Resynchronization during received data word is disabled"] _1,
}
impl RESYNCDISR {
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
            RESYNCDISR::_0 => false,
            RESYNCDISR::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RESYNCDISR {
        match value {
            false => RESYNCDISR::_0,
            true => RESYNCDISR::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RESYNCDISR::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RESYNCDISR::_1
    }
}
#[doc = "Possible values of the field `BOTHEDGE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BOTHEDGER {
    #[doc = "Receiver samples input data using the rising edge of the baud rate clock."] _0,
    #[doc = "Receiver samples input data using the rising and falling edge of the baud rate clock."]
    _1,
}
impl BOTHEDGER {
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
            BOTHEDGER::_0 => false,
            BOTHEDGER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> BOTHEDGER {
        match value {
            false => BOTHEDGER::_0,
            true => BOTHEDGER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == BOTHEDGER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == BOTHEDGER::_1
    }
}
#[doc = "Possible values of the field `MATCFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MATCFGR {
    #[doc = "Address Match Wakeup"] _00,
    #[doc = "Idle Match Wakeup"] _01,
    #[doc = "Match On and Match Off"] _10,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl MATCFGR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MATCFGR::_00 => 0,
            MATCFGR::_01 => 1,
            MATCFGR::_10 => 2,
            MATCFGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MATCFGR {
        match value {
            0 => MATCFGR::_00,
            1 => MATCFGR::_01,
            2 => MATCFGR::_10,
            i => MATCFGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == MATCFGR::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == MATCFGR::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == MATCFGR::_10
    }
}
#[doc = "Possible values of the field `RIDMAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIDMAER {
    #[doc = "DMA request disabled."] _0,
    #[doc = "DMA request enabled."] _1,
}
impl RIDMAER {
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
            RIDMAER::_0 => false,
            RIDMAER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RIDMAER {
        match value {
            false => RIDMAER::_0,
            true => RIDMAER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RIDMAER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RIDMAER::_1
    }
}
#[doc = "Possible values of the field `RDMAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDMAER {
    #[doc = "DMA request disabled."] _0,
    #[doc = "DMA request enabled."] _1,
}
impl RDMAER {
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
            RDMAER::_0 => false,
            RDMAER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDMAER {
        match value {
            false => RDMAER::_0,
            true => RDMAER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == RDMAER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == RDMAER::_1
    }
}
#[doc = "Possible values of the field `TDMAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDMAER {
    #[doc = "DMA request disabled."] _0,
    #[doc = "DMA request enabled."] _1,
}
impl TDMAER {
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
            TDMAER::_0 => false,
            TDMAER::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TDMAER {
        match value {
            false => TDMAER::_0,
            true => TDMAER::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TDMAER::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TDMAER::_1
    }
}
#[doc = "Possible values of the field `OSR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSRR {
    #[doc = "Writing 0 to this field will result in an oversampling ratio of 16"] _00000,
    #[doc = "Oversampling ratio of 4, requires BOTHEDGE to be set."] _00011,
    #[doc = "Oversampling ratio of 5, requires BOTHEDGE to be set."] _00100,
    #[doc = "Oversampling ratio of 6, requires BOTHEDGE to be set."] _00101,
    #[doc = "Oversampling ratio of 7, requires BOTHEDGE to be set."] _00110,
    #[doc = "Oversampling ratio of 8."] _00111,
    #[doc = "Oversampling ratio of 9."] _01000,
    #[doc = "Oversampling ratio of 10."] _01001,
    #[doc = "Oversampling ratio of 11."] _01010,
    #[doc = "Oversampling ratio of 12."] _01011,
    #[doc = "Oversampling ratio of 13."] _01100,
    #[doc = "Oversampling ratio of 14."] _01101,
    #[doc = "Oversampling ratio of 15."] _01110,
    #[doc = "Oversampling ratio of 16."] _01111,
    #[doc = "Oversampling ratio of 17."] _10000,
    #[doc = "Oversampling ratio of 18."] _10001,
    #[doc = "Oversampling ratio of 19."] _10010,
    #[doc = "Oversampling ratio of 20."] _10011,
    #[doc = "Oversampling ratio of 21."] _10100,
    #[doc = "Oversampling ratio of 22."] _10101,
    #[doc = "Oversampling ratio of 23."] _10110,
    #[doc = "Oversampling ratio of 24."] _10111,
    #[doc = "Oversampling ratio of 25."] _11000,
    #[doc = "Oversampling ratio of 26."] _11001,
    #[doc = "Oversampling ratio of 27."] _11010,
    #[doc = "Oversampling ratio of 28."] _11011,
    #[doc = "Oversampling ratio of 29."] _11100,
    #[doc = "Oversampling ratio of 30."] _11101,
    #[doc = "Oversampling ratio of 31."] _11110,
    #[doc = "Oversampling ratio of 32."] _11111,
    #[doc = r" Reserved"] _Reserved(u8),
}
impl OSRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OSRR::_00000 => 0,
            OSRR::_00011 => 3,
            OSRR::_00100 => 4,
            OSRR::_00101 => 5,
            OSRR::_00110 => 6,
            OSRR::_00111 => 7,
            OSRR::_01000 => 8,
            OSRR::_01001 => 9,
            OSRR::_01010 => 10,
            OSRR::_01011 => 11,
            OSRR::_01100 => 12,
            OSRR::_01101 => 13,
            OSRR::_01110 => 14,
            OSRR::_01111 => 15,
            OSRR::_10000 => 16,
            OSRR::_10001 => 17,
            OSRR::_10010 => 18,
            OSRR::_10011 => 19,
            OSRR::_10100 => 20,
            OSRR::_10101 => 21,
            OSRR::_10110 => 22,
            OSRR::_10111 => 23,
            OSRR::_11000 => 24,
            OSRR::_11001 => 25,
            OSRR::_11010 => 26,
            OSRR::_11011 => 27,
            OSRR::_11100 => 28,
            OSRR::_11101 => 29,
            OSRR::_11110 => 30,
            OSRR::_11111 => 31,
            OSRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OSRR {
        match value {
            0 => OSRR::_00000,
            3 => OSRR::_00011,
            4 => OSRR::_00100,
            5 => OSRR::_00101,
            6 => OSRR::_00110,
            7 => OSRR::_00111,
            8 => OSRR::_01000,
            9 => OSRR::_01001,
            10 => OSRR::_01010,
            11 => OSRR::_01011,
            12 => OSRR::_01100,
            13 => OSRR::_01101,
            14 => OSRR::_01110,
            15 => OSRR::_01111,
            16 => OSRR::_10000,
            17 => OSRR::_10001,
            18 => OSRR::_10010,
            19 => OSRR::_10011,
            20 => OSRR::_10100,
            21 => OSRR::_10101,
            22 => OSRR::_10110,
            23 => OSRR::_10111,
            24 => OSRR::_11000,
            25 => OSRR::_11001,
            26 => OSRR::_11010,
            27 => OSRR::_11011,
            28 => OSRR::_11100,
            29 => OSRR::_11101,
            30 => OSRR::_11110,
            31 => OSRR::_11111,
            i => OSRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline]
    pub fn is_00000(&self) -> bool {
        *self == OSRR::_00000
    }
    #[doc = "Checks if the value of the field is `_00011`"]
    #[inline]
    pub fn is_00011(&self) -> bool {
        *self == OSRR::_00011
    }
    #[doc = "Checks if the value of the field is `_00100`"]
    #[inline]
    pub fn is_00100(&self) -> bool {
        *self == OSRR::_00100
    }
    #[doc = "Checks if the value of the field is `_00101`"]
    #[inline]
    pub fn is_00101(&self) -> bool {
        *self == OSRR::_00101
    }
    #[doc = "Checks if the value of the field is `_00110`"]
    #[inline]
    pub fn is_00110(&self) -> bool {
        *self == OSRR::_00110
    }
    #[doc = "Checks if the value of the field is `_00111`"]
    #[inline]
    pub fn is_00111(&self) -> bool {
        *self == OSRR::_00111
    }
    #[doc = "Checks if the value of the field is `_01000`"]
    #[inline]
    pub fn is_01000(&self) -> bool {
        *self == OSRR::_01000
    }
    #[doc = "Checks if the value of the field is `_01001`"]
    #[inline]
    pub fn is_01001(&self) -> bool {
        *self == OSRR::_01001
    }
    #[doc = "Checks if the value of the field is `_01010`"]
    #[inline]
    pub fn is_01010(&self) -> bool {
        *self == OSRR::_01010
    }
    #[doc = "Checks if the value of the field is `_01011`"]
    #[inline]
    pub fn is_01011(&self) -> bool {
        *self == OSRR::_01011
    }
    #[doc = "Checks if the value of the field is `_01100`"]
    #[inline]
    pub fn is_01100(&self) -> bool {
        *self == OSRR::_01100
    }
    #[doc = "Checks if the value of the field is `_01101`"]
    #[inline]
    pub fn is_01101(&self) -> bool {
        *self == OSRR::_01101
    }
    #[doc = "Checks if the value of the field is `_01110`"]
    #[inline]
    pub fn is_01110(&self) -> bool {
        *self == OSRR::_01110
    }
    #[doc = "Checks if the value of the field is `_01111`"]
    #[inline]
    pub fn is_01111(&self) -> bool {
        *self == OSRR::_01111
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline]
    pub fn is_10000(&self) -> bool {
        *self == OSRR::_10000
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline]
    pub fn is_10001(&self) -> bool {
        *self == OSRR::_10001
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline]
    pub fn is_10010(&self) -> bool {
        *self == OSRR::_10010
    }
    #[doc = "Checks if the value of the field is `_10011`"]
    #[inline]
    pub fn is_10011(&self) -> bool {
        *self == OSRR::_10011
    }
    #[doc = "Checks if the value of the field is `_10100`"]
    #[inline]
    pub fn is_10100(&self) -> bool {
        *self == OSRR::_10100
    }
    #[doc = "Checks if the value of the field is `_10101`"]
    #[inline]
    pub fn is_10101(&self) -> bool {
        *self == OSRR::_10101
    }
    #[doc = "Checks if the value of the field is `_10110`"]
    #[inline]
    pub fn is_10110(&self) -> bool {
        *self == OSRR::_10110
    }
    #[doc = "Checks if the value of the field is `_10111`"]
    #[inline]
    pub fn is_10111(&self) -> bool {
        *self == OSRR::_10111
    }
    #[doc = "Checks if the value of the field is `_11000`"]
    #[inline]
    pub fn is_11000(&self) -> bool {
        *self == OSRR::_11000
    }
    #[doc = "Checks if the value of the field is `_11001`"]
    #[inline]
    pub fn is_11001(&self) -> bool {
        *self == OSRR::_11001
    }
    #[doc = "Checks if the value of the field is `_11010`"]
    #[inline]
    pub fn is_11010(&self) -> bool {
        *self == OSRR::_11010
    }
    #[doc = "Checks if the value of the field is `_11011`"]
    #[inline]
    pub fn is_11011(&self) -> bool {
        *self == OSRR::_11011
    }
    #[doc = "Checks if the value of the field is `_11100`"]
    #[inline]
    pub fn is_11100(&self) -> bool {
        *self == OSRR::_11100
    }
    #[doc = "Checks if the value of the field is `_11101`"]
    #[inline]
    pub fn is_11101(&self) -> bool {
        *self == OSRR::_11101
    }
    #[doc = "Checks if the value of the field is `_11110`"]
    #[inline]
    pub fn is_11110(&self) -> bool {
        *self == OSRR::_11110
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline]
    pub fn is_11111(&self) -> bool {
        *self == OSRR::_11111
    }
}
#[doc = "Possible values of the field `M10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M10R {
    #[doc = "Receiver and transmitter use 7-bit to 9-bit data characters."] _0,
    #[doc = "Receiver and transmitter use 10-bit data characters."] _1,
}
impl M10R {
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
            M10R::_0 => false,
            M10R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> M10R {
        match value {
            false => M10R::_0,
            true => M10R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == M10R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == M10R::_1
    }
}
#[doc = "Possible values of the field `MAEN2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAEN2R {
    #[doc = "Normal operation."] _0,
    #[doc = "Enables automatic address matching or data matching mode for MATCH[MA2]."] _1,
}
impl MAEN2R {
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
            MAEN2R::_0 => false,
            MAEN2R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAEN2R {
        match value {
            false => MAEN2R::_0,
            true => MAEN2R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MAEN2R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MAEN2R::_1
    }
}
#[doc = "Possible values of the field `MAEN1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAEN1R {
    #[doc = "Normal operation."] _0,
    #[doc = "Enables automatic address matching or data matching mode for MATCH[MA1]."] _1,
}
impl MAEN1R {
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
            MAEN1R::_0 => false,
            MAEN1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MAEN1R {
        match value {
            false => MAEN1R::_0,
            true => MAEN1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == MAEN1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == MAEN1R::_1
    }
}
#[doc = r" Proxy"]
pub struct _SBRW<'a> {
    w: &'a mut W,
}
impl<'a> _SBRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 8191;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SBNS`"]
pub enum SBNSW {
    #[doc = "One stop bit."] _0,
    #[doc = "Two stop bits."] _1,
}
impl SBNSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SBNSW::_0 => false,
            SBNSW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SBNSW<'a> {
    w: &'a mut W,
}
impl<'a> _SBNSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SBNSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "One stop bit."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBNSW::_0)
    }
    #[doc = "Two stop bits."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBNSW::_1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RXEDGIE`"]
pub enum RXEDGIEW {
    #[doc = "Hardware interrupts from LPUART_STAT[RXEDGIF] disabled."] _0,
    #[doc = "Hardware interrupt requested when LPUART_STAT[RXEDGIF] flag is 1."] _1,
}
impl RXEDGIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RXEDGIEW::_0 => false,
            RXEDGIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RXEDGIEW<'a> {
    w: &'a mut W,
}
impl<'a> _RXEDGIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RXEDGIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware interrupts from LPUART_STAT[RXEDGIF] disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXEDGIEW::_0)
    }
    #[doc = "Hardware interrupt requested when LPUART_STAT[RXEDGIF] flag is 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXEDGIEW::_1)
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
#[doc = "Values that can be written to the field `LBKDIE`"]
pub enum LBKDIEW {
    #[doc = "Hardware interrupts from LPUART_STAT[LBKDIF] disabled (use polling)."] _0,
    #[doc = "Hardware interrupt requested when LPUART_STAT[LBKDIF] flag is 1."] _1,
}
impl LBKDIEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            LBKDIEW::_0 => false,
            LBKDIEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LBKDIEW<'a> {
    w: &'a mut W,
}
impl<'a> _LBKDIEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LBKDIEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware interrupts from LPUART_STAT[LBKDIF] disabled (use polling)."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDIEW::_0)
    }
    #[doc = "Hardware interrupt requested when LPUART_STAT[LBKDIF] flag is 1."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDIEW::_1)
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
#[doc = "Values that can be written to the field `RESYNCDIS`"]
pub enum RESYNCDISW {
    #[doc = "Resynchronization during received data word is supported"] _0,
    #[doc = "Resynchronization during received data word is disabled"] _1,
}
impl RESYNCDISW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RESYNCDISW::_0 => false,
            RESYNCDISW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESYNCDISW<'a> {
    w: &'a mut W,
}
impl<'a> _RESYNCDISW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESYNCDISW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Resynchronization during received data word is supported"]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESYNCDISW::_0)
    }
    #[doc = "Resynchronization during received data word is disabled"]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESYNCDISW::_1)
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
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BOTHEDGE`"]
pub enum BOTHEDGEW {
    #[doc = "Receiver samples input data using the rising edge of the baud rate clock."] _0,
    #[doc = "Receiver samples input data using the rising and falling edge of the baud rate clock."]
    _1,
}
impl BOTHEDGEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BOTHEDGEW::_0 => false,
            BOTHEDGEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BOTHEDGEW<'a> {
    w: &'a mut W,
}
impl<'a> _BOTHEDGEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BOTHEDGEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver samples input data using the rising edge of the baud rate clock."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOTHEDGEW::_0)
    }
    #[doc = "Receiver samples input data using the rising and falling edge of the baud rate clock."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOTHEDGEW::_1)
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
#[doc = "Values that can be written to the field `MATCFG`"]
pub enum MATCFGW {
    #[doc = "Address Match Wakeup"] _00,
    #[doc = "Idle Match Wakeup"] _01,
    #[doc = "Match On and Match Off"] _10,
}
impl MATCFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MATCFGW::_00 => 0,
            MATCFGW::_01 => 1,
            MATCFGW::_10 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MATCFGW<'a> {
    w: &'a mut W,
}
impl<'a> _MATCFGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MATCFGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Address Match Wakeup"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(MATCFGW::_00)
    }
    #[doc = "Idle Match Wakeup"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(MATCFGW::_01)
    }
    #[doc = "Match On and Match Off"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(MATCFGW::_10)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RIDMAE`"]
pub enum RIDMAEW {
    #[doc = "DMA request disabled."] _0,
    #[doc = "DMA request enabled."] _1,
}
impl RIDMAEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RIDMAEW::_0 => false,
            RIDMAEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RIDMAEW<'a> {
    w: &'a mut W,
}
impl<'a> _RIDMAEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RIDMAEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RIDMAEW::_0)
    }
    #[doc = "DMA request enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RIDMAEW::_1)
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
#[doc = "Values that can be written to the field `RDMAE`"]
pub enum RDMAEW {
    #[doc = "DMA request disabled."] _0,
    #[doc = "DMA request enabled."] _1,
}
impl RDMAEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDMAEW::_0 => false,
            RDMAEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDMAEW<'a> {
    w: &'a mut W,
}
impl<'a> _RDMAEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDMAEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDMAEW::_0)
    }
    #[doc = "DMA request enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDMAEW::_1)
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
#[doc = "Values that can be written to the field `TDMAE`"]
pub enum TDMAEW {
    #[doc = "DMA request disabled."] _0,
    #[doc = "DMA request enabled."] _1,
}
impl TDMAEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TDMAEW::_0 => false,
            TDMAEW::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TDMAEW<'a> {
    w: &'a mut W,
}
impl<'a> _TDMAEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TDMAEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "DMA request disabled."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDMAEW::_0)
    }
    #[doc = "DMA request enabled."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDMAEW::_1)
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
#[doc = "Values that can be written to the field `OSR`"]
pub enum OSRW {
    #[doc = "Writing 0 to this field will result in an oversampling ratio of 16"] _00000,
    #[doc = "Oversampling ratio of 4, requires BOTHEDGE to be set."] _00011,
    #[doc = "Oversampling ratio of 5, requires BOTHEDGE to be set."] _00100,
    #[doc = "Oversampling ratio of 6, requires BOTHEDGE to be set."] _00101,
    #[doc = "Oversampling ratio of 7, requires BOTHEDGE to be set."] _00110,
    #[doc = "Oversampling ratio of 8."] _00111,
    #[doc = "Oversampling ratio of 9."] _01000,
    #[doc = "Oversampling ratio of 10."] _01001,
    #[doc = "Oversampling ratio of 11."] _01010,
    #[doc = "Oversampling ratio of 12."] _01011,
    #[doc = "Oversampling ratio of 13."] _01100,
    #[doc = "Oversampling ratio of 14."] _01101,
    #[doc = "Oversampling ratio of 15."] _01110,
    #[doc = "Oversampling ratio of 16."] _01111,
    #[doc = "Oversampling ratio of 17."] _10000,
    #[doc = "Oversampling ratio of 18."] _10001,
    #[doc = "Oversampling ratio of 19."] _10010,
    #[doc = "Oversampling ratio of 20."] _10011,
    #[doc = "Oversampling ratio of 21."] _10100,
    #[doc = "Oversampling ratio of 22."] _10101,
    #[doc = "Oversampling ratio of 23."] _10110,
    #[doc = "Oversampling ratio of 24."] _10111,
    #[doc = "Oversampling ratio of 25."] _11000,
    #[doc = "Oversampling ratio of 26."] _11001,
    #[doc = "Oversampling ratio of 27."] _11010,
    #[doc = "Oversampling ratio of 28."] _11011,
    #[doc = "Oversampling ratio of 29."] _11100,
    #[doc = "Oversampling ratio of 30."] _11101,
    #[doc = "Oversampling ratio of 31."] _11110,
    #[doc = "Oversampling ratio of 32."] _11111,
}
impl OSRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            OSRW::_00000 => 0,
            OSRW::_00011 => 3,
            OSRW::_00100 => 4,
            OSRW::_00101 => 5,
            OSRW::_00110 => 6,
            OSRW::_00111 => 7,
            OSRW::_01000 => 8,
            OSRW::_01001 => 9,
            OSRW::_01010 => 10,
            OSRW::_01011 => 11,
            OSRW::_01100 => 12,
            OSRW::_01101 => 13,
            OSRW::_01110 => 14,
            OSRW::_01111 => 15,
            OSRW::_10000 => 16,
            OSRW::_10001 => 17,
            OSRW::_10010 => 18,
            OSRW::_10011 => 19,
            OSRW::_10100 => 20,
            OSRW::_10101 => 21,
            OSRW::_10110 => 22,
            OSRW::_10111 => 23,
            OSRW::_11000 => 24,
            OSRW::_11001 => 25,
            OSRW::_11010 => 26,
            OSRW::_11011 => 27,
            OSRW::_11100 => 28,
            OSRW::_11101 => 29,
            OSRW::_11110 => 30,
            OSRW::_11111 => 31,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OSRW<'a> {
    w: &'a mut W,
}
impl<'a> _OSRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OSRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Writing 0 to this field will result in an oversampling ratio of 16"]
    #[inline]
    pub fn _00000(self) -> &'a mut W {
        self.variant(OSRW::_00000)
    }
    #[doc = "Oversampling ratio of 4, requires BOTHEDGE to be set."]
    #[inline]
    pub fn _00011(self) -> &'a mut W {
        self.variant(OSRW::_00011)
    }
    #[doc = "Oversampling ratio of 5, requires BOTHEDGE to be set."]
    #[inline]
    pub fn _00100(self) -> &'a mut W {
        self.variant(OSRW::_00100)
    }
    #[doc = "Oversampling ratio of 6, requires BOTHEDGE to be set."]
    #[inline]
    pub fn _00101(self) -> &'a mut W {
        self.variant(OSRW::_00101)
    }
    #[doc = "Oversampling ratio of 7, requires BOTHEDGE to be set."]
    #[inline]
    pub fn _00110(self) -> &'a mut W {
        self.variant(OSRW::_00110)
    }
    #[doc = "Oversampling ratio of 8."]
    #[inline]
    pub fn _00111(self) -> &'a mut W {
        self.variant(OSRW::_00111)
    }
    #[doc = "Oversampling ratio of 9."]
    #[inline]
    pub fn _01000(self) -> &'a mut W {
        self.variant(OSRW::_01000)
    }
    #[doc = "Oversampling ratio of 10."]
    #[inline]
    pub fn _01001(self) -> &'a mut W {
        self.variant(OSRW::_01001)
    }
    #[doc = "Oversampling ratio of 11."]
    #[inline]
    pub fn _01010(self) -> &'a mut W {
        self.variant(OSRW::_01010)
    }
    #[doc = "Oversampling ratio of 12."]
    #[inline]
    pub fn _01011(self) -> &'a mut W {
        self.variant(OSRW::_01011)
    }
    #[doc = "Oversampling ratio of 13."]
    #[inline]
    pub fn _01100(self) -> &'a mut W {
        self.variant(OSRW::_01100)
    }
    #[doc = "Oversampling ratio of 14."]
    #[inline]
    pub fn _01101(self) -> &'a mut W {
        self.variant(OSRW::_01101)
    }
    #[doc = "Oversampling ratio of 15."]
    #[inline]
    pub fn _01110(self) -> &'a mut W {
        self.variant(OSRW::_01110)
    }
    #[doc = "Oversampling ratio of 16."]
    #[inline]
    pub fn _01111(self) -> &'a mut W {
        self.variant(OSRW::_01111)
    }
    #[doc = "Oversampling ratio of 17."]
    #[inline]
    pub fn _10000(self) -> &'a mut W {
        self.variant(OSRW::_10000)
    }
    #[doc = "Oversampling ratio of 18."]
    #[inline]
    pub fn _10001(self) -> &'a mut W {
        self.variant(OSRW::_10001)
    }
    #[doc = "Oversampling ratio of 19."]
    #[inline]
    pub fn _10010(self) -> &'a mut W {
        self.variant(OSRW::_10010)
    }
    #[doc = "Oversampling ratio of 20."]
    #[inline]
    pub fn _10011(self) -> &'a mut W {
        self.variant(OSRW::_10011)
    }
    #[doc = "Oversampling ratio of 21."]
    #[inline]
    pub fn _10100(self) -> &'a mut W {
        self.variant(OSRW::_10100)
    }
    #[doc = "Oversampling ratio of 22."]
    #[inline]
    pub fn _10101(self) -> &'a mut W {
        self.variant(OSRW::_10101)
    }
    #[doc = "Oversampling ratio of 23."]
    #[inline]
    pub fn _10110(self) -> &'a mut W {
        self.variant(OSRW::_10110)
    }
    #[doc = "Oversampling ratio of 24."]
    #[inline]
    pub fn _10111(self) -> &'a mut W {
        self.variant(OSRW::_10111)
    }
    #[doc = "Oversampling ratio of 25."]
    #[inline]
    pub fn _11000(self) -> &'a mut W {
        self.variant(OSRW::_11000)
    }
    #[doc = "Oversampling ratio of 26."]
    #[inline]
    pub fn _11001(self) -> &'a mut W {
        self.variant(OSRW::_11001)
    }
    #[doc = "Oversampling ratio of 27."]
    #[inline]
    pub fn _11010(self) -> &'a mut W {
        self.variant(OSRW::_11010)
    }
    #[doc = "Oversampling ratio of 28."]
    #[inline]
    pub fn _11011(self) -> &'a mut W {
        self.variant(OSRW::_11011)
    }
    #[doc = "Oversampling ratio of 29."]
    #[inline]
    pub fn _11100(self) -> &'a mut W {
        self.variant(OSRW::_11100)
    }
    #[doc = "Oversampling ratio of 30."]
    #[inline]
    pub fn _11101(self) -> &'a mut W {
        self.variant(OSRW::_11101)
    }
    #[doc = "Oversampling ratio of 31."]
    #[inline]
    pub fn _11110(self) -> &'a mut W {
        self.variant(OSRW::_11110)
    }
    #[doc = "Oversampling ratio of 32."]
    #[inline]
    pub fn _11111(self) -> &'a mut W {
        self.variant(OSRW::_11111)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `M10`"]
pub enum M10W {
    #[doc = "Receiver and transmitter use 7-bit to 9-bit data characters."] _0,
    #[doc = "Receiver and transmitter use 10-bit data characters."] _1,
}
impl M10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            M10W::_0 => false,
            M10W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _M10W<'a> {
    w: &'a mut W,
}
impl<'a> _M10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: M10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Receiver and transmitter use 7-bit to 9-bit data characters."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(M10W::_0)
    }
    #[doc = "Receiver and transmitter use 10-bit data characters."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(M10W::_1)
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MAEN2`"]
pub enum MAEN2W {
    #[doc = "Normal operation."] _0,
    #[doc = "Enables automatic address matching or data matching mode for MATCH[MA2]."] _1,
}
impl MAEN2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAEN2W::_0 => false,
            MAEN2W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _MAEN2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAEN2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAEN2W::_0)
    }
    #[doc = "Enables automatic address matching or data matching mode for MATCH[MA2]."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAEN2W::_1)
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
#[doc = "Values that can be written to the field `MAEN1`"]
pub enum MAEN1W {
    #[doc = "Normal operation."] _0,
    #[doc = "Enables automatic address matching or data matching mode for MATCH[MA1]."] _1,
}
impl MAEN1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAEN1W::_0 => false,
            MAEN1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _MAEN1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAEN1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAEN1W::_0)
    }
    #[doc = "Enables automatic address matching or data matching mode for MATCH[MA1]."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAEN1W::_1)
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
    #[doc = "Bits 0:12 - Baud Rate Modulo Divisor."]
    #[inline]
    pub fn sbr(&self) -> SBRR {
        let bits = {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        SBRR { bits }
    }
    #[doc = "Bit 13 - Stop Bit Number Select"]
    #[inline]
    pub fn sbns(&self) -> SBNSR {
        SBNSR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - RX Input Active Edge Interrupt Enable"]
    #[inline]
    pub fn rxedgie(&self) -> RXEDGIER {
        RXEDGIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 15 - LIN Break Detect Interrupt Enable"]
    #[inline]
    pub fn lbkdie(&self) -> LBKDIER {
        LBKDIER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Resynchronization Disable"]
    #[inline]
    pub fn resyncdis(&self) -> RESYNCDISR {
        RESYNCDISR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Both Edge Sampling"]
    #[inline]
    pub fn bothedge(&self) -> BOTHEDGER {
        BOTHEDGER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 18:19 - Match Configuration"]
    #[inline]
    pub fn matcfg(&self) -> MATCFGR {
        MATCFGR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 20 - Receiver Idle DMA Enable"]
    #[inline]
    pub fn ridmae(&self) -> RIDMAER {
        RIDMAER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 21 - Receiver Full DMA Enable"]
    #[inline]
    pub fn rdmae(&self) -> RDMAER {
        RDMAER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Transmitter DMA Enable"]
    #[inline]
    pub fn tdmae(&self) -> TDMAER {
        TDMAER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:28 - Oversampling Ratio"]
    #[inline]
    pub fn osr(&self) -> OSRR {
        OSRR::_from({
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 29 - 10-bit Mode select"]
    #[inline]
    pub fn m10(&self) -> M10R {
        M10R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 30 - Match Address Mode Enable 2"]
    #[inline]
    pub fn maen2(&self) -> MAEN2R {
        MAEN2R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 31 - Match Address Mode Enable 1"]
    #[inline]
    pub fn maen1(&self) -> MAEN1R {
        MAEN1R::_from({
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
        W { bits: 251658244 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:12 - Baud Rate Modulo Divisor."]
    #[inline]
    pub fn sbr(&mut self) -> _SBRW {
        _SBRW { w: self }
    }
    #[doc = "Bit 13 - Stop Bit Number Select"]
    #[inline]
    pub fn sbns(&mut self) -> _SBNSW {
        _SBNSW { w: self }
    }
    #[doc = "Bit 14 - RX Input Active Edge Interrupt Enable"]
    #[inline]
    pub fn rxedgie(&mut self) -> _RXEDGIEW {
        _RXEDGIEW { w: self }
    }
    #[doc = "Bit 15 - LIN Break Detect Interrupt Enable"]
    #[inline]
    pub fn lbkdie(&mut self) -> _LBKDIEW {
        _LBKDIEW { w: self }
    }
    #[doc = "Bit 16 - Resynchronization Disable"]
    #[inline]
    pub fn resyncdis(&mut self) -> _RESYNCDISW {
        _RESYNCDISW { w: self }
    }
    #[doc = "Bit 17 - Both Edge Sampling"]
    #[inline]
    pub fn bothedge(&mut self) -> _BOTHEDGEW {
        _BOTHEDGEW { w: self }
    }
    #[doc = "Bits 18:19 - Match Configuration"]
    #[inline]
    pub fn matcfg(&mut self) -> _MATCFGW {
        _MATCFGW { w: self }
    }
    #[doc = "Bit 20 - Receiver Idle DMA Enable"]
    #[inline]
    pub fn ridmae(&mut self) -> _RIDMAEW {
        _RIDMAEW { w: self }
    }
    #[doc = "Bit 21 - Receiver Full DMA Enable"]
    #[inline]
    pub fn rdmae(&mut self) -> _RDMAEW {
        _RDMAEW { w: self }
    }
    #[doc = "Bit 23 - Transmitter DMA Enable"]
    #[inline]
    pub fn tdmae(&mut self) -> _TDMAEW {
        _TDMAEW { w: self }
    }
    #[doc = "Bits 24:28 - Oversampling Ratio"]
    #[inline]
    pub fn osr(&mut self) -> _OSRW {
        _OSRW { w: self }
    }
    #[doc = "Bit 29 - 10-bit Mode select"]
    #[inline]
    pub fn m10(&mut self) -> _M10W {
        _M10W { w: self }
    }
    #[doc = "Bit 30 - Match Address Mode Enable 2"]
    #[inline]
    pub fn maen2(&mut self) -> _MAEN2W {
        _MAEN2W { w: self }
    }
    #[doc = "Bit 31 - Match Address Mode Enable 1"]
    #[inline]
    pub fn maen1(&mut self) -> _MAEN1W {
        _MAEN1W { w: self }
    }
}
