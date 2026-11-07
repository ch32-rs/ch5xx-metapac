
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Sys",
            extends: None,
            description: Some(
                "System Control Register.",
            ),
            items: &[
                BlockItem {
                    name: "iwdg_kr",
                    description: Some(
                        "WO, Watchdog key register, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "IwdgKr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "iwdg_cfg",
                    description: Some(
                        "RW, Watchdog configuration register, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "IwdgCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clk_cfg",
                    description: Some(
                        "RWA, system clock configuration, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "ClkCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hfck_pwr_ctrl",
                    description: Some(
                        "RWA, High frequency clock module power control register, SAM.",
                    ),
                    array: None,
                    byte_offset: 0xa,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "HfckPwrCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_clk_off0",
                    description: Some(
                        "RWA, sleep clock off control byte 0, SAM.",
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
                        "RWA, sleep clock off control byte 1, SAM.",
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
                        "RWA, wake control, SAM.",
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
                        "RWA, peripherals power down control, SAM.",
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
                        "RW, function pin alternate configuration.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "PinAlternate",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pin_alternate_h",
                    description: Some(
                        "RW, function pin high alternate configuration.",
                    ),
                    array: None,
                    byte_offset: 0x1a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "PinAlternateH",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "long_rst_cfg",
                    description: Some(
                        "RWA, Long reset configuration register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "LongRstCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_clk_off2",
                    description: Some(
                        "RWA, Sleep clock control register 2.",
                    ),
                    array: None,
                    byte_offset: 0x1d,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SlpClkOff2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "slp_wake_cfg",
                    description: Some(
                        "RWA, Sleep clock control register.",
                    ),
                    array: None,
                    byte_offset: 0x1e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "SlpWakeCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "power_plan",
                    description: Some(
                        "RWA, power plan before sleep instruction, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "PowerPlan",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "aux_power_adj",
                    description: Some(
                        "RWA, aux power adjust control, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x22,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "AuxPowerAdj",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bat_det_ctrl",
                    description: Some(
                        "RWA, battery voltage detector control, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "BatDetCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bat_det_cfg",
                    description: Some(
                        "RWA, battery voltage detector configuration, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x25,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "BatDetCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bat_status",
                    description: Some(
                        "RO, battery status.",
                    ),
                    array: None,
                    byte_offset: 0x26,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "BatStatus",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lsi_config",
                    description: Some(
                        "RWA, LSI configure register.",
                    ),
                    array: None,
                    byte_offset: 0x2f,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "LsiConfig",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "AuxPowerAdj",
            extends: None,
            description: Some(
                "RWA, aux power adjust control, SAM.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "ulpldo_adj",
                    description: Some(
                        "RWA, Output voltage regulation value of auxiliary power supply for ultra-low power consumption LDO.",
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
                    name: "cfg_ivref",
                    description: Some(
                        "RO, retain. The original value must be kept unchanged when writing.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "BatDetCfg",
            extends: None,
            description: Some(
                "RWA, battery voltage detector configuration, SAM.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "bat_low_vth",
                    description: Some(
                        "RWA, select threshold voltage of battery voltage low.",
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
            ],
        },
        FieldSet {
            name: "BatDetCtrl",
            extends: None,
            description: Some(
                "RWA, battery voltage detector control, SAM.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "pwr_ldo_en",
                    description: Some(
                        "RWA, -.",
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
                    name: "bat_mon_en",
                    description: Some(
                        "RWA, Enable low-power battery voltage monitoring function.",
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
                    name: "bat_low_ie",
                    description: Some(
                        "RWA, Battery low voltage interrupt enable.",
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
            name: "BatStatus",
            extends: None,
            description: Some(
                "RO, battery status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "bat_stat_low",
                    description: Some(
                        "RO, battery low voltage status, high action.",
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
            name: "ClkCfg",
            extends: None,
            description: Some(
                "RWA, system clock configuration, SAM.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "clk_pll_div",
                    description: Some(
                        "RWA, output clock divider from PLL or CK32M.",
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
                    name: "clk_sys_mod",
                    description: Some(
                        "RWA, system clock source mode: 00=divided from 32MHz.",
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
            name: "HfckPwrCtrl",
            extends: None,
            description: Some(
                "RWA, High frequency clock module power control register, SAM.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "clk_xt32m_pon",
                    description: Some(
                        "RWA, External 32MHz oscillator HSE power control bit.",
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
                    name: "clk_xt32m_keep",
                    description: Some(
                        "RWA, Used to control the stopping of the clock system in pause mode.",
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
                    name: "clk_pll_pon",
                    description: Some(
                        "RWA, PLL power control bit.",
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
            name: "IwdgCfg",
            extends: None,
            description: Some(
                "RW, Watchdog configuration register, SAM.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rlr",
                    description: Some(
                        "RW, RL [11:0]: Watchdog counter reload.",
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
                Field {
                    name: "pr",
                    description: Some(
                        "RO, Pre division factor (with write protection).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pvu",
                    description: Some(
                        "RO, Configure register update flag bit (with write protection).",
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
                    name: "count",
                    description: Some(
                        "RO, Watchdog countdown counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "stop_en",
                    description: Some(
                        "RW, Watchdog stop enabled (with write protection).",
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
                    name: "wr_protect",
                    description: Some(
                        "RW, Write protection.",
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
                    name: "iwdg_en",
                    description: Some(
                        "RW, Watch dog start switch: 1: ON; 0: Close.",
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
            name: "IwdgKr",
            extends: None,
            description: Some(
                "WO, Watchdog key register, SAM.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "iwdg_kr",
                    description: Some(
                        "RO,KEY [15:0]: Key value.",
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
            name: "LongRstCfg",
            extends: None,
            description: Some(
                "RWA, Long reset configuration register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "long_rst_en",
                    description: Some(
                        "RWA, Long reset enable.",
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
                    name: "long_tim_sel",
                    description: Some(
                        "RWA, Long reset duration selection.",
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
            ],
        },
        FieldSet {
            name: "LsiConfig",
            extends: None,
            description: Some(
                "RWA, LSI configure register.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "clk_lsi_pon",
                    description: Some(
                        "RWA, Internal low-frequency RC oscillator LSI power control bit.",
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
                    name: "lsi_clk_pin",
                    description: Some(
                        "RO, LSI clock pin status (asynchronous signal).",
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
            name: "PinAlternate",
            extends: None,
            description: Some(
                "RW, function pin alternate configuration.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "pa_di_dis",
                    description: Some(
                        "RW, PA0-PA11 channel pin digital input enable.",
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
                Field {
                    name: "udp_pu_en",
                    description: Some(
                        "RW, Enable internal pull-up resistor for UDP pin of full speed USB.",
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
                    name: "pin_usb_en",
                    description: Some(
                        "RW, Full speed USB pin enable.",
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
                    name: "pin_debug_en",
                    description: Some(
                        "RW, Debugging interface control bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PinAlternateH",
            extends: None,
            description: Some(
                "RW, function pin high alternate configuration.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "uart_rxd",
                    description: Some(
                        "RW, RXD alternate pin enable.",
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
                    name: "uart_txd",
                    description: Some(
                        "RW, TXD alternate pin enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tmr_pin",
                    description: Some(
                        "RW, Timer function pin mapping selection bit.",
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
                    name: "spi_cs",
                    description: Some(
                        "RW, SPI CS function pin mapping selection bit.",
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
                    name: "i2c_pin",
                    description: Some(
                        "RW, I2C function pin mapping selection bit.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "spi_clk",
                    description: Some(
                        "RW, SPI CLK function pin mapping selection bit.",
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
                    name: "_25m_en",
                    description: Some(
                        "RW, Enable 25MHz clock output.",
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
            ],
        },
        FieldSet {
            name: "PowerPlan",
            extends: None,
            description: Some(
                "RWA, power plan before sleep instruction, SAM.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "pwr_xrom",
                    description: Some(
                        "RWA, power for FlashROM.",
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
                    name: "pwr_ram12k",
                    description: Some(
                        "RWA, SRAM power supply for RAM12K.",
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
                    name: "pwr_core",
                    description: Some(
                        "RWA, power retention for core and base peripherals.",
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
                    name: "pwr_extend",
                    description: Some(
                        "RWA, power retention for USB and BLE.",
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
                    name: "main_act",
                    description: Some(
                        "RWA, Main power selection.",
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
                    name: "pwr_sys_en",
                    description: Some(
                        "RWA, power for system.",
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
                    name: "pwr_ldo5v_en",
                    description: Some(
                        "RWA, Internal LDO5V control.",
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
                    name: "pwr_plan_en",
                    description: Some(
                        "RWA, Enable sleep power planning and control.",
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
            ],
        },
        FieldSet {
            name: "SlpClkOff0",
            extends: None,
            description: Some(
                "RWA, sleep clock off control byte 0, SAM.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "slp_clk_tmr",
                    description: Some(
                        "RWA, Timer clock source.",
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
                    name: "slp_clk_cmp",
                    description: Some(
                        "RWA, CMP clock source.",
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
                    name: "slp_clk_uart",
                    description: Some(
                        "RWA, UART clock source.",
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
                    name: "slp_keyscan_wake",
                    description: Some(
                        "RWA, KEYSCAN wakeup enable.",
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
                "RWA, sleep clock off control byte 1, SAM.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "slp_clk_spi",
                    description: Some(
                        "RWA, close SPI clock.",
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
                    name: "clk_off_aesccm",
                    description: Some(
                        "RWA, close AES_CCM clock.",
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
                    name: "slp_clk_pwmx",
                    description: Some(
                        "RWA, close PWMx clock.",
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
                    name: "slp_clk_i2c",
                    description: Some(
                        "RWA, close I2C clock.",
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
                    name: "slp_clk_usb",
                    description: Some(
                        "RWA, close USB clock.",
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
                    name: "slp_clk_ble",
                    description: Some(
                        "RWA, close BLE clock.",
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
            name: "SlpClkOff2",
            extends: None,
            description: Some(
                "RWA, Sleep clock control register 2.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "clk_off_xrom",
                    description: Some(
                        "RWA, Turn off the 64M or 600M clock of the FLASH controller.",
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
                    name: "clk_off_debug",
                    description: Some(
                        "RWA, Close the two-wire debugging 32M clock.",
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
                    name: "clk_off_hclk",
                    description: Some(
                        "RWA, Turn off the HCLK clock of the FLASH controller.",
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
            name: "SlpPowerCtrl",
            extends: None,
            description: Some(
                "RWA, peripherals power down control, SAM.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "wake_dly_mod",
                    description: Some(
                        "RWA, wakeup delay time selection.",
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
                    name: "slp_clk_ramx",
                    description: Some(
                        "RWA, close main SRAM clock.",
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
                    name: "ram_ret_lv",
                    description: Some(
                        "RWA, SRAM retention voltage selection.",
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
            name: "SlpWakeCfg",
            extends: None,
            description: Some(
                "RWA, Sleep clock control register.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "oscclk_rdy_keep",
                    description: Some(
                        "RWA, During Halt sleep, the peripheral clock remains on.",
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
                    name: "preclk_cnt_en",
                    description: Some(
                        "RWA, When waking up from sleep, it takes a certain amount of time to release HCLK.",
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
                    name: "preclk_cnt_sel",
                    description: Some(
                        "RWA, Sleep time configuration, sleep duration=number of cycles * Fsys.",
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
                    name: "acauto_enable",
                    description: Some(
                        "RWA, The secure register channel automatically closes.",
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
            ],
        },
        FieldSet {
            name: "SlpWakeCtrl",
            extends: None,
            description: Some(
                "RWA, wake control, SAM.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "slp_usb_wake",
                    description: Some(
                        "RWA, enable USB waking.",
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
                    name: "slp_enc_wake",
                    description: Some(
                        "RWA, enable ENC waking.",
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
                    name: "slp_gpio_edge_mode",
                    description: Some(
                        "RWA, GPIO wake-up mode selection.",
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
                    name: "slp_rtc_wake",
                    description: Some(
                        "RWA, enable RTC waking.",
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
                        "RWA, enable GPIO waking.",
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
                    name: "slp_bat_wake",
                    description: Some(
                        "RWA, enable BAT waking.",
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
                    name: "wake_ev_mode",
                    description: Some(
                        "RWA, event wakeup mode.",
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
                    name: "gpio_wake_mode",
                    description: Some(
                        "RWA, enable edge GPIO wake-up mode.",
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
                