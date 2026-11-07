#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "16 - RESERVED16"]
    RESERVED16 = 16,
    #[doc = "17 - GPIO_A"]
    GPIO_A = 17,
    #[doc = "19 - SPI0"]
    SPI0 = 19,
    #[doc = "20 - BLEB"]
    BLEB = 20,
    #[doc = "21 - BLEL"]
    BLEL = 21,
    #[doc = "22 - USB"]
    USB = 22,
    #[doc = "24 - TMR0"]
    TMR0 = 24,
    #[doc = "27 - UART0"]
    UART0 = 27,
    #[doc = "28 - RTC"]
    RTC = 28,
    #[doc = "29 - CMP"]
    CMP = 29,
    #[doc = "30 - I2C0"]
    I2C0 = 30,
    #[doc = "31 - PWMX"]
    PWMX = 31,
    #[doc = "33 - KEYSCAN"]
    KEYSCAN = 33,
    #[doc = "34 - ENCODER"]
    ENCODER = 34,
    #[doc = "35 - WDOG_BAT"]
    WDOG_BAT = 35,
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
        fn RESERVED16();
        fn GPIO_A();
        fn SPI0();
        fn BLEB();
        fn BLEL();
        fn USB();
        fn TMR0();
        fn UART0();
        fn RTC();
        fn CMP();
        fn I2C0();
        fn PWMX();
        fn KEYSCAN();
        fn ENCODER();
        fn WDOG_BAT();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.external_interrupts")]
    #[unsafe(no_mangle)]
    pub static __EXTERNAL_INTERRUPTS: [Vector; 20] = [
        Vector {
            _handler: RESERVED16,
        },
        Vector { _handler: GPIO_A },
        Vector { _reserved: 0 },
        Vector { _handler: SPI0 },
        Vector { _handler: BLEB },
        Vector { _handler: BLEL },
        Vector { _handler: USB },
        Vector { _reserved: 0 },
        Vector { _handler: TMR0 },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: UART0 },
        Vector { _handler: RTC },
        Vector { _handler: CMP },
        Vector { _handler: I2C0 },
        Vector { _handler: PWMX },
        Vector { _reserved: 0 },
        Vector { _handler: KEYSCAN },
        Vector { _handler: ENCODER },
        Vector { _handler: WDOG_BAT },
    ];
}
pub const SYS: sys::Sys = unsafe { sys::Sys::from_ptr(0x4000_1000usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_1000usize as _) };
pub const SYSTEM_CONTROL: system_control::SystemControl =
    unsafe { system_control::SystemControl::from_ptr(0x4000_1000usize as _) };
pub const CMP: cmp::Cmp = unsafe { cmp::Cmp::from_ptr(0x4000_1000usize as _) };
pub const KEYSCAN: keyscan::Keyscan = unsafe { keyscan::Keyscan::from_ptr(0x4000_1000usize as _) };
pub const GPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4000_1000usize as _) };
pub const TMR0: tmr::TmrFull = unsafe { tmr::TmrFull::from_ptr(0x4000_2400usize as _) };
pub const ENCODER: encoder::Encoder = unsafe { encoder::Encoder::from_ptr(0x4000_2400usize as _) };
pub const UART0: uart::UartFull = unsafe { uart::UartFull::from_ptr(0x4000_3400usize as _) };
pub const SPI0: spi::SpiFull = unsafe { spi::SpiFull::from_ptr(0x4000_4000usize as _) };
pub const I2C0: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_4800usize as _) };
pub const PWMX: pwmx::Pwmx = unsafe { pwmx::Pwmx::from_ptr(0x4000_5000usize as _) };
pub const USBFS: usbfs::Usbfs = unsafe { usbfs::Usbfs::from_ptr(0x4000_8000usize as _) };
pub const BLE: *mut () = 0x4000_c000usize as _;
pub const PFIC: pfic::Pfic = unsafe { pfic::Pfic::from_ptr(0xe000_e000usize as _) };
pub const SYSTICK: systick::Systick = unsafe { systick::Systick::from_ptr(0xe000_f000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/cmp_ch572.rs"]
pub mod cmp;
#[path = "../../peripherals/encoder_ch572.rs"]
pub mod encoder;
#[path = "../../peripherals/gpio_ch572.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_ch585.rs"]
pub mod i2c;
#[path = "../../peripherals/keyscan_ch572.rs"]
pub mod keyscan;
#[path = "../../peripherals/pfic_ch585.rs"]
pub mod pfic;
#[path = "../../peripherals/pwmx_ch572.rs"]
pub mod pwmx;
#[path = "../../peripherals/rtc_ch572.rs"]
pub mod rtc;
#[path = "../../peripherals/spi_ch572.rs"]
pub mod spi;
#[path = "../../peripherals/sys_ch572.rs"]
pub mod sys;
#[path = "../../peripherals/system_control_ch572.rs"]
pub mod system_control;
#[path = "../../peripherals/systick_ch572.rs"]
pub mod systick;
#[path = "../../peripherals/tmr_ch572.rs"]
pub mod tmr;
#[path = "../../peripherals/uart_ch572.rs"]
pub mod uart;
#[path = "../../peripherals/usbfs_ch585.rs"]
pub mod usbfs;
pub const CORE_INDEX: usize = 0;
#[deprecated(note = "use ch5xx_metapac::MEMORY_LAYOUT instead")]
pub const FLASH_BASE: usize = 0;
#[deprecated(note = "use ch5xx_metapac::MEMORY_LAYOUT instead")]
pub const FLASH_SIZE: usize = 245760;
#[deprecated(note = "use ch5xx_metapac::MEMORY_LAYOUT instead")]
pub const WRITE_SIZE: usize = 4;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout {
    regions: crate::memory_select::MEMORY,
};
