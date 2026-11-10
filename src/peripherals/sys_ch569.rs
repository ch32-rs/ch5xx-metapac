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
    pub const fn clk_pll_div(self) -> crate::common::Reg<regs::ClkPllDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "clock control."]
    #[inline(always)]
    pub const fn clk_cfg_ctrl(self) -> crate::common::Reg<regs::ClkCfgCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "clock mode aux register."]
    #[inline(always)]
    pub const fn clk_mod_aux(self) -> crate::common::Reg<regs::ClkModAux, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0busize) as _) }
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
    #[doc = "power control."]
    #[inline(always)]
    pub const fn slp_power_ctrl(self) -> crate::common::Reg<regs::SlpPowerCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fusize) as _) }
    }
    #[doc = "alternate pin control."]
    #[inline(always)]
    pub const fn pin_alternate(self) -> crate::common::Reg<regs::PinAlternate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "Serdes Analog parameter configuration1."]
    #[inline(always)]
    pub const fn serd_ana_cfg1(self) -> crate::common::Reg<regs::SerdAnaCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Serdes Analog parameter configuration2."]
    #[inline(always)]
    pub const fn serd_ana_cfg2(self) -> crate::common::Reg<regs::SerdAnaCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
}
pub mod regs {
    #[doc = "clock control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClkCfgCtrl(pub u8);
    impl ClkCfgCtrl {
        #[doc = "PLL sleep control."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_pll_sleep(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PLL sleep control."]
        #[inline(always)]
        pub const fn set_clk_pll_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "clock source selection."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_sel_pll(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "clock source selection."]
        #[inline(always)]
        pub const fn set_clk_sel_pll(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
    }
    impl Default for ClkCfgCtrl {
        #[inline(always)]
        fn default() -> ClkCfgCtrl {
            ClkCfgCtrl(0)
        }
    }
    impl core::fmt::Debug for ClkCfgCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ClkCfgCtrl")
                .field("clk_pll_sleep", &self.clk_pll_sleep())
                .field("clk_sel_pll", &self.clk_sel_pll())
                .finish()
        }
    }
    #[doc = "clock mode aux register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClkModAux(pub u8);
    impl ClkModAux {
        #[doc = "clock from USB_PHY PCLK(125MHz)."]
        #[must_use]
        #[inline(always)]
        pub const fn int_125m_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "clock from USB_PHY PCLK(125MHz)."]
        #[inline(always)]
        pub const fn set_int_125m_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "clock from pin_PA\\[16\\]."]
        #[must_use]
        #[inline(always)]
        pub const fn ext_125m_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "clock from pin_PA\\[16\\]."]
        #[inline(always)]
        pub const fn set_ext_125m_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "MCO output selection."]
        #[must_use]
        #[inline(always)]
        pub const fn mco_sel_msk(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "MCO output selection."]
        #[inline(always)]
        pub const fn set_mco_sel_msk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "MCO output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mco_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MCO output enable."]
        #[inline(always)]
        pub const fn set_mco_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for ClkModAux {
        #[inline(always)]
        fn default() -> ClkModAux {
            ClkModAux(0)
        }
    }
    impl core::fmt::Debug for ClkModAux {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ClkModAux")
                .field("int_125m_en", &self.int_125m_en())
                .field("ext_125m_en", &self.ext_125m_en())
                .field("mco_sel_msk", &self.mco_sel_msk())
                .field("mco_en", &self.mco_en())
                .finish()
        }
    }
    #[doc = "output clock divider from PLL."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClkPllDiv(pub u8);
    impl ClkPllDiv {
        #[doc = "output clock divider from PLL."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_pll_div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "output clock divider from PLL."]
        #[inline(always)]
        pub const fn set_clk_pll_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for ClkPllDiv {
        #[inline(always)]
        fn default() -> ClkPllDiv {
            ClkPllDiv(0)
        }
    }
    impl core::fmt::Debug for ClkPllDiv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ClkPllDiv")
                .field("clk_pll_div", &self.clk_pll_div())
                .finish()
        }
    }
    #[doc = "alternate pin control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PinAlternate(pub u8);
    impl PinAlternate {
        #[doc = "ETH mii interface selection."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_mii(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ETH mii interface selection."]
        #[inline(always)]
        pub const fn set_pin_mii(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "TMR1 alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_tmr1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "TMR1 alternate pin enable."]
        #[inline(always)]
        pub const fn set_pin_tmr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "TMR2 alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_tmr2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "TMR2 alternate pin enable."]
        #[inline(always)]
        pub const fn set_pin_tmr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RXD0/TXD0 alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_uart0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RXD0/TXD0 alternate pin enable."]
        #[inline(always)]
        pub const fn set_pin_uart0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for PinAlternate {
        #[inline(always)]
        fn default() -> PinAlternate {
            PinAlternate(0)
        }
    }
    impl core::fmt::Debug for PinAlternate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PinAlternate")
                .field("pin_mii", &self.pin_mii())
                .field("pin_tmr1", &self.pin_tmr1())
                .field("pin_tmr2", &self.pin_tmr2())
                .field("pin_uart0", &self.pin_uart0())
                .finish()
        }
    }
    #[doc = "Serdes Analog parameter configuration1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SerdAnaCfg1(pub u16);
    impl SerdAnaCfg1 {
        #[doc = "SerDes PHY internal configuration bit."]
        #[must_use]
        #[inline(always)]
        pub const fn serd_pll_cfg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SerDes PHY internal configuration bit."]
        #[inline(always)]
        pub const fn set_serd_pll_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
        #[doc = "SerDes PHY reference clock source seletion."]
        #[must_use]
        #[inline(always)]
        pub const fn serd_30m_sel(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SerDes PHY reference clock source seletion."]
        #[inline(always)]
        pub const fn set_serd_30m_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "Enable SerDes PHY GXM test pin."]
        #[must_use]
        #[inline(always)]
        pub const fn serd_dn_sel(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Enable SerDes PHY GXM test pin."]
        #[inline(always)]
        pub const fn set_serd_dn_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
    }
    impl Default for SerdAnaCfg1 {
        #[inline(always)]
        fn default() -> SerdAnaCfg1 {
            SerdAnaCfg1(0)
        }
    }
    impl core::fmt::Debug for SerdAnaCfg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SerdAnaCfg1")
                .field("serd_pll_cfg", &self.serd_pll_cfg())
                .field("serd_30m_sel", &self.serd_30m_sel())
                .field("serd_dn_sel", &self.serd_dn_sel())
                .finish()
        }
    }
    #[doc = "Serdes Analog parameter configuration2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SerdAnaCfg2(pub u32);
    impl SerdAnaCfg2 {
        #[doc = "Tx and RX parameter setting."]
        #[must_use]
        #[inline(always)]
        pub const fn serd_trx_cfg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Tx and RX parameter setting."]
        #[inline(always)]
        pub const fn set_serd_trx_cfg(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
        }
    }
    impl Default for SerdAnaCfg2 {
        #[inline(always)]
        fn default() -> SerdAnaCfg2 {
            SerdAnaCfg2(0)
        }
    }
    impl core::fmt::Debug for SerdAnaCfg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SerdAnaCfg2")
                .field("serd_trx_cfg", &self.serd_trx_cfg())
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
        #[doc = "sleep PWMX clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_pwmx(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "sleep PWMX clock."]
        #[inline(always)]
        pub const fn set_slp_clk_pwmx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "sleep UART0 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_uart0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "sleep UART0 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_uart0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "sleep UART1 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_uart1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "sleep UART1 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_uart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "sleep UART2 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_uart2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "sleep UART2 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_uart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "sleep UART3 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_uart3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "sleep UART3 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_uart3(&mut self, val: bool) {
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
                .field("slp_clk_pwmx", &self.slp_clk_pwmx())
                .field("slp_clk_uart0", &self.slp_clk_uart0())
                .field("slp_clk_uart1", &self.slp_clk_uart1())
                .field("slp_clk_uart2", &self.slp_clk_uart2())
                .field("slp_clk_uart3", &self.slp_clk_uart3())
                .finish()
        }
    }
    #[doc = "sleep clock off control byte 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpClkOff1(pub u8);
    impl SlpClkOff1 {
        #[doc = "sleep SPI0 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_spi0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "sleep SPI0 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_spi0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "sleep SPI1 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_spi1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "sleep SPI1 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_spi1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "sleep EMMC clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_emmc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "sleep EMMC clock."]
        #[inline(always)]
        pub const fn set_slp_clk_emmc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "sleep HSPI clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_hspi(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "sleep HSPI clock."]
        #[inline(always)]
        pub const fn set_slp_clk_hspi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "sleep USBHS clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_usbhs(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "sleep USBHS clock."]
        #[inline(always)]
        pub const fn set_slp_clk_usbhs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "sleep USBSS clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_usbss(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "sleep USBSS clock."]
        #[inline(always)]
        pub const fn set_slp_clk_usbss(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "sleep SERD clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_serd(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "sleep SERD clock."]
        #[inline(always)]
        pub const fn set_slp_clk_serd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "sleep DVP clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_dvp(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "sleep DVP clock."]
        #[inline(always)]
        pub const fn set_slp_clk_dvp(&mut self, val: bool) {
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
                .field("slp_clk_spi0", &self.slp_clk_spi0())
                .field("slp_clk_spi1", &self.slp_clk_spi1())
                .field("slp_clk_emmc", &self.slp_clk_emmc())
                .field("slp_clk_hspi", &self.slp_clk_hspi())
                .field("slp_clk_usbhs", &self.slp_clk_usbhs())
                .field("slp_clk_usbss", &self.slp_clk_usbss())
                .field("slp_clk_serd", &self.slp_clk_serd())
                .field("slp_clk_dvp", &self.slp_clk_dvp())
                .finish()
        }
    }
    #[doc = "power control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpPowerCtrl(pub u8);
    impl SlpPowerCtrl {
        #[doc = "enable USBHS power down."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_usbhs_pwrdn(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable USBHS power down."]
        #[inline(always)]
        pub const fn set_slp_usbhs_pwrdn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for SlpPowerCtrl {
        #[inline(always)]
        fn default() -> SlpPowerCtrl {
            SlpPowerCtrl(0)
        }
    }
    impl core::fmt::Debug for SlpPowerCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SlpPowerCtrl")
                .field("slp_usbhs_pwrdn", &self.slp_usbhs_pwrdn())
                .finish()
        }
    }
    #[doc = "wake control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpWakeCtrl(pub u8);
    impl SlpWakeCtrl {
        #[doc = "enable USBHS waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_usbhs_wake(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable USBHS waking."]
        #[inline(always)]
        pub const fn set_slp_usbhs_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable USBSS waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_usbss_wake(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enable USBSS waking."]
        #[inline(always)]
        pub const fn set_slp_usbss_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "sleep ETH clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_eth(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "sleep ETH clock."]
        #[inline(always)]
        pub const fn set_slp_clk_eth(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "sleep ECDC clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_ecdc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "sleep ECDC clock."]
        #[inline(always)]
        pub const fn set_slp_clk_ecdc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "enable GPIO waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_gpio_wake(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable GPIO waking."]
        #[inline(always)]
        pub const fn set_slp_gpio_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable Eth waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_eth_wake(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable Eth waking."]
        #[inline(always)]
        pub const fn set_slp_eth_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
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
                .field("slp_usbhs_wake", &self.slp_usbhs_wake())
                .field("slp_usbss_wake", &self.slp_usbss_wake())
                .field("slp_clk_eth", &self.slp_clk_eth())
                .field("slp_clk_ecdc", &self.slp_clk_ecdc())
                .field("slp_gpio_wake", &self.slp_gpio_wake())
                .field("slp_eth_wake", &self.slp_eth_wake())
                .finish()
        }
    }
}
