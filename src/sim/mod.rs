use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Chip Control register"]
    pub chipctl: CHIPCTL,
    _reserved1: [u8; 4usize],
    #[doc = "0x0c - FTM Option Register 0"]
    pub ftmopt0: FTMOPT0,
    #[doc = "0x10 - LPO Clock Select Register"]
    pub lpoclks: LPOCLKS,
    _reserved2: [u8; 4usize],
    #[doc = "0x18 - ADC Options Register"]
    pub adcopt: ADCOPT,
    #[doc = "0x1c - FTM Option Register 1"]
    pub ftmopt1: FTMOPT1,
    #[doc = "0x20 - Miscellaneous control register 0"]
    pub misctrl0: MISCTRL0,
    #[doc = "0x24 - System Device Identification Register"]
    pub sdid: SDID,
    _reserved3: [u8; 24usize],
    #[doc = "0x40 - Platform Clock Gating Control Register"]
    pub platcgc: PLATCGC,
    _reserved4: [u8; 8usize],
    #[doc = "0x4c - Flash Configuration Register 1"]
    pub fcfg1: FCFG1,
    _reserved5: [u8; 4usize],
    #[doc = "0x54 - Unique Identification Register High"]
    pub uidh: UIDH,
    #[doc = "0x58 - Unique Identification Register Mid-High"]
    pub uidmh: UIDMH,
    #[doc = "0x5c - Unique Identification Register Mid Low"]
    pub uidml: UIDML,
    #[doc = "0x60 - Unique Identification Register Low"]
    pub uidl: UIDL,
    _reserved6: [u8; 4usize],
    #[doc = "0x68 - System Clock Divider Register 4"]
    pub clkdiv4: CLKDIV4,
    #[doc = "0x6c - Miscellaneous Control register 1"]
    pub misctrl1: MISCTRL1,
}
#[doc = "Chip Control register"]
pub struct CHIPCTL {
    register: VolatileCell<u32>,
}
#[doc = "Chip Control register"]
pub mod chipctl;
#[doc = "FTM Option Register 0"]
pub struct FTMOPT0 {
    register: VolatileCell<u32>,
}
#[doc = "FTM Option Register 0"]
pub mod ftmopt0;
#[doc = "LPO Clock Select Register"]
pub struct LPOCLKS {
    register: VolatileCell<u32>,
}
#[doc = "LPO Clock Select Register"]
pub mod lpoclks;
#[doc = "ADC Options Register"]
pub struct ADCOPT {
    register: VolatileCell<u32>,
}
#[doc = "ADC Options Register"]
pub mod adcopt;
#[doc = "FTM Option Register 1"]
pub struct FTMOPT1 {
    register: VolatileCell<u32>,
}
#[doc = "FTM Option Register 1"]
pub mod ftmopt1;
#[doc = "Miscellaneous control register 0"]
pub struct MISCTRL0 {
    register: VolatileCell<u32>,
}
#[doc = "Miscellaneous control register 0"]
pub mod misctrl0;
#[doc = "System Device Identification Register"]
pub struct SDID {
    register: VolatileCell<u32>,
}
#[doc = "System Device Identification Register"]
pub mod sdid;
#[doc = "Platform Clock Gating Control Register"]
pub struct PLATCGC {
    register: VolatileCell<u32>,
}
#[doc = "Platform Clock Gating Control Register"]
pub mod platcgc;
#[doc = "Flash Configuration Register 1"]
pub struct FCFG1 {
    register: VolatileCell<u32>,
}
#[doc = "Flash Configuration Register 1"]
pub mod fcfg1;
#[doc = "Unique Identification Register High"]
pub struct UIDH {
    register: VolatileCell<u32>,
}
#[doc = "Unique Identification Register High"]
pub mod uidh;
#[doc = "Unique Identification Register Mid-High"]
pub struct UIDMH {
    register: VolatileCell<u32>,
}
#[doc = "Unique Identification Register Mid-High"]
pub mod uidmh;
#[doc = "Unique Identification Register Mid Low"]
pub struct UIDML {
    register: VolatileCell<u32>,
}
#[doc = "Unique Identification Register Mid Low"]
pub mod uidml;
#[doc = "Unique Identification Register Low"]
pub struct UIDL {
    register: VolatileCell<u32>,
}
#[doc = "Unique Identification Register Low"]
pub mod uidl;
#[doc = "System Clock Divider Register 4"]
pub struct CLKDIV4 {
    register: VolatileCell<u32>,
}
#[doc = "System Clock Divider Register 4"]
pub mod clkdiv4;
#[doc = "Miscellaneous Control register 1"]
pub struct MISCTRL1 {
    register: VolatileCell<u32>,
}
#[doc = "Miscellaneous Control register 1"]
pub mod misctrl1;
