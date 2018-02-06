use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Status and Control Register 1"]
    pub sc1a: SC1,
    #[doc = "0x04 - ADC Status and Control Register 1"]
    pub sc1b: SC1,
    #[doc = "0x08 - ADC Status and Control Register 1"]
    pub sc1c: SC1,
    #[doc = "0x0c - ADC Status and Control Register 1"]
    pub sc1d: SC1,
    #[doc = "0x10 - ADC Status and Control Register 1"]
    pub sc1e: SC1,
    #[doc = "0x14 - ADC Status and Control Register 1"]
    pub sc1f: SC1,
    #[doc = "0x18 - ADC Status and Control Register 1"]
    pub sc1g: SC1,
    #[doc = "0x1c - ADC Status and Control Register 1"]
    pub sc1h: SC1,
    #[doc = "0x20 - ADC Status and Control Register 1"]
    pub sc1i: SC1,
    #[doc = "0x24 - ADC Status and Control Register 1"]
    pub sc1j: SC1,
    #[doc = "0x28 - ADC Status and Control Register 1"]
    pub sc1k: SC1,
    #[doc = "0x2c - ADC Status and Control Register 1"]
    pub sc1l: SC1,
    #[doc = "0x30 - ADC Status and Control Register 1"]
    pub sc1m: SC1,
    #[doc = "0x34 - ADC Status and Control Register 1"]
    pub sc1n: SC1,
    #[doc = "0x38 - ADC Status and Control Register 1"]
    pub sc1o: SC1,
    #[doc = "0x3c - ADC Status and Control Register 1"]
    pub sc1p: SC1,
    #[doc = "0x40 - ADC Configuration Register 1"]
    pub cfg1: CFG1,
    #[doc = "0x44 - ADC Configuration Register 2"]
    pub cfg2: CFG2,
    #[doc = "0x48 - ADC Data Result Registers"]
    pub ra: R,
    #[doc = "0x4c - ADC Data Result Registers"]
    pub rb: R,
    #[doc = "0x50 - ADC Data Result Registers"]
    pub rc: R,
    #[doc = "0x54 - ADC Data Result Registers"]
    pub rd: R,
    #[doc = "0x58 - ADC Data Result Registers"]
    pub re: R,
    #[doc = "0x5c - ADC Data Result Registers"]
    pub rf: R,
    #[doc = "0x60 - ADC Data Result Registers"]
    pub rg: R,
    #[doc = "0x64 - ADC Data Result Registers"]
    pub rh: R,
    #[doc = "0x68 - ADC Data Result Registers"]
    pub ri: R,
    #[doc = "0x6c - ADC Data Result Registers"]
    pub rj: R,
    #[doc = "0x70 - ADC Data Result Registers"]
    pub rk: R,
    #[doc = "0x74 - ADC Data Result Registers"]
    pub rl: R,
    #[doc = "0x78 - ADC Data Result Registers"]
    pub rm: R,
    #[doc = "0x7c - ADC Data Result Registers"]
    pub rn: R,
    #[doc = "0x80 - ADC Data Result Registers"]
    pub ro: R,
    #[doc = "0x84 - ADC Data Result Registers"]
    pub rp: R,
    #[doc = "0x88 - Compare Value Registers"]
    pub cv1: CV,
    #[doc = "0x8c - Compare Value Registers"]
    pub cv2: CV,
    #[doc = "0x90 - Status and Control Register 2"]
    pub sc2: SC2,
    #[doc = "0x94 - Status and Control Register 3"]
    pub sc3: SC3,
    #[doc = "0x98 - BASE Offset Register"]
    pub base_ofs: BASE_OFS,
    #[doc = "0x9c - ADC Offset Correction Register"]
    pub ofs: OFS,
    #[doc = "0xa0 - USER Offset Correction Register"]
    pub usr_ofs: USR_OFS,
    #[doc = "0xa4 - ADC X Offset Correction Register"]
    pub xofs: XOFS,
    #[doc = "0xa8 - ADC Y Offset Correction Register"]
    pub yofs: YOFS,
    #[doc = "0xac - ADC Gain Register"]
    pub g: G,
    #[doc = "0xb0 - ADC User Gain Register"]
    pub ug: UG,
    #[doc = "0xb4 - ADC General Calibration Value Register S"]
    pub clps: CLPS,
    #[doc = "0xb8 - ADC Plus-Side General Calibration Value Register 3"]
    pub clp3: CLP3,
    #[doc = "0xbc - ADC Plus-Side General Calibration Value Register 2"]
    pub clp2: CLP2,
    #[doc = "0xc0 - ADC Plus-Side General Calibration Value Register 1"]
    pub clp1: CLP1,
    #[doc = "0xc4 - ADC Plus-Side General Calibration Value Register 0"]
    pub clp0: CLP0,
    #[doc = "0xc8 - ADC Plus-Side General Calibration Value Register X"]
    pub clpx: CLPX,
    #[doc = "0xcc - ADC Plus-Side General Calibration Value Register 9"]
    pub clp9: CLP9,
    #[doc = "0xd0 - ADC General Calibration Offset Value Register S"]
    pub clps_ofs: CLPS_OFS,
    #[doc = "0xd4 - ADC Plus-Side General Calibration Offset Value Register 3"]
    pub clp3_ofs: CLP3_OFS,
    #[doc = "0xd8 - ADC Plus-Side General Calibration Offset Value Register 2"]
    pub clp2_ofs: CLP2_OFS,
    #[doc = "0xdc - ADC Plus-Side General Calibration Offset Value Register 1"]
    pub clp1_ofs: CLP1_OFS,
    #[doc = "0xe0 - ADC Plus-Side General Calibration Offset Value Register 0"]
    pub clp0_ofs: CLP0_OFS,
    #[doc = "0xe4 - ADC Plus-Side General Calibration Offset Value Register X"]
    pub clpx_ofs: CLPX_OFS,
    #[doc = "0xe8 - ADC Plus-Side General Calibration Offset Value Register 9"]
    pub clp9_ofs: CLP9_OFS,
}
#[doc = "ADC Status and Control Register 1"]
pub struct SC1 {
    register: VolatileCell<u32>,
}
#[doc = "ADC Status and Control Register 1"]
pub mod sc1;
#[doc = "ADC Configuration Register 1"]
pub struct CFG1 {
    register: VolatileCell<u32>,
}
#[doc = "ADC Configuration Register 1"]
pub mod cfg1;
#[doc = "ADC Configuration Register 2"]
pub struct CFG2 {
    register: VolatileCell<u32>,
}
#[doc = "ADC Configuration Register 2"]
pub mod cfg2;
#[doc = "ADC Data Result Registers"]
pub struct R {
    register: VolatileCell<u32>,
}
#[doc = "ADC Data Result Registers"]
pub mod r;
#[doc = "Compare Value Registers"]
pub struct CV {
    register: VolatileCell<u32>,
}
#[doc = "Compare Value Registers"]
pub mod cv;
#[doc = "Status and Control Register 2"]
pub struct SC2 {
    register: VolatileCell<u32>,
}
#[doc = "Status and Control Register 2"]
pub mod sc2;
#[doc = "Status and Control Register 3"]
pub struct SC3 {
    register: VolatileCell<u32>,
}
#[doc = "Status and Control Register 3"]
pub mod sc3;
#[doc = "BASE Offset Register"]
pub struct BASE_OFS {
    register: VolatileCell<u32>,
}
#[doc = "BASE Offset Register"]
pub mod base_ofs;
#[doc = "ADC Offset Correction Register"]
pub struct OFS {
    register: VolatileCell<u32>,
}
#[doc = "ADC Offset Correction Register"]
pub mod ofs;
#[doc = "USER Offset Correction Register"]
pub struct USR_OFS {
    register: VolatileCell<u32>,
}
#[doc = "USER Offset Correction Register"]
pub mod usr_ofs;
#[doc = "ADC X Offset Correction Register"]
pub struct XOFS {
    register: VolatileCell<u32>,
}
#[doc = "ADC X Offset Correction Register"]
pub mod xofs;
#[doc = "ADC Y Offset Correction Register"]
pub struct YOFS {
    register: VolatileCell<u32>,
}
#[doc = "ADC Y Offset Correction Register"]
pub mod yofs;
#[doc = "ADC Gain Register"]
pub struct G {
    register: VolatileCell<u32>,
}
#[doc = "ADC Gain Register"]
pub mod g;
#[doc = "ADC User Gain Register"]
pub struct UG {
    register: VolatileCell<u32>,
}
#[doc = "ADC User Gain Register"]
pub mod ug;
#[doc = "ADC General Calibration Value Register S"]
pub struct CLPS {
    register: VolatileCell<u32>,
}
#[doc = "ADC General Calibration Value Register S"]
pub mod clps;
#[doc = "ADC Plus-Side General Calibration Value Register 3"]
pub struct CLP3 {
    register: VolatileCell<u32>,
}
#[doc = "ADC Plus-Side General Calibration Value Register 3"]
pub mod clp3;
#[doc = "ADC Plus-Side General Calibration Value Register 2"]
pub struct CLP2 {
    register: VolatileCell<u32>,
}
#[doc = "ADC Plus-Side General Calibration Value Register 2"]
pub mod clp2;
#[doc = "ADC Plus-Side General Calibration Value Register 1"]
pub struct CLP1 {
    register: VolatileCell<u32>,
}
#[doc = "ADC Plus-Side General Calibration Value Register 1"]
pub mod clp1;
#[doc = "ADC Plus-Side General Calibration Value Register 0"]
pub struct CLP0 {
    register: VolatileCell<u32>,
}
#[doc = "ADC Plus-Side General Calibration Value Register 0"]
pub mod clp0;
#[doc = "ADC Plus-Side General Calibration Value Register X"]
pub struct CLPX {
    register: VolatileCell<u32>,
}
#[doc = "ADC Plus-Side General Calibration Value Register X"]
pub mod clpx;
#[doc = "ADC Plus-Side General Calibration Value Register 9"]
pub struct CLP9 {
    register: VolatileCell<u32>,
}
#[doc = "ADC Plus-Side General Calibration Value Register 9"]
pub mod clp9;
#[doc = "ADC General Calibration Offset Value Register S"]
pub struct CLPS_OFS {
    register: VolatileCell<u32>,
}
#[doc = "ADC General Calibration Offset Value Register S"]
pub mod clps_ofs;
#[doc = "ADC Plus-Side General Calibration Offset Value Register 3"]
pub struct CLP3_OFS {
    register: VolatileCell<u32>,
}
#[doc = "ADC Plus-Side General Calibration Offset Value Register 3"]
pub mod clp3_ofs;
#[doc = "ADC Plus-Side General Calibration Offset Value Register 2"]
pub struct CLP2_OFS {
    register: VolatileCell<u32>,
}
#[doc = "ADC Plus-Side General Calibration Offset Value Register 2"]
pub mod clp2_ofs;
#[doc = "ADC Plus-Side General Calibration Offset Value Register 1"]
pub struct CLP1_OFS {
    register: VolatileCell<u32>,
}
#[doc = "ADC Plus-Side General Calibration Offset Value Register 1"]
pub mod clp1_ofs;
#[doc = "ADC Plus-Side General Calibration Offset Value Register 0"]
pub struct CLP0_OFS {
    register: VolatileCell<u32>,
}
#[doc = "ADC Plus-Side General Calibration Offset Value Register 0"]
pub mod clp0_ofs;
#[doc = "ADC Plus-Side General Calibration Offset Value Register X"]
pub struct CLPX_OFS {
    register: VolatileCell<u32>,
}
#[doc = "ADC Plus-Side General Calibration Offset Value Register X"]
pub mod clpx_ofs;
#[doc = "ADC Plus-Side General Calibration Offset Value Register 9"]
pub struct CLP9_OFS {
    register: VolatileCell<u32>,
}
#[doc = "ADC Plus-Side General Calibration Offset Value Register 9"]
pub mod clp9_ofs;
