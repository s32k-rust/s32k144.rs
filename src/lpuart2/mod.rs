use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"] pub verid: VERID,
    #[doc = "0x04 - Parameter Register"] pub param: PARAM,
    #[doc = "0x08 - LPUART Global Register"] pub global: GLOBAL,
    #[doc = "0x0c - LPUART Pin Configuration Register"] pub pincfg: PINCFG,
    #[doc = "0x10 - LPUART Baud Rate Register"] pub baud: BAUD,
    #[doc = "0x14 - LPUART Status Register"] pub stat: STAT,
    #[doc = "0x18 - LPUART Control Register"] pub ctrl: CTRL,
    #[doc = "0x1c - LPUART Data Register"] pub data: DATA,
    #[doc = "0x20 - LPUART Match Address Register"] pub match_: MATCH,
    #[doc = "0x24 - LPUART Modem IrDA Register"] pub modir: MODIR,
    #[doc = "0x28 - LPUART FIFO Register"] pub fifo: FIFO,
    #[doc = "0x2c - LPUART Watermark Register"] pub water: WATER,
}
#[doc = "Version ID Register"]
pub struct VERID {
    register: VolatileCell<u32>,
}
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "Parameter Register"]
pub struct PARAM {
    register: VolatileCell<u32>,
}
#[doc = "Parameter Register"]
pub mod param;
#[doc = "LPUART Global Register"]
pub struct GLOBAL {
    register: VolatileCell<u32>,
}
#[doc = "LPUART Global Register"]
pub mod global;
#[doc = "LPUART Pin Configuration Register"]
pub struct PINCFG {
    register: VolatileCell<u32>,
}
#[doc = "LPUART Pin Configuration Register"]
pub mod pincfg;
#[doc = "LPUART Baud Rate Register"]
pub struct BAUD {
    register: VolatileCell<u32>,
}
#[doc = "LPUART Baud Rate Register"]
pub mod baud;
#[doc = "LPUART Status Register"]
pub struct STAT {
    register: VolatileCell<u32>,
}
#[doc = "LPUART Status Register"]
pub mod stat;
#[doc = "LPUART Control Register"]
pub struct CTRL {
    register: VolatileCell<u32>,
}
#[doc = "LPUART Control Register"]
pub mod ctrl;
#[doc = "LPUART Data Register"]
pub struct DATA {
    register: VolatileCell<u32>,
}
#[doc = "LPUART Data Register"]
pub mod data;
#[doc = "LPUART Match Address Register"]
pub struct MATCH {
    register: VolatileCell<u32>,
}
#[doc = "LPUART Match Address Register"]
pub mod match_;
#[doc = "LPUART Modem IrDA Register"]
pub struct MODIR {
    register: VolatileCell<u32>,
}
#[doc = "LPUART Modem IrDA Register"]
pub mod modir;
#[doc = "LPUART FIFO Register"]
pub struct FIFO {
    register: VolatileCell<u32>,
}
#[doc = "LPUART FIFO Register"]
pub mod fifo;
#[doc = "LPUART Watermark Register"]
pub struct WATER {
    register: VolatileCell<u32>,
}
#[doc = "LPUART Watermark Register"]
pub mod water;
