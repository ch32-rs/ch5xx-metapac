#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "USB register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbhs {
    ptr: *mut u8,
}
unsafe impl Send for Usbhs {}
unsafe impl Sync for Usbhs {}
impl Usbhs {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RW, USBHS control register."]
    #[inline(always)]
    pub const fn usb2_ctrl(self) -> crate::common::Reg<regs::Usb2Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RW, USBHS control register."]
    #[inline(always)]
    pub const fn usb2_base_mode(self) -> crate::common::Reg<regs::Usb2BaseMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "RW, USBHS interrupt enable register."]
    #[inline(always)]
    pub const fn usb2_int_en(self) -> crate::common::Reg<regs::Usb2IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "RW, USBHS device address register."]
    #[inline(always)]
    pub const fn usb2_dev_ad(self) -> crate::common::Reg<regs::Usb2DevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "RW1Z, USBHS remote wake-up register."]
    #[inline(always)]
    pub const fn usb2_wake_ctrl(self) -> crate::common::Reg<regs::Usb2WakeCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RW, USBHS test mode register."]
    #[inline(always)]
    pub const fn usb2_test_mode(self) -> crate::common::Reg<regs::Usb2TestMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "RW, USBHS power management register."]
    #[inline(always)]
    pub const fn usb2_lpm_data(self) -> crate::common::Reg<regs::Usb2LpmData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "RW1Z, USBHS interrupt flag register."]
    #[inline(always)]
    pub const fn usb2_int_fg(self) -> crate::common::Reg<regs::Usb2IntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RO, USBHS Interrupt Status Register."]
    #[inline(always)]
    pub const fn usb2_int_st(self) -> crate::common::Reg<regs::Usb2IntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "RO, USBHS Miscellaneous Status Register."]
    #[inline(always)]
    pub const fn usb2_mis_st(self) -> crate::common::Reg<regs::Usb2MisSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "RO, USBHS frame register."]
    #[inline(always)]
    pub const fn usb2_fram_no(self) -> crate::common::Reg<regs::Usb2FramNo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "RO, USBHS bus status memory."]
    #[inline(always)]
    pub const fn usb2_bus(self) -> crate::common::Reg<regs::Usb2Bus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "RW, USBHS endpoint sends enable register."]
    #[inline(always)]
    pub const fn uep_tx_en(self) -> crate::common::Reg<regs::UepTxEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "RW, USBHS endpoint receive enable register."]
    #[inline(always)]
    pub const fn uep_rx_en(self) -> crate::common::Reg<regs::UepRxEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "RW, USBHS endpoint receives automatic flip enable register."]
    #[inline(always)]
    pub const fn uep_t_tog_auto(self) -> crate::common::Reg<regs::UepTTogAuto, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "RW, USBHS endpoint receives automatic flip enable register."]
    #[inline(always)]
    pub const fn uep_r_tog_auto(self) -> crate::common::Reg<regs::UepRTogAuto, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "RW, 0~7 endpoints burst send enable."]
    #[inline(always)]
    pub const fn uep_t_burst(self) -> crate::common::Reg<regs::UepTBurst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "RW, USBHS Endpoint Send Mode Register."]
    #[inline(always)]
    pub const fn uep_t_burst_mode(
        self,
    ) -> crate::common::Reg<regs::UepTBurstMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x19usize) as _) }
    }
    #[doc = "RW, 0~7 endpoints burst receive enable."]
    #[inline(always)]
    pub const fn uep_r_burst(self) -> crate::common::Reg<regs::UepRBurst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "RW, USBHS Endpoint Receive Mode Register."]
    #[inline(always)]
    pub const fn uep_r_res_mode(self) -> crate::common::Reg<regs::UepRResMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1busize) as _) }
    }
    #[doc = "RW, USBHS Endpoint Reuse Register."]
    #[inline(always)]
    pub const fn uep_af_mode(self) -> crate::common::Reg<regs::UepAfMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "RW, Starting address register of endpoint 0 buffer."]
    #[inline(always)]
    pub const fn uep0_dma(self) -> crate::common::Reg<regs::Uep0Dma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "RW, Endpoint n receive buffer start address (n=1~7)."]
    #[inline(always)]
    pub const fn uep_rx_dma(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxDma, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize + n * 4usize) as _) }
    }
    #[doc = "RW, Endpoint n transmit buffer start address (n=1~7)."]
    #[inline(always)]
    pub const fn uep_tx_dma(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepTxDma, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "RW, Endpoint 0 maximum packet length."]
    #[inline(always)]
    pub const fn uep0_max_len(self) -> crate::common::Reg<regs::Uep0MaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "RW, Endpoint n maximum packet length (n=1~7)."]
    #[inline(always)]
    pub const fn uep_max_len(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepMaxLen, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "RW, Endpoint 0 received length."]
    #[inline(always)]
    pub const fn uep0_rx_len(self) -> crate::common::Reg<regs::Uep0RxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "RW, Endpoint n received length (n=1~7)."]
    #[inline(always)]
    pub const fn uep_rx_len(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxLen, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "RW, Endpoint 0 transmit length."]
    #[inline(always)]
    pub const fn uep0_t_len(self) -> crate::common::Reg<regs::Uep0TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "RW, Endpoint 0 transmit control."]
    #[inline(always)]
    pub const fn uep0_tx_ctrl(self) -> crate::common::Reg<regs::Uep0TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9eusize) as _) }
    }
    #[doc = "RW, Endpoint 0 receive control."]
    #[inline(always)]
    pub const fn uep0_rx_ctrl(self) -> crate::common::Reg<regs::Uep0RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9fusize) as _) }
    }
    #[doc = "RW, Endpoint n transmit length (n=1~7)."]
    #[inline(always)]
    pub const fn uep_t_len(self, n: usize) -> crate::common::Reg<regs::UepTLen, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _) }
    }
    #[doc = "RW, Endpoint n transmit control (n=1~7)."]
    #[inline(always)]
    pub const fn uep_tx_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepTxCtrl, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa2usize + n * 4usize) as _) }
    }
    #[doc = "RW, Endpoint n receive control (n=1~7)."]
    #[inline(always)]
    pub const fn uep_rx_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxCtrl, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa3usize + n * 4usize) as _) }
    }
    #[doc = "RW, USBHS endpoint sends synchronization mode enable register."]
    #[inline(always)]
    pub const fn uep_t_iso(self) -> crate::common::Reg<regs::UepTIso, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "RW, USBHS endpoint receiving synchronization mode enable register."]
    #[inline(always)]
    pub const fn uep_r_iso(self) -> crate::common::Reg<regs::UepRIso, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbeusize) as _) }
    }
    #[doc = "RW, USBHS Host Configuration Register."]
    #[inline(always)]
    pub const fn uh_cfg(self) -> crate::common::Reg<regs::UhCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "RW, USBHS host interrupt enable register."]
    #[inline(always)]
    pub const fn uh_int_en(self) -> crate::common::Reg<regs::UhIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0102usize) as _) }
    }
    #[doc = "RW, USBHS host device address register."]
    #[inline(always)]
    pub const fn uh_dev_addr(self) -> crate::common::Reg<regs::UhDevAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0103usize) as _) }
    }
    #[doc = "RW, USBHS Host Control Register."]
    #[inline(always)]
    pub const fn uh_control(self) -> crate::common::Reg<regs::UhControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "RW1Z, USBHS host interrupt flag register."]
    #[inline(always)]
    pub const fn uh_int_flag(self) -> crate::common::Reg<regs::UhIntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "RW, USBHS host interrupt status register."]
    #[inline(always)]
    pub const fn uh_int_st(self) -> crate::common::Reg<regs::UhIntSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0109usize) as _) }
    }
    #[doc = "RW, USBHS host miscellaneous status register."]
    #[inline(always)]
    pub const fn uh_mis_st(self) -> crate::common::Reg<regs::UhMisSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010ausize) as _) }
    }
    #[doc = "RW, USBHS Host Power Management Data Register."]
    #[inline(always)]
    pub const fn uh_lpm(self) -> crate::common::Reg<regs::UhLpm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "RW, USBHS Host SPLIT Register."]
    #[inline(always)]
    pub const fn uh_split(self) -> crate::common::Reg<regs::UhSplit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "RW, USBHS Host Frame Register."]
    #[inline(always)]
    pub const fn uh_frame(self) -> crate::common::Reg<regs::UhFrame, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "RW, USBHS host sends length register."]
    #[inline(always)]
    pub const fn uh_tx_len(self) -> crate::common::Reg<regs::UhTxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "RW, USBHS host receives length register."]
    #[inline(always)]
    pub const fn uh_rx_len(self) -> crate::common::Reg<regs::UhRxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "RW, USBHS host receives maximum length register."]
    #[inline(always)]
    pub const fn uh_rx_max_len(self) -> crate::common::Reg<regs::UhRxMaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "RW, DMA receiving address register."]
    #[inline(always)]
    pub const fn uh_rx_dma(self) -> crate::common::Reg<regs::UhRxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "RW, DMA sending address register."]
    #[inline(always)]
    pub const fn uh_tx_dma(self) -> crate::common::Reg<regs::UhTxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "RW, USBHS host port control register."]
    #[inline(always)]
    pub const fn port_ctrl(self) -> crate::common::Reg<regs::PortCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "RW, USBHS host port configuration register."]
    #[inline(always)]
    pub const fn port_cfg(self) -> crate::common::Reg<regs::PortCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "RW, USBHS host port interrupt enable register."]
    #[inline(always)]
    pub const fn port_int_en(self) -> crate::common::Reg<regs::PortIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0132usize) as _) }
    }
    #[doc = "RW, USBHS host port test mode register."]
    #[inline(always)]
    pub const fn port_test_ct(self) -> crate::common::Reg<regs::PortTestCt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0133usize) as _) }
    }
    #[doc = "RO, USBHS Host Port Status Register."]
    #[inline(always)]
    pub const fn port_status(self) -> crate::common::Reg<regs::PortStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "RW1Z, USBHS Host Port Status Register."]
    #[inline(always)]
    pub const fn port_status_chg(
        self,
    ) -> crate::common::Reg<regs::PortStatusChg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0136usize) as _) }
    }
    #[doc = "RW, USBHS host BC charging control register."]
    #[inline(always)]
    pub const fn root_bc_ctrl(self) -> crate::common::Reg<regs::RootBcCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "RW, USBHS PLL Clock Control Register."]
    #[inline(always)]
    pub const fn pll_ctrl(self) -> crate::common::Reg<regs::PllCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
}
pub mod regs {
    #[doc = "RW, USBHS PLL Clock Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PllCtrl(pub u8);
    impl PllCtrl {
        #[doc = "Selection of internal reference clock source for USBHS."]
        #[must_use]
        #[inline(always)]
        pub const fn pll_cksel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Selection of internal reference clock source for USBHS."]
        #[inline(always)]
        pub const fn set_pll_cksel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USBHS PLL low-power mode (can only be enabled when USB20 application is at fixed low full speed, cannot be enabled when not at fixed low full speed)."]
        #[must_use]
        #[inline(always)]
        pub const fn pll_lowpow(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USBHS PLL low-power mode (can only be enabled when USB20 application is at fixed low full speed, cannot be enabled when not at fixed low full speed)."]
        #[inline(always)]
        pub const fn set_pll_lowpow(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USBHS PLL Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pll_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USBHS PLL Enable."]
        #[inline(always)]
        pub const fn set_pll_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
    }
    impl Default for PllCtrl {
        #[inline(always)]
        fn default() -> PllCtrl {
            PllCtrl(0)
        }
    }
    impl core::fmt::Debug for PllCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PllCtrl")
                .field("pll_cksel", &self.pll_cksel())
                .field("pll_lowpow", &self.pll_lowpow())
                .field("pll_en", &self.pll_en())
                .finish()
        }
    }
    #[doc = "RO, USBHS host port configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortCfg(pub u8);
    impl PortCfg {
        #[doc = "USB port mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn host_mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB port mode selection."]
        #[inline(always)]
        pub const fn set_host_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Enable 15K resistor pull-down in host mode."]
        #[must_use]
        #[inline(always)]
        pub const fn port_rpd(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable 15K resistor pull-down in host mode."]
        #[inline(always)]
        pub const fn set_port_rpd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for PortCfg {
        #[inline(always)]
        fn default() -> PortCfg {
            PortCfg(0)
        }
    }
    impl core::fmt::Debug for PortCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PortCfg")
                .field("host_mode", &self.host_mode())
                .field("port_rpd", &self.port_rpd())
                .finish()
        }
    }
    #[doc = "RW, USBHS host port control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortCtrl(pub u32);
    impl PortCtrl {
        #[doc = "Port sending reset."]
        #[must_use]
        #[inline(always)]
        pub const fn set_port_reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port sending reset."]
        #[inline(always)]
        pub const fn set_set_port_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "PORT enters suspended state."]
        #[must_use]
        #[inline(always)]
        pub const fn set_port_susp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PORT enters suspended state."]
        #[inline(always)]
        pub const fn set_set_port_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "PORT exits suspended state."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_port_susp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "PORT exits suspended state."]
        #[inline(always)]
        pub const fn set_clr_port_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PORT enters SLEEP state (LPM)."]
        #[must_use]
        #[inline(always)]
        pub const fn set_port_sleep(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PORT enters SLEEP state (LPM)."]
        #[inline(always)]
        pub const fn set_set_port_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PORT exits the enabled state and enters the disabled state."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_port_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PORT exits the enabled state and enters the disabled state."]
        #[inline(always)]
        pub const fn set_clr_port_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Put the port into port state."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_port_connect(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Put the port into port state."]
        #[inline(always)]
        pub const fn set_clr_port_connect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "PORT exits SLEEP state (LPM)."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_port_sleep(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "PORT exits SLEEP state (LPM)."]
        #[inline(always)]
        pub const fn set_clr_port_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Wake up time control."]
        #[must_use]
        #[inline(always)]
        pub const fn port_sleep_besl(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Wake up time control."]
        #[inline(always)]
        pub const fn set_port_sleep_besl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "Bus reset time selection."]
        #[must_use]
        #[inline(always)]
        pub const fn bus_rst_long(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Bus reset time selection."]
        #[inline(always)]
        pub const fn set_bus_rst_long(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for PortCtrl {
        #[inline(always)]
        fn default() -> PortCtrl {
            PortCtrl(0)
        }
    }
    impl core::fmt::Debug for PortCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PortCtrl")
                .field("set_port_reset", &self.set_port_reset())
                .field("set_port_susp", &self.set_port_susp())
                .field("clr_port_susp", &self.clr_port_susp())
                .field("set_port_sleep", &self.set_port_sleep())
                .field("clr_port_en", &self.clr_port_en())
                .field("clr_port_connect", &self.clr_port_connect())
                .field("clr_port_sleep", &self.clr_port_sleep())
                .field("port_sleep_besl", &self.port_sleep_besl())
                .field("bus_rst_long", &self.bus_rst_long())
                .finish()
        }
    }
    #[doc = "RO, USBHS host port configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortIntEn(pub u8);
    impl PortIntEn {
        #[doc = "Port connection status change interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn port_connect_ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port connection status change interrupt enable."]
        #[inline(always)]
        pub const fn set_port_connect_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Port enable status change interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn port_en_ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port enable status change interrupt enable."]
        #[inline(always)]
        pub const fn set_port_en_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Port pause status change interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn port_susp_ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port pause status change interrupt enable."]
        #[inline(always)]
        pub const fn set_port_susp_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Port reset status change interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn port_reset_ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port reset status change interrupt enable."]
        #[inline(always)]
        pub const fn set_port_reset_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Port sleep state change interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn port_slp_ie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port sleep state change interrupt enable."]
        #[inline(always)]
        pub const fn set_port_slp_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for PortIntEn {
        #[inline(always)]
        fn default() -> PortIntEn {
            PortIntEn(0)
        }
    }
    impl core::fmt::Debug for PortIntEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PortIntEn")
                .field("port_connect_ie", &self.port_connect_ie())
                .field("port_en_ie", &self.port_en_ie())
                .field("port_susp_ie", &self.port_susp_ie())
                .field("port_reset_ie", &self.port_reset_ie())
                .field("port_slp_ie", &self.port_slp_ie())
                .finish()
        }
    }
    #[doc = "RO, USBHS Host Port Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortStatus(pub u16);
    impl PortStatus {
        #[doc = "Port connection status."]
        #[must_use]
        #[inline(always)]
        pub const fn port_c0nnect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port connection status."]
        #[inline(always)]
        pub const fn set_port_c0nnect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "Port Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn port_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port Enable."]
        #[inline(always)]
        pub const fn set_port_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "Port paused state."]
        #[must_use]
        #[inline(always)]
        pub const fn port_susp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port paused state."]
        #[inline(always)]
        pub const fn set_port_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "Port reset status."]
        #[must_use]
        #[inline(always)]
        pub const fn port_restting(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port reset status."]
        #[inline(always)]
        pub const fn set_port_restting(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "Port sleep."]
        #[must_use]
        #[inline(always)]
        pub const fn port_slp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port sleep."]
        #[inline(always)]
        pub const fn set_port_slp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "Is the port connection speed low."]
        #[must_use]
        #[inline(always)]
        pub const fn port_low_spd(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Is the port connection speed low."]
        #[inline(always)]
        pub const fn set_port_low_spd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "Is the port connection speed high."]
        #[must_use]
        #[inline(always)]
        pub const fn port_high_spd(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Is the port connection speed high."]
        #[inline(always)]
        pub const fn set_port_high_spd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "Is the port in test mode."]
        #[must_use]
        #[inline(always)]
        pub const fn port_test(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Is the port in test mode."]
        #[inline(always)]
        pub const fn set_port_test(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
    }
    impl Default for PortStatus {
        #[inline(always)]
        fn default() -> PortStatus {
            PortStatus(0)
        }
    }
    impl core::fmt::Debug for PortStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PortStatus")
                .field("port_c0nnect", &self.port_c0nnect())
                .field("port_en", &self.port_en())
                .field("port_susp", &self.port_susp())
                .field("port_restting", &self.port_restting())
                .field("port_slp", &self.port_slp())
                .field("port_low_spd", &self.port_low_spd())
                .field("port_high_spd", &self.port_high_spd())
                .field("port_test", &self.port_test())
                .finish()
        }
    }
    #[doc = "RW1Z, USBHS Host Port Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortStatusChg(pub u8);
    impl PortStatusChg {
        #[doc = "Port connection status changes, write 1 to reset to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn port_connect_if(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port connection status changes, write 1 to reset to zero."]
        #[inline(always)]
        pub const fn set_port_connect_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Port enable status changes, write 1 to reset to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn port_en_if(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port enable status changes, write 1 to reset to zero."]
        #[inline(always)]
        pub const fn set_port_en_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Port pause status changes, write 1 to reset to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn port_susp_if(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port pause status changes, write 1 to reset to zero."]
        #[inline(always)]
        pub const fn set_port_susp_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Port reset status changes, write 1 to reset to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn port_reset_if(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port reset status changes, write 1 to reset to zero."]
        #[inline(always)]
        pub const fn set_port_reset_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Port sleep status changes, write 1 to reset to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn port_slp_if(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port sleep status changes, write 1 to reset to zero."]
        #[inline(always)]
        pub const fn set_port_slp_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for PortStatusChg {
        #[inline(always)]
        fn default() -> PortStatusChg {
            PortStatusChg(0)
        }
    }
    impl core::fmt::Debug for PortStatusChg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PortStatusChg")
                .field("port_connect_if", &self.port_connect_if())
                .field("port_en_if", &self.port_en_if())
                .field("port_susp_if", &self.port_susp_if())
                .field("port_reset_if", &self.port_reset_if())
                .field("port_slp_if", &self.port_slp_if())
                .finish()
        }
    }
    #[doc = "RW, USBHS host port test mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortTestCt(pub u8);
    impl PortTestCt {
        #[doc = "Test output J."]
        #[must_use]
        #[inline(always)]
        pub const fn test_j(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Test output J."]
        #[inline(always)]
        pub const fn set_test_j(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Test output K."]
        #[must_use]
        #[inline(always)]
        pub const fn test_k(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Test output K."]
        #[inline(always)]
        pub const fn set_test_k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Enable Test Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn test_force_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Test Mode."]
        #[inline(always)]
        pub const fn set_test_force_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
    }
    impl Default for PortTestCt {
        #[inline(always)]
        fn default() -> PortTestCt {
            PortTestCt(0)
        }
    }
    impl core::fmt::Debug for PortTestCt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PortTestCt")
                .field("test_j", &self.test_j())
                .field("test_k", &self.test_k())
                .field("test_force_en", &self.test_force_en())
                .finish()
        }
    }
    #[doc = "RW, USBHS host BC charging control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RootBcCtrl(pub u32);
    impl RootBcCtrl {
        #[doc = "UDP pin BC protocol comparator status."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_bc_cmpo(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "UDP pin BC protocol comparator status."]
        #[inline(always)]
        pub const fn set_udp_bc_cmpo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "UDM pin BC protocol comparator status."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_bc_cmpo(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "UDM pin BC protocol comparator status."]
        #[inline(always)]
        pub const fn set_udm_bc_cmpo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "UDP pin BC protocol comparator enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_bc_cmpe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "UDP pin BC protocol comparator enabled."]
        #[inline(always)]
        pub const fn set_udp_bc_cmpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "UDM pin BC protocol comparator enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_bc_cmpe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "UDM pin BC protocol comparator enabled."]
        #[inline(always)]
        pub const fn set_udm_bc_cmpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Automatic mode enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn bc_auto_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic mode enabled."]
        #[inline(always)]
        pub const fn set_bc_auto_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "UDP pin BC protocol source voltage enable."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_bc_vsrc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "UDP pin BC protocol source voltage enable."]
        #[inline(always)]
        pub const fn set_udp_bc_vsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "UDM pin BC protocol source voltage enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_bc_vsrc(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "UDM pin BC protocol source voltage enabled."]
        #[inline(always)]
        pub const fn set_udm_bc_vsrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "In automatic mode, this bit represents the UDM output VBC_SRC, otherwise it is controlled by UDM-BC_CMPE."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_vsrc_act(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "In automatic mode, this bit represents the UDM output VBC_SRC, otherwise it is controlled by UDM-BC_CMPE."]
        #[inline(always)]
        pub const fn set_udm_vsrc_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for RootBcCtrl {
        #[inline(always)]
        fn default() -> RootBcCtrl {
            RootBcCtrl(0)
        }
    }
    impl core::fmt::Debug for RootBcCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RootBcCtrl")
                .field("udp_bc_cmpo", &self.udp_bc_cmpo())
                .field("udm_bc_cmpo", &self.udm_bc_cmpo())
                .field("udp_bc_cmpe", &self.udp_bc_cmpe())
                .field("udm_bc_cmpe", &self.udm_bc_cmpe())
                .field("bc_auto_mode", &self.bc_auto_mode())
                .field("udp_bc_vsrc", &self.udp_bc_vsrc())
                .field("udm_bc_vsrc", &self.udm_bc_vsrc())
                .field("udm_vsrc_act", &self.udm_vsrc_act())
                .finish()
        }
    }
    #[doc = "RW, Starting address register of endpoint 0 buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0Dma(pub u32);
    impl Uep0Dma {
        #[doc = "Endpoint 0 buffer address, aligned to 4 bytes."]
        #[must_use]
        #[inline(always)]
        pub const fn address(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "Endpoint 0 buffer address, aligned to 4 bytes."]
        #[inline(always)]
        pub const fn set_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for Uep0Dma {
        #[inline(always)]
        fn default() -> Uep0Dma {
            Uep0Dma(0)
        }
    }
    impl core::fmt::Debug for Uep0Dma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep0Dma")
                .field("address", &self.address())
                .finish()
        }
    }
    #[doc = "RW, Endpoint 0 maximum packet length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0MaxLen(pub u32);
    impl Uep0MaxLen {
        #[doc = "Maximum packet length in bytes."]
        #[must_use]
        #[inline(always)]
        pub const fn max_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Maximum packet length in bytes."]
        #[inline(always)]
        pub const fn set_max_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
    }
    impl Default for Uep0MaxLen {
        #[inline(always)]
        fn default() -> Uep0MaxLen {
            Uep0MaxLen(0)
        }
    }
    impl core::fmt::Debug for Uep0MaxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep0MaxLen")
                .field("max_len", &self.max_len())
                .finish()
        }
    }
    #[doc = "RW, Endpoint 0 receives control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0RxCtrl(pub u8);
    impl Uep0RxCtrl {
        #[doc = "Endpoint 0 controls the received response."]
        #[must_use]
        #[inline(always)]
        pub const fn res_mask10(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Endpoint 0 controls the received response."]
        #[inline(always)]
        pub const fn set_res_mask10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "Expected synchronization trigger bit for receiving endpoint 0."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tog_mask1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Expected synchronization trigger bit for receiving endpoint 0."]
        #[inline(always)]
        pub const fn set_uep_tog_mask1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Is the transaction received by endpoint 0 a SETUP transaction."]
        #[must_use]
        #[inline(always)]
        pub const fn setup_is(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Is the transaction received by endpoint 0 a SETUP transaction."]
        #[inline(always)]
        pub const fn set_setup_is(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "The received synchronization trigger bit matches the expected synchronization trigger bit."]
        #[must_use]
        #[inline(always)]
        pub const fn endp_r_tog_match(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "The received synchronization trigger bit matches the expected synchronization trigger bit."]
        #[inline(always)]
        pub const fn set_endp_r_tog_match(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Endpoint 0 returns NAK for received data packet type."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_nak_tog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint 0 returns NAK for received data packet type."]
        #[inline(always)]
        pub const fn set_uep_r_nak_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Endpoint 0 receives NAK end flag, write 0 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint 0 receives NAK end flag, write 0 to clear."]
        #[inline(always)]
        pub const fn set_uep_r_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint 0 receives end flag, write 0 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn endp_r_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint 0 receives end flag, write 0 to clear."]
        #[inline(always)]
        pub const fn set_endp_r_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep0RxCtrl {
        #[inline(always)]
        fn default() -> Uep0RxCtrl {
            Uep0RxCtrl(0)
        }
    }
    impl core::fmt::Debug for Uep0RxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep0RxCtrl")
                .field("res_mask10", &self.res_mask10())
                .field("uep_tog_mask1", &self.uep_tog_mask1())
                .field("setup_is", &self.setup_is())
                .field("endp_r_tog_match", &self.endp_r_tog_match())
                .field("uep_r_nak_tog", &self.uep_r_nak_tog())
                .field("uep_r_nak_act", &self.uep_r_nak_act())
                .field("endp_r_done", &self.endp_r_done())
                .finish()
        }
    }
    #[doc = "RW, Endpoint 0 received length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0RxLen(pub u16);
    impl Uep0RxLen {
        #[doc = "Number of bytes received on endpoint 0."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of bytes received on endpoint 0."]
        #[inline(always)]
        pub const fn set_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep0RxLen {
        #[inline(always)]
        fn default() -> Uep0RxLen {
            Uep0RxLen(0)
        }
    }
    impl core::fmt::Debug for Uep0RxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep0RxLen")
                .field("rx_len", &self.rx_len())
                .finish()
        }
    }
    #[doc = "RW, Endpoint 0 sends length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0TLen(pub u16);
    impl Uep0TLen {
        #[doc = "Endpoint 0 sends data length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep0_t_len60(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Endpoint 0 sends data length."]
        #[inline(always)]
        pub const fn set_uep0_t_len60(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
        }
    }
    impl Default for Uep0TLen {
        #[inline(always)]
        fn default() -> Uep0TLen {
            Uep0TLen(0)
        }
    }
    impl core::fmt::Debug for Uep0TLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep0TLen")
                .field("uep0_t_len60", &self.uep0_t_len60())
                .finish()
        }
    }
    #[doc = "RW, Endpoint 0 sends control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0TxCtrl(pub u8);
    impl Uep0TxCtrl {
        #[doc = "Endpoint 0 controls the response sent."]
        #[must_use]
        #[inline(always)]
        pub const fn res_mask10(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Endpoint 0 controls the response sent."]
        #[inline(always)]
        pub const fn set_res_mask10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "Expected synchronization trigger bit for endpoint 0."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tog_mask10(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Expected synchronization trigger bit for endpoint 0."]
        #[inline(always)]
        pub const fn set_uep_tog_mask10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "Endpoint 0 sends NAK end flag, write 0 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint 0 sends NAK end flag, write 0 to clear."]
        #[inline(always)]
        pub const fn set_uep_t_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint 0 sends end flag, write 0 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn endp_t_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint 0 sends end flag, write 0 to clear."]
        #[inline(always)]
        pub const fn set_endp_t_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep0TxCtrl {
        #[inline(always)]
        fn default() -> Uep0TxCtrl {
            Uep0TxCtrl(0)
        }
    }
    impl core::fmt::Debug for Uep0TxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep0TxCtrl")
                .field("res_mask10", &self.res_mask10())
                .field("uep_tog_mask10", &self.uep_tog_mask10())
                .field("uep_t_nak_act", &self.uep_t_nak_act())
                .field("endp_t_done", &self.endp_t_done())
                .finish()
        }
    }
    #[doc = "RW, USBHS Endpoint Reuse Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepAfMode(pub u32);
    impl UepAfMode {
        #[doc = "1-7 endpoint reuse enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_af60(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "1-7 endpoint reuse enabled."]
        #[inline(always)]
        pub const fn set_uep_t_af60(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
        }
    }
    impl Default for UepAfMode {
        #[inline(always)]
        fn default() -> UepAfMode {
            UepAfMode(0)
        }
    }
    impl core::fmt::Debug for UepAfMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepAfMode")
                .field("uep_t_af60", &self.uep_t_af60())
                .finish()
        }
    }
    #[doc = "RW, Endpoint n maximum packet length (n=1~7)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepMaxLen(pub u32);
    impl UepMaxLen {
        #[doc = "Maximum packet length in bytes."]
        #[must_use]
        #[inline(always)]
        pub const fn max_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Maximum packet length in bytes."]
        #[inline(always)]
        pub const fn set_max_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for UepMaxLen {
        #[inline(always)]
        fn default() -> UepMaxLen {
            UepMaxLen(0)
        }
    }
    impl core::fmt::Debug for UepMaxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepMaxLen")
                .field("max_len", &self.max_len())
                .finish()
        }
    }
    #[doc = "RW, 0~7 endpoints burst receive enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRBurst(pub u8);
    impl UepRBurst {
        #[doc = "0~7 endpoints burst receive enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_burst_en70(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0~7 endpoints burst receive enable."]
        #[inline(always)]
        pub const fn set_uep_r_burst_en70(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for UepRBurst {
        #[inline(always)]
        fn default() -> UepRBurst {
            UepRBurst(0)
        }
    }
    impl core::fmt::Debug for UepRBurst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRBurst")
                .field("uep_r_burst_en70", &self.uep_r_burst_en70())
                .finish()
        }
    }
    #[doc = "RW, USBHS endpoint receiving synchronization mode enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRIso(pub u16);
    impl UepRIso {
        #[doc = "Enable the synchronization mode of the downstream endpoint (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_iso60(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "Enable the synchronization mode of the downstream endpoint (OUT)."]
        #[inline(always)]
        pub const fn set_uep_r_iso60(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u16) & 0x7f) << 1usize);
        }
    }
    impl Default for UepRIso {
        #[inline(always)]
        fn default() -> UepRIso {
            UepRIso(0)
        }
    }
    impl core::fmt::Debug for UepRIso {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRIso")
                .field("uep_r_iso60", &self.uep_r_iso60())
                .finish()
        }
    }
    #[doc = "RW, USBHS Endpoint Receive Mode Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRResMode(pub u8);
    impl UepRResMode {
        #[doc = "0~7 endpoints burst receive enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_res_mode70(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0~7 endpoints burst receive enable."]
        #[inline(always)]
        pub const fn set_uep_r_res_mode70(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for UepRResMode {
        #[inline(always)]
        fn default() -> UepRResMode {
            UepRResMode(0)
        }
    }
    impl core::fmt::Debug for UepRResMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRResMode")
                .field("uep_r_res_mode70", &self.uep_r_res_mode70())
                .finish()
        }
    }
    #[doc = "RW, USBHS endpoint receives automatic flip enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRTogAuto(pub u16);
    impl UepRTogAuto {
        #[doc = "0~7 endpoint synchronization trigger bit automatic flip enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog_auto70(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0~7 endpoint synchronization trigger bit automatic flip enable."]
        #[inline(always)]
        pub const fn set_uep_r_tog_auto70(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
    }
    impl Default for UepRTogAuto {
        #[inline(always)]
        fn default() -> UepRTogAuto {
            UepRTogAuto(0)
        }
    }
    impl core::fmt::Debug for UepRTogAuto {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRTogAuto")
                .field("uep_r_tog_auto70", &self.uep_r_tog_auto70())
                .finish()
        }
    }
    #[doc = "RW, Endpoint n receives control registers(n=1~7)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxCtrl(pub u8);
    impl UepRxCtrl {
        #[doc = "Endpoint n controls the received response(n=1~7)."]
        #[must_use]
        #[inline(always)]
        pub const fn res_mask10(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Endpoint n controls the received response(n=1~7)."]
        #[inline(always)]
        pub const fn set_res_mask10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "Expected synchronization trigger bit for receiving endpoint n(n=1~7)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tog_mask10(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Expected synchronization trigger bit for receiving endpoint n(n=1~7)."]
        #[inline(always)]
        pub const fn set_uep_tog_mask10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "The synchronization trigger bit received by endpoint n matches the expected synchronization trigger bit(n=1~7)."]
        #[must_use]
        #[inline(always)]
        pub const fn endp_r_tog_match(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "The synchronization trigger bit received by endpoint n matches the expected synchronization trigger bit(n=1~7)."]
        #[inline(always)]
        pub const fn set_endp_r_tog_match(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Endpoint n receives the return NAK and packet type(n=1~7)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_nak_tog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint n receives the return NAK and packet type(n=1~7)."]
        #[inline(always)]
        pub const fn set_uep_r_nak_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Endpoint n receives NAK end flag, writes 0 to clear(n=1~7)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint n receives NAK end flag, writes 0 to clear(n=1~7)."]
        #[inline(always)]
        pub const fn set_uep_r_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint n receives the end flag, write 0 to clear(n=1~7)."]
        #[must_use]
        #[inline(always)]
        pub const fn endp_r_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint n receives the end flag, write 0 to clear(n=1~7)."]
        #[inline(always)]
        pub const fn set_endp_r_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UepRxCtrl {
        #[inline(always)]
        fn default() -> UepRxCtrl {
            UepRxCtrl(0)
        }
    }
    impl core::fmt::Debug for UepRxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRxCtrl")
                .field("res_mask10", &self.res_mask10())
                .field("uep_tog_mask10", &self.uep_tog_mask10())
                .field("endp_r_tog_match", &self.endp_r_tog_match())
                .field("uep_r_nak_tog", &self.uep_r_nak_tog())
                .field("uep_r_nak_act", &self.uep_r_nak_act())
                .field("endp_r_done", &self.endp_r_done())
                .finish()
        }
    }
    #[doc = "RW, Endpoint n receive buffer start address (n=1~7)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxDma(pub u32);
    impl UepRxDma {
        #[doc = "Receive buffer address, aligned to 4 bytes."]
        #[must_use]
        #[inline(always)]
        pub const fn address(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "Receive buffer address, aligned to 4 bytes."]
        #[inline(always)]
        pub const fn set_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for UepRxDma {
        #[inline(always)]
        fn default() -> UepRxDma {
            UepRxDma(0)
        }
    }
    impl core::fmt::Debug for UepRxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRxDma")
                .field("address", &self.address())
                .finish()
        }
    }
    #[doc = "RW, USBHS endpoint receive enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxEn(pub u16);
    impl UepRxEn {
        #[doc = "0~15 endpoints receive enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rx_en150(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "0~15 endpoints receive enable."]
        #[inline(always)]
        pub const fn set_uep_rx_en150(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for UepRxEn {
        #[inline(always)]
        fn default() -> UepRxEn {
            UepRxEn(0)
        }
    }
    impl core::fmt::Debug for UepRxEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRxEn")
                .field("uep_rx_en150", &self.uep_rx_en150())
                .finish()
        }
    }
    #[doc = "RW, Endpoint n received length (n=1~7)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxLen(pub u16);
    impl UepRxLen {
        #[doc = "Number of bytes received on endpoint n."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of bytes received on endpoint n."]
        #[inline(always)]
        pub const fn set_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for UepRxLen {
        #[inline(always)]
        fn default() -> UepRxLen {
            UepRxLen(0)
        }
    }
    impl core::fmt::Debug for UepRxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRxLen")
                .field("rx_len", &self.rx_len())
                .finish()
        }
    }
    #[doc = "RW, 0~7 endpoints burst send enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTBurst(pub u8);
    impl UepTBurst {
        #[doc = "0~7 endpoints burst send enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_burst_en70(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0~7 endpoints burst send enable."]
        #[inline(always)]
        pub const fn set_uep_t_burst_en70(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for UepTBurst {
        #[inline(always)]
        fn default() -> UepTBurst {
            UepTBurst(0)
        }
    }
    impl core::fmt::Debug for UepTBurst {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTBurst")
                .field("uep_t_burst_en70", &self.uep_t_burst_en70())
                .finish()
        }
    }
    #[doc = "RW, USBHS Endpoint Send Mode Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTBurstMode(pub u8);
    impl UepTBurstMode {
        #[doc = "0~7 endpoints burst send enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_burst_en70(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0~7 endpoints burst send enable."]
        #[inline(always)]
        pub const fn set_uep_t_burst_en70(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for UepTBurstMode {
        #[inline(always)]
        fn default() -> UepTBurstMode {
            UepTBurstMode(0)
        }
    }
    impl core::fmt::Debug for UepTBurstMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTBurstMode")
                .field("uep_t_burst_en70", &self.uep_t_burst_en70())
                .finish()
        }
    }
    #[doc = "RW, USBHS endpoint sends synchronization mode enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTIso(pub u16);
    impl UepTIso {
        #[doc = "Upload endpoint (IN) synchronization mode enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_iso60(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "Upload endpoint (IN) synchronization mode enabled."]
        #[inline(always)]
        pub const fn set_uep_t_iso60(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u16) & 0x7f) << 1usize);
        }
    }
    impl Default for UepTIso {
        #[inline(always)]
        fn default() -> UepTIso {
            UepTIso(0)
        }
    }
    impl core::fmt::Debug for UepTIso {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTIso")
                .field("uep_t_iso60", &self.uep_t_iso60())
                .finish()
        }
    }
    #[doc = "RW, Endpoint n transmit length (n=1~7)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTLen(pub u16);
    impl UepTLen {
        #[doc = "Number of bytes to transmit on endpoint n."]
        #[must_use]
        #[inline(always)]
        pub const fn t_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Number of bytes to transmit on endpoint n."]
        #[inline(always)]
        pub const fn set_t_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for UepTLen {
        #[inline(always)]
        fn default() -> UepTLen {
            UepTLen(0)
        }
    }
    impl core::fmt::Debug for UepTLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTLen")
                .field("t_len", &self.t_len())
                .finish()
        }
    }
    #[doc = "RW, USBHS endpoint receives automatic flip enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTTogAuto(pub u16);
    impl UepTTogAuto {
        #[doc = "0~7 endpoint synchronization trigger bit automatic flip enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog_auto70(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0~7 endpoint synchronization trigger bit automatic flip enable."]
        #[inline(always)]
        pub const fn set_uep_t_tog_auto70(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
        }
    }
    impl Default for UepTTogAuto {
        #[inline(always)]
        fn default() -> UepTTogAuto {
            UepTTogAuto(0)
        }
    }
    impl core::fmt::Debug for UepTTogAuto {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTTogAuto")
                .field("uep_t_tog_auto70", &self.uep_t_tog_auto70())
                .finish()
        }
    }
    #[doc = "RW, Endpoint n sends data length(n=1~7)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxCtrl(pub u8);
    impl UepTxCtrl {
        #[doc = "Endpoint n controls the response sent(n=1~7)."]
        #[must_use]
        #[inline(always)]
        pub const fn res_mask10(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Endpoint n controls the response sent(n=1~7)."]
        #[inline(always)]
        pub const fn set_res_mask10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "Expected synchronization trigger bit for endpoint n's sending(n=1~7)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tog_mask10(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Expected synchronization trigger bit for endpoint n's sending(n=1~7)."]
        #[inline(always)]
        pub const fn set_uep_tog_mask10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "Endpoint n sends NAK end flag, write 0 to clear(n=1~7)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint n sends NAK end flag, write 0 to clear(n=1~7)."]
        #[inline(always)]
        pub const fn set_uep_t_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Endpoint n sends an end flag, write 0 to clear(n=1~7)."]
        #[must_use]
        #[inline(always)]
        pub const fn endp_t_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint n sends an end flag, write 0 to clear(n=1~7)."]
        #[inline(always)]
        pub const fn set_endp_t_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UepTxCtrl {
        #[inline(always)]
        fn default() -> UepTxCtrl {
            UepTxCtrl(0)
        }
    }
    impl core::fmt::Debug for UepTxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTxCtrl")
                .field("res_mask10", &self.res_mask10())
                .field("uep_tog_mask10", &self.uep_tog_mask10())
                .field("uep_t_nak_act", &self.uep_t_nak_act())
                .field("endp_t_done", &self.endp_t_done())
                .finish()
        }
    }
    #[doc = "RW, Endpoint n transmit buffer start address (n=1~7)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxDma(pub u32);
    impl UepTxDma {
        #[doc = "Transmit buffer address, aligned to 4 bytes."]
        #[must_use]
        #[inline(always)]
        pub const fn address(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "Transmit buffer address, aligned to 4 bytes."]
        #[inline(always)]
        pub const fn set_address(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for UepTxDma {
        #[inline(always)]
        fn default() -> UepTxDma {
            UepTxDma(0)
        }
    }
    impl core::fmt::Debug for UepTxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTxDma")
                .field("address", &self.address())
                .finish()
        }
    }
    #[doc = "RW, USBHS endpoint sends enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxEn(pub u16);
    impl UepTxEn {
        #[doc = "0~15 endpoints send enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tx_en150(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "0~15 endpoints send enable."]
        #[inline(always)]
        pub const fn set_uep_tx_en150(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for UepTxEn {
        #[inline(always)]
        fn default() -> UepTxEn {
            UepTxEn(0)
        }
    }
    impl core::fmt::Debug for UepTxEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepTxEn")
                .field("uep_tx_en150", &self.uep_tx_en150())
                .finish()
        }
    }
    #[doc = "RW, USBHS Host Configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhCfg(pub u8);
    impl UhCfg {
        #[doc = "USB connection control module reset."]
        #[must_use]
        #[inline(always)]
        pub const fn root_link_reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB connection control module reset."]
        #[inline(always)]
        pub const fn set_root_link_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB protocol processor reset, software reset required."]
        #[must_use]
        #[inline(always)]
        pub const fn root_sie_reset(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB protocol processor reset, software reset required."]
        #[inline(always)]
        pub const fn set_root_sie_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Clear interrupt flag and FIFO, software zeroing is required."]
        #[must_use]
        #[inline(always)]
        pub const fn root_all_clr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear interrupt flag and FIFO, software zeroing is required."]
        #[inline(always)]
        pub const fn set_root_all_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "PHY hangs, turn off the UTMI clock of the controller."]
        #[must_use]
        #[inline(always)]
        pub const fn root_phy_suspendm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PHY hangs, turn off the UTMI clock of the controller."]
        #[inline(always)]
        pub const fn set_root_phy_suspendm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Enable DMA transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn root_dma_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable DMA transmission."]
        #[inline(always)]
        pub const fn set_root_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Enable SOF packet transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn root_sof_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable SOF packet transmission."]
        #[inline(always)]
        pub const fn set_root_sof_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Force full speed FS."]
        #[must_use]
        #[inline(always)]
        pub const fn root_force_fs(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Force full speed FS."]
        #[inline(always)]
        pub const fn set_root_force_fs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "Enable LPM."]
        #[must_use]
        #[inline(always)]
        pub const fn root_lpm_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable LPM."]
        #[inline(always)]
        pub const fn set_root_lpm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhCfg {
        #[inline(always)]
        fn default() -> UhCfg {
            UhCfg(0)
        }
    }
    impl core::fmt::Debug for UhCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhCfg")
                .field("root_link_reset", &self.root_link_reset())
                .field("root_sie_reset", &self.root_sie_reset())
                .field("root_all_clr", &self.root_all_clr())
                .field("root_phy_suspendm", &self.root_phy_suspendm())
                .field("root_dma_en", &self.root_dma_en())
                .field("root_sof_en", &self.root_sof_en())
                .field("root_force_fs", &self.root_force_fs())
                .field("root_lpm_en", &self.root_lpm_en())
                .finish()
        }
    }
    #[doc = "RW, USBHS Host Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhControl(pub u32);
    impl UhControl {
        #[doc = "Transaction token package PID."]
        #[must_use]
        #[inline(always)]
        pub const fn pid_mask30(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Transaction token package PID."]
        #[inline(always)]
        pub const fn set_pid_mask30(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Transaction token package endpoint number."]
        #[must_use]
        #[inline(always)]
        pub const fn endp_mask30(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Transaction token package endpoint number."]
        #[inline(always)]
        pub const fn set_endp_mask30(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Sent data PID."]
        #[must_use]
        #[inline(always)]
        pub const fn tog_mask10(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "Sent data PID."]
        #[inline(always)]
        pub const fn set_tog_mask10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "Data cache control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn buf_mode(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Data cache control bit."]
        #[inline(always)]
        pub const fn set_buf_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "HOST enables transaction execution, and this bit is automatically reset to zero after the transaction is completed."]
        #[must_use]
        #[inline(always)]
        pub const fn host_action(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "HOST enables transaction execution, and this bit is automatically reset to zero after the transaction is completed."]
        #[inline(always)]
        pub const fn set_host_action(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Sending LPM package is valid."]
        #[must_use]
        #[inline(always)]
        pub const fn lpm_valid(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Sending LPM package is valid."]
        #[inline(always)]
        pub const fn set_lpm_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Sending SPLIT package is valid."]
        #[must_use]
        #[inline(always)]
        pub const fn split_valid(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Sending SPLIT package is valid."]
        #[inline(always)]
        pub const fn set_split_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "When the port is operating at full speed and needs to send low-speed packets (PRE), this bit must be enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn down_lowspd(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "When the port is operating at full speed and needs to send low-speed packets (PRE), this bit must be enabled."]
        #[inline(always)]
        pub const fn set_down_lowspd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "There is no data packet after the OUT/SETUP token packet, used for high-speed SPLIT packets."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_no_data(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "There is no data packet after the OUT/SETUP token packet, used for high-speed SPLIT packets."]
        #[inline(always)]
        pub const fn set_tx_no_data(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Expected data packet after IN token packet, used for high-speed SPLIT packet."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_no_data(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Expected data packet after IN token packet, used for high-speed SPLIT packet."]
        #[inline(always)]
        pub const fn set_rx_no_data(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OUT/SETUP - DATA does not expect a response, used for synchronous transmission or high-speed SPLIT packets."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_no_res(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OUT/SETUP - DATA does not expect a response, used for synchronous transmission or high-speed SPLIT packets."]
        #[inline(always)]
        pub const fn set_tx_no_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "IN-DATA no response, used for synchronous transmission or high-speed SPLIT packets."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_no_res(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "IN-DATA no response, used for synchronous transmission or high-speed SPLIT packets."]
        #[inline(always)]
        pub const fn set_rx_no_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for UhControl {
        #[inline(always)]
        fn default() -> UhControl {
            UhControl(0)
        }
    }
    impl core::fmt::Debug for UhControl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhControl")
                .field("pid_mask30", &self.pid_mask30())
                .field("endp_mask30", &self.endp_mask30())
                .field("tog_mask10", &self.tog_mask10())
                .field("buf_mode", &self.buf_mode())
                .field("host_action", &self.host_action())
                .field("lpm_valid", &self.lpm_valid())
                .field("split_valid", &self.split_valid())
                .field("down_lowspd", &self.down_lowspd())
                .field("tx_no_data", &self.tx_no_data())
                .field("rx_no_data", &self.rx_no_data())
                .field("tx_no_res", &self.tx_no_res())
                .field("rx_no_res", &self.rx_no_res())
                .finish()
        }
    }
    #[doc = "RW, USBHS host device address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhDevAddr(pub u8);
    impl UhDevAddr {
        #[doc = "Device Address."]
        #[must_use]
        #[inline(always)]
        pub const fn dev_addr70(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Device Address."]
        #[inline(always)]
        pub const fn set_dev_addr70(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for UhDevAddr {
        #[inline(always)]
        fn default() -> UhDevAddr {
            UhDevAddr(0)
        }
    }
    impl core::fmt::Debug for UhDevAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhDevAddr")
                .field("dev_addr70", &self.dev_addr70())
                .finish()
        }
    }
    #[doc = "RW, USBHS Host Frame Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhFrame(pub u32);
    impl UhFrame {
        #[doc = "frame number."]
        #[must_use]
        #[inline(always)]
        pub const fn frame_no(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "frame number."]
        #[inline(always)]
        pub const fn set_frame_no(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Micro frame number."]
        #[must_use]
        #[inline(always)]
        pub const fn micro_frame_hs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Micro frame number."]
        #[inline(always)]
        pub const fn set_micro_frame_hs(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "SOF Count Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sof_cnt_en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SOF Count Enable."]
        #[inline(always)]
        pub const fn set_sof_cnt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "SOF count reset to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn sof_cnt_clr(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "SOF count reset to zero."]
        #[inline(always)]
        pub const fn set_sof_cnt_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for UhFrame {
        #[inline(always)]
        fn default() -> UhFrame {
            UhFrame(0)
        }
    }
    impl core::fmt::Debug for UhFrame {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhFrame")
                .field("frame_no", &self.frame_no())
                .field("micro_frame_hs", &self.micro_frame_hs())
                .field("sof_cnt_en", &self.sof_cnt_en())
                .field("sof_cnt_clr", &self.sof_cnt_clr())
                .finish()
        }
    }
    #[doc = "RW, USBHS host interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhIntEn(pub u8);
    impl UhIntEn {
        #[doc = "Wake up interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wkup_act_ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wake up interrupt enable."]
        #[inline(always)]
        pub const fn set_wkup_act_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Bus recovery interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn resume_act_ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Bus recovery interrupt enable."]
        #[inline(always)]
        pub const fn set_resume_act_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB end or transfer completion interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn act_ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB end or transfer completion interrupt enable."]
        #[inline(always)]
        pub const fn set_act_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Enable SOF packet sending interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn sof_act_ie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Enable SOF packet sending interrupt."]
        #[inline(always)]
        pub const fn set_sof_act_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Enable sending pause interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_halt_ie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable sending pause interrupt."]
        #[inline(always)]
        pub const fn set_tx_halt_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "FIFO overflow interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_ov_ie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overflow interrupt enable."]
        #[inline(always)]
        pub const fn set_fifo_ov_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhIntEn {
        #[inline(always)]
        fn default() -> UhIntEn {
            UhIntEn(0)
        }
    }
    impl core::fmt::Debug for UhIntEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhIntEn")
                .field("wkup_act_ie", &self.wkup_act_ie())
                .field("resume_act_ie", &self.resume_act_ie())
                .field("act_ie", &self.act_ie())
                .field("sof_act_ie", &self.sof_act_ie())
                .field("tx_halt_ie", &self.tx_halt_ie())
                .field("fifo_ov_ie", &self.fifo_ov_ie())
                .finish()
        }
    }
    #[doc = "RW1Z, USBHS host interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhIntFlag(pub u8);
    impl UhIntFlag {
        #[doc = "Wake up interrupt flag, write 1 to reset to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn wkup_if(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wake up interrupt flag, write 1 to reset to zero."]
        #[inline(always)]
        pub const fn set_wkup_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Bus recovery interrupt flag, write 1 to reset to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn resume_act_if(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Bus recovery interrupt flag, write 1 to reset to zero."]
        #[inline(always)]
        pub const fn set_resume_act_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB transaction transfer completion interrupt flag, write 1 to reset to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn act_if(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB transaction transfer completion interrupt flag, write 1 to reset to zero."]
        #[inline(always)]
        pub const fn set_act_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "SOF packet sending completion interrupt flag, write 1 to reset to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn sof_act_if(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SOF packet sending completion interrupt flag, write 1 to reset to zero."]
        #[inline(always)]
        pub const fn set_sof_act_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Send abort interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_halt_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Send abort interrupt flag."]
        #[inline(always)]
        pub const fn set_tx_halt_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "FIFO overflow interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_ov_if(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overflow interrupt flag."]
        #[inline(always)]
        pub const fn set_fifo_ov_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhIntFlag {
        #[inline(always)]
        fn default() -> UhIntFlag {
            UhIntFlag(0)
        }
    }
    impl core::fmt::Debug for UhIntFlag {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhIntFlag")
                .field("wkup_if", &self.wkup_if())
                .field("resume_act_if", &self.resume_act_if())
                .field("act_if", &self.act_if())
                .field("sof_act_if", &self.sof_act_if())
                .field("tx_halt_if", &self.tx_halt_if())
                .field("fifo_ov_if", &self.fifo_ov_if())
                .finish()
        }
    }
    #[doc = "RW, USBHS host interrupt status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhIntSt(pub u8);
    impl UhIntSt {
        #[doc = "Received PID."]
        #[must_use]
        #[inline(always)]
        pub const fn this_pid_mask30(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Received PID."]
        #[inline(always)]
        pub const fn set_this_pid_mask30(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "This bit is 1, indicating that the port has received a wake-up signal."]
        #[must_use]
        #[inline(always)]
        pub const fn port_rx_resume(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "This bit is 1, indicating that the port has received a wake-up signal."]
        #[inline(always)]
        pub const fn set_port_rx_resume(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for UhIntSt {
        #[inline(always)]
        fn default() -> UhIntSt {
            UhIntSt(0)
        }
    }
    impl core::fmt::Debug for UhIntSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhIntSt")
                .field("this_pid_mask30", &self.this_pid_mask30())
                .field("port_rx_resume", &self.port_rx_resume())
                .finish()
        }
    }
    #[doc = "RW, USBHS Host Power Management Data Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhLpm(pub u32);
    impl UhLpm {
        #[doc = "Power management data."]
        #[must_use]
        #[inline(always)]
        pub const fn lpm_data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Power management data."]
        #[inline(always)]
        pub const fn set_lpm_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for UhLpm {
        #[inline(always)]
        fn default() -> UhLpm {
            UhLpm(0)
        }
    }
    impl core::fmt::Debug for UhLpm {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhLpm")
                .field("lpm_data", &self.lpm_data())
                .finish()
        }
    }
    #[doc = "RW, USBHS host miscellaneous status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhMisSt(pub u8);
    impl UhMisSt {
        #[doc = "Port enabled status."]
        #[must_use]
        #[inline(always)]
        pub const fn sof_free(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port enabled status."]
        #[inline(always)]
        pub const fn set_sof_free(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "The USB bus SOF packet indicates a status of."]
        #[must_use]
        #[inline(always)]
        pub const fn sof_pre(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "The USB bus SOF packet indicates a status of."]
        #[inline(always)]
        pub const fn set_sof_pre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB bus SOF status."]
        #[must_use]
        #[inline(always)]
        pub const fn sof_st(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus SOF status."]
        #[inline(always)]
        pub const fn set_sof_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "USB bus wake-up."]
        #[must_use]
        #[inline(always)]
        pub const fn wakeup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus wake-up."]
        #[inline(always)]
        pub const fn set_wakeup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB bus status."]
        #[must_use]
        #[inline(always)]
        pub const fn linestate_mask(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "USB bus status."]
        #[inline(always)]
        pub const fn set_linestate_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "J on the USB bus."]
        #[must_use]
        #[inline(always)]
        pub const fn bus_j(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "J on the USB bus."]
        #[inline(always)]
        pub const fn set_bus_j(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "SEO on USB bus."]
        #[must_use]
        #[inline(always)]
        pub const fn bus_se0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SEO on USB bus."]
        #[inline(always)]
        pub const fn set_bus_se0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhMisSt {
        #[inline(always)]
        fn default() -> UhMisSt {
            UhMisSt(0)
        }
    }
    impl core::fmt::Debug for UhMisSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhMisSt")
                .field("sof_free", &self.sof_free())
                .field("sof_pre", &self.sof_pre())
                .field("sof_st", &self.sof_st())
                .field("wakeup", &self.wakeup())
                .field("linestate_mask", &self.linestate_mask())
                .field("bus_j", &self.bus_j())
                .field("bus_se0", &self.bus_se0())
                .finish()
        }
    }
    #[doc = "RW, DMA receiving address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhRxDma(pub u32);
    impl UhRxDma {
        #[doc = "Receiving Address."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "Receiving Address."]
        #[inline(always)]
        pub const fn set_uh_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for UhRxDma {
        #[inline(always)]
        fn default() -> UhRxDma {
            UhRxDma(0)
        }
    }
    impl core::fmt::Debug for UhRxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhRxDma")
                .field("uh_rx_dma", &self.uh_rx_dma())
                .finish()
        }
    }
    #[doc = "RW, USBHS host receives length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhRxLen(pub u32);
    impl UhRxLen {
        #[doc = "Received data length."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Received data length."]
        #[inline(always)]
        pub const fn set_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for UhRxLen {
        #[inline(always)]
        fn default() -> UhRxLen {
            UhRxLen(0)
        }
    }
    impl core::fmt::Debug for UhRxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhRxLen")
                .field("rx_len", &self.rx_len())
                .finish()
        }
    }
    #[doc = "RW, USBHS host receives maximum length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhRxMaxLen(pub u32);
    impl UhRxMaxLen {
        #[doc = "Received data max length."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_max_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Received data max length."]
        #[inline(always)]
        pub const fn set_uh_rx_max_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for UhRxMaxLen {
        #[inline(always)]
        fn default() -> UhRxMaxLen {
            UhRxMaxLen(0)
        }
    }
    impl core::fmt::Debug for UhRxMaxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhRxMaxLen")
                .field("uh_rx_max_len", &self.uh_rx_max_len())
                .finish()
        }
    }
    #[doc = "RW, USBHS Host SPLIT Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhSplit(pub u32);
    impl UhSplit {
        #[doc = "SPLIT_DATA."]
        #[must_use]
        #[inline(always)]
        pub const fn split_data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "SPLIT_DATA."]
        #[inline(always)]
        pub const fn set_split_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
    }
    impl Default for UhSplit {
        #[inline(always)]
        fn default() -> UhSplit {
            UhSplit(0)
        }
    }
    impl core::fmt::Debug for UhSplit {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhSplit")
                .field("split_data", &self.split_data())
                .finish()
        }
    }
    #[doc = "RW, DMA sending address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhTxDma(pub u32);
    impl UhTxDma {
        #[doc = "Sending Address."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tx_dma_hs(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "Sending Address."]
        #[inline(always)]
        pub const fn set_uh_tx_dma_hs(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for UhTxDma {
        #[inline(always)]
        fn default() -> UhTxDma {
            UhTxDma(0)
        }
    }
    impl core::fmt::Debug for UhTxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhTxDma")
                .field("uh_tx_dma_hs", &self.uh_tx_dma_hs())
                .finish()
        }
    }
    #[doc = "RW, USBHS host sends length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhTxLen(pub u32);
    impl UhTxLen {
        #[doc = "Sending data length."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Sending data length."]
        #[inline(always)]
        pub const fn set_tx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for UhTxLen {
        #[inline(always)]
        fn default() -> UhTxLen {
            UhTxLen(0)
        }
    }
    impl core::fmt::Debug for UhTxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhTxLen")
                .field("tx_len", &self.tx_len())
                .finish()
        }
    }
    #[doc = "RW, USBHS control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usb2BaseMode(pub u8);
    impl Usb2BaseMode {
        #[doc = "Expected speed mode of the device."]
        #[must_use]
        #[inline(always)]
        pub const fn exp_spd10(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Expected speed mode of the device."]
        #[inline(always)]
        pub const fn set_exp_spd10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
    }
    impl Default for Usb2BaseMode {
        #[inline(always)]
        fn default() -> Usb2BaseMode {
            Usb2BaseMode(0)
        }
    }
    impl core::fmt::Debug for Usb2BaseMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usb2BaseMode")
                .field("exp_spd10", &self.exp_spd10())
                .finish()
        }
    }
    #[doc = "RO, USBHS bus status memory."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usb2Bus(pub u16);
    impl Usb2Bus {
        #[doc = "USB wake-up (highly effective)."]
        #[must_use]
        #[inline(always)]
        pub const fn wakeup(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB wake-up (highly effective)."]
        #[inline(always)]
        pub const fn set_wakeup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "UDP status."]
        #[must_use]
        #[inline(always)]
        pub const fn dp_st(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "UDP status."]
        #[inline(always)]
        pub const fn set_dp_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "UDM status."]
        #[must_use]
        #[inline(always)]
        pub const fn dm_st(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "UDM status."]
        #[inline(always)]
        pub const fn set_dm_st(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
    }
    impl Default for Usb2Bus {
        #[inline(always)]
        fn default() -> Usb2Bus {
            Usb2Bus(0)
        }
    }
    impl core::fmt::Debug for Usb2Bus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usb2Bus")
                .field("wakeup", &self.wakeup())
                .field("dp_st", &self.dp_st())
                .field("dm_st", &self.dm_st())
                .finish()
        }
    }
    #[doc = "RW, USBHS control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usb2Ctrl(pub u8);
    impl Usb2Ctrl {
        #[doc = "LINK layer reset, highly effective."]
        #[must_use]
        #[inline(always)]
        pub const fn link_reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LINK layer reset, highly effective."]
        #[inline(always)]
        pub const fn set_link_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB protocol processor reset, including endpoint related registers, highly effective."]
        #[must_use]
        #[inline(always)]
        pub const fn sie_reset(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB protocol processor reset, including endpoint related registers, highly effective."]
        #[inline(always)]
        pub const fn set_sie_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Clear all interrupt flags."]
        #[must_use]
        #[inline(always)]
        pub const fn all_clr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear all interrupt flags."]
        #[inline(always)]
        pub const fn set_all_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "USB PHY suspended."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_suspendm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USB PHY suspended."]
        #[inline(always)]
        pub const fn set_phy_suspendm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "DMA transfer enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dev_dma_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA transfer enable."]
        #[inline(always)]
        pub const fn set_dev_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "USB device enablement."]
        #[must_use]
        #[inline(always)]
        pub const fn dev_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USB device enablement."]
        #[inline(always)]
        pub const fn set_dev_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "LPM Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dev_lpm_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LPM Enable."]
        #[inline(always)]
        pub const fn set_dev_lpm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Usb2Ctrl {
        #[inline(always)]
        fn default() -> Usb2Ctrl {
            Usb2Ctrl(0)
        }
    }
    impl core::fmt::Debug for Usb2Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usb2Ctrl")
                .field("link_reset", &self.link_reset())
                .field("sie_reset", &self.sie_reset())
                .field("all_clr", &self.all_clr())
                .field("phy_suspendm", &self.phy_suspendm())
                .field("dev_dma_en", &self.dev_dma_en())
                .field("dev_en", &self.dev_en())
                .field("dev_lpm_en", &self.dev_lpm_en())
                .finish()
        }
    }
    #[doc = "RW, USBHS device address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usb2DevAd(pub u8);
    impl Usb2DevAd {
        #[doc = "USB device address."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_usb_addr60(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "USB device address."]
        #[inline(always)]
        pub const fn set_mask_usb_addr60(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
    }
    impl Default for Usb2DevAd {
        #[inline(always)]
        fn default() -> Usb2DevAd {
            Usb2DevAd(0)
        }
    }
    impl core::fmt::Debug for Usb2DevAd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usb2DevAd")
                .field("mask_usb_addr60", &self.mask_usb_addr60())
                .finish()
        }
    }
    #[doc = "RO, USBHS frame register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usb2FramNo(pub u16);
    impl Usb2FramNo {
        #[doc = "Received frame number."]
        #[must_use]
        #[inline(always)]
        pub const fn frame_no100(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Received frame number."]
        #[inline(always)]
        pub const fn set_frame_no100(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
        #[doc = "Received microframe number."]
        #[must_use]
        #[inline(always)]
        pub const fn micro_frame20(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Received microframe number."]
        #[inline(always)]
        pub const fn set_micro_frame20(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
        }
    }
    impl Default for Usb2FramNo {
        #[inline(always)]
        fn default() -> Usb2FramNo {
            Usb2FramNo(0)
        }
    }
    impl core::fmt::Debug for Usb2FramNo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usb2FramNo")
                .field("frame_no100", &self.frame_no100())
                .field("micro_frame20", &self.micro_frame20())
                .finish()
        }
    }
    #[doc = "RW, USBHS interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usb2IntEn(pub u8);
    impl Usb2IntEn {
        #[doc = "USB bus reset interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bus_rest_ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus reset interrupt enable."]
        #[inline(always)]
        pub const fn set_bus_rest_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB bus pause interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bus_susp_ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus pause interrupt enable."]
        #[inline(always)]
        pub const fn set_bus_susp_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB bus sleep interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bus_sleep_ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus sleep interrupt enable."]
        #[inline(always)]
        pub const fn set_bus_sleep_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "LPM transmission end interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn lpm_act_ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPM transmission end interrupt enable."]
        #[inline(always)]
        pub const fn set_lpm_act_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB Transfer End Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rtx_act_ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB Transfer End Interrupt Enable."]
        #[inline(always)]
        pub const fn set_rtx_act_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Receive SOF packet interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_sof_ie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Receive SOF packet interrupt enable."]
        #[inline(always)]
        pub const fn set_rx_sof_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "USB connection interruption enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn link_rdy_ie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "USB connection interruption enabled."]
        #[inline(always)]
        pub const fn set_link_rdy_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "FIFO overflow interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_over_ie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overflow interrupt."]
        #[inline(always)]
        pub const fn set_fifo_over_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Usb2IntEn {
        #[inline(always)]
        fn default() -> Usb2IntEn {
            Usb2IntEn(0)
        }
    }
    impl core::fmt::Debug for Usb2IntEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usb2IntEn")
                .field("bus_rest_ie", &self.bus_rest_ie())
                .field("bus_susp_ie", &self.bus_susp_ie())
                .field("bus_sleep_ie", &self.bus_sleep_ie())
                .field("lpm_act_ie", &self.lpm_act_ie())
                .field("rtx_act_ie", &self.rtx_act_ie())
                .field("rx_sof_ie", &self.rx_sof_ie())
                .field("link_rdy_ie", &self.link_rdy_ie())
                .field("fifo_over_ie", &self.fifo_over_ie())
                .finish()
        }
    }
    #[doc = "RW1Z, USBHS interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usb2IntFg(pub u8);
    impl Usb2IntFg {
        #[doc = "USB bus reset interrupt flag bit, write 1 to reset to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_bus_rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus reset interrupt flag bit, write 1 to reset to zero."]
        #[inline(always)]
        pub const fn set_uif_bus_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB bus hangs interrupt flag, write 1 to reset to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_suspend_hs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus hangs interrupt flag, write 1 to reset to zero."]
        #[inline(always)]
        pub const fn set_uif_suspend_hs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB bus sleep interrupt flag, write 1 to reset to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_bus_sleep(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus sleep interrupt flag, write 1 to reset to zero."]
        #[inline(always)]
        pub const fn set_uif_bus_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "LPM transmission end interrupt flag, write 1 to reset to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_lpm_act(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPM transmission end interrupt flag, write 1 to reset to zero."]
        #[inline(always)]
        pub const fn set_uif_lpm_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB transfer end interrupt flag bit, cleared by RB-UMS_SHMOD bit."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_rtx_act(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB transfer end interrupt flag bit, cleared by RB-UMS_SHMOD bit."]
        #[inline(always)]
        pub const fn set_uif_rtx_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Receive SOF packet interrupt flag, write 1 to reset to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_rx_sof(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Receive SOF packet interrupt flag, write 1 to reset to zero."]
        #[inline(always)]
        pub const fn set_uif_rx_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "USB connection interruption flag bit, write 1 to reset to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_link_rdy(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "USB connection interruption flag bit, write 1 to reset to zero."]
        #[inline(always)]
        pub const fn set_uif_link_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "FIFO overflow interrupt flag bit, write 1 to reset to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_fifo_ov_hs(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overflow interrupt flag bit, write 1 to reset to zero."]
        #[inline(always)]
        pub const fn set_uif_fifo_ov_hs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Usb2IntFg {
        #[inline(always)]
        fn default() -> Usb2IntFg {
            Usb2IntFg(0)
        }
    }
    impl core::fmt::Debug for Usb2IntFg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usb2IntFg")
                .field("uif_bus_rst", &self.uif_bus_rst())
                .field("uif_suspend_hs", &self.uif_suspend_hs())
                .field("uif_bus_sleep", &self.uif_bus_sleep())
                .field("uif_lpm_act", &self.uif_lpm_act())
                .field("uif_rtx_act", &self.uif_rtx_act())
                .field("uif_rx_sof", &self.uif_rx_sof())
                .field("uif_link_rdy", &self.uif_link_rdy())
                .field("uif_fifo_ov_hs", &self.uif_fifo_ov_hs())
                .finish()
        }
    }
    #[doc = "RO, USBHS Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usb2IntSt(pub u8);
    impl Usb2IntSt {
        #[doc = "Endpoint number where data transmission occurs."]
        #[must_use]
        #[inline(always)]
        pub const fn uis_ep_id_mask20(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Endpoint number where data transmission occurs."]
        #[inline(always)]
        pub const fn set_uis_ep_id_mask20(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
        #[doc = "Endpoint data transmission direction."]
        #[must_use]
        #[inline(always)]
        pub const fn uis_ep_dir(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Endpoint data transmission direction."]
        #[inline(always)]
        pub const fn set_uis_ep_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for Usb2IntSt {
        #[inline(always)]
        fn default() -> Usb2IntSt {
            Usb2IntSt(0)
        }
    }
    impl core::fmt::Debug for Usb2IntSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usb2IntSt")
                .field("uis_ep_id_mask20", &self.uis_ep_id_mask20())
                .field("uis_ep_dir", &self.uis_ep_dir())
                .finish()
        }
    }
    #[doc = "RW, USBHS power management register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usb2LpmData(pub u16);
    impl Usb2LpmData {
        #[doc = "Power management data."]
        #[must_use]
        #[inline(always)]
        pub const fn lpm_data100(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Power management data."]
        #[inline(always)]
        pub const fn set_lpm_data100(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
        #[doc = "Power management busy."]
        #[must_use]
        #[inline(always)]
        pub const fn lpm_busy(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Power management busy."]
        #[inline(always)]
        pub const fn set_lpm_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for Usb2LpmData {
        #[inline(always)]
        fn default() -> Usb2LpmData {
            Usb2LpmData(0)
        }
    }
    impl core::fmt::Debug for Usb2LpmData {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usb2LpmData")
                .field("lpm_data100", &self.lpm_data100())
                .field("lpm_busy", &self.lpm_busy())
                .finish()
        }
    }
    #[doc = "RO, USBHS Miscellaneous Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usb2MisSt(pub u8);
    impl Usb2MisSt {
        #[doc = "USB connection status."]
        #[must_use]
        #[inline(always)]
        pub const fn ums_ready(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB connection status."]
        #[inline(always)]
        pub const fn set_ums_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB suspended status."]
        #[must_use]
        #[inline(always)]
        pub const fn ums_suspend_hs(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB suspended status."]
        #[inline(always)]
        pub const fn set_ums_suspend_hs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB sleep state."]
        #[must_use]
        #[inline(always)]
        pub const fn ums_sleep(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB sleep state."]
        #[inline(always)]
        pub const fn set_ums_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "USB idle state."]
        #[must_use]
        #[inline(always)]
        pub const fn ums_sie_free_hs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USB idle state."]
        #[inline(always)]
        pub const fn set_ums_sie_free_hs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB suspend request."]
        #[must_use]
        #[inline(always)]
        pub const fn ums_susp_req(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB suspend request."]
        #[inline(always)]
        pub const fn set_ums_susp_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Is the host high-speed."]
        #[must_use]
        #[inline(always)]
        pub const fn ums_hs_mod(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Is the host high-speed."]
        #[inline(always)]
        pub const fn set_ums_hs_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Usb2MisSt {
        #[inline(always)]
        fn default() -> Usb2MisSt {
            Usb2MisSt(0)
        }
    }
    impl core::fmt::Debug for Usb2MisSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usb2MisSt")
                .field("ums_ready", &self.ums_ready())
                .field("ums_suspend_hs", &self.ums_suspend_hs())
                .field("ums_sleep", &self.ums_sleep())
                .field("ums_sie_free_hs", &self.ums_sie_free_hs())
                .field("ums_susp_req", &self.ums_susp_req())
                .field("ums_hs_mod", &self.ums_hs_mod())
                .finish()
        }
    }
    #[doc = "RW, USBHS test mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usb2TestMode(pub u8);
    impl Usb2TestMode {
        #[doc = "Test mode, output J."]
        #[must_use]
        #[inline(always)]
        pub const fn test_j(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Test mode, output J."]
        #[inline(always)]
        pub const fn set_test_j(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Test mode, output K."]
        #[must_use]
        #[inline(always)]
        pub const fn test_k(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Test mode, output K."]
        #[inline(always)]
        pub const fn set_test_k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Test mode, output a package."]
        #[must_use]
        #[inline(always)]
        pub const fn test_pkt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Test mode, output a package."]
        #[inline(always)]
        pub const fn set_test_pkt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Test mode, output SE0."]
        #[must_use]
        #[inline(always)]
        pub const fn test_se0nak(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Test mode, output SE0."]
        #[inline(always)]
        pub const fn set_test_se0nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Enable Test Mode."]
        #[must_use]
        #[inline(always)]
        pub const fn test_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Test Mode."]
        #[inline(always)]
        pub const fn set_test_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Usb2TestMode {
        #[inline(always)]
        fn default() -> Usb2TestMode {
            Usb2TestMode(0)
        }
    }
    impl core::fmt::Debug for Usb2TestMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usb2TestMode")
                .field("test_j", &self.test_j())
                .field("test_k", &self.test_k())
                .field("test_pkt", &self.test_pkt())
                .field("test_se0nak", &self.test_se0nak())
                .field("test_en", &self.test_en())
                .finish()
        }
    }
    #[doc = "RW1Z, USBHS remote wake-up register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Usb2WakeCtrl(pub u8);
    impl Usb2WakeCtrl {
        #[doc = "Remote wake-up, hardware auto reset."]
        #[must_use]
        #[inline(always)]
        pub const fn rmt_wake(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Remote wake-up, hardware auto reset."]
        #[inline(always)]
        pub const fn set_rmt_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Usb2WakeCtrl {
        #[inline(always)]
        fn default() -> Usb2WakeCtrl {
            Usb2WakeCtrl(0)
        }
    }
    impl core::fmt::Debug for Usb2WakeCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Usb2WakeCtrl")
                .field("rmt_wake", &self.rmt_wake())
                .finish()
        }
    }
}
