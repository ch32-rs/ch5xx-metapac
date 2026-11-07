
pub(crate) static PERIPHERALS: &[Peripheral] = &[
    Peripheral {
        name: "SYS",
        address: 0x40400000,
        registers: Some(PeripheralRegisters {
            kind: "sys",
            version: "ch564",
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
        address: 0x40400000,
        registers: Some(PeripheralRegisters {
            kind: "system_control",
            version: "ch564",
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
        address: 0x40400000,
        registers: Some(PeripheralRegisters {
            kind: "gpio",
            version: "ch564",
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
        name: "SLV",
        address: 0x40400000,
        registers: Some(PeripheralRegisters {
            kind: "slv",
            version: "ch564",
            block: "SLV",
            ir: &slv::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USBHS_DEV",
        address: 0x40404000,
        registers: Some(PeripheralRegisters {
            kind: "usbhs_dev",
            version: "ch564",
            block: "USBHS_DEV",
            ir: &usbhs_dev::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USBHS_HOST",
        address: 0x40404800,
        registers: Some(PeripheralRegisters {
            kind: "usbhs_host",
            version: "ch564",
            block: "USBHS_HOST",
            ir: &usbhs_host::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "USBHS_HSI",
        address: 0x40405000,
        registers: Some(PeripheralRegisters {
            kind: "usbhs_hsi",
            version: "ch564",
            block: "USBHS_HSI",
            ir: &usbhs_hsi::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "TMR0",
        address: 0x40408000,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "ch564",
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
        name: "TMR1",
        address: 0x40408400,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "ch564",
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
        address: 0x40408800,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "ch564",
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
        address: 0x40408c00,
        registers: Some(PeripheralRegisters {
            kind: "tmr",
            version: "ch564",
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
        name: "UART2",
        address: 0x40409000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "ch564",
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
        name: "UART3",
        address: 0x40409800,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "ch564",
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
        name: "ADC",
        address: 0x4040a000,
        registers: Some(PeripheralRegisters {
            kind: "adc",
            version: "ch564",
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
        name: "SPI0",
        address: 0x4040c000,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "ch564",
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
        address: 0x4040c800,
        registers: Some(PeripheralRegisters {
            kind: "spi",
            version: "ch564",
            block: "SPI_NODMA",
            ir: &spi::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "UART0",
        address: 0x4040d000,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "ch564",
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
        address: 0x4040d800,
        registers: Some(PeripheralRegisters {
            kind: "uart",
            version: "ch564",
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
        name: "USBPD",
        address: 0x4040e000,
        registers: Some(PeripheralRegisters {
            kind: "usbpd",
            version: "ch564",
            block: "USBPD",
            ir: &usbpd::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "I2C0",
        address: 0x4040f000,
        registers: Some(PeripheralRegisters {
            kind: "i2c",
            version: "ch564",
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
        name: "ETHERNET_MAC",
        address: 0x40028000,
        registers: Some(PeripheralRegisters {
            kind: "ethernet_mac",
            version: "ch564",
            block: "ETHERNET_MAC",
            ir: &ethernet_mac::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ETHERNET_MMC",
        address: 0x40028100,
        registers: Some(PeripheralRegisters {
            kind: "ethernet_mmc",
            version: "ch564",
            block: "ETHERNET_MMC",
            ir: &ethernet_mmc::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ETHERNET_PTP",
        address: 0x40028700,
        registers: Some(PeripheralRegisters {
            kind: "ethernet_ptp",
            version: "ch564",
            block: "ETHERNET_PTP",
            ir: &ethernet_ptp::REGISTERS,
        }),
        rcc: None,
        remap: None,
        pins: &[],
        dma_channels: &[],
        interrupts: &[],
    },
    Peripheral {
        name: "ETHERNET_DMA",
        address: 0x40029000,
        registers: Some(PeripheralRegisters {
            kind: "ethernet_dma",
            version: "ch564",
            block: "ETHERNET_DMA",
            ir: &ethernet_dma::REGISTERS,
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
            version: "ch564",
            block: "PFIC",
            ir: &pfic::REGISTERS,
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
        name: "I2C_EV",
        number: 16,
    },
    Interrupt {
        name: "I2C_ER",
        number: 17,
    },
    Interrupt {
        name: "ETH",
        number: 18,
    },
    Interrupt {
        name: "USBPD",
        number: 19,
    },
    Interrupt {
        name: "TMR0",
        number: 20,
    },
    Interrupt {
        name: "TMR1",
        number: 21,
    },
    Interrupt {
        name: "TMR2",
        number: 22,
    },
    Interrupt {
        name: "TMR3",
        number: 23,
    },
    Interrupt {
        name: "SPI0",
        number: 24,
    },
    Interrupt {
        name: "SPI1",
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
        name: "PA",
        number: 28,
    },
    Interrupt {
        name: "PB",
        number: 29,
    },
    Interrupt {
        name: "PD",
        number: 30,
    },
    Interrupt {
        name: "ADC",
        number: 31,
    },
    Interrupt {
        name: "SLV",
        number: 32,
    },
    Interrupt {
        name: "USBHS_HOST",
        number: 33,
    },
    Interrupt {
        name: "USBHS_DEV",
        number: 34,
    },
    Interrupt {
        name: "UART2",
        number: 35,
    },
    Interrupt {
        name: "UART3",
        number: 36,
    },
    Interrupt {
        name: "ETHWAKEUP",
        number: 37,
    },
    Interrupt {
        name: "USBWAKEUP",
        number: 38,
    },
    Interrupt {
        name: "USBPDWAKEUP",
        number: 39,
    },
];
pub(crate) static DMA_CHANNELS: &[DmaChannel] = &[];
#[path = "../registers/adc_ch564.rs"]
pub mod adc;
#[path = "../registers/ethernet_dma_ch564.rs"]
pub mod ethernet_dma;
#[path = "../registers/ethernet_mac_ch564.rs"]
pub mod ethernet_mac;
#[path = "../registers/ethernet_mmc_ch564.rs"]
pub mod ethernet_mmc;
#[path = "../registers/ethernet_ptp_ch564.rs"]
pub mod ethernet_ptp;
#[path = "../registers/gpio_ch564.rs"]
pub mod gpio;
#[path = "../registers/i2c_ch564.rs"]
pub mod i2c;
#[path = "../registers/pfic_ch564.rs"]
pub mod pfic;
#[path = "../registers/slv_ch564.rs"]
pub mod slv;
#[path = "../registers/spi_ch564.rs"]
pub mod spi;
#[path = "../registers/sys_ch564.rs"]
pub mod sys;
#[path = "../registers/system_control_ch564.rs"]
pub mod system_control;
#[path = "../registers/tmr_ch564.rs"]
pub mod tmr;
#[path = "../registers/uart_ch564.rs"]
pub mod uart;
#[path = "../registers/usbhs_dev_ch564.rs"]
pub mod usbhs_dev;
#[path = "../registers/usbhs_host_ch564.rs"]
pub mod usbhs_host;
#[path = "../registers/usbhs_hsi_ch564.rs"]
pub mod usbhs_hsi;
#[path = "../registers/usbpd_ch564.rs"]
pub mod usbpd;
