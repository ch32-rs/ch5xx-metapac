
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Keyscan",
            extends: None,
            description: Some(
                "keys scanning.",
            ),
            items: &[
                BlockItem {
                    name: "ctrl",
                    description: Some(
                        "RW, Key scan control register.",
                    ),
                    array: None,
                    byte_offset: 0x64,
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
                        "RW, Press button scan interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0x66,
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
                        "RW, Press the button to scan the interrupt flag register.",
                    ),
                    array: None,
                    byte_offset: 0x67,
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
                        "RW, Scan key number register.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "RW, Start scanning enable.",
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
                        "RW, Scan clock frequency division.",
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
                        "RW, Enable I/O pins involved in key scanning.",
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
                        "RW, Activate wake-up signal after a single scan.",
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
                        "RW, Enable automatic clearing of wake-up signal after wake-up.",
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
                "RW, Press button scan interrupt enable register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "key_pressed_ie",
                    description: Some(
                        "RW, Detected key press and interrupted when the same key value is reached the set value.",
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
                        "RW, Interrupt enable after a single scan is completed.",
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
            name: "IntFlag",
            extends: None,
            description: Some(
                "RW, Press the button to scan the interrupt flag register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "key_pressed_if",
                    description: Some(
                        "RW, Detected button press, and the same button value reaches the set value for the number of times interrupt flag.",
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
                        "RW, Interrupt flag after word scanning is completed.",
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
            name: "Numb",
            extends: None,
            description: Some(
                "RW, Scan key number register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "numb",
                    description: Some(
                        "RO, Current key number value.",
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
                        "RO, Current number of key scans.",
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
                