use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TRGMUX DMAMUX0 Register"] pub trgmux_dmamux0: TRGMUX_DMAMUX0,
    #[doc = "0x04 - TRGMUX EXTOUT0 Register"] pub trgmux_extout0: TRGMUX_EXTOUT0,
    #[doc = "0x08 - TRGMUX EXTOUT1 Register"] pub trgmux_extout1: TRGMUX_EXTOUT1,
    #[doc = "0x0c - TRGMUX ADC0 Register"] pub trgmux_adc0: TRGMUX_ADC0,
    #[doc = "0x10 - TRGMUX ADC1 Register"] pub trgmux_adc1: TRGMUX_ADC1,
    #[doc = "0x14 - TRGMUX Reserved Register 5"] pub trgmuxdummy5: TRGMUXDUMMY5,
    #[doc = "0x18 - TRGMUX Reserved Register 6"] pub trgmuxdummy6: TRGMUXDUMMY6,
    #[doc = "0x1c - TRGMUX CMP0 Register"] pub trgmux_cmp0: TRGMUX_CMP0,
    #[doc = "0x20 - TRGMUX Reserved Register 8"] pub trgmuxdummy8: TRGMUXDUMMY8,
    #[doc = "0x24 - TRGMUX Reserved Register 9"] pub trgmuxdummy9: TRGMUXDUMMY9,
    #[doc = "0x28 - TRGMUX FTM0 Register"] pub trgmux_ftm0: TRGMUX_FTM0,
    #[doc = "0x2c - TRGMUX FTM1 Register"] pub trgmux_ftm1: TRGMUX_FTM1,
    #[doc = "0x30 - TRGMUX FTM2 Register"] pub trgmux_ftm2: TRGMUX_FTM2,
    #[doc = "0x34 - TRGMUX FTM3 Register"] pub trgmux_ftm3: TRGMUX_FTM3,
    #[doc = "0x38 - TRGMUX PDB0 Register"] pub trgmux_pdb0: TRGMUX_PDB0,
    #[doc = "0x3c - TRGMUX PDB1 Register"] pub trgmux_pdb1: TRGMUX_PDB1,
    #[doc = "0x40 - TRGMUX Reserved Register 16"] pub trgmuxdummy16: TRGMUXDUMMY16,
    #[doc = "0x44 - TRGMUX FLEXIO Register"] pub trgmux_flexio: TRGMUX_FLEXIO,
    #[doc = "0x48 - TRGMUX LPIT0 Register"] pub trgmux_lpit0: TRGMUX_LPIT0,
    #[doc = "0x4c - TRGMUX LPUART0 Register"] pub trgmux_lpuart0: TRGMUX_LPUART0,
    #[doc = "0x50 - TRGMUX LPUART1 Register"] pub trgmux_lpuart1: TRGMUX_LPUART1,
    #[doc = "0x54 - TRGMUX LPI2C0 Register"] pub trgmux_lpi2c0: TRGMUX_LPI2C0,
    #[doc = "0x58 - TRGMUX Reserved Register 22"] pub trgmuxdummy22: TRGMUXDUMMY22,
    #[doc = "0x5c - TRGMUX LPSPI0 Register"] pub trgmux_lpspi0: TRGMUX_LPSPI0,
    #[doc = "0x60 - TRGMUX LPSPI1 Register"] pub trgmux_lpspi1: TRGMUX_LPSPI1,
    #[doc = "0x64 - TRGMUX LPTMR0 Register"] pub trgmux_lptmr0: TRGMUX_LPTMR0,
}
#[doc = "TRGMUX DMAMUX0 Register"]
pub struct TRGMUX_DMAMUX0 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX DMAMUX0 Register"]
pub mod trgmux_dmamux0;
#[doc = "TRGMUX EXTOUT0 Register"]
pub struct TRGMUX_EXTOUT0 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX EXTOUT0 Register"]
pub mod trgmux_extout0;
#[doc = "TRGMUX EXTOUT1 Register"]
pub struct TRGMUX_EXTOUT1 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX EXTOUT1 Register"]
pub mod trgmux_extout1;
#[doc = "TRGMUX ADC0 Register"]
pub struct TRGMUX_ADC0 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX ADC0 Register"]
pub mod trgmux_adc0;
#[doc = "TRGMUX ADC1 Register"]
pub struct TRGMUX_ADC1 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX ADC1 Register"]
pub mod trgmux_adc1;
#[doc = "TRGMUX Reserved Register 5"]
pub struct TRGMUXDUMMY5 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX Reserved Register 5"]
pub mod trgmuxdummy5;
#[doc = "TRGMUX Reserved Register 6"]
pub struct TRGMUXDUMMY6 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX Reserved Register 6"]
pub mod trgmuxdummy6;
#[doc = "TRGMUX CMP0 Register"]
pub struct TRGMUX_CMP0 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX CMP0 Register"]
pub mod trgmux_cmp0;
#[doc = "TRGMUX Reserved Register 8"]
pub struct TRGMUXDUMMY8 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX Reserved Register 8"]
pub mod trgmuxdummy8;
#[doc = "TRGMUX Reserved Register 9"]
pub struct TRGMUXDUMMY9 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX Reserved Register 9"]
pub mod trgmuxdummy9;
#[doc = "TRGMUX FTM0 Register"]
pub struct TRGMUX_FTM0 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX FTM0 Register"]
pub mod trgmux_ftm0;
#[doc = "TRGMUX FTM1 Register"]
pub struct TRGMUX_FTM1 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX FTM1 Register"]
pub mod trgmux_ftm1;
#[doc = "TRGMUX FTM2 Register"]
pub struct TRGMUX_FTM2 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX FTM2 Register"]
pub mod trgmux_ftm2;
#[doc = "TRGMUX FTM3 Register"]
pub struct TRGMUX_FTM3 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX FTM3 Register"]
pub mod trgmux_ftm3;
#[doc = "TRGMUX PDB0 Register"]
pub struct TRGMUX_PDB0 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX PDB0 Register"]
pub mod trgmux_pdb0;
#[doc = "TRGMUX PDB1 Register"]
pub struct TRGMUX_PDB1 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX PDB1 Register"]
pub mod trgmux_pdb1;
#[doc = "TRGMUX Reserved Register 16"]
pub struct TRGMUXDUMMY16 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX Reserved Register 16"]
pub mod trgmuxdummy16;
#[doc = "TRGMUX FLEXIO Register"]
pub struct TRGMUX_FLEXIO {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX FLEXIO Register"]
pub mod trgmux_flexio;
#[doc = "TRGMUX LPIT0 Register"]
pub struct TRGMUX_LPIT0 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX LPIT0 Register"]
pub mod trgmux_lpit0;
#[doc = "TRGMUX LPUART0 Register"]
pub struct TRGMUX_LPUART0 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX LPUART0 Register"]
pub mod trgmux_lpuart0;
#[doc = "TRGMUX LPUART1 Register"]
pub struct TRGMUX_LPUART1 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX LPUART1 Register"]
pub mod trgmux_lpuart1;
#[doc = "TRGMUX LPI2C0 Register"]
pub struct TRGMUX_LPI2C0 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX LPI2C0 Register"]
pub mod trgmux_lpi2c0;
#[doc = "TRGMUX Reserved Register 22"]
pub struct TRGMUXDUMMY22 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX Reserved Register 22"]
pub mod trgmuxdummy22;
#[doc = "TRGMUX LPSPI0 Register"]
pub struct TRGMUX_LPSPI0 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX LPSPI0 Register"]
pub mod trgmux_lpspi0;
#[doc = "TRGMUX LPSPI1 Register"]
pub struct TRGMUX_LPSPI1 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX LPSPI1 Register"]
pub mod trgmux_lpspi1;
#[doc = "TRGMUX LPTMR0 Register"]
pub struct TRGMUX_LPTMR0 {
    register: VolatileCell<u32>,
}
#[doc = "TRGMUX LPTMR0 Register"]
pub mod trgmux_lptmr0;
