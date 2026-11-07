use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Gpio",
            extends: None,
            description: Some(
                "GPIO register.",
            ),
            items: &[
                BlockItem {
                    name: "int_flag",
                    description: Some(
                        "GPIO interrupt flag; write 1 to clear.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
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
                    name: "int_enable",
                    description: Some(
                        "GPIO interrupt enable.",
                    ),
                    array: None,
                    byte_offset: 0x1d,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntEnable",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_mode",
                    description: Some(
                        "GPIO interrupt mode.",
                    ),
                    array: None,
                    byte_offset: 0x1e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntMode",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_polar",
                    description: Some(
                        "GPIO interrupt polarity.",
                    ),
                    array: None,
                    byte_offset: 0x1f,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntPolar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pa_dir",
                    description: Some(
                        "GPIO PA I/O direction.",
                    ),
                    array: None,
                    byte_offset: 0x40,
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
                        "GPIO PA input.",
                    ),
                    array: None,
                    byte_offset: 0x44,
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
                        "GPIO PA output.",
                    ),
                    array: None,
                    byte_offset: 0x48,
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
                        "GPIO PA clear output.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
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
                        "GPIO PA pullup resistance enable.",
                    ),
                    array: None,
                    byte_offset: 0x50,
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
                    name: "pa_pd",
                    description: Some(
                        "GPIO PA output open-drain and input pulldown resistance enable.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PaPd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pa_drv",
                    description: Some(
                        "GPIO PA driving capability.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PaDrv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pa_smt",
                    description: Some(
                        "GPIO PA output slew rate and input schmitt trigger.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PaSmt",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pb_dir",
                    description: Some(
                        "GPIO PB I/O direction.",
                    ),
                    array: None,
                    byte_offset: 0x60,
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
                        "GPIO PB input.",
                    ),
                    array: None,
                    byte_offset: 0x64,
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
                        "GPIO PB output.",
                    ),
                    array: None,
                    byte_offset: 0x68,
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
                        "GPIO PB clear output.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
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
                        "GPIO PB pullup resistance enable.",
                    ),
                    array: None,
                    byte_offset: 0x70,
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
                    name: "pb_pd",
                    description: Some(
                        "GPIO PB output open-drain and input pulldown resistance enable.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PbPd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pb_drv",
                    description: Some(
                        "GPIO PB driving capability.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PbDrv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pb_smt",
                    description: Some(
                        "GPIO PB output slew rate and input schmitt trigger.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PbSmt",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "IntEnable",
            extends: None,
            description: Some(
                "GPIO interrupt enable.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pa2_ie",
                    description: Some(
                        "PA2 pin interrupt enable.",
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
                    name: "pa3_ie",
                    description: Some(
                        "PA3 pin interrupt enable.",
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
                    name: "pa4_ie",
                    description: Some(
                        "PA4 pin interrupt enable.",
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
                    name: "pb3_ie",
                    description: Some(
                        "PB3 pin interrupt enable.",
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
                    name: "pb4_ie",
                    description: Some(
                        "PB4 pin interrupt enable.",
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
                    name: "pb11_ie",
                    description: Some(
                        "PB11 pin interrupt enable.",
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
                    name: "pb12_ie",
                    description: Some(
                        "PB12 pin interrupt enable.",
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
                    name: "pb15_ie",
                    description: Some(
                        "PB15 pin interrupt enable.",
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
            name: "IntFlag",
            extends: None,
            description: Some(
                "GPIO interrupt flag; write 1 to clear.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pa2_if",
                    description: Some(
                        "PA2 pin interrupt flag.",
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
                    name: "pa3_if",
                    description: Some(
                        "PA3 pin interrupt flag.",
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
                    name: "pa4_if",
                    description: Some(
                        "PA4 pin interrupt flag.",
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
                    name: "pb3_if",
                    description: Some(
                        "PB3 pin interrupt flag.",
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
                    name: "pb4_if",
                    description: Some(
                        "PB4 pin interrupt flag.",
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
                    name: "pb11_if",
                    description: Some(
                        "PB11 pin interrupt flag.",
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
                    name: "pb12_if",
                    description: Some(
                        "PB12 pin interrupt flag.",
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
                    name: "pb15_if",
                    description: Some(
                        "PB15 pin interrupt flag.",
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
            name: "IntMode",
            extends: None,
            description: Some(
                "GPIO interrupt trigger mode: 1=edge, 0=level.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pa2_im",
                    description: Some(
                        "PA2 pin interrupt mode.",
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
                    name: "pa3_im",
                    description: Some(
                        "PA3 pin interrupt mode.",
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
                    name: "pa4_im",
                    description: Some(
                        "PA4 pin interrupt mode.",
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
                    name: "pb3_im",
                    description: Some(
                        "PB3 pin interrupt mode.",
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
                    name: "pb4_im",
                    description: Some(
                        "PB4 pin interrupt mode.",
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
                    name: "pb11_im",
                    description: Some(
                        "PB11 pin interrupt mode.",
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
                    name: "pb12_im",
                    description: Some(
                        "PB12 pin interrupt mode.",
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
                    name: "pb15_im",
                    description: Some(
                        "PB15 pin interrupt mode.",
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
            name: "IntPolar",
            extends: None,
            description: Some(
                "GPIO interrupt polarity: 1=high level or rising edge, 0=low level or falling edge.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pa2_ip",
                    description: Some(
                        "PA2 pin interrupt polarity.",
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
                    name: "pa3_ip",
                    description: Some(
                        "PA3 pin interrupt polarity.",
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
                    name: "pa4_ip",
                    description: Some(
                        "PA4 pin interrupt polarity.",
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
                    name: "pb3_ip",
                    description: Some(
                        "PB3 pin interrupt polarity.",
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
                    name: "pb4_ip",
                    description: Some(
                        "PB4 pin interrupt polarity.",
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
                    name: "pb11_ip",
                    description: Some(
                        "PB11 pin interrupt polarity.",
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
                    name: "pb12_ip",
                    description: Some(
                        "PB12 pin interrupt polarity.",
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
                    name: "pb15_ip",
                    description: Some(
                        "PB15 pin interrupt polarity.",
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
            name: "PaClr",
            extends: None,
            description: Some(
                "GPIO PA clear output.",
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
                "GPIO PA I/O direction.",
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
            name: "PaDrv",
            extends: None,
            description: Some(
                "GPIO PA driving capability.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pa_drv",
                    description: Some(
                        "GPIO PA output drive current: 1=max 16mA, 0=max 8mA.",
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
            name: "PaOut",
            extends: None,
            description: Some(
                "GPIO PA output.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pa_out",
                    description: Some(
                        "GPIO PA output level: 1=high, 0=low.",
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
            name: "PaPd",
            extends: None,
            description: Some(
                "GPIO PA output open-drain and input pulldown resistance enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pa_pd",
                    description: Some(
                        "GPIO PA open-drain output (output mode) / pull-down resistor (input mode): 1=enabled, 0=disabled.",
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
                "GPIO PA input.",
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
                "GPIO PA pullup resistance enable.",
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
            name: "PaSmt",
            extends: None,
            description: Some(
                "GPIO PA output slew rate and input schmitt trigger.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pa_smt",
                    description: Some(
                        "GPIO PA low-slew-rate output (output mode) / Schmitt-trigger input (input mode): 1=enabled, 0=disabled.",
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
                "GPIO PB clear output.",
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
                "GPIO PB I/O direction.",
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
            name: "PbDrv",
            extends: None,
            description: Some(
                "GPIO PB driving capability.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pb_drv",
                    description: Some(
                        "GPIO PB output drive current: 1=max 16mA, 0=max 8mA.",
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
            name: "PbOut",
            extends: None,
            description: Some(
                "GPIO PB output.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pb_out",
                    description: Some(
                        "GPIO PB output level: 1=high, 0=low.",
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
            name: "PbPd",
            extends: None,
            description: Some(
                "GPIO PB output open-drain and input pulldown resistance enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pb_pd",
                    description: Some(
                        "GPIO PB open-drain output (output mode) / pull-down resistor (input mode): 1=enabled, 0=disabled.",
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
                "GPIO PB input.",
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
                "GPIO PB pullup resistance enable.",
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
            name: "PbSmt",
            extends: None,
            description: Some(
                "GPIO PB output slew rate and input schmitt trigger.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pb_smt",
                    description: Some(
                        "GPIO PB low-slew-rate output (output mode) / Schmitt-trigger input (input mode): 1=enabled, 0=disabled.",
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
