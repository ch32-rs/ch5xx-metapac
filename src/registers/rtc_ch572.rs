use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rtc",
            extends: None,
            description: Some(
                "Real time clock.",
            ),
            items: &[
                BlockItem {
                    name: "flag_ctrl",
                    description: Some(
                        "RWA, Real time clock.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "FlagCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mode_ctrl",
                    description: Some(
                        "RWA, RTC mode control, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x31,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "ModeCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "trig",
                    description: Some(
                        "RWA, RTC trigger value, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Trig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt_lsi",
                    description: Some(
                        "RO, RTC count based lsi.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "CntLsi",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt_div1",
                    description: Some(
                        "RO, RTC count based 65536 LSI clock cycles.",
                    ),
                    array: None,
                    byte_offset: 0x3a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "CntDiv1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt_div2",
                    description: Some(
                        "RO, RTC count based 2831155200 LSI clock cycles, only low 14 bit.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "CntDiv2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "CntDiv1",
            extends: None,
            description: Some(
                "RO, RTC count based 65536 LSI clock cycles.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "cnt_div1",
                    description: Some(
                        "RO, RTC count based 65536 LSI clock cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "CntDiv2",
            extends: None,
            description: Some(
                "RO, RTC count based 2831155200 LSI clock cycles, only low 14 bit.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt_div2",
                    description: Some(
                        "RWA,RTC count based 2831155200 LSI clock cycles.",
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
            name: "CntLsi",
            extends: None,
            description: Some(
                "RO, RTC count based lsi.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "cnt_lsi",
                    description: Some(
                        "R0,RTC count based lsi.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "FlagCtrl",
            extends: None,
            description: Some(
                "RWA, Real time clock.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tmr_clr",
                    description: Some(
                        "RW, set 1 to clear RTC timer action flag, auto clear.",
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
                    name: "trig_clr",
                    description: Some(
                        "RW, set 1 to clear RTC trigger action flag, auto clear.",
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
                    name: "tmr_flag",
                    description: Some(
                        "RO, RTC timer action flag.",
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
                    name: "trig_flag",
                    description: Some(
                        "RO, RTC trigger action flag.",
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
            name: "ModeCtrl",
            extends: None,
            description: Some(
                "RWA, RTC mode control, SAM.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tmr_mode",
                    description: Some(
                        "RWA, RTC timer mode: 000=0.125S, 001=0.25S, 010=0.5S, 011=1S, 100=2S, 101=4S, 110=8S, 111=16S.",
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
                    name: "ignore_b0",
                    description: Some(
                        "RWA, force ignore bit0 for trigger mode: 0=compare bit0, 1=ignore bit0.",
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
                    name: "tmr_en",
                    description: Some(
                        "RWA, RTC timer mode enable.",
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
                    name: "trig_en",
                    description: Some(
                        "RWA, RTC trigger mode enable.",
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
                    name: "load_lo",
                    description: Some(
                        "RWA, set 1 to load RTC count low word R32_RTC_CNT_32K, auto clear after loaded.",
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
                    name: "load_hi",
                    description: Some(
                        "RWA, set 1 to load RTC count high word R32_RTC_CNT_DAY, auto clear after loaded.",
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
            name: "Trig",
            extends: None,
            description: Some(
                "RWA, RTC trigger value, SAM.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trig",
                    description: Some(
                        "RWA, RTC trigger value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
