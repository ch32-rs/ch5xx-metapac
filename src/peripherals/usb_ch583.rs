#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "USB register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb {
    ptr: *mut u8,
}
unsafe impl Send for Usb {}
unsafe impl Sync for Usb {}
impl Usb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB base control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USB device physical port control."]
    #[inline(always)]
    pub const fn udev_ctrl(self) -> crate::common::Reg<regs::UdevCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "USB host physical port control."]
    #[inline(always)]
    pub const fn uhost_ctrl(self) -> crate::common::Reg<regs::UhostCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "USB interrupt enable."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "USB device address."]
    #[inline(always)]
    pub const fn dev_ad(self) -> crate::common::Reg<regs::DevAd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "USB miscellaneous status."]
    #[inline(always)]
    pub const fn mis_st(self) -> crate::common::Reg<regs::MisSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "USB interrupt flag."]
    #[inline(always)]
    pub const fn int_fg(self) -> crate::common::Reg<regs::IntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "USB interrupt status."]
    #[inline(always)]
    pub const fn int_st(self) -> crate::common::Reg<regs::IntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "USB receiving length."]
    #[inline(always)]
    pub const fn rx_len(self) -> crate::common::Reg<regs::RxLen, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "endpoint 4/1 mode."]
    #[inline(always)]
    pub const fn uep4_1_mod(self) -> crate::common::Reg<regs::Uep41Mod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Device endpoints 2 and 3 mode."]
    #[inline(always)]
    pub const fn uep2_3_mod(self) -> crate::common::Reg<regs::Uep23Mod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dusize) as _) }
    }
    #[doc = "Host endpoint mode."]
    #[inline(always)]
    pub const fn uh_ep_mod(self) -> crate::common::Reg<regs::UhEpMod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dusize) as _) }
    }
    #[doc = "endpoint 5/6/7 mode."]
    #[inline(always)]
    pub const fn uep567_mod(self) -> crate::common::Reg<regs::Uep567Mod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "endpoint 0 DMA buffer address."]
    #[inline(always)]
    pub const fn uep0_dma(self) -> crate::common::Reg<regs::Uep0Dma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "endpoint 1 DMA buffer address."]
    #[inline(always)]
    pub const fn uep1_dma(self) -> crate::common::Reg<regs::Uep1Dma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Endpoint 2 DMA buffer address."]
    #[inline(always)]
    pub const fn uep2_dma(self) -> crate::common::Reg<regs::Uep2Dma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Host receive endpoint DMA buffer address."]
    #[inline(always)]
    pub const fn uh_rx_dma(self) -> crate::common::Reg<regs::UhRxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Endpoint 3 DMA buffer address."]
    #[inline(always)]
    pub const fn uep3_dma(self) -> crate::common::Reg<regs::Uep3Dma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Host transmit endpoint DMA buffer address."]
    #[inline(always)]
    pub const fn uh_tx_dma(self) -> crate::common::Reg<regs::UhTxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "endpoint 0 transmittal length."]
    #[inline(always)]
    pub const fn uep0_t_len(self) -> crate::common::Reg<regs::Uep0TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "Device endpoint transmit length, indexed by endpoint number. In host mode, endpoints 2 and 3 have different meanings."]
    #[inline(always)]
    pub const fn uep_t_len(self, n: usize) -> crate::common::Reg<regs::UepTLen, crate::common::RW> {
        assert!(n < 8usize);
        unsafe {
            crate::common::Reg::from_ptr(self.ptr.wrapping_add(
                0x20usize
                    + ([
                        0usize, 4usize, 8usize, 12usize, 16usize, 68usize, 72usize, 76usize,
                    ][n] as usize),
            ) as _)
        }
    }
    #[doc = "endpoint 0 control."]
    #[inline(always)]
    pub const fn uep0_ctrl(self) -> crate::common::Reg<regs::Uep0Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
    #[doc = "endpoint 1 transmittal length."]
    #[inline(always)]
    pub const fn uep1_t_len(self) -> crate::common::Reg<regs::Uep1TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "Endpoint 1 control."]
    #[inline(always)]
    pub const fn uep1_ctrl(self) -> crate::common::Reg<regs::Uep1Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "Host auxiliary setup."]
    #[inline(always)]
    pub const fn uh_setup(self) -> crate::common::Reg<regs::UhSetup, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "Endpoint 2 transmit length."]
    #[inline(always)]
    pub const fn uep2_t_len(self) -> crate::common::Reg<regs::Uep2TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Host endpoint and token PID."]
    #[inline(always)]
    pub const fn uh_ep_pid(self) -> crate::common::Reg<regs::UhEpPid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Endpoint 2 control."]
    #[inline(always)]
    pub const fn uep2_ctrl(self) -> crate::common::Reg<regs::Uep2Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "Host receive endpoint control."]
    #[inline(always)]
    pub const fn uh_rx_ctrl(self) -> crate::common::Reg<regs::UhRxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "Endpoint 3 transmit length."]
    #[inline(always)]
    pub const fn uep3_t_len(self) -> crate::common::Reg<regs::Uep3TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Host transmit length."]
    #[inline(always)]
    pub const fn uh_tx_len(self) -> crate::common::Reg<regs::UhTxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Endpoint 3 control."]
    #[inline(always)]
    pub const fn uep3_ctrl(self) -> crate::common::Reg<regs::Uep3Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2eusize) as _) }
    }
    #[doc = "Host transmit endpoint control."]
    #[inline(always)]
    pub const fn uh_tx_ctrl(self) -> crate::common::Reg<regs::UhTxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2eusize) as _) }
    }
    #[doc = "endpoint 4 transmittal length."]
    #[inline(always)]
    pub const fn uep4_t_len(self) -> crate::common::Reg<regs::Uep4TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "endpoint 4 control."]
    #[inline(always)]
    pub const fn uep4_ctrl(self) -> crate::common::Reg<regs::Uep4Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
    #[doc = "endpoint 5 DMA buffer address."]
    #[inline(always)]
    pub const fn uep5_dma(self) -> crate::common::Reg<regs::Uep5Dma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "endpoint 6 DMA buffer address."]
    #[inline(always)]
    pub const fn uep6_dma(self) -> crate::common::Reg<regs::Uep6Dma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "endpoint 7 DMA buffer address."]
    #[inline(always)]
    pub const fn uep7_dma(self) -> crate::common::Reg<regs::Uep7Dma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "endpoint 5 transmittal length."]
    #[inline(always)]
    pub const fn uep5_t_len(self) -> crate::common::Reg<regs::Uep5TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "endpoint 5 control."]
    #[inline(always)]
    pub const fn uep5_ctrl(self) -> crate::common::Reg<regs::Uep5Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x66usize) as _) }
    }
    #[doc = "endpoint 6 transmittal length."]
    #[inline(always)]
    pub const fn uep6_t_len(self) -> crate::common::Reg<regs::Uep6TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "endpoint 6 control."]
    #[inline(always)]
    pub const fn uep6_ctrl(self) -> crate::common::Reg<regs::Uep6Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6ausize) as _) }
    }
    #[doc = "endpoint 7 transmittal length."]
    #[inline(always)]
    pub const fn uep7_t_len(self) -> crate::common::Reg<regs::Uep7TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "endpoint 7 control."]
    #[inline(always)]
    pub const fn uep7_ctrl(self) -> crate::common::Reg<regs::Uep7Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6eusize) as _) }
    }
}
pub mod regs {
    #[doc = "USB base control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u8);
    impl Ctrl {
        #[doc = "DMA enable and DMA interrupt enable for USB."]
        #[must_use]
        #[inline(always)]
        pub const fn uc_dma_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable and DMA interrupt enable for USB."]
        #[inline(always)]
        pub const fn set_uc_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "force clear FIFO and count of USB."]
        #[must_use]
        #[inline(always)]
        pub const fn uc_clr_all(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force clear FIFO and count of USB."]
        #[inline(always)]
        pub const fn set_uc_clr_all(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "force reset USB SIE, need software clear."]
        #[must_use]
        #[inline(always)]
        pub const fn uc_reset_sie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "force reset USB SIE, need software clear."]
        #[inline(always)]
        pub const fn set_uc_reset_sie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid."]
        #[must_use]
        #[inline(always)]
        pub const fn uc_int_busy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid."]
        #[inline(always)]
        pub const fn set_uc_int_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "bit mask of USB system control."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uc_sys_ctrl(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of USB system control."]
        #[inline(always)]
        pub const fn set_mask_uc_sys_ctrl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "USB device enable and internal pullup resistance enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uc_dev_pu_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "USB device enable and internal pullup resistance enable."]
        #[inline(always)]
        pub const fn set_uc_dev_pu_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "enable USB low speed: 0=12Mbps, 1=1.5Mbps."]
        #[must_use]
        #[inline(always)]
        pub const fn uc_low_speed(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB low speed: 0=12Mbps, 1=1.5Mbps."]
        #[inline(always)]
        pub const fn set_uc_low_speed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "enable USB host mode: 0=device mode, 1=host mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uc_host_mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB host mode: 0=device mode, 1=host mode."]
        #[inline(always)]
        pub const fn set_uc_host_mode(&mut self, val: bool) {
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
                .field("uc_dma_en", &self.uc_dma_en())
                .field("uc_clr_all", &self.uc_clr_all())
                .field("uc_reset_sie", &self.uc_reset_sie())
                .field("uc_int_busy", &self.uc_int_busy())
                .field("mask_uc_sys_ctrl", &self.mask_uc_sys_ctrl())
                .field("uc_dev_pu_en", &self.uc_dev_pu_en())
                .field("uc_low_speed", &self.uc_low_speed())
                .field("uc_host_mode", &self.uc_host_mode())
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
        #[doc = "general purpose bit."]
        #[must_use]
        #[inline(always)]
        pub const fn uda_gp_bit(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "general purpose bit."]
        #[inline(always)]
        pub const fn set_uda_gp_bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("uda_gp_bit", &self.uda_gp_bit())
                .finish()
        }
    }
    #[doc = "USB interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u8);
    impl IntEn {
        #[doc = "Enable bus reset interrupt in device mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uie_bus_rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable bus reset interrupt in device mode."]
        #[inline(always)]
        pub const fn set_uie_bus_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uie_bus_rst__uie_detect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for USB bus reset event for USB device mode;enable interrupt for USB device detected event for USB host mode."]
        #[inline(always)]
        pub const fn set_uie_bus_rst__uie_detect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Enable device detection interrupt in host mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uie_detect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable device detection interrupt in host mode."]
        #[inline(always)]
        pub const fn set_uie_detect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable interrupt for USB transfer completion."]
        #[must_use]
        #[inline(always)]
        pub const fn uie_transfer(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for USB transfer completion."]
        #[inline(always)]
        pub const fn set_uie_transfer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "enable interrupt for USB suspend or resume event."]
        #[must_use]
        #[inline(always)]
        pub const fn uie_suspend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for USB suspend or resume event."]
        #[inline(always)]
        pub const fn set_uie_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable interrupt for host SOF timer action for USB host mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uie_hst_sof(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for host SOF timer action for USB host mode."]
        #[inline(always)]
        pub const fn set_uie_hst_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "enable interrupt for FIFO overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn uie_fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for FIFO overflow."]
        #[inline(always)]
        pub const fn set_uie_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable interrupt for NAK responded for USB device mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uie_dev_nak(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for NAK responded for USB device mode."]
        #[inline(always)]
        pub const fn set_uie_dev_nak(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "enable interrupt for SOF received for USB device mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uie_dev_sof(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for SOF received for USB device mode."]
        #[inline(always)]
        pub const fn set_uie_dev_sof(&mut self, val: bool) {
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
                .field("uie_bus_rst", &self.uie_bus_rst())
                .field("uie_bus_rst__uie_detect", &self.uie_bus_rst__uie_detect())
                .field("uie_detect", &self.uie_detect())
                .field("uie_transfer", &self.uie_transfer())
                .field("uie_suspend", &self.uie_suspend())
                .field("uie_hst_sof", &self.uie_hst_sof())
                .field("uie_fifo_ov", &self.uie_fifo_ov())
                .field("uie_dev_nak", &self.uie_dev_nak())
                .field("uie_dev_sof", &self.uie_dev_sof())
                .finish()
        }
    }
    #[doc = "USB interrupt flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFg(pub u8);
    impl IntFg {
        #[doc = "Device bus reset interrupt flag; write 1 to clear. Avoid read-modify-write when clearing flags."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_bus_rst(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Device bus reset interrupt flag; write 1 to clear. Avoid read-modify-write when clearing flags."]
        #[inline(always)]
        pub const fn set_uif_bus_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW,bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_bus_rst__uif_detect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW,bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn set_uif_bus_rst__uif_detect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Host device detection interrupt flag; write 1 to clear. Avoid read-modify-write when clearing flags."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_detect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Host device detection interrupt flag; write 1 to clear. Avoid read-modify-write when clearing flags."]
        #[inline(always)]
        pub const fn set_uif_detect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW,USB transfer completion interrupt flag, direct bit address clear or write 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_transfer(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW,USB transfer completion interrupt flag, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn set_uif_transfer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW,USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_suspend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW,USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn set_uif_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW,host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_hst_sof(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW,host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn set_uif_hst_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW,FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn uif_fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW,FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn set_uif_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RO, indicate USB SIE free status."]
        #[must_use]
        #[inline(always)]
        pub const fn u_sie_free(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate USB SIE free status."]
        #[inline(always)]
        pub const fn set_u_sie_free(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RO, indicate current USB transfer toggle is OK."]
        #[must_use]
        #[inline(always)]
        pub const fn u_tog_ok(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate current USB transfer toggle is OK."]
        #[inline(always)]
        pub const fn set_u_tog_ok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RO, indicate current USB transfer is NAK received."]
        #[must_use]
        #[inline(always)]
        pub const fn u_is_nak(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate current USB transfer is NAK received."]
        #[inline(always)]
        pub const fn set_u_is_nak(&mut self, val: bool) {
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
                .field("uif_bus_rst__uif_detect", &self.uif_bus_rst__uif_detect())
                .field("uif_detect", &self.uif_detect())
                .field("uif_transfer", &self.uif_transfer())
                .field("uif_suspend", &self.uif_suspend())
                .field("uif_hst_sof", &self.uif_hst_sof())
                .field("uif_fifo_ov", &self.uif_fifo_ov())
                .field("u_sie_free", &self.u_sie_free())
                .field("u_tog_ok", &self.u_tog_ok())
                .field("u_is_nak", &self.u_is_nak())
                .finish()
        }
    }
    #[doc = "USB interrupt status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntSt(pub u8);
    impl IntSt {
        #[doc = "Device transfer endpoint number."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uis_endp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Device transfer endpoint number."]
        #[inline(always)]
        pub const fn set_mask_uis_endp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "Host transfer handshake response PID, or zero on timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uis_h_res(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Host transfer handshake response PID, or zero on timeout."]
        #[inline(always)]
        pub const fn set_mask_uis_h_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uis_h_res__mask_uis_endp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode."]
        #[inline(always)]
        pub const fn set_mask_uis_h_res__mask_uis_endp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "RO, bit mask of current token PID code received for USB device mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uis_token(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "RO, bit mask of current token PID code received for USB device mode."]
        #[inline(always)]
        pub const fn set_mask_uis_token(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "RO, indicate current USB transfer toggle is OK."]
        #[must_use]
        #[inline(always)]
        pub const fn uis_tog_ok(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate current USB transfer toggle is OK."]
        #[inline(always)]
        pub const fn set_uis_tog_ok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RO, indicates a SETUP token and 8-byte setup request were received in USB device mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uis_setup_act(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicates a SETUP token and 8-byte setup request were received in USB device mode."]
        #[inline(always)]
        pub const fn set_uis_setup_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("mask_uis_endp", &self.mask_uis_endp())
                .field("mask_uis_h_res", &self.mask_uis_h_res())
                .field(
                    "mask_uis_h_res__mask_uis_endp",
                    &self.mask_uis_h_res__mask_uis_endp(),
                )
                .field("mask_uis_token", &self.mask_uis_token())
                .field("uis_tog_ok", &self.uis_tog_ok())
                .field("uis_setup_act", &self.uis_setup_act())
                .finish()
        }
    }
    #[doc = "USB miscellaneous status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MisSt(pub u8);
    impl MisSt {
        #[doc = "RO, indicate device attached status on USB host."]
        #[must_use]
        #[inline(always)]
        pub const fn ums_dev_attach(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate device attached status on USB host."]
        #[inline(always)]
        pub const fn set_ums_dev_attach(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RO, indicate UDM level saved at device attached to USB host."]
        #[must_use]
        #[inline(always)]
        pub const fn ums_dm_level(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate UDM level saved at device attached to USB host."]
        #[inline(always)]
        pub const fn set_ums_dm_level(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RO, indicate USB suspend status."]
        #[must_use]
        #[inline(always)]
        pub const fn ums_suspend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate USB suspend status."]
        #[inline(always)]
        pub const fn set_ums_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RO, indicate USB bus reset status."]
        #[must_use]
        #[inline(always)]
        pub const fn ums_bus_reset(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate USB bus reset status."]
        #[inline(always)]
        pub const fn set_ums_bus_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RO, indicate USB receiving FIFO ready status (not empty)."]
        #[must_use]
        #[inline(always)]
        pub const fn ums_r_fifo_rdy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate USB receiving FIFO ready status (not empty)."]
        #[inline(always)]
        pub const fn set_ums_r_fifo_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RO, indicate USB SIE free status."]
        #[must_use]
        #[inline(always)]
        pub const fn ums_sie_free(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate USB SIE free status."]
        #[inline(always)]
        pub const fn set_ums_sie_free(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RO, indicate host SOF timer action status for USB host."]
        #[must_use]
        #[inline(always)]
        pub const fn ums_sof_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate host SOF timer action status for USB host."]
        #[inline(always)]
        pub const fn set_ums_sof_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RO, indicate host SOF timer presage status."]
        #[must_use]
        #[inline(always)]
        pub const fn ums_sof_pres(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate host SOF timer presage status."]
        #[inline(always)]
        pub const fn set_ums_sof_pres(&mut self, val: bool) {
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
                .field("ums_dev_attach", &self.ums_dev_attach())
                .field("ums_dm_level", &self.ums_dm_level())
                .field("ums_suspend", &self.ums_suspend())
                .field("ums_bus_reset", &self.ums_bus_reset())
                .field("ums_r_fifo_rdy", &self.ums_r_fifo_rdy())
                .field("ums_sie_free", &self.ums_sie_free())
                .field("ums_sof_act", &self.ums_sof_act())
                .field("ums_sof_pres", &self.ums_sof_pres())
                .finish()
        }
    }
    #[doc = "USB receiving length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxLen(pub u8);
    impl RxLen {
        #[doc = "RO,USB receiving length."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RO,USB receiving length."]
        #[inline(always)]
        pub const fn set_rx_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for RxLen {
        #[inline(always)]
        fn default() -> RxLen {
            RxLen(0)
        }
    }
    impl core::fmt::Debug for RxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxLen")
                .field("rx_len", &self.rx_len())
                .finish()
        }
    }
    #[doc = "USB device physical port control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UdevCtrl(pub u8);
    impl UdevCtrl {
        #[doc = "enable USB physical port I-O: 0=disable, 1=enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ud_port_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB physical port I-O: 0=disable, 1=enable"]
        #[inline(always)]
        pub const fn set_ud_port_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "general purpose bit"]
        #[must_use]
        #[inline(always)]
        pub const fn ud_gp_bit(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "general purpose bit"]
        #[inline(always)]
        pub const fn set_ud_gp_bit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "enable USB physical port low speed: 0=full speed, 1=low speed"]
        #[must_use]
        #[inline(always)]
        pub const fn ud_low_speed(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB physical port low speed: 0=full speed, 1=low speed"]
        #[inline(always)]
        pub const fn set_ud_low_speed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "ReadOnly: indicate current UDM pin level."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_dm_pin(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ReadOnly: indicate current UDM pin level."]
        #[inline(always)]
        pub const fn set_ud_dm_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "ReadOnly: indicate current UDP pin level."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_dp_pin(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ReadOnly: indicate current UDP pin level."]
        #[inline(always)]
        pub const fn set_ud_dp_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "disable USB UDP-UDM pulldown resistance: 0=enable pulldown, 1=disable."]
        #[must_use]
        #[inline(always)]
        pub const fn ud_pd_dis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "disable USB UDP-UDM pulldown resistance: 0=enable pulldown, 1=disable."]
        #[inline(always)]
        pub const fn set_ud_pd_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UdevCtrl {
        #[inline(always)]
        fn default() -> UdevCtrl {
            UdevCtrl(0)
        }
    }
    impl core::fmt::Debug for UdevCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UdevCtrl")
                .field("ud_port_en", &self.ud_port_en())
                .field("ud_gp_bit", &self.ud_gp_bit())
                .field("ud_low_speed", &self.ud_low_speed())
                .field("ud_dm_pin", &self.ud_dm_pin())
                .field("ud_dp_pin", &self.ud_dp_pin())
                .field("ud_pd_dis", &self.ud_pd_dis())
                .finish()
        }
    }
    #[doc = "endpoint 0 control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0Ctrl(pub u8);
    impl Uep0Ctrl {
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uep_t_res(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[inline(always)]
        pub const fn set_mask_uep_t_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uep_r_res(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[inline(always)]
        pub const fn set_mask_uep_r_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "Automatic toggle is documented for endpoints 1, 2 and 3 only."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_auto_tog(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic toggle is documented for endpoints 1, 2 and 3 only."]
        #[inline(always)]
        pub const fn set_uep_auto_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn set_uep_t_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn set_uep_r_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep0Ctrl {
        #[inline(always)]
        fn default() -> Uep0Ctrl {
            Uep0Ctrl(0)
        }
    }
    impl core::fmt::Debug for Uep0Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep0Ctrl")
                .field("mask_uep_t_res", &self.mask_uep_t_res())
                .field("mask_uep_r_res", &self.mask_uep_r_res())
                .field("uep_auto_tog", &self.uep_auto_tog())
                .field("uep_t_tog", &self.uep_t_tog())
                .field("uep_r_tog", &self.uep_r_tog())
                .finish()
        }
    }
    #[doc = "endpoint 0 DMA buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0Dma(pub u16);
    impl Uep0Dma {
        #[doc = "RW,endpoint 0 DMA buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep0_dma(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RW,endpoint 0 DMA buffer address."]
        #[inline(always)]
        pub const fn set_uep0_dma(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
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
    #[doc = "endpoint 0 transmittal length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0TLen(pub u8);
    impl Uep0TLen {
        #[doc = "endpoint 0 transmittal length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep0_t_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "endpoint 0 transmittal length."]
        #[inline(always)]
        pub const fn set_uep0_t_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
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
    #[doc = "Endpoint 1 control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1Ctrl(pub u8);
    impl Uep1Ctrl {
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uep_t_res(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[inline(always)]
        pub const fn set_mask_uep_t_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uep_r_res(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[inline(always)]
        pub const fn set_mask_uep_r_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 1_2_3: 0=manual toggle, 1=automatic toggle."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_auto_tog(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 1_2_3: 0=manual toggle, 1=automatic toggle."]
        #[inline(always)]
        pub const fn set_uep_auto_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1"]
        #[inline(always)]
        pub const fn set_uep_t_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1"]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1"]
        #[inline(always)]
        pub const fn set_uep_r_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep1Ctrl {
        #[inline(always)]
        fn default() -> Uep1Ctrl {
            Uep1Ctrl(0)
        }
    }
    impl core::fmt::Debug for Uep1Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1Ctrl")
                .field("mask_uep_t_res", &self.mask_uep_t_res())
                .field("mask_uep_r_res", &self.mask_uep_r_res())
                .field("uep_auto_tog", &self.uep_auto_tog())
                .field("uep_t_tog", &self.uep_t_tog())
                .field("uep_r_tog", &self.uep_r_tog())
                .finish()
        }
    }
    #[doc = "endpoint 1 DMA buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1Dma(pub u16);
    impl Uep1Dma {
        #[doc = "RW,endpoint 1 DMA buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep1_dma(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RW,endpoint 1 DMA buffer address."]
        #[inline(always)]
        pub const fn set_uep1_dma(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep1Dma {
        #[inline(always)]
        fn default() -> Uep1Dma {
            Uep1Dma(0)
        }
    }
    impl core::fmt::Debug for Uep1Dma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1Dma")
                .field("uep1_dma", &self.uep1_dma())
                .finish()
        }
    }
    #[doc = "endpoint 1 transmittal length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1TLen(pub u8);
    impl Uep1TLen {
        #[doc = "endpoint 1 transmittal length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep1_t_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "endpoint 1 transmittal length."]
        #[inline(always)]
        pub const fn set_uep1_t_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep1TLen {
        #[inline(always)]
        fn default() -> Uep1TLen {
            Uep1TLen(0)
        }
    }
    impl core::fmt::Debug for Uep1TLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1TLen")
                .field("uep1_t_len", &self.uep1_t_len())
                .finish()
        }
    }
    #[doc = "Device endpoints 2 and 3 mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep23Mod(pub u8);
    impl Uep23Mod {
        #[doc = "buffer mode of USB endpoint 2"]
        #[must_use]
        #[inline(always)]
        pub const fn uep2_buf_mod(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint 2"]
        #[inline(always)]
        pub const fn set_uep2_buf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable USB endpoint 2 transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep2_tx_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 2 transmittal (IN)."]
        #[inline(always)]
        pub const fn set_uep2_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable USB endpoint 2 receiving (OUT)"]
        #[must_use]
        #[inline(always)]
        pub const fn uep2_rx_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 2 receiving (OUT)"]
        #[inline(always)]
        pub const fn set_uep2_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "buffer mode of USB endpoint 3"]
        #[must_use]
        #[inline(always)]
        pub const fn uep3_buf_mod(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint 3"]
        #[inline(always)]
        pub const fn set_uep3_buf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable USB endpoint 3 transmittal (IN)"]
        #[must_use]
        #[inline(always)]
        pub const fn uep3_tx_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 3 transmittal (IN)"]
        #[inline(always)]
        pub const fn set_uep3_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "enable USB endpoint 3 receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep3_rx_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 3 receiving (OUT)."]
        #[inline(always)]
        pub const fn set_uep3_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep23Mod {
        #[inline(always)]
        fn default() -> Uep23Mod {
            Uep23Mod(0)
        }
    }
    impl core::fmt::Debug for Uep23Mod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep23Mod")
                .field("uep2_buf_mod", &self.uep2_buf_mod())
                .field("uep2_tx_en", &self.uep2_tx_en())
                .field("uep2_rx_en", &self.uep2_rx_en())
                .field("uep3_buf_mod", &self.uep3_buf_mod())
                .field("uep3_tx_en", &self.uep3_tx_en())
                .field("uep3_rx_en", &self.uep3_rx_en())
                .finish()
        }
    }
    #[doc = "Endpoint 2 control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2Ctrl(pub u8);
    impl Uep2Ctrl {
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uep_t_res(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[inline(always)]
        pub const fn set_mask_uep_t_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uep_r_res(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[inline(always)]
        pub const fn set_mask_uep_r_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 1_2_3: 0=manual toggle, 1=automatic toggle"]
        #[must_use]
        #[inline(always)]
        pub const fn uep_auto_tog(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 1_2_3: 0=manual toggle, 1=automatic toggle"]
        #[inline(always)]
        pub const fn set_uep_auto_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn set_uep_t_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1"]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1"]
        #[inline(always)]
        pub const fn set_uep_r_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep2Ctrl {
        #[inline(always)]
        fn default() -> Uep2Ctrl {
            Uep2Ctrl(0)
        }
    }
    impl core::fmt::Debug for Uep2Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2Ctrl")
                .field("mask_uep_t_res", &self.mask_uep_t_res())
                .field("mask_uep_r_res", &self.mask_uep_r_res())
                .field("uep_auto_tog", &self.uep_auto_tog())
                .field("uep_t_tog", &self.uep_t_tog())
                .field("uep_r_tog", &self.uep_r_tog())
                .finish()
        }
    }
    #[doc = "Endpoint 2 DMA buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2Dma(pub u16);
    impl Uep2Dma {
        #[doc = "Endpoint 2 DMA buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep2_dma(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Endpoint 2 DMA buffer address."]
        #[inline(always)]
        pub const fn set_uep2_dma(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep2Dma {
        #[inline(always)]
        fn default() -> Uep2Dma {
            Uep2Dma(0)
        }
    }
    impl core::fmt::Debug for Uep2Dma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2Dma")
                .field("uep2_dma", &self.uep2_dma())
                .finish()
        }
    }
    #[doc = "Endpoint 2 transmit length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2TLen(pub u8);
    impl Uep2TLen {
        #[doc = "endpoint 2 transmittal length"]
        #[must_use]
        #[inline(always)]
        pub const fn uep2_t_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "endpoint 2 transmittal length"]
        #[inline(always)]
        pub const fn set_uep2_t_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep2TLen {
        #[inline(always)]
        fn default() -> Uep2TLen {
            Uep2TLen(0)
        }
    }
    impl core::fmt::Debug for Uep2TLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2TLen")
                .field("uep2_t_len", &self.uep2_t_len())
                .finish()
        }
    }
    #[doc = "Endpoint 3 control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3Ctrl(pub u8);
    impl Uep3Ctrl {
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uep_t_res(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[inline(always)]
        pub const fn set_mask_uep_t_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uep_r_res(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[inline(always)]
        pub const fn set_mask_uep_r_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 1_2_3: 0=manual toggle, 1=automatic toggle."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_auto_tog(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 1_2_3: 0=manual toggle, 1=automatic toggle."]
        #[inline(always)]
        pub const fn set_uep_auto_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn set_uep_t_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn set_uep_r_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep3Ctrl {
        #[inline(always)]
        fn default() -> Uep3Ctrl {
            Uep3Ctrl(0)
        }
    }
    impl core::fmt::Debug for Uep3Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3Ctrl")
                .field("mask_uep_t_res", &self.mask_uep_t_res())
                .field("mask_uep_r_res", &self.mask_uep_r_res())
                .field("uep_auto_tog", &self.uep_auto_tog())
                .field("uep_t_tog", &self.uep_t_tog())
                .field("uep_r_tog", &self.uep_r_tog())
                .finish()
        }
    }
    #[doc = "Endpoint 3 DMA buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3Dma(pub u16);
    impl Uep3Dma {
        #[doc = "Endpoint 3 DMA buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep3_dma(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Endpoint 3 DMA buffer address."]
        #[inline(always)]
        pub const fn set_uep3_dma(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep3Dma {
        #[inline(always)]
        fn default() -> Uep3Dma {
            Uep3Dma(0)
        }
    }
    impl core::fmt::Debug for Uep3Dma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3Dma")
                .field("uep3_dma", &self.uep3_dma())
                .finish()
        }
    }
    #[doc = "Endpoint 3 transmit length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3TLen(pub u8);
    impl Uep3TLen {
        #[doc = "Endpoint 3 transmit length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep3_t_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Endpoint 3 transmit length."]
        #[inline(always)]
        pub const fn set_uep3_t_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep3TLen {
        #[inline(always)]
        fn default() -> Uep3TLen {
            Uep3TLen(0)
        }
    }
    impl core::fmt::Debug for Uep3TLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3TLen")
                .field("uep3_t_len", &self.uep3_t_len())
                .finish()
        }
    }
    #[doc = "endpoint 4/1 mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep41Mod(pub u8);
    impl Uep41Mod {
        #[doc = "enable USB endpoint 4 transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep4_tx_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 4 transmittal (IN)."]
        #[inline(always)]
        pub const fn set_uep4_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable USB endpoint 4 receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep4_rx_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 4 receiving (OUT)."]
        #[inline(always)]
        pub const fn set_uep4_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "buffer mode of USB endpoint 1."]
        #[must_use]
        #[inline(always)]
        pub const fn uep1_buf_mod(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint 1."]
        #[inline(always)]
        pub const fn set_uep1_buf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable USB endpoint 1 transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep1_tx_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 1 transmittal (IN)."]
        #[inline(always)]
        pub const fn set_uep1_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "enable USB endpoint 1 receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep1_rx_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 1 receiving (OUT)."]
        #[inline(always)]
        pub const fn set_uep1_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep41Mod {
        #[inline(always)]
        fn default() -> Uep41Mod {
            Uep41Mod(0)
        }
    }
    impl core::fmt::Debug for Uep41Mod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep41Mod")
                .field("uep4_tx_en", &self.uep4_tx_en())
                .field("uep4_rx_en", &self.uep4_rx_en())
                .field("uep1_buf_mod", &self.uep1_buf_mod())
                .field("uep1_tx_en", &self.uep1_tx_en())
                .field("uep1_rx_en", &self.uep1_rx_en())
                .finish()
        }
    }
    #[doc = "endpoint 4 control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4Ctrl(pub u8);
    impl Uep4Ctrl {
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uep_t_res(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[inline(always)]
        pub const fn set_mask_uep_t_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uep_r_res(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[inline(always)]
        pub const fn set_mask_uep_r_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "Automatic toggle is documented for endpoints 1, 2 and 3 only."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_auto_tog(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic toggle is documented for endpoints 1, 2 and 3 only."]
        #[inline(always)]
        pub const fn set_uep_auto_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn set_uep_t_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn set_uep_r_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep4Ctrl {
        #[inline(always)]
        fn default() -> Uep4Ctrl {
            Uep4Ctrl(0)
        }
    }
    impl core::fmt::Debug for Uep4Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4Ctrl")
                .field("mask_uep_t_res", &self.mask_uep_t_res())
                .field("mask_uep_r_res", &self.mask_uep_r_res())
                .field("uep_auto_tog", &self.uep_auto_tog())
                .field("uep_t_tog", &self.uep_t_tog())
                .field("uep_r_tog", &self.uep_r_tog())
                .finish()
        }
    }
    #[doc = "endpoint 4 transmittal length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4TLen(pub u8);
    impl Uep4TLen {
        #[doc = "endpoint 4 transmittal length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep4_t_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "endpoint 4 transmittal length."]
        #[inline(always)]
        pub const fn set_uep4_t_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep4TLen {
        #[inline(always)]
        fn default() -> Uep4TLen {
            Uep4TLen(0)
        }
    }
    impl core::fmt::Debug for Uep4TLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4TLen")
                .field("uep4_t_len", &self.uep4_t_len())
                .finish()
        }
    }
    #[doc = "endpoint 5/6/7 mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep567Mod(pub u8);
    impl Uep567Mod {
        #[doc = "enable USB endpoint 5 transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep5_tx_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 5 transmittal (IN)."]
        #[inline(always)]
        pub const fn set_uep5_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable USB endpoint 5 receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep5_rx_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 5 receiving (OUT)."]
        #[inline(always)]
        pub const fn set_uep5_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "enable USB endpoint 6 transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep6_tx_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 6 transmittal (IN)."]
        #[inline(always)]
        pub const fn set_uep6_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable USB endpoint 6 receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep6_rx_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 6 receiving (OUT)."]
        #[inline(always)]
        pub const fn set_uep6_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "enable USB endpoint 7 transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep7_tx_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 7 transmittal (IN)."]
        #[inline(always)]
        pub const fn set_uep7_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable USB endpoint 7 receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep7_rx_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 7 receiving (OUT)."]
        #[inline(always)]
        pub const fn set_uep7_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for Uep567Mod {
        #[inline(always)]
        fn default() -> Uep567Mod {
            Uep567Mod(0)
        }
    }
    impl core::fmt::Debug for Uep567Mod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep567Mod")
                .field("uep5_tx_en", &self.uep5_tx_en())
                .field("uep5_rx_en", &self.uep5_rx_en())
                .field("uep6_tx_en", &self.uep6_tx_en())
                .field("uep6_rx_en", &self.uep6_rx_en())
                .field("uep7_tx_en", &self.uep7_tx_en())
                .field("uep7_rx_en", &self.uep7_rx_en())
                .finish()
        }
    }
    #[doc = "endpoint 5 control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5Ctrl(pub u8);
    impl Uep5Ctrl {
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uep_t_res(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[inline(always)]
        pub const fn set_mask_uep_t_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uep_r_res(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[inline(always)]
        pub const fn set_mask_uep_r_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "Automatic toggle is documented for endpoints 1, 2 and 3 only."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_auto_tog(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic toggle is documented for endpoints 1, 2 and 3 only."]
        #[inline(always)]
        pub const fn set_uep_auto_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn set_uep_t_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn set_uep_r_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep5Ctrl {
        #[inline(always)]
        fn default() -> Uep5Ctrl {
            Uep5Ctrl(0)
        }
    }
    impl core::fmt::Debug for Uep5Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5Ctrl")
                .field("mask_uep_t_res", &self.mask_uep_t_res())
                .field("mask_uep_r_res", &self.mask_uep_r_res())
                .field("uep_auto_tog", &self.uep_auto_tog())
                .field("uep_t_tog", &self.uep_t_tog())
                .field("uep_r_tog", &self.uep_r_tog())
                .finish()
        }
    }
    #[doc = "endpoint 5 DMA buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5Dma(pub u16);
    impl Uep5Dma {
        #[doc = "RW, endpoint 5 DMA buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep5_dma(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RW, endpoint 5 DMA buffer address."]
        #[inline(always)]
        pub const fn set_uep5_dma(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep5Dma {
        #[inline(always)]
        fn default() -> Uep5Dma {
            Uep5Dma(0)
        }
    }
    impl core::fmt::Debug for Uep5Dma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5Dma")
                .field("uep5_dma", &self.uep5_dma())
                .finish()
        }
    }
    #[doc = "endpoint 5 transmittal length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5TLen(pub u8);
    impl Uep5TLen {
        #[doc = "endpoint 5 transmittal length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep5_t_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "endpoint 5 transmittal length."]
        #[inline(always)]
        pub const fn set_uep5_t_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep5TLen {
        #[inline(always)]
        fn default() -> Uep5TLen {
            Uep5TLen(0)
        }
    }
    impl core::fmt::Debug for Uep5TLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5TLen")
                .field("uep5_t_len", &self.uep5_t_len())
                .finish()
        }
    }
    #[doc = "endpoint 6 control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6Ctrl(pub u8);
    impl Uep6Ctrl {
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uep_t_res(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[inline(always)]
        pub const fn set_mask_uep_t_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uep_r_res(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[inline(always)]
        pub const fn set_mask_uep_r_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "Automatic toggle is documented for endpoints 1, 2 and 3 only."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_auto_tog(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic toggle is documented for endpoints 1, 2 and 3 only."]
        #[inline(always)]
        pub const fn set_uep_auto_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn set_uep_t_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn set_uep_r_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep6Ctrl {
        #[inline(always)]
        fn default() -> Uep6Ctrl {
            Uep6Ctrl(0)
        }
    }
    impl core::fmt::Debug for Uep6Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6Ctrl")
                .field("mask_uep_t_res", &self.mask_uep_t_res())
                .field("mask_uep_r_res", &self.mask_uep_r_res())
                .field("uep_auto_tog", &self.uep_auto_tog())
                .field("uep_t_tog", &self.uep_t_tog())
                .field("uep_r_tog", &self.uep_r_tog())
                .finish()
        }
    }
    #[doc = "endpoint 6 DMA buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6Dma(pub u16);
    impl Uep6Dma {
        #[doc = "RW, endpoint 6 DMA buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep6_dma(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RW, endpoint 6 DMA buffer address."]
        #[inline(always)]
        pub const fn set_uep6_dma(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep6Dma {
        #[inline(always)]
        fn default() -> Uep6Dma {
            Uep6Dma(0)
        }
    }
    impl core::fmt::Debug for Uep6Dma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6Dma")
                .field("uep6_dma", &self.uep6_dma())
                .finish()
        }
    }
    #[doc = "endpoint 6 transmittal length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6TLen(pub u8);
    impl Uep6TLen {
        #[doc = "endpoint 6 transmittal length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep6_t_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "endpoint 6 transmittal length."]
        #[inline(always)]
        pub const fn set_uep6_t_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep6TLen {
        #[inline(always)]
        fn default() -> Uep6TLen {
            Uep6TLen(0)
        }
    }
    impl core::fmt::Debug for Uep6TLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6TLen")
                .field("uep6_t_len", &self.uep6_t_len())
                .finish()
        }
    }
    #[doc = "endpoint 7 control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7Ctrl(pub u8);
    impl Uep7Ctrl {
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uep_t_res(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[inline(always)]
        pub const fn set_mask_uep_t_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uep_r_res(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[inline(always)]
        pub const fn set_mask_uep_r_res(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "Automatic toggle is documented for endpoints 1, 2 and 3 only."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_auto_tog(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic toggle is documented for endpoints 1, 2 and 3 only."]
        #[inline(always)]
        pub const fn set_uep_auto_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal (IN): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn set_uep_t_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving (OUT): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn set_uep_r_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep7Ctrl {
        #[inline(always)]
        fn default() -> Uep7Ctrl {
            Uep7Ctrl(0)
        }
    }
    impl core::fmt::Debug for Uep7Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7Ctrl")
                .field("mask_uep_t_res", &self.mask_uep_t_res())
                .field("mask_uep_r_res", &self.mask_uep_r_res())
                .field("uep_auto_tog", &self.uep_auto_tog())
                .field("uep_t_tog", &self.uep_t_tog())
                .field("uep_r_tog", &self.uep_r_tog())
                .finish()
        }
    }
    #[doc = "endpoint 7 DMA buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7Dma(pub u16);
    impl Uep7Dma {
        #[doc = "RW, endpoint 7 DMA buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep7_dma(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RW, endpoint 7 DMA buffer address."]
        #[inline(always)]
        pub const fn set_uep7_dma(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep7Dma {
        #[inline(always)]
        fn default() -> Uep7Dma {
            Uep7Dma(0)
        }
    }
    impl core::fmt::Debug for Uep7Dma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7Dma")
                .field("uep7_dma", &self.uep7_dma())
                .finish()
        }
    }
    #[doc = "endpoint 7 transmittal length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7TLen(pub u8);
    impl Uep7TLen {
        #[doc = "endpoint 7 transmittal length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep7_t_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "endpoint 7 transmittal length."]
        #[inline(always)]
        pub const fn set_uep7_t_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Uep7TLen {
        #[inline(always)]
        fn default() -> Uep7TLen {
            Uep7TLen(0)
        }
    }
    impl core::fmt::Debug for Uep7TLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7TLen")
                .field("uep7_t_len", &self.uep7_t_len())
                .finish()
        }
    }
    #[doc = "Device endpoint transmit length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UepTLen(pub u8);
    impl UepTLen {
        #[doc = "Transmit length in bytes."]
        #[must_use]
        #[inline(always)]
        pub const fn t_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Transmit length in bytes."]
        #[inline(always)]
        pub const fn set_t_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
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
    #[doc = "Host endpoint mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhEpMod(pub u8);
    impl UhEpMod {
        #[doc = "buffer mode of USB host IN endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_ep_rbuf_mod(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB host IN endpoint."]
        #[inline(always)]
        pub const fn set_uh_ep_rbuf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable USB host IN endpoint receiving."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_ep_rx_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB host IN endpoint receiving."]
        #[inline(always)]
        pub const fn set_uh_ep_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "buffer mode of USB host OUT endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_ep_tbuf_mod(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB host OUT endpoint."]
        #[inline(always)]
        pub const fn set_uh_ep_tbuf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable USB host OUT endpoint transmittal."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_ep_tx_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB host OUT endpoint transmittal."]
        #[inline(always)]
        pub const fn set_uh_ep_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for UhEpMod {
        #[inline(always)]
        fn default() -> UhEpMod {
            UhEpMod(0)
        }
    }
    impl core::fmt::Debug for UhEpMod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhEpMod")
                .field("uh_ep_rbuf_mod", &self.uh_ep_rbuf_mod())
                .field("uh_ep_rx_en", &self.uh_ep_rx_en())
                .field("uh_ep_tbuf_mod", &self.uh_ep_tbuf_mod())
                .field("uh_ep_tx_en", &self.uh_ep_tx_en())
                .finish()
        }
    }
    #[doc = "Host endpoint and token PID."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhEpPid(pub u8);
    impl UhEpPid {
        #[doc = "bit mask of endpoint number for USB host transfer."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uh_endp(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "bit mask of endpoint number for USB host transfer."]
        #[inline(always)]
        pub const fn set_mask_uh_endp(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "bit mask of token PID for USB host transfer."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uh_token(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "bit mask of token PID for USB host transfer."]
        #[inline(always)]
        pub const fn set_mask_uh_token(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
        }
    }
    impl Default for UhEpPid {
        #[inline(always)]
        fn default() -> UhEpPid {
            UhEpPid(0)
        }
    }
    impl core::fmt::Debug for UhEpPid {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhEpPid")
                .field("mask_uh_endp", &self.mask_uh_endp())
                .field("mask_uh_token", &self.mask_uh_token())
                .finish()
        }
    }
    #[doc = "Host receive endpoint control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhRxCtrl(pub u8);
    impl UhRxCtrl {
        #[doc = "Host receive handshake response control."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_uh_r_res(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Host receive handshake response control."]
        #[inline(always)]
        pub const fn set_mask_uh_r_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion: 0=manual toggle, 1=automatic toggle."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_r_auto_tog(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion: 0=manual toggle, 1=automatic toggle."]
        #[inline(always)]
        pub const fn set_uh_r_auto_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "expected data toggle flag of host receiving (IN): 0=DATA0, 1=DATA1."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_r_tog(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "expected data toggle flag of host receiving (IN): 0=DATA0, 1=DATA1."]
        #[inline(always)]
        pub const fn set_uh_r_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhRxCtrl {
        #[inline(always)]
        fn default() -> UhRxCtrl {
            UhRxCtrl(0)
        }
    }
    impl core::fmt::Debug for UhRxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhRxCtrl")
                .field("mask_uh_r_res", &self.mask_uh_r_res())
                .field("uh_r_auto_tog", &self.uh_r_auto_tog())
                .field("uh_r_tog", &self.uh_r_tog())
                .finish()
        }
    }
    #[doc = "Host receive endpoint DMA buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhRxDma(pub u16);
    impl UhRxDma {
        #[doc = "Host receive endpoint DMA buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rx_dma(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Host receive endpoint DMA buffer address."]
        #[inline(always)]
        pub const fn set_uh_rx_dma(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
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
    #[doc = "Host auxiliary setup."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhSetup(pub u8);
    impl UhSetup {
        #[doc = "USB host automatic SOF enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_sof_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "USB host automatic SOF enable."]
        #[inline(always)]
        pub const fn set_uh_sof_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "USB host PRE PID enable for low speed device via hub."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_pre_pid_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "USB host PRE PID enable for low speed device via hub."]
        #[inline(always)]
        pub const fn set_uh_pre_pid_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhSetup {
        #[inline(always)]
        fn default() -> UhSetup {
            UhSetup(0)
        }
    }
    impl core::fmt::Debug for UhSetup {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhSetup")
                .field("uh_sof_en", &self.uh_sof_en())
                .field("uh_pre_pid_en", &self.uh_pre_pid_en())
                .finish()
        }
    }
    #[doc = "Host transmit endpoint control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhTxCtrl(pub u8);
    impl UhTxCtrl {
        #[doc = "Host transmit handshake response control."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_t_res(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Host transmit handshake response control."]
        #[inline(always)]
        pub const fn set_uh_t_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Enable automatic toggle after successful host transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_t_auto_tog(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable automatic toggle after successful host transmission."]
        #[inline(always)]
        pub const fn set_uh_t_auto_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Prepared host transmit data toggle flag."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_t_tog(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Prepared host transmit data toggle flag."]
        #[inline(always)]
        pub const fn set_uh_t_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for UhTxCtrl {
        #[inline(always)]
        fn default() -> UhTxCtrl {
            UhTxCtrl(0)
        }
    }
    impl core::fmt::Debug for UhTxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhTxCtrl")
                .field("uh_t_res", &self.uh_t_res())
                .field("uh_t_auto_tog", &self.uh_t_auto_tog())
                .field("uh_t_tog", &self.uh_t_tog())
                .finish()
        }
    }
    #[doc = "Host transmit endpoint DMA buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhTxDma(pub u16);
    impl UhTxDma {
        #[doc = "Host transmit endpoint DMA buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tx_dma(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Host transmit endpoint DMA buffer address."]
        #[inline(always)]
        pub const fn set_uh_tx_dma(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
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
                .field("uh_tx_dma", &self.uh_tx_dma())
                .finish()
        }
    }
    #[doc = "Host transmit length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhTxLen(pub u8);
    impl UhTxLen {
        #[doc = "Host transmit length."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tx_len(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Host transmit length."]
        #[inline(always)]
        pub const fn set_uh_tx_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
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
                .field("uh_tx_len", &self.uh_tx_len())
                .finish()
        }
    }
    #[doc = "USB host physical port control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhostCtrl(pub u8);
    impl UhostCtrl {
        #[doc = "enable USB port: 0=disable, 1=enable port, automatic disabled if USB device detached."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_port_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB port: 0=disable, 1=enable port, automatic disabled if USB device detached."]
        #[inline(always)]
        pub const fn set_uh_port_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "control USB bus reset: 0=normal, 1=force bus reset."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_bus_reset(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "control USB bus reset: 0=normal, 1=force bus reset."]
        #[inline(always)]
        pub const fn set_uh_bus_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "enable USB port low speed: 0=full speed, 1=low speed."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_low_speed(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB port low speed: 0=full speed, 1=low speed."]
        #[inline(always)]
        pub const fn set_uh_low_speed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "ReadOnly: indicate current UDM pin level."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_dm_pin(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "ReadOnly: indicate current UDM pin level."]
        #[inline(always)]
        pub const fn set_uh_dm_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "ReadOnly: indicate current UDP pin level."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_dp_pin(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "ReadOnly: indicate current UDP pin level."]
        #[inline(always)]
        pub const fn set_uh_dp_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "disable USB UDP-UDM pulldown resistance: 0=enable pulldown, 1=disable."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_pd_dis(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "disable USB UDP-UDM pulldown resistance: 0=enable pulldown, 1=disable."]
        #[inline(always)]
        pub const fn set_uh_pd_dis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for UhostCtrl {
        #[inline(always)]
        fn default() -> UhostCtrl {
            UhostCtrl(0)
        }
    }
    impl core::fmt::Debug for UhostCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("UhostCtrl")
                .field("uh_port_en", &self.uh_port_en())
                .field("uh_bus_reset", &self.uh_bus_reset())
                .field("uh_low_speed", &self.uh_low_speed())
                .field("uh_dm_pin", &self.uh_dm_pin())
                .field("uh_dp_pin", &self.uh_dp_pin())
                .field("uh_pd_dis", &self.uh_pd_dis())
                .finish()
        }
    }
}
