
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Systick",
            extends: None,
            description: Some(
                "Systick register.",
            ),
            items: &[
                BlockItem {
                    name: "stk_ctlr",
                    description: Some(
                        "Systick counter control register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "StkCtlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stk_cntl",
                    description: Some(
                        "Systick counter low register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "StkCntl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stk_cnth",
                    description: Some(
                        "Systick counter high register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "StkCnth",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stk_cmplr",
                    description: Some(
                        "Systick compare low register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "StkCmplr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stk_cmphr",
                    description: Some(
                        "Systick compare high register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "StkCmphr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "stk_cntfg",
                    description: Some(
                        "Systick counter flag.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "StkCntfg",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "StkCmphr",
            extends: None,
            description: Some(
                "Systick compare high register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmph",
                    description: Some(
                        "CMPH.",
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
            name: "StkCmplr",
            extends: None,
            description: Some(
                "Systick compare low register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmpl",
                    description: Some(
                        "CMPL.",
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
            name: "StkCntfg",
            extends: None,
            description: Some(
                "Systick counter flag.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "swie",
                    description: Some(
                        "System soft interrupt enable.",
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
                    name: "cntif",
                    description: Some(
                        "Systick counter clear zero flag.",
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
            name: "StkCnth",
            extends: None,
            description: Some(
                "Systick counter high register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cnth",
                    description: Some(
                        "CNTH.",
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
            name: "StkCntl",
            extends: None,
            description: Some(
                "Systick counter low register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cntl",
                    description: Some(
                        "CNTL.",
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
            name: "StkCtlr",
            extends: None,
            description: Some(
                "Systick counter control register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ste",
                    description: Some(
                        "Systick counter enable.",
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
                    name: "stie",
                    description: Some(
                        "Systick counter interrupt enable.",
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
                    name: "stclk",
                    description: Some(
                        "System counter clock Source selection.",
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
                    name: "streload",
                    description: Some(
                        "System counter reload control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                