#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "SYS register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sys {
    ptr: *mut u8,
}
unsafe impl Send for Sys {}
unsafe impl Sync for Sys {}
impl Sys {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "output clock divider from PLL."]
    #[inline(always)]
    pub const fn pll_out_div(self) -> crate::common::Reg<regs::PllOutDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "sleep clock off control byte 0."]
    #[inline(always)]
    pub const fn slp_clk_off0(self) -> crate::common::Reg<regs::SlpClkOff0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "sleep clock off control byte 1."]
    #[inline(always)]
    pub const fn slp_clk_off1(self) -> crate::common::Reg<regs::SlpClkOff1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dusize) as _) }
    }
    #[doc = "wake control."]
    #[inline(always)]
    pub const fn slp_wake_ctrl(self) -> crate::common::Reg<regs::SlpWakeCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "PLL SLEEP control."]
    #[inline(always)]
    pub const fn slp_ctrl_pll(self) -> crate::common::Reg<regs::SlpCtrlPll, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fusize) as _) }
    }
    #[doc = "external bus configuration."]
    #[inline(always)]
    pub const fn xbus_config(self) -> crate::common::Reg<regs::XbusConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "external bus read/write cyccles register."]
    #[inline(always)]
    pub const fn xbus_cycle(self) -> crate::common::Reg<regs::XbusCycle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "external bus holdtime setting register."]
    #[inline(always)]
    pub const fn xbus_setup_hold(
        self,
    ) -> crate::common::Reg<regs::XbusSetupHold, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13usize) as _) }
    }
}
pub mod regs {
    #[doc = "output clock divider from PLL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PllOutDiv(pub u8);
    impl PllOutDiv {
        #[doc = "System clock division factor."]
        #[must_use]
        #[inline(always)]
        pub const fn pll_sys_div(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "System clock division factor."]
        #[inline(always)]
        pub const fn set_pll_sys_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
        }
    }
    impl Default for PllOutDiv {
        #[inline(always)]
        fn default() -> PllOutDiv {
            PllOutDiv(0)
        }
    }
    impl core::fmt::Debug for PllOutDiv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PllOutDiv")
                .field("pll_sys_div", &self.pll_sys_div())
                .finish()
        }
    }
    #[doc = "sleep clock off control byte 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpClkOff0(pub u8);
    impl SlpClkOff0 {
        #[doc = "sleep TMR0 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_tmr0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "sleep TMR0 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_tmr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "sleep TMR1 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_tmr1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "sleep TMR1 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_tmr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "sleep TMR2 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_tmr2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "sleep TMR2 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_tmr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "sleep TMR3 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_tmr3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "sleep TMR3 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_tmr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "sleep SPI0 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_spi0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "sleep SPI0 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_spi0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "sleep SPI1 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_spi1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "sleep SPI1 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_spi1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "sleep UART0 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_uart0(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "sleep UART0 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_uart0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "sleep UART1 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_uart1(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "sleep UART1 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_uart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for SlpClkOff0 {
        #[inline(always)]
        fn default() -> SlpClkOff0 {
            SlpClkOff0(0)
        }
    }
    impl core::fmt::Debug for SlpClkOff0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SlpClkOff0")
                .field("slp_clk_tmr0", &self.slp_clk_tmr0())
                .field("slp_clk_tmr1", &self.slp_clk_tmr1())
                .field("slp_clk_tmr2", &self.slp_clk_tmr2())
                .field("slp_clk_tmr3", &self.slp_clk_tmr3())
                .field("slp_clk_spi0", &self.slp_clk_spi0())
                .field("slp_clk_spi1", &self.slp_clk_spi1())
                .field("slp_clk_uart0", &self.slp_clk_uart0())
                .field("slp_clk_uart1", &self.slp_clk_uart1())
                .finish()
        }
    }
    #[doc = "sleep clock off control byte 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpClkOff1(pub u8);
    impl SlpClkOff1 {
        #[doc = "sleep UTMI clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_utmi(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "sleep UTMI clock."]
        #[inline(always)]
        pub const fn set_slp_clk_utmi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "sleep I2C clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_i2c(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "sleep I2C clock."]
        #[inline(always)]
        pub const fn set_slp_clk_i2c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "sleep USBPD clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_usbpd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "sleep USBPD clock."]
        #[inline(always)]
        pub const fn set_slp_clk_usbpd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "sleep ADC clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_adc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "sleep ADC clock."]
        #[inline(always)]
        pub const fn set_slp_clk_adc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "sleep GPIO clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_gpio(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "sleep GPIO clock."]
        #[inline(always)]
        pub const fn set_slp_clk_gpio(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "sleep USB clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_usb(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "sleep USB clock."]
        #[inline(always)]
        pub const fn set_slp_clk_usb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "sleep Ethernet clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_eth(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "sleep Ethernet clock."]
        #[inline(always)]
        pub const fn set_slp_clk_eth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for SlpClkOff1 {
        #[inline(always)]
        fn default() -> SlpClkOff1 {
            SlpClkOff1(0)
        }
    }
    impl core::fmt::Debug for SlpClkOff1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SlpClkOff1")
                .field("slp_clk_utmi", &self.slp_clk_utmi())
                .field("slp_clk_i2c", &self.slp_clk_i2c())
                .field("slp_clk_usbpd", &self.slp_clk_usbpd())
                .field("slp_clk_adc", &self.slp_clk_adc())
                .field("slp_clk_gpio", &self.slp_clk_gpio())
                .field("slp_clk_usb", &self.slp_clk_usb())
                .field("slp_clk_eth", &self.slp_clk_eth())
                .finish()
        }
    }
    #[doc = "PLL SLEEP control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpCtrlPll(pub u8);
    impl SlpCtrlPll {
        #[doc = "Current ethernet WOL remote wake-up signal status."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_wol_status(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Current ethernet WOL remote wake-up signal status."]
        #[inline(always)]
        pub const fn set_slp_wol_status(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "sleep UART2 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_uart2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "sleep UART2 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_uart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "sleep UART3 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_uart3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "sleep UART3 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_uart3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
    }
    impl Default for SlpCtrlPll {
        #[inline(always)]
        fn default() -> SlpCtrlPll {
            SlpCtrlPll(0)
        }
    }
    impl core::fmt::Debug for SlpCtrlPll {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SlpCtrlPll")
                .field("slp_wol_status", &self.slp_wol_status())
                .field("slp_clk_uart2", &self.slp_clk_uart2())
                .field("slp_clk_uart3", &self.slp_clk_uart3())
                .finish()
        }
    }
    #[doc = "wake control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpWakeCtrl(pub u8);
    impl SlpWakeCtrl {
        #[doc = "enable PA waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_pa_wake(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable PA waking."]
        #[inline(always)]
        pub const fn set_slp_pa_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable PB waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_pb_wake(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enable PB waking."]
        #[inline(always)]
        pub const fn set_slp_pb_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "enable USBPD waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_usbpd_wake(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable USBPD waking."]
        #[inline(always)]
        pub const fn set_slp_usbpd_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable PD waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_pd_wake(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable PD waking."]
        #[inline(always)]
        pub const fn set_slp_pd_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "enable USB waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_usb_wake(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB waking."]
        #[inline(always)]
        pub const fn set_slp_usb_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable Ethernet WOL waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_wol_wake(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable Ethernet WOL waking."]
        #[inline(always)]
        pub const fn set_slp_wol_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Ethernet module power control."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_eth_pwr_dn(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet module power control."]
        #[inline(always)]
        pub const fn set_slp_eth_pwr_dn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for SlpWakeCtrl {
        #[inline(always)]
        fn default() -> SlpWakeCtrl {
            SlpWakeCtrl(0)
        }
    }
    impl core::fmt::Debug for SlpWakeCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SlpWakeCtrl")
                .field("slp_pa_wake", &self.slp_pa_wake())
                .field("slp_pb_wake", &self.slp_pb_wake())
                .field("slp_usbpd_wake", &self.slp_usbpd_wake())
                .field("slp_pd_wake", &self.slp_pd_wake())
                .field("slp_usb_wake", &self.slp_usb_wake())
                .field("slp_wol_wake", &self.slp_wol_wake())
                .field("slp_eth_pwr_dn", &self.slp_eth_pwr_dn())
                .finish()
        }
    }
    #[doc = "external bus configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct XbusConfig(pub u8);
    impl XbusConfig {
        #[doc = "external bus enable."]
        #[must_use]
        #[inline(always)]
        pub const fn xbus_enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "external bus enable."]
        #[inline(always)]
        pub const fn set_xbus_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "external bus data control enable."]
        #[must_use]
        #[inline(always)]
        pub const fn xbus_en_32bit(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "external bus data control enable."]
        #[inline(always)]
        pub const fn set_xbus_en_32bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "external bus address output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn xbus_addr_oe(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "external bus address output enable."]
        #[inline(always)]
        pub const fn set_xbus_addr_oe(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
    }
    impl Default for XbusConfig {
        #[inline(always)]
        fn default() -> XbusConfig {
            XbusConfig(0)
        }
    }
    impl core::fmt::Debug for XbusConfig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("XbusConfig")
                .field("xbus_enable", &self.xbus_enable())
                .field("xbus_en_32bit", &self.xbus_en_32bit())
                .field("xbus_addr_oe", &self.xbus_addr_oe())
                .finish()
        }
    }
    #[doc = "external bus read/write cyccles register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct XbusCycle(pub u8);
    impl XbusCycle {
        #[doc = "external bus read/write cyccles."]
        #[must_use]
        #[inline(always)]
        pub const fn xbus_cycle(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "external bus read/write cyccles."]
        #[inline(always)]
        pub const fn set_xbus_cycle(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
    }
    impl Default for XbusCycle {
        #[inline(always)]
        fn default() -> XbusCycle {
            XbusCycle(0)
        }
    }
    impl core::fmt::Debug for XbusCycle {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("XbusCycle")
                .field("xbus_cycle", &self.xbus_cycle())
                .finish()
        }
    }
    #[doc = "external bus holdtime setting register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct XbusSetupHold(pub u8);
    impl XbusSetupHold {
        #[doc = "Number of external bus read/write signal end point clock."]
        #[must_use]
        #[inline(always)]
        pub const fn xbus_hold(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "Number of external bus read/write signal end point clock."]
        #[inline(always)]
        pub const fn set_xbus_hold(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u8) & 0x1f) << 0usize);
        }
        #[doc = "The address of the external bus and the creation time of. the data."]
        #[must_use]
        #[inline(always)]
        pub const fn xbus_setup(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The address of the external bus and the creation time of. the data."]
        #[inline(always)]
        pub const fn set_xbus_setup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for XbusSetupHold {
        #[inline(always)]
        fn default() -> XbusSetupHold {
            XbusSetupHold(0)
        }
    }
    impl core::fmt::Debug for XbusSetupHold {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("XbusSetupHold")
                .field("xbus_hold", &self.xbus_hold())
                .field("xbus_setup", &self.xbus_setup())
                .finish()
        }
    }
}
