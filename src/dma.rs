#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Error Status Register"]
    pub es: ES,
    _reserved0: [u8; 4usize],
    #[doc = "0x0c - Enable Request Register"]
    pub erq: ERQ,
    _reserved1: [u8; 4usize],
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
    _reserved2: [u8; 4usize],
    #[doc = "0x24 - Interrupt Request Register"]
    pub int: INT,
    _reserved3: [u8; 4usize],
    #[doc = "0x2c - Error Register"]
    pub err: ERR,
    _reserved4: [u8; 4usize],
    #[doc = "0x34 - Hardware Request Status Register"]
    pub hrs: HRS,
    _reserved5: [u8; 12usize],
    #[doc = "0x44 - Enable Asynchronous Request in Stop Register"]
    pub ears: EARS,
    _reserved6: [u8; 184usize],
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
    _reserved7: [u8; 3824usize],
    #[doc = "0x1000 - TCD Source Address"]
    pub tcd0_saddr: TCD0_SADDR,
    #[doc = "0x1004 - TCD Signed Source Address Offset"]
    pub tcd0_soff: TCD0_SOFF,
    #[doc = "0x1006 - TCD Transfer Attributes"]
    pub tcd0_attr: TCD0_ATTR,
    #[doc = "0x1008 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd0_nbytes_mlno: TCD0_NBYTES_MLNO,
    #[doc = "0x100c - TCD Last Source Address Adjustment"]
    pub tcd0_slast: TCD0_SLAST,
    #[doc = "0x1010 - TCD Destination Address"]
    pub tcd0_daddr: TCD0_DADDR,
    #[doc = "0x1014 - TCD Signed Destination Address Offset"]
    pub tcd0_doff: TCD0_DOFF,
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd0_citer_elinkno: TCD0_CITER_ELINKNO,
    #[doc = "0x1018 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd0_dlastsga: TCD0_DLASTSGA,
    #[doc = "0x101c - TCD Control and Status"]
    pub tcd0_csr: TCD0_CSR,
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd0_biter_elinkno: TCD0_BITER_ELINKNO,
    #[doc = "0x1020 - TCD Source Address"]
    pub tcd1_saddr: TCD1_SADDR,
    #[doc = "0x1024 - TCD Signed Source Address Offset"]
    pub tcd1_soff: TCD1_SOFF,
    #[doc = "0x1026 - TCD Transfer Attributes"]
    pub tcd1_attr: TCD1_ATTR,
    #[doc = "0x1028 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd1_nbytes_mlno: TCD1_NBYTES_MLNO,
    #[doc = "0x102c - TCD Last Source Address Adjustment"]
    pub tcd1_slast: TCD1_SLAST,
    #[doc = "0x1030 - TCD Destination Address"]
    pub tcd1_daddr: TCD1_DADDR,
    #[doc = "0x1034 - TCD Signed Destination Address Offset"]
    pub tcd1_doff: TCD1_DOFF,
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd1_citer_elinkno: TCD1_CITER_ELINKNO,
    #[doc = "0x1038 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd1_dlastsga: TCD1_DLASTSGA,
    #[doc = "0x103c - TCD Control and Status"]
    pub tcd1_csr: TCD1_CSR,
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd1_biter_elinkno: TCD1_BITER_ELINKNO,
    #[doc = "0x1040 - TCD Source Address"]
    pub tcd2_saddr: TCD2_SADDR,
    #[doc = "0x1044 - TCD Signed Source Address Offset"]
    pub tcd2_soff: TCD2_SOFF,
    #[doc = "0x1046 - TCD Transfer Attributes"]
    pub tcd2_attr: TCD2_ATTR,
    #[doc = "0x1048 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd2_nbytes_mlno: TCD2_NBYTES_MLNO,
    #[doc = "0x104c - TCD Last Source Address Adjustment"]
    pub tcd2_slast: TCD2_SLAST,
    #[doc = "0x1050 - TCD Destination Address"]
    pub tcd2_daddr: TCD2_DADDR,
    #[doc = "0x1054 - TCD Signed Destination Address Offset"]
    pub tcd2_doff: TCD2_DOFF,
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd2_citer_elinkno: TCD2_CITER_ELINKNO,
    #[doc = "0x1058 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd2_dlastsga: TCD2_DLASTSGA,
    #[doc = "0x105c - TCD Control and Status"]
    pub tcd2_csr: TCD2_CSR,
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd2_biter_elinkno: TCD2_BITER_ELINKNO,
    #[doc = "0x1060 - TCD Source Address"]
    pub tcd3_saddr: TCD3_SADDR,
    #[doc = "0x1064 - TCD Signed Source Address Offset"]
    pub tcd3_soff: TCD3_SOFF,
    #[doc = "0x1066 - TCD Transfer Attributes"]
    pub tcd3_attr: TCD3_ATTR,
    #[doc = "0x1068 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd3_nbytes_mlno: TCD3_NBYTES_MLNO,
    #[doc = "0x106c - TCD Last Source Address Adjustment"]
    pub tcd3_slast: TCD3_SLAST,
    #[doc = "0x1070 - TCD Destination Address"]
    pub tcd3_daddr: TCD3_DADDR,
    #[doc = "0x1074 - TCD Signed Destination Address Offset"]
    pub tcd3_doff: TCD3_DOFF,
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd3_citer_elinkno: TCD3_CITER_ELINKNO,
    #[doc = "0x1078 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd3_dlastsga: TCD3_DLASTSGA,
    #[doc = "0x107c - TCD Control and Status"]
    pub tcd3_csr: TCD3_CSR,
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd3_biter_elinkno: TCD3_BITER_ELINKNO,
    #[doc = "0x1080 - TCD Source Address"]
    pub tcd4_saddr: TCD4_SADDR,
    #[doc = "0x1084 - TCD Signed Source Address Offset"]
    pub tcd4_soff: TCD4_SOFF,
    #[doc = "0x1086 - TCD Transfer Attributes"]
    pub tcd4_attr: TCD4_ATTR,
    #[doc = "0x1088 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd4_nbytes_mlno: TCD4_NBYTES_MLNO,
    #[doc = "0x108c - TCD Last Source Address Adjustment"]
    pub tcd4_slast: TCD4_SLAST,
    #[doc = "0x1090 - TCD Destination Address"]
    pub tcd4_daddr: TCD4_DADDR,
    #[doc = "0x1094 - TCD Signed Destination Address Offset"]
    pub tcd4_doff: TCD4_DOFF,
    #[doc = "0x1096 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd4_citer_elinkno: TCD4_CITER_ELINKNO,
    #[doc = "0x1098 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd4_dlastsga: TCD4_DLASTSGA,
    #[doc = "0x109c - TCD Control and Status"]
    pub tcd4_csr: TCD4_CSR,
    #[doc = "0x109e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd4_biter_elinkno: TCD4_BITER_ELINKNO,
    #[doc = "0x10a0 - TCD Source Address"]
    pub tcd5_saddr: TCD5_SADDR,
    #[doc = "0x10a4 - TCD Signed Source Address Offset"]
    pub tcd5_soff: TCD5_SOFF,
    #[doc = "0x10a6 - TCD Transfer Attributes"]
    pub tcd5_attr: TCD5_ATTR,
    #[doc = "0x10a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd5_nbytes_mlno: TCD5_NBYTES_MLNO,
    #[doc = "0x10ac - TCD Last Source Address Adjustment"]
    pub tcd5_slast: TCD5_SLAST,
    #[doc = "0x10b0 - TCD Destination Address"]
    pub tcd5_daddr: TCD5_DADDR,
    #[doc = "0x10b4 - TCD Signed Destination Address Offset"]
    pub tcd5_doff: TCD5_DOFF,
    #[doc = "0x10b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd5_citer_elinkno: TCD5_CITER_ELINKNO,
    #[doc = "0x10b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd5_dlastsga: TCD5_DLASTSGA,
    #[doc = "0x10bc - TCD Control and Status"]
    pub tcd5_csr: TCD5_CSR,
    #[doc = "0x10be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd5_biter_elinkno: TCD5_BITER_ELINKNO,
    #[doc = "0x10c0 - TCD Source Address"]
    pub tcd6_saddr: TCD6_SADDR,
    #[doc = "0x10c4 - TCD Signed Source Address Offset"]
    pub tcd6_soff: TCD6_SOFF,
    #[doc = "0x10c6 - TCD Transfer Attributes"]
    pub tcd6_attr: TCD6_ATTR,
    #[doc = "0x10c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd6_nbytes_mlno: TCD6_NBYTES_MLNO,
    #[doc = "0x10cc - TCD Last Source Address Adjustment"]
    pub tcd6_slast: TCD6_SLAST,
    #[doc = "0x10d0 - TCD Destination Address"]
    pub tcd6_daddr: TCD6_DADDR,
    #[doc = "0x10d4 - TCD Signed Destination Address Offset"]
    pub tcd6_doff: TCD6_DOFF,
    #[doc = "0x10d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd6_citer_elinkno: TCD6_CITER_ELINKNO,
    #[doc = "0x10d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd6_dlastsga: TCD6_DLASTSGA,
    #[doc = "0x10dc - TCD Control and Status"]
    pub tcd6_csr: TCD6_CSR,
    #[doc = "0x10de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd6_biter_elinkno: TCD6_BITER_ELINKNO,
    #[doc = "0x10e0 - TCD Source Address"]
    pub tcd7_saddr: TCD7_SADDR,
    #[doc = "0x10e4 - TCD Signed Source Address Offset"]
    pub tcd7_soff: TCD7_SOFF,
    #[doc = "0x10e6 - TCD Transfer Attributes"]
    pub tcd7_attr: TCD7_ATTR,
    #[doc = "0x10e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd7_nbytes_mlno: TCD7_NBYTES_MLNO,
    #[doc = "0x10ec - TCD Last Source Address Adjustment"]
    pub tcd7_slast: TCD7_SLAST,
    #[doc = "0x10f0 - TCD Destination Address"]
    pub tcd7_daddr: TCD7_DADDR,
    #[doc = "0x10f4 - TCD Signed Destination Address Offset"]
    pub tcd7_doff: TCD7_DOFF,
    #[doc = "0x10f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd7_citer_elinkno: TCD7_CITER_ELINKNO,
    #[doc = "0x10f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd7_dlastsga: TCD7_DLASTSGA,
    #[doc = "0x10fc - TCD Control and Status"]
    pub tcd7_csr: TCD7_CSR,
    #[doc = "0x10fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd7_biter_elinkno: TCD7_BITER_ELINKNO,
    #[doc = "0x1100 - TCD Source Address"]
    pub tcd8_saddr: TCD8_SADDR,
    #[doc = "0x1104 - TCD Signed Source Address Offset"]
    pub tcd8_soff: TCD8_SOFF,
    #[doc = "0x1106 - TCD Transfer Attributes"]
    pub tcd8_attr: TCD8_ATTR,
    #[doc = "0x1108 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd8_nbytes_mlno: TCD8_NBYTES_MLNO,
    #[doc = "0x110c - TCD Last Source Address Adjustment"]
    pub tcd8_slast: TCD8_SLAST,
    #[doc = "0x1110 - TCD Destination Address"]
    pub tcd8_daddr: TCD8_DADDR,
    #[doc = "0x1114 - TCD Signed Destination Address Offset"]
    pub tcd8_doff: TCD8_DOFF,
    #[doc = "0x1116 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd8_citer_elinkno: TCD8_CITER_ELINKNO,
    #[doc = "0x1118 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd8_dlastsga: TCD8_DLASTSGA,
    #[doc = "0x111c - TCD Control and Status"]
    pub tcd8_csr: TCD8_CSR,
    #[doc = "0x111e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd8_biter_elinkno: TCD8_BITER_ELINKNO,
    #[doc = "0x1120 - TCD Source Address"]
    pub tcd9_saddr: TCD9_SADDR,
    #[doc = "0x1124 - TCD Signed Source Address Offset"]
    pub tcd9_soff: TCD9_SOFF,
    #[doc = "0x1126 - TCD Transfer Attributes"]
    pub tcd9_attr: TCD9_ATTR,
    #[doc = "0x1128 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd9_nbytes_mlno: TCD9_NBYTES_MLNO,
    #[doc = "0x112c - TCD Last Source Address Adjustment"]
    pub tcd9_slast: TCD9_SLAST,
    #[doc = "0x1130 - TCD Destination Address"]
    pub tcd9_daddr: TCD9_DADDR,
    #[doc = "0x1134 - TCD Signed Destination Address Offset"]
    pub tcd9_doff: TCD9_DOFF,
    #[doc = "0x1136 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd9_citer_elinkno: TCD9_CITER_ELINKNO,
    #[doc = "0x1138 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd9_dlastsga: TCD9_DLASTSGA,
    #[doc = "0x113c - TCD Control and Status"]
    pub tcd9_csr: TCD9_CSR,
    #[doc = "0x113e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd9_biter_elinkno: TCD9_BITER_ELINKNO,
    #[doc = "0x1140 - TCD Source Address"]
    pub tcd10_saddr: TCD10_SADDR,
    #[doc = "0x1144 - TCD Signed Source Address Offset"]
    pub tcd10_soff: TCD10_SOFF,
    #[doc = "0x1146 - TCD Transfer Attributes"]
    pub tcd10_attr: TCD10_ATTR,
    #[doc = "0x1148 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd10_nbytes_mlno: TCD10_NBYTES_MLNO,
    #[doc = "0x114c - TCD Last Source Address Adjustment"]
    pub tcd10_slast: TCD10_SLAST,
    #[doc = "0x1150 - TCD Destination Address"]
    pub tcd10_daddr: TCD10_DADDR,
    #[doc = "0x1154 - TCD Signed Destination Address Offset"]
    pub tcd10_doff: TCD10_DOFF,
    #[doc = "0x1156 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd10_citer_elinkno: TCD10_CITER_ELINKNO,
    #[doc = "0x1158 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd10_dlastsga: TCD10_DLASTSGA,
    #[doc = "0x115c - TCD Control and Status"]
    pub tcd10_csr: TCD10_CSR,
    #[doc = "0x115e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd10_biter_elinkno: TCD10_BITER_ELINKNO,
    #[doc = "0x1160 - TCD Source Address"]
    pub tcd11_saddr: TCD11_SADDR,
    #[doc = "0x1164 - TCD Signed Source Address Offset"]
    pub tcd11_soff: TCD11_SOFF,
    #[doc = "0x1166 - TCD Transfer Attributes"]
    pub tcd11_attr: TCD11_ATTR,
    #[doc = "0x1168 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd11_nbytes_mlno: TCD11_NBYTES_MLNO,
    #[doc = "0x116c - TCD Last Source Address Adjustment"]
    pub tcd11_slast: TCD11_SLAST,
    #[doc = "0x1170 - TCD Destination Address"]
    pub tcd11_daddr: TCD11_DADDR,
    #[doc = "0x1174 - TCD Signed Destination Address Offset"]
    pub tcd11_doff: TCD11_DOFF,
    #[doc = "0x1176 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd11_citer_elinkno: TCD11_CITER_ELINKNO,
    #[doc = "0x1178 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd11_dlastsga: TCD11_DLASTSGA,
    #[doc = "0x117c - TCD Control and Status"]
    pub tcd11_csr: TCD11_CSR,
    #[doc = "0x117e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd11_biter_elinkno: TCD11_BITER_ELINKNO,
    #[doc = "0x1180 - TCD Source Address"]
    pub tcd12_saddr: TCD12_SADDR,
    #[doc = "0x1184 - TCD Signed Source Address Offset"]
    pub tcd12_soff: TCD12_SOFF,
    #[doc = "0x1186 - TCD Transfer Attributes"]
    pub tcd12_attr: TCD12_ATTR,
    #[doc = "0x1188 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd12_nbytes_mlno: TCD12_NBYTES_MLNO,
    #[doc = "0x118c - TCD Last Source Address Adjustment"]
    pub tcd12_slast: TCD12_SLAST,
    #[doc = "0x1190 - TCD Destination Address"]
    pub tcd12_daddr: TCD12_DADDR,
    #[doc = "0x1194 - TCD Signed Destination Address Offset"]
    pub tcd12_doff: TCD12_DOFF,
    #[doc = "0x1196 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd12_citer_elinkno: TCD12_CITER_ELINKNO,
    #[doc = "0x1198 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd12_dlastsga: TCD12_DLASTSGA,
    #[doc = "0x119c - TCD Control and Status"]
    pub tcd12_csr: TCD12_CSR,
    #[doc = "0x119e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd12_biter_elinkno: TCD12_BITER_ELINKNO,
    #[doc = "0x11a0 - TCD Source Address"]
    pub tcd13_saddr: TCD13_SADDR,
    #[doc = "0x11a4 - TCD Signed Source Address Offset"]
    pub tcd13_soff: TCD13_SOFF,
    #[doc = "0x11a6 - TCD Transfer Attributes"]
    pub tcd13_attr: TCD13_ATTR,
    #[doc = "0x11a8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd13_nbytes_mlno: TCD13_NBYTES_MLNO,
    #[doc = "0x11ac - TCD Last Source Address Adjustment"]
    pub tcd13_slast: TCD13_SLAST,
    #[doc = "0x11b0 - TCD Destination Address"]
    pub tcd13_daddr: TCD13_DADDR,
    #[doc = "0x11b4 - TCD Signed Destination Address Offset"]
    pub tcd13_doff: TCD13_DOFF,
    #[doc = "0x11b6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd13_citer_elinkno: TCD13_CITER_ELINKNO,
    #[doc = "0x11b8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd13_dlastsga: TCD13_DLASTSGA,
    #[doc = "0x11bc - TCD Control and Status"]
    pub tcd13_csr: TCD13_CSR,
    #[doc = "0x11be - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd13_biter_elinkno: TCD13_BITER_ELINKNO,
    #[doc = "0x11c0 - TCD Source Address"]
    pub tcd14_saddr: TCD14_SADDR,
    #[doc = "0x11c4 - TCD Signed Source Address Offset"]
    pub tcd14_soff: TCD14_SOFF,
    #[doc = "0x11c6 - TCD Transfer Attributes"]
    pub tcd14_attr: TCD14_ATTR,
    #[doc = "0x11c8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd14_nbytes_mlno: TCD14_NBYTES_MLNO,
    #[doc = "0x11cc - TCD Last Source Address Adjustment"]
    pub tcd14_slast: TCD14_SLAST,
    #[doc = "0x11d0 - TCD Destination Address"]
    pub tcd14_daddr: TCD14_DADDR,
    #[doc = "0x11d4 - TCD Signed Destination Address Offset"]
    pub tcd14_doff: TCD14_DOFF,
    #[doc = "0x11d6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd14_citer_elinkno: TCD14_CITER_ELINKNO,
    #[doc = "0x11d8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd14_dlastsga: TCD14_DLASTSGA,
    #[doc = "0x11dc - TCD Control and Status"]
    pub tcd14_csr: TCD14_CSR,
    #[doc = "0x11de - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd14_biter_elinkno: TCD14_BITER_ELINKNO,
    #[doc = "0x11e0 - TCD Source Address"]
    pub tcd15_saddr: TCD15_SADDR,
    #[doc = "0x11e4 - TCD Signed Source Address Offset"]
    pub tcd15_soff: TCD15_SOFF,
    #[doc = "0x11e6 - TCD Transfer Attributes"]
    pub tcd15_attr: TCD15_ATTR,
    #[doc = "0x11e8 - TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
    pub tcd15_nbytes_mlno: TCD15_NBYTES_MLNO,
    #[doc = "0x11ec - TCD Last Source Address Adjustment"]
    pub tcd15_slast: TCD15_SLAST,
    #[doc = "0x11f0 - TCD Destination Address"]
    pub tcd15_daddr: TCD15_DADDR,
    #[doc = "0x11f4 - TCD Signed Destination Address Offset"]
    pub tcd15_doff: TCD15_DOFF,
    #[doc = "0x11f6 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd15_citer_elinkno: TCD15_CITER_ELINKNO,
    #[doc = "0x11f8 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd15_dlastsga: TCD15_DLASTSGA,
    #[doc = "0x11fc - TCD Control and Status"]
    pub tcd15_csr: TCD15_CSR,
    #[doc = "0x11fe - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    pub tcd15_biter_elinkno: TCD15_BITER_ELINKNO,
}
#[doc = "Control Register"]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Control Register"]
pub mod cr;
#[doc = "Error Status Register"]
pub struct ES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Status Register"]
pub mod es;
#[doc = "Enable Request Register"]
pub struct ERQ {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Request Register"]
pub mod erq;
#[doc = "Enable Error Interrupt Register"]
pub struct EEI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Error Interrupt Register"]
pub mod eei;
#[doc = "Clear Enable Error Interrupt Register"]
pub struct CEEI {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Enable Error Interrupt Register"]
pub mod ceei;
#[doc = "Set Enable Error Interrupt Register"]
pub struct SEEI {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Set Enable Error Interrupt Register"]
pub mod seei;
#[doc = "Clear Enable Request Register"]
pub struct CERQ {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Enable Request Register"]
pub mod cerq;
#[doc = "Set Enable Request Register"]
pub struct SERQ {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Set Enable Request Register"]
pub mod serq;
#[doc = "Clear DONE Status Bit Register"]
pub struct CDNE {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear DONE Status Bit Register"]
pub mod cdne;
#[doc = "Set START Bit Register"]
pub struct SSRT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Set START Bit Register"]
pub mod ssrt;
#[doc = "Clear Error Register"]
pub struct CERR {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Error Register"]
pub mod cerr;
#[doc = "Clear Interrupt Request Register"]
pub struct CINT {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Clear Interrupt Request Register"]
pub mod cint;
#[doc = "Interrupt Request Register"]
pub struct INT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Request Register"]
pub mod int;
#[doc = "Error Register"]
pub struct ERR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Error Register"]
pub mod err;
#[doc = "Hardware Request Status Register"]
pub struct HRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hardware Request Status Register"]
pub mod hrs;
#[doc = "Enable Asynchronous Request in Stop Register"]
pub struct EARS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable Asynchronous Request in Stop Register"]
pub mod ears;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI3 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri3;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI2 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri2;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI1 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri1;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI0 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri0;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI7 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri7;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI6 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri6;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI5 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri5;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI4 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri4;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI11 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri11;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI10 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri10;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI9 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri9;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI8 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri8;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI15 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri15;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI14 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri14;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI13 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri13;
#[doc = "Channel n Priority Register"]
pub struct DCHPRI12 {
    register: ::vcell::VolatileCell<u8>,
}
#[doc = "Channel n Priority Register"]
pub mod dchpri12;
#[doc = "TCD Source Address"]
pub struct TCD0_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd0_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD0_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd0_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD0_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd0_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub struct TCD0_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd0_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub struct TCD0_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd0_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub struct TCD0_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd0_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD0_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd0_slast;
#[doc = "TCD Destination Address"]
pub struct TCD0_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd0_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD0_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd0_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD0_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd0_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD0_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd0_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD0_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd0_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD0_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd0_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD0_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd0_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD0_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd0_biter_elinkyes;
#[doc = "TCD Source Address"]
pub struct TCD1_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd1_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD1_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd1_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD1_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd1_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub struct TCD1_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd1_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub struct TCD1_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd1_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub struct TCD1_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd1_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD1_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd1_slast;
#[doc = "TCD Destination Address"]
pub struct TCD1_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd1_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD1_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd1_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD1_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd1_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD1_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd1_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD1_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd1_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD1_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd1_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD1_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd1_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD1_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd1_biter_elinkyes;
#[doc = "TCD Source Address"]
pub struct TCD2_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd2_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD2_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd2_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD2_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd2_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub struct TCD2_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd2_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub struct TCD2_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd2_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub struct TCD2_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd2_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD2_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd2_slast;
#[doc = "TCD Destination Address"]
pub struct TCD2_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd2_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD2_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd2_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD2_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd2_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD2_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd2_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD2_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd2_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD2_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd2_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD2_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd2_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD2_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd2_biter_elinkyes;
#[doc = "TCD Source Address"]
pub struct TCD3_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd3_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD3_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd3_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD3_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd3_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub struct TCD3_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd3_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub struct TCD3_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd3_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub struct TCD3_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd3_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD3_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd3_slast;
#[doc = "TCD Destination Address"]
pub struct TCD3_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd3_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD3_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd3_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD3_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd3_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD3_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd3_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD3_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd3_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD3_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd3_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD3_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd3_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD3_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd3_biter_elinkyes;
#[doc = "TCD Source Address"]
pub struct TCD4_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd4_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD4_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd4_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD4_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd4_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub struct TCD4_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd4_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub struct TCD4_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd4_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub struct TCD4_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd4_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD4_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd4_slast;
#[doc = "TCD Destination Address"]
pub struct TCD4_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd4_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD4_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd4_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD4_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd4_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD4_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd4_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD4_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd4_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD4_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd4_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD4_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd4_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD4_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd4_biter_elinkyes;
#[doc = "TCD Source Address"]
pub struct TCD5_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd5_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD5_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd5_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD5_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd5_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub struct TCD5_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd5_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub struct TCD5_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd5_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub struct TCD5_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd5_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD5_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd5_slast;
#[doc = "TCD Destination Address"]
pub struct TCD5_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd5_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD5_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd5_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD5_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd5_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD5_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd5_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD5_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd5_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD5_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd5_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD5_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd5_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD5_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd5_biter_elinkyes;
#[doc = "TCD Source Address"]
pub struct TCD6_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd6_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD6_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd6_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD6_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd6_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub struct TCD6_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd6_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub struct TCD6_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd6_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub struct TCD6_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd6_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD6_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd6_slast;
#[doc = "TCD Destination Address"]
pub struct TCD6_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd6_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD6_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd6_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD6_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd6_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD6_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd6_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD6_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd6_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD6_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd6_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD6_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd6_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD6_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd6_biter_elinkyes;
#[doc = "TCD Source Address"]
pub struct TCD7_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd7_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD7_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd7_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD7_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd7_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub struct TCD7_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd7_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub struct TCD7_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd7_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub struct TCD7_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd7_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD7_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd7_slast;
#[doc = "TCD Destination Address"]
pub struct TCD7_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd7_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD7_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd7_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD7_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd7_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD7_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd7_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD7_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd7_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD7_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd7_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD7_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd7_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD7_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd7_biter_elinkyes;
#[doc = "TCD Source Address"]
pub struct TCD8_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd8_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD8_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd8_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD8_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd8_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub struct TCD8_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd8_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub struct TCD8_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd8_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub struct TCD8_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd8_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD8_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd8_slast;
#[doc = "TCD Destination Address"]
pub struct TCD8_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd8_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD8_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd8_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD8_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd8_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD8_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd8_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD8_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd8_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD8_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd8_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD8_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd8_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD8_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd8_biter_elinkyes;
#[doc = "TCD Source Address"]
pub struct TCD9_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd9_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD9_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd9_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD9_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd9_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub struct TCD9_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd9_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub struct TCD9_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd9_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub struct TCD9_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd9_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD9_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd9_slast;
#[doc = "TCD Destination Address"]
pub struct TCD9_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd9_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD9_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd9_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD9_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd9_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD9_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd9_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD9_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd9_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD9_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd9_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD9_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd9_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD9_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd9_biter_elinkyes;
#[doc = "TCD Source Address"]
pub struct TCD10_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd10_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD10_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd10_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD10_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd10_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub struct TCD10_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd10_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub struct TCD10_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd10_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub struct TCD10_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd10_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD10_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd10_slast;
#[doc = "TCD Destination Address"]
pub struct TCD10_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd10_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD10_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd10_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD10_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd10_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD10_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd10_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD10_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd10_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD10_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd10_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD10_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd10_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD10_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd10_biter_elinkyes;
#[doc = "TCD Source Address"]
pub struct TCD11_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd11_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD11_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd11_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD11_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd11_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub struct TCD11_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd11_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub struct TCD11_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd11_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub struct TCD11_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd11_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD11_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd11_slast;
#[doc = "TCD Destination Address"]
pub struct TCD11_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd11_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD11_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd11_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD11_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd11_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD11_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd11_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD11_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd11_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD11_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd11_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD11_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd11_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD11_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd11_biter_elinkyes;
#[doc = "TCD Source Address"]
pub struct TCD12_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd12_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD12_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd12_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD12_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd12_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub struct TCD12_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd12_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub struct TCD12_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd12_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub struct TCD12_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd12_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD12_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd12_slast;
#[doc = "TCD Destination Address"]
pub struct TCD12_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd12_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD12_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd12_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD12_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd12_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD12_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd12_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD12_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd12_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD12_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd12_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD12_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd12_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD12_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd12_biter_elinkyes;
#[doc = "TCD Source Address"]
pub struct TCD13_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd13_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD13_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd13_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD13_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd13_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub struct TCD13_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd13_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub struct TCD13_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd13_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub struct TCD13_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd13_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD13_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd13_slast;
#[doc = "TCD Destination Address"]
pub struct TCD13_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd13_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD13_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd13_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD13_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd13_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD13_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd13_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD13_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd13_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD13_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd13_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD13_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd13_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD13_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd13_biter_elinkyes;
#[doc = "TCD Source Address"]
pub struct TCD14_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd14_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD14_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd14_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD14_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd14_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub struct TCD14_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd14_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub struct TCD14_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd14_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub struct TCD14_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd14_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD14_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd14_slast;
#[doc = "TCD Destination Address"]
pub struct TCD14_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd14_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD14_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd14_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD14_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd14_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD14_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd14_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD14_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd14_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD14_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd14_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD14_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd14_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD14_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd14_biter_elinkyes;
#[doc = "TCD Source Address"]
pub struct TCD15_SADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Source Address"]
pub mod tcd15_saddr;
#[doc = "TCD Signed Source Address Offset"]
pub struct TCD15_SOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd15_soff;
#[doc = "TCD Transfer Attributes"]
pub struct TCD15_ATTR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Transfer Attributes"]
pub mod tcd15_attr;
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub struct TCD15_NBYTES_MLNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Minor Byte Count (Minor Loop Mapping Disabled)"]
pub mod tcd15_nbytes_mlno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub struct TCD15_NBYTES_MLOFFNO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping Enabled and Offset Disabled)"]
pub mod tcd15_nbytes_mloffno;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub struct TCD15_NBYTES_MLOFFYES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Mapping and Offset Enabled)"]
pub mod tcd15_nbytes_mloffyes;
#[doc = "TCD Last Source Address Adjustment"]
pub struct TCD15_SLAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd15_slast;
#[doc = "TCD Destination Address"]
pub struct TCD15_DADDR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Destination Address"]
pub mod tcd15_daddr;
#[doc = "TCD Signed Destination Address Offset"]
pub struct TCD15_DOFF {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd15_doff;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD15_CITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd15_citer_elinkno;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD15_CITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd15_citer_elinkyes;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub struct TCD15_DLASTSGA {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd15_dlastsga;
#[doc = "TCD Control and Status"]
pub struct TCD15_CSR {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Control and Status"]
pub mod tcd15_csr;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub struct TCD15_BITER_ELINKNO {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod tcd15_biter_elinkno;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub struct TCD15_BITER_ELINKYES {
    register: ::vcell::VolatileCell<u16>,
}
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod tcd15_biter_elinkyes;
