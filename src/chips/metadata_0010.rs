
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "SYS",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "sys",
            version: "ch595",
            block: "SYS",
            ir: &sys::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
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
        interrupts: &[],
    },
    Peripheral {
        name: "System_control",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "system_control",
            version: "ch595",
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
            version: "ch595",
            block: "TOUCH_KEY",
            ir: &touchkey::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ADC",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "ch595",
            block: "ADC",
            ir: &adc::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "GPIO",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ch595",
            block: "GPIO",
            ir: &gpio::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
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
            version: "ch595",
            block: "TMR_NODMA",
            ir: &tmr::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR1",
        address: 0x40002400,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "ch595",
            block: "TMR_FULL",
            ir: &tmr::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR2",
        address: 0x40002800,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "ch595",
            block: "TMR_FULL",
            ir: &tmr::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR3",
        address: 0x40002c00,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "ch595",
            block: "TMR_NODMA",
            ir: &tmr::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
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
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "UART1",
        address: 0x40003400,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "ch585",
            block: "UART_NOMODEM",
            ir: &uart::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "UART2",
        address: 0x40003800,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "ch585",
            block: "UART_NOMODEM",
            ir: &uart::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "UART3",
        address: 0x40003c00,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "ch585",
            block: "UART_NOMODEM",
            ir: &uart::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SPI0",
        address: 0x40004000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "ch595",
            block: "SPI_FULL",
            ir: &spi::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
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
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
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
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "LCD",
        address: 0x40006000,
        registers: Some(PeripheralRegisters {
            kind: "lcd",
            version: "ch595",
            block: "LCD",
            ir: &lcd::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "KEYSCAN",
        address: 0x40006040,
        registers: Some(PeripheralRegisters {
            kind: "keyscan",
            version: "ch595",
            block: "KEYSCAN",
            ir: &keyscan::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ENCODER",
        address: 0x40006080,
        registers: Some(PeripheralRegisters {
            kind: "encoder",
            version: "ch595",
            block: "ENCODER",
            ir: &encoder::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
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
        name: "LED",
        address: 0x4000f000,
        registers: Some(PeripheralRegisters {
            kind: "led",
            version: "ch595",
            block: "LED",
            ir: &led::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
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
        name: "PWMX_SPI1",
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
        name: "ENCODER",
        number: 36,
    },
    Interrupt {
        name: "KEYSCAN",
        number: 37,
    },
    Interrupt {
        name: "LED",
        number: 39,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
#[path = "../registers/adc_ch595.rs"]
pub mod adc;
#[path = "../registers/encoder_ch595.rs"]
pub mod encoder;
#[path = "../registers/gpio_ch595.rs"]
pub mod gpio;
#[path = "../registers/i2c_ch585.rs"]
pub mod i2c;
#[path = "../registers/keyscan_ch595.rs"]
pub mod keyscan;
#[path = "../registers/lcd_ch595.rs"]
pub mod lcd;
#[path = "../registers/led_ch595.rs"]
pub mod led;
#[path = "../registers/pfic_ch585.rs"]
pub mod pfic;
#[path = "../registers/pwmx_ch585.rs"]
pub mod pwmx;
#[path = "../registers/rtc_ch585.rs"]
pub mod rtc;
#[path = "../registers/spi_ch595.rs"]
pub mod spi;
#[path = "../registers/sys_ch595.rs"]
pub mod sys;
#[path = "../registers/system_control_ch595.rs"]
pub mod system_control;
#[path = "../registers/systick_ch585.rs"]
pub mod systick;
#[path = "../registers/tmr_ch595.rs"]
pub mod tmr;
#[path = "../registers/touchkey_ch595.rs"]
pub mod touchkey;
#[path = "../registers/uart_ch585.rs"]
pub mod uart;
#[path = "../registers/usbfs_ch585.rs"]
pub mod usbfs;
