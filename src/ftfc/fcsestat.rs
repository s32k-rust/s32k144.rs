#[doc = "Reader of register FCSESTAT"]
pub type R = crate::R<u8, super::FCSESTAT>;
#[doc = "Reader of field `BSY`"]
pub type BSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `SB`"]
pub type SB_R = crate::R<bool, bool>;
#[doc = "Reader of field `BIN`"]
pub type BIN_R = crate::R<bool, bool>;
#[doc = "Reader of field `BFN`"]
pub type BFN_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOK`"]
pub type BOK_R = crate::R<bool, bool>;
#[doc = "Reader of field `RIN`"]
pub type RIN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EDB`"]
pub type EDB_R = crate::R<bool, bool>;
#[doc = "Reader of field `IDB`"]
pub type IDB_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Busy"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Secure Boot"]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Secure Boot Initialization"]
    #[inline(always)]
    pub fn bin(&self) -> BIN_R {
        BIN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Secure Boot Finished"]
    #[inline(always)]
    pub fn bfn(&self) -> BFN_R {
        BFN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Secure Boot OK"]
    #[inline(always)]
    pub fn bok(&self) -> BOK_R {
        BOK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Random Number Generator Initialized"]
    #[inline(always)]
    pub fn rin(&self) -> RIN_R {
        RIN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - External Debug"]
    #[inline(always)]
    pub fn edb(&self) -> EDB_R {
        EDB_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Internal Debug"]
    #[inline(always)]
    pub fn idb(&self) -> IDB_R {
        IDB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
