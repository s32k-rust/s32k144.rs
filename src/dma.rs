#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Error Status Register"]
    pub es: ES,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - Enable Request Register"]
    pub erq: ERQ,
    _reserved3: [u8; 4usize],
    #[doc = "0x14 - Enable Error Interrupt Register"]
    pub eei: EEI,
    #[doc = "0x18 - Clear Enable Error Interrupt Register"]
    pub ceei: CEEI,
    #[doc = "0x19 - Set Enable Error Interrupt Register"]
    pub seei: SEEI,
    #[doc = "0x1a - Clear Enable Request Register"]
    pub cerq: CERQ,
    #[doc = "0x1b - Set Enable Request Register"]
    pub serq: SERQ,
    #[doc = "0x1c - Clear DONE Status Bit Register"]
    pub cdne: CDNE,
    #[doc = "0x1d - Set START Bit Register"]
    pub ssrt: SSRT,
    #[doc = "0x1e - Clear Error Register"]
    pub cerr: CERR,
    #[doc = "0x1f - Clear Interrupt Request Register"]
    pub cint: CINT,
    _reserved12: [u8; 4usize],
    #[doc = "0x24 - Interrupt Request Register"]
    pub int: INT,
    _reserved13: [u8; 4usize],
    #[doc = "0x2c - Error Register"]
    pub err: ERR,
    _reserved14: [u8; 4usize],
    #[doc = "0x34 - Hardware Request Status Register"]
    pub hrs: HRS,
    _reserved15: [u8; 12usize],
    #[doc = "0x44 - Enable Asynchronous Request in Stop Register"]
    pub ears: EARS,
    _reserved16: [u8; 184usize],
    #[doc = "0x100 - Channel n Priority Register"]
    pub dchpri3: DCHPRI3,
    #[doc = "0x101 - Channel n Priority Register"]
    pub dchpri2: DCHPRI2,
    #[doc = "0x102 - Channel n Priority Register"]
    pub dchpri1: DCHPRI1,
    #[doc = "0x103 - Channel n Priority Register"]
    pub dchpri0: DCHPRI0,
    #[doc = "0x104 - Channel n Priority Register"]
    pub dchpri7: DCHPRI7,
    #[doc = "0x105 - Channel n Priority Register"]
    pub dchpri6: DCHPRI6,
    #[doc = "0x106 - Channel n Priority Register"]
    pub dchpri5: DCHPRI5,
    #[doc = "0x107 - Channel n Priority Register"]
    pub dchpri4: DCHPRI4,
    #[doc = "0x108 - Channel n Priority Register"]
    pub dchpri11: DCHPRI11,
    #[doc = "0x109 - Channel n Priority Register"]
    pub dchpri10: DCHPRI10,
    #[doc = "0x10a - Channel n Priority Register"]
    pub dchpri9: DCHPRI9,
    #[doc = "0x10b - Channel n Priority Register"]
    pub dchpri8: DCHPRI8,
    #[doc = "0x10c - Channel n Priority Register"]
    pub dchpri15: DCHPRI15,
    #[doc = "0x10d - Channel n Priority Register"]
    pub dchpri14: DCHPRI14,
    #[doc = "0x10e - Channel n Priority Register"]
    pub dchpri13: DCHPRI13,
    #[doc = "0x10f - Channel n Priority Register"]
    pub dchpri12: DCHPRI12,
    _reserved32: [u8; 3824usize],
    #[doc = "0x1000 - TCDn Source Address"]
    pub tcd0_saddr: TCD_SADDR,
    #[doc = "0x1004 - TCDn Signed Source Address Offset"]
    pub tcd0_soff: TCD_SOFF,
    #[doc = "0x1006 - TCDn Transfer Attributes"]
    pub tcd0_attr: TCD_ATTR,
    _reserved_35_tcd0_nbytes: [u8; 4usize],
    #[doc = "0x100c - TCDn Last Source Address Adjustment"]
    pub tcd0_slast: TCD_SLAST,
    #[doc = "0x1010 - TCDn Destination Address"]
    pub tcd0_daddr: TCD_DADDR,
    #[doc = "0x1014 - TCDn Signed Destination Address Offset"]
    pub tcd0_doff: TCD_DOFF,
    _reserved_39_tcd0_citer: [u8; 2usize],
    #[doc = "0x1018 - TCDn Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd0_dlastsga: TCD_DLASTSGA,
    #[doc = "0x101c - TCDn Control and Status"]
    pub tcd0_csr: TCD_CSR,
    _reserved_42_tcd0_biter: [u8; 2usize],
    #[doc = "0x1020 - TCDn Source Address"]
    pub tcd1_saddr: TCD_SADDR,
    #[doc = "0x1024 - TCDn Signed Source Address Offset"]
    pub tcd1_soff: TCD_SOFF,
    #[doc = "0x1026 - TCDn Transfer Attributes"]
    pub tcd1_attr: TCD_ATTR,
    _reserved_46_tcd1_nbytes: [u8; 4usize],
    #[doc = "0x102c - TCDn Last Source Address Adjustment"]
    pub tcd1_slast: TCD_SLAST,
    #[doc = "0x1030 - TCDn Destination Address"]
    pub tcd1_daddr: TCD_DADDR,
    #[doc = "0x1034 - TCDn Signed Destination Address Offset"]
    pub tcd1_doff: TCD_DOFF,
    _reserved_50_tcd1_citer: [u8; 2usize],
    #[doc = "0x1038 - TCDn Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd1_dlastsga: TCD_DLASTSGA,
    #[doc = "0x103c - TCDn Control and Status"]
    pub tcd1_csr: TCD_CSR,
    _reserved_53_tcd1_biter: [u8; 2usize],
    #[doc = "0x1040 - TCDn Source Address"]
    pub tcd2_saddr: TCD_SADDR,
    #[doc = "0x1044 - TCDn Signed Source Address Offset"]
    pub tcd2_soff: TCD_SOFF,
    #[doc = "0x1046 - TCDn Transfer Attributes"]
    pub tcd2_attr: TCD_ATTR,
    _reserved_57_tcd2_nbytes: [u8; 4usize],
    #[doc = "0x104c - TCDn Last Source Address Adjustment"]
    pub tcd2_slast: TCD_SLAST,
    #[doc = "0x1050 - TCDn Destination Address"]
    pub tcd2_daddr: TCD_DADDR,
    #[doc = "0x1054 - TCDn Signed Destination Address Offset"]
    pub tcd2_doff: TCD_DOFF,
    _reserved_61_tcd2_citer: [u8; 2usize],
    #[doc = "0x1058 - TCDn Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd2_dlastsga: TCD_DLASTSGA,
    #[doc = "0x105c - TCDn Control and Status"]
    pub tcd2_csr: TCD_CSR,
    _reserved_64_tcd2_biter: [u8; 2usize],
    #[doc = "0x1060 - TCDn Source Address"]
    pub tcd3_saddr: TCD_SADDR,
    #[doc = "0x1064 - TCDn Signed Source Address Offset"]
    pub tcd3_soff: TCD_SOFF,
    #[doc = "0x1066 - TCDn Transfer Attributes"]
    pub tcd3_attr: TCD_ATTR,
    _reserved_68_tcd3_nbytes: [u8; 4usize],
    #[doc = "0x106c - TCDn Last Source Address Adjustment"]
    pub tcd3_slast: TCD_SLAST,
    #[doc = "0x1070 - TCDn Destination Address"]
    pub tcd3_daddr: TCD_DADDR,
    #[doc = "0x1074 - TCDn Signed Destination Address Offset"]
    pub tcd3_doff: TCD_DOFF,
    _reserved_72_tcd3_citer: [u8; 2usize],
    #[doc = "0x1078 - TCDn Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd3_dlastsga: TCD_DLASTSGA,
    #[doc = "0x107c - TCDn Control and Status"]
    pub tcd3_csr: TCD_CSR,
    _reserved_75_tcd3_biter: [u8; 2usize],
    #[doc = "0x1080 - TCDn Source Address"]
    pub tcd4_saddr: TCD_SADDR,
    #[doc = "0x1084 - TCDn Signed Source Address Offset"]
    pub tcd4_soff: TCD_SOFF,
    #[doc = "0x1086 - TCDn Transfer Attributes"]
    pub tcd4_attr: TCD_ATTR,
    _reserved_79_tcd4_nbytes: [u8; 4usize],
    #[doc = "0x108c - TCDn Last Source Address Adjustment"]
    pub tcd4_slast: TCD_SLAST,
    #[doc = "0x1090 - TCDn Destination Address"]
    pub tcd4_daddr: TCD_DADDR,
    #[doc = "0x1094 - TCDn Signed Destination Address Offset"]
    pub tcd4_doff: TCD_DOFF,
    _reserved_83_tcd4_citer: [u8; 2usize],
    #[doc = "0x1098 - TCDn Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd4_dlastsga: TCD_DLASTSGA,
    #[doc = "0x109c - TCDn Control and Status"]
    pub tcd4_csr: TCD_CSR,
    _reserved_86_tcd4_biter: [u8; 2usize],
    #[doc = "0x10a0 - TCDn Source Address"]
    pub tcd5_saddr: TCD_SADDR,
    #[doc = "0x10a4 - TCDn Signed Source Address Offset"]
    pub tcd5_soff: TCD_SOFF,
    #[doc = "0x10a6 - TCDn Transfer Attributes"]
    pub tcd5_attr: TCD_ATTR,
    _reserved_90_tcd5_nbytes: [u8; 4usize],
    #[doc = "0x10ac - TCDn Last Source Address Adjustment"]
    pub tcd5_slast: TCD_SLAST,
    #[doc = "0x10b0 - TCDn Destination Address"]
    pub tcd5_daddr: TCD_DADDR,
    #[doc = "0x10b4 - TCDn Signed Destination Address Offset"]
    pub tcd5_doff: TCD_DOFF,
    _reserved_94_tcd5_citer: [u8; 2usize],
    #[doc = "0x10b8 - TCDn Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd5_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10bc - TCDn Control and Status"]
    pub tcd5_csr: TCD_CSR,
    _reserved_97_tcd5_biter: [u8; 2usize],
    #[doc = "0x10c0 - TCDn Source Address"]
    pub tcd6_saddr: TCD_SADDR,
    #[doc = "0x10c4 - TCDn Signed Source Address Offset"]
    pub tcd6_soff: TCD_SOFF,
    #[doc = "0x10c6 - TCDn Transfer Attributes"]
    pub tcd6_attr: TCD_ATTR,
    _reserved_101_tcd6_nbytes: [u8; 4usize],
    #[doc = "0x10cc - TCDn Last Source Address Adjustment"]
    pub tcd6_slast: TCD_SLAST,
    #[doc = "0x10d0 - TCDn Destination Address"]
    pub tcd6_daddr: TCD_DADDR,
    #[doc = "0x10d4 - TCDn Signed Destination Address Offset"]
    pub tcd6_doff: TCD_DOFF,
    _reserved_105_tcd6_citer: [u8; 2usize],
    #[doc = "0x10d8 - TCDn Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd6_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10dc - TCDn Control and Status"]
    pub tcd6_csr: TCD_CSR,
    _reserved_108_tcd6_biter: [u8; 2usize],
    #[doc = "0x10e0 - TCDn Source Address"]
    pub tcd7_saddr: TCD_SADDR,
    #[doc = "0x10e4 - TCDn Signed Source Address Offset"]
    pub tcd7_soff: TCD_SOFF,
    #[doc = "0x10e6 - TCDn Transfer Attributes"]
    pub tcd7_attr: TCD_ATTR,
    _reserved_112_tcd7_nbytes: [u8; 4usize],
    #[doc = "0x10ec - TCDn Last Source Address Adjustment"]
    pub tcd7_slast: TCD_SLAST,
    #[doc = "0x10f0 - TCDn Destination Address"]
    pub tcd7_daddr: TCD_DADDR,
    #[doc = "0x10f4 - TCDn Signed Destination Address Offset"]
    pub tcd7_doff: TCD_DOFF,
    _reserved_116_tcd7_citer: [u8; 2usize],
    #[doc = "0x10f8 - TCDn Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd7_dlastsga: TCD_DLASTSGA,
    #[doc = "0x10fc - TCDn Control and Status"]
    pub tcd7_csr: TCD_CSR,
    _reserved_119_tcd7_biter: [u8; 2usize],
    #[doc = "0x1100 - TCDn Source Address"]
    pub tcd8_saddr: TCD_SADDR,
    #[doc = "0x1104 - TCDn Signed Source Address Offset"]
    pub tcd8_soff: TCD_SOFF,
    #[doc = "0x1106 - TCDn Transfer Attributes"]
    pub tcd8_attr: TCD_ATTR,
    _reserved_123_tcd8_nbytes: [u8; 4usize],
    #[doc = "0x110c - TCDn Last Source Address Adjustment"]
    pub tcd8_slast: TCD_SLAST,
    #[doc = "0x1110 - TCDn Destination Address"]
    pub tcd8_daddr: TCD_DADDR,
    #[doc = "0x1114 - TCDn Signed Destination Address Offset"]
    pub tcd8_doff: TCD_DOFF,
    _reserved_127_tcd8_citer: [u8; 2usize],
    #[doc = "0x1118 - TCDn Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd8_dlastsga: TCD_DLASTSGA,
    #[doc = "0x111c - TCDn Control and Status"]
    pub tcd8_csr: TCD_CSR,
    _reserved_130_tcd8_biter: [u8; 2usize],
    #[doc = "0x1120 - TCDn Source Address"]
    pub tcd9_saddr: TCD_SADDR,
    #[doc = "0x1124 - TCDn Signed Source Address Offset"]
    pub tcd9_soff: TCD_SOFF,
    #[doc = "0x1126 - TCDn Transfer Attributes"]
    pub tcd9_attr: TCD_ATTR,
    _reserved_134_tcd9_nbytes: [u8; 4usize],
    #[doc = "0x112c - TCDn Last Source Address Adjustment"]
    pub tcd9_slast: TCD_SLAST,
    #[doc = "0x1130 - TCDn Destination Address"]
    pub tcd9_daddr: TCD_DADDR,
    #[doc = "0x1134 - TCDn Signed Destination Address Offset"]
    pub tcd9_doff: TCD_DOFF,
    _reserved_138_tcd9_citer: [u8; 2usize],
    #[doc = "0x1138 - TCDn Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd9_dlastsga: TCD_DLASTSGA,
    #[doc = "0x113c - TCDn Control and Status"]
    pub tcd9_csr: TCD_CSR,
    _reserved_141_tcd9_biter: [u8; 2usize],
    #[doc = "0x1140 - TCDn Source Address"]
    pub tcd10_saddr: TCD_SADDR,
    #[doc = "0x1144 - TCDn Signed Source Address Offset"]
    pub tcd10_soff: TCD_SOFF,
    #[doc = "0x1146 - TCDn Transfer Attributes"]
    pub tcd10_attr: TCD_ATTR,
    _reserved_145_tcd10_nbytes: [u8; 4usize],
    #[doc = "0x114c - TCDn Last Source Address Adjustment"]
    pub tcd10_slast: TCD_SLAST,
    #[doc = "0x1150 - TCDn Destination Address"]
    pub tcd10_daddr: TCD_DADDR,
    #[doc = "0x1154 - TCDn Signed Destination Address Offset"]
    pub tcd10_doff: TCD_DOFF,
    _reserved_149_tcd10_citer: [u8; 2usize],
    #[doc = "0x1158 - TCDn Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd10_dlastsga: TCD_DLASTSGA,
    #[doc = "0x115c - TCDn Control and Status"]
    pub tcd10_csr: TCD_CSR,
    _reserved_152_tcd10_biter: [u8; 2usize],
    #[doc = "0x1160 - TCDn Source Address"]
    pub tcd11_saddr: TCD_SADDR,
    #[doc = "0x1164 - TCDn Signed Source Address Offset"]
    pub tcd11_soff: TCD_SOFF,
    #[doc = "0x1166 - TCDn Transfer Attributes"]
    pub tcd11_attr: TCD_ATTR,
    _reserved_156_tcd11_nbytes: [u8; 4usize],
    #[doc = "0x116c - TCDn Last Source Address Adjustment"]
    pub tcd11_slast: TCD_SLAST,
    #[doc = "0x1170 - TCDn Destination Address"]
    pub tcd11_daddr: TCD_DADDR,
    #[doc = "0x1174 - TCDn Signed Destination Address Offset"]
    pub tcd11_doff: TCD_DOFF,
    _reserved_160_tcd11_citer: [u8; 2usize],
    #[doc = "0x1178 - TCDn Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd11_dlastsga: TCD_DLASTSGA,
    #[doc = "0x117c - TCDn Control and Status"]
    pub tcd11_csr: TCD_CSR,
    _reserved_163_tcd11_biter: [u8; 2usize],
    #[doc = "0x1180 - TCDn Source Address"]
    pub tcd12_saddr: TCD_SADDR,
    #[doc = "0x1184 - TCDn Signed Source Address Offset"]
    pub tcd12_soff: TCD_SOFF,
    #[doc = "0x1186 - TCDn Transfer Attributes"]
    pub tcd12_attr: TCD_ATTR,
    _reserved_167_tcd12_nbytes: [u8; 4usize],
    #[doc = "0x118c - TCDn Last Source Address Adjustment"]
    pub tcd12_slast: TCD_SLAST,
    #[doc = "0x1190 - TCDn Destination Address"]
    pub tcd12_daddr: TCD_DADDR,
    #[doc = "0x1194 - TCDn Signed Destination Address Offset"]
    pub tcd12_doff: TCD_DOFF,
    _reserved_171_tcd12_citer: [u8; 2usize],
    #[doc = "0x1198 - TCDn Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd12_dlastsga: TCD_DLASTSGA,
    #[doc = "0x119c - TCDn Control and Status"]
    pub tcd12_csr: TCD_CSR,
    _reserved_174_tcd12_biter: [u8; 2usize],
    #[doc = "0x11a0 - TCDn Source Address"]
    pub tcd13_saddr: TCD_SADDR,
    #[doc = "0x11a4 - TCDn Signed Source Address Offset"]
    pub tcd13_soff: TCD_SOFF,
    #[doc = "0x11a6 - TCDn Transfer Attributes"]
    pub tcd13_attr: TCD_ATTR,
    _reserved_178_tcd13_nbytes: [u8; 4usize],
    #[doc = "0x11ac - TCDn Last Source Address Adjustment"]
    pub tcd13_slast: TCD_SLAST,
    #[doc = "0x11b0 - TCDn Destination Address"]
    pub tcd13_daddr: TCD_DADDR,
    #[doc = "0x11b4 - TCDn Signed Destination Address Offset"]
    pub tcd13_doff: TCD_DOFF,
    _reserved_182_tcd13_citer: [u8; 2usize],
    #[doc = "0x11b8 - TCDn Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd13_dlastsga: TCD_DLASTSGA,
    #[doc = "0x11bc - TCDn Control and Status"]
    pub tcd13_csr: TCD_CSR,
    _reserved_185_tcd13_biter: [u8; 2usize],
    #[doc = "0x11c0 - TCDn Source Address"]
    pub tcd14_saddr: TCD_SADDR,
    #[doc = "0x11c4 - TCDn Signed Source Address Offset"]
    pub tcd14_soff: TCD_SOFF,
    #[doc = "0x11c6 - TCDn Transfer Attributes"]
    pub tcd14_attr: TCD_ATTR,
    _reserved_189_tcd14_nbytes: [u8; 4usize],
    #[doc = "0x11cc - TCDn Last Source Address Adjustment"]
    pub tcd14_slast: TCD_SLAST,
    #[doc = "0x11d0 - TCDn Destination Address"]
    pub tcd14_daddr: TCD_DADDR,
    #[doc = "0x11d4 - TCDn Signed Destination Address Offset"]
    pub tcd14_doff: TCD_DOFF,
    _reserved_193_tcd14_citer: [u8; 2usize],
    #[doc = "0x11d8 - TCDn Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd14_dlastsga: TCD_DLASTSGA,
    #[doc = "0x11dc - TCDn Control and Status"]
    pub tcd14_csr: TCD_CSR,
    _reserved_196_tcd14_biter: [u8; 2usize],
    #[doc = "0x11e0 - TCDn Source Address"]
    pub tcd15_saddr: TCD_SADDR,
    #[doc = "0x11e4 - TCDn Signed Source Address Offset"]
    pub tcd15_soff: TCD_SOFF,
    #[doc = "0x11e6 - TCDn Transfer Attributes"]
    pub tcd15_attr: TCD_ATTR,
    _reserved_200_tcd15_nbytes: [u8; 4usize],
    #[doc = "0x11ec - TCDn Last Source Address Adjustment"]
    pub tcd15_slast: TCD_SLAST,
    #[doc = "0x11f0 - TCDn Destination Address"]
    pub tcd15_daddr: TCD_DADDR,
    #[doc = "0x11f4 - TCDn Signed Destination Address Offset"]
    pub tcd15_doff: TCD_DOFF,
    _reserved_204_tcd15_citer: [u8; 2usize],
    #[doc = "0x11f8 - TCDn Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd15_dlastsga: TCD_DLASTSGA,
    #[doc = "0x11fc - TCDn Control and Status"]
    pub tcd15_csr: TCD_CSR,
    _reserved_207_tcd15_biter: [u8; 2usize],
}
impl RegisterBlock {
    #[doc = "0x1008 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4104usize) as *const TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1008 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4104usize) as *mut TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1008 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4104usize) as *const TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1008 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4104usize) as *mut TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1008 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4104usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1008 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4104usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1016 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd0_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4118usize) as *const TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1016 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd0_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4118usize) as *mut TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1016 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd0_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4118usize) as *const TCD_CITER_ELINKNO) }
    }
    #[doc = "0x1016 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd0_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4118usize) as *mut TCD_CITER_ELINKNO) }
    }
    #[doc = "0x101e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd0_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4126usize) as *const TCD_BITER_ELINKYES) }
    }
    #[doc = "0x101e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd0_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4126usize) as *mut TCD_BITER_ELINKYES) }
    }
    #[doc = "0x101e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd0_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4126usize) as *const TCD_BITER_ELINKNO) }
    }
    #[doc = "0x101e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd0_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4126usize) as *mut TCD_BITER_ELINKNO) }
    }
    #[doc = "0x1028 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4136usize) as *const TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1028 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4136usize) as *mut TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1028 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4136usize) as *const TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1028 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4136usize) as *mut TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1028 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4136usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1028 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4136usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1036 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd1_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4150usize) as *const TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1036 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd1_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4150usize) as *mut TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1036 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd1_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4150usize) as *const TCD_CITER_ELINKNO) }
    }
    #[doc = "0x1036 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd1_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4150usize) as *mut TCD_CITER_ELINKNO) }
    }
    #[doc = "0x103e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd1_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4158usize) as *const TCD_BITER_ELINKYES) }
    }
    #[doc = "0x103e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd1_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4158usize) as *mut TCD_BITER_ELINKYES) }
    }
    #[doc = "0x103e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd1_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4158usize) as *const TCD_BITER_ELINKNO) }
    }
    #[doc = "0x103e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd1_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4158usize) as *mut TCD_BITER_ELINKNO) }
    }
    #[doc = "0x1048 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4168usize) as *const TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1048 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4168usize) as *mut TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1048 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4168usize) as *const TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1048 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4168usize) as *mut TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1048 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4168usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1048 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4168usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1056 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd2_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4182usize) as *const TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1056 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd2_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4182usize) as *mut TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1056 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd2_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4182usize) as *const TCD_CITER_ELINKNO) }
    }
    #[doc = "0x1056 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd2_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4182usize) as *mut TCD_CITER_ELINKNO) }
    }
    #[doc = "0x105e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd2_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4190usize) as *const TCD_BITER_ELINKYES) }
    }
    #[doc = "0x105e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd2_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4190usize) as *mut TCD_BITER_ELINKYES) }
    }
    #[doc = "0x105e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd2_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4190usize) as *const TCD_BITER_ELINKNO) }
    }
    #[doc = "0x105e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd2_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4190usize) as *mut TCD_BITER_ELINKNO) }
    }
    #[doc = "0x1068 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4200usize) as *const TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1068 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4200usize) as *mut TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1068 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4200usize) as *const TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1068 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4200usize) as *mut TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1068 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4200usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1068 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4200usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1076 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd3_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4214usize) as *const TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1076 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd3_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4214usize) as *mut TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1076 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd3_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4214usize) as *const TCD_CITER_ELINKNO) }
    }
    #[doc = "0x1076 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd3_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4214usize) as *mut TCD_CITER_ELINKNO) }
    }
    #[doc = "0x107e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd3_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4222usize) as *const TCD_BITER_ELINKYES) }
    }
    #[doc = "0x107e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd3_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4222usize) as *mut TCD_BITER_ELINKYES) }
    }
    #[doc = "0x107e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd3_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4222usize) as *const TCD_BITER_ELINKNO) }
    }
    #[doc = "0x107e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd3_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4222usize) as *mut TCD_BITER_ELINKNO) }
    }
    #[doc = "0x1088 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4232usize) as *const TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1088 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4232usize) as *mut TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1088 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4232usize) as *const TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1088 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4232usize) as *mut TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1088 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4232usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1088 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4232usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1096 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd4_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4246usize) as *const TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1096 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd4_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4246usize) as *mut TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1096 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd4_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4246usize) as *const TCD_CITER_ELINKNO) }
    }
    #[doc = "0x1096 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd4_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4246usize) as *mut TCD_CITER_ELINKNO) }
    }
    #[doc = "0x109e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd4_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4254usize) as *const TCD_BITER_ELINKYES) }
    }
    #[doc = "0x109e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd4_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4254usize) as *mut TCD_BITER_ELINKYES) }
    }
    #[doc = "0x109e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd4_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4254usize) as *const TCD_BITER_ELINKNO) }
    }
    #[doc = "0x109e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd4_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4254usize) as *mut TCD_BITER_ELINKNO) }
    }
    #[doc = "0x10a8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4264usize) as *const TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x10a8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4264usize) as *mut TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x10a8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4264usize) as *const TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x10a8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4264usize) as *mut TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x10a8 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4264usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x10a8 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4264usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x10b6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd5_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4278usize) as *const TCD_CITER_ELINKYES) }
    }
    #[doc = "0x10b6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd5_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4278usize) as *mut TCD_CITER_ELINKYES) }
    }
    #[doc = "0x10b6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd5_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4278usize) as *const TCD_CITER_ELINKNO) }
    }
    #[doc = "0x10b6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd5_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4278usize) as *mut TCD_CITER_ELINKNO) }
    }
    #[doc = "0x10be - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd5_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4286usize) as *const TCD_BITER_ELINKYES) }
    }
    #[doc = "0x10be - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd5_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4286usize) as *mut TCD_BITER_ELINKYES) }
    }
    #[doc = "0x10be - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd5_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4286usize) as *const TCD_BITER_ELINKNO) }
    }
    #[doc = "0x10be - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd5_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4286usize) as *mut TCD_BITER_ELINKNO) }
    }
    #[doc = "0x10c8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4296usize) as *const TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x10c8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4296usize) as *mut TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x10c8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4296usize) as *const TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x10c8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4296usize) as *mut TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x10c8 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4296usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x10c8 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4296usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x10d6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd6_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4310usize) as *const TCD_CITER_ELINKYES) }
    }
    #[doc = "0x10d6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd6_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4310usize) as *mut TCD_CITER_ELINKYES) }
    }
    #[doc = "0x10d6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd6_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4310usize) as *const TCD_CITER_ELINKNO) }
    }
    #[doc = "0x10d6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd6_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4310usize) as *mut TCD_CITER_ELINKNO) }
    }
    #[doc = "0x10de - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd6_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4318usize) as *const TCD_BITER_ELINKYES) }
    }
    #[doc = "0x10de - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd6_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4318usize) as *mut TCD_BITER_ELINKYES) }
    }
    #[doc = "0x10de - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd6_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4318usize) as *const TCD_BITER_ELINKNO) }
    }
    #[doc = "0x10de - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd6_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4318usize) as *mut TCD_BITER_ELINKNO) }
    }
    #[doc = "0x10e8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4328usize) as *const TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x10e8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4328usize) as *mut TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x10e8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4328usize) as *const TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x10e8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4328usize) as *mut TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x10e8 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4328usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x10e8 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4328usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x10f6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd7_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4342usize) as *const TCD_CITER_ELINKYES) }
    }
    #[doc = "0x10f6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd7_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4342usize) as *mut TCD_CITER_ELINKYES) }
    }
    #[doc = "0x10f6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd7_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4342usize) as *const TCD_CITER_ELINKNO) }
    }
    #[doc = "0x10f6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd7_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4342usize) as *mut TCD_CITER_ELINKNO) }
    }
    #[doc = "0x10fe - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd7_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4350usize) as *const TCD_BITER_ELINKYES) }
    }
    #[doc = "0x10fe - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd7_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4350usize) as *mut TCD_BITER_ELINKYES) }
    }
    #[doc = "0x10fe - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd7_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4350usize) as *const TCD_BITER_ELINKNO) }
    }
    #[doc = "0x10fe - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd7_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4350usize) as *mut TCD_BITER_ELINKNO) }
    }
    #[doc = "0x1108 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd8_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4360usize) as *const TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1108 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd8_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4360usize) as *mut TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1108 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd8_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4360usize) as *const TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1108 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd8_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4360usize) as *mut TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1108 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd8_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4360usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1108 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd8_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4360usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1116 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd8_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4374usize) as *const TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1116 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd8_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4374usize) as *mut TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1116 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd8_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4374usize) as *const TCD_CITER_ELINKNO) }
    }
    #[doc = "0x1116 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd8_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4374usize) as *mut TCD_CITER_ELINKNO) }
    }
    #[doc = "0x111e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd8_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4382usize) as *const TCD_BITER_ELINKYES) }
    }
    #[doc = "0x111e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd8_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4382usize) as *mut TCD_BITER_ELINKYES) }
    }
    #[doc = "0x111e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd8_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4382usize) as *const TCD_BITER_ELINKNO) }
    }
    #[doc = "0x111e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd8_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4382usize) as *mut TCD_BITER_ELINKNO) }
    }
    #[doc = "0x1128 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd9_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4392usize) as *const TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1128 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd9_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4392usize) as *mut TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1128 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd9_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4392usize) as *const TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1128 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd9_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4392usize) as *mut TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1128 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd9_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4392usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1128 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd9_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4392usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1136 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd9_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4406usize) as *const TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1136 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd9_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4406usize) as *mut TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1136 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd9_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4406usize) as *const TCD_CITER_ELINKNO) }
    }
    #[doc = "0x1136 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd9_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4406usize) as *mut TCD_CITER_ELINKNO) }
    }
    #[doc = "0x113e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd9_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4414usize) as *const TCD_BITER_ELINKYES) }
    }
    #[doc = "0x113e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd9_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4414usize) as *mut TCD_BITER_ELINKYES) }
    }
    #[doc = "0x113e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd9_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4414usize) as *const TCD_BITER_ELINKNO) }
    }
    #[doc = "0x113e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd9_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4414usize) as *mut TCD_BITER_ELINKNO) }
    }
    #[doc = "0x1148 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd10_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4424usize) as *const TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1148 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd10_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4424usize) as *mut TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1148 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd10_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4424usize) as *const TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1148 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd10_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4424usize) as *mut TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1148 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd10_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4424usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1148 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd10_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4424usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1156 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd10_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4438usize) as *const TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1156 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd10_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4438usize) as *mut TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1156 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd10_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4438usize) as *const TCD_CITER_ELINKNO) }
    }
    #[doc = "0x1156 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd10_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4438usize) as *mut TCD_CITER_ELINKNO) }
    }
    #[doc = "0x115e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd10_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4446usize) as *const TCD_BITER_ELINKYES) }
    }
    #[doc = "0x115e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd10_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4446usize) as *mut TCD_BITER_ELINKYES) }
    }
    #[doc = "0x115e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd10_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4446usize) as *const TCD_BITER_ELINKNO) }
    }
    #[doc = "0x115e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd10_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4446usize) as *mut TCD_BITER_ELINKNO) }
    }
    #[doc = "0x1168 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd11_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4456usize) as *const TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1168 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd11_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4456usize) as *mut TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1168 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd11_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4456usize) as *const TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1168 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd11_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4456usize) as *mut TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1168 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd11_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4456usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1168 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd11_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4456usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1176 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd11_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4470usize) as *const TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1176 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd11_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4470usize) as *mut TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1176 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd11_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4470usize) as *const TCD_CITER_ELINKNO) }
    }
    #[doc = "0x1176 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd11_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4470usize) as *mut TCD_CITER_ELINKNO) }
    }
    #[doc = "0x117e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd11_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4478usize) as *const TCD_BITER_ELINKYES) }
    }
    #[doc = "0x117e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd11_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4478usize) as *mut TCD_BITER_ELINKYES) }
    }
    #[doc = "0x117e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd11_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4478usize) as *const TCD_BITER_ELINKNO) }
    }
    #[doc = "0x117e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd11_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4478usize) as *mut TCD_BITER_ELINKNO) }
    }
    #[doc = "0x1188 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd12_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4488usize) as *const TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1188 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd12_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4488usize) as *mut TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1188 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd12_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4488usize) as *const TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1188 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd12_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4488usize) as *mut TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1188 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd12_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4488usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1188 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd12_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4488usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x1196 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd12_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4502usize) as *const TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1196 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd12_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4502usize) as *mut TCD_CITER_ELINKYES) }
    }
    #[doc = "0x1196 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd12_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4502usize) as *const TCD_CITER_ELINKNO) }
    }
    #[doc = "0x1196 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd12_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4502usize) as *mut TCD_CITER_ELINKNO) }
    }
    #[doc = "0x119e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd12_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4510usize) as *const TCD_BITER_ELINKYES) }
    }
    #[doc = "0x119e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd12_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4510usize) as *mut TCD_BITER_ELINKYES) }
    }
    #[doc = "0x119e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd12_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4510usize) as *const TCD_BITER_ELINKNO) }
    }
    #[doc = "0x119e - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd12_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4510usize) as *mut TCD_BITER_ELINKNO) }
    }
    #[doc = "0x11a8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd13_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4520usize) as *const TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x11a8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd13_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4520usize) as *mut TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x11a8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd13_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4520usize) as *const TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x11a8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd13_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4520usize) as *mut TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x11a8 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd13_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4520usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x11a8 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd13_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4520usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x11b6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd13_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4534usize) as *const TCD_CITER_ELINKYES) }
    }
    #[doc = "0x11b6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd13_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4534usize) as *mut TCD_CITER_ELINKYES) }
    }
    #[doc = "0x11b6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd13_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4534usize) as *const TCD_CITER_ELINKNO) }
    }
    #[doc = "0x11b6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd13_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4534usize) as *mut TCD_CITER_ELINKNO) }
    }
    #[doc = "0x11be - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd13_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4542usize) as *const TCD_BITER_ELINKYES) }
    }
    #[doc = "0x11be - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd13_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4542usize) as *mut TCD_BITER_ELINKYES) }
    }
    #[doc = "0x11be - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd13_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4542usize) as *const TCD_BITER_ELINKNO) }
    }
    #[doc = "0x11be - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd13_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4542usize) as *mut TCD_BITER_ELINKNO) }
    }
    #[doc = "0x11c8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd14_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4552usize) as *const TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x11c8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd14_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4552usize) as *mut TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x11c8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd14_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4552usize) as *const TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x11c8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd14_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4552usize) as *mut TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x11c8 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd14_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4552usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x11c8 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd14_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4552usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x11d6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd14_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4566usize) as *const TCD_CITER_ELINKYES) }
    }
    #[doc = "0x11d6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd14_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4566usize) as *mut TCD_CITER_ELINKYES) }
    }
    #[doc = "0x11d6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd14_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4566usize) as *const TCD_CITER_ELINKNO) }
    }
    #[doc = "0x11d6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd14_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4566usize) as *mut TCD_CITER_ELINKNO) }
    }
    #[doc = "0x11de - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd14_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4574usize) as *const TCD_BITER_ELINKYES) }
    }
    #[doc = "0x11de - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd14_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4574usize) as *mut TCD_BITER_ELINKYES) }
    }
    #[doc = "0x11de - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd14_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4574usize) as *const TCD_BITER_ELINKNO) }
    }
    #[doc = "0x11de - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd14_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4574usize) as *mut TCD_BITER_ELINKNO) }
    }
    #[doc = "0x11e8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd15_nbytes_mloffyes(&self) -> &TCD_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4584usize) as *const TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x11e8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd15_nbytes_mloffyes_mut(&self) -> &mut TCD_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4584usize) as *mut TCD_NBYTES_MLOFFYES) }
    }
    #[doc = "0x11e8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd15_nbytes_mloffno(&self) -> &TCD_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4584usize) as *const TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x11e8 - TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd15_nbytes_mloffno_mut(&self) -> &mut TCD_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4584usize) as *mut TCD_NBYTES_MLOFFNO) }
    }
    #[doc = "0x11e8 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd15_nbytes_mlno(&self) -> &TCD_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4584usize) as *const TCD_NBYTES_MLNO) }
    }
    #[doc = "0x11e8 - TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd15_nbytes_mlno_mut(&self) -> &mut TCD_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4584usize) as *mut TCD_NBYTES_MLNO) }
    }
    #[doc = "0x11f6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd15_citer_elinkyes(&self) -> &TCD_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4598usize) as *const TCD_CITER_ELINKYES) }
    }
    #[doc = "0x11f6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd15_citer_elinkyes_mut(&self) -> &mut TCD_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4598usize) as *mut TCD_CITER_ELINKYES) }
    }
    #[doc = "0x11f6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd15_citer_elinkno(&self) -> &TCD_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4598usize) as *const TCD_CITER_ELINKNO) }
    }
    #[doc = "0x11f6 - TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd15_citer_elinkno_mut(&self) -> &mut TCD_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4598usize) as *mut TCD_CITER_ELINKNO) }
    }
    #[doc = "0x11fe - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd15_biter_elinkyes(&self) -> &TCD_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4606usize) as *const TCD_BITER_ELINKYES) }
    }
    #[doc = "0x11fe - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd15_biter_elinkyes_mut(&self) -> &mut TCD_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4606usize) as *mut TCD_BITER_ELINKYES) }
    }
    #[doc = "0x11fe - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd15_biter_elinkno(&self) -> &TCD_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4606usize) as *const TCD_BITER_ELINKNO) }
    }
    #[doc = "0x11fe - TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd15_biter_elinkno_mut(&self) -> &mut TCD_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4606usize) as *mut TCD_BITER_ELINKNO) }
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [es](es) module"]
pub type ES = crate::Reg<u32, _ES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ES;
#[doc = "`read()` method returns [es::R](es::R) reader structure"]
impl crate::Readable for ES {}
#[doc = "Error Status Register"]
pub mod es;
#[doc = "Enable Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erq](erq) module"]
pub type ERQ = crate::Reg<u32, _ERQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERQ;
#[doc = "`read()` method returns [erq::R](erq::R) reader structure"]
impl crate::Readable for ERQ {}
#[doc = "`write(|w| ..)` method takes [erq::W](erq::W) writer structure"]
impl crate::Writable for ERQ {}
#[doc = "Enable Request Register"]
pub mod erq;
#[doc = "Enable Error Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eei](eei) module"]
pub type EEI = crate::Reg<u32, _EEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEI;
#[doc = "`read()` method returns [eei::R](eei::R) reader structure"]
impl crate::Readable for EEI {}
#[doc = "`write(|w| ..)` method takes [eei::W](eei::W) writer structure"]
impl crate::Writable for EEI {}
#[doc = "Enable Error Interrupt Register"]
pub mod eei;
#[doc = "Clear Enable Error Interrupt Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ceei](ceei) module"]
pub type CEEI = crate::Reg<u8, _CEEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEEI;
#[doc = "`write(|w| ..)` method takes [ceei::W](ceei::W) writer structure"]
impl crate::Writable for CEEI {}
#[doc = "Clear Enable Error Interrupt Register"]
pub mod ceei;
#[doc = "Set Enable Error Interrupt Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seei](seei) module"]
pub type SEEI = crate::Reg<u8, _SEEI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEEI;
#[doc = "`write(|w| ..)` method takes [seei::W](seei::W) writer structure"]
impl crate::Writable for SEEI {}
#[doc = "Set Enable Error Interrupt Register"]
pub mod seei;
#[doc = "Clear Enable Request Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cerq](cerq) module"]
pub type CERQ = crate::Reg<u8, _CERQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CERQ;
#[doc = "`write(|w| ..)` method takes [cerq::W](cerq::W) writer structure"]
impl crate::Writable for CERQ {}
#[doc = "Clear Enable Request Register"]
pub mod cerq;
#[doc = "Set Enable Request Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [serq](serq) module"]
pub type SERQ = crate::Reg<u8, _SERQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SERQ;
#[doc = "`write(|w| ..)` method takes [serq::W](serq::W) writer structure"]
impl crate::Writable for SERQ {}
#[doc = "Set Enable Request Register"]
pub mod serq;
#[doc = "Clear DONE Status Bit Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdne](cdne) module"]
pub type CDNE = crate::Reg<u8, _CDNE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDNE;
#[doc = "`write(|w| ..)` method takes [cdne::W](cdne::W) writer structure"]
impl crate::Writable for CDNE {}
#[doc = "Clear DONE Status Bit Register"]
pub mod cdne;
#[doc = "Set START Bit Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssrt](ssrt) module"]
pub type SSRT = crate::Reg<u8, _SSRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSRT;
#[doc = "`write(|w| ..)` method takes [ssrt::W](ssrt::W) writer structure"]
impl crate::Writable for SSRT {}
#[doc = "Set START Bit Register"]
pub mod ssrt;
#[doc = "Clear Error Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cerr](cerr) module"]
pub type CERR = crate::Reg<u8, _CERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CERR;
#[doc = "`write(|w| ..)` method takes [cerr::W](cerr::W) writer structure"]
impl crate::Writable for CERR {}
#[doc = "Clear Error Register"]
pub mod cerr;
#[doc = "Clear Interrupt Request Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cint](cint) module"]
pub type CINT = crate::Reg<u8, _CINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CINT;
#[doc = "`write(|w| ..)` method takes [cint::W](cint::W) writer structure"]
impl crate::Writable for CINT {}
#[doc = "Clear Interrupt Request Register"]
pub mod cint;
#[doc = "Interrupt Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int](int) module"]
pub type INT = crate::Reg<u32, _INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT;
#[doc = "`read()` method returns [int::R](int::R) reader structure"]
impl crate::Readable for INT {}
#[doc = "`write(|w| ..)` method takes [int::W](int::W) writer structure"]
impl crate::Writable for INT {}
#[doc = "Interrupt Request Register"]
pub mod int;
#[doc = "Error Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [err](err) module"]
pub type ERR = crate::Reg<u32, _ERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERR;
#[doc = "`read()` method returns [err::R](err::R) reader structure"]
impl crate::Readable for ERR {}
#[doc = "`write(|w| ..)` method takes [err::W](err::W) writer structure"]
impl crate::Writable for ERR {}
#[doc = "Error Register"]
pub mod err;
#[doc = "Hardware Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrs](hrs) module"]
pub type HRS = crate::Reg<u32, _HRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HRS;
#[doc = "`read()` method returns [hrs::R](hrs::R) reader structure"]
impl crate::Readable for HRS {}
#[doc = "Hardware Request Status Register"]
pub mod hrs;
#[doc = "Enable Asynchronous Request in Stop Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ears](ears) module"]
pub type EARS = crate::Reg<u32, _EARS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EARS;
#[doc = "`read()` method returns [ears::R](ears::R) reader structure"]
impl crate::Readable for EARS {}
#[doc = "`write(|w| ..)` method takes [ears::W](ears::W) writer structure"]
impl crate::Writable for EARS {}
#[doc = "Enable Asynchronous Request in Stop Register"]
pub mod ears;
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri3](dchpri3) module"]
pub type DCHPRI3 = crate::Reg<u8, _DCHPRI3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCHPRI3;
#[doc = "`read()` method returns [dchpri3::R](dchpri3::R) reader structure"]
impl crate::Readable for DCHPRI3 {}
#[doc = "`write(|w| ..)` method takes [dchpri3::W](dchpri3::W) writer structure"]
impl crate::Writable for DCHPRI3 {}
#[doc = "Channel n Priority Register"]
pub mod dchpri3;
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri2](dchpri2) module"]
pub type DCHPRI2 = crate::Reg<u8, _DCHPRI2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCHPRI2;
#[doc = "`read()` method returns [dchpri2::R](dchpri2::R) reader structure"]
impl crate::Readable for DCHPRI2 {}
#[doc = "`write(|w| ..)` method takes [dchpri2::W](dchpri2::W) writer structure"]
impl crate::Writable for DCHPRI2 {}
#[doc = "Channel n Priority Register"]
pub mod dchpri2;
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri1](dchpri1) module"]
pub type DCHPRI1 = crate::Reg<u8, _DCHPRI1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCHPRI1;
#[doc = "`read()` method returns [dchpri1::R](dchpri1::R) reader structure"]
impl crate::Readable for DCHPRI1 {}
#[doc = "`write(|w| ..)` method takes [dchpri1::W](dchpri1::W) writer structure"]
impl crate::Writable for DCHPRI1 {}
#[doc = "Channel n Priority Register"]
pub mod dchpri1;
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri0](dchpri0) module"]
pub type DCHPRI0 = crate::Reg<u8, _DCHPRI0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCHPRI0;
#[doc = "`read()` method returns [dchpri0::R](dchpri0::R) reader structure"]
impl crate::Readable for DCHPRI0 {}
#[doc = "`write(|w| ..)` method takes [dchpri0::W](dchpri0::W) writer structure"]
impl crate::Writable for DCHPRI0 {}
#[doc = "Channel n Priority Register"]
pub mod dchpri0;
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri7](dchpri7) module"]
pub type DCHPRI7 = crate::Reg<u8, _DCHPRI7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCHPRI7;
#[doc = "`read()` method returns [dchpri7::R](dchpri7::R) reader structure"]
impl crate::Readable for DCHPRI7 {}
#[doc = "`write(|w| ..)` method takes [dchpri7::W](dchpri7::W) writer structure"]
impl crate::Writable for DCHPRI7 {}
#[doc = "Channel n Priority Register"]
pub mod dchpri7;
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri6](dchpri6) module"]
pub type DCHPRI6 = crate::Reg<u8, _DCHPRI6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCHPRI6;
#[doc = "`read()` method returns [dchpri6::R](dchpri6::R) reader structure"]
impl crate::Readable for DCHPRI6 {}
#[doc = "`write(|w| ..)` method takes [dchpri6::W](dchpri6::W) writer structure"]
impl crate::Writable for DCHPRI6 {}
#[doc = "Channel n Priority Register"]
pub mod dchpri6;
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri5](dchpri5) module"]
pub type DCHPRI5 = crate::Reg<u8, _DCHPRI5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCHPRI5;
#[doc = "`read()` method returns [dchpri5::R](dchpri5::R) reader structure"]
impl crate::Readable for DCHPRI5 {}
#[doc = "`write(|w| ..)` method takes [dchpri5::W](dchpri5::W) writer structure"]
impl crate::Writable for DCHPRI5 {}
#[doc = "Channel n Priority Register"]
pub mod dchpri5;
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri4](dchpri4) module"]
pub type DCHPRI4 = crate::Reg<u8, _DCHPRI4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCHPRI4;
#[doc = "`read()` method returns [dchpri4::R](dchpri4::R) reader structure"]
impl crate::Readable for DCHPRI4 {}
#[doc = "`write(|w| ..)` method takes [dchpri4::W](dchpri4::W) writer structure"]
impl crate::Writable for DCHPRI4 {}
#[doc = "Channel n Priority Register"]
pub mod dchpri4;
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri11](dchpri11) module"]
pub type DCHPRI11 = crate::Reg<u8, _DCHPRI11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCHPRI11;
#[doc = "`read()` method returns [dchpri11::R](dchpri11::R) reader structure"]
impl crate::Readable for DCHPRI11 {}
#[doc = "`write(|w| ..)` method takes [dchpri11::W](dchpri11::W) writer structure"]
impl crate::Writable for DCHPRI11 {}
#[doc = "Channel n Priority Register"]
pub mod dchpri11;
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri10](dchpri10) module"]
pub type DCHPRI10 = crate::Reg<u8, _DCHPRI10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCHPRI10;
#[doc = "`read()` method returns [dchpri10::R](dchpri10::R) reader structure"]
impl crate::Readable for DCHPRI10 {}
#[doc = "`write(|w| ..)` method takes [dchpri10::W](dchpri10::W) writer structure"]
impl crate::Writable for DCHPRI10 {}
#[doc = "Channel n Priority Register"]
pub mod dchpri10;
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri9](dchpri9) module"]
pub type DCHPRI9 = crate::Reg<u8, _DCHPRI9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCHPRI9;
#[doc = "`read()` method returns [dchpri9::R](dchpri9::R) reader structure"]
impl crate::Readable for DCHPRI9 {}
#[doc = "`write(|w| ..)` method takes [dchpri9::W](dchpri9::W) writer structure"]
impl crate::Writable for DCHPRI9 {}
#[doc = "Channel n Priority Register"]
pub mod dchpri9;
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri8](dchpri8) module"]
pub type DCHPRI8 = crate::Reg<u8, _DCHPRI8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCHPRI8;
#[doc = "`read()` method returns [dchpri8::R](dchpri8::R) reader structure"]
impl crate::Readable for DCHPRI8 {}
#[doc = "`write(|w| ..)` method takes [dchpri8::W](dchpri8::W) writer structure"]
impl crate::Writable for DCHPRI8 {}
#[doc = "Channel n Priority Register"]
pub mod dchpri8;
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri15](dchpri15) module"]
pub type DCHPRI15 = crate::Reg<u8, _DCHPRI15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCHPRI15;
#[doc = "`read()` method returns [dchpri15::R](dchpri15::R) reader structure"]
impl crate::Readable for DCHPRI15 {}
#[doc = "`write(|w| ..)` method takes [dchpri15::W](dchpri15::W) writer structure"]
impl crate::Writable for DCHPRI15 {}
#[doc = "Channel n Priority Register"]
pub mod dchpri15;
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri14](dchpri14) module"]
pub type DCHPRI14 = crate::Reg<u8, _DCHPRI14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCHPRI14;
#[doc = "`read()` method returns [dchpri14::R](dchpri14::R) reader structure"]
impl crate::Readable for DCHPRI14 {}
#[doc = "`write(|w| ..)` method takes [dchpri14::W](dchpri14::W) writer structure"]
impl crate::Writable for DCHPRI14 {}
#[doc = "Channel n Priority Register"]
pub mod dchpri14;
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri13](dchpri13) module"]
pub type DCHPRI13 = crate::Reg<u8, _DCHPRI13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCHPRI13;
#[doc = "`read()` method returns [dchpri13::R](dchpri13::R) reader structure"]
impl crate::Readable for DCHPRI13 {}
#[doc = "`write(|w| ..)` method takes [dchpri13::W](dchpri13::W) writer structure"]
impl crate::Writable for DCHPRI13 {}
#[doc = "Channel n Priority Register"]
pub mod dchpri13;
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri12](dchpri12) module"]
pub type DCHPRI12 = crate::Reg<u8, _DCHPRI12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCHPRI12;
#[doc = "`read()` method returns [dchpri12::R](dchpri12::R) reader structure"]
impl crate::Readable for DCHPRI12 {}
#[doc = "`write(|w| ..)` method takes [dchpri12::W](dchpri12::W) writer structure"]
impl crate::Writable for DCHPRI12 {}
#[doc = "Channel n Priority Register"]
pub mod dchpri12;
#[doc = "TCDn Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_saddr](tcd_saddr) module"]
pub type TCD_SADDR = crate::Reg<u32, _TCD_SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_SADDR;
#[doc = "`read()` method returns [tcd_saddr::R](tcd_saddr::R) reader structure"]
impl crate::Readable for TCD_SADDR {}
#[doc = "`write(|w| ..)` method takes [tcd_saddr::W](tcd_saddr::W) writer structure"]
impl crate::Writable for TCD_SADDR {}
#[doc = "TCDn Source Address"]
pub mod tcd_saddr;
#[doc = "TCDn Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_soff](tcd_soff) module"]
pub type TCD_SOFF = crate::Reg<u16, _TCD_SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_SOFF;
#[doc = "`read()` method returns [tcd_soff::R](tcd_soff::R) reader structure"]
impl crate::Readable for TCD_SOFF {}
#[doc = "`write(|w| ..)` method takes [tcd_soff::W](tcd_soff::W) writer structure"]
impl crate::Writable for TCD_SOFF {}
#[doc = "TCDn Signed Source Address Offset"]
pub mod tcd_soff;
#[doc = "TCDn Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_attr](tcd_attr) module"]
pub type TCD_ATTR = crate::Reg<u16, _TCD_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_ATTR;
#[doc = "`read()` method returns [tcd_attr::R](tcd_attr::R) reader structure"]
impl crate::Readable for TCD_ATTR {}
#[doc = "`write(|w| ..)` method takes [tcd_attr::W](tcd_attr::W) writer structure"]
impl crate::Writable for TCD_ATTR {}
#[doc = "TCDn Transfer Attributes"]
pub mod tcd_attr;
#[doc = "TCDn Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_nbytes_mlno](tcd_nbytes_mlno) module"]
pub type TCD_NBYTES_MLNO = crate::Reg<u32, _TCD_NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_NBYTES_MLNO;
#[doc = "`read()` method returns [tcd_nbytes_mlno::R](tcd_nbytes_mlno::R) reader structure"]
impl crate::Readable for TCD_NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [tcd_nbytes_mlno::W](tcd_nbytes_mlno::W) writer structure"]
impl crate::Writable for TCD_NBYTES_MLNO {}
#[doc = "TCDn Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd_nbytes_mlno;
#[doc = "TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_nbytes_mloffno](tcd_nbytes_mloffno) module"]
pub type TCD_NBYTES_MLOFFNO = crate::Reg<u32, _TCD_NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_NBYTES_MLOFFNO;
#[doc = "`read()` method returns [tcd_nbytes_mloffno::R](tcd_nbytes_mloffno::R) reader structure"]
impl crate::Readable for TCD_NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [tcd_nbytes_mloffno::W](tcd_nbytes_mloffno::W) writer structure"]
impl crate::Writable for TCD_NBYTES_MLOFFNO {}
#[doc = "TCDn Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd_nbytes_mloffno;
#[doc = "TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_nbytes_mloffyes](tcd_nbytes_mloffyes) module"]
pub type TCD_NBYTES_MLOFFYES = crate::Reg<u32, _TCD_NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_NBYTES_MLOFFYES;
#[doc = "`read()` method returns [tcd_nbytes_mloffyes::R](tcd_nbytes_mloffyes::R) reader structure"]
impl crate::Readable for TCD_NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [tcd_nbytes_mloffyes::W](tcd_nbytes_mloffyes::W) writer structure"]
impl crate::Writable for TCD_NBYTES_MLOFFYES {}
#[doc = "TCDn Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd_nbytes_mloffyes;
#[doc = "TCDn Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_slast](tcd_slast) module"]
pub type TCD_SLAST = crate::Reg<u32, _TCD_SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_SLAST;
#[doc = "`read()` method returns [tcd_slast::R](tcd_slast::R) reader structure"]
impl crate::Readable for TCD_SLAST {}
#[doc = "`write(|w| ..)` method takes [tcd_slast::W](tcd_slast::W) writer structure"]
impl crate::Writable for TCD_SLAST {}
#[doc = "TCDn Last Source Address Adjustment"]
pub mod tcd_slast;
#[doc = "TCDn Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_daddr](tcd_daddr) module"]
pub type TCD_DADDR = crate::Reg<u32, _TCD_DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_DADDR;
#[doc = "`read()` method returns [tcd_daddr::R](tcd_daddr::R) reader structure"]
impl crate::Readable for TCD_DADDR {}
#[doc = "`write(|w| ..)` method takes [tcd_daddr::W](tcd_daddr::W) writer structure"]
impl crate::Writable for TCD_DADDR {}
#[doc = "TCDn Destination Address"]
pub mod tcd_daddr;
#[doc = "TCDn Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_doff](tcd_doff) module"]
pub type TCD_DOFF = crate::Reg<u16, _TCD_DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_DOFF;
#[doc = "`read()` method returns [tcd_doff::R](tcd_doff::R) reader structure"]
impl crate::Readable for TCD_DOFF {}
#[doc = "`write(|w| ..)` method takes [tcd_doff::W](tcd_doff::W) writer structure"]
impl crate::Writable for TCD_DOFF {}
#[doc = "TCDn Signed Destination Address Offset"]
pub mod tcd_doff;
#[doc = "TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_citer_elinkno](tcd_citer_elinkno) module"]
pub type TCD_CITER_ELINKNO = crate::Reg<u16, _TCD_CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_CITER_ELINKNO;
#[doc = "`read()` method returns [tcd_citer_elinkno::R](tcd_citer_elinkno::R) reader structure"]
impl crate::Readable for TCD_CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd_citer_elinkno::W](tcd_citer_elinkno::W) writer structure"]
impl crate::Writable for TCD_CITER_ELINKNO {}
#[doc = "TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_citer_elinkno;
#[doc = "TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_citer_elinkyes](tcd_citer_elinkyes) module"]
pub type TCD_CITER_ELINKYES = crate::Reg<u16, _TCD_CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_CITER_ELINKYES;
#[doc = "`read()` method returns [tcd_citer_elinkyes::R](tcd_citer_elinkyes::R) reader structure"]
impl crate::Readable for TCD_CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd_citer_elinkyes::W](tcd_citer_elinkyes::W) writer structure"]
impl crate::Writable for TCD_CITER_ELINKYES {}
#[doc = "TCDn Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_citer_elinkyes;
#[doc = "TCDn Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_dlastsga](tcd_dlastsga) module"]
pub type TCD_DLASTSGA = crate::Reg<u32, _TCD_DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_DLASTSGA;
#[doc = "`read()` method returns [tcd_dlastsga::R](tcd_dlastsga::R) reader structure"]
impl crate::Readable for TCD_DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [tcd_dlastsga::W](tcd_dlastsga::W) writer structure"]
impl crate::Writable for TCD_DLASTSGA {}
#[doc = "TCDn Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd_dlastsga;
#[doc = "TCDn Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_csr](tcd_csr) module"]
pub type TCD_CSR = crate::Reg<u16, _TCD_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_CSR;
#[doc = "`read()` method returns [tcd_csr::R](tcd_csr::R) reader structure"]
impl crate::Readable for TCD_CSR {}
#[doc = "`write(|w| ..)` method takes [tcd_csr::W](tcd_csr::W) writer structure"]
impl crate::Writable for TCD_CSR {}
#[doc = "TCDn Control and Status"]
pub mod tcd_csr;
#[doc = "TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_biter_elinkno](tcd_biter_elinkno) module"]
pub type TCD_BITER_ELINKNO = crate::Reg<u16, _TCD_BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_BITER_ELINKNO;
#[doc = "`read()` method returns [tcd_biter_elinkno::R](tcd_biter_elinkno::R) reader structure"]
impl crate::Readable for TCD_BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd_biter_elinkno::W](tcd_biter_elinkno::W) writer structure"]
impl crate::Writable for TCD_BITER_ELINKNO {}
#[doc = "TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd_biter_elinkno;
#[doc = "TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_biter_elinkyes](tcd_biter_elinkyes) module"]
pub type TCD_BITER_ELINKYES = crate::Reg<u16, _TCD_BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD_BITER_ELINKYES;
#[doc = "`read()` method returns [tcd_biter_elinkyes::R](tcd_biter_elinkyes::R) reader structure"]
impl crate::Readable for TCD_BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd_biter_elinkyes::W](tcd_biter_elinkyes::W) writer structure"]
impl crate::Writable for TCD_BITER_ELINKYES {}
#[doc = "TCDn Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd_biter_elinkyes;
