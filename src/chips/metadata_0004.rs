
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "SYS",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "sys",
            version: "ch581",
            block: "SYS",
            ir: &sys::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "BAT",
            interrupt: "WDOG_BAT",
        }],
    },
    Peripheral {
        name: "RTC",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "rtc",
            version: "ch585",
            block: "RTC",
            ir: &rtc::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "RTC",
        }],
    },
    Peripheral {
        name: "System_control",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "system_control",
            version: "ch585",
            block: "System_control",
            ir: &system_control::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TOUCHKEY",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "touchkey",
            version: "ch585",
            block: "TOUCH_KEY",
            ir: &touchkey::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "A0",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "A1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "A2",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "A3",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "A4",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "A5",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "A6",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "A7",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "A8",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "A9",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "A10",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "A11",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "A12",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "A13",
                remap: None,
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC",
        }],
    },
    Peripheral {
        name: "ADC",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "ch585",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "A0",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "A1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA12",
                signal: "A2",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "A3",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "A4",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "A5",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "A6",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "A7",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "A8",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA0",
                signal: "A9",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "A10",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "A11",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "A12",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "A13",
                remap: None,
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "ADC",
        }],
    },
    Peripheral {
        name: "GPIO",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ch585",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "PORT_A",
                interrupt: "GPIO_A",
            },
            PeripheralInterrupt {
                signal: "PORT_B",
                interrupt: "GPIO_B",
            },
        ],
    },
    Peripheral {
        name: "FLASHROM",
        address: 0x40001800,
        registers: None,
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR0",
        address: 0x40002000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "ch585",
            block: "TMR_FULL",
            ir: &tmr::REGISTERS,
        }),
        rcc: None,
        remap: Some(PeripheralRemapRegister {
            register: "PIN_ALTERNATE",
            field: "PIN_TMR0",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA9",
                signal: "TMR0",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB23",
                signal: "TMR0",
                remap: Some(1),
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TMR0",
        }],
    },
    Peripheral {
        name: "TMR1",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "ch585",
            block: "TMR_FULL",
            ir: &tmr::REGISTERS,
        }),
        rcc: None,
        remap: Some(PeripheralRemapRegister {
            register: "PIN_ALTERNATE",
            field: "PIN_TMR1",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA10",
                signal: "TMR1",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "TMR1",
                remap: Some(1),
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TMR1",
        }],
    },
    Peripheral {
        name: "TMR2",
        address: 0x40002800,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "ch585",
            block: "TMR_FULL",
            ir: &tmr::REGISTERS,
        }),
        rcc: None,
        remap: Some(PeripheralRemapRegister {
            register: "PIN_ALTERNATE",
            field: "PIN_TMR2",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA11",
                signal: "TMR2",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB11",
                signal: "TMR2",
                remap: Some(1),
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TMR2",
        }],
    },
    Peripheral {
        name: "TMR3",
        address: 0x40002c00,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "ch585",
            block: "TMR_FULL",
            ir: &tmr::REGISTERS,
        }),
        rcc: None,
        remap: Some(PeripheralRemapRegister {
            register: "PIN_ALTERNATE",
            field: "PIN_TMR3",
        }),
        pins: &[
            PeripheralPin {
                pin: "PB22",
                signal: "TMR3",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "TMR3",
                remap: Some(1),
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "TMR3",
        }],
    },
    Peripheral {
        name: "UART0",
        address: 0x40003000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "ch585",
            block: "UART_FULL",
            ir: &uart::REGISTERS,
        }),
        rcc: None,
        remap: Some(PeripheralRemapRegister {
            register: "PIN_ALTERNATE",
            field: "PIN_UART0",
        }),
        pins: &[
            PeripheralPin {
                pin: "PB4",
                signal: "RXD",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "RXD",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "TXD",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "TXD",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "CTS",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "RTS",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB5",
                signal: "DTR",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "DTR",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "DSR",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "DSR",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "DCD",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "RI",
                remap: None,
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART0",
        }],
    },
    Peripheral {
        name: "UART1",
        address: 0x40003400,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "ch585",
            block: "UART_FULL",
            ir: &uart::REGISTERS,
        }),
        rcc: None,
        remap: Some(PeripheralRemapRegister {
            register: "PIN_ALTERNATE",
            field: "PIN_UART1",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA8",
                signal: "RXD",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "RXD",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PA9",
                signal: "TXD",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "TXD",
                remap: Some(1),
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART1",
        }],
    },
    Peripheral {
        name: "UART2",
        address: 0x40003800,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "ch585",
            block: "UART_FULL",
            ir: &uart::REGISTERS,
        }),
        rcc: None,
        remap: Some(PeripheralRemapRegister {
            register: "PIN_ALTERNATE",
            field: "PIN_UART2",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA6",
                signal: "RXD",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB22",
                signal: "RXD",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "TXD",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB23",
                signal: "TXD",
                remap: Some(1),
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART2",
        }],
    },
    Peripheral {
        name: "UART3",
        address: 0x40003c00,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "ch585",
            block: "UART_FULL",
            ir: &uart::REGISTERS,
        }),
        rcc: None,
        remap: Some(PeripheralRemapRegister {
            register: "PIN_ALTERNATE",
            field: "PIN_UART3",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA4",
                signal: "RXD",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB20",
                signal: "RXD",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "TXD",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB21",
                signal: "TXD",
                remap: Some(1),
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "UART3",
        }],
    },
    Peripheral {
        name: "SPI0",
        address: 0x40004000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "ch585",
            block: "SPI_FULL",
            ir: &spi::REGISTERS,
        }),
        rcc: None,
        remap: Some(PeripheralRemapRegister {
            register: "PIN_ALTERNATE",
            field: "PIN_SPI0",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "SCS",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SCS",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "SCK",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB13",
                signal: "SCK",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PA14",
                signal: "MOSI",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "MOSI",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PA15",
                signal: "MISO",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB15",
                signal: "MISO",
                remap: Some(1),
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "SPI",
        }],
    },
    Peripheral {
        name: "I2C0",
        address: 0x40004800,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "ch585",
            block: "I2C",
            ir: &i2c::REGISTERS,
        }),
        rcc: None,
        remap: Some(PeripheralRemapRegister {
            register: "PIN_ALTERNATE",
            field: "PIN_I2C",
        }),
        pins: &[
            PeripheralPin {
                pin: "PB13",
                signal: "SCL",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB21",
                signal: "SCL",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB12",
                signal: "SDA",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB20",
                signal: "SDA",
                remap: Some(1),
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "I2C0",
        }],
    },
    Peripheral {
        name: "PWMX",
        address: 0x40005000,
        registers: Some(PeripheralRegisters {
            kind: "pwmx",
            version: "ch585",
            block: "PWMX",
            ir: &pwmx::REGISTERS,
        }),
        rcc: None,
        remap: Some(PeripheralRemapRegister {
            register: "PIN_ALTERNATE",
            field: "PIN_PWMX",
        }),
        pins: &[
            PeripheralPin {
                pin: "PA12",
                signal: "PWM4",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "PWM4",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PA13",
                signal: "PWM5",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "PWM5",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB0",
                signal: "PWM6",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB4",
                signal: "PWM7",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB1",
                signal: "PWM7",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB6",
                signal: "PWM8",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB2",
                signal: "PWM8",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB7",
                signal: "PWM9",
                remap: Some(0),
                af: None,
            },
            PeripheralPin {
                pin: "PB3",
                signal: "PWM9",
                remap: Some(1),
                af: None,
            },
            PeripheralPin {
                pin: "PB14",
                signal: "PWM10",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB23",
                signal: "PWM11",
                remap: None,
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "PWMX",
        }],
    },
    Peripheral {
        name: "USBFS",
        address: 0x40008000,
        registers: Some(PeripheralRegisters {
            kind: "usbfs",
            version: "ch585",
            block: "USBFS",
            ir: &usbfs::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PB11",
                signal: "DP",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PB10",
                signal: "DM",
                remap: None,
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "USB",
        }],
    },
    Peripheral {
        name: "BLE",
        address: 0x4000c000,
        registers: None,
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[
            PeripheralInterrupt {
                signal: "BB",
                interrupt: "BLEB",
            },
            PeripheralInterrupt {
                signal: "LLE",
                interrupt: "BLEL",
            },
        ],
    },
    Peripheral {
        name: "LED",
        address: 0x4000f000,
        registers: Some(PeripheralRegisters {
            kind: "led",
            version: "ch585",
            block: "LED",
            ir: &led::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[
            PeripheralPin {
                pin: "PA0",
                signal: "LED0",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA1",
                signal: "LED1",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA2",
                signal: "LED2",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA3",
                signal: "LED3",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA4",
                signal: "LEDC",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA5",
                signal: "LED4",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA6",
                signal: "LED5",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA7",
                signal: "LED6",
                remap: None,
                af: None,
            },
            PeripheralPin {
                pin: "PA8",
                signal: "LED7",
                remap: None,
                af: None,
            },
        ],
        dma_channels: &[],
        interrupts: &[PeripheralInterrupt {
            signal: "GLOBAL",
            interrupt: "LED",
        }],
    },
    Peripheral {
        name: "PFIC",
        address: 0xe000e000,
        registers: Some(PeripheralRegisters {
            kind: "pfic",
            version: "ch585",
            block: "PFIC",
            ir: &pfic::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "Systick",
        address: 0xe000f000,
        registers: Some(PeripheralRegisters {
            kind: "systick",
            version: "ch585",
            block: "Systick",
            ir: &systick::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
];
pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "TMR0",
        number: 16,
    },
    Interrupt {
        name: "GPIO_A",
        number: 17,
    },
    Interrupt {
        name: "GPIO_B",
        number: 18,
    },
    Interrupt {
        name: "SPI",
        number: 19,
    },
    Interrupt {
        name: "BLEB",
        number: 20,
    },
    Interrupt {
        name: "BLEL",
        number: 21,
    },
    Interrupt {
        name: "USB",
        number: 22,
    },
    Interrupt {
        name: "TMR1",
        number: 24,
    },
    Interrupt {
        name: "TMR2",
        number: 25,
    },
    Interrupt {
        name: "UART0",
        number: 26,
    },
    Interrupt {
        name: "UART1",
        number: 27,
    },
    Interrupt {
        name: "RTC",
        number: 28,
    },
    Interrupt {
        name: "ADC",
        number: 29,
    },
    Interrupt {
        name: "I2C0",
        number: 30,
    },
    Interrupt {
        name: "PWMX",
        number: 31,
    },
    Interrupt {
        name: "TMR3",
        number: 32,
    },
    Interrupt {
        name: "UART2",
        number: 33,
    },
    Interrupt {
        name: "UART3",
        number: 34,
    },
    Interrupt {
        name: "WDOG_BAT",
        number: 35,
    },
    Interrupt {
        name: "LED",
        number: 39,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
#[path = "../registers/adc_ch585.rs"]
pub mod adc;
#[path = "../registers/gpio_ch585.rs"]
pub mod gpio;
#[path = "../registers/i2c_ch585.rs"]
pub mod i2c;
#[path = "../registers/led_ch585.rs"]
pub mod led;
#[path = "../registers/pfic_ch585.rs"]
pub mod pfic;
#[path = "../registers/pwmx_ch585.rs"]
pub mod pwmx;
#[path = "../registers/rtc_ch585.rs"]
pub mod rtc;
#[path = "../registers/spi_ch585.rs"]
pub mod spi;
#[path = "../registers/sys_ch581.rs"]
pub mod sys;
#[path = "../registers/system_control_ch585.rs"]
pub mod system_control;
#[path = "../registers/systick_ch585.rs"]
pub mod systick;
#[path = "../registers/tmr_ch585.rs"]
pub mod tmr;
#[path = "../registers/touchkey_ch585.rs"]
pub mod touchkey;
#[path = "../registers/uart_ch585.rs"]
pub mod uart;
#[path = "../registers/usbfs_ch585.rs"]
pub mod usbfs;
