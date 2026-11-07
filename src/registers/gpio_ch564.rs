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
                    name: "int_status_pa",
                    description: Some(
                        "The interrupt status register at the PA port.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntStatusPa",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_enable_pa",
                    description: Some(
                        "The interrupt enable register at the PA port.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntEnablePa",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_mode_pa",
                    description: Some(
                        "The interrupt mode register at the PA port.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntModePa",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_polar_pa",
                    description: Some(
                        "The interrupt polar register at the PA port.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntPolarPa",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_status_pb",
                    description: Some(
                        "The interrupt status register at the PB port.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntStatusPb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_enable_pb",
                    description: Some(
                        "The interrupt enable register at the PB port.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntEnablePb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_mode_pb",
                    description: Some(
                        "The interrupt mode register at the PB port.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntModePb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_polar_pb",
                    description: Some(
                        "The interrupt polar register at the PB port.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntPolarPb",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_status_pd",
                    description: Some(
                        "The interrupt status register at the PA port.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntStatusPd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_enable_pd",
                    description: Some(
                        "The interrupt enable register at the PD port.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntEnablePd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_mode_pd",
                    description: Some(
                        "The interrupt mode register at the PD port.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntModePd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "int_polar_pd",
                    description: Some(
                        "The interrupt polar register at the PD port.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IntPolarPd",
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
                    byte_offset: 0x80,
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
                    byte_offset: 0x84,
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
                    byte_offset: 0x88,
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
                    byte_offset: 0x8c,
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
                    byte_offset: 0x90,
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
                        "GPIO PA output open-drain_input pulldown resistance enable.",
                    ),
                    array: None,
                    byte_offset: 0x94,
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
                    name: "pb_dir",
                    description: Some(
                        "GPIO PB I/O direction.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
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
                    byte_offset: 0xa4,
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
                    byte_offset: 0xa8,
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
                    byte_offset: 0xac,
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
                    byte_offset: 0xb0,
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
                        "GPIO PB output open-drain_input pulldown resistance enable.",
                    ),
                    array: None,
                    byte_offset: 0xb4,
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
                    name: "pd_dir",
                    description: Some(
                        "GPIO PD I/O direction.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PdDir",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pd_pin",
                    description: Some(
                        "GPIO PD input.",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "PdPin",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pd_out",
                    description: Some(
                        "GPIO PD output.",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PdOut",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pd_pu",
                    description: Some(
                        "GPIO PD pullup resistance enable.",
                    ),
                    array: None,
                    byte_offset: 0xd0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PdPu",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pd_pd",
                    description: Some(
                        "GPIO PD output open-drain_input pulldown resistance enable.",
                    ),
                    array: None,
                    byte_offset: 0xd4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PdPd",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "afio_pcfr1",
                    description: Some(
                        "AF remap and debug I/O configuration register (AFIO_PCFR1).",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AfioPcfr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exten_ctlr0",
                    description: Some(
                        "Configure the extended control register 0.",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ExtenCtlr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exten_ctlr1",
                    description: Some(
                        "Configure the extended control register 1.",
                    ),
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ExtenCtlr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "afio_pcfr2",
                    description: Some(
                        "AF remap and debug I/O configuration register (AFIO_PCFR2).",
                    ),
                    array: None,
                    byte_offset: 0x160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "AfioPcfr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "exten_ctlr2",
                    description: Some(
                        "Configure the extended control register 2.",
                    ),
                    array: None,
                    byte_offset: 0x16c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "ExtenCtlr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "adc_mode",
                    description: Some(
                        "ADC port mode configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x16e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "AdcMode",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "AdcMode",
            extends: None,
            description: Some(
                "ADC port mode configuration register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "adc_gpio_mode",
                    description: Some(
                        "ADC external channel input mode selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "AfioPcfr1",
            extends: None,
            description: Some(
                "AF remap and debug I/O configuration register (AFIO_PCFR1).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spi0_rm",
                    description: Some(
                        "SPI0 remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart0_rm",
                    description: Some(
                        "UART0 remapping.",
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
                    name: "uart1_rm",
                    description: Some(
                        "UART1 remapping.",
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
                    name: "uart2_rm",
                    description: Some(
                        "UART2 remapping.",
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
                Field {
                    name: "uart3_rm",
                    description: Some(
                        "UART3 remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart0_modem_rm",
                    description: Some(
                        "UART0 MODEM remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart1_modem_rm",
                    description: Some(
                        "UART1 MODEM remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart2_modem_rm",
                    description: Some(
                        "UART2 MODEM remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c_rm",
                    description: Some(
                        "I2C remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "slv_interrupt_rm",
                    description: Some(
                        "SLV interrupt pin remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "slv_cs_rm",
                    description: Some(
                        "SLV piece selection pin remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "slv_addr_rm",
                    description: Some(
                        "SLV command data selection input pin remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "slv_addr1_rm",
                    description: Some(
                        "SLV address1 remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "slv_data_rm",
                    description: Some(
                        "SLV data remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "slv_rw_rm",
                    description: Some(
                        "SLV remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 28,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "link_led_rm",
                    description: Some(
                        "LINK LED remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 30,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "AfioPcfr2",
            extends: None,
            description: Some(
                "AF remap and debug I/O configuration register (AFIO_PCFR2).",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "act_led_rm",
                    description: Some(
                        "active led remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rst_rm",
                    description: Some(
                        "reset pin remapping.",
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
                    name: "timer0_rm",
                    description: Some(
                        "TIMER0 remapping.",
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
                    name: "timer1_rm",
                    description: Some(
                        "TIMER1 remapping.",
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
                    name: "busy_rm",
                    description: Some(
                        "BUSY remapping.",
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
                    name: "spi1_rm",
                    description: Some(
                        "UART3 remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tnow0_rm",
                    description: Some(
                        "TNOW0 remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tnow1_rm",
                    description: Some(
                        "TNOW1 remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tnow2_rm",
                    description: Some(
                        "TNOW2 remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tnow3_rm",
                    description: Some(
                        "TNOW3 remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uart3_modem",
                    description: Some(
                        "UART3 MODEM remapping.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "ExtenCtlr0",
            extends: None,
            description: Some(
                "Configure the extended control register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "sw_cfg",
                    description: Some(
                        "Serial wire JTAG configuration.",
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
                    name: "usbpllsrc",
                    description: Some(
                        "USBPLL reference clock source.",
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
                    name: "sw",
                    description: Some(
                        "select the systemitem clock source.",
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
                    name: "tnow0_en",
                    description: Some(
                        "the TNOW signal output of the UART0 is enabled.",
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
                    name: "tnow1_en",
                    description: Some(
                        "the TNOW signal output of the UART1 is enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tnow2_en",
                    description: Some(
                        "the TNOW signal output of the UART2 is enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tnow3_en",
                    description: Some(
                        "the TNOW signal output of the UART3 is enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "busy_en",
                    description: Some(
                        "the busy signal output of spi and slv enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rst_cmd_en",
                    description: Some(
                        "global reset enabled.",
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
                    name: "usbpllmul",
                    description: Some(
                        "USBPLL input clock frequency multiplication factor.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ana_eth_phy_rst",
                    description: Some(
                        "ethernet phy analog module reset control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dig_eth_phy_rst",
                    description: Some(
                        "ethernet phy digital module reset control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ethrst",
                    description: Some(
                        "ethernet reset control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flash_pre_en",
                    description: Some(
                        "flash clock pre-divisor enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usbpllon",
                    description: Some(
                        "USBPLL clock enable bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rst_dleay_en",
                    description: Some(
                        "reset time extension control bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "core_halt_int_en",
                    description: Some(
                        "Kernel error interrupt enabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_pll_rdy",
                    description: Some(
                        "USB PLL ready flag bit.",
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
                    name: "eth_pll_rdy",
                    description: Some(
                        "ETH PLL ready flag bit.",
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
                    name: "core_prot_status",
                    description: Some(
                        "Core protected moed status bit.",
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
            ],
        },
        FieldSet {
            name: "ExtenCtlr1",
            extends: None,
            description: Some(
                "Configure the extended control register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hseon",
                    description: Some(
                        "External High Speed clock enable.",
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
                    name: "hsion",
                    description: Some(
                        "Internal High Speed clock enable.",
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
                    name: "vio_rdy",
                    description: Some(
                        "VIO status.",
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
                    name: "hse_status",
                    description: Some(
                        "External crystal current level status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clksel",
                    description: Some(
                        "Clock signal selection.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ldo_slp_en",
                    description: Some(
                        "Enable the LDO to go into sleep mode when the VIO power is. down.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vio_pwn_io_en",
                    description: Some(
                        "Enable the IO port of the peripheral to input a high level. when the VIO power is down.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vio_pwn_rst_en",
                    description: Some(
                        "Enable system reset when the VIO power is down.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vio_pwn_int_en",
                    description: Some(
                        "Enables an outage when the VIO power is down.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "ExtenCtlr2",
            extends: None,
            description: Some(
                "Configure the extended control register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usbpd_in_hvt0",
                    description: Some(
                        "PD PIN PB18 high threshold input mode.",
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
                    name: "usbpd_in_hvt1",
                    description: Some(
                        "PD PIN PB19 high threshold input mode.",
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
                    name: "xi_oe",
                    description: Some(
                        "XI ouput enable bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xo_oe",
                    description: Some(
                        "XO ouput enable bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "xixo_gpio_en",
                    description: Some(
                        "XI/XO pin open-drain_ouput control bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IntEnablePa",
            extends: None,
            description: Some(
                "The interrupt enable register at the PA port.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pa",
                    description: Some(
                        "The interrupt enable bit of the PA port.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IntEnablePb",
            extends: None,
            description: Some(
                "The interrupt enable register at the PB port.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pb",
                    description: Some(
                        "The interrupt enable bit of the PB port.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 23,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IntEnablePd",
            extends: None,
            description: Some(
                "The interrupt enable register at the PD port.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd",
                    description: Some(
                        "The interrupt enable bit of the PD port.",
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
            name: "IntModePa",
            extends: None,
            description: Some(
                "The interrupt mode register at the PA port.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pa",
                    description: Some(
                        "The interrupt mode bit of the PA port.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IntModePb",
            extends: None,
            description: Some(
                "The interrupt mode register at the PB port.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pb",
                    description: Some(
                        "The interrupt mode bit of the PB port.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 23,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IntModePd",
            extends: None,
            description: Some(
                "The interrupt mode register at the PD port.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd",
                    description: Some(
                        "The interrupt mode bit of the PD port.",
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
            name: "IntPolarPa",
            extends: None,
            description: Some(
                "The interrupt polar register at the PA port.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pa",
                    description: Some(
                        "The interrupt polar bit of the PA port.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IntPolarPb",
            extends: None,
            description: Some(
                "The interrupt polar register at the PB port.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pb",
                    description: Some(
                        "The interrupt polar bit of the PB port.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 23,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IntPolarPd",
            extends: None,
            description: Some(
                "The interrupt polar register at the PD port.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd",
                    description: Some(
                        "The interrupt polar bit of the PD port.",
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
            name: "IntStatusPa",
            extends: None,
            description: Some(
                "The interrupt status register at the PA port.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vio_pwd_if",
                    description: Some(
                        "VIO power down interrupt flag bit.",
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
                    name: "pa",
                    description: Some(
                        "The interrupt status bit of the PA port.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IntStatusPb",
            extends: None,
            description: Some(
                "The interrupt status register at the PB port.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pb",
                    description: Some(
                        "The interrupt status bit of the PB port.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 23,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "IntStatusPd",
            extends: None,
            description: Some(
                "The interrupt status register at the PD port.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd",
                    description: Some(
                        "The interrupt status bit of the PD port.",
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
                "GPIO PA output open-drain_input pulldown resistance enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pa_pd",
                    description: Some(
                        "GPIO PA pull-down resistor (input) or open-drain output enable: 1=enabled, 0=disabled.",
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
                                len: 23,
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
                                len: 23,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "usb_dp_dir",
                    description: Some(
                        "Pin current intput/output direction control domain.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_dm_dir",
                    description: Some(
                        "Pin current intput/output direction control domain.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
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
                                len: 23,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "usb_udp_out",
                    description: Some(
                        "Pin output level status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_udm_out",
                    description: Some(
                        "Pin output level status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PbPd",
            extends: None,
            description: Some(
                "GPIO PB output open-drain_input pulldown resistance enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pb_pd",
                    description: Some(
                        "GPIO PB pull-down resistor (input) or open-drain output enable: 1=enabled, 0=disabled.",
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
                                len: 23,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "usb_udp_pd",
                    description: Some(
                        "Pin function enable /close control bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_udm_pd",
                    description: Some(
                        "Pin function enable /close control bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
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
                                len: 23,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "usb_udp_in",
                    description: Some(
                        "Pin intput level status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_udm_in",
                    description: Some(
                        "Pin intput level status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
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
                                len: 23,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "usb_udp_pu",
                    description: Some(
                        "Pin pull-up function enable /close control bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "usb_udm_pu",
                    description: Some(
                        "Pin pull-up function enable /close control bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PdDir",
            extends: None,
            description: Some(
                "GPIO PD I/O direction.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd_dir",
                    description: Some(
                        "GPIO PD I/O direction: 0=input, 1=output.",
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
            name: "PdOut",
            extends: None,
            description: Some(
                "GPIO PD output.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd_out",
                    description: Some(
                        "GPIO PD output level: 1=high, 0=low.",
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
            name: "PdPd",
            extends: None,
            description: Some(
                "GPIO PD output open-drain_input pulldown resistance enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd_pd",
                    description: Some(
                        "GPIO PD pull-down resistor (input) or open-drain output enable: 1=enabled, 0=disabled.",
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
            name: "PdPin",
            extends: None,
            description: Some(
                "GPIO PD input.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd_pin",
                    description: Some(
                        "GPIO PD input level: 1=high, 0=low.",
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
            name: "PdPu",
            extends: None,
            description: Some(
                "GPIO PD pullup resistance enable.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd_pu",
                    description: Some(
                        "GPIO PD pull-up resistor: 1=enabled, 0=disabled.",
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
