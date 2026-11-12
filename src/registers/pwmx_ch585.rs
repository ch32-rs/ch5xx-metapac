use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pwmx",
            extends: None,
            description: Some(
                "PWMx register.",
            ),
            items: &[
                BlockItem {
                    name: "out_en",
                    description: Some(
                        "RW, PWM output enable control.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "OutEn",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "polar",
                    description: Some(
                        "RW, PWM output polarity control.",
                    ),
                    array: None,
                    byte_offset: 0x1,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Polar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "config",
                    description: Some(
                        "RW, PWM configuration.",
                    ),
                    array: None,
                    byte_offset: 0x2,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Config",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clock_div",
                    description: Some(
                        "RW, PWM clock divisor.",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "ClockDiv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm4_7_data",
                    description: Some(
                        "RW, PWM data retention register group 1.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pwm47Data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm8_11_data",
                    description: Some(
                        "RW, PWM data retention register group 2.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pwm811Data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_ctrl",
                    description: Some(
                        "RW, PWM interrupt control.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "IntCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "reg_data8",
                    description: Some(
                        "RW, PWM8/9 data register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RegData8",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "reg_cycle",
                    description: Some(
                        "RW, PWM cycle end register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "RegCycle",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "ClockDiv",
            extends: None,
            description: Some(
                "RW, PWM clock divisor.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "clock_div",
                    description: Some(
                        "RW, PWM clock divisor.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Config",
            extends: None,
            description: Some(
                "RW, PWM configuration.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "cycle_sel",
                    description: Some(
                        "RW, PWM cycle selection: 0=256;128;64;32 clocks, 1=255;127;63;31 clocks.",
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
                    name: "stag_st",
                    description: Some(
                        "RO, PWM stagger cycle status.",
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
                    name: "cyc_mod",
                    description: Some(
                        "RW, PWM data width mode: 00=8 bits data, 01=7 bits data, 10=6 bits data, 11=16 bits data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwm4_5_stag_en",
                    description: Some(
                        "RW, PWM4/5 stagger output enable: 0=independent output, 1=stagger output.",
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
                    name: "pwm6_7_stag_en",
                    description: Some(
                        "RW, PWM6/7 stagger output enable: 0=independent output, 1=stagger output.",
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
                    name: "pwm8_9_stag_en",
                    description: Some(
                        "RW, PWM8/9 stagger output enable: 0=independent output, 1=stagger output.",
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
                    name: "pwm10_11_stag_en",
                    description: Some(
                        "RW, PWM10/11 stagger output enable: 0=independent output, 1=stagger output.",
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
            name: "IntCtrl",
            extends: None,
            description: Some(
                "RW, PWM interrupt control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ie_cyc",
                    description: Some(
                        "RW, enable interrupt for PWM cycle end.",
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
                    name: "cyc_pre",
                    description: Some(
                        "RW, select PWM cycle interrupt point.",
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
                    name: "if_cyc",
                    description: Some(
                        "RW1, interrupt flag for PWM cycle end.",
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
            name: "OutEn",
            extends: None,
            description: Some(
                "RW, PWM output enable control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pwm4_out_en",
                    description: Some(
                        "RW, PWM4 output enable.",
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
                    name: "pwm5_out_en",
                    description: Some(
                        "RW, PWM5 output enable.",
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
                    name: "pwm6_out_en",
                    description: Some(
                        "RW, PWM6 output enable.",
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
                    name: "pwm7_out_en",
                    description: Some(
                        "RW, PWM7 output enable.",
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
                    name: "pwm8_out_en",
                    description: Some(
                        "RW, PWM8 output enable.",
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
                    name: "pwm9_out_en",
                    description: Some(
                        "RW, PWM9 output enable.",
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
                    name: "pwm10_out_en",
                    description: Some(
                        "RW, PWM10 output enable.",
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
                    name: "pwm11_out_en",
                    description: Some(
                        "RW, PWM11 output enable.",
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
            name: "Polar",
            extends: None,
            description: Some(
                "RW, PWM output polarity control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pwm4_polar",
                    description: Some(
                        "RW, PWM4 output polarity: 0=default low and high action, 1=default high and low action.",
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
                    name: "pwm5_polar",
                    description: Some(
                        "RW, PWM5 output polarity: 0=default low and high action, 1=default high and low action.",
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
                    name: "pwm6_polar",
                    description: Some(
                        "RW, PWM6 output polarity: 0=default low and high action, 1=default high and low action.",
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
                    name: "pwm7_polar",
                    description: Some(
                        "RW, PWM7 output polarity: 0=default low and high action, 1=default high and low action.",
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
                    name: "pwm8_polar",
                    description: Some(
                        "RW, PWM8 output polarity: 0=default low and high action, 1=default high and low action.",
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
                    name: "pwm9_polar",
                    description: Some(
                        "RW, PWM9 output polarity: 0=default low and high action, 1=default high and low action.",
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
                    name: "pwm10_polar",
                    description: Some(
                        "RW, PWM10 output polarity: 0=default low and high action, 1=default high and low action.",
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
                    name: "pwm11_polar",
                    description: Some(
                        "RW, PWM11 output polarity: 0=default low and high action, 1=default high and low action.",
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
            name: "Pwm47Data",
            extends: None,
            description: Some(
                "RW, PWM data retention register group 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwm4_data__pwm4_data",
                    description: Some(
                        "RW, PWM4 data holding or PWM4 data holding(16bits wide).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwm5_data__pwm4_data",
                    description: Some(
                        "RW, PWM5 data holding or PWM4 data holding(16bits wide).",
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
                    name: "pwm6_data__pwm5_data",
                    description: Some(
                        "RW, PWM6 data holding or PWM5 data holding(16bits wide).",
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
                    name: "pwm7_data__pwm5_data",
                    description: Some(
                        "RW, PWM7 data holding or PWM5 data holding(16bits wide).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pwm811Data",
            extends: None,
            description: Some(
                "RW, PWM data retention register group 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwm8_data__pwm6_data",
                    description: Some(
                        "RW, PWM8 data holding or PWM6 data holding(16bits wide).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwm9_data__pwm6_data",
                    description: Some(
                        "RW, PWM9 data holding or PWM6 data holding(16bits wide).",
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
                    name: "pwm10_data__pwm7_data",
                    description: Some(
                        "RW, PWM10 data holding or PWM7 data holding(16bits wide).",
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
                    name: "pwm11_data__pwm7_data",
                    description: Some(
                        "RW, PWM11 data holding or PWM7 data holding(16bits wide).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "RegCycle",
            extends: None,
            description: Some(
                "RW, PWM cycle end register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cyc_value",
                    description: Some(
                        "RW, PWM End Cycles.",
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
            name: "RegData8",
            extends: None,
            description: Some(
                "RW, PWM8/9 data register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwm8_reg_data",
                    description: Some(
                        "RW, 16 bit data of PWM8 channel.",
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
                Field {
                    name: "pwm9_reg_data",
                    description: Some(
                        "RW, 16 bit data of PWM9 channel.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
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
