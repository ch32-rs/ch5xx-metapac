use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Rtc",
            extends: None,
            description: Some(
                "RTC register.",
            ),
            items: &[
                BlockItem {
                    name: "flag_ctrl",
                    description: Some(
                        "RW, RTC flag and clear control.",
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
                    name: "cnt_32k",
                    description: Some(
                        "RO, RTC count based 32KHz.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "Cnt32k",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt_2s",
                    description: Some(
                        "RO, RTC count based 2 second.",
                    ),
                    array: None,
                    byte_offset: 0x3a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "Cnt2s",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cnt_day",
                    description: Some(
                        "RO, RTC count based one day, only low 14 bit.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "CntDay",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cnt2s",
            extends: None,
            description: Some(
                "RO, RTC count based 2 second.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "cnt_2s",
                    description: Some(
                        "RO, RTC count based 2 second.",
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
            name: "Cnt32k",
            extends: None,
            description: Some(
                "RO, RTC count based 32KHz.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "cnt_32k",
                    description: Some(
                        "RWA,RTC count based 32KHz.",
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
            name: "CntDay",
            extends: None,
            description: Some(
                "RO, RTC count based one day, only low 14 bit.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnt_day",
                    description: Some(
                        "RWA,RTC count based one day.",
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
            name: "FlagCtrl",
            extends: None,
            description: Some(
                "RW, RTC flag and clear control.",
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
