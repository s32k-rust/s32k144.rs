#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    pub verid: VERID,
    #[doc = "0x04 - Parameter Register"]
    pub param: PARAM,
    _reserved0: [u8; 8usize],
    #[doc = "0x10 - Clock Status Register"]
    pub csr: CSR,
    #[doc = "0x14 - Run Clock Control Register"]
    pub rccr: RCCR,
    #[doc = "0x18 - VLPR Clock Control Register"]
    pub vccr: VCCR,
    #[doc = "0x1c - HSRUN Clock Control Register"]
    pub hccr: HCCR,
    #[doc = "0x20 - SCG CLKOUT Configuration Register"]
    pub clkoutcnfg: CLKOUTCNFG,
    _reserved1: [u8; 220usize],
    #[doc = "0x100 - System OSC Control Status Register"]
    pub sosccsr: SOSCCSR,
    #[doc = "0x104 - System OSC Divide Register"]
    pub soscdiv: SOSCDIV,
    #[doc = "0x108 - System Oscillator Configuration Register"]
    pub sosccfg: SOSCCFG,
    _reserved2: [u8; 244usize],
    #[doc = "0x200 - Slow IRC Control Status Register"]
    pub sirccsr: SIRCCSR,
    #[doc = "0x204 - Slow IRC Divide Register"]
    pub sircdiv: SIRCDIV,
    #[doc = "0x208 - Slow IRC Configuration Register"]
    pub sirccfg: SIRCCFG,
    _reserved3: [u8; 244usize],
    #[doc = "0x300 - Fast IRC Control Status Register"]
    pub firccsr: FIRCCSR,
    #[doc = "0x304 - Fast IRC Divide Register"]
    pub fircdiv: FIRCDIV,
    #[doc = "0x308 - Fast IRC Configuration Register"]
    pub firccfg: FIRCCFG,
    _reserved4: [u8; 756usize],
    #[doc = "0x600 - System PLL Control Status Register"]
    pub spllcsr: SPLLCSR,
    #[doc = "0x604 - System PLL Divide Register"]
    pub splldiv: SPLLDIV,
    #[doc = "0x608 - System PLL Configuration Register"]
    pub spllcfg: SPLLCFG,
}
#[doc = "Version ID Register"]
pub struct VERID {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "Parameter Register"]
pub struct PARAM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Parameter Register"]
pub mod param;
#[doc = "Clock Status Register"]
pub struct CSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Clock Status Register"]
pub mod csr;
#[doc = "Run Clock Control Register"]
pub struct RCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Run Clock Control Register"]
pub mod rccr;
#[doc = "VLPR Clock Control Register"]
pub struct VCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "VLPR Clock Control Register"]
pub mod vccr;
#[doc = "HSRUN Clock Control Register"]
pub struct HCCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "HSRUN Clock Control Register"]
pub mod hccr;
#[doc = "SCG CLKOUT Configuration Register"]
pub struct CLKOUTCNFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SCG CLKOUT Configuration Register"]
pub mod clkoutcnfg;
#[doc = "System OSC Control Status Register"]
pub struct SOSCCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System OSC Control Status Register"]
pub mod sosccsr;
#[doc = "System OSC Divide Register"]
pub struct SOSCDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System OSC Divide Register"]
pub mod soscdiv;
#[doc = "System Oscillator Configuration Register"]
pub struct SOSCCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System Oscillator Configuration Register"]
pub mod sosccfg;
#[doc = "Slow IRC Control Status Register"]
pub struct SIRCCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slow IRC Control Status Register"]
pub mod sirccsr;
#[doc = "Slow IRC Divide Register"]
pub struct SIRCDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slow IRC Divide Register"]
pub mod sircdiv;
#[doc = "Slow IRC Configuration Register"]
pub struct SIRCCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Slow IRC Configuration Register"]
pub mod sirccfg;
#[doc = "Fast IRC Control Status Register"]
pub struct FIRCCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fast IRC Control Status Register"]
pub mod firccsr;
#[doc = "Fast IRC Divide Register"]
pub struct FIRCDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fast IRC Divide Register"]
pub mod fircdiv;
#[doc = "Fast IRC Configuration Register"]
pub struct FIRCCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Fast IRC Configuration Register"]
pub mod firccfg;
#[doc = "System PLL Control Status Register"]
pub struct SPLLCSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System PLL Control Status Register"]
pub mod spllcsr;
#[doc = "System PLL Divide Register"]
pub struct SPLLDIV {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System PLL Divide Register"]
pub mod splldiv;
#[doc = "System PLL Configuration Register"]
pub struct SPLLCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System PLL Configuration Register"]
pub mod spllcfg;
