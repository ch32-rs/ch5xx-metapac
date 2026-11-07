
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
                    name: "clk_pll_div",
                    description: Some(
                        "output clock divider from PLL.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "ClkPllDiv",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clk_cfg_ctrl",
                    description: Some(
                        "clock control.",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "ClkCfgCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clk_mod_aux",
                    description: Some(
                        "clock mode aux register.",
                    ),
                    array: None,
                    byte_offset: 0xb,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "ClkModAux",
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
                    name: "slp_power_ctrl",
                    description: Some(
                        "power control.",
                    ),
                    array: None,
                    byte_offset: 0xf,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SlpPowerCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pin_alternate",
                    description: Some(
                        "alternate pin control.",
                    ),
                    array: None,
                    byte_offset: 0x12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "PinAlternate",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "serd_ana_cfg1",
                    description: Some(
                        "Serdes Analog parameter configuration1.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "SerdAnaCfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "serd_ana_cfg2",
                    description: Some(
                        "Serdes Analog parameter configuration2.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "SerdAnaCfg2",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "ClkCfgCtrl",
            extends: None,
            description: Some(
                "clock control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "clk_pll_sleep",
                    description: Some(
                        "PLL sleep control.",
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
                    name: "clk_sel_pll",
                    description: Some(
                        "clock source selection.",
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
            name: "ClkModAux",
            extends: None,
            description: Some(
                "clock mode aux register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "int_125m_en",
                    description: Some(
                        "clock from USB_PHY PCLK(125MHz).",
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
                    name: "ext_125m_en",
                    description: Some(
                        "clock from pin_PA[16].",
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
                    name: "mco_sel_msk",
                    description: Some(
                        "MCO output selection.",
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
                    name: "mco_en",
                    description: Some(
                        "MCO output enable.",
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
            name: "ClkPllDiv",
            extends: None,
            description: Some(
                "output clock divider from PLL.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "clk_pll_div",
                    description: Some(
                        "output clock divider from PLL.",
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
            name: "PinAlternate",
            extends: None,
            description: Some(
                "alternate pin control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pin_mii",
                    description: Some(
                        "ETH mii interface selection.",
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
                    name: "pin_tmr1",
                    description: Some(
                        "TMR1 alternate pin enable.",
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
                    name: "pin_tmr2",
                    description: Some(
                        "TMR2 alternate pin enable.",
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
                    name: "pin_uart0",
                    description: Some(
                        "RXD0/TXD0 alternate pin enable.",
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
            name: "SerdAnaCfg1",
            extends: None,
            description: Some(
                "Serdes Analog parameter configuration1.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "serd_pll_cfg",
                    description: Some(
                        "SerDes PHY internal configuration bit.",
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
                    name: "serd_30m_sel",
                    description: Some(
                        "SerDes PHY reference clock source seletion.",
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
                    name: "serd_dn_sel",
                    description: Some(
                        "Enable SerDes PHY GXM test pin.",
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
            ],
        },
        FieldSet {
            name: "SerdAnaCfg2",
            extends: None,
            description: Some(
                "Serdes Analog parameter configuration2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "serd_trx_cfg",
                    description: Some(
                        "Tx and RX parameter setting.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 25,
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
                    name: "slp_clk_pwmx",
                    description: Some(
                        "sleep PWMX clock.",
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
                    name: "slp_clk_uart0",
                    description: Some(
                        "sleep UART0 clock.",
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
                    name: "slp_clk_uart1",
                    description: Some(
                        "sleep UART1 clock.",
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
                    name: "slp_clk_uart2",
                    description: Some(
                        "sleep UART2 clock.",
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
                    name: "slp_clk_uart3",
                    description: Some(
                        "sleep UART3 clock.",
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
                    name: "slp_clk_spi0",
                    description: Some(
                        "sleep SPI0 clock.",
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
                    name: "slp_clk_spi1",
                    description: Some(
                        "sleep SPI1 clock.",
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
                    name: "slp_clk_emmc",
                    description: Some(
                        "sleep EMMC clock.",
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
                    name: "slp_clk_hspi",
                    description: Some(
                        "sleep HSPI clock.",
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
                    name: "slp_clk_usbhs",
                    description: Some(
                        "sleep USBHS clock.",
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
                    name: "slp_clk_usbss",
                    description: Some(
                        "sleep USBSS clock.",
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
                    name: "slp_clk_serd",
                    description: Some(
                        "sleep SERD clock.",
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
                    name: "slp_clk_dvp",
                    description: Some(
                        "sleep DVP clock.",
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
            name: "SlpPowerCtrl",
            extends: None,
            description: Some(
                "power control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "slp_usbhs_pwrdn",
                    description: Some(
                        "enable USBHS power down.",
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
            name: "SlpWakeCtrl",
            extends: None,
            description: Some(
                "wake control.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "slp_usbhs_wake",
                    description: Some(
                        "enable USBHS waking.",
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
                    name: "slp_usbss_wake",
                    description: Some(
                        "enable USBSS waking.",
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
                    name: "slp_clk_eth",
                    description: Some(
                        "sleep ETH clock.",
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
                    name: "slp_clk_ecdc",
                    description: Some(
                        "sleep ECDC clock.",
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
                    name: "slp_gpio_wake",
                    description: Some(
                        "enable GPIO waking.",
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
                    name: "slp_eth_wake",
                    description: Some(
                        "enable Eth waking.",
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
            ],
        },
    ],
    enums: &[],
};
                