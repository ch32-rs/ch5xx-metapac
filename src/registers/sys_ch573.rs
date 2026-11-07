
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
                    name: "glob_rom_cfg",
                    description: Some(
                        "RWA, flash ROM configuration.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "GlobRomCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "reset_status",
                    description: Some(
                        "RWA, reset status.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 8,
                            fieldset: Some(
                                "ResetStatus",
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
                            bit_size: 16,
                            fieldset: Some(
                                "ClkCfg",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "hfck_pwr_ctrl",
                    description: Some(
                        "RWA, high frequency clock module power control, SAM.",
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
                    name: "pin_analog_ie",
                    description: Some(
                        "RW, analog pin enable and digital input disable.",
                    ),
                    array: None,
                    byte_offset: 0x1a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "PinAnalogIe",
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
                            bit_size: 8,
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
                    name: "int32k_tune",
                    description: Some(
                        "RWA, internal 32KHz oscillator tune control, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "Int32kTune",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "xt32k_tune",
                    description: Some(
                        "RWA, external 32KHz oscillator tune control, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x2e,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Xt32kTune",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ck32k_config",
                    description: Some(
                        "RWA, 32KHz oscillator configure.",
                    ),
                    array: None,
                    byte_offset: 0x2f,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "Ck32kConfig",
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
            bit_size: 8,
            fields: &[
                Field {
                    name: "ulpldo_adj",
                    description: Some(
                        "RWA, Ultra-Low-Power LDO voltage adjust.",
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
                    name: "bat_det_en_rb_bat_low_vthx",
                    description: Some(
                        "RWA, battery voltage detector enable/select monitor threshold voltage.",
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
                        "RWA, battery voltage monitor enable under sleep mode.",
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
                    name: "bat_lower_ie",
                    description: Some(
                        "RWA, interrupt enable for battery lower voltage.",
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
                    name: "bat_low_ie",
                    description: Some(
                        "RWA, interrupt enable for battery low voltage.",
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
                    name: "bat_stat_lower",
                    description: Some(
                        "RO, battery lower voltage status, high action.",
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
            name: "Ck32kConfig",
            extends: None,
            description: Some(
                "RWA, 32KHz oscillator configure.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "clk_xt32k_pon",
                    description: Some(
                        "RWA, external 32KHz oscillator power on.",
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
                    name: "clk_int32k_pon",
                    description: Some(
                        "RWA, internal 32KHz oscillator power on.",
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
                    name: "clk_osc32k_xt",
                    description: Some(
                        "RWA, 32KHz oscillator source selection: 0=RC, 1=XT.",
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
                    name: "_32k_clk_pin",
                    description: Some(
                        "RO, 32KHz oscillator clock pin status.",
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
            name: "ClkCfg",
            extends: None,
            description: Some(
                "RWA, system clock configuration, SAM.",
            ),
            bit_size: 16,
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
                        "RWA, system clock source mode: 00=divided from 32MHz, 01=divided from PLL-480MHz, 10=directly from 32MHz, 11=directly from 32KHz.",
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
            name: "GlobRomCfg",
            extends: None,
            description: Some(
                "RWA, flash ROM configuration.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "rom_code_ofs",
                    description: Some(
                        "RWA, code offset address selection in Flash ROM: 0=start address 0x000000, 1=start address 0x008000.",
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
                    name: "rom_ctrl_en",
                    description: Some(
                        "RWA, enable flash ROM control interface enable.",
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
                    name: "rom_data_we",
                    description: Some(
                        "RWA,enable flash ROM data and code area being erase/write.",
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
                    name: "rom_code_we",
                    description: Some(
                        "RWA, enable flash ROM code area being erase or write.",
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
            name: "HfckPwrCtrl",
            extends: None,
            description: Some(
                "RWA, high frequency clock module power control, SAM.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "clk_xt32m_pon",
                    description: Some(
                        "RWA, external 32MHz oscillator power control: 0=power down, 1-power on.",
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
                    name: "clk_pll_pon",
                    description: Some(
                        "RWA, PLL power control: 0=power down, 1-power on.",
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
            name: "Int32kTune",
            extends: None,
            description: Some(
                "RWA, internal 32KHz oscillator tune control, SAM.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "int32k_tune",
                    description: Some(
                        "RWA, internal 32KHz oscillator frequency tune.",
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
            name: "PinAlternate",
            extends: None,
            description: Some(
                "RW, function pin alternate configuration.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "pin_tmr0",
                    description: Some(
                        "RW, TMR0 alternate pin enable.",
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
                        "RW, TMR1 alternate pin enable.",
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
                        "RW, TMR2 alternate pin enable.",
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
                        "RW, RXD0/TXD0 alternate pin enable.",
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
                    name: "pin_uart1",
                    description: Some(
                        "RW, RXD1/TXD1 alternate pin enable.",
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
                    name: "pin_spi0",
                    description: Some(
                        "RW, SCS/SCK0/MOSI/MISO alternate pin enable.",
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
            name: "PinAnalogIe",
            extends: None,
            description: Some(
                "RW, analog pin enable and digital input disable.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "pin_usb_dp_pu",
                    description: Some(
                        "RW,USB UDP internal pullup resistance enable.",
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
                    name: "pin_usb_ie",
                    description: Some(
                        "RW, USB analog I/O enable: 0=analog I/O disable, 1=analog I/O enable.",
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
                    name: "pin_adc8_9_ie",
                    description: Some(
                        "RW, ADC/TouchKey channel 9/8 digital input disable: 0=digital input enable, 1=digital input disable.",
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
                    name: "pin_adc0_ie",
                    description: Some(
                        "RW, ADC/TouchKey channel0 digital input disable: 0=digital input enable, 1=digital input disable.",
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
                    name: "pin_adc1_ie",
                    description: Some(
                        "RW, ADC/TouchKey channel1 digital input disable: 0=digital input enable, 1=digital input disable.",
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
                    name: "pin_adc12_ie",
                    description: Some(
                        "RW, ADC/TouchKey channel12 digital input disable: 0=digital input enable, 1=digital input disable.",
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
                    name: "pin_adc13_ie",
                    description: Some(
                        "RW, ADC/TouchKey channel13 digital input disable: 0=digital input enable, 1=digital input disable.",
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
                    name: "pin_xt32k_ie",
                    description: Some(
                        "RW, external 32KHz oscillator digital input disable: 0=digital input enable, 1=digital input disable.",
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
                    name: "pin_adc2_3_ie",
                    description: Some(
                        "RW, ADC/TouchKey channel 2/3 digital input disable: 0=digital input enable, 1=digital input disable.",
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
                Field {
                    name: "pin_adc4_5_ie",
                    description: Some(
                        "RW, ADC/TouchKey channel 4/5 digital input disable: 0=digital input enable, 1=digital input disable.",
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
                        "RWA, power for retention 2KB SRAM.",
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
                    name: "pwr_ram2k",
                    description: Some(
                        "RWA, power for retention 2KB SRAM.",
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
                    name: "pwr_ram16k",
                    description: Some(
                        "RWA, power for main SRAM.",
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
                    name: "pwr_ldo_en",
                    description: Some(
                        "RWA, LDO enable.",
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
                    name: "pwr_dcdc_en",
                    description: Some(
                        "RWA, DC/DC converter enable: 0=DC/DC disable and bypass, 1=DC/DC enable.",
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
                    name: "pwr_dcdc_pre",
                    description: Some(
                        "RWA, DC/DC converter pre-enable.",
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
                    name: "pwr_must_0010",
                    description: Some(
                        "RWA, power plan enable, auto clear after sleep executed.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwr_plan_en",
                    description: Some(
                        "RWA, must write 0010.",
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
            name: "ResetStatus",
            extends: None,
            description: Some(
                "RWA, reset status.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "reset_flag",
                    description: Some(
                        "RO, recent reset flag.",
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
                    name: "slp_clk_tmr0",
                    description: Some(
                        "RWA, close TMR0 clock.",
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
                        "RWA, close TMR1 clock.",
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
                        "RWA, close TMR2 clock.",
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
                        "RWA, close TMR3 clock.",
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
                        "RWA, close UART0 clock.",
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
                        "RWA, close UART1 clock.",
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
                        "RWA, close UART2 clock.",
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
                        "RWA, close UART3 clock.",
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
                    name: "slp_clk_spi0",
                    description: Some(
                        "RWA, close SPI0 clock.",
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
            name: "SlpPowerCtrl",
            extends: None,
            description: Some(
                "RWA, peripherals power down control, SAM.",
            ),
            bit_size: 8,
            fields: &[
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
                    name: "slp_clk_ram2k",
                    description: Some(
                        "RWA, close retention 2KB SRAM clock.",
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
                    name: "ram_ret_lv",
                    description: Some(
                        "RWA, SRAM retention voltage selection.",
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
                    name: "wake_delay",
                    description: Some(
                        "RWA, wakeup delay.",
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
            name: "Xt32kTune",
            extends: None,
            description: Some(
                "RWA, external 32KHz oscillator tune control, SAM.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "xt32k_i_tune",
                    description: Some(
                        "RWA, external 32KHz oscillator current tune: 00=75% current, 01=standard current, 10=150% current, 11=200% current.",
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
                    name: "xt32k_c_load",
                    description: Some(
                        "RWA, external 32KHz oscillator load capacitor tune: Cap = RB_XT32K_C_LOAD + 12pF.",
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
    ],
    enums: &[],
};
                