#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCCRMR {
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
#[doc = "Possible values of the field `R15`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R15R {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R15R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            R15R::_00 => 0,
            R15R::_01 => 1,
            R15R::_10 => 2,
            R15R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> R15R {
        match value {
            0 => R15R::_00,
            1 => R15R::_01,
            2 => R15R::_10,
            3 => R15R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == R15R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == R15R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == R15R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == R15R::_11
    }
}
#[doc = "Possible values of the field `R14`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R14R {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R14R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            R14R::_00 => 0,
            R14R::_01 => 1,
            R14R::_10 => 2,
            R14R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> R14R {
        match value {
            0 => R14R::_00,
            1 => R14R::_01,
            2 => R14R::_10,
            3 => R14R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == R14R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == R14R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == R14R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == R14R::_11
    }
}
#[doc = "Possible values of the field `R13`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R13R {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R13R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            R13R::_00 => 0,
            R13R::_01 => 1,
            R13R::_10 => 2,
            R13R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> R13R {
        match value {
            0 => R13R::_00,
            1 => R13R::_01,
            2 => R13R::_10,
            3 => R13R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == R13R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == R13R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == R13R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == R13R::_11
    }
}
#[doc = "Possible values of the field `R12`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R12R {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R12R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            R12R::_00 => 0,
            R12R::_01 => 1,
            R12R::_10 => 2,
            R12R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> R12R {
        match value {
            0 => R12R::_00,
            1 => R12R::_01,
            2 => R12R::_10,
            3 => R12R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == R12R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == R12R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == R12R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == R12R::_11
    }
}
#[doc = "Possible values of the field `R11`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R11R {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R11R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            R11R::_00 => 0,
            R11R::_01 => 1,
            R11R::_10 => 2,
            R11R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> R11R {
        match value {
            0 => R11R::_00,
            1 => R11R::_01,
            2 => R11R::_10,
            3 => R11R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == R11R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == R11R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == R11R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == R11R::_11
    }
}
#[doc = "Possible values of the field `R10`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R10R {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R10R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            R10R::_00 => 0,
            R10R::_01 => 1,
            R10R::_10 => 2,
            R10R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> R10R {
        match value {
            0 => R10R::_00,
            1 => R10R::_01,
            2 => R10R::_10,
            3 => R10R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == R10R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == R10R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == R10R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == R10R::_11
    }
}
#[doc = "Possible values of the field `R9`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R9R {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R9R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            R9R::_00 => 0,
            R9R::_01 => 1,
            R9R::_10 => 2,
            R9R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> R9R {
        match value {
            0 => R9R::_00,
            1 => R9R::_01,
            2 => R9R::_10,
            3 => R9R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == R9R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == R9R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == R9R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == R9R::_11
    }
}
#[doc = "Possible values of the field `R8`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R8R {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R8R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            R8R::_00 => 0,
            R8R::_01 => 1,
            R8R::_10 => 2,
            R8R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> R8R {
        match value {
            0 => R8R::_00,
            1 => R8R::_01,
            2 => R8R::_10,
            3 => R8R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == R8R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == R8R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == R8R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == R8R::_11
    }
}
#[doc = "Possible values of the field `R7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R7R {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            R7R::_00 => 0,
            R7R::_01 => 1,
            R7R::_10 => 2,
            R7R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> R7R {
        match value {
            0 => R7R::_00,
            1 => R7R::_01,
            2 => R7R::_10,
            3 => R7R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == R7R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == R7R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == R7R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == R7R::_11
    }
}
#[doc = "Possible values of the field `R6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R6R {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            R6R::_00 => 0,
            R6R::_01 => 1,
            R6R::_10 => 2,
            R6R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> R6R {
        match value {
            0 => R6R::_00,
            1 => R6R::_01,
            2 => R6R::_10,
            3 => R6R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == R6R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == R6R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == R6R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == R6R::_11
    }
}
#[doc = "Possible values of the field `R5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R5R {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            R5R::_00 => 0,
            R5R::_01 => 1,
            R5R::_10 => 2,
            R5R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> R5R {
        match value {
            0 => R5R::_00,
            1 => R5R::_01,
            2 => R5R::_10,
            3 => R5R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == R5R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == R5R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == R5R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == R5R::_11
    }
}
#[doc = "Possible values of the field `R4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R4R {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            R4R::_00 => 0,
            R4R::_01 => 1,
            R4R::_10 => 2,
            R4R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> R4R {
        match value {
            0 => R4R::_00,
            1 => R4R::_01,
            2 => R4R::_10,
            3 => R4R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == R4R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == R4R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == R4R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == R4R::_11
    }
}
#[doc = "Possible values of the field `R3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R3R {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            R3R::_00 => 0,
            R3R::_01 => 1,
            R3R::_10 => 2,
            R3R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> R3R {
        match value {
            0 => R3R::_00,
            1 => R3R::_01,
            2 => R3R::_10,
            3 => R3R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == R3R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == R3R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == R3R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == R3R::_11
    }
}
#[doc = "Possible values of the field `R2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R2R {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            R2R::_00 => 0,
            R2R::_01 => 1,
            R2R::_10 => 2,
            R2R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> R2R {
        match value {
            0 => R2R::_00,
            1 => R2R::_01,
            2 => R2R::_10,
            3 => R2R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == R2R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == R2R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == R2R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == R2R::_11
    }
}
#[doc = "Possible values of the field `R1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R1R {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            R1R::_00 => 0,
            R1R::_01 => 1,
            R1R::_10 => 2,
            R1R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> R1R {
        match value {
            0 => R1R::_00,
            1 => R1R::_01,
            2 => R1R::_10,
            3 => R1R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == R1R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == R1R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == R1R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == R1R::_11
    }
}
#[doc = "Possible values of the field `R0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum R0R {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            R0R::_00 => 0,
            R0R::_01 => 1,
            R0R::_10 => 2,
            R0R::_11 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> R0R {
        match value {
            0 => R0R::_00,
            1 => R0R::_01,
            2 => R0R::_10,
            3 => R0R::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline]
    pub fn is_00(&self) -> bool {
        *self == R0R::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline]
    pub fn is_01(&self) -> bool {
        *self == R0R::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline]
    pub fn is_10(&self) -> bool {
        *self == R0R::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline]
    pub fn is_11(&self) -> bool {
        *self == R0R::_11
    }
}
#[doc = "Values that can be written to the field `R15`"]
pub enum R15W {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R15W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            R15W::_00 => 0,
            R15W::_01 => 1,
            R15W::_10 => 2,
            R15W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _R15W<'a> {
    w: &'a mut W,
}
impl<'a> _R15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: R15W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(R15W::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(R15W::_01)
    }
    #[doc = "Write-through"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(R15W::_10)
    }
    #[doc = "Write-back"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(R15W::_11)
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
#[doc = "Values that can be written to the field `R14`"]
pub enum R14W {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R14W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            R14W::_00 => 0,
            R14W::_01 => 1,
            R14W::_10 => 2,
            R14W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _R14W<'a> {
    w: &'a mut W,
}
impl<'a> _R14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: R14W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(R14W::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(R14W::_01)
    }
    #[doc = "Write-through"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(R14W::_10)
    }
    #[doc = "Write-back"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(R14W::_11)
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
#[doc = "Values that can be written to the field `R13`"]
pub enum R13W {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R13W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            R13W::_00 => 0,
            R13W::_01 => 1,
            R13W::_10 => 2,
            R13W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _R13W<'a> {
    w: &'a mut W,
}
impl<'a> _R13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: R13W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(R13W::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(R13W::_01)
    }
    #[doc = "Write-through"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(R13W::_10)
    }
    #[doc = "Write-back"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(R13W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `R12`"]
pub enum R12W {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R12W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            R12W::_00 => 0,
            R12W::_01 => 1,
            R12W::_10 => 2,
            R12W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _R12W<'a> {
    w: &'a mut W,
}
impl<'a> _R12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: R12W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(R12W::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(R12W::_01)
    }
    #[doc = "Write-through"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(R12W::_10)
    }
    #[doc = "Write-back"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(R12W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `R11`"]
pub enum R11W {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R11W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            R11W::_00 => 0,
            R11W::_01 => 1,
            R11W::_10 => 2,
            R11W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _R11W<'a> {
    w: &'a mut W,
}
impl<'a> _R11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: R11W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(R11W::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(R11W::_01)
    }
    #[doc = "Write-through"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(R11W::_10)
    }
    #[doc = "Write-back"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(R11W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `R10`"]
pub enum R10W {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            R10W::_00 => 0,
            R10W::_01 => 1,
            R10W::_10 => 2,
            R10W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _R10W<'a> {
    w: &'a mut W,
}
impl<'a> _R10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: R10W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(R10W::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(R10W::_01)
    }
    #[doc = "Write-through"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(R10W::_10)
    }
    #[doc = "Write-back"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(R10W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `R9`"]
pub enum R9W {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R9W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            R9W::_00 => 0,
            R9W::_01 => 1,
            R9W::_10 => 2,
            R9W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _R9W<'a> {
    w: &'a mut W,
}
impl<'a> _R9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: R9W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(R9W::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(R9W::_01)
    }
    #[doc = "Write-through"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(R9W::_10)
    }
    #[doc = "Write-back"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(R9W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `R8`"]
pub enum R8W {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R8W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            R8W::_00 => 0,
            R8W::_01 => 1,
            R8W::_10 => 2,
            R8W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _R8W<'a> {
    w: &'a mut W,
}
impl<'a> _R8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: R8W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(R8W::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(R8W::_01)
    }
    #[doc = "Write-through"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(R8W::_10)
    }
    #[doc = "Write-back"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(R8W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `R7`"]
pub enum R7W {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            R7W::_00 => 0,
            R7W::_01 => 1,
            R7W::_10 => 2,
            R7W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _R7W<'a> {
    w: &'a mut W,
}
impl<'a> _R7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: R7W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(R7W::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(R7W::_01)
    }
    #[doc = "Write-through"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(R7W::_10)
    }
    #[doc = "Write-back"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(R7W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `R6`"]
pub enum R6W {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            R6W::_00 => 0,
            R6W::_01 => 1,
            R6W::_10 => 2,
            R6W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _R6W<'a> {
    w: &'a mut W,
}
impl<'a> _R6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: R6W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(R6W::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(R6W::_01)
    }
    #[doc = "Write-through"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(R6W::_10)
    }
    #[doc = "Write-back"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(R6W::_11)
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
#[doc = "Values that can be written to the field `R5`"]
pub enum R5W {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            R5W::_00 => 0,
            R5W::_01 => 1,
            R5W::_10 => 2,
            R5W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _R5W<'a> {
    w: &'a mut W,
}
impl<'a> _R5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: R5W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(R5W::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(R5W::_01)
    }
    #[doc = "Write-through"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(R5W::_10)
    }
    #[doc = "Write-back"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(R5W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `R4`"]
pub enum R4W {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            R4W::_00 => 0,
            R4W::_01 => 1,
            R4W::_10 => 2,
            R4W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _R4W<'a> {
    w: &'a mut W,
}
impl<'a> _R4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: R4W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(R4W::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(R4W::_01)
    }
    #[doc = "Write-through"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(R4W::_10)
    }
    #[doc = "Write-back"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(R4W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `R3`"]
pub enum R3W {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            R3W::_00 => 0,
            R3W::_01 => 1,
            R3W::_10 => 2,
            R3W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _R3W<'a> {
    w: &'a mut W,
}
impl<'a> _R3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: R3W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(R3W::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(R3W::_01)
    }
    #[doc = "Write-through"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(R3W::_10)
    }
    #[doc = "Write-back"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(R3W::_11)
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
#[doc = "Values that can be written to the field `R2`"]
pub enum R2W {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            R2W::_00 => 0,
            R2W::_01 => 1,
            R2W::_10 => 2,
            R2W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _R2W<'a> {
    w: &'a mut W,
}
impl<'a> _R2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: R2W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(R2W::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(R2W::_01)
    }
    #[doc = "Write-through"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(R2W::_10)
    }
    #[doc = "Write-back"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(R2W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `R1`"]
pub enum R1W {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            R1W::_00 => 0,
            R1W::_01 => 1,
            R1W::_10 => 2,
            R1W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _R1W<'a> {
    w: &'a mut W,
}
impl<'a> _R1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: R1W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(R1W::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(R1W::_01)
    }
    #[doc = "Write-through"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(R1W::_10)
    }
    #[doc = "Write-back"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(R1W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `R0`"]
pub enum R0W {
    #[doc = "Non-cacheable"]
    _00,
    #[doc = "Non-cacheable"]
    _01,
    #[doc = "Write-through"]
    _10,
    #[doc = "Write-back"]
    _11,
}
impl R0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            R0W::_00 => 0,
            R0W::_01 => 1,
            R0W::_10 => 2,
            R0W::_11 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _R0W<'a> {
    w: &'a mut W,
}
impl<'a> _R0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: R0W) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _00(self) -> &'a mut W {
        self.variant(R0W::_00)
    }
    #[doc = "Non-cacheable"]
    #[inline]
    pub fn _01(self) -> &'a mut W {
        self.variant(R0W::_01)
    }
    #[doc = "Write-through"]
    #[inline]
    pub fn _10(self) -> &'a mut W {
        self.variant(R0W::_10)
    }
    #[doc = "Write-back"]
    #[inline]
    pub fn _11(self) -> &'a mut W {
        self.variant(R0W::_11)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 30;
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
    #[doc = "Bits 0:1 - Region 15 mode"]
    #[inline]
    pub fn r15(&self) -> R15R {
        R15R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 2:3 - Region 14 mode"]
    #[inline]
    pub fn r14(&self) -> R14R {
        R14R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:5 - Region 13 mode"]
    #[inline]
    pub fn r13(&self) -> R13R {
        R13R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 6:7 - Region 12 mode"]
    #[inline]
    pub fn r12(&self) -> R12R {
        R12R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:9 - Region 11 mode"]
    #[inline]
    pub fn r11(&self) -> R11R {
        R11R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Region 10 mode"]
    #[inline]
    pub fn r10(&self) -> R10R {
        R10R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:13 - Region 9 mode"]
    #[inline]
    pub fn r9(&self) -> R9R {
        R9R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 14:15 - Region 8 mode"]
    #[inline]
    pub fn r8(&self) -> R8R {
        R8R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - Region 7 mode"]
    #[inline]
    pub fn r7(&self) -> R7R {
        R7R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Region 6 mode"]
    #[inline]
    pub fn r6(&self) -> R6R {
        R6R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:21 - Region 5 mode"]
    #[inline]
    pub fn r5(&self) -> R5R {
        R5R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 22:23 - Region 4 mode"]
    #[inline]
    pub fn r4(&self) -> R4R {
        R4R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:25 - Region 3 mode"]
    #[inline]
    pub fn r3(&self) -> R3R {
        R3R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 26:27 - Region 2 mode"]
    #[inline]
    pub fn r2(&self) -> R2R {
        R2R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:29 - Region 1 mode"]
    #[inline]
    pub fn r1(&self) -> R1R {
        R1R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 30:31 - Region 0 mode"]
    #[inline]
    pub fn r0(&self) -> R0R {
        R0R::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2853150720 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Region 15 mode"]
    #[inline]
    pub fn r15(&mut self) -> _R15W {
        _R15W { w: self }
    }
    #[doc = "Bits 2:3 - Region 14 mode"]
    #[inline]
    pub fn r14(&mut self) -> _R14W {
        _R14W { w: self }
    }
    #[doc = "Bits 4:5 - Region 13 mode"]
    #[inline]
    pub fn r13(&mut self) -> _R13W {
        _R13W { w: self }
    }
    #[doc = "Bits 6:7 - Region 12 mode"]
    #[inline]
    pub fn r12(&mut self) -> _R12W {
        _R12W { w: self }
    }
    #[doc = "Bits 8:9 - Region 11 mode"]
    #[inline]
    pub fn r11(&mut self) -> _R11W {
        _R11W { w: self }
    }
    #[doc = "Bits 10:11 - Region 10 mode"]
    #[inline]
    pub fn r10(&mut self) -> _R10W {
        _R10W { w: self }
    }
    #[doc = "Bits 12:13 - Region 9 mode"]
    #[inline]
    pub fn r9(&mut self) -> _R9W {
        _R9W { w: self }
    }
    #[doc = "Bits 14:15 - Region 8 mode"]
    #[inline]
    pub fn r8(&mut self) -> _R8W {
        _R8W { w: self }
    }
    #[doc = "Bits 16:17 - Region 7 mode"]
    #[inline]
    pub fn r7(&mut self) -> _R7W {
        _R7W { w: self }
    }
    #[doc = "Bits 18:19 - Region 6 mode"]
    #[inline]
    pub fn r6(&mut self) -> _R6W {
        _R6W { w: self }
    }
    #[doc = "Bits 20:21 - Region 5 mode"]
    #[inline]
    pub fn r5(&mut self) -> _R5W {
        _R5W { w: self }
    }
    #[doc = "Bits 22:23 - Region 4 mode"]
    #[inline]
    pub fn r4(&mut self) -> _R4W {
        _R4W { w: self }
    }
    #[doc = "Bits 24:25 - Region 3 mode"]
    #[inline]
    pub fn r3(&mut self) -> _R3W {
        _R3W { w: self }
    }
    #[doc = "Bits 26:27 - Region 2 mode"]
    #[inline]
    pub fn r2(&mut self) -> _R2W {
        _R2W { w: self }
    }
    #[doc = "Bits 28:29 - Region 1 mode"]
    #[inline]
    pub fn r1(&mut self) -> _R1W {
        _R1W { w: self }
    }
    #[doc = "Bits 30:31 - Region 0 mode"]
    #[inline]
    pub fn r0(&mut self) -> _R0W {
        _R0W { w: self }
    }
}
