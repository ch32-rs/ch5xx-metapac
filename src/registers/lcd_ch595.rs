
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Lcd",
            extends: None,
            description: Some(
                "LCD register.",
            ),
            items: &[
                BlockItem {
                    name: "cmd",
                    description: Some(
                        "RW, LCD Command Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ram0",
                    description: Some(
                        "RW, LCD RAM0 register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ram0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ram1",
                    description: Some(
                        "RW, LCD RAM1 register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ram1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ram2",
                    description: Some(
                        "RW, LCD RAM2 register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ram2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cmd",
            extends: None,
            description: Some(
                "RW, LCD Command Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sys_en",
                    description: Some(
                        "RW, System Enable.",
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
                    name: "on",
                    description: Some(
                        "RW, LCD switch.",
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
                    name: "bias",
                    description: Some(
                        "RW, PWM6 Bias ratio selection enable.",
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
                    name: "duty",
                    description: Some(
                        "RW, LCD duty cycle selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scan_clk_sel",
                    description: Some(
                        "RW, Scan clock selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vlcd_sel",
                    description: Some(
                        "RW, LCD driver voltage selection.",
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
                Field {
                    name: "seg_en1",
                    description: Some(
                        "RW, SEG0-SEG7 section enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seg_en2",
                    description: Some(
                        "RW, SEG8-SEG15 section enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "seg_en3",
                    description: Some(
                        "RW, SEG16-SEG19 section enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ram0",
            extends: None,
            description: Some(
                "RW, LCD RAM0 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "segx",
                    description: Some(
                        "RW, SEG0-SEG7 segment data.",
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
        FieldSet {
            name: "Ram1",
            extends: None,
            description: Some(
                "RW, LCD RAM1 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "segx",
                    description: Some(
                        "RW, SEG8-SEG15 segment data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ram2",
            extends: None,
            description: Some(
                "RW, LCD RAM2 register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "segx",
                    description: Some(
                        "RW, SEG16-SEG19 segment data.",
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
    ],
    enums: &[],
};
                