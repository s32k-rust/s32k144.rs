#[doc = "Reader of register PDOR"]
pub type R = crate::R<u32, super::PDOR>;
#[doc = "Writer for register PDOR"]
pub type W = crate::W<u32, super::PDOR>;
#[doc = "Register PDOR `reset()`'s with value 0"]
impl crate::ResetValue for super::PDOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PDO`"]
pub type PDO_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PDO`"]
pub struct PDO_W<'a> {
    w: &'a mut W,
}
impl<'a> PDO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Output"]
    #[inline(always)]
    pub fn pdo(&self) -> PDO_R {
        PDO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Output"]
    #[inline(always)]
    pub fn pdo(&mut self) -> PDO_W {
        PDO_W { w: self }
    }
}
