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
                    name: "safe_mode_ctrl",
                    description: Some(
                        "RWA, Secure Access Mode Control Register, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SafeModeCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "safe_clk_ctrl",
                    description: Some(
                        "RWA, Secure access to clock control registers, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x11,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SafeClkCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "safe_debug_ctrl",
                    description: Some(
                        "RWA, Secure Access Debugging Control Register, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SafeDebugCtrl",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "safe_lrst_ctrl",
                    description: Some(
                        "RWA, Long term reset control register for secure access, SAM.",
                    ),
                    array: None,
                    byte_offset: 0x13,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 8,
                            fieldset: Some(
                                "SafeLrstCtrl",
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
                    name: "pin_config",
                    description: Some(
                        "RW, Function pin configuration registere.",
                    ),
                    array: None,
                    byte_offset: 0x1a,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 16,
                            fieldset: Some(
                                "PinConfig",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pin_in_dis",
                    description: Some(
                        "RW, Function pin digital input disable register.",
                    ),
                    array: None,
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "PinInDis",
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
                        "RWA, External 32KHz oscillator tune control, SAM.",
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
                    name: "dcdc_charge",
                    description: Some(
                        "RWA, Low power auxiliary DC-DC enable bit.",
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
                    name: "cfg_ivref",
                    description: Some(
                        "RWA, retain. The original value must be kept unchanged when writing.",
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
                Field {
                    name: "ib_tkey_sel",
                    description: Some(
                        "RWA, TouchKey bias current, used to stabilize I/O potential and prevent accidental wake-up.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
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
                    name: "bat_det_en",
                    description: Some(
                        "RWA, When RB_BAT_MON_EN=0, it is enabled by high-precision battery voltage detection function.",
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
                    name: "bat_low_vthx",
                    description: Some(
                        "RWA, When RB_BAT_MON_EN=1, set low power consumption.",
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
                        "RWA, Battery ultra-low voltage interrupt enable.",
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
                    name: "clk_osc32k_filt",
                    description: Some(
                        "RWA, internal 32KHz oscillator low noise mode enable.",
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
                Field {
                    name: "xrom_sclk_sel",
                    description: Some(
                        "RWA, XROM clock source.",
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
                    name: "osc32m_sel",
                    description: Some(
                        "RWA, Selection of PLL clock source.",
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
                    name: "pll_gate_diss",
                    description: Some(
                        "RWA, Do you turn off the PLL clock when switching the PLL clock source.",
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
                    name: "pll_gate_time",
                    description: Some(
                        "RWA, When switching the PLL clock source, turn off the time selection of the PLL clock.",
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
                    name: "clk_rc16m_pon",
                    description: Some(
                        "RWA,Enable items for the internal 16MHz oscillator HSI.",
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
                        "RWA, Internal RC 32KHz clock frequency calibration value.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
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
                    name: "pin_tmr3",
                    description: Some(
                        "RW, TMR3 alternate pin enable.",
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
                    name: "pin_uart2",
                    description: Some(
                        "RW, RXD2/TXD2 alternate pin enable.",
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
                    name: "pin_uart3",
                    description: Some(
                        "RW, RXD3/TXD3 alternate pin enable.",
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
                Field {
                    name: "pin_pwmx",
                    description: Some(
                        "RW, PWM4/PWM5/PWM7/PWM8/PWM9 alternate pin enable.",
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
                    name: "pin_i2c",
                    description: Some(
                        "RW, SCL/SDA alternate pin enable.",
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
                    name: "pin_modem",
                    description: Some(
                        "RW, DSR/DTR alternate pin enable.",
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
                    name: "pin_intx",
                    description: Some(
                        "RW, INT24/INT25 function pin mapping selection bit.",
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
                    name: "pin_u0_inv",
                    description: Some(
                        "RW, UART0 input/output inversion enable.",
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
                    name: "rf_ant_sw_en",
                    description: Some(
                        "RW, RF antenna switch control output enable.",
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
            name: "PinConfig",
            extends: None,
            description: Some(
                "RW, Function pin configuration registere.",
            ),
            bit_size: 16,
            fields: &[
                Field {
                    name: "pb16_8_sel",
                    description: Some(
                        "RW, Selection of interrupt pins for PB8 and PB16.",
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
                    name: "pin_usb2_en",
                    description: Some(
                        "RW, Enable high-speed USB pins.",
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
                    name: "udp_pu_en",
                    description: Some(
                        "RW, Full speed USB UD+pin internal pull-up resistor enable.",
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
                    name: "pin_usb_en",
                    description: Some(
                        "RW, Full speed USB pin enable.",
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
                    name: "pbh_in_dis",
                    description: Some(
                        "RW, PB16~PB23 channel pin digital input disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 8,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "PinInDis",
            extends: None,
            description: Some(
                "RW, Function pin digital input disable register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pa_in_dis",
                    description: Some(
                        "RW, PA0~PA15 channel pin digital input disabled.",
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
                                len: 16,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
                Field {
                    name: "pbl_in_dis",
                    description: Some(
                        "RW, PB0~PB15 channel pin digital input disabled.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 16,
                                stride: 1,
                            },
                        ),
                    ),
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
                    name: "pwr_ram32k",
                    description: Some(
                        "RWA, SRAM power supply for RAM2K.",
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
                    name: "pwr_ram96k",
                    description: Some(
                        "RWA, SRAM power supply for RAM24K.",
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
                    name: "pwr_ldo_en",
                    description: Some(
                        "RWA, Internal LDO control.",
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
                        "RWA, DC/DC converter enable.",
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
                        "RWA, Must write 0010.",
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
            name: "SafeClkCtrl",
            extends: None,
            description: Some(
                "RWA, Secure access to clock control registers, SAM.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "clk_off_nfc",
                    description: Some(
                        "RWA, NFC clock off enable.",
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
                    name: "clk_off_adc",
                    description: Some(
                        "RWA, ADC clock off enable.",
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
                    name: "clk_off_led",
                    description: Some(
                        "RWA, LED clock off enable.",
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
            name: "SafeDebugCtrl",
            extends: None,
            description: Some(
                "RWA, Secure Access Debugging Control Register, SAM.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "debug_dis",
                    description: Some(
                        "RWA, DEBUG disable enable.",
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
            name: "SafeLrstCtrl",
            extends: None,
            description: Some(
                "RWA, Long term reset control register for secure access, SAM.",
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
                        "RWA, Long reset time selection.",
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
                    name: "iwdg_rst_en",
                    description: Some(
                        "RWA, Independent watchdog reset enable.",
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
            name: "SafeModeCtrl",
            extends: None,
            description: Some(
                "RWA, Secure Access Mode Control Register, SAM.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "safe_auto_en",
                    description: Some(
                        "RWA, Enable automatic shutdown of security register.",
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
                    name: "xrom_312m_sel",
                    description: Some(
                        "RWA, XROM Clock Selection.",
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
                        "RWA, Timer 0 clock source.",
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
                        "RWA, Timer 1 clock source.",
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
                        "RWA, Timer 2 clock source.",
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
                        "RWA, Timer 3 clock source.",
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
                        "RWA, UART0 clock source.",
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
                        "RWA, UART1 clock source.",
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
                        "RWA, UART2 clock source.",
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
                        "RWA, UART3 clock source.",
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
                    name: "slp_clk_spi1",
                    description: Some(
                        "RWA, close SPI1 clock.",
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
                    name: "slp_clk_usb2",
                    description: Some(
                        "RWA, close USB2 clock.",
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
                    name: "slp_clk_lcd",
                    description: Some(
                        "RWA, close LCD clock.",
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
                    name: "wake_dly_mod",
                    description: Some(
                        "RWA, wakeup delay time selection.",
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
                    name: "clk_off_utmi",
                    description: Some(
                        "RWA, Utmi_clk clock source.",
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
                    bit_size: 2,
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
                    name: "slp_usb2_wake",
                    description: Some(
                        "RWA, enable USB2 waking.",
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
                    name: "gpio_edge_wake",
                    description: Some(
                        "RWA, GPIO edge wake-up mode selection.",
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
                        "RWA, enable GPIO wake-up mode.",
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
                "RWA, External 32KHz oscillator tune control, SAM.",
            ),
            bit_size: 8,
            fields: &[
                Field {
                    name: "xt32k_i_tune",
                    description: Some(
                        "RWA, Selection of bias current for external 32KHz oscillator.",
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
                    name: "rc32k_i_tune",
                    description: Some(
                        "RWA, RC32K adjustment bit, internal 32KHz oscillator bias current selection.",
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
                    name: "xt32k_c_load",
                    description: Some(
                        "RWA, Choose a built-in load capacitor that matches the external 32KHz crystal.",
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
