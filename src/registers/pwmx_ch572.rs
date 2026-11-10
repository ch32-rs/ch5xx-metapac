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
                    name: "dma_ctrl",
                    description: Some(
                        "RW, PWMx DMA control register.",
                    ),
                    array: None,
                    byte_offset: 0x3,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "DmaCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm1_3_data",
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
                                "Pwm13Data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm1_data",
                    description: Some(
                        "RW, PWM1 data holding, 16 bits wide.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Pwm1Data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm2_data",
                    description: Some(
                        "RW, PWM2 data holding, 16 bits wide.",
                    ),
                    array: None,
                    byte_offset: 0x6,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Pwm2Data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm3_data",
                    description: Some(
                        "RW, PWM data retention register group 2.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Pwm3Data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_en",
                    description: Some(
                        "RW, PWMx interrupt enable register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
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
                        "RW1, PWMx interrupt flag register.",
                    ),
                    array: None,
                    byte_offset: 0xd,
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
                    name: "pwm4_5_data",
                    description: Some(
                        "RW, PWM data retention register group 3.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pwm45Data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm4_data",
                    description: Some(
                        "RW, PWM4 data holding, 16 bits wide.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Pwm4Data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pwm5_data",
                    description: Some(
                        "RW, PWM5 data holding, 16 bits wide.",
                    ),
                    array: None,
                    byte_offset: 0x12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Pwm5Data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cyc_value",
                    description: Some(
                        "RW, PWM1/2/3 cycles end register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "CycValue",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cyc1_value",
                    description: Some(
                        "RW, PWM4/5 cycles end register.",
                    ),
                    array: None,
                    byte_offset: 0x16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Cyc1Value",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clock_div",
                    description: Some(
                        "RW, PWM reference clock frequency division coefficient.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "ClockDiv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_now",
                    description: Some(
                        "RW, Current address of PWMx DMA buffer.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaNow",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_beg",
                    description: Some(
                        "RW, Starting address of PWMx DMA buffer.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaBeg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_end",
                    description: Some(
                        "RW, End of PWMx DMA buffer.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "DmaEnd",
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
                "RW, PWM reference clock frequency division coefficient.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "clock_div",
                    description: Some(
                        "RW, DMA data buffer current address.",
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
                    name: "cyc_mod",
                    description: Some(
                        "RW, PWM data width mode: 00=8 bits data, 01=7 bits data, 10=6 bits data, 11=5 bits data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
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
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwm4_5_ch",
                    description: Some(
                        "RW, PWM4/5 channel output status.",
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
                    name: "sync_start",
                    description: Some(
                        "RW, Sync Start.",
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
                    name: "sync_en",
                    description: Some(
                        "RW, Synchronization enablement allows.",
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
            name: "Cyc1Value",
            extends: None,
            description: Some(
                "RW, PWM4/5 cycles end register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "cyc1_value",
                    description: Some(
                        "RW, PWM4/5 clock cycle at 16 data widths.",
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
            name: "CycValue",
            extends: None,
            description: Some(
                "RW, PWM1/2/3 cycles end register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "cyc_value",
                    description: Some(
                        "RW, PWM1/2/3 clock cycle at 16 data widths.",
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
            name: "DmaBeg",
            extends: None,
            description: Some(
                "RW, Starting address of PWMx DMA buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma_beg",
                    description: Some(
                        "RW, Starting address of PWMx DMA buffer.",
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
            name: "DmaCtrl",
            extends: None,
            description: Some(
                "RW, PWMx DMA control register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dma_enable",
                    description: Some(
                        "RW, DMA enabled (only valid when the data width is 16).",
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
                    name: "dma_addr_loop",
                    description: Some(
                        "RW, DMA Loop Output Enable.",
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
                    name: "dma_sel",
                    description: Some(
                        "RW, DMA output channel selection when RB_PWM_SYNC_EN=0.",
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
            name: "DmaEnd",
            extends: None,
            description: Some(
                "RW, End of PWMx DMA buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma_end",
                    description: Some(
                        "RW, End of PWMx DMA buffer.",
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
            name: "DmaNow",
            extends: None,
            description: Some(
                "RW, Current address of PWMx DMA buffer.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma_now",
                    description: Some(
                        "RW, Current address of PWMx DMA buffer.",
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
            name: "IntEn",
            extends: None,
            description: Some(
                "RW, PWMx interrupt enable register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ie_cyc",
                    description: Some(
                        "RW, End of PWM1/2/3 cycle interrupt enable bit.",
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
                        "RW, Select the interrupt time point at the end of the PWM cycle.",
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
                    name: "pwm1_ie_cyc",
                    description: Some(
                        "RW, End of PWM4/5 cycle interrupt enable bit.",
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
                    name: "ie_fifo",
                    description: Some(
                        "RW, FIFO data less than 4 interrupt enable bits.",
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
                    name: "ie_dma",
                    description: Some(
                        "RW, DMA transfer end interrupt enable bit.",
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
                    name: "ie_over",
                    description: Some(
                        "RW, FIFO read air interrupt enable bit.",
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
            name: "IntFlag",
            extends: None,
            description: Some(
                "RW1, PWMx interrupt flag register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "if_cyc",
                    description: Some(
                        "RW1, PWM1/2/3 cycle end flag.",
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
                    name: "pwm1_if_cyc",
                    description: Some(
                        "RW1, PWM4/5 cycle end flag.",
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
                    name: "if_fifo",
                    description: Some(
                        "RW1, FIFO data less than 4 interrupt flag bits.",
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
                    name: "if_dma",
                    description: Some(
                        "RW1, DMA transfer end interrupt flag bit.",
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
                    name: "if_over",
                    description: Some(
                        "RW1, FIFO overflow interrupt flag bit.",
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
                    name: "pwm1_out_en",
                    description: Some(
                        "RW, PWM1 output enable.",
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
                    name: "pwm2_out_en",
                    description: Some(
                        "RW, PWM2 output enable.",
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
                    name: "pwm3_out_en",
                    description: Some(
                        "RW, PWM3 output enable.",
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
                    name: "pwm4_out_en",
                    description: Some(
                        "RW, PWM4 output enable.",
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
                    name: "pwm5_out_en",
                    description: Some(
                        "RW, PWM5 output enable.",
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
                    name: "pwm1_polar",
                    description: Some(
                        "RW, PWM1 output polarity: 0=default low and high action, 1=default high and low action.",
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
                    name: "pwm2_polar",
                    description: Some(
                        "RW, PWM2 output polarity: 0=default low and high action, 1=default high and low action.",
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
                    name: "pwm3_polar",
                    description: Some(
                        "RW, PWM3 output polarity: 0=default low and high action, 1=default high and low action.",
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
                    name: "pwm4_polar",
                    description: Some(
                        "RW, PWM4 output polarity: 0=default low and high action, 1=default high and low action.",
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
                    name: "pwm5_polar",
                    description: Some(
                        "RW, PWM5 output polarity: 0=default low and high action, 1=default high and low action.",
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
            ],
        },
        FieldSet {
            name: "Pwm13Data",
            extends: None,
            description: Some(
                "RW, PWM data retention register group 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pwm1_data__pwm1_data",
                    description: Some(
                        "RW, PWM1 data holding or PWM1 data holding(16bits wide).",
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
                    name: "pwm2_data__pwm1_data",
                    description: Some(
                        "RW, PWM2 data holding or PWM1 data holding(16bits wide).",
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
                    name: "pwm3_data__pwm2_data",
                    description: Some(
                        "RW, PWM3 data holding or PWM2 data holding(16bits wide).",
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
                    name: "pwm2_data",
                    description: Some(
                        "RW, PWM2 data holding(16bits wide).",
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
            name: "Pwm1Data",
            extends: None,
            description: Some(
                "RW, PWM1 data holding, 16 bits wide.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "pwm1_data",
                    description: Some(
                        "RW, PWM1 data holding, 16 bits wide.",
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
            name: "Pwm2Data",
            extends: None,
            description: Some(
                "RW, PWM2 data holding, 16 bits wide.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "pwm2_data",
                    description: Some(
                        "RW, PWM2 data holding, 16 bits wide.",
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
            name: "Pwm3Data",
            extends: None,
            description: Some(
                "RW, PWM data retention register group 2.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "pwm3_data",
                    description: Some(
                        "RW, PWM3 data retention register.",
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
            name: "Pwm45Data",
            extends: None,
            description: Some(
                "RW, PWM data retention register group 3.",
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
                    name: "pwm5_data",
                    description: Some(
                        "RW, PWM5 data holding(16bits wide).",
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
        FieldSet {
            name: "Pwm4Data",
            extends: None,
            description: Some(
                "RW, PWM4 data holding, 16 bits wide.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "pwm4_data",
                    description: Some(
                        "RW, PWM4 data holding, 16 bits wide.",
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
            name: "Pwm5Data",
            extends: None,
            description: Some(
                "RW, PWM5 data holding, 16 bits wide.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "pwm5_data",
                    description: Some(
                        "RW, PWM5 data holding, 16 bits wide.",
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
