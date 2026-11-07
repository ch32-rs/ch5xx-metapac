
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Sys",
            extends: None,
            description: Some(
                "SYS register.",
            ),
            items: &[
                BlockItem {
                    name: "pll_out_div",
                    description: Some(
                        "output clock divider from PLL.",
                    ),
                    array: None,
                    byte_offset: 0x9,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "PllOutDiv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_clk_off0",
                    description: Some(
                        "sleep clock off control byte 0.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SlpClkOff0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_clk_off1",
                    description: Some(
                        "sleep clock off control byte 1.",
                    ),
                    array: None,
                    byte_offset: 0xd,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SlpClkOff1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_wake_ctrl",
                    description: Some(
                        "wake control.",
                    ),
                    array: None,
                    byte_offset: 0xe,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SlpWakeCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_ctrl_pll",
                    description: Some(
                        "PLL SLEEP control.",
                    ),
                    array: None,
                    byte_offset: 0xf,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SlpCtrlPll",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "xbus_config",
                    description: Some(
                        "external bus configuration.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "XbusConfig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "xbus_cycle",
                    description: Some(
                        "external bus read/write cyccles register.",
                    ),
                    array: None,
                    byte_offset: 0x12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "XbusCycle",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "xbus_setup_hold",
                    description: Some(
                        "external bus holdtime setting register.",
                    ),
                    array: None,
                    byte_offset: 0x13,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "XbusSetupHold",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "PllOutDiv",
            extends: None,
            description: Some(
                "output clock divider from PLL.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pll_sys_div",
                    description: Some(
                        "System clock division factor.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "SlpClkOff0",
            extends: None,
            description: Some(
                "sleep clock off control byte 0.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "slp_clk_tmr0",
                    description: Some(
                        "sleep TMR0 clock.",
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
                    name: "slp_clk_tmr1",
                    description: Some(
                        "sleep TMR1 clock.",
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
                    name: "slp_clk_tmr2",
                    description: Some(
                        "sleep TMR2 clock.",
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
                    name: "slp_clk_tmr3",
                    description: Some(
                        "sleep TMR3 clock.",
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
                    name: "slp_clk_spi0",
                    description: Some(
                        "sleep SPI0 clock.",
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
                    name: "slp_clk_spi1",
                    description: Some(
                        "sleep SPI1 clock.",
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
                    name: "slp_clk_uart0",
                    description: Some(
                        "sleep UART0 clock.",
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
                    name: "slp_clk_uart1",
                    description: Some(
                        "sleep UART1 clock.",
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
            name: "SlpClkOff1",
            extends: None,
            description: Some(
                "sleep clock off control byte 1.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "slp_clk_utmi",
                    description: Some(
                        "sleep UTMI clock.",
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
                    name: "slp_clk_i2c",
                    description: Some(
                        "sleep I2C clock.",
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
                    name: "slp_clk_usbpd",
                    description: Some(
                        "sleep USBPD clock.",
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
                    name: "slp_clk_adc",
                    description: Some(
                        "sleep ADC clock.",
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
                    name: "slp_clk_gpio",
                    description: Some(
                        "sleep GPIO clock.",
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
                    name: "slp_clk_usb",
                    description: Some(
                        "sleep USB clock.",
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
                    name: "slp_clk_eth",
                    description: Some(
                        "sleep Ethernet clock.",
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
            name: "SlpCtrlPll",
            extends: None,
            description: Some(
                "PLL SLEEP control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "slp_wol_status",
                    description: Some(
                        "Current ethernet WOL remote wake-up signal status.",
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
                    name: "slp_clk_uart2",
                    description: Some(
                        "sleep UART2 clock.",
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
                    name: "slp_clk_uart3",
                    description: Some(
                        "sleep UART3 clock.",
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
            name: "SlpWakeCtrl",
            extends: None,
            description: Some(
                "wake control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "slp_pa_wake",
                    description: Some(
                        "enable PA waking.",
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
                    name: "slp_pb_wake",
                    description: Some(
                        "enable PB waking.",
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
                    name: "slp_usbpd_wake",
                    description: Some(
                        "enable USBPD waking.",
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
                    name: "slp_pd_wake",
                    description: Some(
                        "enable PD waking.",
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
                    name: "slp_usb_wake",
                    description: Some(
                        "enable USB waking.",
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
                    name: "slp_wol_wake",
                    description: Some(
                        "enable Ethernet WOL waking.",
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
                    name: "slp_eth_pwr_dn",
                    description: Some(
                        "Ethernet module power control.",
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
            name: "XbusConfig",
            extends: None,
            description: Some(
                "external bus configuration.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "xbus_enable",
                    description: Some(
                        "external bus enable.",
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
                    name: "xbus_en_32bit",
                    description: Some(
                        "external bus data control enable.",
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
                    name: "xbus_addr_oe",
                    description: Some(
                        "external bus address output enable.",
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
            ],
        },
        FieldSet {
            name: "XbusCycle",
            extends: None,
            description: Some(
                "external bus read/write cyccles register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "xbus_cycle",
                    description: Some(
                        "external bus read/write cyccles.",
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
            name: "XbusSetupHold",
            extends: None,
            description: Some(
                "external bus holdtime setting register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "xbus_hold",
                    description: Some(
                        "Number of external bus read/write signal end point clock.",
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
                    name: "xbus_setup",
                    description: Some(
                        "The address of the external bus and the creation time of. the data.",
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
                