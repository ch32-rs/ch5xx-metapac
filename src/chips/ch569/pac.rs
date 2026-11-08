#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "16 - WDOG"]
    WDOG = 16,
    #[doc = "17 - TMR0"]
    TMR0 = 17,
    #[doc = "18 - GPIO"]
    GPIO = 18,
    #[doc = "19 - SPI0"]
    SPI0 = 19,
    #[doc = "20 - USBSS"]
    USBSS = 20,
    #[doc = "21 - USBSS_LINK"]
    USBSS_LINK = 21,
    #[doc = "22 - TMR1"]
    TMR1 = 22,
    #[doc = "23 - TMR2"]
    TMR2 = 23,
    #[doc = "24 - UART0"]
    UART0 = 24,
    #[doc = "25 - USBHS"]
    USBHS = 25,
    #[doc = "26 - EMMC"]
    EMMC = 26,
    #[doc = "27 - DVP"]
    DVP = 27,
    #[doc = "28 - HSPI"]
    HSPI = 28,
    #[doc = "29 - SPI1"]
    SPI1 = 29,
    #[doc = "30 - UART1"]
    UART1 = 30,
    #[doc = "31 - UART2"]
    UART2 = 31,
    #[doc = "32 - UART3"]
    UART3 = 32,
    #[doc = "33 - SERDES"]
    SERDES = 33,
    #[doc = "34 - ETH"]
    ETH = 34,
    #[doc = "35 - PMT"]
    PMT = 35,
    #[doc = "36 - ECDC"]
    ECDC = 36,
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
        fn WDOG();
        fn TMR0();
        fn GPIO();
        fn SPI0();
        fn USBSS();
        fn USBSS_LINK();
        fn TMR1();
        fn TMR2();
        fn UART0();
        fn USBHS();
        fn EMMC();
        fn DVP();
        fn HSPI();
        fn SPI1();
        fn UART1();
        fn UART2();
        fn UART3();
        fn SERDES();
        fn ETH();
        fn PMT();
        fn ECDC();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.external_interrupts")]
    #[unsafe(no_mangle)]
    pub static __EXTERNAL_INTERRUPTS: [Vector; 21] = [
        Vector { _handler: WDOG },
        Vector { _handler: TMR0 },
        Vector { _handler: GPIO },
        Vector { _handler: SPI0 },
        Vector { _handler: USBSS },
        Vector {
            _handler: USBSS_LINK,
        },
        Vector { _handler: TMR1 },
        Vector { _handler: TMR2 },
        Vector { _handler: UART0 },
        Vector { _handler: USBHS },
        Vector { _handler: EMMC },
        Vector { _handler: DVP },
        Vector { _handler: HSPI },
        Vector { _handler: SPI1 },
        Vector { _handler: UART1 },
        Vector { _handler: UART2 },
        Vector { _handler: UART3 },
        Vector { _handler: SERDES },
        Vector { _handler: ETH },
        Vector { _handler: PMT },
        Vector { _handler: ECDC },
    ];
}
pub const SYS: sys::Sys = unsafe { sys::Sys::from_ptr(0x4000_1000usize as _) };
pub const SYSTEM_CONTROL: system_control::SystemControl =
    unsafe { system_control::SystemControl::from_ptr(0x4000_1000usize as _) };
pub const GPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4000_1000usize as _) };
pub const FLASHROM: *mut () = 0x4000_1800usize as _;
pub const TMR0: tmr::TmrNodma = unsafe { tmr::TmrNodma::from_ptr(0x4000_2000usize as _) };
pub const TMR1: tmr::TmrFull = unsafe { tmr::TmrFull::from_ptr(0x4000_2400usize as _) };
pub const TMR2: tmr::TmrFull = unsafe { tmr::TmrFull::from_ptr(0x4000_2800usize as _) };
pub const UART0: uart::UartFull = unsafe { uart::UartFull::from_ptr(0x4000_3000usize as _) };
pub const UART1: uart::UartNomodem = unsafe { uart::UartNomodem::from_ptr(0x4000_3400usize as _) };
pub const UART2: uart::UartNomodem = unsafe { uart::UartNomodem::from_ptr(0x4000_3800usize as _) };
pub const UART3: uart::UartNomodem = unsafe { uart::UartNomodem::from_ptr(0x4000_3c00usize as _) };
pub const SPI0: spi::SpiFull = unsafe { spi::SpiFull::from_ptr(0x4000_4000usize as _) };
pub const SPI1: spi::SpiFull = unsafe { spi::SpiFull::from_ptr(0x4000_4400usize as _) };
pub const PWMX: pwmx::Pwmx = unsafe { pwmx::Pwmx::from_ptr(0x4000_5000usize as _) };
pub const HSPI: hspi::Hspi = unsafe { hspi::Hspi::from_ptr(0x4000_6000usize as _) };
pub const ECDC: ecdc::Ecdc = unsafe { ecdc::Ecdc::from_ptr(0x4000_7000usize as _) };
pub const USBSS: *mut () = 0x4000_8000usize as _;
pub const USBHS: usbhs::Usbhs = unsafe { usbhs::Usbhs::from_ptr(0x4000_9000usize as _) };
pub const EMMC: emmc::Emmc = unsafe { emmc::Emmc::from_ptr(0x4000_a000usize as _) };
pub const SERDES: *mut () = 0x4000_b000usize as _;
pub const ETH: *mut () = 0x4000_c000usize as _;
pub const DVP: dvp::Dvp = unsafe { dvp::Dvp::from_ptr(0x4000_e000usize as _) };
pub const PFIC: pfic::Pfic = unsafe { pfic::Pfic::from_ptr(0xe000_e000usize as _) };
pub const SYSTICK: systick::Systick = unsafe { systick::Systick::from_ptr(0xe000_f000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/dvp_ch569.rs"]
pub mod dvp;
#[path = "../../peripherals/ecdc_ch569.rs"]
pub mod ecdc;
#[path = "../../peripherals/emmc_ch569.rs"]
pub mod emmc;
#[path = "../../peripherals/gpio_ch569.rs"]
pub mod gpio;
#[path = "../../peripherals/hspi_ch569.rs"]
pub mod hspi;
#[path = "../../peripherals/pfic_ch569.rs"]
pub mod pfic;
#[path = "../../peripherals/pwmx_ch569.rs"]
pub mod pwmx;
#[path = "../../peripherals/spi_ch569.rs"]
pub mod spi;
#[path = "../../peripherals/sys_ch569.rs"]
pub mod sys;
#[path = "../../peripherals/system_control_ch569.rs"]
pub mod system_control;
#[path = "../../peripherals/systick_ch569.rs"]
pub mod systick;
#[path = "../../peripherals/tmr_ch569.rs"]
pub mod tmr;
#[path = "../../peripherals/uart_ch569.rs"]
pub mod uart;
#[path = "../../peripherals/usbhs_ch569.rs"]
pub mod usbhs;
pub const CORE_INDEX: usize = 0;
#[deprecated(note = "use ch5xx_metapac::MEMORY_LAYOUT instead")]
pub const FLASH_BASE: usize = 0;
#[deprecated(note = "use ch5xx_metapac::MEMORY_LAYOUT instead")]
pub const FLASH_SIZE: usize = 491520;
#[deprecated(note = "use ch5xx_metapac::MEMORY_LAYOUT instead")]
pub const WRITE_SIZE: usize = 4;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout {
    regions: crate::memory_select::MEMORY,
};
