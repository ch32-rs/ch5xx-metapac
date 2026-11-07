
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Adc",
            extends: None,
            description: Some(
                "Analog to Digital Converter.",
            ),
            items: &[
                BlockItem {
                    name: "channel",
                    description: Some(
                        "RW, ADC input channel selection.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Channel",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfg",
                    description: Some(
                        "RW, ADC configure.",
                    ),
                    array: None,
                    byte_offset: 0x59,
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
                    name: "convert",
                    description: Some(
                        "RW, ADC convert control.",
                    ),
                    array: None,
                    byte_offset: 0x5a,
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
                    name: "tem_sensor",
                    description: Some(
                        "RW, temperature sensor control.",
                    ),
                    array: None,
                    byte_offset: 0x5b,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "TemSensor",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "data",
                    description: Some(
                        "RO, ADC data.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 16,
                            fieldset: Some(
                                "Data",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_flag",
                    description: Some(
                        "RO, ADC interrupt flag register.",
                    ),
                    array: None,
                    byte_offset: 0x5e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "IntFlag",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ctrl_dma",
                    description: Some(
                        "RW, ADC DMA control.",
                    ),
                    array: None,
                    byte_offset: 0x61,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "CtrlDma",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_if",
                    description: Some(
                        "RW1, ADC and DMA interrupt flag registers.",
                    ),
                    array: None,
                    byte_offset: 0x62,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "DmaIf",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "auto_cycle",
                    description: Some(
                        "RW, auto ADC cycle value, unit is 16 Fsys.",
                    ),
                    array: None,
                    byte_offset: 0x63,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "AutoCycle",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dma_now",
                    description: Some(
                        "RO, ADC DMA current address.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
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
                        "RW, ADC DMA begin address.",
                    ),
                    array: None,
                    byte_offset: 0x68,
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
                        "RW, ADC DMA end address.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
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
                BlockItem {
                    name: "scan_cfg1",
                    description: Some(
                        "RW, ADC scan sequence, channels 2 through 9.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ScanCfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "scan_cfg2",
                    description: Some(
                        "RW, ADC scan sequence, channels 10 through 14, and scan control.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ScanCfg2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "AutoCycle",
            extends: None,
            description: Some(
                "RW, auto ADC cycle value, unit is 16 Fsys.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "auto_cycle",
                    description: Some(
                        "auto ADC cycle value, unit is 16 Fsys.",
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
            name: "Cfg",
            extends: None,
            description: Some(
                "RW, ADC configure.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "power_on",
                    description: Some(
                        "RW, ADC power control: 0=power down, 1=power on.",
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
                    name: "buf_en",
                    description: Some(
                        "RW, ADC input buffer enable.",
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
                    name: "diff_en",
                    description: Some(
                        "RW, ADC input channel mode: 0=single-end, 1=differnetial.",
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
                    name: "ofs_test",
                    description: Some(
                        "RW, enable ADC offset test mode: 0=normal mode, 1=short to test offset.",
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
                    name: "pga_gain10",
                    description: Some(
                        "RW, set ADC input PGA gain: 00=-12dB, 01=-6dB, 10=0dB, 11=6dB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clk_div",
                    description: Some(
                        "RW, select ADC clock frequency: 00=3.2MHz, 01=2.67MHz, 10=5.33MHz, 11=4MHz.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Channel",
            extends: None,
            description: Some(
                "RW, ADC input channel selection.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "ch_inx",
                    description: Some(
                        "RW, ADC input channel index.",
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
            ],
        },
        FieldSet {
            name: "Convert",
            extends: None,
            description: Some(
                "RW, ADC convert control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "start",
                    description: Some(
                        "RW, ADC convert start control: 0=stop ADC convert, 1=start an ADC convert, auto clear.",
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
                    name: "pga_gain2",
                    description: Some(
                        "RW, ADC input PGA gain direction selection, combined with RB_ ADC_ PGA_ GAIN [1:0] usage.",
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
                    name: "sample_time",
                    description: Some(
                        "RW, ADC sampling period selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eoc_x",
                    description: Some(
                        "RO, end of ADC conversion flag.",
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
            name: "CtrlDma",
            extends: None,
            description: Some(
                "RW, ADC DMA control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "dma_enable",
                    description: Some(
                        "RW, ADC DMA enable.",
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
                    name: "dma_loop",
                    description: Some(
                        "RW, ADC DMA address loop enable.",
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
                    name: "ie_dma_end",
                    description: Some(
                        "RW, enable interrupt for ADC DMA completion.",
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
                    name: "ie_eoc",
                    description: Some(
                        "RW, enable interrupt for end of ADC conversion.",
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
                    name: "scan_auto_type",
                    description: Some(
                        "RW, ADC SCAN mode: 1: continuous triggering 0: periodic triggering.",
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
                    name: "cont_en",
                    description: Some(
                        "RW, enable contineous conversion ADC.",
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
                    name: "auto_en",
                    description: Some(
                        "RW, enable auto continuing ADC for DMA.",
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
            name: "Data",
            extends: None,
            description: Some(
                "RO, ADC data.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "data",
                    description: Some(
                        "RO, ADC conversion data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmaBeg",
            extends: None,
            description: Some(
                "RW, ADC DMA begin address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma_beg",
                    description: Some(
                        "ADC DMA begin address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmaEnd",
            extends: None,
            description: Some(
                "RW, ADC DMA end address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma_end",
                    description: Some(
                        "ADC DMA end address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "DmaIf",
            extends: None,
            description: Some(
                "RW1, ADC and DMA interrupt flag registers.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "if_dma_end",
                    description: Some(
                        "interrupt flag for ADC DMA completion.",
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
                    name: "if_end",
                    description: Some(
                        "interrupt flag for end of ADC conversion.",
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
            name: "DmaNow",
            extends: None,
            description: Some(
                "RO, ADC DMA current address.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma_now",
                    description: Some(
                        "ADC DMA current address.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IntFlag",
            extends: None,
            description: Some(
                "RO, ADC interrupt flag register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "if_eoc",
                    description: Some(
                        "RO, ADC conversion interrupt flag.",
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
            name: "ScanCfg1",
            extends: None,
            description: Some(
                "RW, ADC scan sequence, channels 2 through 9.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ch",
                    description: Some(
                        "Scan sequence entry; entry 0 holds the second channel in the sequence.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "ScanCfg2",
            extends: None,
            description: Some(
                "RW, ADC scan sequence, channels 10 through 14, and scan control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ch",
                    description: Some(
                        "Scan sequence entry; entry 0 holds the tenth channel in the sequence.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 4,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "scan_num",
                    description: Some(
                        "Number of scanned channels minus one.",
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
                Field {
                    name: "scan_sel",
                    description: Some(
                        "Scan source: 0 selects the touch-key block, 1 selects the ADC.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ie_scan_end",
                    description: Some(
                        "Enable an interrupt when one scan pass ends.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 29,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "scan_mask_dis",
                    description: Some(
                        "Disable the channel mask after the sampled channel ends.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "if_scan_end",
                    description: Some(
                        "Scan pass end flag; write 1 to clear.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "TemSensor",
            extends: None,
            description: Some(
                "RW, temperature sensor control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "tem_sen_pwr_on",
                    description: Some(
                        "RW, temperature sensor power control: 0=power down, 1=power on.",
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
    ],
    enums: &[],
};
                