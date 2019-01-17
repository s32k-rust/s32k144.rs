#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 128usize],
    #[doc = "0x80 - PCC FTFC Register"]
    pub pcc_ftfc: PCC_FTFC,
    #[doc = "0x84 - PCC DMAMUX Register"]
    pub pcc_dmamux: PCC_DMAMUX,
    _reserved1: [u8; 8usize],
    #[doc = "0x90 - PCC FlexCAN0 Register"]
    pub pcc_flex_can0: PCC_FLEXCAN0,
    #[doc = "0x94 - PCC FlexCAN1 Register"]
    pub pcc_flex_can1: PCC_FLEXCAN1,
    #[doc = "0x98 - PCC FTM3 Register"]
    pub pcc_ftm3: PCC_FTM3,
    #[doc = "0x9c - PCC ADC1 Register"]
    pub pcc_adc1: PCC_ADC1,
    _reserved2: [u8; 12usize],
    #[doc = "0xac - PCC FlexCAN2 Register"]
    pub pcc_flex_can2: PCC_FLEXCAN2,
    #[doc = "0xb0 - PCC LPSPI0 Register"]
    pub pcc_lpspi0: PCC_LPSPI0,
    #[doc = "0xb4 - PCC LPSPI1 Register"]
    pub pcc_lpspi1: PCC_LPSPI1,
    #[doc = "0xb8 - PCC LPSPI2 Register"]
    pub pcc_lpspi2: PCC_LPSPI2,
    _reserved3: [u8; 8usize],
    #[doc = "0xc4 - PCC PDB1 Register"]
    pub pcc_pdb1: PCC_PDB1,
    #[doc = "0xc8 - PCC CRC Register"]
    pub pcc_crc: PCC_CRC,
    _reserved4: [u8; 12usize],
    #[doc = "0xd8 - PCC PDB0 Register"]
    pub pcc_pdb0: PCC_PDB0,
    #[doc = "0xdc - PCC LPIT Register"]
    pub pcc_lpit: PCC_LPIT,
    #[doc = "0xe0 - PCC FTM0 Register"]
    pub pcc_ftm0: PCC_FTM0,
    #[doc = "0xe4 - PCC FTM1 Register"]
    pub pcc_ftm1: PCC_FTM1,
    #[doc = "0xe8 - PCC FTM2 Register"]
    pub pcc_ftm2: PCC_FTM2,
    #[doc = "0xec - PCC ADC0 Register"]
    pub pcc_adc0: PCC_ADC0,
    _reserved5: [u8; 4usize],
    #[doc = "0xf4 - PCC RTC Register"]
    pub pcc_rtc: PCC_RTC,
    _reserved6: [u8; 8usize],
    #[doc = "0x100 - PCC LPTMR0 Register"]
    pub pcc_lptmr0: PCC_LPTMR0,
    _reserved7: [u8; 32usize],
    #[doc = "0x124 - PCC PORTA Register"]
    pub pcc_porta: PCC_PORTA,
    #[doc = "0x128 - PCC PORTB Register"]
    pub pcc_portb: PCC_PORTB,
    #[doc = "0x12c - PCC PORTC Register"]
    pub pcc_portc: PCC_PORTC,
    #[doc = "0x130 - PCC PORTD Register"]
    pub pcc_portd: PCC_PORTD,
    #[doc = "0x134 - PCC PORTE Register"]
    pub pcc_porte: PCC_PORTE,
    _reserved8: [u8; 48usize],
    #[doc = "0x168 - PCC FlexIO Register"]
    pub pcc_flexio: PCC_FLEXIO,
    _reserved9: [u8; 24usize],
    #[doc = "0x184 - PCC EWM Register"]
    pub pcc_ewm: PCC_EWM,
    _reserved10: [u8; 16usize],
    #[doc = "0x198 - PCC LPI2C0 Register"]
    pub pcc_lpi2c0: PCC_LPI2C0,
    _reserved11: [u8; 12usize],
    #[doc = "0x1a8 - PCC LPUART0 Register"]
    pub pcc_lpuart0: PCC_LPUART0,
    #[doc = "0x1ac - PCC LPUART1 Register"]
    pub pcc_lpuart1: PCC_LPUART1,
    #[doc = "0x1b0 - PCC LPUART2 Register"]
    pub pcc_lpuart2: PCC_LPUART2,
    _reserved12: [u8; 24usize],
    #[doc = "0x1cc - PCC CMP0 Register"]
    pub pcc_cmp0: PCC_CMP0,
}
#[doc = "PCC FTFC Register"]
pub struct PCC_FTFC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC FTFC Register"]
pub mod pcc_ftfc;
#[doc = "PCC DMAMUX Register"]
pub struct PCC_DMAMUX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC DMAMUX Register"]
pub mod pcc_dmamux;
#[doc = "PCC FlexCAN0 Register"]
pub struct PCC_FLEXCAN0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC FlexCAN0 Register"]
pub mod pcc_flex_can0;
#[doc = "PCC FlexCAN1 Register"]
pub struct PCC_FLEXCAN1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC FlexCAN1 Register"]
pub mod pcc_flex_can1;
#[doc = "PCC FTM3 Register"]
pub struct PCC_FTM3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC FTM3 Register"]
pub mod pcc_ftm3;
#[doc = "PCC ADC1 Register"]
pub struct PCC_ADC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC ADC1 Register"]
pub mod pcc_adc1;
#[doc = "PCC FlexCAN2 Register"]
pub struct PCC_FLEXCAN2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC FlexCAN2 Register"]
pub mod pcc_flex_can2;
#[doc = "PCC LPSPI0 Register"]
pub struct PCC_LPSPI0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC LPSPI0 Register"]
pub mod pcc_lpspi0;
#[doc = "PCC LPSPI1 Register"]
pub struct PCC_LPSPI1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC LPSPI1 Register"]
pub mod pcc_lpspi1;
#[doc = "PCC LPSPI2 Register"]
pub struct PCC_LPSPI2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC LPSPI2 Register"]
pub mod pcc_lpspi2;
#[doc = "PCC PDB1 Register"]
pub struct PCC_PDB1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC PDB1 Register"]
pub mod pcc_pdb1;
#[doc = "PCC CRC Register"]
pub struct PCC_CRC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC CRC Register"]
pub mod pcc_crc;
#[doc = "PCC PDB0 Register"]
pub struct PCC_PDB0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC PDB0 Register"]
pub mod pcc_pdb0;
#[doc = "PCC LPIT Register"]
pub struct PCC_LPIT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC LPIT Register"]
pub mod pcc_lpit;
#[doc = "PCC FTM0 Register"]
pub struct PCC_FTM0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC FTM0 Register"]
pub mod pcc_ftm0;
#[doc = "PCC FTM1 Register"]
pub struct PCC_FTM1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC FTM1 Register"]
pub mod pcc_ftm1;
#[doc = "PCC FTM2 Register"]
pub struct PCC_FTM2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC FTM2 Register"]
pub mod pcc_ftm2;
#[doc = "PCC ADC0 Register"]
pub struct PCC_ADC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC ADC0 Register"]
pub mod pcc_adc0;
#[doc = "PCC RTC Register"]
pub struct PCC_RTC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC RTC Register"]
pub mod pcc_rtc;
#[doc = "PCC LPTMR0 Register"]
pub struct PCC_LPTMR0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC LPTMR0 Register"]
pub mod pcc_lptmr0;
#[doc = "PCC PORTA Register"]
pub struct PCC_PORTA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC PORTA Register"]
pub mod pcc_porta;
#[doc = "PCC PORTB Register"]
pub struct PCC_PORTB {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC PORTB Register"]
pub mod pcc_portb;
#[doc = "PCC PORTC Register"]
pub struct PCC_PORTC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC PORTC Register"]
pub mod pcc_portc;
#[doc = "PCC PORTD Register"]
pub struct PCC_PORTD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC PORTD Register"]
pub mod pcc_portd;
#[doc = "PCC PORTE Register"]
pub struct PCC_PORTE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC PORTE Register"]
pub mod pcc_porte;
#[doc = "PCC FlexIO Register"]
pub struct PCC_FLEXIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC FlexIO Register"]
pub mod pcc_flexio;
#[doc = "PCC EWM Register"]
pub struct PCC_EWM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC EWM Register"]
pub mod pcc_ewm;
#[doc = "PCC LPI2C0 Register"]
pub struct PCC_LPI2C0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC LPI2C0 Register"]
pub mod pcc_lpi2c0;
#[doc = "PCC LPUART0 Register"]
pub struct PCC_LPUART0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC LPUART0 Register"]
pub mod pcc_lpuart0;
#[doc = "PCC LPUART1 Register"]
pub struct PCC_LPUART1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC LPUART1 Register"]
pub mod pcc_lpuart1;
#[doc = "PCC LPUART2 Register"]
pub struct PCC_LPUART2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC LPUART2 Register"]
pub mod pcc_lpuart2;
#[doc = "PCC CMP0 Register"]
pub struct PCC_CMP0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PCC CMP0 Register"]
pub mod pcc_cmp0;
