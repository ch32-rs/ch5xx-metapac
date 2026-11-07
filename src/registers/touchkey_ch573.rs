
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "TouchKey",
            extends: None,
            description: Some(
                "TouchKey register.",
            ),
            items: &[
                BlockItem {
                    name: "count",
                    description: Some(
                        "RW, Touchkey charge and discharge count.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Count",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "convert",
                    description: Some(
                        "RW, Touchkey convert start control.",
                    ),
                    array: None,
                    byte_offset: 0x56,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Convert",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfg",
                    description: Some(
                        "RW, Touchkey configure.",
                    ),
                    array: None,
                    byte_offset: 0x57,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Cfg",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cfg",
            extends: None,
            description: Some(
                "RW, Touchkey configure.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pwr_on",
                    description: Some(
                        "RW, Touchkey power on.",
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
                    name: "current",
                    description: Some(
                        "RW, Touchkey charge current selection.",
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
                    name: "pga_adj",
                    description: Some(
                        "RW, ADC input PGA speed selection.",
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
            ],
        },
        FieldSet {
            name: "Convert",
            extends: None,
            description: Some(
                "RW, Touchkey convert start control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "start",
                    description: Some(
                        "RW, Touchkey convert start control.",
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
            ],
        },
        FieldSet {
            name: "Count",
            extends: None,
            description: Some(
                "RW, Touchkey charge and discharge count.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "charg_cnt",
                    description: Some(
                        "RW, Touchkey charge count.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "disch_cnt",
                    description: Some(
                        "RW, Touchkey discharge count.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
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
                