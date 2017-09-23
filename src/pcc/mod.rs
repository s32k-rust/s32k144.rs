use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PCC Reserved Register 0"] pub pccdummy0: PCCDUMMY0,
    #[doc = "0x04 - PCC Reserved Register 1"] pub pccdummy1: PCCDUMMY1,
    #[doc = "0x08 - PCC Reserved Register 2"] pub pccdummy2: PCCDUMMY2,
    #[doc = "0x0c - PCC Reserved Register 3"] pub pccdummy3: PCCDUMMY3,
    #[doc = "0x10 - PCC Reserved Register 4"] pub pccdummy4: PCCDUMMY4,
    #[doc = "0x14 - PCC Reserved Register 5"] pub pccdummy5: PCCDUMMY5,
    #[doc = "0x18 - PCC Reserved Register 6"] pub pccdummy6: PCCDUMMY6,
    #[doc = "0x1c - PCC Reserved Register 7"] pub pccdummy7: PCCDUMMY7,
    #[doc = "0x20 - PCC Reserved Register 8"] pub pccdummy8: PCCDUMMY8,
    #[doc = "0x24 - PCC Reserved Register 9"] pub pccdummy9: PCCDUMMY9,
    #[doc = "0x28 - PCC Reserved Register 10"] pub pccdummy10: PCCDUMMY10,
    #[doc = "0x2c - PCC Reserved Register 11"] pub pccdummy11: PCCDUMMY11,
    #[doc = "0x30 - PCC Reserved Register 12"] pub pccdummy12: PCCDUMMY12,
    #[doc = "0x34 - PCC Reserved Register 13"] pub pccdummy13: PCCDUMMY13,
    #[doc = "0x38 - PCC Reserved Register 14"] pub pccdummy14: PCCDUMMY14,
    #[doc = "0x3c - PCC Reserved Register 15"] pub pccdummy15: PCCDUMMY15,
    #[doc = "0x40 - PCC Reserved Register 16"] pub pccdummy16: PCCDUMMY16,
    #[doc = "0x44 - PCC Reserved Register 17"] pub pccdummy17: PCCDUMMY17,
    #[doc = "0x48 - PCC Reserved Register 18"] pub pccdummy18: PCCDUMMY18,
    #[doc = "0x4c - PCC Reserved Register 19"] pub pccdummy19: PCCDUMMY19,
    #[doc = "0x50 - PCC Reserved Register 20"] pub pccdummy20: PCCDUMMY20,
    #[doc = "0x54 - PCC Reserved Register 21"] pub pccdummy21: PCCDUMMY21,
    #[doc = "0x58 - PCC Reserved Register 22"] pub pccdummy22: PCCDUMMY22,
    #[doc = "0x5c - PCC Reserved Register 23"] pub pccdummy23: PCCDUMMY23,
    #[doc = "0x60 - PCC Reserved Register 24"] pub pccdummy24: PCCDUMMY24,
    #[doc = "0x64 - PCC Reserved Register 25"] pub pccdummy25: PCCDUMMY25,
    #[doc = "0x68 - PCC Reserved Register 26"] pub pccdummy26: PCCDUMMY26,
    #[doc = "0x6c - PCC Reserved Register 27"] pub pccdummy27: PCCDUMMY27,
    #[doc = "0x70 - PCC Reserved Register 28"] pub pccdummy28: PCCDUMMY28,
    #[doc = "0x74 - PCC Reserved Register 29"] pub pccdummy29: PCCDUMMY29,
    #[doc = "0x78 - PCC Reserved Register 30"] pub pccdummy30: PCCDUMMY30,
    #[doc = "0x7c - PCC Reserved Register 31"] pub pccdummy31: PCCDUMMY31,
    #[doc = "0x80 - PCC FTFC Register"] pub pcc_ftfc: PCC_FTFC,
    #[doc = "0x84 - PCC DMAMUX Register"] pub pcc_dmamux: PCC_DMAMUX,
    #[doc = "0x88 - PCC Reserved Register 34"] pub pccdummy34: PCCDUMMY34,
    #[doc = "0x8c - PCC Reserved Register 35"] pub pccdummy35: PCCDUMMY35,
    #[doc = "0x90 - PCC FlexCAN0 Register"] pub pcc_flex_can0: PCC_FLEXCAN0,
    #[doc = "0x94 - PCC FlexCAN1 Register"] pub pcc_flex_can1: PCC_FLEXCAN1,
    #[doc = "0x98 - PCC FTM3 Register"] pub pcc_ftm3: PCC_FTM3,
    #[doc = "0x9c - PCC ADC1 Register"] pub pcc_adc1: PCC_ADC1,
    #[doc = "0xa0 - PCC Reserved Register 40"] pub pccdummy40: PCCDUMMY40,
    #[doc = "0xa4 - PCC Reserved Register 41"] pub pccdummy41: PCCDUMMY41,
    #[doc = "0xa8 - PCC Reserved Register 42"] pub pccdummy42: PCCDUMMY42,
    #[doc = "0xac - PCC FlexCAN2 Register"] pub pcc_flex_can2: PCC_FLEXCAN2,
    #[doc = "0xb0 - PCC LPSPI0 Register"] pub pcc_lpspi0: PCC_LPSPI0,
    #[doc = "0xb4 - PCC LPSPI1 Register"] pub pcc_lpspi1: PCC_LPSPI1,
    #[doc = "0xb8 - PCC LPSPI2 Register"] pub pcc_lpspi2: PCC_LPSPI2,
    #[doc = "0xbc - PCC Reserved Register 47"] pub pccdummy47: PCCDUMMY47,
    #[doc = "0xc0 - PCC Reserved Register 48"] pub pccdummy48: PCCDUMMY48,
    #[doc = "0xc4 - PCC PDB1 Register"] pub pcc_pdb1: PCC_PDB1,
    #[doc = "0xc8 - PCC CRC Register"] pub pcc_crc: PCC_CRC,
    #[doc = "0xcc - PCC Reserved Register 51"] pub pccdummy51: PCCDUMMY51,
    #[doc = "0xd0 - PCC Reserved Register 52"] pub pccdummy52: PCCDUMMY52,
    #[doc = "0xd4 - PCC Reserved Register 53"] pub pccdummy53: PCCDUMMY53,
    #[doc = "0xd8 - PCC PDB0 Register"] pub pcc_pdb0: PCC_PDB0,
    #[doc = "0xdc - PCC LPIT Register"] pub pcc_lpit: PCC_LPIT,
    #[doc = "0xe0 - PCC FTM0 Register"] pub pcc_ftm0: PCC_FTM0,
    #[doc = "0xe4 - PCC FTM1 Register"] pub pcc_ftm1: PCC_FTM1,
    #[doc = "0xe8 - PCC FTM2 Register"] pub pcc_ftm2: PCC_FTM2,
    #[doc = "0xec - PCC ADC0 Register"] pub pcc_adc0: PCC_ADC0,
    #[doc = "0xf0 - PCC Reserved Register 60"] pub pccdummy60: PCCDUMMY60,
    #[doc = "0xf4 - PCC RTC Register"] pub pcc_rtc: PCC_RTC,
    #[doc = "0xf8 - PCC Reserved Register 62"] pub pccdummy62: PCCDUMMY62,
    #[doc = "0xfc - PCC Reserved Register 63"] pub pccdummy63: PCCDUMMY63,
    #[doc = "0x100 - PCC LPTMR0 Register"] pub pcc_lptmr0: PCC_LPTMR0,
    #[doc = "0x104 - PCC Reserved Register 65"] pub pccdummy65: PCCDUMMY65,
    #[doc = "0x108 - PCC Reserved Register 66"] pub pccdummy66: PCCDUMMY66,
    #[doc = "0x10c - PCC Reserved Register 67"] pub pccdummy67: PCCDUMMY67,
    #[doc = "0x110 - PCC Reserved Register 68"] pub pccdummy68: PCCDUMMY68,
    #[doc = "0x114 - PCC Reserved Register 69"] pub pccdummy69: PCCDUMMY69,
    #[doc = "0x118 - PCC Reserved Register 70"] pub pccdummy70: PCCDUMMY70,
    #[doc = "0x11c - PCC Reserved Register 71"] pub pccdummy71: PCCDUMMY71,
    #[doc = "0x120 - PCC Reserved Register 72"] pub pccdummy72: PCCDUMMY72,
    #[doc = "0x124 - PCC PORTA Register"] pub pcc_porta: PCC_PORTA,
    #[doc = "0x128 - PCC PORTB Register"] pub pcc_portb: PCC_PORTB,
    #[doc = "0x12c - PCC PORTC Register"] pub pcc_portc: PCC_PORTC,
    #[doc = "0x130 - PCC PORTD Register"] pub pcc_portd: PCC_PORTD,
    #[doc = "0x134 - PCC PORTE Register"] pub pcc_porte: PCC_PORTE,
    #[doc = "0x138 - PCC Reserved Register 78"] pub pccdummy78: PCCDUMMY78,
    #[doc = "0x13c - PCC Reserved Register 79"] pub pccdummy79: PCCDUMMY79,
    #[doc = "0x140 - PCC Reserved Register 80"] pub pccdummy80: PCCDUMMY80,
    #[doc = "0x144 - PCC Reserved Register 81"] pub pccdummy81: PCCDUMMY81,
    #[doc = "0x148 - PCC Reserved Register 82"] pub pccdummy82: PCCDUMMY82,
    #[doc = "0x14c - PCC Reserved Register 83"] pub pccdummy83: PCCDUMMY83,
    #[doc = "0x150 - PCC Reserved Register 84"] pub pccdummy84: PCCDUMMY84,
    #[doc = "0x154 - PCC Reserved Register 85"] pub pccdummy85: PCCDUMMY85,
    #[doc = "0x158 - PCC Reserved Register 86"] pub pccdummy86: PCCDUMMY86,
    #[doc = "0x15c - PCC Reserved Register 87"] pub pccdummy87: PCCDUMMY87,
    #[doc = "0x160 - PCC Reserved Register 88"] pub pccdummy88: PCCDUMMY88,
    #[doc = "0x164 - PCC Reserved Register 89"] pub pccdummy89: PCCDUMMY89,
    #[doc = "0x168 - PCC FlexIO Register"] pub pcc_flexio: PCC_FLEXIO,
    #[doc = "0x16c - PCC Reserved Register 91"] pub pccdummy91: PCCDUMMY91,
    #[doc = "0x170 - PCC Reserved Register 92"] pub pccdummy92: PCCDUMMY92,
    #[doc = "0x174 - PCC Reserved Register 93"] pub pccdummy93: PCCDUMMY93,
    #[doc = "0x178 - PCC Reserved Register 94"] pub pccdummy94: PCCDUMMY94,
    #[doc = "0x17c - PCC Reserved Register 95"] pub pccdummy95: PCCDUMMY95,
    #[doc = "0x180 - PCC Reserved Register 96"] pub pccdummy96: PCCDUMMY96,
    #[doc = "0x184 - PCC EWM Register"] pub pcc_ewm: PCC_EWM,
    #[doc = "0x188 - PCC Reserved Register 98"] pub pccdummy98: PCCDUMMY98,
    #[doc = "0x18c - PCC Reserved Register 99"] pub pccdummy99: PCCDUMMY99,
    #[doc = "0x190 - PCC Reserved Register 100"] pub pccdummy100: PCCDUMMY100,
    #[doc = "0x194 - PCC Reserved Register 101"] pub pccdummy101: PCCDUMMY101,
    #[doc = "0x198 - PCC LPI2C0 Register"] pub pcc_lpi2c0: PCC_LPI2C0,
    #[doc = "0x19c - PCC Reserved Register 103"] pub pccdummy103: PCCDUMMY103,
    #[doc = "0x1a0 - PCC Reserved Register 104"] pub pccdummy104: PCCDUMMY104,
    #[doc = "0x1a4 - PCC Reserved Register 105"] pub pccdummy105: PCCDUMMY105,
    #[doc = "0x1a8 - PCC LPUART0 Register"] pub pcc_lpuart0: PCC_LPUART0,
    #[doc = "0x1ac - PCC LPUART1 Register"] pub pcc_lpuart1: PCC_LPUART1,
    #[doc = "0x1b0 - PCC LPUART2 Register"] pub pcc_lpuart2: PCC_LPUART2,
    #[doc = "0x1b4 - PCC Reserved Register 109"] pub pccdummy109: PCCDUMMY109,
    #[doc = "0x1b8 - PCC Reserved Register 110"] pub pccdummy110: PCCDUMMY110,
    #[doc = "0x1bc - PCC Reserved Register 111"] pub pccdummy111: PCCDUMMY111,
    #[doc = "0x1c0 - PCC Reserved Register 112"] pub pccdummy112: PCCDUMMY112,
    #[doc = "0x1c4 - PCC Reserved Register 113"] pub pccdummy113: PCCDUMMY113,
    #[doc = "0x1c8 - PCC Reserved Register 114"] pub pccdummy114: PCCDUMMY114,
    #[doc = "0x1cc - PCC CMP0 Register"] pub pcc_cmp0: PCC_CMP0,
}
#[doc = "PCC Reserved Register 0"]
pub struct PCCDUMMY0 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 0"]
pub mod pccdummy0;
#[doc = "PCC Reserved Register 1"]
pub struct PCCDUMMY1 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 1"]
pub mod pccdummy1;
#[doc = "PCC Reserved Register 2"]
pub struct PCCDUMMY2 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 2"]
pub mod pccdummy2;
#[doc = "PCC Reserved Register 3"]
pub struct PCCDUMMY3 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 3"]
pub mod pccdummy3;
#[doc = "PCC Reserved Register 4"]
pub struct PCCDUMMY4 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 4"]
pub mod pccdummy4;
#[doc = "PCC Reserved Register 5"]
pub struct PCCDUMMY5 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 5"]
pub mod pccdummy5;
#[doc = "PCC Reserved Register 6"]
pub struct PCCDUMMY6 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 6"]
pub mod pccdummy6;
#[doc = "PCC Reserved Register 7"]
pub struct PCCDUMMY7 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 7"]
pub mod pccdummy7;
#[doc = "PCC Reserved Register 8"]
pub struct PCCDUMMY8 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 8"]
pub mod pccdummy8;
#[doc = "PCC Reserved Register 9"]
pub struct PCCDUMMY9 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 9"]
pub mod pccdummy9;
#[doc = "PCC Reserved Register 10"]
pub struct PCCDUMMY10 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 10"]
pub mod pccdummy10;
#[doc = "PCC Reserved Register 11"]
pub struct PCCDUMMY11 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 11"]
pub mod pccdummy11;
#[doc = "PCC Reserved Register 12"]
pub struct PCCDUMMY12 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 12"]
pub mod pccdummy12;
#[doc = "PCC Reserved Register 13"]
pub struct PCCDUMMY13 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 13"]
pub mod pccdummy13;
#[doc = "PCC Reserved Register 14"]
pub struct PCCDUMMY14 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 14"]
pub mod pccdummy14;
#[doc = "PCC Reserved Register 15"]
pub struct PCCDUMMY15 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 15"]
pub mod pccdummy15;
#[doc = "PCC Reserved Register 16"]
pub struct PCCDUMMY16 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 16"]
pub mod pccdummy16;
#[doc = "PCC Reserved Register 17"]
pub struct PCCDUMMY17 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 17"]
pub mod pccdummy17;
#[doc = "PCC Reserved Register 18"]
pub struct PCCDUMMY18 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 18"]
pub mod pccdummy18;
#[doc = "PCC Reserved Register 19"]
pub struct PCCDUMMY19 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 19"]
pub mod pccdummy19;
#[doc = "PCC Reserved Register 20"]
pub struct PCCDUMMY20 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 20"]
pub mod pccdummy20;
#[doc = "PCC Reserved Register 21"]
pub struct PCCDUMMY21 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 21"]
pub mod pccdummy21;
#[doc = "PCC Reserved Register 22"]
pub struct PCCDUMMY22 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 22"]
pub mod pccdummy22;
#[doc = "PCC Reserved Register 23"]
pub struct PCCDUMMY23 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 23"]
pub mod pccdummy23;
#[doc = "PCC Reserved Register 24"]
pub struct PCCDUMMY24 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 24"]
pub mod pccdummy24;
#[doc = "PCC Reserved Register 25"]
pub struct PCCDUMMY25 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 25"]
pub mod pccdummy25;
#[doc = "PCC Reserved Register 26"]
pub struct PCCDUMMY26 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 26"]
pub mod pccdummy26;
#[doc = "PCC Reserved Register 27"]
pub struct PCCDUMMY27 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 27"]
pub mod pccdummy27;
#[doc = "PCC Reserved Register 28"]
pub struct PCCDUMMY28 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 28"]
pub mod pccdummy28;
#[doc = "PCC Reserved Register 29"]
pub struct PCCDUMMY29 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 29"]
pub mod pccdummy29;
#[doc = "PCC Reserved Register 30"]
pub struct PCCDUMMY30 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 30"]
pub mod pccdummy30;
#[doc = "PCC Reserved Register 31"]
pub struct PCCDUMMY31 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 31"]
pub mod pccdummy31;
#[doc = "PCC FTFC Register"]
pub struct PCC_FTFC {
    register: VolatileCell<u32>,
}
#[doc = "PCC FTFC Register"]
pub mod pcc_ftfc;
#[doc = "PCC DMAMUX Register"]
pub struct PCC_DMAMUX {
    register: VolatileCell<u32>,
}
#[doc = "PCC DMAMUX Register"]
pub mod pcc_dmamux;
#[doc = "PCC Reserved Register 34"]
pub struct PCCDUMMY34 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 34"]
pub mod pccdummy34;
#[doc = "PCC Reserved Register 35"]
pub struct PCCDUMMY35 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 35"]
pub mod pccdummy35;
#[doc = "PCC FlexCAN0 Register"]
pub struct PCC_FLEXCAN0 {
    register: VolatileCell<u32>,
}
#[doc = "PCC FlexCAN0 Register"]
pub mod pcc_flex_can0;
#[doc = "PCC FlexCAN1 Register"]
pub struct PCC_FLEXCAN1 {
    register: VolatileCell<u32>,
}
#[doc = "PCC FlexCAN1 Register"]
pub mod pcc_flex_can1;
#[doc = "PCC FTM3 Register"]
pub struct PCC_FTM3 {
    register: VolatileCell<u32>,
}
#[doc = "PCC FTM3 Register"]
pub mod pcc_ftm3;
#[doc = "PCC ADC1 Register"]
pub struct PCC_ADC1 {
    register: VolatileCell<u32>,
}
#[doc = "PCC ADC1 Register"]
pub mod pcc_adc1;
#[doc = "PCC Reserved Register 40"]
pub struct PCCDUMMY40 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 40"]
pub mod pccdummy40;
#[doc = "PCC Reserved Register 41"]
pub struct PCCDUMMY41 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 41"]
pub mod pccdummy41;
#[doc = "PCC Reserved Register 42"]
pub struct PCCDUMMY42 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 42"]
pub mod pccdummy42;
#[doc = "PCC FlexCAN2 Register"]
pub struct PCC_FLEXCAN2 {
    register: VolatileCell<u32>,
}
#[doc = "PCC FlexCAN2 Register"]
pub mod pcc_flex_can2;
#[doc = "PCC LPSPI0 Register"]
pub struct PCC_LPSPI0 {
    register: VolatileCell<u32>,
}
#[doc = "PCC LPSPI0 Register"]
pub mod pcc_lpspi0;
#[doc = "PCC LPSPI1 Register"]
pub struct PCC_LPSPI1 {
    register: VolatileCell<u32>,
}
#[doc = "PCC LPSPI1 Register"]
pub mod pcc_lpspi1;
#[doc = "PCC LPSPI2 Register"]
pub struct PCC_LPSPI2 {
    register: VolatileCell<u32>,
}
#[doc = "PCC LPSPI2 Register"]
pub mod pcc_lpspi2;
#[doc = "PCC Reserved Register 47"]
pub struct PCCDUMMY47 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 47"]
pub mod pccdummy47;
#[doc = "PCC Reserved Register 48"]
pub struct PCCDUMMY48 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 48"]
pub mod pccdummy48;
#[doc = "PCC PDB1 Register"]
pub struct PCC_PDB1 {
    register: VolatileCell<u32>,
}
#[doc = "PCC PDB1 Register"]
pub mod pcc_pdb1;
#[doc = "PCC CRC Register"]
pub struct PCC_CRC {
    register: VolatileCell<u32>,
}
#[doc = "PCC CRC Register"]
pub mod pcc_crc;
#[doc = "PCC Reserved Register 51"]
pub struct PCCDUMMY51 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 51"]
pub mod pccdummy51;
#[doc = "PCC Reserved Register 52"]
pub struct PCCDUMMY52 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 52"]
pub mod pccdummy52;
#[doc = "PCC Reserved Register 53"]
pub struct PCCDUMMY53 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 53"]
pub mod pccdummy53;
#[doc = "PCC PDB0 Register"]
pub struct PCC_PDB0 {
    register: VolatileCell<u32>,
}
#[doc = "PCC PDB0 Register"]
pub mod pcc_pdb0;
#[doc = "PCC LPIT Register"]
pub struct PCC_LPIT {
    register: VolatileCell<u32>,
}
#[doc = "PCC LPIT Register"]
pub mod pcc_lpit;
#[doc = "PCC FTM0 Register"]
pub struct PCC_FTM0 {
    register: VolatileCell<u32>,
}
#[doc = "PCC FTM0 Register"]
pub mod pcc_ftm0;
#[doc = "PCC FTM1 Register"]
pub struct PCC_FTM1 {
    register: VolatileCell<u32>,
}
#[doc = "PCC FTM1 Register"]
pub mod pcc_ftm1;
#[doc = "PCC FTM2 Register"]
pub struct PCC_FTM2 {
    register: VolatileCell<u32>,
}
#[doc = "PCC FTM2 Register"]
pub mod pcc_ftm2;
#[doc = "PCC ADC0 Register"]
pub struct PCC_ADC0 {
    register: VolatileCell<u32>,
}
#[doc = "PCC ADC0 Register"]
pub mod pcc_adc0;
#[doc = "PCC Reserved Register 60"]
pub struct PCCDUMMY60 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 60"]
pub mod pccdummy60;
#[doc = "PCC RTC Register"]
pub struct PCC_RTC {
    register: VolatileCell<u32>,
}
#[doc = "PCC RTC Register"]
pub mod pcc_rtc;
#[doc = "PCC Reserved Register 62"]
pub struct PCCDUMMY62 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 62"]
pub mod pccdummy62;
#[doc = "PCC Reserved Register 63"]
pub struct PCCDUMMY63 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 63"]
pub mod pccdummy63;
#[doc = "PCC LPTMR0 Register"]
pub struct PCC_LPTMR0 {
    register: VolatileCell<u32>,
}
#[doc = "PCC LPTMR0 Register"]
pub mod pcc_lptmr0;
#[doc = "PCC Reserved Register 65"]
pub struct PCCDUMMY65 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 65"]
pub mod pccdummy65;
#[doc = "PCC Reserved Register 66"]
pub struct PCCDUMMY66 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 66"]
pub mod pccdummy66;
#[doc = "PCC Reserved Register 67"]
pub struct PCCDUMMY67 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 67"]
pub mod pccdummy67;
#[doc = "PCC Reserved Register 68"]
pub struct PCCDUMMY68 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 68"]
pub mod pccdummy68;
#[doc = "PCC Reserved Register 69"]
pub struct PCCDUMMY69 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 69"]
pub mod pccdummy69;
#[doc = "PCC Reserved Register 70"]
pub struct PCCDUMMY70 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 70"]
pub mod pccdummy70;
#[doc = "PCC Reserved Register 71"]
pub struct PCCDUMMY71 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 71"]
pub mod pccdummy71;
#[doc = "PCC Reserved Register 72"]
pub struct PCCDUMMY72 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 72"]
pub mod pccdummy72;
#[doc = "PCC PORTA Register"]
pub struct PCC_PORTA {
    register: VolatileCell<u32>,
}
#[doc = "PCC PORTA Register"]
pub mod pcc_porta;
#[doc = "PCC PORTB Register"]
pub struct PCC_PORTB {
    register: VolatileCell<u32>,
}
#[doc = "PCC PORTB Register"]
pub mod pcc_portb;
#[doc = "PCC PORTC Register"]
pub struct PCC_PORTC {
    register: VolatileCell<u32>,
}
#[doc = "PCC PORTC Register"]
pub mod pcc_portc;
#[doc = "PCC PORTD Register"]
pub struct PCC_PORTD {
    register: VolatileCell<u32>,
}
#[doc = "PCC PORTD Register"]
pub mod pcc_portd;
#[doc = "PCC PORTE Register"]
pub struct PCC_PORTE {
    register: VolatileCell<u32>,
}
#[doc = "PCC PORTE Register"]
pub mod pcc_porte;
#[doc = "PCC Reserved Register 78"]
pub struct PCCDUMMY78 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 78"]
pub mod pccdummy78;
#[doc = "PCC Reserved Register 79"]
pub struct PCCDUMMY79 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 79"]
pub mod pccdummy79;
#[doc = "PCC Reserved Register 80"]
pub struct PCCDUMMY80 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 80"]
pub mod pccdummy80;
#[doc = "PCC Reserved Register 81"]
pub struct PCCDUMMY81 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 81"]
pub mod pccdummy81;
#[doc = "PCC Reserved Register 82"]
pub struct PCCDUMMY82 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 82"]
pub mod pccdummy82;
#[doc = "PCC Reserved Register 83"]
pub struct PCCDUMMY83 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 83"]
pub mod pccdummy83;
#[doc = "PCC Reserved Register 84"]
pub struct PCCDUMMY84 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 84"]
pub mod pccdummy84;
#[doc = "PCC Reserved Register 85"]
pub struct PCCDUMMY85 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 85"]
pub mod pccdummy85;
#[doc = "PCC Reserved Register 86"]
pub struct PCCDUMMY86 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 86"]
pub mod pccdummy86;
#[doc = "PCC Reserved Register 87"]
pub struct PCCDUMMY87 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 87"]
pub mod pccdummy87;
#[doc = "PCC Reserved Register 88"]
pub struct PCCDUMMY88 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 88"]
pub mod pccdummy88;
#[doc = "PCC Reserved Register 89"]
pub struct PCCDUMMY89 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 89"]
pub mod pccdummy89;
#[doc = "PCC FlexIO Register"]
pub struct PCC_FLEXIO {
    register: VolatileCell<u32>,
}
#[doc = "PCC FlexIO Register"]
pub mod pcc_flexio;
#[doc = "PCC Reserved Register 91"]
pub struct PCCDUMMY91 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 91"]
pub mod pccdummy91;
#[doc = "PCC Reserved Register 92"]
pub struct PCCDUMMY92 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 92"]
pub mod pccdummy92;
#[doc = "PCC Reserved Register 93"]
pub struct PCCDUMMY93 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 93"]
pub mod pccdummy93;
#[doc = "PCC Reserved Register 94"]
pub struct PCCDUMMY94 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 94"]
pub mod pccdummy94;
#[doc = "PCC Reserved Register 95"]
pub struct PCCDUMMY95 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 95"]
pub mod pccdummy95;
#[doc = "PCC Reserved Register 96"]
pub struct PCCDUMMY96 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 96"]
pub mod pccdummy96;
#[doc = "PCC EWM Register"]
pub struct PCC_EWM {
    register: VolatileCell<u32>,
}
#[doc = "PCC EWM Register"]
pub mod pcc_ewm;
#[doc = "PCC Reserved Register 98"]
pub struct PCCDUMMY98 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 98"]
pub mod pccdummy98;
#[doc = "PCC Reserved Register 99"]
pub struct PCCDUMMY99 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 99"]
pub mod pccdummy99;
#[doc = "PCC Reserved Register 100"]
pub struct PCCDUMMY100 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 100"]
pub mod pccdummy100;
#[doc = "PCC Reserved Register 101"]
pub struct PCCDUMMY101 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 101"]
pub mod pccdummy101;
#[doc = "PCC LPI2C0 Register"]
pub struct PCC_LPI2C0 {
    register: VolatileCell<u32>,
}
#[doc = "PCC LPI2C0 Register"]
pub mod pcc_lpi2c0;
#[doc = "PCC Reserved Register 103"]
pub struct PCCDUMMY103 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 103"]
pub mod pccdummy103;
#[doc = "PCC Reserved Register 104"]
pub struct PCCDUMMY104 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 104"]
pub mod pccdummy104;
#[doc = "PCC Reserved Register 105"]
pub struct PCCDUMMY105 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 105"]
pub mod pccdummy105;
#[doc = "PCC LPUART0 Register"]
pub struct PCC_LPUART0 {
    register: VolatileCell<u32>,
}
#[doc = "PCC LPUART0 Register"]
pub mod pcc_lpuart0;
#[doc = "PCC LPUART1 Register"]
pub struct PCC_LPUART1 {
    register: VolatileCell<u32>,
}
#[doc = "PCC LPUART1 Register"]
pub mod pcc_lpuart1;
#[doc = "PCC LPUART2 Register"]
pub struct PCC_LPUART2 {
    register: VolatileCell<u32>,
}
#[doc = "PCC LPUART2 Register"]
pub mod pcc_lpuart2;
#[doc = "PCC Reserved Register 109"]
pub struct PCCDUMMY109 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 109"]
pub mod pccdummy109;
#[doc = "PCC Reserved Register 110"]
pub struct PCCDUMMY110 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 110"]
pub mod pccdummy110;
#[doc = "PCC Reserved Register 111"]
pub struct PCCDUMMY111 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 111"]
pub mod pccdummy111;
#[doc = "PCC Reserved Register 112"]
pub struct PCCDUMMY112 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 112"]
pub mod pccdummy112;
#[doc = "PCC Reserved Register 113"]
pub struct PCCDUMMY113 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 113"]
pub mod pccdummy113;
#[doc = "PCC Reserved Register 114"]
pub struct PCCDUMMY114 {
    register: VolatileCell<u32>,
}
#[doc = "PCC Reserved Register 114"]
pub mod pccdummy114;
#[doc = "PCC CMP0 Register"]
pub struct PCC_CMP0 {
    register: VolatileCell<u32>,
}
#[doc = "PCC CMP0 Register"]
pub mod pcc_cmp0;
