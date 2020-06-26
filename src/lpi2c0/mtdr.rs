#[doc = "Reader of register MTDR"]
pub type R = crate::R<u32, super::MTDR>;
#[doc = "Writer for register MTDR"]
pub type W = crate::W<u32, super::MTDR>;
#[doc = "Register MTDR `reset()`'s with value 0"]
impl crate::ResetValue for super::MTDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Command Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_AW {
    #[doc = "0: Transmit DATA\\[7:0\\]."]
    _000 = 0,
    #[doc = "1: Receive (DATA\\[7:0\\]
+ 1) bytes."]
    _001 = 1,
    #[doc = "2: Generate STOP condition."]
    _010 = 2,
    #[doc = "3: Receive and discard (DATA\\[7:0\\]
+ 1) bytes."]
    _011 = 3,
    #[doc = "4: Generate (repeated) START and transmit address in DATA\\[7:0\\]."]
    _100 = 4,
    #[doc = "5: Generate (repeated) START and transmit address in DATA\\[7:0\\]. This transfer expects a NACK to be returned."]
    _101 = 5,
    #[doc = "6: Generate (repeated) START and transmit address in DATA\\[7:0\\]
using high speed mode."]
    _110 = 6,
    #[doc = "7: Generate (repeated) START and transmit address in DATA\\[7:0\\]
using high speed mode. This transfer expects a NACK to be returned."]
    _111 = 7,
}
impl From<CMD_AW> for u8 {
    #[inline(always)]
    fn from(variant: CMD_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `CMD`"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_AW) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Transmit DATA\\[7:0\\]."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CMD_AW::_000)
    }
    #[doc = "Receive (DATA\\[7:0\\]
+ 1) bytes."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CMD_AW::_001)
    }
    #[doc = "Generate STOP condition."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CMD_AW::_010)
    }
    #[doc = "Receive and discard (DATA\\[7:0\\]
+ 1) bytes."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CMD_AW::_011)
    }
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\]."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CMD_AW::_100)
    }
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\]. This transfer expects a NACK to be returned."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CMD_AW::_101)
    }
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\]
using high speed mode."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CMD_AW::_110)
    }
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\]
using high speed mode. This transfer expects a NACK to be returned."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CMD_AW::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
impl R {}
impl W {
    #[doc = "Bits 0:7 - Transmit Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Bits 8:10 - Command Data"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
}
