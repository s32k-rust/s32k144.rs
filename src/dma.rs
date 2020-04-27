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
    #[doc = "0x1000 - TCD Source Address"]
    pub tcd0_saddr: TCD0_SADDR,
    #[doc = "0x1004 - TCD Signed Source Address Offset"]
    pub tcd0_soff: TCD0_SOFF,
    #[doc = "0x1006 - TCD Transfer Attributes"]
    pub tcd0_attr: TCD0_ATTR,
    _reserved_35_tcd0_nbytes: [u8; 4usize],
    #[doc = "0x100c - TCD Last Source Address Adjustment"]
    pub tcd0_slast: TCD0_SLAST,
    #[doc = "0x1010 - TCD Destination Address"]
    pub tcd0_daddr: TCD0_DADDR,
    #[doc = "0x1014 - TCD Signed Destination Address Offset"]
    pub tcd0_doff: TCD0_DOFF,
    _reserved_39_tcd0_citer: [u8; 2usize],
    #[doc = "0x1018 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd0_dlastsga: TCD0_DLASTSGA,
    #[doc = "0x101c - TCD Control and Status"]
    pub tcd0_csr: TCD0_CSR,
    _reserved_42_tcd0_biter: [u8; 2usize],
    #[doc = "0x1020 - TCD Source Address"]
    pub tcd1_saddr: TCD1_SADDR,
    #[doc = "0x1024 - TCD Signed Source Address Offset"]
    pub tcd1_soff: TCD1_SOFF,
    #[doc = "0x1026 - TCD Transfer Attributes"]
    pub tcd1_attr: TCD1_ATTR,
    _reserved_46_tcd1_nbytes: [u8; 4usize],
    #[doc = "0x102c - TCD Last Source Address Adjustment"]
    pub tcd1_slast: TCD1_SLAST,
    #[doc = "0x1030 - TCD Destination Address"]
    pub tcd1_daddr: TCD1_DADDR,
    #[doc = "0x1034 - TCD Signed Destination Address Offset"]
    pub tcd1_doff: TCD1_DOFF,
    _reserved_50_tcd1_citer: [u8; 2usize],
    #[doc = "0x1038 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd1_dlastsga: TCD1_DLASTSGA,
    #[doc = "0x103c - TCD Control and Status"]
    pub tcd1_csr: TCD1_CSR,
    _reserved_53_tcd1_biter: [u8; 2usize],
    #[doc = "0x1040 - TCD Source Address"]
    pub tcd2_saddr: TCD2_SADDR,
    #[doc = "0x1044 - TCD Signed Source Address Offset"]
    pub tcd2_soff: TCD2_SOFF,
    #[doc = "0x1046 - TCD Transfer Attributes"]
    pub tcd2_attr: TCD2_ATTR,
    _reserved_57_tcd2_nbytes: [u8; 4usize],
    #[doc = "0x104c - TCD Last Source Address Adjustment"]
    pub tcd2_slast: TCD2_SLAST,
    #[doc = "0x1050 - TCD Destination Address"]
    pub tcd2_daddr: TCD2_DADDR,
    #[doc = "0x1054 - TCD Signed Destination Address Offset"]
    pub tcd2_doff: TCD2_DOFF,
    _reserved_61_tcd2_citer: [u8; 2usize],
    #[doc = "0x1058 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd2_dlastsga: TCD2_DLASTSGA,
    #[doc = "0x105c - TCD Control and Status"]
    pub tcd2_csr: TCD2_CSR,
    _reserved_64_tcd2_biter: [u8; 2usize],
    #[doc = "0x1060 - TCD Source Address"]
    pub tcd3_saddr: TCD3_SADDR,
    #[doc = "0x1064 - TCD Signed Source Address Offset"]
    pub tcd3_soff: TCD3_SOFF,
    #[doc = "0x1066 - TCD Transfer Attributes"]
    pub tcd3_attr: TCD3_ATTR,
    _reserved_68_tcd3_nbytes: [u8; 4usize],
    #[doc = "0x106c - TCD Last Source Address Adjustment"]
    pub tcd3_slast: TCD3_SLAST,
    #[doc = "0x1070 - TCD Destination Address"]
    pub tcd3_daddr: TCD3_DADDR,
    #[doc = "0x1074 - TCD Signed Destination Address Offset"]
    pub tcd3_doff: TCD3_DOFF,
    _reserved_72_tcd3_citer: [u8; 2usize],
    #[doc = "0x1078 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd3_dlastsga: TCD3_DLASTSGA,
    #[doc = "0x107c - TCD Control and Status"]
    pub tcd3_csr: TCD3_CSR,
    _reserved_75_tcd3_biter: [u8; 2usize],
    #[doc = "0x1080 - TCD Source Address"]
    pub tcd4_saddr: TCD4_SADDR,
    #[doc = "0x1084 - TCD Signed Source Address Offset"]
    pub tcd4_soff: TCD4_SOFF,
    #[doc = "0x1086 - TCD Transfer Attributes"]
    pub tcd4_attr: TCD4_ATTR,
    _reserved_79_tcd4_nbytes: [u8; 4usize],
    #[doc = "0x108c - TCD Last Source Address Adjustment"]
    pub tcd4_slast: TCD4_SLAST,
    #[doc = "0x1090 - TCD Destination Address"]
    pub tcd4_daddr: TCD4_DADDR,
    #[doc = "0x1094 - TCD Signed Destination Address Offset"]
    pub tcd4_doff: TCD4_DOFF,
    _reserved_83_tcd4_citer: [u8; 2usize],
    #[doc = "0x1098 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd4_dlastsga: TCD4_DLASTSGA,
    #[doc = "0x109c - TCD Control and Status"]
    pub tcd4_csr: TCD4_CSR,
    _reserved_86_tcd4_biter: [u8; 2usize],
    #[doc = "0x10a0 - TCD Source Address"]
    pub tcd5_saddr: TCD5_SADDR,
    #[doc = "0x10a4 - TCD Signed Source Address Offset"]
    pub tcd5_soff: TCD5_SOFF,
    #[doc = "0x10a6 - TCD Transfer Attributes"]
    pub tcd5_attr: TCD5_ATTR,
    _reserved_90_tcd5_nbytes: [u8; 4usize],
    #[doc = "0x10ac - TCD Last Source Address Adjustment"]
    pub tcd5_slast: TCD5_SLAST,
    #[doc = "0x10b0 - TCD Destination Address"]
    pub tcd5_daddr: TCD5_DADDR,
    #[doc = "0x10b4 - TCD Signed Destination Address Offset"]
    pub tcd5_doff: TCD5_DOFF,
    _reserved_94_tcd5_citer: [u8; 2usize],
    #[doc = "0x10b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd5_dlastsga: TCD5_DLASTSGA,
    #[doc = "0x10bc - TCD Control and Status"]
    pub tcd5_csr: TCD5_CSR,
    _reserved_97_tcd5_biter: [u8; 2usize],
    #[doc = "0x10c0 - TCD Source Address"]
    pub tcd6_saddr: TCD6_SADDR,
    #[doc = "0x10c4 - TCD Signed Source Address Offset"]
    pub tcd6_soff: TCD6_SOFF,
    #[doc = "0x10c6 - TCD Transfer Attributes"]
    pub tcd6_attr: TCD6_ATTR,
    _reserved_101_tcd6_nbytes: [u8; 4usize],
    #[doc = "0x10cc - TCD Last Source Address Adjustment"]
    pub tcd6_slast: TCD6_SLAST,
    #[doc = "0x10d0 - TCD Destination Address"]
    pub tcd6_daddr: TCD6_DADDR,
    #[doc = "0x10d4 - TCD Signed Destination Address Offset"]
    pub tcd6_doff: TCD6_DOFF,
    _reserved_105_tcd6_citer: [u8; 2usize],
    #[doc = "0x10d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd6_dlastsga: TCD6_DLASTSGA,
    #[doc = "0x10dc - TCD Control and Status"]
    pub tcd6_csr: TCD6_CSR,
    _reserved_108_tcd6_biter: [u8; 2usize],
    #[doc = "0x10e0 - TCD Source Address"]
    pub tcd7_saddr: TCD7_SADDR,
    #[doc = "0x10e4 - TCD Signed Source Address Offset"]
    pub tcd7_soff: TCD7_SOFF,
    #[doc = "0x10e6 - TCD Transfer Attributes"]
    pub tcd7_attr: TCD7_ATTR,
    _reserved_112_tcd7_nbytes: [u8; 4usize],
    #[doc = "0x10ec - TCD Last Source Address Adjustment"]
    pub tcd7_slast: TCD7_SLAST,
    #[doc = "0x10f0 - TCD Destination Address"]
    pub tcd7_daddr: TCD7_DADDR,
    #[doc = "0x10f4 - TCD Signed Destination Address Offset"]
    pub tcd7_doff: TCD7_DOFF,
    _reserved_116_tcd7_citer: [u8; 2usize],
    #[doc = "0x10f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd7_dlastsga: TCD7_DLASTSGA,
    #[doc = "0x10fc - TCD Control and Status"]
    pub tcd7_csr: TCD7_CSR,
    _reserved_119_tcd7_biter: [u8; 2usize],
    #[doc = "0x1100 - TCD Source Address"]
    pub tcd8_saddr: TCD8_SADDR,
    #[doc = "0x1104 - TCD Signed Source Address Offset"]
    pub tcd8_soff: TCD8_SOFF,
    #[doc = "0x1106 - TCD Transfer Attributes"]
    pub tcd8_attr: TCD8_ATTR,
    _reserved_123_tcd8_nbytes: [u8; 4usize],
    #[doc = "0x110c - TCD Last Source Address Adjustment"]
    pub tcd8_slast: TCD8_SLAST,
    #[doc = "0x1110 - TCD Destination Address"]
    pub tcd8_daddr: TCD8_DADDR,
    #[doc = "0x1114 - TCD Signed Destination Address Offset"]
    pub tcd8_doff: TCD8_DOFF,
    _reserved_127_tcd8_citer: [u8; 2usize],
    #[doc = "0x1118 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd8_dlastsga: TCD8_DLASTSGA,
    #[doc = "0x111c - TCD Control and Status"]
    pub tcd8_csr: TCD8_CSR,
    _reserved_130_tcd8_biter: [u8; 2usize],
    #[doc = "0x1120 - TCD Source Address"]
    pub tcd9_saddr: TCD9_SADDR,
    #[doc = "0x1124 - TCD Signed Source Address Offset"]
    pub tcd9_soff: TCD9_SOFF,
    #[doc = "0x1126 - TCD Transfer Attributes"]
    pub tcd9_attr: TCD9_ATTR,
    _reserved_134_tcd9_nbytes: [u8; 4usize],
    #[doc = "0x112c - TCD Last Source Address Adjustment"]
    pub tcd9_slast: TCD9_SLAST,
    #[doc = "0x1130 - TCD Destination Address"]
    pub tcd9_daddr: TCD9_DADDR,
    #[doc = "0x1134 - TCD Signed Destination Address Offset"]
    pub tcd9_doff: TCD9_DOFF,
    _reserved_138_tcd9_citer: [u8; 2usize],
    #[doc = "0x1138 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd9_dlastsga: TCD9_DLASTSGA,
    #[doc = "0x113c - TCD Control and Status"]
    pub tcd9_csr: TCD9_CSR,
    _reserved_141_tcd9_biter: [u8; 2usize],
    #[doc = "0x1140 - TCD Source Address"]
    pub tcd10_saddr: TCD10_SADDR,
    #[doc = "0x1144 - TCD Signed Source Address Offset"]
    pub tcd10_soff: TCD10_SOFF,
    #[doc = "0x1146 - TCD Transfer Attributes"]
    pub tcd10_attr: TCD10_ATTR,
    _reserved_145_tcd10_nbytes: [u8; 4usize],
    #[doc = "0x114c - TCD Last Source Address Adjustment"]
    pub tcd10_slast: TCD10_SLAST,
    #[doc = "0x1150 - TCD Destination Address"]
    pub tcd10_daddr: TCD10_DADDR,
    #[doc = "0x1154 - TCD Signed Destination Address Offset"]
    pub tcd10_doff: TCD10_DOFF,
    _reserved_149_tcd10_citer: [u8; 2usize],
    #[doc = "0x1158 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd10_dlastsga: TCD10_DLASTSGA,
    #[doc = "0x115c - TCD Control and Status"]
    pub tcd10_csr: TCD10_CSR,
    _reserved_152_tcd10_biter: [u8; 2usize],
    #[doc = "0x1160 - TCD Source Address"]
    pub tcd11_saddr: TCD11_SADDR,
    #[doc = "0x1164 - TCD Signed Source Address Offset"]
    pub tcd11_soff: TCD11_SOFF,
    #[doc = "0x1166 - TCD Transfer Attributes"]
    pub tcd11_attr: TCD11_ATTR,
    _reserved_156_tcd11_nbytes: [u8; 4usize],
    #[doc = "0x116c - TCD Last Source Address Adjustment"]
    pub tcd11_slast: TCD11_SLAST,
    #[doc = "0x1170 - TCD Destination Address"]
    pub tcd11_daddr: TCD11_DADDR,
    #[doc = "0x1174 - TCD Signed Destination Address Offset"]
    pub tcd11_doff: TCD11_DOFF,
    _reserved_160_tcd11_citer: [u8; 2usize],
    #[doc = "0x1178 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd11_dlastsga: TCD11_DLASTSGA,
    #[doc = "0x117c - TCD Control and Status"]
    pub tcd11_csr: TCD11_CSR,
    _reserved_163_tcd11_biter: [u8; 2usize],
    #[doc = "0x1180 - TCD Source Address"]
    pub tcd12_saddr: TCD12_SADDR,
    #[doc = "0x1184 - TCD Signed Source Address Offset"]
    pub tcd12_soff: TCD12_SOFF,
    #[doc = "0x1186 - TCD Transfer Attributes"]
    pub tcd12_attr: TCD12_ATTR,
    _reserved_167_tcd12_nbytes: [u8; 4usize],
    #[doc = "0x118c - TCD Last Source Address Adjustment"]
    pub tcd12_slast: TCD12_SLAST,
    #[doc = "0x1190 - TCD Destination Address"]
    pub tcd12_daddr: TCD12_DADDR,
    #[doc = "0x1194 - TCD Signed Destination Address Offset"]
    pub tcd12_doff: TCD12_DOFF,
    _reserved_171_tcd12_citer: [u8; 2usize],
    #[doc = "0x1198 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd12_dlastsga: TCD12_DLASTSGA,
    #[doc = "0x119c - TCD Control and Status"]
    pub tcd12_csr: TCD12_CSR,
    _reserved_174_tcd12_biter: [u8; 2usize],
    #[doc = "0x11a0 - TCD Source Address"]
    pub tcd13_saddr: TCD13_SADDR,
    #[doc = "0x11a4 - TCD Signed Source Address Offset"]
    pub tcd13_soff: TCD13_SOFF,
    #[doc = "0x11a6 - TCD Transfer Attributes"]
    pub tcd13_attr: TCD13_ATTR,
    _reserved_178_tcd13_nbytes: [u8; 4usize],
    #[doc = "0x11ac - TCD Last Source Address Adjustment"]
    pub tcd13_slast: TCD13_SLAST,
    #[doc = "0x11b0 - TCD Destination Address"]
    pub tcd13_daddr: TCD13_DADDR,
    #[doc = "0x11b4 - TCD Signed Destination Address Offset"]
    pub tcd13_doff: TCD13_DOFF,
    _reserved_182_tcd13_citer: [u8; 2usize],
    #[doc = "0x11b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd13_dlastsga: TCD13_DLASTSGA,
    #[doc = "0x11bc - TCD Control and Status"]
    pub tcd13_csr: TCD13_CSR,
    _reserved_185_tcd13_biter: [u8; 2usize],
    #[doc = "0x11c0 - TCD Source Address"]
    pub tcd14_saddr: TCD14_SADDR,
    #[doc = "0x11c4 - TCD Signed Source Address Offset"]
    pub tcd14_soff: TCD14_SOFF,
    #[doc = "0x11c6 - TCD Transfer Attributes"]
    pub tcd14_attr: TCD14_ATTR,
    _reserved_189_tcd14_nbytes: [u8; 4usize],
    #[doc = "0x11cc - TCD Last Source Address Adjustment"]
    pub tcd14_slast: TCD14_SLAST,
    #[doc = "0x11d0 - TCD Destination Address"]
    pub tcd14_daddr: TCD14_DADDR,
    #[doc = "0x11d4 - TCD Signed Destination Address Offset"]
    pub tcd14_doff: TCD14_DOFF,
    _reserved_193_tcd14_citer: [u8; 2usize],
    #[doc = "0x11d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd14_dlastsga: TCD14_DLASTSGA,
    #[doc = "0x11dc - TCD Control and Status"]
    pub tcd14_csr: TCD14_CSR,
    _reserved_196_tcd14_biter: [u8; 2usize],
    #[doc = "0x11e0 - TCD Source Address"]
    pub tcd15_saddr: TCD15_SADDR,
    #[doc = "0x11e4 - TCD Signed Source Address Offset"]
    pub tcd15_soff: TCD15_SOFF,
    #[doc = "0x11e6 - TCD Transfer Attributes"]
    pub tcd15_attr: TCD15_ATTR,
    _reserved_200_tcd15_nbytes: [u8; 4usize],
    #[doc = "0x11ec - TCD Last Source Address Adjustment"]
    pub tcd15_slast: TCD15_SLAST,
    #[doc = "0x11f0 - TCD Destination Address"]
    pub tcd15_daddr: TCD15_DADDR,
    #[doc = "0x11f4 - TCD Signed Destination Address Offset"]
    pub tcd15_doff: TCD15_DOFF,
    _reserved_204_tcd15_citer: [u8; 2usize],
    #[doc = "0x11f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd15_dlastsga: TCD15_DLASTSGA,
    #[doc = "0x11fc - TCD Control and Status"]
    pub tcd15_csr: TCD15_CSR,
    _reserved_207_tcd15_biter: [u8; 2usize],
}
impl RegisterBlock {
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mloffyes(&self) -> &TCD0_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4104usize) as *const TCD0_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mloffyes_mut(&self) -> &mut TCD0_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4104usize) as *mut TCD0_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mloffno(&self) -> &TCD0_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4104usize) as *const TCD0_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mloffno_mut(&self) -> &mut TCD0_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4104usize) as *mut TCD0_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1008 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mlno(&self) -> &TCD0_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4104usize) as *const TCD0_NBYTES_MLNO) }
    }
    #[doc = "0x1008 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd0_nbytes_mlno_mut(&self) -> &mut TCD0_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4104usize) as *mut TCD0_NBYTES_MLNO) }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd0_citer_elinkyes(&self) -> &TCD0_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4118usize) as *const TCD0_CITER_ELINKYES) }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd0_citer_elinkyes_mut(&self) -> &mut TCD0_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4118usize) as *mut TCD0_CITER_ELINKYES) }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd0_citer_elinkno(&self) -> &TCD0_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4118usize) as *const TCD0_CITER_ELINKNO) }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd0_citer_elinkno_mut(&self) -> &mut TCD0_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4118usize) as *mut TCD0_CITER_ELINKNO) }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd0_biter_elinkyes(&self) -> &TCD0_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4126usize) as *const TCD0_BITER_ELINKYES) }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd0_biter_elinkyes_mut(&self) -> &mut TCD0_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4126usize) as *mut TCD0_BITER_ELINKYES) }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd0_biter_elinkno(&self) -> &TCD0_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4126usize) as *const TCD0_BITER_ELINKNO) }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd0_biter_elinkno_mut(&self) -> &mut TCD0_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4126usize) as *mut TCD0_BITER_ELINKNO) }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mloffyes(&self) -> &TCD1_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4136usize) as *const TCD1_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mloffyes_mut(&self) -> &mut TCD1_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4136usize) as *mut TCD1_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mloffno(&self) -> &TCD1_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4136usize) as *const TCD1_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mloffno_mut(&self) -> &mut TCD1_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4136usize) as *mut TCD1_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1028 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mlno(&self) -> &TCD1_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4136usize) as *const TCD1_NBYTES_MLNO) }
    }
    #[doc = "0x1028 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd1_nbytes_mlno_mut(&self) -> &mut TCD1_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4136usize) as *mut TCD1_NBYTES_MLNO) }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd1_citer_elinkyes(&self) -> &TCD1_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4150usize) as *const TCD1_CITER_ELINKYES) }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd1_citer_elinkyes_mut(&self) -> &mut TCD1_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4150usize) as *mut TCD1_CITER_ELINKYES) }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd1_citer_elinkno(&self) -> &TCD1_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4150usize) as *const TCD1_CITER_ELINKNO) }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd1_citer_elinkno_mut(&self) -> &mut TCD1_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4150usize) as *mut TCD1_CITER_ELINKNO) }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd1_biter_elinkyes(&self) -> &TCD1_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4158usize) as *const TCD1_BITER_ELINKYES) }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd1_biter_elinkyes_mut(&self) -> &mut TCD1_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4158usize) as *mut TCD1_BITER_ELINKYES) }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd1_biter_elinkno(&self) -> &TCD1_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4158usize) as *const TCD1_BITER_ELINKNO) }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd1_biter_elinkno_mut(&self) -> &mut TCD1_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4158usize) as *mut TCD1_BITER_ELINKNO) }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mloffyes(&self) -> &TCD2_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4168usize) as *const TCD2_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mloffyes_mut(&self) -> &mut TCD2_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4168usize) as *mut TCD2_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mloffno(&self) -> &TCD2_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4168usize) as *const TCD2_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mloffno_mut(&self) -> &mut TCD2_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4168usize) as *mut TCD2_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1048 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mlno(&self) -> &TCD2_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4168usize) as *const TCD2_NBYTES_MLNO) }
    }
    #[doc = "0x1048 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd2_nbytes_mlno_mut(&self) -> &mut TCD2_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4168usize) as *mut TCD2_NBYTES_MLNO) }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd2_citer_elinkyes(&self) -> &TCD2_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4182usize) as *const TCD2_CITER_ELINKYES) }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd2_citer_elinkyes_mut(&self) -> &mut TCD2_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4182usize) as *mut TCD2_CITER_ELINKYES) }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd2_citer_elinkno(&self) -> &TCD2_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4182usize) as *const TCD2_CITER_ELINKNO) }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd2_citer_elinkno_mut(&self) -> &mut TCD2_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4182usize) as *mut TCD2_CITER_ELINKNO) }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd2_biter_elinkyes(&self) -> &TCD2_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4190usize) as *const TCD2_BITER_ELINKYES) }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd2_biter_elinkyes_mut(&self) -> &mut TCD2_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4190usize) as *mut TCD2_BITER_ELINKYES) }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd2_biter_elinkno(&self) -> &TCD2_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4190usize) as *const TCD2_BITER_ELINKNO) }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd2_biter_elinkno_mut(&self) -> &mut TCD2_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4190usize) as *mut TCD2_BITER_ELINKNO) }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mloffyes(&self) -> &TCD3_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4200usize) as *const TCD3_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mloffyes_mut(&self) -> &mut TCD3_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4200usize) as *mut TCD3_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mloffno(&self) -> &TCD3_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4200usize) as *const TCD3_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mloffno_mut(&self) -> &mut TCD3_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4200usize) as *mut TCD3_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1068 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mlno(&self) -> &TCD3_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4200usize) as *const TCD3_NBYTES_MLNO) }
    }
    #[doc = "0x1068 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd3_nbytes_mlno_mut(&self) -> &mut TCD3_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4200usize) as *mut TCD3_NBYTES_MLNO) }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd3_citer_elinkyes(&self) -> &TCD3_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4214usize) as *const TCD3_CITER_ELINKYES) }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd3_citer_elinkyes_mut(&self) -> &mut TCD3_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4214usize) as *mut TCD3_CITER_ELINKYES) }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd3_citer_elinkno(&self) -> &TCD3_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4214usize) as *const TCD3_CITER_ELINKNO) }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd3_citer_elinkno_mut(&self) -> &mut TCD3_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4214usize) as *mut TCD3_CITER_ELINKNO) }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd3_biter_elinkyes(&self) -> &TCD3_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4222usize) as *const TCD3_BITER_ELINKYES) }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd3_biter_elinkyes_mut(&self) -> &mut TCD3_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4222usize) as *mut TCD3_BITER_ELINKYES) }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd3_biter_elinkno(&self) -> &TCD3_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4222usize) as *const TCD3_BITER_ELINKNO) }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd3_biter_elinkno_mut(&self) -> &mut TCD3_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4222usize) as *mut TCD3_BITER_ELINKNO) }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mloffyes(&self) -> &TCD4_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4232usize) as *const TCD4_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mloffyes_mut(&self) -> &mut TCD4_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4232usize) as *mut TCD4_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mloffno(&self) -> &TCD4_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4232usize) as *const TCD4_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1088 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mloffno_mut(&self) -> &mut TCD4_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4232usize) as *mut TCD4_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1088 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mlno(&self) -> &TCD4_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4232usize) as *const TCD4_NBYTES_MLNO) }
    }
    #[doc = "0x1088 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd4_nbytes_mlno_mut(&self) -> &mut TCD4_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4232usize) as *mut TCD4_NBYTES_MLNO) }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd4_citer_elinkyes(&self) -> &TCD4_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4246usize) as *const TCD4_CITER_ELINKYES) }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd4_citer_elinkyes_mut(&self) -> &mut TCD4_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4246usize) as *mut TCD4_CITER_ELINKYES) }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd4_citer_elinkno(&self) -> &TCD4_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4246usize) as *const TCD4_CITER_ELINKNO) }
    }
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd4_citer_elinkno_mut(&self) -> &mut TCD4_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4246usize) as *mut TCD4_CITER_ELINKNO) }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd4_biter_elinkyes(&self) -> &TCD4_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4254usize) as *const TCD4_BITER_ELINKYES) }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd4_biter_elinkyes_mut(&self) -> &mut TCD4_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4254usize) as *mut TCD4_BITER_ELINKYES) }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd4_biter_elinkno(&self) -> &TCD4_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4254usize) as *const TCD4_BITER_ELINKNO) }
    }
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd4_biter_elinkno_mut(&self) -> &mut TCD4_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4254usize) as *mut TCD4_BITER_ELINKNO) }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mloffyes(&self) -> &TCD5_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4264usize) as *const TCD5_NBYTES_MLOFFYES) }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mloffyes_mut(&self) -> &mut TCD5_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4264usize) as *mut TCD5_NBYTES_MLOFFYES) }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mloffno(&self) -> &TCD5_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4264usize) as *const TCD5_NBYTES_MLOFFNO) }
    }
    #[doc = "0x10a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mloffno_mut(&self) -> &mut TCD5_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4264usize) as *mut TCD5_NBYTES_MLOFFNO) }
    }
    #[doc = "0x10a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mlno(&self) -> &TCD5_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4264usize) as *const TCD5_NBYTES_MLNO) }
    }
    #[doc = "0x10a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd5_nbytes_mlno_mut(&self) -> &mut TCD5_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4264usize) as *mut TCD5_NBYTES_MLNO) }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd5_citer_elinkyes(&self) -> &TCD5_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4278usize) as *const TCD5_CITER_ELINKYES) }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd5_citer_elinkyes_mut(&self) -> &mut TCD5_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4278usize) as *mut TCD5_CITER_ELINKYES) }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd5_citer_elinkno(&self) -> &TCD5_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4278usize) as *const TCD5_CITER_ELINKNO) }
    }
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd5_citer_elinkno_mut(&self) -> &mut TCD5_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4278usize) as *mut TCD5_CITER_ELINKNO) }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd5_biter_elinkyes(&self) -> &TCD5_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4286usize) as *const TCD5_BITER_ELINKYES) }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd5_biter_elinkyes_mut(&self) -> &mut TCD5_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4286usize) as *mut TCD5_BITER_ELINKYES) }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd5_biter_elinkno(&self) -> &TCD5_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4286usize) as *const TCD5_BITER_ELINKNO) }
    }
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd5_biter_elinkno_mut(&self) -> &mut TCD5_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4286usize) as *mut TCD5_BITER_ELINKNO) }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mloffyes(&self) -> &TCD6_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4296usize) as *const TCD6_NBYTES_MLOFFYES) }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mloffyes_mut(&self) -> &mut TCD6_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4296usize) as *mut TCD6_NBYTES_MLOFFYES) }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mloffno(&self) -> &TCD6_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4296usize) as *const TCD6_NBYTES_MLOFFNO) }
    }
    #[doc = "0x10c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mloffno_mut(&self) -> &mut TCD6_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4296usize) as *mut TCD6_NBYTES_MLOFFNO) }
    }
    #[doc = "0x10c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mlno(&self) -> &TCD6_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4296usize) as *const TCD6_NBYTES_MLNO) }
    }
    #[doc = "0x10c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd6_nbytes_mlno_mut(&self) -> &mut TCD6_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4296usize) as *mut TCD6_NBYTES_MLNO) }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd6_citer_elinkyes(&self) -> &TCD6_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4310usize) as *const TCD6_CITER_ELINKYES) }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd6_citer_elinkyes_mut(&self) -> &mut TCD6_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4310usize) as *mut TCD6_CITER_ELINKYES) }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd6_citer_elinkno(&self) -> &TCD6_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4310usize) as *const TCD6_CITER_ELINKNO) }
    }
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd6_citer_elinkno_mut(&self) -> &mut TCD6_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4310usize) as *mut TCD6_CITER_ELINKNO) }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd6_biter_elinkyes(&self) -> &TCD6_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4318usize) as *const TCD6_BITER_ELINKYES) }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd6_biter_elinkyes_mut(&self) -> &mut TCD6_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4318usize) as *mut TCD6_BITER_ELINKYES) }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd6_biter_elinkno(&self) -> &TCD6_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4318usize) as *const TCD6_BITER_ELINKNO) }
    }
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd6_biter_elinkno_mut(&self) -> &mut TCD6_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4318usize) as *mut TCD6_BITER_ELINKNO) }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mloffyes(&self) -> &TCD7_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4328usize) as *const TCD7_NBYTES_MLOFFYES) }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mloffyes_mut(&self) -> &mut TCD7_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4328usize) as *mut TCD7_NBYTES_MLOFFYES) }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mloffno(&self) -> &TCD7_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4328usize) as *const TCD7_NBYTES_MLOFFNO) }
    }
    #[doc = "0x10e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mloffno_mut(&self) -> &mut TCD7_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4328usize) as *mut TCD7_NBYTES_MLOFFNO) }
    }
    #[doc = "0x10e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mlno(&self) -> &TCD7_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4328usize) as *const TCD7_NBYTES_MLNO) }
    }
    #[doc = "0x10e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd7_nbytes_mlno_mut(&self) -> &mut TCD7_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4328usize) as *mut TCD7_NBYTES_MLNO) }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd7_citer_elinkyes(&self) -> &TCD7_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4342usize) as *const TCD7_CITER_ELINKYES) }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd7_citer_elinkyes_mut(&self) -> &mut TCD7_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4342usize) as *mut TCD7_CITER_ELINKYES) }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd7_citer_elinkno(&self) -> &TCD7_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4342usize) as *const TCD7_CITER_ELINKNO) }
    }
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd7_citer_elinkno_mut(&self) -> &mut TCD7_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4342usize) as *mut TCD7_CITER_ELINKNO) }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd7_biter_elinkyes(&self) -> &TCD7_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4350usize) as *const TCD7_BITER_ELINKYES) }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd7_biter_elinkyes_mut(&self) -> &mut TCD7_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4350usize) as *mut TCD7_BITER_ELINKYES) }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd7_biter_elinkno(&self) -> &TCD7_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4350usize) as *const TCD7_BITER_ELINKNO) }
    }
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd7_biter_elinkno_mut(&self) -> &mut TCD7_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4350usize) as *mut TCD7_BITER_ELINKNO) }
    }
    #[doc = "0x1108 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd8_nbytes_mloffyes(&self) -> &TCD8_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4360usize) as *const TCD8_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1108 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd8_nbytes_mloffyes_mut(&self) -> &mut TCD8_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4360usize) as *mut TCD8_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1108 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd8_nbytes_mloffno(&self) -> &TCD8_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4360usize) as *const TCD8_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1108 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd8_nbytes_mloffno_mut(&self) -> &mut TCD8_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4360usize) as *mut TCD8_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1108 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd8_nbytes_mlno(&self) -> &TCD8_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4360usize) as *const TCD8_NBYTES_MLNO) }
    }
    #[doc = "0x1108 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd8_nbytes_mlno_mut(&self) -> &mut TCD8_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4360usize) as *mut TCD8_NBYTES_MLNO) }
    }
    #[doc = "0x1116 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd8_citer_elinkyes(&self) -> &TCD8_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4374usize) as *const TCD8_CITER_ELINKYES) }
    }
    #[doc = "0x1116 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd8_citer_elinkyes_mut(&self) -> &mut TCD8_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4374usize) as *mut TCD8_CITER_ELINKYES) }
    }
    #[doc = "0x1116 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd8_citer_elinkno(&self) -> &TCD8_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4374usize) as *const TCD8_CITER_ELINKNO) }
    }
    #[doc = "0x1116 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd8_citer_elinkno_mut(&self) -> &mut TCD8_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4374usize) as *mut TCD8_CITER_ELINKNO) }
    }
    #[doc = "0x111e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd8_biter_elinkyes(&self) -> &TCD8_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4382usize) as *const TCD8_BITER_ELINKYES) }
    }
    #[doc = "0x111e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd8_biter_elinkyes_mut(&self) -> &mut TCD8_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4382usize) as *mut TCD8_BITER_ELINKYES) }
    }
    #[doc = "0x111e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd8_biter_elinkno(&self) -> &TCD8_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4382usize) as *const TCD8_BITER_ELINKNO) }
    }
    #[doc = "0x111e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd8_biter_elinkno_mut(&self) -> &mut TCD8_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4382usize) as *mut TCD8_BITER_ELINKNO) }
    }
    #[doc = "0x1128 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd9_nbytes_mloffyes(&self) -> &TCD9_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4392usize) as *const TCD9_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1128 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd9_nbytes_mloffyes_mut(&self) -> &mut TCD9_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4392usize) as *mut TCD9_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1128 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd9_nbytes_mloffno(&self) -> &TCD9_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4392usize) as *const TCD9_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1128 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd9_nbytes_mloffno_mut(&self) -> &mut TCD9_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4392usize) as *mut TCD9_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1128 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd9_nbytes_mlno(&self) -> &TCD9_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4392usize) as *const TCD9_NBYTES_MLNO) }
    }
    #[doc = "0x1128 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd9_nbytes_mlno_mut(&self) -> &mut TCD9_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4392usize) as *mut TCD9_NBYTES_MLNO) }
    }
    #[doc = "0x1136 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd9_citer_elinkyes(&self) -> &TCD9_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4406usize) as *const TCD9_CITER_ELINKYES) }
    }
    #[doc = "0x1136 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd9_citer_elinkyes_mut(&self) -> &mut TCD9_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4406usize) as *mut TCD9_CITER_ELINKYES) }
    }
    #[doc = "0x1136 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd9_citer_elinkno(&self) -> &TCD9_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4406usize) as *const TCD9_CITER_ELINKNO) }
    }
    #[doc = "0x1136 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd9_citer_elinkno_mut(&self) -> &mut TCD9_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4406usize) as *mut TCD9_CITER_ELINKNO) }
    }
    #[doc = "0x113e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd9_biter_elinkyes(&self) -> &TCD9_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4414usize) as *const TCD9_BITER_ELINKYES) }
    }
    #[doc = "0x113e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd9_biter_elinkyes_mut(&self) -> &mut TCD9_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4414usize) as *mut TCD9_BITER_ELINKYES) }
    }
    #[doc = "0x113e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd9_biter_elinkno(&self) -> &TCD9_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4414usize) as *const TCD9_BITER_ELINKNO) }
    }
    #[doc = "0x113e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd9_biter_elinkno_mut(&self) -> &mut TCD9_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4414usize) as *mut TCD9_BITER_ELINKNO) }
    }
    #[doc = "0x1148 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd10_nbytes_mloffyes(&self) -> &TCD10_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4424usize) as *const TCD10_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1148 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd10_nbytes_mloffyes_mut(&self) -> &mut TCD10_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4424usize) as *mut TCD10_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1148 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd10_nbytes_mloffno(&self) -> &TCD10_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4424usize) as *const TCD10_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1148 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd10_nbytes_mloffno_mut(&self) -> &mut TCD10_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4424usize) as *mut TCD10_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1148 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd10_nbytes_mlno(&self) -> &TCD10_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4424usize) as *const TCD10_NBYTES_MLNO) }
    }
    #[doc = "0x1148 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd10_nbytes_mlno_mut(&self) -> &mut TCD10_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4424usize) as *mut TCD10_NBYTES_MLNO) }
    }
    #[doc = "0x1156 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd10_citer_elinkyes(&self) -> &TCD10_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4438usize) as *const TCD10_CITER_ELINKYES) }
    }
    #[doc = "0x1156 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd10_citer_elinkyes_mut(&self) -> &mut TCD10_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4438usize) as *mut TCD10_CITER_ELINKYES) }
    }
    #[doc = "0x1156 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd10_citer_elinkno(&self) -> &TCD10_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4438usize) as *const TCD10_CITER_ELINKNO) }
    }
    #[doc = "0x1156 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd10_citer_elinkno_mut(&self) -> &mut TCD10_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4438usize) as *mut TCD10_CITER_ELINKNO) }
    }
    #[doc = "0x115e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd10_biter_elinkyes(&self) -> &TCD10_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4446usize) as *const TCD10_BITER_ELINKYES) }
    }
    #[doc = "0x115e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd10_biter_elinkyes_mut(&self) -> &mut TCD10_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4446usize) as *mut TCD10_BITER_ELINKYES) }
    }
    #[doc = "0x115e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd10_biter_elinkno(&self) -> &TCD10_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4446usize) as *const TCD10_BITER_ELINKNO) }
    }
    #[doc = "0x115e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd10_biter_elinkno_mut(&self) -> &mut TCD10_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4446usize) as *mut TCD10_BITER_ELINKNO) }
    }
    #[doc = "0x1168 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd11_nbytes_mloffyes(&self) -> &TCD11_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4456usize) as *const TCD11_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1168 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd11_nbytes_mloffyes_mut(&self) -> &mut TCD11_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4456usize) as *mut TCD11_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1168 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd11_nbytes_mloffno(&self) -> &TCD11_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4456usize) as *const TCD11_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1168 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd11_nbytes_mloffno_mut(&self) -> &mut TCD11_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4456usize) as *mut TCD11_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1168 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd11_nbytes_mlno(&self) -> &TCD11_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4456usize) as *const TCD11_NBYTES_MLNO) }
    }
    #[doc = "0x1168 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd11_nbytes_mlno_mut(&self) -> &mut TCD11_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4456usize) as *mut TCD11_NBYTES_MLNO) }
    }
    #[doc = "0x1176 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd11_citer_elinkyes(&self) -> &TCD11_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4470usize) as *const TCD11_CITER_ELINKYES) }
    }
    #[doc = "0x1176 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd11_citer_elinkyes_mut(&self) -> &mut TCD11_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4470usize) as *mut TCD11_CITER_ELINKYES) }
    }
    #[doc = "0x1176 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd11_citer_elinkno(&self) -> &TCD11_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4470usize) as *const TCD11_CITER_ELINKNO) }
    }
    #[doc = "0x1176 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd11_citer_elinkno_mut(&self) -> &mut TCD11_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4470usize) as *mut TCD11_CITER_ELINKNO) }
    }
    #[doc = "0x117e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd11_biter_elinkyes(&self) -> &TCD11_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4478usize) as *const TCD11_BITER_ELINKYES) }
    }
    #[doc = "0x117e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd11_biter_elinkyes_mut(&self) -> &mut TCD11_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4478usize) as *mut TCD11_BITER_ELINKYES) }
    }
    #[doc = "0x117e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd11_biter_elinkno(&self) -> &TCD11_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4478usize) as *const TCD11_BITER_ELINKNO) }
    }
    #[doc = "0x117e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd11_biter_elinkno_mut(&self) -> &mut TCD11_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4478usize) as *mut TCD11_BITER_ELINKNO) }
    }
    #[doc = "0x1188 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd12_nbytes_mloffyes(&self) -> &TCD12_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4488usize) as *const TCD12_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1188 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd12_nbytes_mloffyes_mut(&self) -> &mut TCD12_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4488usize) as *mut TCD12_NBYTES_MLOFFYES) }
    }
    #[doc = "0x1188 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd12_nbytes_mloffno(&self) -> &TCD12_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4488usize) as *const TCD12_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1188 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd12_nbytes_mloffno_mut(&self) -> &mut TCD12_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4488usize) as *mut TCD12_NBYTES_MLOFFNO) }
    }
    #[doc = "0x1188 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd12_nbytes_mlno(&self) -> &TCD12_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4488usize) as *const TCD12_NBYTES_MLNO) }
    }
    #[doc = "0x1188 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd12_nbytes_mlno_mut(&self) -> &mut TCD12_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4488usize) as *mut TCD12_NBYTES_MLNO) }
    }
    #[doc = "0x1196 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd12_citer_elinkyes(&self) -> &TCD12_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4502usize) as *const TCD12_CITER_ELINKYES) }
    }
    #[doc = "0x1196 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd12_citer_elinkyes_mut(&self) -> &mut TCD12_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4502usize) as *mut TCD12_CITER_ELINKYES) }
    }
    #[doc = "0x1196 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd12_citer_elinkno(&self) -> &TCD12_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4502usize) as *const TCD12_CITER_ELINKNO) }
    }
    #[doc = "0x1196 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd12_citer_elinkno_mut(&self) -> &mut TCD12_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4502usize) as *mut TCD12_CITER_ELINKNO) }
    }
    #[doc = "0x119e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd12_biter_elinkyes(&self) -> &TCD12_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4510usize) as *const TCD12_BITER_ELINKYES) }
    }
    #[doc = "0x119e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd12_biter_elinkyes_mut(&self) -> &mut TCD12_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4510usize) as *mut TCD12_BITER_ELINKYES) }
    }
    #[doc = "0x119e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd12_biter_elinkno(&self) -> &TCD12_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4510usize) as *const TCD12_BITER_ELINKNO) }
    }
    #[doc = "0x119e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd12_biter_elinkno_mut(&self) -> &mut TCD12_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4510usize) as *mut TCD12_BITER_ELINKNO) }
    }
    #[doc = "0x11a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd13_nbytes_mloffyes(&self) -> &TCD13_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4520usize) as *const TCD13_NBYTES_MLOFFYES) }
    }
    #[doc = "0x11a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd13_nbytes_mloffyes_mut(&self) -> &mut TCD13_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4520usize) as *mut TCD13_NBYTES_MLOFFYES) }
    }
    #[doc = "0x11a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd13_nbytes_mloffno(&self) -> &TCD13_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4520usize) as *const TCD13_NBYTES_MLOFFNO) }
    }
    #[doc = "0x11a8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd13_nbytes_mloffno_mut(&self) -> &mut TCD13_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4520usize) as *mut TCD13_NBYTES_MLOFFNO) }
    }
    #[doc = "0x11a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd13_nbytes_mlno(&self) -> &TCD13_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4520usize) as *const TCD13_NBYTES_MLNO) }
    }
    #[doc = "0x11a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd13_nbytes_mlno_mut(&self) -> &mut TCD13_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4520usize) as *mut TCD13_NBYTES_MLNO) }
    }
    #[doc = "0x11b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd13_citer_elinkyes(&self) -> &TCD13_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4534usize) as *const TCD13_CITER_ELINKYES) }
    }
    #[doc = "0x11b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd13_citer_elinkyes_mut(&self) -> &mut TCD13_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4534usize) as *mut TCD13_CITER_ELINKYES) }
    }
    #[doc = "0x11b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd13_citer_elinkno(&self) -> &TCD13_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4534usize) as *const TCD13_CITER_ELINKNO) }
    }
    #[doc = "0x11b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd13_citer_elinkno_mut(&self) -> &mut TCD13_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4534usize) as *mut TCD13_CITER_ELINKNO) }
    }
    #[doc = "0x11be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd13_biter_elinkyes(&self) -> &TCD13_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4542usize) as *const TCD13_BITER_ELINKYES) }
    }
    #[doc = "0x11be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd13_biter_elinkyes_mut(&self) -> &mut TCD13_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4542usize) as *mut TCD13_BITER_ELINKYES) }
    }
    #[doc = "0x11be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd13_biter_elinkno(&self) -> &TCD13_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4542usize) as *const TCD13_BITER_ELINKNO) }
    }
    #[doc = "0x11be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd13_biter_elinkno_mut(&self) -> &mut TCD13_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4542usize) as *mut TCD13_BITER_ELINKNO) }
    }
    #[doc = "0x11c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd14_nbytes_mloffyes(&self) -> &TCD14_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4552usize) as *const TCD14_NBYTES_MLOFFYES) }
    }
    #[doc = "0x11c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd14_nbytes_mloffyes_mut(&self) -> &mut TCD14_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4552usize) as *mut TCD14_NBYTES_MLOFFYES) }
    }
    #[doc = "0x11c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd14_nbytes_mloffno(&self) -> &TCD14_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4552usize) as *const TCD14_NBYTES_MLOFFNO) }
    }
    #[doc = "0x11c8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd14_nbytes_mloffno_mut(&self) -> &mut TCD14_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4552usize) as *mut TCD14_NBYTES_MLOFFNO) }
    }
    #[doc = "0x11c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd14_nbytes_mlno(&self) -> &TCD14_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4552usize) as *const TCD14_NBYTES_MLNO) }
    }
    #[doc = "0x11c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd14_nbytes_mlno_mut(&self) -> &mut TCD14_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4552usize) as *mut TCD14_NBYTES_MLNO) }
    }
    #[doc = "0x11d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd14_citer_elinkyes(&self) -> &TCD14_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4566usize) as *const TCD14_CITER_ELINKYES) }
    }
    #[doc = "0x11d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd14_citer_elinkyes_mut(&self) -> &mut TCD14_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4566usize) as *mut TCD14_CITER_ELINKYES) }
    }
    #[doc = "0x11d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd14_citer_elinkno(&self) -> &TCD14_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4566usize) as *const TCD14_CITER_ELINKNO) }
    }
    #[doc = "0x11d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd14_citer_elinkno_mut(&self) -> &mut TCD14_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4566usize) as *mut TCD14_CITER_ELINKNO) }
    }
    #[doc = "0x11de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd14_biter_elinkyes(&self) -> &TCD14_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4574usize) as *const TCD14_BITER_ELINKYES) }
    }
    #[doc = "0x11de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd14_biter_elinkyes_mut(&self) -> &mut TCD14_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4574usize) as *mut TCD14_BITER_ELINKYES) }
    }
    #[doc = "0x11de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd14_biter_elinkno(&self) -> &TCD14_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4574usize) as *const TCD14_BITER_ELINKNO) }
    }
    #[doc = "0x11de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd14_biter_elinkno_mut(&self) -> &mut TCD14_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4574usize) as *mut TCD14_BITER_ELINKNO) }
    }
    #[doc = "0x11e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd15_nbytes_mloffyes(&self) -> &TCD15_NBYTES_MLOFFYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4584usize) as *const TCD15_NBYTES_MLOFFYES) }
    }
    #[doc = "0x11e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
    #[inline(always)]
    pub fn tcd15_nbytes_mloffyes_mut(&self) -> &mut TCD15_NBYTES_MLOFFYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4584usize) as *mut TCD15_NBYTES_MLOFFYES) }
    }
    #[doc = "0x11e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd15_nbytes_mloffno(&self) -> &TCD15_NBYTES_MLOFFNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4584usize) as *const TCD15_NBYTES_MLOFFNO) }
    }
    #[doc = "0x11e8 - TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
    #[inline(always)]
    pub fn tcd15_nbytes_mloffno_mut(&self) -> &mut TCD15_NBYTES_MLOFFNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4584usize) as *mut TCD15_NBYTES_MLOFFNO) }
    }
    #[doc = "0x11e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd15_nbytes_mlno(&self) -> &TCD15_NBYTES_MLNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4584usize) as *const TCD15_NBYTES_MLNO) }
    }
    #[doc = "0x11e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    #[inline(always)]
    pub fn tcd15_nbytes_mlno_mut(&self) -> &mut TCD15_NBYTES_MLNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4584usize) as *mut TCD15_NBYTES_MLNO) }
    }
    #[doc = "0x11f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd15_citer_elinkyes(&self) -> &TCD15_CITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4598usize) as *const TCD15_CITER_ELINKYES) }
    }
    #[doc = "0x11f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd15_citer_elinkyes_mut(&self) -> &mut TCD15_CITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4598usize) as *mut TCD15_CITER_ELINKYES) }
    }
    #[doc = "0x11f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd15_citer_elinkno(&self) -> &TCD15_CITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4598usize) as *const TCD15_CITER_ELINKNO) }
    }
    #[doc = "0x11f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd15_citer_elinkno_mut(&self) -> &mut TCD15_CITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4598usize) as *mut TCD15_CITER_ELINKNO) }
    }
    #[doc = "0x11fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd15_biter_elinkyes(&self) -> &TCD15_BITER_ELINKYES {
        unsafe { &*(((self as *const Self) as *const u8).add(4606usize) as *const TCD15_BITER_ELINKYES) }
    }
    #[doc = "0x11fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub fn tcd15_biter_elinkyes_mut(&self) -> &mut TCD15_BITER_ELINKYES {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4606usize) as *mut TCD15_BITER_ELINKYES) }
    }
    #[doc = "0x11fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd15_biter_elinkno(&self) -> &TCD15_BITER_ELINKNO {
        unsafe { &*(((self as *const Self) as *const u8).add(4606usize) as *const TCD15_BITER_ELINKNO) }
    }
    #[doc = "0x11fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub fn tcd15_biter_elinkno_mut(&self) -> &mut TCD15_BITER_ELINKNO {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4606usize) as *mut TCD15_BITER_ELINKNO) }
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
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd0_saddr](tcd0_saddr) module"]
pub type TCD0_SADDR = crate::Reg<u32, _TCD0_SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD0_SADDR;
#[doc = "`read()` method returns [tcd0_saddr::R](tcd0_saddr::R) reader structure"]
impl crate::Readable for TCD0_SADDR {}
#[doc = "`write(|w| ..)` method takes [tcd0_saddr::W](tcd0_saddr::W) writer structure"]
impl crate::Writable for TCD0_SADDR {}
#[doc = "TCD Source Address"]
pub mod tcd0_saddr;
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd0_soff](tcd0_soff) module"]
pub type TCD0_SOFF = crate::Reg<u16, _TCD0_SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD0_SOFF;
#[doc = "`read()` method returns [tcd0_soff::R](tcd0_soff::R) reader structure"]
impl crate::Readable for TCD0_SOFF {}
#[doc = "`write(|w| ..)` method takes [tcd0_soff::W](tcd0_soff::W) writer structure"]
impl crate::Writable for TCD0_SOFF {}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd0_soff;
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd0_attr](tcd0_attr) module"]
pub type TCD0_ATTR = crate::Reg<u16, _TCD0_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD0_ATTR;
#[doc = "`read()` method returns [tcd0_attr::R](tcd0_attr::R) reader structure"]
impl crate::Readable for TCD0_ATTR {}
#[doc = "`write(|w| ..)` method takes [tcd0_attr::W](tcd0_attr::W) writer structure"]
impl crate::Writable for TCD0_ATTR {}
#[doc = "TCD Transfer Attributes"]
pub mod tcd0_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd0_nbytes_mlno](tcd0_nbytes_mlno) module"]
pub type TCD0_NBYTES_MLNO = crate::Reg<u32, _TCD0_NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD0_NBYTES_MLNO;
#[doc = "`read()` method returns [tcd0_nbytes_mlno::R](tcd0_nbytes_mlno::R) reader structure"]
impl crate::Readable for TCD0_NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [tcd0_nbytes_mlno::W](tcd0_nbytes_mlno::W) writer structure"]
impl crate::Writable for TCD0_NBYTES_MLNO {}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd0_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd0_nbytes_mloffno](tcd0_nbytes_mloffno) module"]
pub type TCD0_NBYTES_MLOFFNO = crate::Reg<u32, _TCD0_NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD0_NBYTES_MLOFFNO;
#[doc = "`read()` method returns [tcd0_nbytes_mloffno::R](tcd0_nbytes_mloffno::R) reader structure"]
impl crate::Readable for TCD0_NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [tcd0_nbytes_mloffno::W](tcd0_nbytes_mloffno::W) writer structure"]
impl crate::Writable for TCD0_NBYTES_MLOFFNO {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd0_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd0_nbytes_mloffyes](tcd0_nbytes_mloffyes) module"]
pub type TCD0_NBYTES_MLOFFYES = crate::Reg<u32, _TCD0_NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD0_NBYTES_MLOFFYES;
#[doc = "`read()` method returns [tcd0_nbytes_mloffyes::R](tcd0_nbytes_mloffyes::R) reader structure"]
impl crate::Readable for TCD0_NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [tcd0_nbytes_mloffyes::W](tcd0_nbytes_mloffyes::W) writer structure"]
impl crate::Writable for TCD0_NBYTES_MLOFFYES {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd0_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd0_slast](tcd0_slast) module"]
pub type TCD0_SLAST = crate::Reg<u32, _TCD0_SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD0_SLAST;
#[doc = "`read()` method returns [tcd0_slast::R](tcd0_slast::R) reader structure"]
impl crate::Readable for TCD0_SLAST {}
#[doc = "`write(|w| ..)` method takes [tcd0_slast::W](tcd0_slast::W) writer structure"]
impl crate::Writable for TCD0_SLAST {}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd0_slast;
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd0_daddr](tcd0_daddr) module"]
pub type TCD0_DADDR = crate::Reg<u32, _TCD0_DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD0_DADDR;
#[doc = "`read()` method returns [tcd0_daddr::R](tcd0_daddr::R) reader structure"]
impl crate::Readable for TCD0_DADDR {}
#[doc = "`write(|w| ..)` method takes [tcd0_daddr::W](tcd0_daddr::W) writer structure"]
impl crate::Writable for TCD0_DADDR {}
#[doc = "TCD Destination Address"]
pub mod tcd0_daddr;
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd0_doff](tcd0_doff) module"]
pub type TCD0_DOFF = crate::Reg<u16, _TCD0_DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD0_DOFF;
#[doc = "`read()` method returns [tcd0_doff::R](tcd0_doff::R) reader structure"]
impl crate::Readable for TCD0_DOFF {}
#[doc = "`write(|w| ..)` method takes [tcd0_doff::W](tcd0_doff::W) writer structure"]
impl crate::Writable for TCD0_DOFF {}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd0_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd0_citer_elinkno](tcd0_citer_elinkno) module"]
pub type TCD0_CITER_ELINKNO = crate::Reg<u16, _TCD0_CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD0_CITER_ELINKNO;
#[doc = "`read()` method returns [tcd0_citer_elinkno::R](tcd0_citer_elinkno::R) reader structure"]
impl crate::Readable for TCD0_CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd0_citer_elinkno::W](tcd0_citer_elinkno::W) writer structure"]
impl crate::Writable for TCD0_CITER_ELINKNO {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd0_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd0_citer_elinkyes](tcd0_citer_elinkyes) module"]
pub type TCD0_CITER_ELINKYES = crate::Reg<u16, _TCD0_CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD0_CITER_ELINKYES;
#[doc = "`read()` method returns [tcd0_citer_elinkyes::R](tcd0_citer_elinkyes::R) reader structure"]
impl crate::Readable for TCD0_CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd0_citer_elinkyes::W](tcd0_citer_elinkyes::W) writer structure"]
impl crate::Writable for TCD0_CITER_ELINKYES {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd0_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd0_dlastsga](tcd0_dlastsga) module"]
pub type TCD0_DLASTSGA = crate::Reg<u32, _TCD0_DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD0_DLASTSGA;
#[doc = "`read()` method returns [tcd0_dlastsga::R](tcd0_dlastsga::R) reader structure"]
impl crate::Readable for TCD0_DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [tcd0_dlastsga::W](tcd0_dlastsga::W) writer structure"]
impl crate::Writable for TCD0_DLASTSGA {}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd0_dlastsga;
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd0_csr](tcd0_csr) module"]
pub type TCD0_CSR = crate::Reg<u16, _TCD0_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD0_CSR;
#[doc = "`read()` method returns [tcd0_csr::R](tcd0_csr::R) reader structure"]
impl crate::Readable for TCD0_CSR {}
#[doc = "`write(|w| ..)` method takes [tcd0_csr::W](tcd0_csr::W) writer structure"]
impl crate::Writable for TCD0_CSR {}
#[doc = "TCD Control and Status"]
pub mod tcd0_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd0_biter_elinkno](tcd0_biter_elinkno) module"]
pub type TCD0_BITER_ELINKNO = crate::Reg<u16, _TCD0_BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD0_BITER_ELINKNO;
#[doc = "`read()` method returns [tcd0_biter_elinkno::R](tcd0_biter_elinkno::R) reader structure"]
impl crate::Readable for TCD0_BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd0_biter_elinkno::W](tcd0_biter_elinkno::W) writer structure"]
impl crate::Writable for TCD0_BITER_ELINKNO {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd0_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd0_biter_elinkyes](tcd0_biter_elinkyes) module"]
pub type TCD0_BITER_ELINKYES = crate::Reg<u16, _TCD0_BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD0_BITER_ELINKYES;
#[doc = "`read()` method returns [tcd0_biter_elinkyes::R](tcd0_biter_elinkyes::R) reader structure"]
impl crate::Readable for TCD0_BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd0_biter_elinkyes::W](tcd0_biter_elinkyes::W) writer structure"]
impl crate::Writable for TCD0_BITER_ELINKYES {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd0_biter_elinkyes;
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd1_saddr](tcd1_saddr) module"]
pub type TCD1_SADDR = crate::Reg<u32, _TCD1_SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD1_SADDR;
#[doc = "`read()` method returns [tcd1_saddr::R](tcd1_saddr::R) reader structure"]
impl crate::Readable for TCD1_SADDR {}
#[doc = "`write(|w| ..)` method takes [tcd1_saddr::W](tcd1_saddr::W) writer structure"]
impl crate::Writable for TCD1_SADDR {}
#[doc = "TCD Source Address"]
pub mod tcd1_saddr;
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd1_soff](tcd1_soff) module"]
pub type TCD1_SOFF = crate::Reg<u16, _TCD1_SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD1_SOFF;
#[doc = "`read()` method returns [tcd1_soff::R](tcd1_soff::R) reader structure"]
impl crate::Readable for TCD1_SOFF {}
#[doc = "`write(|w| ..)` method takes [tcd1_soff::W](tcd1_soff::W) writer structure"]
impl crate::Writable for TCD1_SOFF {}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd1_soff;
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd1_attr](tcd1_attr) module"]
pub type TCD1_ATTR = crate::Reg<u16, _TCD1_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD1_ATTR;
#[doc = "`read()` method returns [tcd1_attr::R](tcd1_attr::R) reader structure"]
impl crate::Readable for TCD1_ATTR {}
#[doc = "`write(|w| ..)` method takes [tcd1_attr::W](tcd1_attr::W) writer structure"]
impl crate::Writable for TCD1_ATTR {}
#[doc = "TCD Transfer Attributes"]
pub mod tcd1_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd1_nbytes_mlno](tcd1_nbytes_mlno) module"]
pub type TCD1_NBYTES_MLNO = crate::Reg<u32, _TCD1_NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD1_NBYTES_MLNO;
#[doc = "`read()` method returns [tcd1_nbytes_mlno::R](tcd1_nbytes_mlno::R) reader structure"]
impl crate::Readable for TCD1_NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [tcd1_nbytes_mlno::W](tcd1_nbytes_mlno::W) writer structure"]
impl crate::Writable for TCD1_NBYTES_MLNO {}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd1_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd1_nbytes_mloffno](tcd1_nbytes_mloffno) module"]
pub type TCD1_NBYTES_MLOFFNO = crate::Reg<u32, _TCD1_NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD1_NBYTES_MLOFFNO;
#[doc = "`read()` method returns [tcd1_nbytes_mloffno::R](tcd1_nbytes_mloffno::R) reader structure"]
impl crate::Readable for TCD1_NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [tcd1_nbytes_mloffno::W](tcd1_nbytes_mloffno::W) writer structure"]
impl crate::Writable for TCD1_NBYTES_MLOFFNO {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd1_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd1_nbytes_mloffyes](tcd1_nbytes_mloffyes) module"]
pub type TCD1_NBYTES_MLOFFYES = crate::Reg<u32, _TCD1_NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD1_NBYTES_MLOFFYES;
#[doc = "`read()` method returns [tcd1_nbytes_mloffyes::R](tcd1_nbytes_mloffyes::R) reader structure"]
impl crate::Readable for TCD1_NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [tcd1_nbytes_mloffyes::W](tcd1_nbytes_mloffyes::W) writer structure"]
impl crate::Writable for TCD1_NBYTES_MLOFFYES {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd1_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd1_slast](tcd1_slast) module"]
pub type TCD1_SLAST = crate::Reg<u32, _TCD1_SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD1_SLAST;
#[doc = "`read()` method returns [tcd1_slast::R](tcd1_slast::R) reader structure"]
impl crate::Readable for TCD1_SLAST {}
#[doc = "`write(|w| ..)` method takes [tcd1_slast::W](tcd1_slast::W) writer structure"]
impl crate::Writable for TCD1_SLAST {}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd1_slast;
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd1_daddr](tcd1_daddr) module"]
pub type TCD1_DADDR = crate::Reg<u32, _TCD1_DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD1_DADDR;
#[doc = "`read()` method returns [tcd1_daddr::R](tcd1_daddr::R) reader structure"]
impl crate::Readable for TCD1_DADDR {}
#[doc = "`write(|w| ..)` method takes [tcd1_daddr::W](tcd1_daddr::W) writer structure"]
impl crate::Writable for TCD1_DADDR {}
#[doc = "TCD Destination Address"]
pub mod tcd1_daddr;
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd1_doff](tcd1_doff) module"]
pub type TCD1_DOFF = crate::Reg<u16, _TCD1_DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD1_DOFF;
#[doc = "`read()` method returns [tcd1_doff::R](tcd1_doff::R) reader structure"]
impl crate::Readable for TCD1_DOFF {}
#[doc = "`write(|w| ..)` method takes [tcd1_doff::W](tcd1_doff::W) writer structure"]
impl crate::Writable for TCD1_DOFF {}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd1_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd1_citer_elinkno](tcd1_citer_elinkno) module"]
pub type TCD1_CITER_ELINKNO = crate::Reg<u16, _TCD1_CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD1_CITER_ELINKNO;
#[doc = "`read()` method returns [tcd1_citer_elinkno::R](tcd1_citer_elinkno::R) reader structure"]
impl crate::Readable for TCD1_CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd1_citer_elinkno::W](tcd1_citer_elinkno::W) writer structure"]
impl crate::Writable for TCD1_CITER_ELINKNO {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd1_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd1_citer_elinkyes](tcd1_citer_elinkyes) module"]
pub type TCD1_CITER_ELINKYES = crate::Reg<u16, _TCD1_CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD1_CITER_ELINKYES;
#[doc = "`read()` method returns [tcd1_citer_elinkyes::R](tcd1_citer_elinkyes::R) reader structure"]
impl crate::Readable for TCD1_CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd1_citer_elinkyes::W](tcd1_citer_elinkyes::W) writer structure"]
impl crate::Writable for TCD1_CITER_ELINKYES {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd1_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd1_dlastsga](tcd1_dlastsga) module"]
pub type TCD1_DLASTSGA = crate::Reg<u32, _TCD1_DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD1_DLASTSGA;
#[doc = "`read()` method returns [tcd1_dlastsga::R](tcd1_dlastsga::R) reader structure"]
impl crate::Readable for TCD1_DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [tcd1_dlastsga::W](tcd1_dlastsga::W) writer structure"]
impl crate::Writable for TCD1_DLASTSGA {}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd1_dlastsga;
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd1_csr](tcd1_csr) module"]
pub type TCD1_CSR = crate::Reg<u16, _TCD1_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD1_CSR;
#[doc = "`read()` method returns [tcd1_csr::R](tcd1_csr::R) reader structure"]
impl crate::Readable for TCD1_CSR {}
#[doc = "`write(|w| ..)` method takes [tcd1_csr::W](tcd1_csr::W) writer structure"]
impl crate::Writable for TCD1_CSR {}
#[doc = "TCD Control and Status"]
pub mod tcd1_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd1_biter_elinkno](tcd1_biter_elinkno) module"]
pub type TCD1_BITER_ELINKNO = crate::Reg<u16, _TCD1_BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD1_BITER_ELINKNO;
#[doc = "`read()` method returns [tcd1_biter_elinkno::R](tcd1_biter_elinkno::R) reader structure"]
impl crate::Readable for TCD1_BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd1_biter_elinkno::W](tcd1_biter_elinkno::W) writer structure"]
impl crate::Writable for TCD1_BITER_ELINKNO {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd1_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd1_biter_elinkyes](tcd1_biter_elinkyes) module"]
pub type TCD1_BITER_ELINKYES = crate::Reg<u16, _TCD1_BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD1_BITER_ELINKYES;
#[doc = "`read()` method returns [tcd1_biter_elinkyes::R](tcd1_biter_elinkyes::R) reader structure"]
impl crate::Readable for TCD1_BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd1_biter_elinkyes::W](tcd1_biter_elinkyes::W) writer structure"]
impl crate::Writable for TCD1_BITER_ELINKYES {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd1_biter_elinkyes;
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd2_saddr](tcd2_saddr) module"]
pub type TCD2_SADDR = crate::Reg<u32, _TCD2_SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD2_SADDR;
#[doc = "`read()` method returns [tcd2_saddr::R](tcd2_saddr::R) reader structure"]
impl crate::Readable for TCD2_SADDR {}
#[doc = "`write(|w| ..)` method takes [tcd2_saddr::W](tcd2_saddr::W) writer structure"]
impl crate::Writable for TCD2_SADDR {}
#[doc = "TCD Source Address"]
pub mod tcd2_saddr;
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd2_soff](tcd2_soff) module"]
pub type TCD2_SOFF = crate::Reg<u16, _TCD2_SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD2_SOFF;
#[doc = "`read()` method returns [tcd2_soff::R](tcd2_soff::R) reader structure"]
impl crate::Readable for TCD2_SOFF {}
#[doc = "`write(|w| ..)` method takes [tcd2_soff::W](tcd2_soff::W) writer structure"]
impl crate::Writable for TCD2_SOFF {}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd2_soff;
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd2_attr](tcd2_attr) module"]
pub type TCD2_ATTR = crate::Reg<u16, _TCD2_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD2_ATTR;
#[doc = "`read()` method returns [tcd2_attr::R](tcd2_attr::R) reader structure"]
impl crate::Readable for TCD2_ATTR {}
#[doc = "`write(|w| ..)` method takes [tcd2_attr::W](tcd2_attr::W) writer structure"]
impl crate::Writable for TCD2_ATTR {}
#[doc = "TCD Transfer Attributes"]
pub mod tcd2_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd2_nbytes_mlno](tcd2_nbytes_mlno) module"]
pub type TCD2_NBYTES_MLNO = crate::Reg<u32, _TCD2_NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD2_NBYTES_MLNO;
#[doc = "`read()` method returns [tcd2_nbytes_mlno::R](tcd2_nbytes_mlno::R) reader structure"]
impl crate::Readable for TCD2_NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [tcd2_nbytes_mlno::W](tcd2_nbytes_mlno::W) writer structure"]
impl crate::Writable for TCD2_NBYTES_MLNO {}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd2_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd2_nbytes_mloffno](tcd2_nbytes_mloffno) module"]
pub type TCD2_NBYTES_MLOFFNO = crate::Reg<u32, _TCD2_NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD2_NBYTES_MLOFFNO;
#[doc = "`read()` method returns [tcd2_nbytes_mloffno::R](tcd2_nbytes_mloffno::R) reader structure"]
impl crate::Readable for TCD2_NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [tcd2_nbytes_mloffno::W](tcd2_nbytes_mloffno::W) writer structure"]
impl crate::Writable for TCD2_NBYTES_MLOFFNO {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd2_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd2_nbytes_mloffyes](tcd2_nbytes_mloffyes) module"]
pub type TCD2_NBYTES_MLOFFYES = crate::Reg<u32, _TCD2_NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD2_NBYTES_MLOFFYES;
#[doc = "`read()` method returns [tcd2_nbytes_mloffyes::R](tcd2_nbytes_mloffyes::R) reader structure"]
impl crate::Readable for TCD2_NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [tcd2_nbytes_mloffyes::W](tcd2_nbytes_mloffyes::W) writer structure"]
impl crate::Writable for TCD2_NBYTES_MLOFFYES {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd2_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd2_slast](tcd2_slast) module"]
pub type TCD2_SLAST = crate::Reg<u32, _TCD2_SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD2_SLAST;
#[doc = "`read()` method returns [tcd2_slast::R](tcd2_slast::R) reader structure"]
impl crate::Readable for TCD2_SLAST {}
#[doc = "`write(|w| ..)` method takes [tcd2_slast::W](tcd2_slast::W) writer structure"]
impl crate::Writable for TCD2_SLAST {}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd2_slast;
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd2_daddr](tcd2_daddr) module"]
pub type TCD2_DADDR = crate::Reg<u32, _TCD2_DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD2_DADDR;
#[doc = "`read()` method returns [tcd2_daddr::R](tcd2_daddr::R) reader structure"]
impl crate::Readable for TCD2_DADDR {}
#[doc = "`write(|w| ..)` method takes [tcd2_daddr::W](tcd2_daddr::W) writer structure"]
impl crate::Writable for TCD2_DADDR {}
#[doc = "TCD Destination Address"]
pub mod tcd2_daddr;
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd2_doff](tcd2_doff) module"]
pub type TCD2_DOFF = crate::Reg<u16, _TCD2_DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD2_DOFF;
#[doc = "`read()` method returns [tcd2_doff::R](tcd2_doff::R) reader structure"]
impl crate::Readable for TCD2_DOFF {}
#[doc = "`write(|w| ..)` method takes [tcd2_doff::W](tcd2_doff::W) writer structure"]
impl crate::Writable for TCD2_DOFF {}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd2_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd2_citer_elinkno](tcd2_citer_elinkno) module"]
pub type TCD2_CITER_ELINKNO = crate::Reg<u16, _TCD2_CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD2_CITER_ELINKNO;
#[doc = "`read()` method returns [tcd2_citer_elinkno::R](tcd2_citer_elinkno::R) reader structure"]
impl crate::Readable for TCD2_CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd2_citer_elinkno::W](tcd2_citer_elinkno::W) writer structure"]
impl crate::Writable for TCD2_CITER_ELINKNO {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd2_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd2_citer_elinkyes](tcd2_citer_elinkyes) module"]
pub type TCD2_CITER_ELINKYES = crate::Reg<u16, _TCD2_CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD2_CITER_ELINKYES;
#[doc = "`read()` method returns [tcd2_citer_elinkyes::R](tcd2_citer_elinkyes::R) reader structure"]
impl crate::Readable for TCD2_CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd2_citer_elinkyes::W](tcd2_citer_elinkyes::W) writer structure"]
impl crate::Writable for TCD2_CITER_ELINKYES {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd2_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd2_dlastsga](tcd2_dlastsga) module"]
pub type TCD2_DLASTSGA = crate::Reg<u32, _TCD2_DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD2_DLASTSGA;
#[doc = "`read()` method returns [tcd2_dlastsga::R](tcd2_dlastsga::R) reader structure"]
impl crate::Readable for TCD2_DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [tcd2_dlastsga::W](tcd2_dlastsga::W) writer structure"]
impl crate::Writable for TCD2_DLASTSGA {}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd2_dlastsga;
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd2_csr](tcd2_csr) module"]
pub type TCD2_CSR = crate::Reg<u16, _TCD2_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD2_CSR;
#[doc = "`read()` method returns [tcd2_csr::R](tcd2_csr::R) reader structure"]
impl crate::Readable for TCD2_CSR {}
#[doc = "`write(|w| ..)` method takes [tcd2_csr::W](tcd2_csr::W) writer structure"]
impl crate::Writable for TCD2_CSR {}
#[doc = "TCD Control and Status"]
pub mod tcd2_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd2_biter_elinkno](tcd2_biter_elinkno) module"]
pub type TCD2_BITER_ELINKNO = crate::Reg<u16, _TCD2_BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD2_BITER_ELINKNO;
#[doc = "`read()` method returns [tcd2_biter_elinkno::R](tcd2_biter_elinkno::R) reader structure"]
impl crate::Readable for TCD2_BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd2_biter_elinkno::W](tcd2_biter_elinkno::W) writer structure"]
impl crate::Writable for TCD2_BITER_ELINKNO {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd2_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd2_biter_elinkyes](tcd2_biter_elinkyes) module"]
pub type TCD2_BITER_ELINKYES = crate::Reg<u16, _TCD2_BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD2_BITER_ELINKYES;
#[doc = "`read()` method returns [tcd2_biter_elinkyes::R](tcd2_biter_elinkyes::R) reader structure"]
impl crate::Readable for TCD2_BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd2_biter_elinkyes::W](tcd2_biter_elinkyes::W) writer structure"]
impl crate::Writable for TCD2_BITER_ELINKYES {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd2_biter_elinkyes;
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd3_saddr](tcd3_saddr) module"]
pub type TCD3_SADDR = crate::Reg<u32, _TCD3_SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD3_SADDR;
#[doc = "`read()` method returns [tcd3_saddr::R](tcd3_saddr::R) reader structure"]
impl crate::Readable for TCD3_SADDR {}
#[doc = "`write(|w| ..)` method takes [tcd3_saddr::W](tcd3_saddr::W) writer structure"]
impl crate::Writable for TCD3_SADDR {}
#[doc = "TCD Source Address"]
pub mod tcd3_saddr;
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd3_soff](tcd3_soff) module"]
pub type TCD3_SOFF = crate::Reg<u16, _TCD3_SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD3_SOFF;
#[doc = "`read()` method returns [tcd3_soff::R](tcd3_soff::R) reader structure"]
impl crate::Readable for TCD3_SOFF {}
#[doc = "`write(|w| ..)` method takes [tcd3_soff::W](tcd3_soff::W) writer structure"]
impl crate::Writable for TCD3_SOFF {}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd3_soff;
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd3_attr](tcd3_attr) module"]
pub type TCD3_ATTR = crate::Reg<u16, _TCD3_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD3_ATTR;
#[doc = "`read()` method returns [tcd3_attr::R](tcd3_attr::R) reader structure"]
impl crate::Readable for TCD3_ATTR {}
#[doc = "`write(|w| ..)` method takes [tcd3_attr::W](tcd3_attr::W) writer structure"]
impl crate::Writable for TCD3_ATTR {}
#[doc = "TCD Transfer Attributes"]
pub mod tcd3_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd3_nbytes_mlno](tcd3_nbytes_mlno) module"]
pub type TCD3_NBYTES_MLNO = crate::Reg<u32, _TCD3_NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD3_NBYTES_MLNO;
#[doc = "`read()` method returns [tcd3_nbytes_mlno::R](tcd3_nbytes_mlno::R) reader structure"]
impl crate::Readable for TCD3_NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [tcd3_nbytes_mlno::W](tcd3_nbytes_mlno::W) writer structure"]
impl crate::Writable for TCD3_NBYTES_MLNO {}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd3_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd3_nbytes_mloffno](tcd3_nbytes_mloffno) module"]
pub type TCD3_NBYTES_MLOFFNO = crate::Reg<u32, _TCD3_NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD3_NBYTES_MLOFFNO;
#[doc = "`read()` method returns [tcd3_nbytes_mloffno::R](tcd3_nbytes_mloffno::R) reader structure"]
impl crate::Readable for TCD3_NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [tcd3_nbytes_mloffno::W](tcd3_nbytes_mloffno::W) writer structure"]
impl crate::Writable for TCD3_NBYTES_MLOFFNO {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd3_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd3_nbytes_mloffyes](tcd3_nbytes_mloffyes) module"]
pub type TCD3_NBYTES_MLOFFYES = crate::Reg<u32, _TCD3_NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD3_NBYTES_MLOFFYES;
#[doc = "`read()` method returns [tcd3_nbytes_mloffyes::R](tcd3_nbytes_mloffyes::R) reader structure"]
impl crate::Readable for TCD3_NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [tcd3_nbytes_mloffyes::W](tcd3_nbytes_mloffyes::W) writer structure"]
impl crate::Writable for TCD3_NBYTES_MLOFFYES {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd3_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd3_slast](tcd3_slast) module"]
pub type TCD3_SLAST = crate::Reg<u32, _TCD3_SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD3_SLAST;
#[doc = "`read()` method returns [tcd3_slast::R](tcd3_slast::R) reader structure"]
impl crate::Readable for TCD3_SLAST {}
#[doc = "`write(|w| ..)` method takes [tcd3_slast::W](tcd3_slast::W) writer structure"]
impl crate::Writable for TCD3_SLAST {}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd3_slast;
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd3_daddr](tcd3_daddr) module"]
pub type TCD3_DADDR = crate::Reg<u32, _TCD3_DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD3_DADDR;
#[doc = "`read()` method returns [tcd3_daddr::R](tcd3_daddr::R) reader structure"]
impl crate::Readable for TCD3_DADDR {}
#[doc = "`write(|w| ..)` method takes [tcd3_daddr::W](tcd3_daddr::W) writer structure"]
impl crate::Writable for TCD3_DADDR {}
#[doc = "TCD Destination Address"]
pub mod tcd3_daddr;
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd3_doff](tcd3_doff) module"]
pub type TCD3_DOFF = crate::Reg<u16, _TCD3_DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD3_DOFF;
#[doc = "`read()` method returns [tcd3_doff::R](tcd3_doff::R) reader structure"]
impl crate::Readable for TCD3_DOFF {}
#[doc = "`write(|w| ..)` method takes [tcd3_doff::W](tcd3_doff::W) writer structure"]
impl crate::Writable for TCD3_DOFF {}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd3_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd3_citer_elinkno](tcd3_citer_elinkno) module"]
pub type TCD3_CITER_ELINKNO = crate::Reg<u16, _TCD3_CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD3_CITER_ELINKNO;
#[doc = "`read()` method returns [tcd3_citer_elinkno::R](tcd3_citer_elinkno::R) reader structure"]
impl crate::Readable for TCD3_CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd3_citer_elinkno::W](tcd3_citer_elinkno::W) writer structure"]
impl crate::Writable for TCD3_CITER_ELINKNO {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd3_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd3_citer_elinkyes](tcd3_citer_elinkyes) module"]
pub type TCD3_CITER_ELINKYES = crate::Reg<u16, _TCD3_CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD3_CITER_ELINKYES;
#[doc = "`read()` method returns [tcd3_citer_elinkyes::R](tcd3_citer_elinkyes::R) reader structure"]
impl crate::Readable for TCD3_CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd3_citer_elinkyes::W](tcd3_citer_elinkyes::W) writer structure"]
impl crate::Writable for TCD3_CITER_ELINKYES {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd3_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd3_dlastsga](tcd3_dlastsga) module"]
pub type TCD3_DLASTSGA = crate::Reg<u32, _TCD3_DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD3_DLASTSGA;
#[doc = "`read()` method returns [tcd3_dlastsga::R](tcd3_dlastsga::R) reader structure"]
impl crate::Readable for TCD3_DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [tcd3_dlastsga::W](tcd3_dlastsga::W) writer structure"]
impl crate::Writable for TCD3_DLASTSGA {}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd3_dlastsga;
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd3_csr](tcd3_csr) module"]
pub type TCD3_CSR = crate::Reg<u16, _TCD3_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD3_CSR;
#[doc = "`read()` method returns [tcd3_csr::R](tcd3_csr::R) reader structure"]
impl crate::Readable for TCD3_CSR {}
#[doc = "`write(|w| ..)` method takes [tcd3_csr::W](tcd3_csr::W) writer structure"]
impl crate::Writable for TCD3_CSR {}
#[doc = "TCD Control and Status"]
pub mod tcd3_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd3_biter_elinkno](tcd3_biter_elinkno) module"]
pub type TCD3_BITER_ELINKNO = crate::Reg<u16, _TCD3_BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD3_BITER_ELINKNO;
#[doc = "`read()` method returns [tcd3_biter_elinkno::R](tcd3_biter_elinkno::R) reader structure"]
impl crate::Readable for TCD3_BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd3_biter_elinkno::W](tcd3_biter_elinkno::W) writer structure"]
impl crate::Writable for TCD3_BITER_ELINKNO {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd3_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd3_biter_elinkyes](tcd3_biter_elinkyes) module"]
pub type TCD3_BITER_ELINKYES = crate::Reg<u16, _TCD3_BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD3_BITER_ELINKYES;
#[doc = "`read()` method returns [tcd3_biter_elinkyes::R](tcd3_biter_elinkyes::R) reader structure"]
impl crate::Readable for TCD3_BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd3_biter_elinkyes::W](tcd3_biter_elinkyes::W) writer structure"]
impl crate::Writable for TCD3_BITER_ELINKYES {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd3_biter_elinkyes;
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd4_saddr](tcd4_saddr) module"]
pub type TCD4_SADDR = crate::Reg<u32, _TCD4_SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD4_SADDR;
#[doc = "`read()` method returns [tcd4_saddr::R](tcd4_saddr::R) reader structure"]
impl crate::Readable for TCD4_SADDR {}
#[doc = "`write(|w| ..)` method takes [tcd4_saddr::W](tcd4_saddr::W) writer structure"]
impl crate::Writable for TCD4_SADDR {}
#[doc = "TCD Source Address"]
pub mod tcd4_saddr;
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd4_soff](tcd4_soff) module"]
pub type TCD4_SOFF = crate::Reg<u16, _TCD4_SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD4_SOFF;
#[doc = "`read()` method returns [tcd4_soff::R](tcd4_soff::R) reader structure"]
impl crate::Readable for TCD4_SOFF {}
#[doc = "`write(|w| ..)` method takes [tcd4_soff::W](tcd4_soff::W) writer structure"]
impl crate::Writable for TCD4_SOFF {}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd4_soff;
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd4_attr](tcd4_attr) module"]
pub type TCD4_ATTR = crate::Reg<u16, _TCD4_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD4_ATTR;
#[doc = "`read()` method returns [tcd4_attr::R](tcd4_attr::R) reader structure"]
impl crate::Readable for TCD4_ATTR {}
#[doc = "`write(|w| ..)` method takes [tcd4_attr::W](tcd4_attr::W) writer structure"]
impl crate::Writable for TCD4_ATTR {}
#[doc = "TCD Transfer Attributes"]
pub mod tcd4_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd4_nbytes_mlno](tcd4_nbytes_mlno) module"]
pub type TCD4_NBYTES_MLNO = crate::Reg<u32, _TCD4_NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD4_NBYTES_MLNO;
#[doc = "`read()` method returns [tcd4_nbytes_mlno::R](tcd4_nbytes_mlno::R) reader structure"]
impl crate::Readable for TCD4_NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [tcd4_nbytes_mlno::W](tcd4_nbytes_mlno::W) writer structure"]
impl crate::Writable for TCD4_NBYTES_MLNO {}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd4_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd4_nbytes_mloffno](tcd4_nbytes_mloffno) module"]
pub type TCD4_NBYTES_MLOFFNO = crate::Reg<u32, _TCD4_NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD4_NBYTES_MLOFFNO;
#[doc = "`read()` method returns [tcd4_nbytes_mloffno::R](tcd4_nbytes_mloffno::R) reader structure"]
impl crate::Readable for TCD4_NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [tcd4_nbytes_mloffno::W](tcd4_nbytes_mloffno::W) writer structure"]
impl crate::Writable for TCD4_NBYTES_MLOFFNO {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd4_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd4_nbytes_mloffyes](tcd4_nbytes_mloffyes) module"]
pub type TCD4_NBYTES_MLOFFYES = crate::Reg<u32, _TCD4_NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD4_NBYTES_MLOFFYES;
#[doc = "`read()` method returns [tcd4_nbytes_mloffyes::R](tcd4_nbytes_mloffyes::R) reader structure"]
impl crate::Readable for TCD4_NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [tcd4_nbytes_mloffyes::W](tcd4_nbytes_mloffyes::W) writer structure"]
impl crate::Writable for TCD4_NBYTES_MLOFFYES {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd4_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd4_slast](tcd4_slast) module"]
pub type TCD4_SLAST = crate::Reg<u32, _TCD4_SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD4_SLAST;
#[doc = "`read()` method returns [tcd4_slast::R](tcd4_slast::R) reader structure"]
impl crate::Readable for TCD4_SLAST {}
#[doc = "`write(|w| ..)` method takes [tcd4_slast::W](tcd4_slast::W) writer structure"]
impl crate::Writable for TCD4_SLAST {}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd4_slast;
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd4_daddr](tcd4_daddr) module"]
pub type TCD4_DADDR = crate::Reg<u32, _TCD4_DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD4_DADDR;
#[doc = "`read()` method returns [tcd4_daddr::R](tcd4_daddr::R) reader structure"]
impl crate::Readable for TCD4_DADDR {}
#[doc = "`write(|w| ..)` method takes [tcd4_daddr::W](tcd4_daddr::W) writer structure"]
impl crate::Writable for TCD4_DADDR {}
#[doc = "TCD Destination Address"]
pub mod tcd4_daddr;
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd4_doff](tcd4_doff) module"]
pub type TCD4_DOFF = crate::Reg<u16, _TCD4_DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD4_DOFF;
#[doc = "`read()` method returns [tcd4_doff::R](tcd4_doff::R) reader structure"]
impl crate::Readable for TCD4_DOFF {}
#[doc = "`write(|w| ..)` method takes [tcd4_doff::W](tcd4_doff::W) writer structure"]
impl crate::Writable for TCD4_DOFF {}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd4_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd4_citer_elinkno](tcd4_citer_elinkno) module"]
pub type TCD4_CITER_ELINKNO = crate::Reg<u16, _TCD4_CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD4_CITER_ELINKNO;
#[doc = "`read()` method returns [tcd4_citer_elinkno::R](tcd4_citer_elinkno::R) reader structure"]
impl crate::Readable for TCD4_CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd4_citer_elinkno::W](tcd4_citer_elinkno::W) writer structure"]
impl crate::Writable for TCD4_CITER_ELINKNO {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd4_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd4_citer_elinkyes](tcd4_citer_elinkyes) module"]
pub type TCD4_CITER_ELINKYES = crate::Reg<u16, _TCD4_CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD4_CITER_ELINKYES;
#[doc = "`read()` method returns [tcd4_citer_elinkyes::R](tcd4_citer_elinkyes::R) reader structure"]
impl crate::Readable for TCD4_CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd4_citer_elinkyes::W](tcd4_citer_elinkyes::W) writer structure"]
impl crate::Writable for TCD4_CITER_ELINKYES {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd4_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd4_dlastsga](tcd4_dlastsga) module"]
pub type TCD4_DLASTSGA = crate::Reg<u32, _TCD4_DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD4_DLASTSGA;
#[doc = "`read()` method returns [tcd4_dlastsga::R](tcd4_dlastsga::R) reader structure"]
impl crate::Readable for TCD4_DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [tcd4_dlastsga::W](tcd4_dlastsga::W) writer structure"]
impl crate::Writable for TCD4_DLASTSGA {}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd4_dlastsga;
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd4_csr](tcd4_csr) module"]
pub type TCD4_CSR = crate::Reg<u16, _TCD4_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD4_CSR;
#[doc = "`read()` method returns [tcd4_csr::R](tcd4_csr::R) reader structure"]
impl crate::Readable for TCD4_CSR {}
#[doc = "`write(|w| ..)` method takes [tcd4_csr::W](tcd4_csr::W) writer structure"]
impl crate::Writable for TCD4_CSR {}
#[doc = "TCD Control and Status"]
pub mod tcd4_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd4_biter_elinkno](tcd4_biter_elinkno) module"]
pub type TCD4_BITER_ELINKNO = crate::Reg<u16, _TCD4_BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD4_BITER_ELINKNO;
#[doc = "`read()` method returns [tcd4_biter_elinkno::R](tcd4_biter_elinkno::R) reader structure"]
impl crate::Readable for TCD4_BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd4_biter_elinkno::W](tcd4_biter_elinkno::W) writer structure"]
impl crate::Writable for TCD4_BITER_ELINKNO {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd4_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd4_biter_elinkyes](tcd4_biter_elinkyes) module"]
pub type TCD4_BITER_ELINKYES = crate::Reg<u16, _TCD4_BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD4_BITER_ELINKYES;
#[doc = "`read()` method returns [tcd4_biter_elinkyes::R](tcd4_biter_elinkyes::R) reader structure"]
impl crate::Readable for TCD4_BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd4_biter_elinkyes::W](tcd4_biter_elinkyes::W) writer structure"]
impl crate::Writable for TCD4_BITER_ELINKYES {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd4_biter_elinkyes;
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd5_saddr](tcd5_saddr) module"]
pub type TCD5_SADDR = crate::Reg<u32, _TCD5_SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD5_SADDR;
#[doc = "`read()` method returns [tcd5_saddr::R](tcd5_saddr::R) reader structure"]
impl crate::Readable for TCD5_SADDR {}
#[doc = "`write(|w| ..)` method takes [tcd5_saddr::W](tcd5_saddr::W) writer structure"]
impl crate::Writable for TCD5_SADDR {}
#[doc = "TCD Source Address"]
pub mod tcd5_saddr;
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd5_soff](tcd5_soff) module"]
pub type TCD5_SOFF = crate::Reg<u16, _TCD5_SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD5_SOFF;
#[doc = "`read()` method returns [tcd5_soff::R](tcd5_soff::R) reader structure"]
impl crate::Readable for TCD5_SOFF {}
#[doc = "`write(|w| ..)` method takes [tcd5_soff::W](tcd5_soff::W) writer structure"]
impl crate::Writable for TCD5_SOFF {}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd5_soff;
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd5_attr](tcd5_attr) module"]
pub type TCD5_ATTR = crate::Reg<u16, _TCD5_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD5_ATTR;
#[doc = "`read()` method returns [tcd5_attr::R](tcd5_attr::R) reader structure"]
impl crate::Readable for TCD5_ATTR {}
#[doc = "`write(|w| ..)` method takes [tcd5_attr::W](tcd5_attr::W) writer structure"]
impl crate::Writable for TCD5_ATTR {}
#[doc = "TCD Transfer Attributes"]
pub mod tcd5_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd5_nbytes_mlno](tcd5_nbytes_mlno) module"]
pub type TCD5_NBYTES_MLNO = crate::Reg<u32, _TCD5_NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD5_NBYTES_MLNO;
#[doc = "`read()` method returns [tcd5_nbytes_mlno::R](tcd5_nbytes_mlno::R) reader structure"]
impl crate::Readable for TCD5_NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [tcd5_nbytes_mlno::W](tcd5_nbytes_mlno::W) writer structure"]
impl crate::Writable for TCD5_NBYTES_MLNO {}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd5_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd5_nbytes_mloffno](tcd5_nbytes_mloffno) module"]
pub type TCD5_NBYTES_MLOFFNO = crate::Reg<u32, _TCD5_NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD5_NBYTES_MLOFFNO;
#[doc = "`read()` method returns [tcd5_nbytes_mloffno::R](tcd5_nbytes_mloffno::R) reader structure"]
impl crate::Readable for TCD5_NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [tcd5_nbytes_mloffno::W](tcd5_nbytes_mloffno::W) writer structure"]
impl crate::Writable for TCD5_NBYTES_MLOFFNO {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd5_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd5_nbytes_mloffyes](tcd5_nbytes_mloffyes) module"]
pub type TCD5_NBYTES_MLOFFYES = crate::Reg<u32, _TCD5_NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD5_NBYTES_MLOFFYES;
#[doc = "`read()` method returns [tcd5_nbytes_mloffyes::R](tcd5_nbytes_mloffyes::R) reader structure"]
impl crate::Readable for TCD5_NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [tcd5_nbytes_mloffyes::W](tcd5_nbytes_mloffyes::W) writer structure"]
impl crate::Writable for TCD5_NBYTES_MLOFFYES {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd5_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd5_slast](tcd5_slast) module"]
pub type TCD5_SLAST = crate::Reg<u32, _TCD5_SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD5_SLAST;
#[doc = "`read()` method returns [tcd5_slast::R](tcd5_slast::R) reader structure"]
impl crate::Readable for TCD5_SLAST {}
#[doc = "`write(|w| ..)` method takes [tcd5_slast::W](tcd5_slast::W) writer structure"]
impl crate::Writable for TCD5_SLAST {}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd5_slast;
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd5_daddr](tcd5_daddr) module"]
pub type TCD5_DADDR = crate::Reg<u32, _TCD5_DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD5_DADDR;
#[doc = "`read()` method returns [tcd5_daddr::R](tcd5_daddr::R) reader structure"]
impl crate::Readable for TCD5_DADDR {}
#[doc = "`write(|w| ..)` method takes [tcd5_daddr::W](tcd5_daddr::W) writer structure"]
impl crate::Writable for TCD5_DADDR {}
#[doc = "TCD Destination Address"]
pub mod tcd5_daddr;
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd5_doff](tcd5_doff) module"]
pub type TCD5_DOFF = crate::Reg<u16, _TCD5_DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD5_DOFF;
#[doc = "`read()` method returns [tcd5_doff::R](tcd5_doff::R) reader structure"]
impl crate::Readable for TCD5_DOFF {}
#[doc = "`write(|w| ..)` method takes [tcd5_doff::W](tcd5_doff::W) writer structure"]
impl crate::Writable for TCD5_DOFF {}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd5_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd5_citer_elinkno](tcd5_citer_elinkno) module"]
pub type TCD5_CITER_ELINKNO = crate::Reg<u16, _TCD5_CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD5_CITER_ELINKNO;
#[doc = "`read()` method returns [tcd5_citer_elinkno::R](tcd5_citer_elinkno::R) reader structure"]
impl crate::Readable for TCD5_CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd5_citer_elinkno::W](tcd5_citer_elinkno::W) writer structure"]
impl crate::Writable for TCD5_CITER_ELINKNO {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd5_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd5_citer_elinkyes](tcd5_citer_elinkyes) module"]
pub type TCD5_CITER_ELINKYES = crate::Reg<u16, _TCD5_CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD5_CITER_ELINKYES;
#[doc = "`read()` method returns [tcd5_citer_elinkyes::R](tcd5_citer_elinkyes::R) reader structure"]
impl crate::Readable for TCD5_CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd5_citer_elinkyes::W](tcd5_citer_elinkyes::W) writer structure"]
impl crate::Writable for TCD5_CITER_ELINKYES {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd5_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd5_dlastsga](tcd5_dlastsga) module"]
pub type TCD5_DLASTSGA = crate::Reg<u32, _TCD5_DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD5_DLASTSGA;
#[doc = "`read()` method returns [tcd5_dlastsga::R](tcd5_dlastsga::R) reader structure"]
impl crate::Readable for TCD5_DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [tcd5_dlastsga::W](tcd5_dlastsga::W) writer structure"]
impl crate::Writable for TCD5_DLASTSGA {}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd5_dlastsga;
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd5_csr](tcd5_csr) module"]
pub type TCD5_CSR = crate::Reg<u16, _TCD5_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD5_CSR;
#[doc = "`read()` method returns [tcd5_csr::R](tcd5_csr::R) reader structure"]
impl crate::Readable for TCD5_CSR {}
#[doc = "`write(|w| ..)` method takes [tcd5_csr::W](tcd5_csr::W) writer structure"]
impl crate::Writable for TCD5_CSR {}
#[doc = "TCD Control and Status"]
pub mod tcd5_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd5_biter_elinkno](tcd5_biter_elinkno) module"]
pub type TCD5_BITER_ELINKNO = crate::Reg<u16, _TCD5_BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD5_BITER_ELINKNO;
#[doc = "`read()` method returns [tcd5_biter_elinkno::R](tcd5_biter_elinkno::R) reader structure"]
impl crate::Readable for TCD5_BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd5_biter_elinkno::W](tcd5_biter_elinkno::W) writer structure"]
impl crate::Writable for TCD5_BITER_ELINKNO {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd5_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd5_biter_elinkyes](tcd5_biter_elinkyes) module"]
pub type TCD5_BITER_ELINKYES = crate::Reg<u16, _TCD5_BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD5_BITER_ELINKYES;
#[doc = "`read()` method returns [tcd5_biter_elinkyes::R](tcd5_biter_elinkyes::R) reader structure"]
impl crate::Readable for TCD5_BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd5_biter_elinkyes::W](tcd5_biter_elinkyes::W) writer structure"]
impl crate::Writable for TCD5_BITER_ELINKYES {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd5_biter_elinkyes;
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd6_saddr](tcd6_saddr) module"]
pub type TCD6_SADDR = crate::Reg<u32, _TCD6_SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD6_SADDR;
#[doc = "`read()` method returns [tcd6_saddr::R](tcd6_saddr::R) reader structure"]
impl crate::Readable for TCD6_SADDR {}
#[doc = "`write(|w| ..)` method takes [tcd6_saddr::W](tcd6_saddr::W) writer structure"]
impl crate::Writable for TCD6_SADDR {}
#[doc = "TCD Source Address"]
pub mod tcd6_saddr;
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd6_soff](tcd6_soff) module"]
pub type TCD6_SOFF = crate::Reg<u16, _TCD6_SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD6_SOFF;
#[doc = "`read()` method returns [tcd6_soff::R](tcd6_soff::R) reader structure"]
impl crate::Readable for TCD6_SOFF {}
#[doc = "`write(|w| ..)` method takes [tcd6_soff::W](tcd6_soff::W) writer structure"]
impl crate::Writable for TCD6_SOFF {}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd6_soff;
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd6_attr](tcd6_attr) module"]
pub type TCD6_ATTR = crate::Reg<u16, _TCD6_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD6_ATTR;
#[doc = "`read()` method returns [tcd6_attr::R](tcd6_attr::R) reader structure"]
impl crate::Readable for TCD6_ATTR {}
#[doc = "`write(|w| ..)` method takes [tcd6_attr::W](tcd6_attr::W) writer structure"]
impl crate::Writable for TCD6_ATTR {}
#[doc = "TCD Transfer Attributes"]
pub mod tcd6_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd6_nbytes_mlno](tcd6_nbytes_mlno) module"]
pub type TCD6_NBYTES_MLNO = crate::Reg<u32, _TCD6_NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD6_NBYTES_MLNO;
#[doc = "`read()` method returns [tcd6_nbytes_mlno::R](tcd6_nbytes_mlno::R) reader structure"]
impl crate::Readable for TCD6_NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [tcd6_nbytes_mlno::W](tcd6_nbytes_mlno::W) writer structure"]
impl crate::Writable for TCD6_NBYTES_MLNO {}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd6_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd6_nbytes_mloffno](tcd6_nbytes_mloffno) module"]
pub type TCD6_NBYTES_MLOFFNO = crate::Reg<u32, _TCD6_NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD6_NBYTES_MLOFFNO;
#[doc = "`read()` method returns [tcd6_nbytes_mloffno::R](tcd6_nbytes_mloffno::R) reader structure"]
impl crate::Readable for TCD6_NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [tcd6_nbytes_mloffno::W](tcd6_nbytes_mloffno::W) writer structure"]
impl crate::Writable for TCD6_NBYTES_MLOFFNO {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd6_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd6_nbytes_mloffyes](tcd6_nbytes_mloffyes) module"]
pub type TCD6_NBYTES_MLOFFYES = crate::Reg<u32, _TCD6_NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD6_NBYTES_MLOFFYES;
#[doc = "`read()` method returns [tcd6_nbytes_mloffyes::R](tcd6_nbytes_mloffyes::R) reader structure"]
impl crate::Readable for TCD6_NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [tcd6_nbytes_mloffyes::W](tcd6_nbytes_mloffyes::W) writer structure"]
impl crate::Writable for TCD6_NBYTES_MLOFFYES {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd6_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd6_slast](tcd6_slast) module"]
pub type TCD6_SLAST = crate::Reg<u32, _TCD6_SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD6_SLAST;
#[doc = "`read()` method returns [tcd6_slast::R](tcd6_slast::R) reader structure"]
impl crate::Readable for TCD6_SLAST {}
#[doc = "`write(|w| ..)` method takes [tcd6_slast::W](tcd6_slast::W) writer structure"]
impl crate::Writable for TCD6_SLAST {}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd6_slast;
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd6_daddr](tcd6_daddr) module"]
pub type TCD6_DADDR = crate::Reg<u32, _TCD6_DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD6_DADDR;
#[doc = "`read()` method returns [tcd6_daddr::R](tcd6_daddr::R) reader structure"]
impl crate::Readable for TCD6_DADDR {}
#[doc = "`write(|w| ..)` method takes [tcd6_daddr::W](tcd6_daddr::W) writer structure"]
impl crate::Writable for TCD6_DADDR {}
#[doc = "TCD Destination Address"]
pub mod tcd6_daddr;
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd6_doff](tcd6_doff) module"]
pub type TCD6_DOFF = crate::Reg<u16, _TCD6_DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD6_DOFF;
#[doc = "`read()` method returns [tcd6_doff::R](tcd6_doff::R) reader structure"]
impl crate::Readable for TCD6_DOFF {}
#[doc = "`write(|w| ..)` method takes [tcd6_doff::W](tcd6_doff::W) writer structure"]
impl crate::Writable for TCD6_DOFF {}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd6_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd6_citer_elinkno](tcd6_citer_elinkno) module"]
pub type TCD6_CITER_ELINKNO = crate::Reg<u16, _TCD6_CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD6_CITER_ELINKNO;
#[doc = "`read()` method returns [tcd6_citer_elinkno::R](tcd6_citer_elinkno::R) reader structure"]
impl crate::Readable for TCD6_CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd6_citer_elinkno::W](tcd6_citer_elinkno::W) writer structure"]
impl crate::Writable for TCD6_CITER_ELINKNO {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd6_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd6_citer_elinkyes](tcd6_citer_elinkyes) module"]
pub type TCD6_CITER_ELINKYES = crate::Reg<u16, _TCD6_CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD6_CITER_ELINKYES;
#[doc = "`read()` method returns [tcd6_citer_elinkyes::R](tcd6_citer_elinkyes::R) reader structure"]
impl crate::Readable for TCD6_CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd6_citer_elinkyes::W](tcd6_citer_elinkyes::W) writer structure"]
impl crate::Writable for TCD6_CITER_ELINKYES {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd6_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd6_dlastsga](tcd6_dlastsga) module"]
pub type TCD6_DLASTSGA = crate::Reg<u32, _TCD6_DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD6_DLASTSGA;
#[doc = "`read()` method returns [tcd6_dlastsga::R](tcd6_dlastsga::R) reader structure"]
impl crate::Readable for TCD6_DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [tcd6_dlastsga::W](tcd6_dlastsga::W) writer structure"]
impl crate::Writable for TCD6_DLASTSGA {}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd6_dlastsga;
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd6_csr](tcd6_csr) module"]
pub type TCD6_CSR = crate::Reg<u16, _TCD6_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD6_CSR;
#[doc = "`read()` method returns [tcd6_csr::R](tcd6_csr::R) reader structure"]
impl crate::Readable for TCD6_CSR {}
#[doc = "`write(|w| ..)` method takes [tcd6_csr::W](tcd6_csr::W) writer structure"]
impl crate::Writable for TCD6_CSR {}
#[doc = "TCD Control and Status"]
pub mod tcd6_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd6_biter_elinkno](tcd6_biter_elinkno) module"]
pub type TCD6_BITER_ELINKNO = crate::Reg<u16, _TCD6_BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD6_BITER_ELINKNO;
#[doc = "`read()` method returns [tcd6_biter_elinkno::R](tcd6_biter_elinkno::R) reader structure"]
impl crate::Readable for TCD6_BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd6_biter_elinkno::W](tcd6_biter_elinkno::W) writer structure"]
impl crate::Writable for TCD6_BITER_ELINKNO {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd6_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd6_biter_elinkyes](tcd6_biter_elinkyes) module"]
pub type TCD6_BITER_ELINKYES = crate::Reg<u16, _TCD6_BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD6_BITER_ELINKYES;
#[doc = "`read()` method returns [tcd6_biter_elinkyes::R](tcd6_biter_elinkyes::R) reader structure"]
impl crate::Readable for TCD6_BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd6_biter_elinkyes::W](tcd6_biter_elinkyes::W) writer structure"]
impl crate::Writable for TCD6_BITER_ELINKYES {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd6_biter_elinkyes;
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd7_saddr](tcd7_saddr) module"]
pub type TCD7_SADDR = crate::Reg<u32, _TCD7_SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD7_SADDR;
#[doc = "`read()` method returns [tcd7_saddr::R](tcd7_saddr::R) reader structure"]
impl crate::Readable for TCD7_SADDR {}
#[doc = "`write(|w| ..)` method takes [tcd7_saddr::W](tcd7_saddr::W) writer structure"]
impl crate::Writable for TCD7_SADDR {}
#[doc = "TCD Source Address"]
pub mod tcd7_saddr;
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd7_soff](tcd7_soff) module"]
pub type TCD7_SOFF = crate::Reg<u16, _TCD7_SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD7_SOFF;
#[doc = "`read()` method returns [tcd7_soff::R](tcd7_soff::R) reader structure"]
impl crate::Readable for TCD7_SOFF {}
#[doc = "`write(|w| ..)` method takes [tcd7_soff::W](tcd7_soff::W) writer structure"]
impl crate::Writable for TCD7_SOFF {}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd7_soff;
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd7_attr](tcd7_attr) module"]
pub type TCD7_ATTR = crate::Reg<u16, _TCD7_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD7_ATTR;
#[doc = "`read()` method returns [tcd7_attr::R](tcd7_attr::R) reader structure"]
impl crate::Readable for TCD7_ATTR {}
#[doc = "`write(|w| ..)` method takes [tcd7_attr::W](tcd7_attr::W) writer structure"]
impl crate::Writable for TCD7_ATTR {}
#[doc = "TCD Transfer Attributes"]
pub mod tcd7_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd7_nbytes_mlno](tcd7_nbytes_mlno) module"]
pub type TCD7_NBYTES_MLNO = crate::Reg<u32, _TCD7_NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD7_NBYTES_MLNO;
#[doc = "`read()` method returns [tcd7_nbytes_mlno::R](tcd7_nbytes_mlno::R) reader structure"]
impl crate::Readable for TCD7_NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [tcd7_nbytes_mlno::W](tcd7_nbytes_mlno::W) writer structure"]
impl crate::Writable for TCD7_NBYTES_MLNO {}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd7_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd7_nbytes_mloffno](tcd7_nbytes_mloffno) module"]
pub type TCD7_NBYTES_MLOFFNO = crate::Reg<u32, _TCD7_NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD7_NBYTES_MLOFFNO;
#[doc = "`read()` method returns [tcd7_nbytes_mloffno::R](tcd7_nbytes_mloffno::R) reader structure"]
impl crate::Readable for TCD7_NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [tcd7_nbytes_mloffno::W](tcd7_nbytes_mloffno::W) writer structure"]
impl crate::Writable for TCD7_NBYTES_MLOFFNO {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd7_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd7_nbytes_mloffyes](tcd7_nbytes_mloffyes) module"]
pub type TCD7_NBYTES_MLOFFYES = crate::Reg<u32, _TCD7_NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD7_NBYTES_MLOFFYES;
#[doc = "`read()` method returns [tcd7_nbytes_mloffyes::R](tcd7_nbytes_mloffyes::R) reader structure"]
impl crate::Readable for TCD7_NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [tcd7_nbytes_mloffyes::W](tcd7_nbytes_mloffyes::W) writer structure"]
impl crate::Writable for TCD7_NBYTES_MLOFFYES {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd7_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd7_slast](tcd7_slast) module"]
pub type TCD7_SLAST = crate::Reg<u32, _TCD7_SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD7_SLAST;
#[doc = "`read()` method returns [tcd7_slast::R](tcd7_slast::R) reader structure"]
impl crate::Readable for TCD7_SLAST {}
#[doc = "`write(|w| ..)` method takes [tcd7_slast::W](tcd7_slast::W) writer structure"]
impl crate::Writable for TCD7_SLAST {}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd7_slast;
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd7_daddr](tcd7_daddr) module"]
pub type TCD7_DADDR = crate::Reg<u32, _TCD7_DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD7_DADDR;
#[doc = "`read()` method returns [tcd7_daddr::R](tcd7_daddr::R) reader structure"]
impl crate::Readable for TCD7_DADDR {}
#[doc = "`write(|w| ..)` method takes [tcd7_daddr::W](tcd7_daddr::W) writer structure"]
impl crate::Writable for TCD7_DADDR {}
#[doc = "TCD Destination Address"]
pub mod tcd7_daddr;
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd7_doff](tcd7_doff) module"]
pub type TCD7_DOFF = crate::Reg<u16, _TCD7_DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD7_DOFF;
#[doc = "`read()` method returns [tcd7_doff::R](tcd7_doff::R) reader structure"]
impl crate::Readable for TCD7_DOFF {}
#[doc = "`write(|w| ..)` method takes [tcd7_doff::W](tcd7_doff::W) writer structure"]
impl crate::Writable for TCD7_DOFF {}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd7_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd7_citer_elinkno](tcd7_citer_elinkno) module"]
pub type TCD7_CITER_ELINKNO = crate::Reg<u16, _TCD7_CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD7_CITER_ELINKNO;
#[doc = "`read()` method returns [tcd7_citer_elinkno::R](tcd7_citer_elinkno::R) reader structure"]
impl crate::Readable for TCD7_CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd7_citer_elinkno::W](tcd7_citer_elinkno::W) writer structure"]
impl crate::Writable for TCD7_CITER_ELINKNO {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd7_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd7_citer_elinkyes](tcd7_citer_elinkyes) module"]
pub type TCD7_CITER_ELINKYES = crate::Reg<u16, _TCD7_CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD7_CITER_ELINKYES;
#[doc = "`read()` method returns [tcd7_citer_elinkyes::R](tcd7_citer_elinkyes::R) reader structure"]
impl crate::Readable for TCD7_CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd7_citer_elinkyes::W](tcd7_citer_elinkyes::W) writer structure"]
impl crate::Writable for TCD7_CITER_ELINKYES {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd7_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd7_dlastsga](tcd7_dlastsga) module"]
pub type TCD7_DLASTSGA = crate::Reg<u32, _TCD7_DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD7_DLASTSGA;
#[doc = "`read()` method returns [tcd7_dlastsga::R](tcd7_dlastsga::R) reader structure"]
impl crate::Readable for TCD7_DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [tcd7_dlastsga::W](tcd7_dlastsga::W) writer structure"]
impl crate::Writable for TCD7_DLASTSGA {}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd7_dlastsga;
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd7_csr](tcd7_csr) module"]
pub type TCD7_CSR = crate::Reg<u16, _TCD7_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD7_CSR;
#[doc = "`read()` method returns [tcd7_csr::R](tcd7_csr::R) reader structure"]
impl crate::Readable for TCD7_CSR {}
#[doc = "`write(|w| ..)` method takes [tcd7_csr::W](tcd7_csr::W) writer structure"]
impl crate::Writable for TCD7_CSR {}
#[doc = "TCD Control and Status"]
pub mod tcd7_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd7_biter_elinkno](tcd7_biter_elinkno) module"]
pub type TCD7_BITER_ELINKNO = crate::Reg<u16, _TCD7_BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD7_BITER_ELINKNO;
#[doc = "`read()` method returns [tcd7_biter_elinkno::R](tcd7_biter_elinkno::R) reader structure"]
impl crate::Readable for TCD7_BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd7_biter_elinkno::W](tcd7_biter_elinkno::W) writer structure"]
impl crate::Writable for TCD7_BITER_ELINKNO {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd7_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd7_biter_elinkyes](tcd7_biter_elinkyes) module"]
pub type TCD7_BITER_ELINKYES = crate::Reg<u16, _TCD7_BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD7_BITER_ELINKYES;
#[doc = "`read()` method returns [tcd7_biter_elinkyes::R](tcd7_biter_elinkyes::R) reader structure"]
impl crate::Readable for TCD7_BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd7_biter_elinkyes::W](tcd7_biter_elinkyes::W) writer structure"]
impl crate::Writable for TCD7_BITER_ELINKYES {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd7_biter_elinkyes;
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd8_saddr](tcd8_saddr) module"]
pub type TCD8_SADDR = crate::Reg<u32, _TCD8_SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD8_SADDR;
#[doc = "`read()` method returns [tcd8_saddr::R](tcd8_saddr::R) reader structure"]
impl crate::Readable for TCD8_SADDR {}
#[doc = "`write(|w| ..)` method takes [tcd8_saddr::W](tcd8_saddr::W) writer structure"]
impl crate::Writable for TCD8_SADDR {}
#[doc = "TCD Source Address"]
pub mod tcd8_saddr;
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd8_soff](tcd8_soff) module"]
pub type TCD8_SOFF = crate::Reg<u16, _TCD8_SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD8_SOFF;
#[doc = "`read()` method returns [tcd8_soff::R](tcd8_soff::R) reader structure"]
impl crate::Readable for TCD8_SOFF {}
#[doc = "`write(|w| ..)` method takes [tcd8_soff::W](tcd8_soff::W) writer structure"]
impl crate::Writable for TCD8_SOFF {}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd8_soff;
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd8_attr](tcd8_attr) module"]
pub type TCD8_ATTR = crate::Reg<u16, _TCD8_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD8_ATTR;
#[doc = "`read()` method returns [tcd8_attr::R](tcd8_attr::R) reader structure"]
impl crate::Readable for TCD8_ATTR {}
#[doc = "`write(|w| ..)` method takes [tcd8_attr::W](tcd8_attr::W) writer structure"]
impl crate::Writable for TCD8_ATTR {}
#[doc = "TCD Transfer Attributes"]
pub mod tcd8_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd8_nbytes_mlno](tcd8_nbytes_mlno) module"]
pub type TCD8_NBYTES_MLNO = crate::Reg<u32, _TCD8_NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD8_NBYTES_MLNO;
#[doc = "`read()` method returns [tcd8_nbytes_mlno::R](tcd8_nbytes_mlno::R) reader structure"]
impl crate::Readable for TCD8_NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [tcd8_nbytes_mlno::W](tcd8_nbytes_mlno::W) writer structure"]
impl crate::Writable for TCD8_NBYTES_MLNO {}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd8_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd8_nbytes_mloffno](tcd8_nbytes_mloffno) module"]
pub type TCD8_NBYTES_MLOFFNO = crate::Reg<u32, _TCD8_NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD8_NBYTES_MLOFFNO;
#[doc = "`read()` method returns [tcd8_nbytes_mloffno::R](tcd8_nbytes_mloffno::R) reader structure"]
impl crate::Readable for TCD8_NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [tcd8_nbytes_mloffno::W](tcd8_nbytes_mloffno::W) writer structure"]
impl crate::Writable for TCD8_NBYTES_MLOFFNO {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd8_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd8_nbytes_mloffyes](tcd8_nbytes_mloffyes) module"]
pub type TCD8_NBYTES_MLOFFYES = crate::Reg<u32, _TCD8_NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD8_NBYTES_MLOFFYES;
#[doc = "`read()` method returns [tcd8_nbytes_mloffyes::R](tcd8_nbytes_mloffyes::R) reader structure"]
impl crate::Readable for TCD8_NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [tcd8_nbytes_mloffyes::W](tcd8_nbytes_mloffyes::W) writer structure"]
impl crate::Writable for TCD8_NBYTES_MLOFFYES {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd8_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd8_slast](tcd8_slast) module"]
pub type TCD8_SLAST = crate::Reg<u32, _TCD8_SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD8_SLAST;
#[doc = "`read()` method returns [tcd8_slast::R](tcd8_slast::R) reader structure"]
impl crate::Readable for TCD8_SLAST {}
#[doc = "`write(|w| ..)` method takes [tcd8_slast::W](tcd8_slast::W) writer structure"]
impl crate::Writable for TCD8_SLAST {}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd8_slast;
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd8_daddr](tcd8_daddr) module"]
pub type TCD8_DADDR = crate::Reg<u32, _TCD8_DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD8_DADDR;
#[doc = "`read()` method returns [tcd8_daddr::R](tcd8_daddr::R) reader structure"]
impl crate::Readable for TCD8_DADDR {}
#[doc = "`write(|w| ..)` method takes [tcd8_daddr::W](tcd8_daddr::W) writer structure"]
impl crate::Writable for TCD8_DADDR {}
#[doc = "TCD Destination Address"]
pub mod tcd8_daddr;
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd8_doff](tcd8_doff) module"]
pub type TCD8_DOFF = crate::Reg<u16, _TCD8_DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD8_DOFF;
#[doc = "`read()` method returns [tcd8_doff::R](tcd8_doff::R) reader structure"]
impl crate::Readable for TCD8_DOFF {}
#[doc = "`write(|w| ..)` method takes [tcd8_doff::W](tcd8_doff::W) writer structure"]
impl crate::Writable for TCD8_DOFF {}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd8_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd8_citer_elinkno](tcd8_citer_elinkno) module"]
pub type TCD8_CITER_ELINKNO = crate::Reg<u16, _TCD8_CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD8_CITER_ELINKNO;
#[doc = "`read()` method returns [tcd8_citer_elinkno::R](tcd8_citer_elinkno::R) reader structure"]
impl crate::Readable for TCD8_CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd8_citer_elinkno::W](tcd8_citer_elinkno::W) writer structure"]
impl crate::Writable for TCD8_CITER_ELINKNO {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd8_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd8_citer_elinkyes](tcd8_citer_elinkyes) module"]
pub type TCD8_CITER_ELINKYES = crate::Reg<u16, _TCD8_CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD8_CITER_ELINKYES;
#[doc = "`read()` method returns [tcd8_citer_elinkyes::R](tcd8_citer_elinkyes::R) reader structure"]
impl crate::Readable for TCD8_CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd8_citer_elinkyes::W](tcd8_citer_elinkyes::W) writer structure"]
impl crate::Writable for TCD8_CITER_ELINKYES {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd8_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd8_dlastsga](tcd8_dlastsga) module"]
pub type TCD8_DLASTSGA = crate::Reg<u32, _TCD8_DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD8_DLASTSGA;
#[doc = "`read()` method returns [tcd8_dlastsga::R](tcd8_dlastsga::R) reader structure"]
impl crate::Readable for TCD8_DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [tcd8_dlastsga::W](tcd8_dlastsga::W) writer structure"]
impl crate::Writable for TCD8_DLASTSGA {}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd8_dlastsga;
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd8_csr](tcd8_csr) module"]
pub type TCD8_CSR = crate::Reg<u16, _TCD8_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD8_CSR;
#[doc = "`read()` method returns [tcd8_csr::R](tcd8_csr::R) reader structure"]
impl crate::Readable for TCD8_CSR {}
#[doc = "`write(|w| ..)` method takes [tcd8_csr::W](tcd8_csr::W) writer structure"]
impl crate::Writable for TCD8_CSR {}
#[doc = "TCD Control and Status"]
pub mod tcd8_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd8_biter_elinkno](tcd8_biter_elinkno) module"]
pub type TCD8_BITER_ELINKNO = crate::Reg<u16, _TCD8_BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD8_BITER_ELINKNO;
#[doc = "`read()` method returns [tcd8_biter_elinkno::R](tcd8_biter_elinkno::R) reader structure"]
impl crate::Readable for TCD8_BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd8_biter_elinkno::W](tcd8_biter_elinkno::W) writer structure"]
impl crate::Writable for TCD8_BITER_ELINKNO {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd8_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd8_biter_elinkyes](tcd8_biter_elinkyes) module"]
pub type TCD8_BITER_ELINKYES = crate::Reg<u16, _TCD8_BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD8_BITER_ELINKYES;
#[doc = "`read()` method returns [tcd8_biter_elinkyes::R](tcd8_biter_elinkyes::R) reader structure"]
impl crate::Readable for TCD8_BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd8_biter_elinkyes::W](tcd8_biter_elinkyes::W) writer structure"]
impl crate::Writable for TCD8_BITER_ELINKYES {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd8_biter_elinkyes;
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd9_saddr](tcd9_saddr) module"]
pub type TCD9_SADDR = crate::Reg<u32, _TCD9_SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD9_SADDR;
#[doc = "`read()` method returns [tcd9_saddr::R](tcd9_saddr::R) reader structure"]
impl crate::Readable for TCD9_SADDR {}
#[doc = "`write(|w| ..)` method takes [tcd9_saddr::W](tcd9_saddr::W) writer structure"]
impl crate::Writable for TCD9_SADDR {}
#[doc = "TCD Source Address"]
pub mod tcd9_saddr;
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd9_soff](tcd9_soff) module"]
pub type TCD9_SOFF = crate::Reg<u16, _TCD9_SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD9_SOFF;
#[doc = "`read()` method returns [tcd9_soff::R](tcd9_soff::R) reader structure"]
impl crate::Readable for TCD9_SOFF {}
#[doc = "`write(|w| ..)` method takes [tcd9_soff::W](tcd9_soff::W) writer structure"]
impl crate::Writable for TCD9_SOFF {}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd9_soff;
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd9_attr](tcd9_attr) module"]
pub type TCD9_ATTR = crate::Reg<u16, _TCD9_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD9_ATTR;
#[doc = "`read()` method returns [tcd9_attr::R](tcd9_attr::R) reader structure"]
impl crate::Readable for TCD9_ATTR {}
#[doc = "`write(|w| ..)` method takes [tcd9_attr::W](tcd9_attr::W) writer structure"]
impl crate::Writable for TCD9_ATTR {}
#[doc = "TCD Transfer Attributes"]
pub mod tcd9_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd9_nbytes_mlno](tcd9_nbytes_mlno) module"]
pub type TCD9_NBYTES_MLNO = crate::Reg<u32, _TCD9_NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD9_NBYTES_MLNO;
#[doc = "`read()` method returns [tcd9_nbytes_mlno::R](tcd9_nbytes_mlno::R) reader structure"]
impl crate::Readable for TCD9_NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [tcd9_nbytes_mlno::W](tcd9_nbytes_mlno::W) writer structure"]
impl crate::Writable for TCD9_NBYTES_MLNO {}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd9_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd9_nbytes_mloffno](tcd9_nbytes_mloffno) module"]
pub type TCD9_NBYTES_MLOFFNO = crate::Reg<u32, _TCD9_NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD9_NBYTES_MLOFFNO;
#[doc = "`read()` method returns [tcd9_nbytes_mloffno::R](tcd9_nbytes_mloffno::R) reader structure"]
impl crate::Readable for TCD9_NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [tcd9_nbytes_mloffno::W](tcd9_nbytes_mloffno::W) writer structure"]
impl crate::Writable for TCD9_NBYTES_MLOFFNO {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd9_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd9_nbytes_mloffyes](tcd9_nbytes_mloffyes) module"]
pub type TCD9_NBYTES_MLOFFYES = crate::Reg<u32, _TCD9_NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD9_NBYTES_MLOFFYES;
#[doc = "`read()` method returns [tcd9_nbytes_mloffyes::R](tcd9_nbytes_mloffyes::R) reader structure"]
impl crate::Readable for TCD9_NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [tcd9_nbytes_mloffyes::W](tcd9_nbytes_mloffyes::W) writer structure"]
impl crate::Writable for TCD9_NBYTES_MLOFFYES {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd9_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd9_slast](tcd9_slast) module"]
pub type TCD9_SLAST = crate::Reg<u32, _TCD9_SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD9_SLAST;
#[doc = "`read()` method returns [tcd9_slast::R](tcd9_slast::R) reader structure"]
impl crate::Readable for TCD9_SLAST {}
#[doc = "`write(|w| ..)` method takes [tcd9_slast::W](tcd9_slast::W) writer structure"]
impl crate::Writable for TCD9_SLAST {}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd9_slast;
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd9_daddr](tcd9_daddr) module"]
pub type TCD9_DADDR = crate::Reg<u32, _TCD9_DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD9_DADDR;
#[doc = "`read()` method returns [tcd9_daddr::R](tcd9_daddr::R) reader structure"]
impl crate::Readable for TCD9_DADDR {}
#[doc = "`write(|w| ..)` method takes [tcd9_daddr::W](tcd9_daddr::W) writer structure"]
impl crate::Writable for TCD9_DADDR {}
#[doc = "TCD Destination Address"]
pub mod tcd9_daddr;
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd9_doff](tcd9_doff) module"]
pub type TCD9_DOFF = crate::Reg<u16, _TCD9_DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD9_DOFF;
#[doc = "`read()` method returns [tcd9_doff::R](tcd9_doff::R) reader structure"]
impl crate::Readable for TCD9_DOFF {}
#[doc = "`write(|w| ..)` method takes [tcd9_doff::W](tcd9_doff::W) writer structure"]
impl crate::Writable for TCD9_DOFF {}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd9_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd9_citer_elinkno](tcd9_citer_elinkno) module"]
pub type TCD9_CITER_ELINKNO = crate::Reg<u16, _TCD9_CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD9_CITER_ELINKNO;
#[doc = "`read()` method returns [tcd9_citer_elinkno::R](tcd9_citer_elinkno::R) reader structure"]
impl crate::Readable for TCD9_CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd9_citer_elinkno::W](tcd9_citer_elinkno::W) writer structure"]
impl crate::Writable for TCD9_CITER_ELINKNO {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd9_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd9_citer_elinkyes](tcd9_citer_elinkyes) module"]
pub type TCD9_CITER_ELINKYES = crate::Reg<u16, _TCD9_CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD9_CITER_ELINKYES;
#[doc = "`read()` method returns [tcd9_citer_elinkyes::R](tcd9_citer_elinkyes::R) reader structure"]
impl crate::Readable for TCD9_CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd9_citer_elinkyes::W](tcd9_citer_elinkyes::W) writer structure"]
impl crate::Writable for TCD9_CITER_ELINKYES {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd9_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd9_dlastsga](tcd9_dlastsga) module"]
pub type TCD9_DLASTSGA = crate::Reg<u32, _TCD9_DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD9_DLASTSGA;
#[doc = "`read()` method returns [tcd9_dlastsga::R](tcd9_dlastsga::R) reader structure"]
impl crate::Readable for TCD9_DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [tcd9_dlastsga::W](tcd9_dlastsga::W) writer structure"]
impl crate::Writable for TCD9_DLASTSGA {}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd9_dlastsga;
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd9_csr](tcd9_csr) module"]
pub type TCD9_CSR = crate::Reg<u16, _TCD9_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD9_CSR;
#[doc = "`read()` method returns [tcd9_csr::R](tcd9_csr::R) reader structure"]
impl crate::Readable for TCD9_CSR {}
#[doc = "`write(|w| ..)` method takes [tcd9_csr::W](tcd9_csr::W) writer structure"]
impl crate::Writable for TCD9_CSR {}
#[doc = "TCD Control and Status"]
pub mod tcd9_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd9_biter_elinkno](tcd9_biter_elinkno) module"]
pub type TCD9_BITER_ELINKNO = crate::Reg<u16, _TCD9_BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD9_BITER_ELINKNO;
#[doc = "`read()` method returns [tcd9_biter_elinkno::R](tcd9_biter_elinkno::R) reader structure"]
impl crate::Readable for TCD9_BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd9_biter_elinkno::W](tcd9_biter_elinkno::W) writer structure"]
impl crate::Writable for TCD9_BITER_ELINKNO {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd9_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd9_biter_elinkyes](tcd9_biter_elinkyes) module"]
pub type TCD9_BITER_ELINKYES = crate::Reg<u16, _TCD9_BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD9_BITER_ELINKYES;
#[doc = "`read()` method returns [tcd9_biter_elinkyes::R](tcd9_biter_elinkyes::R) reader structure"]
impl crate::Readable for TCD9_BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd9_biter_elinkyes::W](tcd9_biter_elinkyes::W) writer structure"]
impl crate::Writable for TCD9_BITER_ELINKYES {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd9_biter_elinkyes;
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd10_saddr](tcd10_saddr) module"]
pub type TCD10_SADDR = crate::Reg<u32, _TCD10_SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD10_SADDR;
#[doc = "`read()` method returns [tcd10_saddr::R](tcd10_saddr::R) reader structure"]
impl crate::Readable for TCD10_SADDR {}
#[doc = "`write(|w| ..)` method takes [tcd10_saddr::W](tcd10_saddr::W) writer structure"]
impl crate::Writable for TCD10_SADDR {}
#[doc = "TCD Source Address"]
pub mod tcd10_saddr;
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd10_soff](tcd10_soff) module"]
pub type TCD10_SOFF = crate::Reg<u16, _TCD10_SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD10_SOFF;
#[doc = "`read()` method returns [tcd10_soff::R](tcd10_soff::R) reader structure"]
impl crate::Readable for TCD10_SOFF {}
#[doc = "`write(|w| ..)` method takes [tcd10_soff::W](tcd10_soff::W) writer structure"]
impl crate::Writable for TCD10_SOFF {}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd10_soff;
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd10_attr](tcd10_attr) module"]
pub type TCD10_ATTR = crate::Reg<u16, _TCD10_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD10_ATTR;
#[doc = "`read()` method returns [tcd10_attr::R](tcd10_attr::R) reader structure"]
impl crate::Readable for TCD10_ATTR {}
#[doc = "`write(|w| ..)` method takes [tcd10_attr::W](tcd10_attr::W) writer structure"]
impl crate::Writable for TCD10_ATTR {}
#[doc = "TCD Transfer Attributes"]
pub mod tcd10_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd10_nbytes_mlno](tcd10_nbytes_mlno) module"]
pub type TCD10_NBYTES_MLNO = crate::Reg<u32, _TCD10_NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD10_NBYTES_MLNO;
#[doc = "`read()` method returns [tcd10_nbytes_mlno::R](tcd10_nbytes_mlno::R) reader structure"]
impl crate::Readable for TCD10_NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [tcd10_nbytes_mlno::W](tcd10_nbytes_mlno::W) writer structure"]
impl crate::Writable for TCD10_NBYTES_MLNO {}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd10_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd10_nbytes_mloffno](tcd10_nbytes_mloffno) module"]
pub type TCD10_NBYTES_MLOFFNO = crate::Reg<u32, _TCD10_NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD10_NBYTES_MLOFFNO;
#[doc = "`read()` method returns [tcd10_nbytes_mloffno::R](tcd10_nbytes_mloffno::R) reader structure"]
impl crate::Readable for TCD10_NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [tcd10_nbytes_mloffno::W](tcd10_nbytes_mloffno::W) writer structure"]
impl crate::Writable for TCD10_NBYTES_MLOFFNO {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd10_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd10_nbytes_mloffyes](tcd10_nbytes_mloffyes) module"]
pub type TCD10_NBYTES_MLOFFYES = crate::Reg<u32, _TCD10_NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD10_NBYTES_MLOFFYES;
#[doc = "`read()` method returns [tcd10_nbytes_mloffyes::R](tcd10_nbytes_mloffyes::R) reader structure"]
impl crate::Readable for TCD10_NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [tcd10_nbytes_mloffyes::W](tcd10_nbytes_mloffyes::W) writer structure"]
impl crate::Writable for TCD10_NBYTES_MLOFFYES {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd10_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd10_slast](tcd10_slast) module"]
pub type TCD10_SLAST = crate::Reg<u32, _TCD10_SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD10_SLAST;
#[doc = "`read()` method returns [tcd10_slast::R](tcd10_slast::R) reader structure"]
impl crate::Readable for TCD10_SLAST {}
#[doc = "`write(|w| ..)` method takes [tcd10_slast::W](tcd10_slast::W) writer structure"]
impl crate::Writable for TCD10_SLAST {}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd10_slast;
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd10_daddr](tcd10_daddr) module"]
pub type TCD10_DADDR = crate::Reg<u32, _TCD10_DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD10_DADDR;
#[doc = "`read()` method returns [tcd10_daddr::R](tcd10_daddr::R) reader structure"]
impl crate::Readable for TCD10_DADDR {}
#[doc = "`write(|w| ..)` method takes [tcd10_daddr::W](tcd10_daddr::W) writer structure"]
impl crate::Writable for TCD10_DADDR {}
#[doc = "TCD Destination Address"]
pub mod tcd10_daddr;
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd10_doff](tcd10_doff) module"]
pub type TCD10_DOFF = crate::Reg<u16, _TCD10_DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD10_DOFF;
#[doc = "`read()` method returns [tcd10_doff::R](tcd10_doff::R) reader structure"]
impl crate::Readable for TCD10_DOFF {}
#[doc = "`write(|w| ..)` method takes [tcd10_doff::W](tcd10_doff::W) writer structure"]
impl crate::Writable for TCD10_DOFF {}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd10_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd10_citer_elinkno](tcd10_citer_elinkno) module"]
pub type TCD10_CITER_ELINKNO = crate::Reg<u16, _TCD10_CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD10_CITER_ELINKNO;
#[doc = "`read()` method returns [tcd10_citer_elinkno::R](tcd10_citer_elinkno::R) reader structure"]
impl crate::Readable for TCD10_CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd10_citer_elinkno::W](tcd10_citer_elinkno::W) writer structure"]
impl crate::Writable for TCD10_CITER_ELINKNO {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd10_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd10_citer_elinkyes](tcd10_citer_elinkyes) module"]
pub type TCD10_CITER_ELINKYES = crate::Reg<u16, _TCD10_CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD10_CITER_ELINKYES;
#[doc = "`read()` method returns [tcd10_citer_elinkyes::R](tcd10_citer_elinkyes::R) reader structure"]
impl crate::Readable for TCD10_CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd10_citer_elinkyes::W](tcd10_citer_elinkyes::W) writer structure"]
impl crate::Writable for TCD10_CITER_ELINKYES {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd10_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd10_dlastsga](tcd10_dlastsga) module"]
pub type TCD10_DLASTSGA = crate::Reg<u32, _TCD10_DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD10_DLASTSGA;
#[doc = "`read()` method returns [tcd10_dlastsga::R](tcd10_dlastsga::R) reader structure"]
impl crate::Readable for TCD10_DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [tcd10_dlastsga::W](tcd10_dlastsga::W) writer structure"]
impl crate::Writable for TCD10_DLASTSGA {}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd10_dlastsga;
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd10_csr](tcd10_csr) module"]
pub type TCD10_CSR = crate::Reg<u16, _TCD10_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD10_CSR;
#[doc = "`read()` method returns [tcd10_csr::R](tcd10_csr::R) reader structure"]
impl crate::Readable for TCD10_CSR {}
#[doc = "`write(|w| ..)` method takes [tcd10_csr::W](tcd10_csr::W) writer structure"]
impl crate::Writable for TCD10_CSR {}
#[doc = "TCD Control and Status"]
pub mod tcd10_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd10_biter_elinkno](tcd10_biter_elinkno) module"]
pub type TCD10_BITER_ELINKNO = crate::Reg<u16, _TCD10_BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD10_BITER_ELINKNO;
#[doc = "`read()` method returns [tcd10_biter_elinkno::R](tcd10_biter_elinkno::R) reader structure"]
impl crate::Readable for TCD10_BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd10_biter_elinkno::W](tcd10_biter_elinkno::W) writer structure"]
impl crate::Writable for TCD10_BITER_ELINKNO {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd10_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd10_biter_elinkyes](tcd10_biter_elinkyes) module"]
pub type TCD10_BITER_ELINKYES = crate::Reg<u16, _TCD10_BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD10_BITER_ELINKYES;
#[doc = "`read()` method returns [tcd10_biter_elinkyes::R](tcd10_biter_elinkyes::R) reader structure"]
impl crate::Readable for TCD10_BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd10_biter_elinkyes::W](tcd10_biter_elinkyes::W) writer structure"]
impl crate::Writable for TCD10_BITER_ELINKYES {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd10_biter_elinkyes;
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd11_saddr](tcd11_saddr) module"]
pub type TCD11_SADDR = crate::Reg<u32, _TCD11_SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD11_SADDR;
#[doc = "`read()` method returns [tcd11_saddr::R](tcd11_saddr::R) reader structure"]
impl crate::Readable for TCD11_SADDR {}
#[doc = "`write(|w| ..)` method takes [tcd11_saddr::W](tcd11_saddr::W) writer structure"]
impl crate::Writable for TCD11_SADDR {}
#[doc = "TCD Source Address"]
pub mod tcd11_saddr;
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd11_soff](tcd11_soff) module"]
pub type TCD11_SOFF = crate::Reg<u16, _TCD11_SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD11_SOFF;
#[doc = "`read()` method returns [tcd11_soff::R](tcd11_soff::R) reader structure"]
impl crate::Readable for TCD11_SOFF {}
#[doc = "`write(|w| ..)` method takes [tcd11_soff::W](tcd11_soff::W) writer structure"]
impl crate::Writable for TCD11_SOFF {}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd11_soff;
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd11_attr](tcd11_attr) module"]
pub type TCD11_ATTR = crate::Reg<u16, _TCD11_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD11_ATTR;
#[doc = "`read()` method returns [tcd11_attr::R](tcd11_attr::R) reader structure"]
impl crate::Readable for TCD11_ATTR {}
#[doc = "`write(|w| ..)` method takes [tcd11_attr::W](tcd11_attr::W) writer structure"]
impl crate::Writable for TCD11_ATTR {}
#[doc = "TCD Transfer Attributes"]
pub mod tcd11_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd11_nbytes_mlno](tcd11_nbytes_mlno) module"]
pub type TCD11_NBYTES_MLNO = crate::Reg<u32, _TCD11_NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD11_NBYTES_MLNO;
#[doc = "`read()` method returns [tcd11_nbytes_mlno::R](tcd11_nbytes_mlno::R) reader structure"]
impl crate::Readable for TCD11_NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [tcd11_nbytes_mlno::W](tcd11_nbytes_mlno::W) writer structure"]
impl crate::Writable for TCD11_NBYTES_MLNO {}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd11_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd11_nbytes_mloffno](tcd11_nbytes_mloffno) module"]
pub type TCD11_NBYTES_MLOFFNO = crate::Reg<u32, _TCD11_NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD11_NBYTES_MLOFFNO;
#[doc = "`read()` method returns [tcd11_nbytes_mloffno::R](tcd11_nbytes_mloffno::R) reader structure"]
impl crate::Readable for TCD11_NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [tcd11_nbytes_mloffno::W](tcd11_nbytes_mloffno::W) writer structure"]
impl crate::Writable for TCD11_NBYTES_MLOFFNO {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd11_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd11_nbytes_mloffyes](tcd11_nbytes_mloffyes) module"]
pub type TCD11_NBYTES_MLOFFYES = crate::Reg<u32, _TCD11_NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD11_NBYTES_MLOFFYES;
#[doc = "`read()` method returns [tcd11_nbytes_mloffyes::R](tcd11_nbytes_mloffyes::R) reader structure"]
impl crate::Readable for TCD11_NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [tcd11_nbytes_mloffyes::W](tcd11_nbytes_mloffyes::W) writer structure"]
impl crate::Writable for TCD11_NBYTES_MLOFFYES {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd11_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd11_slast](tcd11_slast) module"]
pub type TCD11_SLAST = crate::Reg<u32, _TCD11_SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD11_SLAST;
#[doc = "`read()` method returns [tcd11_slast::R](tcd11_slast::R) reader structure"]
impl crate::Readable for TCD11_SLAST {}
#[doc = "`write(|w| ..)` method takes [tcd11_slast::W](tcd11_slast::W) writer structure"]
impl crate::Writable for TCD11_SLAST {}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd11_slast;
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd11_daddr](tcd11_daddr) module"]
pub type TCD11_DADDR = crate::Reg<u32, _TCD11_DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD11_DADDR;
#[doc = "`read()` method returns [tcd11_daddr::R](tcd11_daddr::R) reader structure"]
impl crate::Readable for TCD11_DADDR {}
#[doc = "`write(|w| ..)` method takes [tcd11_daddr::W](tcd11_daddr::W) writer structure"]
impl crate::Writable for TCD11_DADDR {}
#[doc = "TCD Destination Address"]
pub mod tcd11_daddr;
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd11_doff](tcd11_doff) module"]
pub type TCD11_DOFF = crate::Reg<u16, _TCD11_DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD11_DOFF;
#[doc = "`read()` method returns [tcd11_doff::R](tcd11_doff::R) reader structure"]
impl crate::Readable for TCD11_DOFF {}
#[doc = "`write(|w| ..)` method takes [tcd11_doff::W](tcd11_doff::W) writer structure"]
impl crate::Writable for TCD11_DOFF {}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd11_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd11_citer_elinkno](tcd11_citer_elinkno) module"]
pub type TCD11_CITER_ELINKNO = crate::Reg<u16, _TCD11_CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD11_CITER_ELINKNO;
#[doc = "`read()` method returns [tcd11_citer_elinkno::R](tcd11_citer_elinkno::R) reader structure"]
impl crate::Readable for TCD11_CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd11_citer_elinkno::W](tcd11_citer_elinkno::W) writer structure"]
impl crate::Writable for TCD11_CITER_ELINKNO {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd11_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd11_citer_elinkyes](tcd11_citer_elinkyes) module"]
pub type TCD11_CITER_ELINKYES = crate::Reg<u16, _TCD11_CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD11_CITER_ELINKYES;
#[doc = "`read()` method returns [tcd11_citer_elinkyes::R](tcd11_citer_elinkyes::R) reader structure"]
impl crate::Readable for TCD11_CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd11_citer_elinkyes::W](tcd11_citer_elinkyes::W) writer structure"]
impl crate::Writable for TCD11_CITER_ELINKYES {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd11_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd11_dlastsga](tcd11_dlastsga) module"]
pub type TCD11_DLASTSGA = crate::Reg<u32, _TCD11_DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD11_DLASTSGA;
#[doc = "`read()` method returns [tcd11_dlastsga::R](tcd11_dlastsga::R) reader structure"]
impl crate::Readable for TCD11_DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [tcd11_dlastsga::W](tcd11_dlastsga::W) writer structure"]
impl crate::Writable for TCD11_DLASTSGA {}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd11_dlastsga;
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd11_csr](tcd11_csr) module"]
pub type TCD11_CSR = crate::Reg<u16, _TCD11_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD11_CSR;
#[doc = "`read()` method returns [tcd11_csr::R](tcd11_csr::R) reader structure"]
impl crate::Readable for TCD11_CSR {}
#[doc = "`write(|w| ..)` method takes [tcd11_csr::W](tcd11_csr::W) writer structure"]
impl crate::Writable for TCD11_CSR {}
#[doc = "TCD Control and Status"]
pub mod tcd11_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd11_biter_elinkno](tcd11_biter_elinkno) module"]
pub type TCD11_BITER_ELINKNO = crate::Reg<u16, _TCD11_BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD11_BITER_ELINKNO;
#[doc = "`read()` method returns [tcd11_biter_elinkno::R](tcd11_biter_elinkno::R) reader structure"]
impl crate::Readable for TCD11_BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd11_biter_elinkno::W](tcd11_biter_elinkno::W) writer structure"]
impl crate::Writable for TCD11_BITER_ELINKNO {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd11_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd11_biter_elinkyes](tcd11_biter_elinkyes) module"]
pub type TCD11_BITER_ELINKYES = crate::Reg<u16, _TCD11_BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD11_BITER_ELINKYES;
#[doc = "`read()` method returns [tcd11_biter_elinkyes::R](tcd11_biter_elinkyes::R) reader structure"]
impl crate::Readable for TCD11_BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd11_biter_elinkyes::W](tcd11_biter_elinkyes::W) writer structure"]
impl crate::Writable for TCD11_BITER_ELINKYES {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd11_biter_elinkyes;
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd12_saddr](tcd12_saddr) module"]
pub type TCD12_SADDR = crate::Reg<u32, _TCD12_SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD12_SADDR;
#[doc = "`read()` method returns [tcd12_saddr::R](tcd12_saddr::R) reader structure"]
impl crate::Readable for TCD12_SADDR {}
#[doc = "`write(|w| ..)` method takes [tcd12_saddr::W](tcd12_saddr::W) writer structure"]
impl crate::Writable for TCD12_SADDR {}
#[doc = "TCD Source Address"]
pub mod tcd12_saddr;
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd12_soff](tcd12_soff) module"]
pub type TCD12_SOFF = crate::Reg<u16, _TCD12_SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD12_SOFF;
#[doc = "`read()` method returns [tcd12_soff::R](tcd12_soff::R) reader structure"]
impl crate::Readable for TCD12_SOFF {}
#[doc = "`write(|w| ..)` method takes [tcd12_soff::W](tcd12_soff::W) writer structure"]
impl crate::Writable for TCD12_SOFF {}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd12_soff;
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd12_attr](tcd12_attr) module"]
pub type TCD12_ATTR = crate::Reg<u16, _TCD12_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD12_ATTR;
#[doc = "`read()` method returns [tcd12_attr::R](tcd12_attr::R) reader structure"]
impl crate::Readable for TCD12_ATTR {}
#[doc = "`write(|w| ..)` method takes [tcd12_attr::W](tcd12_attr::W) writer structure"]
impl crate::Writable for TCD12_ATTR {}
#[doc = "TCD Transfer Attributes"]
pub mod tcd12_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd12_nbytes_mlno](tcd12_nbytes_mlno) module"]
pub type TCD12_NBYTES_MLNO = crate::Reg<u32, _TCD12_NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD12_NBYTES_MLNO;
#[doc = "`read()` method returns [tcd12_nbytes_mlno::R](tcd12_nbytes_mlno::R) reader structure"]
impl crate::Readable for TCD12_NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [tcd12_nbytes_mlno::W](tcd12_nbytes_mlno::W) writer structure"]
impl crate::Writable for TCD12_NBYTES_MLNO {}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd12_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd12_nbytes_mloffno](tcd12_nbytes_mloffno) module"]
pub type TCD12_NBYTES_MLOFFNO = crate::Reg<u32, _TCD12_NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD12_NBYTES_MLOFFNO;
#[doc = "`read()` method returns [tcd12_nbytes_mloffno::R](tcd12_nbytes_mloffno::R) reader structure"]
impl crate::Readable for TCD12_NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [tcd12_nbytes_mloffno::W](tcd12_nbytes_mloffno::W) writer structure"]
impl crate::Writable for TCD12_NBYTES_MLOFFNO {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd12_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd12_nbytes_mloffyes](tcd12_nbytes_mloffyes) module"]
pub type TCD12_NBYTES_MLOFFYES = crate::Reg<u32, _TCD12_NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD12_NBYTES_MLOFFYES;
#[doc = "`read()` method returns [tcd12_nbytes_mloffyes::R](tcd12_nbytes_mloffyes::R) reader structure"]
impl crate::Readable for TCD12_NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [tcd12_nbytes_mloffyes::W](tcd12_nbytes_mloffyes::W) writer structure"]
impl crate::Writable for TCD12_NBYTES_MLOFFYES {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd12_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd12_slast](tcd12_slast) module"]
pub type TCD12_SLAST = crate::Reg<u32, _TCD12_SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD12_SLAST;
#[doc = "`read()` method returns [tcd12_slast::R](tcd12_slast::R) reader structure"]
impl crate::Readable for TCD12_SLAST {}
#[doc = "`write(|w| ..)` method takes [tcd12_slast::W](tcd12_slast::W) writer structure"]
impl crate::Writable for TCD12_SLAST {}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd12_slast;
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd12_daddr](tcd12_daddr) module"]
pub type TCD12_DADDR = crate::Reg<u32, _TCD12_DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD12_DADDR;
#[doc = "`read()` method returns [tcd12_daddr::R](tcd12_daddr::R) reader structure"]
impl crate::Readable for TCD12_DADDR {}
#[doc = "`write(|w| ..)` method takes [tcd12_daddr::W](tcd12_daddr::W) writer structure"]
impl crate::Writable for TCD12_DADDR {}
#[doc = "TCD Destination Address"]
pub mod tcd12_daddr;
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd12_doff](tcd12_doff) module"]
pub type TCD12_DOFF = crate::Reg<u16, _TCD12_DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD12_DOFF;
#[doc = "`read()` method returns [tcd12_doff::R](tcd12_doff::R) reader structure"]
impl crate::Readable for TCD12_DOFF {}
#[doc = "`write(|w| ..)` method takes [tcd12_doff::W](tcd12_doff::W) writer structure"]
impl crate::Writable for TCD12_DOFF {}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd12_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd12_citer_elinkno](tcd12_citer_elinkno) module"]
pub type TCD12_CITER_ELINKNO = crate::Reg<u16, _TCD12_CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD12_CITER_ELINKNO;
#[doc = "`read()` method returns [tcd12_citer_elinkno::R](tcd12_citer_elinkno::R) reader structure"]
impl crate::Readable for TCD12_CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd12_citer_elinkno::W](tcd12_citer_elinkno::W) writer structure"]
impl crate::Writable for TCD12_CITER_ELINKNO {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd12_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd12_citer_elinkyes](tcd12_citer_elinkyes) module"]
pub type TCD12_CITER_ELINKYES = crate::Reg<u16, _TCD12_CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD12_CITER_ELINKYES;
#[doc = "`read()` method returns [tcd12_citer_elinkyes::R](tcd12_citer_elinkyes::R) reader structure"]
impl crate::Readable for TCD12_CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd12_citer_elinkyes::W](tcd12_citer_elinkyes::W) writer structure"]
impl crate::Writable for TCD12_CITER_ELINKYES {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd12_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd12_dlastsga](tcd12_dlastsga) module"]
pub type TCD12_DLASTSGA = crate::Reg<u32, _TCD12_DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD12_DLASTSGA;
#[doc = "`read()` method returns [tcd12_dlastsga::R](tcd12_dlastsga::R) reader structure"]
impl crate::Readable for TCD12_DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [tcd12_dlastsga::W](tcd12_dlastsga::W) writer structure"]
impl crate::Writable for TCD12_DLASTSGA {}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd12_dlastsga;
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd12_csr](tcd12_csr) module"]
pub type TCD12_CSR = crate::Reg<u16, _TCD12_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD12_CSR;
#[doc = "`read()` method returns [tcd12_csr::R](tcd12_csr::R) reader structure"]
impl crate::Readable for TCD12_CSR {}
#[doc = "`write(|w| ..)` method takes [tcd12_csr::W](tcd12_csr::W) writer structure"]
impl crate::Writable for TCD12_CSR {}
#[doc = "TCD Control and Status"]
pub mod tcd12_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd12_biter_elinkno](tcd12_biter_elinkno) module"]
pub type TCD12_BITER_ELINKNO = crate::Reg<u16, _TCD12_BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD12_BITER_ELINKNO;
#[doc = "`read()` method returns [tcd12_biter_elinkno::R](tcd12_biter_elinkno::R) reader structure"]
impl crate::Readable for TCD12_BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd12_biter_elinkno::W](tcd12_biter_elinkno::W) writer structure"]
impl crate::Writable for TCD12_BITER_ELINKNO {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd12_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd12_biter_elinkyes](tcd12_biter_elinkyes) module"]
pub type TCD12_BITER_ELINKYES = crate::Reg<u16, _TCD12_BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD12_BITER_ELINKYES;
#[doc = "`read()` method returns [tcd12_biter_elinkyes::R](tcd12_biter_elinkyes::R) reader structure"]
impl crate::Readable for TCD12_BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd12_biter_elinkyes::W](tcd12_biter_elinkyes::W) writer structure"]
impl crate::Writable for TCD12_BITER_ELINKYES {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd12_biter_elinkyes;
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd13_saddr](tcd13_saddr) module"]
pub type TCD13_SADDR = crate::Reg<u32, _TCD13_SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD13_SADDR;
#[doc = "`read()` method returns [tcd13_saddr::R](tcd13_saddr::R) reader structure"]
impl crate::Readable for TCD13_SADDR {}
#[doc = "`write(|w| ..)` method takes [tcd13_saddr::W](tcd13_saddr::W) writer structure"]
impl crate::Writable for TCD13_SADDR {}
#[doc = "TCD Source Address"]
pub mod tcd13_saddr;
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd13_soff](tcd13_soff) module"]
pub type TCD13_SOFF = crate::Reg<u16, _TCD13_SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD13_SOFF;
#[doc = "`read()` method returns [tcd13_soff::R](tcd13_soff::R) reader structure"]
impl crate::Readable for TCD13_SOFF {}
#[doc = "`write(|w| ..)` method takes [tcd13_soff::W](tcd13_soff::W) writer structure"]
impl crate::Writable for TCD13_SOFF {}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd13_soff;
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd13_attr](tcd13_attr) module"]
pub type TCD13_ATTR = crate::Reg<u16, _TCD13_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD13_ATTR;
#[doc = "`read()` method returns [tcd13_attr::R](tcd13_attr::R) reader structure"]
impl crate::Readable for TCD13_ATTR {}
#[doc = "`write(|w| ..)` method takes [tcd13_attr::W](tcd13_attr::W) writer structure"]
impl crate::Writable for TCD13_ATTR {}
#[doc = "TCD Transfer Attributes"]
pub mod tcd13_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd13_nbytes_mlno](tcd13_nbytes_mlno) module"]
pub type TCD13_NBYTES_MLNO = crate::Reg<u32, _TCD13_NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD13_NBYTES_MLNO;
#[doc = "`read()` method returns [tcd13_nbytes_mlno::R](tcd13_nbytes_mlno::R) reader structure"]
impl crate::Readable for TCD13_NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [tcd13_nbytes_mlno::W](tcd13_nbytes_mlno::W) writer structure"]
impl crate::Writable for TCD13_NBYTES_MLNO {}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd13_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd13_nbytes_mloffno](tcd13_nbytes_mloffno) module"]
pub type TCD13_NBYTES_MLOFFNO = crate::Reg<u32, _TCD13_NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD13_NBYTES_MLOFFNO;
#[doc = "`read()` method returns [tcd13_nbytes_mloffno::R](tcd13_nbytes_mloffno::R) reader structure"]
impl crate::Readable for TCD13_NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [tcd13_nbytes_mloffno::W](tcd13_nbytes_mloffno::W) writer structure"]
impl crate::Writable for TCD13_NBYTES_MLOFFNO {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd13_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd13_nbytes_mloffyes](tcd13_nbytes_mloffyes) module"]
pub type TCD13_NBYTES_MLOFFYES = crate::Reg<u32, _TCD13_NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD13_NBYTES_MLOFFYES;
#[doc = "`read()` method returns [tcd13_nbytes_mloffyes::R](tcd13_nbytes_mloffyes::R) reader structure"]
impl crate::Readable for TCD13_NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [tcd13_nbytes_mloffyes::W](tcd13_nbytes_mloffyes::W) writer structure"]
impl crate::Writable for TCD13_NBYTES_MLOFFYES {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd13_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd13_slast](tcd13_slast) module"]
pub type TCD13_SLAST = crate::Reg<u32, _TCD13_SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD13_SLAST;
#[doc = "`read()` method returns [tcd13_slast::R](tcd13_slast::R) reader structure"]
impl crate::Readable for TCD13_SLAST {}
#[doc = "`write(|w| ..)` method takes [tcd13_slast::W](tcd13_slast::W) writer structure"]
impl crate::Writable for TCD13_SLAST {}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd13_slast;
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd13_daddr](tcd13_daddr) module"]
pub type TCD13_DADDR = crate::Reg<u32, _TCD13_DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD13_DADDR;
#[doc = "`read()` method returns [tcd13_daddr::R](tcd13_daddr::R) reader structure"]
impl crate::Readable for TCD13_DADDR {}
#[doc = "`write(|w| ..)` method takes [tcd13_daddr::W](tcd13_daddr::W) writer structure"]
impl crate::Writable for TCD13_DADDR {}
#[doc = "TCD Destination Address"]
pub mod tcd13_daddr;
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd13_doff](tcd13_doff) module"]
pub type TCD13_DOFF = crate::Reg<u16, _TCD13_DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD13_DOFF;
#[doc = "`read()` method returns [tcd13_doff::R](tcd13_doff::R) reader structure"]
impl crate::Readable for TCD13_DOFF {}
#[doc = "`write(|w| ..)` method takes [tcd13_doff::W](tcd13_doff::W) writer structure"]
impl crate::Writable for TCD13_DOFF {}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd13_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd13_citer_elinkno](tcd13_citer_elinkno) module"]
pub type TCD13_CITER_ELINKNO = crate::Reg<u16, _TCD13_CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD13_CITER_ELINKNO;
#[doc = "`read()` method returns [tcd13_citer_elinkno::R](tcd13_citer_elinkno::R) reader structure"]
impl crate::Readable for TCD13_CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd13_citer_elinkno::W](tcd13_citer_elinkno::W) writer structure"]
impl crate::Writable for TCD13_CITER_ELINKNO {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd13_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd13_citer_elinkyes](tcd13_citer_elinkyes) module"]
pub type TCD13_CITER_ELINKYES = crate::Reg<u16, _TCD13_CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD13_CITER_ELINKYES;
#[doc = "`read()` method returns [tcd13_citer_elinkyes::R](tcd13_citer_elinkyes::R) reader structure"]
impl crate::Readable for TCD13_CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd13_citer_elinkyes::W](tcd13_citer_elinkyes::W) writer structure"]
impl crate::Writable for TCD13_CITER_ELINKYES {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd13_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd13_dlastsga](tcd13_dlastsga) module"]
pub type TCD13_DLASTSGA = crate::Reg<u32, _TCD13_DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD13_DLASTSGA;
#[doc = "`read()` method returns [tcd13_dlastsga::R](tcd13_dlastsga::R) reader structure"]
impl crate::Readable for TCD13_DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [tcd13_dlastsga::W](tcd13_dlastsga::W) writer structure"]
impl crate::Writable for TCD13_DLASTSGA {}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd13_dlastsga;
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd13_csr](tcd13_csr) module"]
pub type TCD13_CSR = crate::Reg<u16, _TCD13_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD13_CSR;
#[doc = "`read()` method returns [tcd13_csr::R](tcd13_csr::R) reader structure"]
impl crate::Readable for TCD13_CSR {}
#[doc = "`write(|w| ..)` method takes [tcd13_csr::W](tcd13_csr::W) writer structure"]
impl crate::Writable for TCD13_CSR {}
#[doc = "TCD Control and Status"]
pub mod tcd13_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd13_biter_elinkno](tcd13_biter_elinkno) module"]
pub type TCD13_BITER_ELINKNO = crate::Reg<u16, _TCD13_BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD13_BITER_ELINKNO;
#[doc = "`read()` method returns [tcd13_biter_elinkno::R](tcd13_biter_elinkno::R) reader structure"]
impl crate::Readable for TCD13_BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd13_biter_elinkno::W](tcd13_biter_elinkno::W) writer structure"]
impl crate::Writable for TCD13_BITER_ELINKNO {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd13_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd13_biter_elinkyes](tcd13_biter_elinkyes) module"]
pub type TCD13_BITER_ELINKYES = crate::Reg<u16, _TCD13_BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD13_BITER_ELINKYES;
#[doc = "`read()` method returns [tcd13_biter_elinkyes::R](tcd13_biter_elinkyes::R) reader structure"]
impl crate::Readable for TCD13_BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd13_biter_elinkyes::W](tcd13_biter_elinkyes::W) writer structure"]
impl crate::Writable for TCD13_BITER_ELINKYES {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd13_biter_elinkyes;
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd14_saddr](tcd14_saddr) module"]
pub type TCD14_SADDR = crate::Reg<u32, _TCD14_SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD14_SADDR;
#[doc = "`read()` method returns [tcd14_saddr::R](tcd14_saddr::R) reader structure"]
impl crate::Readable for TCD14_SADDR {}
#[doc = "`write(|w| ..)` method takes [tcd14_saddr::W](tcd14_saddr::W) writer structure"]
impl crate::Writable for TCD14_SADDR {}
#[doc = "TCD Source Address"]
pub mod tcd14_saddr;
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd14_soff](tcd14_soff) module"]
pub type TCD14_SOFF = crate::Reg<u16, _TCD14_SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD14_SOFF;
#[doc = "`read()` method returns [tcd14_soff::R](tcd14_soff::R) reader structure"]
impl crate::Readable for TCD14_SOFF {}
#[doc = "`write(|w| ..)` method takes [tcd14_soff::W](tcd14_soff::W) writer structure"]
impl crate::Writable for TCD14_SOFF {}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd14_soff;
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd14_attr](tcd14_attr) module"]
pub type TCD14_ATTR = crate::Reg<u16, _TCD14_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD14_ATTR;
#[doc = "`read()` method returns [tcd14_attr::R](tcd14_attr::R) reader structure"]
impl crate::Readable for TCD14_ATTR {}
#[doc = "`write(|w| ..)` method takes [tcd14_attr::W](tcd14_attr::W) writer structure"]
impl crate::Writable for TCD14_ATTR {}
#[doc = "TCD Transfer Attributes"]
pub mod tcd14_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd14_nbytes_mlno](tcd14_nbytes_mlno) module"]
pub type TCD14_NBYTES_MLNO = crate::Reg<u32, _TCD14_NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD14_NBYTES_MLNO;
#[doc = "`read()` method returns [tcd14_nbytes_mlno::R](tcd14_nbytes_mlno::R) reader structure"]
impl crate::Readable for TCD14_NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [tcd14_nbytes_mlno::W](tcd14_nbytes_mlno::W) writer structure"]
impl crate::Writable for TCD14_NBYTES_MLNO {}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd14_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd14_nbytes_mloffno](tcd14_nbytes_mloffno) module"]
pub type TCD14_NBYTES_MLOFFNO = crate::Reg<u32, _TCD14_NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD14_NBYTES_MLOFFNO;
#[doc = "`read()` method returns [tcd14_nbytes_mloffno::R](tcd14_nbytes_mloffno::R) reader structure"]
impl crate::Readable for TCD14_NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [tcd14_nbytes_mloffno::W](tcd14_nbytes_mloffno::W) writer structure"]
impl crate::Writable for TCD14_NBYTES_MLOFFNO {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd14_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd14_nbytes_mloffyes](tcd14_nbytes_mloffyes) module"]
pub type TCD14_NBYTES_MLOFFYES = crate::Reg<u32, _TCD14_NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD14_NBYTES_MLOFFYES;
#[doc = "`read()` method returns [tcd14_nbytes_mloffyes::R](tcd14_nbytes_mloffyes::R) reader structure"]
impl crate::Readable for TCD14_NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [tcd14_nbytes_mloffyes::W](tcd14_nbytes_mloffyes::W) writer structure"]
impl crate::Writable for TCD14_NBYTES_MLOFFYES {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd14_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd14_slast](tcd14_slast) module"]
pub type TCD14_SLAST = crate::Reg<u32, _TCD14_SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD14_SLAST;
#[doc = "`read()` method returns [tcd14_slast::R](tcd14_slast::R) reader structure"]
impl crate::Readable for TCD14_SLAST {}
#[doc = "`write(|w| ..)` method takes [tcd14_slast::W](tcd14_slast::W) writer structure"]
impl crate::Writable for TCD14_SLAST {}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd14_slast;
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd14_daddr](tcd14_daddr) module"]
pub type TCD14_DADDR = crate::Reg<u32, _TCD14_DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD14_DADDR;
#[doc = "`read()` method returns [tcd14_daddr::R](tcd14_daddr::R) reader structure"]
impl crate::Readable for TCD14_DADDR {}
#[doc = "`write(|w| ..)` method takes [tcd14_daddr::W](tcd14_daddr::W) writer structure"]
impl crate::Writable for TCD14_DADDR {}
#[doc = "TCD Destination Address"]
pub mod tcd14_daddr;
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd14_doff](tcd14_doff) module"]
pub type TCD14_DOFF = crate::Reg<u16, _TCD14_DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD14_DOFF;
#[doc = "`read()` method returns [tcd14_doff::R](tcd14_doff::R) reader structure"]
impl crate::Readable for TCD14_DOFF {}
#[doc = "`write(|w| ..)` method takes [tcd14_doff::W](tcd14_doff::W) writer structure"]
impl crate::Writable for TCD14_DOFF {}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd14_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd14_citer_elinkno](tcd14_citer_elinkno) module"]
pub type TCD14_CITER_ELINKNO = crate::Reg<u16, _TCD14_CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD14_CITER_ELINKNO;
#[doc = "`read()` method returns [tcd14_citer_elinkno::R](tcd14_citer_elinkno::R) reader structure"]
impl crate::Readable for TCD14_CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd14_citer_elinkno::W](tcd14_citer_elinkno::W) writer structure"]
impl crate::Writable for TCD14_CITER_ELINKNO {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd14_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd14_citer_elinkyes](tcd14_citer_elinkyes) module"]
pub type TCD14_CITER_ELINKYES = crate::Reg<u16, _TCD14_CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD14_CITER_ELINKYES;
#[doc = "`read()` method returns [tcd14_citer_elinkyes::R](tcd14_citer_elinkyes::R) reader structure"]
impl crate::Readable for TCD14_CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd14_citer_elinkyes::W](tcd14_citer_elinkyes::W) writer structure"]
impl crate::Writable for TCD14_CITER_ELINKYES {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd14_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd14_dlastsga](tcd14_dlastsga) module"]
pub type TCD14_DLASTSGA = crate::Reg<u32, _TCD14_DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD14_DLASTSGA;
#[doc = "`read()` method returns [tcd14_dlastsga::R](tcd14_dlastsga::R) reader structure"]
impl crate::Readable for TCD14_DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [tcd14_dlastsga::W](tcd14_dlastsga::W) writer structure"]
impl crate::Writable for TCD14_DLASTSGA {}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd14_dlastsga;
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd14_csr](tcd14_csr) module"]
pub type TCD14_CSR = crate::Reg<u16, _TCD14_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD14_CSR;
#[doc = "`read()` method returns [tcd14_csr::R](tcd14_csr::R) reader structure"]
impl crate::Readable for TCD14_CSR {}
#[doc = "`write(|w| ..)` method takes [tcd14_csr::W](tcd14_csr::W) writer structure"]
impl crate::Writable for TCD14_CSR {}
#[doc = "TCD Control and Status"]
pub mod tcd14_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd14_biter_elinkno](tcd14_biter_elinkno) module"]
pub type TCD14_BITER_ELINKNO = crate::Reg<u16, _TCD14_BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD14_BITER_ELINKNO;
#[doc = "`read()` method returns [tcd14_biter_elinkno::R](tcd14_biter_elinkno::R) reader structure"]
impl crate::Readable for TCD14_BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd14_biter_elinkno::W](tcd14_biter_elinkno::W) writer structure"]
impl crate::Writable for TCD14_BITER_ELINKNO {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd14_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd14_biter_elinkyes](tcd14_biter_elinkyes) module"]
pub type TCD14_BITER_ELINKYES = crate::Reg<u16, _TCD14_BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD14_BITER_ELINKYES;
#[doc = "`read()` method returns [tcd14_biter_elinkyes::R](tcd14_biter_elinkyes::R) reader structure"]
impl crate::Readable for TCD14_BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd14_biter_elinkyes::W](tcd14_biter_elinkyes::W) writer structure"]
impl crate::Writable for TCD14_BITER_ELINKYES {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd14_biter_elinkyes;
#[doc = "TCD Source Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd15_saddr](tcd15_saddr) module"]
pub type TCD15_SADDR = crate::Reg<u32, _TCD15_SADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD15_SADDR;
#[doc = "`read()` method returns [tcd15_saddr::R](tcd15_saddr::R) reader structure"]
impl crate::Readable for TCD15_SADDR {}
#[doc = "`write(|w| ..)` method takes [tcd15_saddr::W](tcd15_saddr::W) writer structure"]
impl crate::Writable for TCD15_SADDR {}
#[doc = "TCD Source Address"]
pub mod tcd15_saddr;
#[doc = "TCD Signed Source Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd15_soff](tcd15_soff) module"]
pub type TCD15_SOFF = crate::Reg<u16, _TCD15_SOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD15_SOFF;
#[doc = "`read()` method returns [tcd15_soff::R](tcd15_soff::R) reader structure"]
impl crate::Readable for TCD15_SOFF {}
#[doc = "`write(|w| ..)` method takes [tcd15_soff::W](tcd15_soff::W) writer structure"]
impl crate::Writable for TCD15_SOFF {}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd15_soff;
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd15_attr](tcd15_attr) module"]
pub type TCD15_ATTR = crate::Reg<u16, _TCD15_ATTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD15_ATTR;
#[doc = "`read()` method returns [tcd15_attr::R](tcd15_attr::R) reader structure"]
impl crate::Readable for TCD15_ATTR {}
#[doc = "`write(|w| ..)` method takes [tcd15_attr::W](tcd15_attr::W) writer structure"]
impl crate::Writable for TCD15_ATTR {}
#[doc = "TCD Transfer Attributes"]
pub mod tcd15_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd15_nbytes_mlno](tcd15_nbytes_mlno) module"]
pub type TCD15_NBYTES_MLNO = crate::Reg<u32, _TCD15_NBYTES_MLNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD15_NBYTES_MLNO;
#[doc = "`read()` method returns [tcd15_nbytes_mlno::R](tcd15_nbytes_mlno::R) reader structure"]
impl crate::Readable for TCD15_NBYTES_MLNO {}
#[doc = "`write(|w| ..)` method takes [tcd15_nbytes_mlno::W](tcd15_nbytes_mlno::W) writer structure"]
impl crate::Writable for TCD15_NBYTES_MLNO {}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd15_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd15_nbytes_mloffno](tcd15_nbytes_mloffno) module"]
pub type TCD15_NBYTES_MLOFFNO = crate::Reg<u32, _TCD15_NBYTES_MLOFFNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD15_NBYTES_MLOFFNO;
#[doc = "`read()` method returns [tcd15_nbytes_mloffno::R](tcd15_nbytes_mloffno::R) reader structure"]
impl crate::Readable for TCD15_NBYTES_MLOFFNO {}
#[doc = "`write(|w| ..)` method takes [tcd15_nbytes_mloffno::W](tcd15_nbytes_mloffno::W) writer structure"]
impl crate::Writable for TCD15_NBYTES_MLOFFNO {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd15_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd15_nbytes_mloffyes](tcd15_nbytes_mloffyes) module"]
pub type TCD15_NBYTES_MLOFFYES = crate::Reg<u32, _TCD15_NBYTES_MLOFFYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD15_NBYTES_MLOFFYES;
#[doc = "`read()` method returns [tcd15_nbytes_mloffyes::R](tcd15_nbytes_mloffyes::R) reader structure"]
impl crate::Readable for TCD15_NBYTES_MLOFFYES {}
#[doc = "`write(|w| ..)` method takes [tcd15_nbytes_mloffyes::W](tcd15_nbytes_mloffyes::W) writer structure"]
impl crate::Writable for TCD15_NBYTES_MLOFFYES {}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd15_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd15_slast](tcd15_slast) module"]
pub type TCD15_SLAST = crate::Reg<u32, _TCD15_SLAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD15_SLAST;
#[doc = "`read()` method returns [tcd15_slast::R](tcd15_slast::R) reader structure"]
impl crate::Readable for TCD15_SLAST {}
#[doc = "`write(|w| ..)` method takes [tcd15_slast::W](tcd15_slast::W) writer structure"]
impl crate::Writable for TCD15_SLAST {}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd15_slast;
#[doc = "TCD Destination Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd15_daddr](tcd15_daddr) module"]
pub type TCD15_DADDR = crate::Reg<u32, _TCD15_DADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD15_DADDR;
#[doc = "`read()` method returns [tcd15_daddr::R](tcd15_daddr::R) reader structure"]
impl crate::Readable for TCD15_DADDR {}
#[doc = "`write(|w| ..)` method takes [tcd15_daddr::W](tcd15_daddr::W) writer structure"]
impl crate::Writable for TCD15_DADDR {}
#[doc = "TCD Destination Address"]
pub mod tcd15_daddr;
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd15_doff](tcd15_doff) module"]
pub type TCD15_DOFF = crate::Reg<u16, _TCD15_DOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD15_DOFF;
#[doc = "`read()` method returns [tcd15_doff::R](tcd15_doff::R) reader structure"]
impl crate::Readable for TCD15_DOFF {}
#[doc = "`write(|w| ..)` method takes [tcd15_doff::W](tcd15_doff::W) writer structure"]
impl crate::Writable for TCD15_DOFF {}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd15_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd15_citer_elinkno](tcd15_citer_elinkno) module"]
pub type TCD15_CITER_ELINKNO = crate::Reg<u16, _TCD15_CITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD15_CITER_ELINKNO;
#[doc = "`read()` method returns [tcd15_citer_elinkno::R](tcd15_citer_elinkno::R) reader structure"]
impl crate::Readable for TCD15_CITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd15_citer_elinkno::W](tcd15_citer_elinkno::W) writer structure"]
impl crate::Writable for TCD15_CITER_ELINKNO {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd15_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd15_citer_elinkyes](tcd15_citer_elinkyes) module"]
pub type TCD15_CITER_ELINKYES = crate::Reg<u16, _TCD15_CITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD15_CITER_ELINKYES;
#[doc = "`read()` method returns [tcd15_citer_elinkyes::R](tcd15_citer_elinkyes::R) reader structure"]
impl crate::Readable for TCD15_CITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd15_citer_elinkyes::W](tcd15_citer_elinkyes::W) writer structure"]
impl crate::Writable for TCD15_CITER_ELINKYES {}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd15_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd15_dlastsga](tcd15_dlastsga) module"]
pub type TCD15_DLASTSGA = crate::Reg<u32, _TCD15_DLASTSGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD15_DLASTSGA;
#[doc = "`read()` method returns [tcd15_dlastsga::R](tcd15_dlastsga::R) reader structure"]
impl crate::Readable for TCD15_DLASTSGA {}
#[doc = "`write(|w| ..)` method takes [tcd15_dlastsga::W](tcd15_dlastsga::W) writer structure"]
impl crate::Writable for TCD15_DLASTSGA {}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd15_dlastsga;
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd15_csr](tcd15_csr) module"]
pub type TCD15_CSR = crate::Reg<u16, _TCD15_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD15_CSR;
#[doc = "`read()` method returns [tcd15_csr::R](tcd15_csr::R) reader structure"]
impl crate::Readable for TCD15_CSR {}
#[doc = "`write(|w| ..)` method takes [tcd15_csr::W](tcd15_csr::W) writer structure"]
impl crate::Writable for TCD15_CSR {}
#[doc = "TCD Control and Status"]
pub mod tcd15_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd15_biter_elinkno](tcd15_biter_elinkno) module"]
pub type TCD15_BITER_ELINKNO = crate::Reg<u16, _TCD15_BITER_ELINKNO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD15_BITER_ELINKNO;
#[doc = "`read()` method returns [tcd15_biter_elinkno::R](tcd15_biter_elinkno::R) reader structure"]
impl crate::Readable for TCD15_BITER_ELINKNO {}
#[doc = "`write(|w| ..)` method takes [tcd15_biter_elinkno::W](tcd15_biter_elinkno::W) writer structure"]
impl crate::Writable for TCD15_BITER_ELINKNO {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd15_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd15_biter_elinkyes](tcd15_biter_elinkyes) module"]
pub type TCD15_BITER_ELINKYES = crate::Reg<u16, _TCD15_BITER_ELINKYES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCD15_BITER_ELINKYES;
#[doc = "`read()` method returns [tcd15_biter_elinkyes::R](tcd15_biter_elinkyes::R) reader structure"]
impl crate::Readable for TCD15_BITER_ELINKYES {}
#[doc = "`write(|w| ..)` method takes [tcd15_biter_elinkyes::W](tcd15_biter_elinkyes::W) writer structure"]
impl crate::Writable for TCD15_BITER_ELINKYES {}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd15_biter_elinkyes;
