use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Gpio",
            extends: None,
            description: Some(
                "General porpose intput output.",
            ),
            items: &[
                BlockItem {
                    name: "pa_int_en",
                    description: Some(
                        "RW, GPIO PA interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0x90,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "PaIntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pb_int_en",
                    description: Some(
                        "RW, GPIO PB interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0x92,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "PbIntEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pa_int_mode",
                    description: Some(
                        "RW, GPIO PA interrupt mode: 0=level action, 1=edge action.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "PaIntMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pb_int_mode",
                    description: Some(
                        "RW, GPIO PB interrupt mode: 0=level action, 1=edge action;RW, status for parallel slave read.",
                    ),
                    array: None,
                    byte_offset: 0x96,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "PbIntMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pa_int_if",
                    description: Some(
                        "RW1, GPIO PA interrupt flag.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "PaIntIf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pb_int_if",
                    description: Some(
                        "RW1, GPIO PB interrupt flag.",
                    ),
                    array: None,
                    byte_offset: 0x9e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "PbIntIf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pa_dir",
                    description: Some(
                        "RW, GPIO PA I/O direction: 0=in, 1=out.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PaDir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pa_pin",
                    description: Some(
                        "RO, GPIO PA input.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "PaPin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pa_out",
                    description: Some(
                        "RW, GPIO PA output.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PaOut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pa_clr",
                    description: Some(
                        "WZ, GPIO PA clear output: 0=keep, 1=clear.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "PaClr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pa_pu",
                    description: Some(
                        "RW, GPIO PA pullup resistance enable.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PaPu",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pa_pd_drv",
                    description: Some(
                        "RW, PA pulldown for input or PA driving capability for output.",
                    ),
                    array: None,
                    byte_offset: 0xb4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PaPdDrv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pa_set",
                    description: Some(
                        "WZ, PA port output set register.",
                    ),
                    array: None,
                    byte_offset: 0xb8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "PaSet",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pb_dir",
                    description: Some(
                        "RW, GPIO PB I/O direction: 0=in, 1=out.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PbDir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pb_pin",
                    description: Some(
                        "RO, GPIO PB input.",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "PbPin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pb_out",
                    description: Some(
                        "RW, GPIO PB output;RW, data for parallel slave read.",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PbOut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pb_clr",
                    description: Some(
                        "WZ, GPIO PB clear output: 0=keep, 1=clear.",
                    ),
                    array: None,
                    byte_offset: 0xcc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "PbClr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pb_pu",
                    description: Some(
                        "RW, GPIO PB pullup resistance enable.",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PbPu",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pb_pd_drv",
                    description: Some(
                        "RW, PB pulldown for input or PB driving capability for output.",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PbPdDrv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pb_set",
                    description: Some(
                        "WZ, PB port output set register.",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "PbSet",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "PaClr",
            extends: None,
            description: Some(
                "WZ, GPIO PA clear output: 0=keep, 1=clear.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pa_clr",
                    description: Some(
                        "GPIO PA clear output: write 1 to drive the pin output low, write 0 leaves it unchanged.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PaDir",
            extends: None,
            description: Some(
                "RW, GPIO PA I/O direction: 0=in, 1=out.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pa_dir",
                    description: Some(
                        "GPIO PA I/O direction: 0=input, 1=output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PaIntEn",
            extends: None,
            description: Some(
                "RW, GPIO PA interrupt enable.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "pa_int_en",
                    description: Some(
                        "GPIO PA interrupt enable.",
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
            name: "PaIntIf",
            extends: None,
            description: Some(
                "RW1, GPIO PA interrupt flag.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "pa_int_if",
                    description: Some(
                        "GPIO PA interrupt flag.",
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
            name: "PaIntMode",
            extends: None,
            description: Some(
                "RW, GPIO PA interrupt mode: 0=level action, 1=edge action.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "pa_int_mode",
                    description: Some(
                        "GPIO PA interrupt mode.",
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
            name: "PaOut",
            extends: None,
            description: Some(
                "RW, GPIO PA output.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pa_out",
                    description: Some(
                        "GPIO PA output level: 1=high, 0=low; when the pin is an input, selects the interrupt polarity (1=high level or rising edge, 0=low level or falling edge).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PaPdDrv",
            extends: None,
            description: Some(
                "RW, PA pulldown for input or PA driving capability for output.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pa_pd_drv",
                    description: Some(
                        "GPIO PA pull-down enable: 1=enabled, 0=disabled; when the pin is an output, selects the drive strength (1=20mA, 0=5mA).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PaPin",
            extends: None,
            description: Some(
                "RO, GPIO PA input.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pa_pin",
                    description: Some(
                        "GPIO PA input level: 1=high, 0=low.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PaPu",
            extends: None,
            description: Some(
                "RW, GPIO PA pullup resistance enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pa_pu",
                    description: Some(
                        "GPIO PA pull-up resistor: 1=enabled, 0=disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PaSet",
            extends: None,
            description: Some(
                "WZ, PA port output set register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pa_set",
                    description: Some(
                        "GPIO PA set output: write 1 to drive the pin output high, write 0 leaves it unchanged.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PbClr",
            extends: None,
            description: Some(
                "WZ, GPIO PB clear output: 0=keep, 1=clear.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pb_clr",
                    description: Some(
                        "GPIO PB clear output: write 1 to drive the pin output low, write 0 leaves it unchanged.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PbDir",
            extends: None,
            description: Some(
                "RW, GPIO PB I/O direction: 0=in, 1=out.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pb_dir",
                    description: Some(
                        "GPIO PB I/O direction: 0=input, 1=output.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PbIntEn",
            extends: None,
            description: Some(
                "RW, GPIO PB interrupt enable.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "pb_int_en",
                    description: Some(
                        "GPIO PB interrupt enable.",
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
            name: "PbIntIf",
            extends: None,
            description: Some(
                "RW1, GPIO PB interrupt flag.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "pb_int_if",
                    description: Some(
                        "GPIO PB interrupt flag.",
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
            name: "PbIntMode",
            extends: None,
            description: Some(
                "RW, GPIO PB interrupt mode: 0=level action, 1=edge action;RW, status for parallel slave read.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "pb_int_mode",
                    description: Some(
                        "GPIO PB interrupt mode.",
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
            name: "PbOut",
            extends: None,
            description: Some(
                "RW, GPIO PB output;RW, data for parallel slave read.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pb_out",
                    description: Some(
                        "GPIO PB output level: 1=high, 0=low; when the pin is an input, selects the interrupt polarity (1=high level or rising edge, 0=low level or falling edge).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PbPdDrv",
            extends: None,
            description: Some(
                "RW, PB pulldown for input or PB driving capability for output.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pb_pd_drv",
                    description: Some(
                        "GPIO PB pull-down enable: 1=enabled, 0=disabled; when the pin is an output, selects the drive strength (1=20mA, 0=5mA).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PbPin",
            extends: None,
            description: Some(
                "RO, GPIO PB input.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pb_pin",
                    description: Some(
                        "GPIO PB input level: 1=high, 0=low.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PbPu",
            extends: None,
            description: Some(
                "RW, GPIO PB pullup resistance enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pb_pu",
                    description: Some(
                        "GPIO PB pull-up resistor: 1=enabled, 0=disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PbSet",
            extends: None,
            description: Some(
                "WZ, PB port output set register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pb_set",
                    description: Some(
                        "GPIO PB set output: write 1 to drive the pin output high, write 0 leaves it unchanged.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
