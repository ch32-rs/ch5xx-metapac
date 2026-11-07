
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "TouchKey",
            extends: None,
            description: Some(
                "Touch buttons.",
            ),
            items: &[
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
                BlockItem {
                    name: "cfg2",
                    description: Some(
                        "RW, TouchKey configuration register 2.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sel",
                    description: Some(
                        "RW, TouchKey selection register.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sel",
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
                    name: "drv_en",
                    description: Some(
                        "RW, Touchkey drive shield enable.",
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
                    name: "adc_pga_cfg",
                    description: Some(
                        "RW, Select the running speed of PGA in ADC.",
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
                    name: "dma_en",
                    description: Some(
                        "RW, TouchKey DMA Enable.",
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
            ],
        },
        FieldSet {
            name: "Cfg2",
            extends: None,
            description: Some(
                "RW, TouchKey configuration register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "trans",
                    description: Some(
                        "RW, Touch key Capacitive charge moving time.",
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
                    name: "cc",
                    description: Some(
                        "RW, Touch key capacitor charging time.",
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
                    name: "tkadc_start_sel",
                    description: Some(
                        "RW, Touch the key to start ADC time selection in advance, unit: moving cycle.",
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
                    name: "en",
                    description: Some(
                        "RW, TKEY module enable.",
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
                    name: "ctrans_act",
                    description: Some(
                        "RW, TKEY module start signal and working status, automatic reset.",
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
                Field {
                    name: "transfer_n",
                    description: Some(
                        "RW, Number of moving cycles.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sel",
            extends: None,
            description: Some(
                "RW, TouchKey selection register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sel_cs",
                    description: Some(
                        "RW, Selection of charge transfer channel.",
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
                    name: "driverout_en",
                    description: Some(
                        "RW, TouchKey drive shield channel selection switch.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                