
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "SYS",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "sys",
            version: "ch569",
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
        name: "System_control",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "system_control",
            version: "ch569",
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
        name: "GPIO",
        address: 0x40001000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ch569",
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
            version: "ch569",
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
            version: "ch569",
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
            version: "ch569",
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
        name: "UART0",
        address: 0x40003000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "ch569",
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
            version: "ch569",
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
            version: "ch569",
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
            version: "ch569",
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
            version: "ch569",
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
        name: "SPI1",
        address: 0x40004400,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "ch569",
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
        name: "PWMx",
        address: 0x40005000,
        registers: Some(PeripheralRegisters {
            kind: "pwmx",
            version: "ch569",
            block: "PWMx",
            ir: &pwmx::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "HSPI",
        address: 0x40006000,
        registers: Some(PeripheralRegisters {
            kind: "hspi",
            version: "ch569",
            block: "HSPI",
            ir: &hspi::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ECDC",
        address: 0x40007000,
        registers: Some(PeripheralRegisters {
            kind: "ecdc",
            version: "ch569",
            block: "ECDC",
            ir: &ecdc::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USBSS",
        address: 0x40008000,
        registers: None,
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USBHS",
        address: 0x40009000,
        registers: Some(PeripheralRegisters {
            kind: "usbhs",
            version: "ch569",
            block: "USBHS",
            ir: &usbhs::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "EMMC",
        address: 0x4000a000,
        registers: Some(PeripheralRegisters {
            kind: "emmc",
            version: "ch569",
            block: "EMMC",
            ir: &emmc::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "SERDES",
        address: 0x4000b000,
        registers: None,
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ETH",
        address: 0x4000c000,
        registers: None,
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "DVP",
        address: 0x4000e000,
        registers: Some(PeripheralRegisters {
            kind: "dvp",
            version: "ch569",
            block: "DVP",
            ir: &dvp::REGISTERS,
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
            version: "ch569",
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
            version: "ch569",
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
        name: "WDOG",
        number: 16,
    },
    Interrupt {
        name: "TMR0",
        number: 17,
    },
    Interrupt {
        name: "GPIO",
        number: 18,
    },
    Interrupt {
        name: "SPI0",
        number: 19,
    },
    Interrupt {
        name: "USBSS",
        number: 20,
    },
    Interrupt {
        name: "USBSS_LINK",
        number: 21,
    },
    Interrupt {
        name: "TMR1",
        number: 22,
    },
    Interrupt {
        name: "TMR2",
        number: 23,
    },
    Interrupt {
        name: "UART0",
        number: 24,
    },
    Interrupt {
        name: "USBHS",
        number: 25,
    },
    Interrupt {
        name: "EMMC",
        number: 26,
    },
    Interrupt {
        name: "DVP",
        number: 27,
    },
    Interrupt {
        name: "HSPI",
        number: 28,
    },
    Interrupt {
        name: "SPI1",
        number: 29,
    },
    Interrupt {
        name: "UART1",
        number: 30,
    },
    Interrupt {
        name: "UART2",
        number: 31,
    },
    Interrupt {
        name: "UART3",
        number: 32,
    },
    Interrupt {
        name: "SERDES",
        number: 33,
    },
    Interrupt {
        name: "ETH",
        number: 34,
    },
    Interrupt {
        name: "PMT",
        number: 35,
    },
    Interrupt {
        name: "ECDC",
        number: 36,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
#[path = "../registers/dvp_ch569.rs"]
pub mod dvp;
#[path = "../registers/ecdc_ch569.rs"]
pub mod ecdc;
#[path = "../registers/emmc_ch569.rs"]
pub mod emmc;
#[path = "../registers/gpio_ch569.rs"]
pub mod gpio;
#[path = "../registers/hspi_ch569.rs"]
pub mod hspi;
#[path = "../registers/pfic_ch569.rs"]
pub mod pfic;
#[path = "../registers/pwmx_ch569.rs"]
pub mod pwmx;
#[path = "../registers/spi_ch569.rs"]
pub mod spi;
#[path = "../registers/sys_ch569.rs"]
pub mod sys;
#[path = "../registers/system_control_ch569.rs"]
pub mod system_control;
#[path = "../registers/systick_ch569.rs"]
pub mod systick;
#[path = "../registers/tmr_ch569.rs"]
pub mod tmr;
#[path = "../registers/uart_ch569.rs"]
pub mod uart;
#[path = "../registers/usbhs_ch569.rs"]
pub mod usbhs;
