use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "SystemControl",
            extends: None,
            description: Some(
                "System_control register.",
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
                        "RF, safe accessing ID register, always 0x04.",
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
                        "RO, system clock count value for 32KHz 5 cycles.",
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
                    name: "osc_cal_ctrl",
                    description: Some(
                        "RWA, oscillator frequency calibration control, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x52,
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
                    name: "cfg_rom_read",
                    description: Some(
                        "RO, indicate protected status of Flash ROM code and data: 0=reading protect, 1=enable read by external programmer.",
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
                    name: "cfg_debug_en",
                    description: Some(
                        "RO, debug enable status.",
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
            name: "OscCalCnt",
            extends: None,
            description: Some(
                "RO, system clock count value for 32KHz 5 cycles.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "osc_cal_cnt",
                    description: Some(
                        "RO, system clock count value for 32KHz 5 cycles.",
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
                    name: "osc_cnt_en",
                    description: Some(
                        "RWA, calibration counter enable.",
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
                    name: "osc_cnt_halt",
                    description: Some(
                        "RO, calibration counter halt status: 0=counting, 1=halt for reading count value.",
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
                    bit_size: 7,
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
                "RF, safe accessing ID register, always 0x04.",
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
                            offset: 3,
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
                            offset: 4,
                        },
                    ),
                    bit_size: 3,
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
                        "RF,watch-dog count, count by clock frequency Fsys/131072.",
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
