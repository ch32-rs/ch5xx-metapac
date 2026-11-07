use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "SystemControl",
            extends: None,
            description: Some(
                "System control related registers.",
            ),
            items: &[
                BlockItem {
                    name: "safe_access_sig",
                    description: Some(
                        "WO, safe accessing sign register, must write SAFE_ACCESS_SIG1 then SAFE_ACCESS_SIG2 to enter safe accessing mode.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 8,
                            fieldset: Some(
                                "SafeAccessSig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "chip_id",
                    description: Some(
                        "RF, chip ID register, always is ID_CH57*.",
                    ),
                    array: None,
                    byte_offset: 0x41,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "ChipId",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "safe_access_id",
                    description: Some(
                        "RF, safe accessing ID register, always 0x0C.",
                    ),
                    array: None,
                    byte_offset: 0x42,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "SafeAccessId",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdog_count",
                    description: Some(
                        "RW, watch-dog count, count by clock frequency Fsys/131072.",
                    ),
                    array: None,
                    byte_offset: 0x43,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "WdogCount",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "glob_rom_cfg",
                    description: Some(
                        "RWA, flash ROM configuration.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "GlobRomCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "reset_status",
                    description: Some(
                        "RO, reset status.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "ResetStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "glob_cfg_info",
                    description: Some(
                        "RO, global configuration information and status.",
                    ),
                    array: None,
                    byte_offset: 0x45,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "GlobCfgInfo",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rst_wdog_ctrl",
                    description: Some(
                        "RWA, reset and watch-dog control, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x46,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "RstWdogCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "glob_reset_keep",
                    description: Some(
                        "RW, value keeper during global reset.",
                    ),
                    array: None,
                    byte_offset: 0x47,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "GlobResetKeep",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll_config",
                    description: Some(
                        "RWA, PLL configuration control, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x4b,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "PllConfig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "xt32m_tune",
                    description: Some(
                        "RWA, external 32MHz oscillator tune control, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x4e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Xt32mTune",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "osc_cal_cnt",
                    description: Some(
                        "RO, system clock count value for 32KHz multi-cycles.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "OscCalCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "osc_cal_ov_cnt",
                    description: Some(
                        "RO, oscillator frequency calibration overflow times.",
                    ),
                    array: None,
                    byte_offset: 0x52,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "OscCalOvCnt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "osc_cal_ctrl",
                    description: Some(
                        "RWA, oscillator frequency calibration control, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x53,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "OscCalCtrl",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "ChipId",
            extends: None,
            description: Some(
                "RF, chip ID register, always is ID_CH57*.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "chip_id",
                    description: Some(
                        "RF,chip ID register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "GlobCfgInfo",
            extends: None,
            description: Some(
                "RO, global configuration information and status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "cfg_reset_en",
                    description: Some(
                        "RO, manual reset input enable status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cfg_boot_en",
                    description: Some(
                        "RO, boot-loader enable status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cfg_rst_pin",
                    description: Some(
                        "RO, Reset pin selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "boot_loader",
                    description: Some(
                        "RO, indicate boot loader status: 0=application status (by software reset), 1=boot loader status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "GlobResetKeep",
            extends: None,
            description: Some(
                "RW, value keeper during global reset.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "glob_reset_keep",
                    description: Some(
                        "RW, value keeper during global reset.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "GlobRomCfg",
            extends: None,
            description: Some(
                "RWA, flash ROM configuration.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rom_code_ofs",
                    description: Some(
                        "RWA, code offset address selection in Flash ROM: 0=start address 0x000000, 1=start address 0x008000.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rom_ctrl_en",
                    description: Some(
                        "RWA, enable flash ROM control interface enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rom_code_we",
                    description: Some(
                        "RWA, enable flash ROM code area being erase or write.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "OscCalCnt",
            extends: None,
            description: Some(
                "RO, system clock count value for 32KHz multi-cycles.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "osc_cal_cnt",
                    description: Some(
                        "RO, system clock count value for 32KHz multi-cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "osc_cal_ov_clr",
                    description: Some(
                        "RW1, indicate R8_OSC_CAL_OV_CNT not zero, set 1 to clear R8_OSC_CAL_OV_CNT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "osc_cal_if",
                    description: Some(
                        "RW1, interrupt flag for oscillator capture end, set 1 to clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "OscCalCtrl",
            extends: None,
            description: Some(
                "RWA, oscillator frequency calibration control, SAM.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "osc_cnt_total",
                    description: Some(
                        "RWA, total cycles mode for oscillator capture.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "osc_cnt_halt",
                    description: Some(
                        "RO, calibration counter halt status: 0=counting, 1=halt for reading count value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "osc_cal_ie",
                    description: Some(
                        "RWA, interrupt enable for oscillator capture end.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "osc_cnt_en",
                    description: Some(
                        "RWA, calibration counter enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "osc_cnt_end",
                    description: Some(
                        "RWA, select oscillator capture end mode: 0=normal, 1=append 2 cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnt_clr",
                    description: Some(
                        "RWA, Software reset RB_SSC_CNT_TOTAL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "OscCalOvCnt",
            extends: None,
            description: Some(
                "RO, oscillator frequency calibration overflow times.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "osc_cal_ov_cnt",
                    description: Some(
                        "RO, oscillator frequency calibration overflow times.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PllConfig",
            extends: None,
            description: Some(
                "RWA, PLL configuration control, SAM.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pll_cfg_dat",
                    description: Some(
                        "RWA, PLL configure data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "ResetStatus",
            extends: None,
            description: Some(
                "RO, reset status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "reset_flag",
                    description: Some(
                        "RO, recent reset flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "RstWdogCtrl",
            extends: None,
            description: Some(
                "RWA, reset and watch-dog control, SAM.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "boot_load_man",
                    description: Some(
                        "Read-Only, Manually enter BOOT Loader flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "software_reset",
                    description: Some(
                        "WA or WZ, global software reset, high action, auto clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdog_rst_en",
                    description: Some(
                        "RWA, enable watch-dog reset if watch-dog timer overflow: 0=as timer only, 1=enable reset if timer overflow.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdog_int_en",
                    description: Some(
                        "RWA, watch-dog timer overflow interrupt enable: 0=disable, 1=enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wdog_int_flag",
                    description: Some(
                        "RW1, watch-dog timer overflow interrupt flag, cleared by RW1 or reload watch-dog count or __SEV(Send-Event).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SafeAccessId",
            extends: None,
            description: Some(
                "RF, safe accessing ID register, always 0x0C.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "safe_access_id",
                    description: Some(
                        "RF,safe accessing ID register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SafeAccessSig",
            extends: None,
            description: Some(
                "WO, safe accessing sign register, must write SAFE_ACCESS_SIG1 then SAFE_ACCESS_SIG2 to enter safe accessing mode.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "safe_access_sig",
                    description: Some(
                        "WO, safe accessing sign register, must write 0x57 then 0xA8 to enter safe accessing mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "safe_acc_mode",
                    description: Some(
                        "RO, current safe accessing mode: 11=safe unlocked (SAM), other=locked (00..01..10..11).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "safe_acc_act",
                    description: Some(
                        "RO, indicate safe accessing status now: 0=locked, read only, 1=safe/unlocked (SAM), write enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "safe_acc_timer",
                    description: Some(
                        "RO, safe accessing timer bit mask (16*clock number).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "WdogCount",
            extends: None,
            description: Some(
                "RW, watch-dog count, count by clock frequency Fsys/131072.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "wdog_count",
                    description: Some(
                        "RW,watch-dog count, count by clock frequency Fsys/131072.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Xt32mTune",
            extends: None,
            description: Some(
                "RWA, external 32MHz oscillator tune control, SAM.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "xt32m_i_bias",
                    description: Some(
                        "RWA, external 32MHz oscillator bias current tune: 00=75% current, 01=standard current, 10=125% current, 11=150% current.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xt32m_c_load",
                    description: Some(
                        "RWA, external 32MHz oscillator load capacitor tune: Cap = RB_XT32M_C_LOAD * 2 + 10pF.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
