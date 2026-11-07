#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "16 - I2C_EV"]
    I2C_EV = 16,
    #[doc = "17 - I2C_ER"]
    I2C_ER = 17,
    #[doc = "18 - ETH"]
    ETH = 18,
    #[doc = "19 - USBPD"]
    USBPD = 19,
    #[doc = "20 - TMR0"]
    TMR0 = 20,
    #[doc = "21 - TMR1"]
    TMR1 = 21,
    #[doc = "22 - TMR2"]
    TMR2 = 22,
    #[doc = "23 - TMR3"]
    TMR3 = 23,
    #[doc = "24 - SPI0"]
    SPI0 = 24,
    #[doc = "25 - SPI1"]
    SPI1 = 25,
    #[doc = "26 - UART0"]
    UART0 = 26,
    #[doc = "27 - UART1"]
    UART1 = 27,
    #[doc = "28 - PA"]
    PA = 28,
    #[doc = "29 - PB"]
    PB = 29,
    #[doc = "30 - PD"]
    PD = 30,
    #[doc = "31 - ADC"]
    ADC = 31,
    #[doc = "32 - SLV"]
    SLV = 32,
    #[doc = "33 - USBHS_HOST"]
    USBHS_HOST = 33,
    #[doc = "34 - USBHS_DEV"]
    USBHS_DEV = 34,
    #[doc = "35 - UART2"]
    UART2 = 35,
    #[doc = "36 - UART3"]
    UART3 = 36,
    #[doc = "37 - ETHWAKEUP"]
    ETHWAKEUP = 37,
    #[doc = "38 - USBWAKEUP"]
    USBWAKEUP = 38,
    #[doc = "39 - USBPDWAKEUP"]
    USBPDWAKEUP = 39,
}
unsafe impl crate::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    unsafe extern "C" {
        fn I2C_EV();
        fn I2C_ER();
        fn ETH();
        fn USBPD();
        fn TMR0();
        fn TMR1();
        fn TMR2();
        fn TMR3();
        fn SPI0();
        fn SPI1();
        fn UART0();
        fn UART1();
        fn PA();
        fn PB();
        fn PD();
        fn ADC();
        fn SLV();
        fn USBHS_HOST();
        fn USBHS_DEV();
        fn UART2();
        fn UART3();
        fn ETHWAKEUP();
        fn USBWAKEUP();
        fn USBPDWAKEUP();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.external_interrupts")]
    #[unsafe(no_mangle)]
    pub static __EXTERNAL_INTERRUPTS: [Vector; 24] = [
        Vector { _handler: I2C_EV },
        Vector { _handler: I2C_ER },
        Vector { _handler: ETH },
        Vector { _handler: USBPD },
        Vector { _handler: TMR0 },
        Vector { _handler: TMR1 },
        Vector { _handler: TMR2 },
        Vector { _handler: TMR3 },
        Vector { _handler: SPI0 },
        Vector { _handler: SPI1 },
        Vector { _handler: UART0 },
        Vector { _handler: UART1 },
        Vector { _handler: PA },
        Vector { _handler: PB },
        Vector { _handler: PD },
        Vector { _handler: ADC },
        Vector { _handler: SLV },
        Vector {
            _handler: USBHS_HOST,
        },
        Vector {
            _handler: USBHS_DEV,
        },
        Vector { _handler: UART2 },
        Vector { _handler: UART3 },
        Vector {
            _handler: ETHWAKEUP,
        },
        Vector {
            _handler: USBWAKEUP,
        },
        Vector {
            _handler: USBPDWAKEUP,
        },
    ];
}
pub const ETHERNET_MAC: ethernet_mac::EthernetMac =
    unsafe { ethernet_mac::EthernetMac::from_ptr(0x4002_8000usize as _) };
pub const ETHERNET_MMC: ethernet_mmc::EthernetMmc =
    unsafe { ethernet_mmc::EthernetMmc::from_ptr(0x4002_8100usize as _) };
pub const ETHERNET_PTP: ethernet_ptp::EthernetPtp =
    unsafe { ethernet_ptp::EthernetPtp::from_ptr(0x4002_8700usize as _) };
pub const ETHERNET_DMA: ethernet_dma::EthernetDma =
    unsafe { ethernet_dma::EthernetDma::from_ptr(0x4002_9000usize as _) };
pub const SYS: sys::Sys = unsafe { sys::Sys::from_ptr(0x4040_0000usize as _) };
pub const SYSTEM_CONTROL: system_control::SystemControl =
    unsafe { system_control::SystemControl::from_ptr(0x4040_0000usize as _) };
pub const GPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4040_0000usize as _) };
pub const SLV: slv::Slv = unsafe { slv::Slv::from_ptr(0x4040_0000usize as _) };
pub const USBHS_DEV: usbhs_dev::UsbhsDev =
    unsafe { usbhs_dev::UsbhsDev::from_ptr(0x4040_4000usize as _) };
pub const USBHS_HOST: usbhs_host::UsbhsHost =
    unsafe { usbhs_host::UsbhsHost::from_ptr(0x4040_4800usize as _) };
pub const USBHS_HSI: usbhs_hsi::UsbhsHsi =
    unsafe { usbhs_hsi::UsbhsHsi::from_ptr(0x4040_5000usize as _) };
pub const TMR0: tmr::TmrFull = unsafe { tmr::TmrFull::from_ptr(0x4040_8000usize as _) };
pub const TMR1: tmr::TmrFull = unsafe { tmr::TmrFull::from_ptr(0x4040_8400usize as _) };
pub const TMR2: tmr::TmrFull = unsafe { tmr::TmrFull::from_ptr(0x4040_8800usize as _) };
pub const TMR3: tmr::TmrNodma = unsafe { tmr::TmrNodma::from_ptr(0x4040_8c00usize as _) };
pub const UART2: uart::UartFull = unsafe { uart::UartFull::from_ptr(0x4040_9000usize as _) };
pub const UART3: uart::UartFull = unsafe { uart::UartFull::from_ptr(0x4040_9800usize as _) };
pub const ADC: adc::Adc = unsafe { adc::Adc::from_ptr(0x4040_a000usize as _) };
pub const SPI0: spi::SpiFull = unsafe { spi::SpiFull::from_ptr(0x4040_c000usize as _) };
pub const SPI1: spi::SpiNodma = unsafe { spi::SpiNodma::from_ptr(0x4040_c800usize as _) };
pub const UART0: uart::UartFull = unsafe { uart::UartFull::from_ptr(0x4040_d000usize as _) };
pub const UART1: uart::UartFull = unsafe { uart::UartFull::from_ptr(0x4040_d800usize as _) };
pub const USBPD: usbpd::Usbpd = unsafe { usbpd::Usbpd::from_ptr(0x4040_e000usize as _) };
pub const I2C0: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4040_f000usize as _) };
pub const PFIC: pfic::Pfic = unsafe { pfic::Pfic::from_ptr(0xe000_e000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_ch564.rs"]
pub mod adc;
#[path = "../../peripherals/ethernet_dma_ch564.rs"]
pub mod ethernet_dma;
#[path = "../../peripherals/ethernet_mac_ch564.rs"]
pub mod ethernet_mac;
#[path = "../../peripherals/ethernet_mmc_ch564.rs"]
pub mod ethernet_mmc;
#[path = "../../peripherals/ethernet_ptp_ch564.rs"]
pub mod ethernet_ptp;
#[path = "../../peripherals/gpio_ch564.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_ch564.rs"]
pub mod i2c;
#[path = "../../peripherals/pfic_ch564.rs"]
pub mod pfic;
#[path = "../../peripherals/slv_ch564.rs"]
pub mod slv;
#[path = "../../peripherals/spi_ch564.rs"]
pub mod spi;
#[path = "../../peripherals/sys_ch564.rs"]
pub mod sys;
#[path = "../../peripherals/system_control_ch564.rs"]
pub mod system_control;
#[path = "../../peripherals/tmr_ch564.rs"]
pub mod tmr;
#[path = "../../peripherals/uart_ch564.rs"]
pub mod uart;
#[path = "../../peripherals/usbhs_dev_ch564.rs"]
pub mod usbhs_dev;
#[path = "../../peripherals/usbhs_host_ch564.rs"]
pub mod usbhs_host;
#[path = "../../peripherals/usbhs_hsi_ch564.rs"]
pub mod usbhs_hsi;
#[path = "../../peripherals/usbpd_ch564.rs"]
pub mod usbpd;
pub const CORE_INDEX: usize = 0;
#[deprecated(note = "use ch5xx_metapac::MEMORY_LAYOUT instead")]
pub const FLASH_BASE: usize = 0;
#[deprecated(note = "use ch5xx_metapac::MEMORY_LAYOUT instead")]
pub const FLASH_SIZE: usize = 229376;
#[deprecated(note = "use ch5xx_metapac::MEMORY_LAYOUT instead")]
pub const WRITE_SIZE: usize = 4;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout {
    regions: crate::memory_select::MEMORY,
};
