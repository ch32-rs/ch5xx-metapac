
                pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "SYS",
        address: 0x40001000,
        registers: Some(
            PeripheralRegisters {
                kind: "sys",
                version: "ch572",
                block: "SYS",
                ir: &sys::REGISTERS,
            },
        ),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "RTC",
        address: 0x40001000,
        registers: Some(
            PeripheralRegisters {
                kind: "rtc",
                version: "ch572",
                block: "RTC",
                ir: &rtc::REGISTERS,
            },
        ),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "System_control",
        address: 0x40001000,
        registers: Some(
            PeripheralRegisters {
                kind: "system_control",
                version: "ch572",
                block: "System_control",
                ir: &system_control::REGISTERS,
            },
        ),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "CMP",
        address: 0x40001000,
        registers: Some(
            PeripheralRegisters {
                kind: "cmp",
                version: "ch572",
                block: "CMP",
                ir: &cmp::REGISTERS,
            },
        ),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "KEYSCAN",
        address: 0x40001000,
        registers: Some(
            PeripheralRegisters {
                kind: "keyscan",
                version: "ch572",
                block: "KEYSCAN",
                ir: &keyscan::REGISTERS,
            },
        ),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIO",
        address: 0x40001000,
        registers: Some(
            PeripheralRegisters {
                kind: "gpio",
                version: "ch572",
                block: "GPIO",
                ir: &gpio::REGISTERS,
            },
        ),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR0",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "tmr",
                version: "ch572",
                block: "TMR_FULL",
                ir: &tmr::REGISTERS,
            },
        ),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ENCODER",
        address: 0x40002400,
        registers: Some(
            PeripheralRegisters {
                kind: "encoder",
                version: "ch572",
                block: "ENCODER",
                ir: &encoder::REGISTERS,
            },
        ),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "UART0",
        address: 0x40003400,
        registers: Some(
            PeripheralRegisters {
                kind: "uart",
                version: "ch572",
                block: "UART_FULL",
                ir: &uart::REGISTERS,
            },
        ),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI0",
        address: 0x40004000,
        registers: Some(
            PeripheralRegisters {
                kind: "spi",
                version: "ch572",
                block: "SPI_FULL",
                ir: &spi::REGISTERS,
            },
        ),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C0",
        address: 0x40004800,
        registers: Some(
            PeripheralRegisters {
                kind: "i2c",
                version: "ch585",
                block: "I2C",
                ir: &i2c::REGISTERS,
            },
        ),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PWMx",
        address: 0x40005000,
        registers: Some(
            PeripheralRegisters {
                kind: "pwmx",
                version: "ch572",
                block: "PWMx",
                ir: &pwmx::REGISTERS,
            },
        ),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USBFS",
        address: 0x40008000,
        registers: Some(
            PeripheralRegisters {
                kind: "usbfs",
                version: "ch585",
                block: "USBFS",
                ir: &usbfs::REGISTERS,
            },
        ),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "BLE",
        address: 0x4000c000,
        registers: None,
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "PFIC",
        address: 0xe000e000,
        registers: Some(
            PeripheralRegisters {
                kind: "pfic",
                version: "ch585",
                block: "PFIC",
                ir: &pfic::REGISTERS,
            },
        ),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "Systick",
        address: 0xe000f000,
        registers: Some(
            PeripheralRegisters {
                kind: "systick",
                version: "ch572",
                block: "Systick",
                ir: &systick::REGISTERS,
            },
        ),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
];
                pub(crate) static INTERRUPTS: &[Interrupt] = &[
    Interrupt {
        name: "RESERVED16",
        number: 16,
    },
    Interrupt {
        name: "GPIO_A",
        number: 17,
    },
    Interrupt {
        name: "SPI0",
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
        name: "TMR0",
        number: 24,
    },
    Interrupt {
        name: "UART0",
        number: 27,
    },
    Interrupt {
        name: "RTC",
        number: 28,
    },
    Interrupt {
        name: "CMP",
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
        name: "KEYSCAN",
        number: 33,
    },
    Interrupt {
        name: "ENCODER",
        number: 34,
    },
    Interrupt {
        name: "WDOG_BAT",
        number: 35,
    },
];
                pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
            #[path="../registers/cmp_ch572.rs"] pub mod cmp;
#[path="../registers/encoder_ch572.rs"] pub mod encoder;
#[path="../registers/gpio_ch572.rs"] pub mod gpio;
#[path="../registers/i2c_ch585.rs"] pub mod i2c;
#[path="../registers/keyscan_ch572.rs"] pub mod keyscan;
#[path="../registers/pfic_ch585.rs"] pub mod pfic;
#[path="../registers/pwmx_ch572.rs"] pub mod pwmx;
#[path="../registers/rtc_ch572.rs"] pub mod rtc;
#[path="../registers/spi_ch572.rs"] pub mod spi;
#[path="../registers/sys_ch572.rs"] pub mod sys;
#[path="../registers/system_control_ch572.rs"] pub mod system_control;
#[path="../registers/systick_ch572.rs"] pub mod systick;
#[path="../registers/tmr_ch572.rs"] pub mod tmr;
#[path="../registers/uart_ch572.rs"] pub mod uart;
#[path="../registers/usbfs_ch585.rs"] pub mod usbfs;
