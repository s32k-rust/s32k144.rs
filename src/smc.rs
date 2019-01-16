#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SMC Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - SMC Parameter Register"]
    pub param: PARAM,
    #[doc = "0x08 - Power Mode Protection register"]
    pub pmprot: PMPROT,
    #[doc = "0x0c - Power Mode Control register"]
    pub pmctrl: PMCTRL,
    #[doc = "0x10 - Stop Control Register"]
    pub stopctrl: STOPCTRL,
    #[doc = "0x14 - Power Mode Status register"]
    pub pmstat: PMSTAT,
}
#[doc = "SMC Version ID Register"]
pub struct VERID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Version ID Register"]
pub mod verid;
#[doc = "SMC Parameter Register"]
pub struct PARAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SMC Parameter Register"]
pub mod param;
#[doc = "Power Mode Protection register"]
pub struct PMPROT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Mode Protection register"]
pub mod pmprot;
#[doc = "Power Mode Control register"]
pub struct PMCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Mode Control register"]
pub mod pmctrl;
#[doc = "Stop Control Register"]
pub struct STOPCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop Control Register"]
pub mod stopctrl;
#[doc = "Power Mode Status register"]
pub struct PMSTAT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Power Mode Status register"]
pub mod pmstat;
