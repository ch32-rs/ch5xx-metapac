#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "16 - TMR0"]
    TMR0 = 16,
    #[doc = "17 - GPIO_A"]
    GPIO_A = 17,
    #[doc = "18 - GPIO_B"]
    GPIO_B = 18,
    #[doc = "19 - SPI0"]
    SPI0 = 19,
    #[doc = "20 - BLEB"]
    BLEB = 20,
    #[doc = "21 - BLEL"]
    BLEL = 21,
    #[doc = "22 - USB"]
    USB = 22,
    #[doc = "24 - TMR1"]
    TMR1 = 24,
    #[doc = "25 - TMR2"]
    TMR2 = 25,
    #[doc = "26 - UART0"]
    UART0 = 26,
    #[doc = "27 - UART1"]
    UART1 = 27,
    #[doc = "28 - RTC"]
    RTC = 28,
    #[doc = "29 - ADC"]
    ADC = 29,
    #[doc = "30 - I2C0"]
    I2C0 = 30,
    #[doc = "31 - PWMX_SPI1"]
    PWMX_SPI1 = 31,
    #[doc = "32 - TMR3"]
    TMR3 = 32,
    #[doc = "33 - UART2"]
    UART2 = 33,
    #[doc = "34 - UART3"]
    UART3 = 34,
    #[doc = "35 - WDOG_BAT"]
    WDOG_BAT = 35,
    #[doc = "36 - NFC"]
    NFC = 36,
    #[doc = "39 - LED"]
    LED = 39,
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
        fn TMR0();
        fn GPIO_A();
        fn GPIO_B();
        fn SPI0();
        fn BLEB();
        fn BLEL();
        fn USB();
        fn TMR1();
        fn TMR2();
        fn UART0();
        fn UART1();
        fn RTC();
        fn ADC();
        fn I2C0();
        fn PWMX_SPI1();
        fn TMR3();
        fn UART2();
        fn UART3();
        fn WDOG_BAT();
        fn NFC();
        fn LED();
    }
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[unsafe(link_section = ".vector_table.external_interrupts")]
    #[unsafe(no_mangle)]
    pub static __EXTERNAL_INTERRUPTS: [Vector; 24] = [
        Vector { _handler: TMR0 },
        Vector { _handler: GPIO_A },
        Vector { _handler: GPIO_B },
        Vector { _handler: SPI0 },
        Vector { _handler: BLEB },
        Vector { _handler: BLEL },
        Vector { _handler: USB },
        Vector { _reserved: 0 },
        Vector { _handler: TMR1 },
        Vector { _handler: TMR2 },
        Vector { _handler: UART0 },
        Vector { _handler: UART1 },
        Vector { _handler: RTC },
        Vector { _handler: ADC },
        Vector { _handler: I2C0 },
        Vector {
            _handler: PWMX_SPI1,
        },
        Vector { _handler: TMR3 },
        Vector { _handler: UART2 },
        Vector { _handler: UART3 },
        Vector { _handler: WDOG_BAT },
        Vector { _handler: NFC },
        Vector { _reserved: 0 },
        Vector { _reserved: 0 },
        Vector { _handler: LED },
    ];
}
pub const SYS: sys::Sys = unsafe { sys::Sys::from_ptr(0x4000_1000usize as _) };
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4000_1000usize as _) };
pub const SYSTEM_CONTROL: system_control::SystemControl =
    unsafe { system_control::SystemControl::from_ptr(0x4000_1000usize as _) };
pub const TOUCHKEY: touchkey::TouchKey =
    unsafe { touchkey::TouchKey::from_ptr(0x4000_1000usize as _) };
pub const ADC: adc::Adc = unsafe { adc::Adc::from_ptr(0x4000_1000usize as _) };
pub const GPIO: gpio::Gpio = unsafe { gpio::Gpio::from_ptr(0x4000_1000usize as _) };
pub const FLASHROM: *mut () = 0x4000_1800usize as _;
pub const TMR0: tmr::TmrFull = unsafe { tmr::TmrFull::from_ptr(0x4000_2000usize as _) };
pub const TMR1: tmr::TmrFull = unsafe { tmr::TmrFull::from_ptr(0x4000_2400usize as _) };
pub const TMR2: tmr::TmrFull = unsafe { tmr::TmrFull::from_ptr(0x4000_2800usize as _) };
pub const TMR3: tmr::TmrFull = unsafe { tmr::TmrFull::from_ptr(0x4000_2c00usize as _) };
pub const UART0: uart::UartFull = unsafe { uart::UartFull::from_ptr(0x4000_3000usize as _) };
pub const UART1: uart::UartFull = unsafe { uart::UartFull::from_ptr(0x4000_3400usize as _) };
pub const UART2: uart::UartFull = unsafe { uart::UartFull::from_ptr(0x4000_3800usize as _) };
pub const UART3: uart::UartFull = unsafe { uart::UartFull::from_ptr(0x4000_3c00usize as _) };
pub const SPI0: spi::SpiFull = unsafe { spi::SpiFull::from_ptr(0x4000_4000usize as _) };
pub const I2C0: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4000_4800usize as _) };
pub const PWMX: pwmx::Pwmx = unsafe { pwmx::Pwmx::from_ptr(0x4000_5000usize as _) };
pub const LCD: lcd::Lcd = unsafe { lcd::Lcd::from_ptr(0x4000_6000usize as _) };
pub const USBFS: usbfs::Usbfs = unsafe { usbfs::Usbfs::from_ptr(0x4000_8000usize as _) };
pub const BLE: *mut () = 0x4000_c000usize as _;
pub const LED: led::Led = unsafe { led::Led::from_ptr(0x4000_f000usize as _) };
pub const PFIC: pfic::Pfic = unsafe { pfic::Pfic::from_ptr(0xe000_e000usize as _) };
pub const SYSTICK: systick::Systick = unsafe { systick::Systick::from_ptr(0xe000_f000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/adc_ch585.rs"]
pub mod adc;
#[path = "../../peripherals/gpio_ch585.rs"]
pub mod gpio;
#[path = "../../peripherals/i2c_ch585.rs"]
pub mod i2c;
#[path = "../../peripherals/lcd_ch585.rs"]
pub mod lcd;
#[path = "../../peripherals/led_ch585.rs"]
pub mod led;
#[path = "../../peripherals/pfic_ch585.rs"]
pub mod pfic;
#[path = "../../peripherals/pwmx_ch585.rs"]
pub mod pwmx;
#[path = "../../peripherals/rtc_ch585.rs"]
pub mod rtc;
#[path = "../../peripherals/spi_ch585.rs"]
pub mod spi;
#[path = "../../peripherals/sys_ch585.rs"]
pub mod sys;
#[path = "../../peripherals/system_control_ch585.rs"]
pub mod system_control;
#[path = "../../peripherals/systick_ch585.rs"]
pub mod systick;
#[path = "../../peripherals/tmr_ch585.rs"]
pub mod tmr;
#[path = "../../peripherals/touchkey_ch585.rs"]
pub mod touchkey;
#[path = "../../peripherals/uart_ch585.rs"]
pub mod uart;
#[path = "../../peripherals/usbfs_ch585.rs"]
pub mod usbfs;
pub const CORE_INDEX: usize = 0;
#[deprecated(note = "use ch5xx_metapac::MEMORY_LAYOUT instead")]
pub const FLASH_BASE: usize = 0;
#[deprecated(note = "use ch5xx_metapac::MEMORY_LAYOUT instead")]
pub const FLASH_SIZE: usize = 1015808;
#[deprecated(note = "use ch5xx_metapac::MEMORY_LAYOUT instead")]
pub const WRITE_SIZE: usize = 4;
pub const MEMORY_LAYOUT: crate::mem_layout::MemoryLayout = crate::mem_layout::MemoryLayout {
    regions: crate::memory_select::MEMORY,
};
