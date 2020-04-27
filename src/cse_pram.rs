#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CSE PRAM 0 Register"]
    pub embedded_ram: [EMBEDDEDRAM; 32],
}
#[doc = "CSE PRAM 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [embedded_ram](embedded_ram) module"]
pub type EMBEDDEDRAM = crate::Reg<u32, _EMBEDDEDRAM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMBEDDEDRAM;
#[doc = "`read()` method returns [embedded_ram::R](embedded_ram::R) reader structure"]
impl crate::Readable for EMBEDDEDRAM {}
#[doc = "`write(|w| ..)` method takes [embedded_ram::W](embedded_ram::W) writer structure"]
impl crate::Writable for EMBEDDEDRAM {}
#[doc = "CSE PRAM 0 Register"]
pub mod embedded_ram;
