
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Keyscan",
            extends: None,
            description: Some(
                "KEYSCAN register.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "RW, Key scan control register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Ctrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_en",
                    description: Some(
                        "RW, Key scan interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_flag",
                    description: Some(
                        "RW, Key scan interrupt flag register.",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntFlag",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "numb",
                    description: Some(
                        "RW, Key scanning key number register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Numb",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ctrl",
            extends: None,
            description: Some(
                "RW, Key scan control register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "scan_start_en",
                    description: Some(
                        "RW, Start scan enable.",
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
                    name: "scan_cnt_end",
                    description: Some(
                        "RW, Set the number of times to scan to the same key value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scan_clk_div",
                    description: Some(
                        "RW, Scan clock division.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pin_scan_en",
                    description: Some(
                        "RW, I/O pin enabling participating in key scanning.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scan_1end_wake_en",
                    description: Some(
                        "RW, Trigger wake-up signal enabling after a single scan.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clr_wakeup_en",
                    description: Some(
                        "RW, Automatically clear wake-up signal enable after wake-up.",
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
            ],
        },
        FieldSet {
            name: "IntEn",
            extends: None,
            description: Some(
                "RW, Key scan interrupt enable register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "key_pressed_ie",
                    description: Some(
                        "RW, It is detected that the key is pressed and the number of times of the same key value reaches the set value (RB_SCAN_CNT_END) interrupt enable.",
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
                    name: "scan_1end_ie",
                    description: Some(
                        "RW, Interrupt enabling after a single scan.",
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
                    name: "key_released_ie",
                    description: Some(
                        "RW, Key release interrupt enable detected.",
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
            ],
        },
        FieldSet {
            name: "IntFlag",
            extends: None,
            description: Some(
                "RW, Key scan interrupt flag register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "key_pressed_if",
                    description: Some(
                        "RW, It is detected that the key is pressed and the number of times of the same key value reaches the set value (RB_SCAN_CNT_END) interrupt flag.",
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
                    name: "scan_1end_if",
                    description: Some(
                        "RW, Interrupt flag after single scan.",
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
                    name: "key_released_if",
                    description: Some(
                        "RW, Key release interrupt flag detected.",
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
            ],
        },
        FieldSet {
            name: "Numb",
            extends: None,
            description: Some(
                "RW, Key scanning key number register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "numb",
                    description: Some(
                        "RW, Current key number value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 20,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cnt",
                    description: Some(
                        "RW, Current key scanning times.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
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
                