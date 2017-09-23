use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Watchdog Control and Status Register"] pub cs: CS,
    #[doc = "0x04 - Watchdog Counter Register"] pub cnt: CNT,
    #[doc = "0x08 - Watchdog Timeout Value Register"] pub toval: TOVAL,
    #[doc = "0x0c - Watchdog Window Register"] pub win: WIN,
}
#[doc = "Watchdog Control and Status Register"]
pub struct CS {
    register: VolatileCell<u32>,
}
#[doc = "Watchdog Control and Status Register"]
pub mod cs;
#[doc = "Watchdog Counter Register"]
pub struct CNT {
    register: VolatileCell<u32>,
}
#[doc = "Watchdog Counter Register"]
pub mod cnt;
#[doc = "Watchdog Timeout Value Register"]
pub struct TOVAL {
    register: VolatileCell<u32>,
}
#[doc = "Watchdog Timeout Value Register"]
pub mod toval;
#[doc = "Watchdog Window Register"]
pub struct WIN {
    register: VolatileCell<u32>,
}
#[doc = "Watchdog Window Register"]
pub mod win;
