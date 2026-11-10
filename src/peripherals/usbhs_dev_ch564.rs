#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "USB register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbhsDev {
    ptr: *mut u8,
}
unsafe impl Send for UsbhsDev {}
unsafe impl Sync for UsbhsDev {}
impl UsbhsDev {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB base control register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USB mode control register."]
    #[inline(always)]
    pub const fn base_mode(self) -> crate::common::Reg<regs::BaseMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "USB interrupt enable register."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn dev_ad(self) -> crate::common::Reg<regs::DevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "USB remote wake up register."]
    #[inline(always)]
    pub const fn wake_ctrl(self) -> crate::common::Reg<regs::WakeCtrl, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "USB test mode register."]
    #[inline(always)]
    pub const fn test_mode(self) -> crate::common::Reg<regs::TestMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "USB power management register."]
    #[inline(always)]
    pub const fn lpm_data(self) -> crate::common::Reg<regs::LpmData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "USB interrupt flag register."]
    #[inline(always)]
    pub const fn int_fg(self) -> crate::common::Reg<regs::IntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "USB interrupt status."]
    #[inline(always)]
    pub const fn int_st(self) -> crate::common::Reg<regs::IntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "USB miscellaneous status."]
    #[inline(always)]
    pub const fn mis_st(self) -> crate::common::Reg<regs::MisSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "USB frame number."]
    #[inline(always)]
    pub const fn fram_no(self) -> crate::common::Reg<regs::FramNo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "USB bus."]
    #[inline(always)]
    pub const fn bus(self) -> crate::common::Reg<regs::Bus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "USB endpoint sends the enable register."]
    #[inline(always)]
    pub const fn uep_tx_en(self) -> crate::common::Reg<regs::UepTxEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "USB endpoint receive the enable registers."]
    #[inline(always)]
    pub const fn uep_rx_en(self) -> crate::common::Reg<regs::UepRxEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "USB endpoint sends the auto-filp enable register."]
    #[inline(always)]
    pub const fn uep_t_tog_auto(self) -> crate::common::Reg<regs::UepTTogAuto, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "USB endpoint receive the auto-filp enable register."]
    #[inline(always)]
    pub const fn uep_r_tog_auto(self) -> crate::common::Reg<regs::UepRTogAuto, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "USB endpoint sends a burst register."]
    #[inline(always)]
    pub const fn uep_t_burst(self) -> crate::common::Reg<regs::UepTBurst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "USB endpoint send the mode register."]
    #[inline(always)]
    pub const fn uep_t_burst_mode(
        self,
    ) -> crate::common::Reg<regs::UepTBurstMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x19usize) as _) }
    }
    #[doc = "USB endpoint receives the burst register."]
    #[inline(always)]
    pub const fn uep_r_burst(self) -> crate::common::Reg<regs::UepRBurst, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "USB endpoint reply mode register."]
    #[inline(always)]
    pub const fn uep_r_res_mode(self) -> crate::common::Reg<regs::UepRResMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1busize) as _) }
    }
    #[doc = "USB endpoint muitiplexing register."]
    #[inline(always)]
    pub const fn uep_af_mode(self) -> crate::common::Reg<regs::UepAfMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "The start address register of the endpoint 0 buffer."]
    #[inline(always)]
    pub const fn uep0_dma(self) -> crate::common::Reg<regs::Uep0Dma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "endpoint n (n=1..7) receives the start address register of the buffer. Array index 0..6 maps to endpoint 1..7."]
    #[inline(always)]
    pub const fn uep_rx_dma(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxDma, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize + n * 4usize) as _) }
    }
    #[doc = "endpoint n (n=1..7) sends the start address register of the buffer. Array index 0..6 maps to endpoint 1..7."]
    #[inline(always)]
    pub const fn uep_tx_dma(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepTxDma, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize + n * 4usize) as _) }
    }
    #[doc = "endpoint 0 max length packet register."]
    #[inline(always)]
    pub const fn uep0_max_len(self) -> crate::common::Reg<regs::Uep0MaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "endpoint n (n=1..7) max length packet register. Array index 0..6 maps to endpoint 1..7."]
    #[inline(always)]
    pub const fn uep_max_len(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepMaxLen, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize + n * 4usize) as _) }
    }
    #[doc = "endpoint 0 acceptable length."]
    #[inline(always)]
    pub const fn uep0_rx_len(self) -> crate::common::Reg<regs::Uep0RxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "endpoint n (n=1..7) receives the lenth register in a single pass. Array index 0..6 maps to endpoint 1..7."]
    #[inline(always)]
    pub const fn uep_rx_len(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxLen, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "the length register of the total received data at endpoint n (n=1..7). Array index 0..6 maps to endpoint 1..7."]
    #[inline(always)]
    pub const fn uep_r_size(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRSize, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x82usize + n * 4usize) as _) }
    }
    #[doc = "endpoint 0 send the length."]
    #[inline(always)]
    pub const fn uep0_t_len(self) -> crate::common::Reg<regs::Uep0TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "endpoint 0 send control register."]
    #[inline(always)]
    pub const fn uep0_tx_ctrl(self) -> crate::common::Reg<regs::Uep0TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9eusize) as _) }
    }
    #[doc = "endpoint 0 send control register."]
    #[inline(always)]
    pub const fn uep0_rx_ctrl(self) -> crate::common::Reg<regs::Uep0RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9fusize) as _) }
    }
    #[doc = "endpoint n (n=1..7) send the length register. Array index 0..6 maps to endpoint 1..7."]
    #[inline(always)]
    pub const fn uep_t_len(self, n: usize) -> crate::common::Reg<regs::UepTLen, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize + n * 4usize) as _) }
    }
    #[doc = "endpoint n (n=1..7) send control register. Array index 0..6 maps to endpoint 1..7."]
    #[inline(always)]
    pub const fn uep_tx_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepTxCtrl, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa2usize + n * 4usize) as _) }
    }
    #[doc = "endpoint n (n=1..7) receive control. Array index 0..6 maps to endpoint 1..7."]
    #[inline(always)]
    pub const fn uep_rx_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::UepRxCtrl, crate::common::RW> {
        assert!(n < 7usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa3usize + n * 4usize) as _) }
    }
    #[doc = "usb endpoint sends a synchronous mode enable register."]
    #[inline(always)]
    pub const fn uep_t_iso(self) -> crate::common::Reg<regs::UepTIso, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "usb endpoint receives a synchronous mode enable register."]
    #[inline(always)]
    pub const fn uep_r_iso(self) -> crate::common::Reg<regs::UepRIso, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbeusize) as _) }
    }
}
pub mod regs {
    #[doc = "USB mode control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BaseMode(pub u8);
    impl BaseMode {
        #[doc = "The desired speed mode of the device."]
        #[must_use]
        #[inline(always)]
        pub const fn exp_spd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "The desired speed mode of the device."]
        #[inline(always)]
        pub const fn set_exp_spd(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
    }
    impl Default for BaseMode {
        #[inline(always)]
        fn default() -> BaseMode {
            BaseMode(0)
        }
    }
    impl core::fmt::Debug for BaseMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BaseMode")
                .field("exp_spd", &self.exp_spd())
                .finish()
        }
    }
    #[doc = "USB bus."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bus(pub u16);
    impl Bus {
        #[doc = "USB wakeup."]
        #[must_use]
        #[inline(always)]
        pub const fn wakeup(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB wakeup."]
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
    impl Default for Bus {
        #[inline(always)]
        fn default() -> Bus {
            Bus(0)
        }
    }
    impl core::fmt::Debug for Bus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bus")
                .field("wakeup", &self.wakeup())
                .field("dp_st", &self.dp_st())
                .field("dm_st", &self.dm_st())
                .finish()
        }
    }
    #[doc = "USB base control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u8);
    impl Ctrl {
        #[doc = "LINK layer reset,highly effective."]
        #[must_use]
        #[inline(always)]
        pub const fn link_reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LINK layer reset,highly effective."]
        #[inline(always)]
        pub const fn set_link_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB protocol processor reset."]
        #[must_use]
        #[inline(always)]
        pub const fn sie_reset(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB protocol processor reset."]
        #[inline(always)]
        pub const fn set_sie_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "clear all interrupt flags."]
        #[must_use]
        #[inline(always)]
        pub const fn all_clr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "clear all interrupt flags."]
        #[inline(always)]
        pub const fn set_all_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "USB PHY suspend."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_suspendm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USB PHY suspend."]
        #[inline(always)]
        pub const fn set_phy_suspendm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "DMA transfer enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn dev_dma_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA transfer enabled."]
        #[inline(always)]
        pub const fn set_dev_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "USB device enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn dev_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USB device enabled."]
        #[inline(always)]
        pub const fn set_dev_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "LPM enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dev_lpm_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LPM enable."]
        #[inline(always)]
        pub const fn set_dev_lpm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl")
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
    #[doc = "USB device address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DevAd(pub u8);
    impl DevAd {
        #[doc = "bit mask for USB device address."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_usb_addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "bit mask for USB device address."]
        #[inline(always)]
        pub const fn set_mask_usb_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u8) & 0x7f) << 0usize);
        }
    }
    impl Default for DevAd {
        #[inline(always)]
        fn default() -> DevAd {
            DevAd(0)
        }
    }
    impl core::fmt::Debug for DevAd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DevAd")
                .field("mask_usb_addr", &self.mask_usb_addr())
                .finish()
        }
    }
    #[doc = "USB frame number."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FramNo(pub u16);
    impl FramNo {
        #[doc = "Received frame number."]
        #[must_use]
        #[inline(always)]
        pub const fn frame_no(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Received frame number."]
        #[inline(always)]
        pub const fn set_frame_no(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
        #[doc = "Received micro frame number."]
        #[must_use]
        #[inline(always)]
        pub const fn micro_frame(&self) -> u8 {
            let val = (self.0 >> 13usize) & 0x07;
            val as u8
        }
        #[doc = "Received micro frame number."]
        #[inline(always)]
        pub const fn set_micro_frame(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u16) & 0x07) << 13usize);
        }
    }
    impl Default for FramNo {
        #[inline(always)]
        fn default() -> FramNo {
            FramNo(0)
        }
    }
    impl core::fmt::Debug for FramNo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FramNo")
                .field("frame_no", &self.frame_no())
                .field("micro_frame", &self.micro_frame())
                .finish()
        }
    }
    #[doc = "USB interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u8);
    impl IntEn {
        #[doc = "USB bus reset interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn bus_rest_ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus reset interrupt enabled."]
        #[inline(always)]
        pub const fn set_bus_rest_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB bus pause interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn bus_susp_ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus pause interrupt enabled."]
        #[inline(always)]
        pub const fn set_bus_susp_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB bus sleep interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn bus_sleep_ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus sleep interrupt enabled."]
        #[inline(always)]
        pub const fn set_bus_sleep_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "LMP transfer end interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn lpm_act_ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LMP transfer end interrupt enabled."]
        #[inline(always)]
        pub const fn set_lpm_act_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB transfer end interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn rtx_act_ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB transfer end interrupt enabled."]
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
        #[doc = "USB connection interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn link_rdy_ie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "USB connection interrupt enable."]
        #[inline(always)]
        pub const fn set_link_rdy_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "USB Overflow interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_over_ie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USB Overflow interrupt enable."]
        #[inline(always)]
        pub const fn set_fifo_over_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for IntEn {
        #[inline(always)]
        fn default() -> IntEn {
            IntEn(0)
        }
    }
    impl core::fmt::Debug for IntEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntEn")
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
    #[doc = "USB interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFg(pub u8);
    impl IntFg {
        #[doc = "USB bus reset interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_bus_rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus reset interrupt flag."]
        #[inline(always)]
        pub const fn set_uif_bus_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB bus suspend interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_suspend(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus suspend interrupt flag."]
        #[inline(always)]
        pub const fn set_uif_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB bus sleep interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_bus_sleep(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus sleep interrupt flag."]
        #[inline(always)]
        pub const fn set_uif_bus_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "LPM transmission end interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_lpm_act(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LPM transmission end interrupt flag."]
        #[inline(always)]
        pub const fn set_uif_lpm_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB transmission end interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_rtx_act(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB transmission end interrupt flag."]
        #[inline(always)]
        pub const fn set_uif_rtx_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Receive SOF packet interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_rx_sof(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Receive SOF packet interrupt flag."]
        #[inline(always)]
        pub const fn set_uif_rx_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "USB connection interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_link_rdy(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "USB connection interrupt flag."]
        #[inline(always)]
        pub const fn set_uif_link_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "USB Overflow interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_fifo_ov(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USB Overflow interrupt flag."]
        #[inline(always)]
        pub const fn set_uif_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for IntFg {
        #[inline(always)]
        fn default() -> IntFg {
            IntFg(0)
        }
    }
    impl core::fmt::Debug for IntFg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntFg")
                .field("uif_bus_rst", &self.uif_bus_rst())
                .field("uif_suspend", &self.uif_suspend())
                .field("uif_bus_sleep", &self.uif_bus_sleep())
                .field("uif_lpm_act", &self.uif_lpm_act())
                .field("uif_rtx_act", &self.uif_rtx_act())
                .field("uif_rx_sof", &self.uif_rx_sof())
                .field("uif_link_rdy", &self.uif_link_rdy())
                .field("uif_fifo_ov", &self.uif_fifo_ov())
                .finish()
        }
    }
    #[doc = "USB interrupt status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntSt(pub u8);
    impl IntSt {
        #[doc = "The endpoint number at which the data transfer occurs."]
        #[must_use]
        #[inline(always)]
        pub const fn uis_ep_id_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "The endpoint number at which the data transfer occurs."]
        #[inline(always)]
        pub const fn set_uis_ep_id_mask(&mut self, val: u8) {
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
    impl Default for IntSt {
        #[inline(always)]
        fn default() -> IntSt {
            IntSt(0)
        }
    }
    impl core::fmt::Debug for IntSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntSt")
                .field("uis_ep_id_mask", &self.uis_ep_id_mask())
                .field("uis_ep_dir", &self.uis_ep_dir())
                .finish()
        }
    }
    #[doc = "USB power management register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LpmData(pub u16);
    impl LpmData {
        #[doc = "power management data."]
        #[must_use]
        #[inline(always)]
        pub const fn lpm_data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "power management data."]
        #[inline(always)]
        pub const fn set_lpm_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u16) & 0x07ff) << 0usize);
        }
        #[doc = "power management busy."]
        #[must_use]
        #[inline(always)]
        pub const fn lpm_busy(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "power management busy."]
        #[inline(always)]
        pub const fn set_lpm_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for LpmData {
        #[inline(always)]
        fn default() -> LpmData {
            LpmData(0)
        }
    }
    impl core::fmt::Debug for LpmData {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("LpmData")
                .field("lpm_data", &self.lpm_data())
                .field("lpm_busy", &self.lpm_busy())
                .finish()
        }
    }
    #[doc = "USB miscellaneous status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MisSt(pub u8);
    impl MisSt {
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
        #[doc = "USB suspend status."]
        #[must_use]
        #[inline(always)]
        pub const fn ums_suspend(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB suspend status."]
        #[inline(always)]
        pub const fn set_ums_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB sleep status."]
        #[must_use]
        #[inline(always)]
        pub const fn ums_sleep(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB sleep status."]
        #[inline(always)]
        pub const fn set_ums_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "USB free status."]
        #[must_use]
        #[inline(always)]
        pub const fn ums_free(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USB free status."]
        #[inline(always)]
        pub const fn set_ums_free(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB suspends the request."]
        #[must_use]
        #[inline(always)]
        pub const fn ums_susp_req(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB suspends the request."]
        #[inline(always)]
        pub const fn set_ums_susp_req(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "whether the host is high-speed."]
        #[must_use]
        #[inline(always)]
        pub const fn ums_hs_mod(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "whether the host is high-speed."]
        #[inline(always)]
        pub const fn set_ums_hs_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for MisSt {
        #[inline(always)]
        fn default() -> MisSt {
            MisSt(0)
        }
    }
    impl core::fmt::Debug for MisSt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("MisSt")
                .field("ums_ready", &self.ums_ready())
                .field("ums_suspend", &self.ums_suspend())
                .field("ums_sleep", &self.ums_sleep())
                .field("ums_free", &self.ums_free())
                .field("ums_susp_req", &self.ums_susp_req())
                .field("ums_hs_mod", &self.ums_hs_mod())
                .finish()
        }
    }
    #[doc = "USB test mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TestMode(pub u8);
    impl TestMode {
        #[doc = "test mode,output J."]
        #[must_use]
        #[inline(always)]
        pub const fn test_j(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "test mode,output J."]
        #[inline(always)]
        pub const fn set_test_j(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "test mode,output K."]
        #[must_use]
        #[inline(always)]
        pub const fn test_k(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "test mode,output K."]
        #[inline(always)]
        pub const fn set_test_k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "test mode,output a packet."]
        #[must_use]
        #[inline(always)]
        pub const fn test_pkt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "test mode,output a packet."]
        #[inline(always)]
        pub const fn set_test_pkt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "test mode,output SEO."]
        #[must_use]
        #[inline(always)]
        pub const fn test_se0nak(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "test mode,output SEO."]
        #[inline(always)]
        pub const fn set_test_se0nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "test mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn test_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "test mode enable."]
        #[inline(always)]
        pub const fn set_test_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for TestMode {
        #[inline(always)]
        fn default() -> TestMode {
            TestMode(0)
        }
    }
    impl core::fmt::Debug for TestMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TestMode")
                .field("test_j", &self.test_j())
                .field("test_k", &self.test_k())
                .field("test_pkt", &self.test_pkt())
                .field("test_se0nak", &self.test_se0nak())
                .field("test_en", &self.test_en())
                .finish()
        }
    }
    #[doc = "The start address register of the endpoint 0 buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0Dma(pub u32);
    impl Uep0Dma {
        #[doc = "The start address of the endpoint 0 buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn uep0_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "The start address of the endpoint 0 buffer."]
        #[inline(always)]
        pub const fn set_uep0_dma(&mut self, val: u32) {
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
                .field("uep0_dma", &self.uep0_dma())
                .finish()
        }
    }
    #[doc = "endpoint 0 max length packet register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0MaxLen(pub u32);
    impl Uep0MaxLen {
        #[doc = "endpoint 0 max acceptable offset length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep0_max_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "endpoint 0 max acceptable offset length."]
        #[inline(always)]
        pub const fn set_uep0_max_len(&mut self, val: u8) {
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
                .field("uep0_max_len", &self.uep0_max_len())
                .finish()
        }
    }
    #[doc = "endpoint 0 send control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0RxCtrl(pub u8);
    impl Uep0RxCtrl {
        #[doc = "endpoint 0 has control over the received response."]
        #[must_use]
        #[inline(always)]
        pub const fn res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 0 has control over the received response."]
        #[inline(always)]
        pub const fn set_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "the reception of endpoint 0 expects a synchronous trigger. bit."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tog_mask(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "the reception of endpoint 0 expects a synchronous trigger. bit."]
        #[inline(always)]
        pub const fn set_uep_tog_mask(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "whether endpoint 0 receives a SETUP transaction."]
        #[must_use]
        #[inline(always)]
        pub const fn setup_is(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "whether endpoint 0 receives a SETUP transaction."]
        #[inline(always)]
        pub const fn set_setup_is(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[must_use]
        #[inline(always)]
        pub const fn endp_r_tog_match(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub const fn set_endp_r_tog_match(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "endpoint 0 for the received return NAK,packet type."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_nak_tog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 0 for the received return NAK,packet type."]
        #[inline(always)]
        pub const fn set_uep_r_nak_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "endpoint 0 receives the end of NAK flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 0 receives the end of NAK flag."]
        #[inline(always)]
        pub const fn set_uep_r_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 0 receives the end flag."]
        #[must_use]
        #[inline(always)]
        pub const fn endp_r_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 0 receives the end flag."]
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
                .field("res_mask", &self.res_mask())
                .field("uep_tog_mask", &self.uep_tog_mask())
                .field("setup_is", &self.setup_is())
                .field("endp_r_tog_match", &self.endp_r_tog_match())
                .field("uep_r_nak_tog", &self.uep_r_nak_tog())
                .field("uep_r_nak_act", &self.uep_r_nak_act())
                .field("endp_r_done", &self.endp_r_done())
                .finish()
        }
    }
    #[doc = "endpoint 0 acceptable length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0RxLen(pub u16);
    impl Uep0RxLen {
        #[doc = "endpoint 0 acceptable data length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep0_rx_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "endpoint 0 acceptable data length."]
        #[inline(always)]
        pub const fn set_uep0_rx_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u16) & 0x7f) << 0usize);
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
                .field("uep0_rx_len", &self.uep0_rx_len())
                .finish()
        }
    }
    #[doc = "endpoint 0 send the length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0TLen(pub u16);
    impl Uep0TLen {
        #[doc = "endpoint 0 send the length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep0_t_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "endpoint 0 send the length."]
        #[inline(always)]
        pub const fn set_uep0_t_len(&mut self, val: u8) {
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
                .field("uep0_t_len", &self.uep0_t_len())
                .finish()
        }
    }
    #[doc = "endpoint 0 send control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0TxCtrl(pub u8);
    impl Uep0TxCtrl {
        #[doc = "endpoint 0 control of the send response to IN transactions."]
        #[must_use]
        #[inline(always)]
        pub const fn res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 0 control of the send response to IN transactions."]
        #[inline(always)]
        pub const fn set_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "endpoint 0 synchronous trigger bit for the sender to. prepare."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_tog_mask(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint 0 synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub const fn set_ep_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "endpoint 0 sends the end flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 0 sends the end flag."]
        #[inline(always)]
        pub const fn set_uep_t_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint 0 sends the end of NAK flag."]
        #[must_use]
        #[inline(always)]
        pub const fn endp_t_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint 0 sends the end of NAK flag."]
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
                .field("res_mask", &self.res_mask())
                .field("ep_tog_mask", &self.ep_tog_mask())
                .field("uep_t_nak_act", &self.uep_t_nak_act())
                .field("endp_t_done", &self.endp_t_done())
                .finish()
        }
    }
    #[doc = "USB endpoint muitiplexing register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepAfMode(pub u32);
    impl UepAfMode {
        #[doc = "1 to 7 endpoint muitiplexing enables."]
        #[must_use]
        #[inline(always)]
        pub const fn ep_t_af(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "1 to 7 endpoint muitiplexing enables."]
        #[inline(always)]
        pub const fn set_ep_t_af(&mut self, val: u8) {
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
                .field("ep_t_af", &self.ep_t_af())
                .finish()
        }
    }
    #[doc = "endpoint n max length packet register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepMaxLen(pub u32);
    impl UepMaxLen {
        #[doc = "endpoint n max acceptable offset length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_max_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "endpoint n max acceptable offset length."]
        #[inline(always)]
        pub const fn set_uep_max_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
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
                .field("uep_max_len", &self.uep_max_len())
                .finish()
        }
    }
    #[doc = "USB endpoint receives the burst register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRBurst(pub u8);
    impl UepRBurst {
        #[doc = "0 to 7 endpoint receive burst enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_burst_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0 to 7 endpoint receive burst enable."]
        #[inline(always)]
        pub const fn set_uep_r_burst_en(&mut self, val: u8) {
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
                .field("uep_r_burst_en", &self.uep_r_burst_en())
                .finish()
        }
    }
    #[doc = "usb endpoint receives a synchronous mode enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRIso(pub u16);
    impl UepRIso {
        #[doc = "down endpoint(OUT) synchronization mode enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_iso(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "down endpoint(OUT) synchronization mode enabled."]
        #[inline(always)]
        pub const fn set_uep_r_iso(&mut self, val: u8) {
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
                .field("uep_r_iso", &self.uep_r_iso())
                .finish()
        }
    }
    #[doc = "USB endpoint reply mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRResMode(pub u8);
    impl UepRResMode {
        #[doc = "0 to 7 endpoint reply mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_res_mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0 to 7 endpoint reply mode."]
        #[inline(always)]
        pub const fn set_uep_r_res_mode(&mut self, val: u8) {
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
                .field("uep_r_res_mode", &self.uep_r_res_mode())
                .finish()
        }
    }
    #[doc = "the length register of the total received data at endpoint n."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRSize(pub u16);
    impl UepRSize {
        #[doc = "the length of the total received data at endpoint n."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_size(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the length of the total received data at endpoint n."]
        #[inline(always)]
        pub const fn set_uep_r_size(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for UepRSize {
        #[inline(always)]
        fn default() -> UepRSize {
            UepRSize(0)
        }
    }
    impl core::fmt::Debug for UepRSize {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UepRSize")
                .field("uep_r_size", &self.uep_r_size())
                .finish()
        }
    }
    #[doc = "USB endpoint receive the auto-filp enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRTogAuto(pub u16);
    impl UepRTogAuto {
        #[doc = "0 to 7 endpoint synchronization triggers bit auto-filp. enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog_auto(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0 to 7 endpoint synchronization triggers bit auto-filp. enable."]
        #[inline(always)]
        pub const fn set_uep_r_tog_auto(&mut self, val: u8) {
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
                .field("uep_r_tog_auto", &self.uep_r_tog_auto())
                .finish()
        }
    }
    #[doc = "endpoint n receive control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxCtrl(pub u8);
    impl UepRxCtrl {
        #[doc = "endpoint n has control over the received response."]
        #[must_use]
        #[inline(always)]
        pub const fn res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint n has control over the received response."]
        #[inline(always)]
        pub const fn set_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "the reception of endpoint n expects a synchronous trigger. bit."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tog_mask(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "the reception of endpoint n expects a synchronous trigger. bit."]
        #[inline(always)]
        pub const fn set_uep_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[must_use]
        #[inline(always)]
        pub const fn endp_r_tog_match(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "received synchronization trigger bit matches the desired. synchronization trigger bit state."]
        #[inline(always)]
        pub const fn set_endp_r_tog_match(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "endpoint n for the received return NAK,packet type."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_nak_tog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint n for the received return NAK,packet type."]
        #[inline(always)]
        pub const fn set_uep_r_nak_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "endpoint n receives the end of NAK flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint n receives the end of NAK flag."]
        #[inline(always)]
        pub const fn set_uep_r_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint n receives the end flag."]
        #[must_use]
        #[inline(always)]
        pub const fn endp_r_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint n receives the end flag."]
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
                .field("res_mask", &self.res_mask())
                .field("uep_tog_mask", &self.uep_tog_mask())
                .field("endp_r_tog_match", &self.endp_r_tog_match())
                .field("uep_r_nak_tog", &self.uep_r_nak_tog())
                .field("uep_r_nak_act", &self.uep_r_nak_act())
                .field("endp_r_done", &self.endp_r_done())
                .finish()
        }
    }
    #[doc = "endpoint n receives the start address register of the buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxDma(pub u32);
    impl UepRxDma {
        #[doc = "endpoint receives the start address register of the buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "endpoint receives the start address register of the buffer."]
        #[inline(always)]
        pub const fn set_uep_rx_dma(&mut self, val: u32) {
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
                .field("uep_rx_dma", &self.uep_rx_dma())
                .finish()
        }
    }
    #[doc = "USB endpoint receive the enable registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxEn(pub u16);
    impl UepRxEn {
        #[doc = "Endpoint 0 to 15 receive enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rx_en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Endpoint 0 to 15 receive enable."]
        #[inline(always)]
        pub const fn set_uep_rx_en(&mut self, val: u16) {
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
                .field("uep_rx_en", &self.uep_rx_en())
                .finish()
        }
    }
    #[doc = "endpoint n receives the lenth register in a single pass."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepRxLen(pub u16);
    impl UepRxLen {
        #[doc = "the length of data received by endpoint n in a single pass."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the length of data received by endpoint n in a single pass."]
        #[inline(always)]
        pub const fn set_uep_rx_len(&mut self, val: u16) {
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
                .field("uep_rx_len", &self.uep_rx_len())
                .finish()
        }
    }
    #[doc = "USB endpoint sends a burst register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTBurst(pub u8);
    impl UepTBurst {
        #[doc = "0 to 7 endpoint send burst enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_burst_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0 to 7 endpoint send burst enable."]
        #[inline(always)]
        pub const fn set_uep_t_burst_en(&mut self, val: u8) {
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
                .field("uep_t_burst_en", &self.uep_t_burst_en())
                .finish()
        }
    }
    #[doc = "USB endpoint send the mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTBurstMode(pub u8);
    impl UepTBurstMode {
        #[doc = "0 to 7 endpoint send the mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_burst_en(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0 to 7 endpoint send the mode enable."]
        #[inline(always)]
        pub const fn set_uep_t_burst_en(&mut self, val: u8) {
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
                .field("uep_t_burst_en", &self.uep_t_burst_en())
                .finish()
        }
    }
    #[doc = "usb endpoint sends a synchronous mode enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTIso(pub u16);
    impl UepTIso {
        #[doc = "upload endpoint(IN) synchronization mode enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_iso(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x7f;
            val as u8
        }
        #[doc = "upload endpoint(IN) synchronization mode enabled."]
        #[inline(always)]
        pub const fn set_uep_t_iso(&mut self, val: u8) {
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
                .field("uep_t_iso", &self.uep_t_iso())
                .finish()
        }
    }
    #[doc = "endpoint n send the length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTLen(pub u16);
    impl UepTLen {
        #[doc = "endpoint n send the length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint n send the length."]
        #[inline(always)]
        pub const fn set_uep_t_len(&mut self, val: u16) {
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
                .field("uep_t_len", &self.uep_t_len())
                .finish()
        }
    }
    #[doc = "USB endpoint sends the auto-filp enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTTogAuto(pub u16);
    impl UepTTogAuto {
        #[doc = "0 to 7 endpoint synchronization triggers bit auto-filp. enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog_auto(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "0 to 7 endpoint synchronization triggers bit auto-filp. enable."]
        #[inline(always)]
        pub const fn set_uep_t_tog_auto(&mut self, val: u8) {
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
                .field("uep_t_tog_auto", &self.uep_t_tog_auto())
                .finish()
        }
    }
    #[doc = "endpoint n send control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxCtrl(pub u8);
    impl UepTxCtrl {
        #[doc = "endpoint n control of the send response to IN transactions."]
        #[must_use]
        #[inline(always)]
        pub const fn res_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint n control of the send response to IN transactions."]
        #[inline(always)]
        pub const fn set_res_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "endpoint n synchronous trigger bit for the sender to. prepare."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tog_mask(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "endpoint n synchronous trigger bit for the sender to. prepare."]
        #[inline(always)]
        pub const fn set_uep_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "endpoint n sends the end flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_nak_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint n sends the end flag."]
        #[inline(always)]
        pub const fn set_uep_t_nak_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "endpoint n sends the end of NAK flag."]
        #[must_use]
        #[inline(always)]
        pub const fn endp_t_done(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "endpoint n sends the end of NAK flag."]
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
                .field("res_mask", &self.res_mask())
                .field("uep_tog_mask", &self.uep_tog_mask())
                .field("uep_t_nak_act", &self.uep_t_nak_act())
                .field("endp_t_done", &self.endp_t_done())
                .finish()
        }
    }
    #[doc = "endpoint n sends the start address register of the buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxDma(pub u32);
    impl UepTxDma {
        #[doc = "endpoint sends the start address register of the buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "endpoint sends the start address register of the buffer."]
        #[inline(always)]
        pub const fn set_uep_tx_dma(&mut self, val: u32) {
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
                .field("uep_tx_dma", &self.uep_tx_dma())
                .finish()
        }
    }
    #[doc = "USB endpoint sends the enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTxEn(pub u16);
    impl UepTxEn {
        #[doc = "Endpoint 0 to 15 sends enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tx_en(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Endpoint 0 to 15 sends enable."]
        #[inline(always)]
        pub const fn set_uep_tx_en(&mut self, val: u16) {
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
                .field("uep_tx_en", &self.uep_tx_en())
                .finish()
        }
    }
    #[doc = "USB remote wake up register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WakeCtrl(pub u16);
    impl WakeCtrl {
        #[doc = "remote wake up."]
        #[must_use]
        #[inline(always)]
        pub const fn rmt_wake(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "remote wake up."]
        #[inline(always)]
        pub const fn set_rmt_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
    }
    impl Default for WakeCtrl {
        #[inline(always)]
        fn default() -> WakeCtrl {
            WakeCtrl(0)
        }
    }
    impl core::fmt::Debug for WakeCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WakeCtrl")
                .field("rmt_wake", &self.rmt_wake())
                .finish()
        }
    }
}
