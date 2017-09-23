#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OPACRI {
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
#[doc = "Possible values of the field `TP6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP6R {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP6R {
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
            TP6R::_0 => false,
            TP6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TP6R {
        match value {
            false => TP6R::_0,
            true => TP6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TP6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TP6R::_1
    }
}
#[doc = "Possible values of the field `WP6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP6R {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP6R {
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
            WP6R::_0 => false,
            WP6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WP6R {
        match value {
            false => WP6R::_0,
            true => WP6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WP6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WP6R::_1
    }
}
#[doc = "Possible values of the field `SP6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP6R {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP6R {
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
            SP6R::_0 => false,
            SP6R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SP6R {
        match value {
            false => SP6R::_0,
            true => SP6R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SP6R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SP6R::_1
    }
}
#[doc = "Possible values of the field `TP5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP5R {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP5R {
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
            TP5R::_0 => false,
            TP5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TP5R {
        match value {
            false => TP5R::_0,
            true => TP5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TP5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TP5R::_1
    }
}
#[doc = "Possible values of the field `WP5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP5R {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP5R {
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
            WP5R::_0 => false,
            WP5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WP5R {
        match value {
            false => WP5R::_0,
            true => WP5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WP5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WP5R::_1
    }
}
#[doc = "Possible values of the field `SP5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP5R {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP5R {
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
            SP5R::_0 => false,
            SP5R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SP5R {
        match value {
            false => SP5R::_0,
            true => SP5R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SP5R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SP5R::_1
    }
}
#[doc = "Possible values of the field `TP4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP4R {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP4R {
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
            TP4R::_0 => false,
            TP4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TP4R {
        match value {
            false => TP4R::_0,
            true => TP4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TP4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TP4R::_1
    }
}
#[doc = "Possible values of the field `WP4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP4R {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP4R {
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
            WP4R::_0 => false,
            WP4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WP4R {
        match value {
            false => WP4R::_0,
            true => WP4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WP4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WP4R::_1
    }
}
#[doc = "Possible values of the field `SP4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP4R {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP4R {
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
            SP4R::_0 => false,
            SP4R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SP4R {
        match value {
            false => SP4R::_0,
            true => SP4R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SP4R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SP4R::_1
    }
}
#[doc = "Possible values of the field `TP3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP3R {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP3R {
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
            TP3R::_0 => false,
            TP3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TP3R {
        match value {
            false => TP3R::_0,
            true => TP3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TP3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TP3R::_1
    }
}
#[doc = "Possible values of the field `WP3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP3R {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP3R {
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
            WP3R::_0 => false,
            WP3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WP3R {
        match value {
            false => WP3R::_0,
            true => WP3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WP3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WP3R::_1
    }
}
#[doc = "Possible values of the field `SP3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP3R {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP3R {
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
            SP3R::_0 => false,
            SP3R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SP3R {
        match value {
            false => SP3R::_0,
            true => SP3R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SP3R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SP3R::_1
    }
}
#[doc = "Possible values of the field `TP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TP1R {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP1R {
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
            TP1R::_0 => false,
            TP1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TP1R {
        match value {
            false => TP1R::_0,
            true => TP1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == TP1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == TP1R::_1
    }
}
#[doc = "Possible values of the field `WP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WP1R {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP1R {
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
            WP1R::_0 => false,
            WP1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WP1R {
        match value {
            false => WP1R::_0,
            true => WP1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == WP1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == WP1R::_1
    }
}
#[doc = "Possible values of the field `SP1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SP1R {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP1R {
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
            SP1R::_0 => false,
            SP1R::_1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SP1R {
        match value {
            false => SP1R::_0,
            true => SP1R::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline]
    pub fn is_0(&self) -> bool {
        *self == SP1R::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline]
    pub fn is_1(&self) -> bool {
        *self == SP1R::_1
    }
}
#[doc = "Values that can be written to the field `TP6`"]
pub enum TP6W {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TP6W::_0 => false,
            TP6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TP6W<'a> {
    w: &'a mut W,
}
impl<'a> _TP6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TP6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP6W::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP6W::_1)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WP6`"]
pub enum WP6W {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WP6W::_0 => false,
            WP6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WP6W<'a> {
    w: &'a mut W,
}
impl<'a> _WP6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WP6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP6W::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP6W::_1)
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
#[doc = "Values that can be written to the field `SP6`"]
pub enum SP6W {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SP6W::_0 => false,
            SP6W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SP6W<'a> {
    w: &'a mut W,
}
impl<'a> _SP6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SP6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP6W::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP6W::_1)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TP5`"]
pub enum TP5W {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TP5W::_0 => false,
            TP5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TP5W<'a> {
    w: &'a mut W,
}
impl<'a> _TP5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TP5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP5W::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP5W::_1)
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
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WP5`"]
pub enum WP5W {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WP5W::_0 => false,
            WP5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WP5W<'a> {
    w: &'a mut W,
}
impl<'a> _WP5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WP5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP5W::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP5W::_1)
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SP5`"]
pub enum SP5W {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SP5W::_0 => false,
            SP5W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SP5W<'a> {
    w: &'a mut W,
}
impl<'a> _SP5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SP5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP5W::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP5W::_1)
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
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TP4`"]
pub enum TP4W {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TP4W::_0 => false,
            TP4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TP4W<'a> {
    w: &'a mut W,
}
impl<'a> _TP4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TP4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP4W::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP4W::_1)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WP4`"]
pub enum WP4W {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WP4W::_0 => false,
            WP4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WP4W<'a> {
    w: &'a mut W,
}
impl<'a> _WP4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WP4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP4W::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP4W::_1)
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
#[doc = "Values that can be written to the field `SP4`"]
pub enum SP4W {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SP4W::_0 => false,
            SP4W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SP4W<'a> {
    w: &'a mut W,
}
impl<'a> _SP4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SP4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP4W::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP4W::_1)
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
#[doc = "Values that can be written to the field `TP3`"]
pub enum TP3W {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TP3W::_0 => false,
            TP3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TP3W<'a> {
    w: &'a mut W,
}
impl<'a> _TP3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TP3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP3W::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP3W::_1)
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
#[doc = "Values that can be written to the field `WP3`"]
pub enum WP3W {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WP3W::_0 => false,
            WP3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WP3W<'a> {
    w: &'a mut W,
}
impl<'a> _WP3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WP3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP3W::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP3W::_1)
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
#[doc = "Values that can be written to the field `SP3`"]
pub enum SP3W {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SP3W::_0 => false,
            SP3W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SP3W<'a> {
    w: &'a mut W,
}
impl<'a> _SP3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SP3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP3W::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP3W::_1)
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
#[doc = "Values that can be written to the field `TP1`"]
pub enum TP1W {
    #[doc = "Accesses from an untrusted master are allowed."]
    _0,
    #[doc = "Accesses from an untrusted master are not allowed."]
    _1,
}
impl TP1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TP1W::_0 => false,
            TP1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TP1W<'a> {
    w: &'a mut W,
}
impl<'a> _TP1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TP1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP1W::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP1W::_1)
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
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WP1`"]
pub enum WP1W {
    #[doc = "This peripheral allows write accesses."]
    _0,
    #[doc = "This peripheral is write protected."]
    _1,
}
impl WP1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WP1W::_0 => false,
            WP1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WP1W<'a> {
    w: &'a mut W,
}
impl<'a> _WP1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WP1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral allows write accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP1W::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP1W::_1)
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SP1`"]
pub enum SP1W {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    _0,
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    _1,
}
impl SP1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SP1W::_0 => false,
            SP1W::_1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SP1W<'a> {
    w: &'a mut W,
}
impl<'a> _SP1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SP1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP1W::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP1W::_1)
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
        const OFFSET: u8 = 26;
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
    #[doc = "Bit 4 - Trusted Protect"]
    #[inline]
    pub fn tp6(&self) -> TP6R {
        TP6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 5 - Write Protect"]
    #[inline]
    pub fn wp6(&self) -> WP6R {
        WP6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Supervisor Protect"]
    #[inline]
    pub fn sp6(&self) -> SP6R {
        SP6R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 8 - Trusted Protect"]
    #[inline]
    pub fn tp5(&self) -> TP5R {
        TP5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 9 - Write Protect"]
    #[inline]
    pub fn wp5(&self) -> WP5R {
        WP5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 10 - Supervisor Protect"]
    #[inline]
    pub fn sp5(&self) -> SP5R {
        SP5R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 12 - Trusted Protect"]
    #[inline]
    pub fn tp4(&self) -> TP4R {
        TP4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 13 - Write Protect"]
    #[inline]
    pub fn wp4(&self) -> WP4R {
        WP4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Supervisor Protect"]
    #[inline]
    pub fn sp4(&self) -> SP4R {
        SP4R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 16 - Trusted Protect"]
    #[inline]
    pub fn tp3(&self) -> TP3R {
        TP3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 17 - Write Protect"]
    #[inline]
    pub fn wp3(&self) -> WP3R {
        WP3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 18 - Supervisor Protect"]
    #[inline]
    pub fn sp3(&self) -> SP3R {
        SP3R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 24 - Trusted Protect"]
    #[inline]
    pub fn tp1(&self) -> TP1R {
        TP1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 25 - Write Protect"]
    #[inline]
    pub fn wp1(&self) -> WP1R {
        WP1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 26 - Supervisor Protect"]
    #[inline]
    pub fn sp1(&self) -> SP1R {
        SP1R::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 67388480 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 4 - Trusted Protect"]
    #[inline]
    pub fn tp6(&mut self) -> _TP6W {
        _TP6W { w: self }
    }
    #[doc = "Bit 5 - Write Protect"]
    #[inline]
    pub fn wp6(&mut self) -> _WP6W {
        _WP6W { w: self }
    }
    #[doc = "Bit 6 - Supervisor Protect"]
    #[inline]
    pub fn sp6(&mut self) -> _SP6W {
        _SP6W { w: self }
    }
    #[doc = "Bit 8 - Trusted Protect"]
    #[inline]
    pub fn tp5(&mut self) -> _TP5W {
        _TP5W { w: self }
    }
    #[doc = "Bit 9 - Write Protect"]
    #[inline]
    pub fn wp5(&mut self) -> _WP5W {
        _WP5W { w: self }
    }
    #[doc = "Bit 10 - Supervisor Protect"]
    #[inline]
    pub fn sp5(&mut self) -> _SP5W {
        _SP5W { w: self }
    }
    #[doc = "Bit 12 - Trusted Protect"]
    #[inline]
    pub fn tp4(&mut self) -> _TP4W {
        _TP4W { w: self }
    }
    #[doc = "Bit 13 - Write Protect"]
    #[inline]
    pub fn wp4(&mut self) -> _WP4W {
        _WP4W { w: self }
    }
    #[doc = "Bit 14 - Supervisor Protect"]
    #[inline]
    pub fn sp4(&mut self) -> _SP4W {
        _SP4W { w: self }
    }
    #[doc = "Bit 16 - Trusted Protect"]
    #[inline]
    pub fn tp3(&mut self) -> _TP3W {
        _TP3W { w: self }
    }
    #[doc = "Bit 17 - Write Protect"]
    #[inline]
    pub fn wp3(&mut self) -> _WP3W {
        _WP3W { w: self }
    }
    #[doc = "Bit 18 - Supervisor Protect"]
    #[inline]
    pub fn sp3(&mut self) -> _SP3W {
        _SP3W { w: self }
    }
    #[doc = "Bit 24 - Trusted Protect"]
    #[inline]
    pub fn tp1(&mut self) -> _TP1W {
        _TP1W { w: self }
    }
    #[doc = "Bit 25 - Write Protect"]
    #[inline]
    pub fn wp1(&mut self) -> _WP1W {
        _WP1W { w: self }
    }
    #[doc = "Bit 26 - Supervisor Protect"]
    #[inline]
    pub fn sp1(&mut self) -> _SP1W {
        _SP1W { w: self }
    }
}
