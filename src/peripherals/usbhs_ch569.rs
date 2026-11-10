#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "USBHS register."]
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
    #[doc = "USB base control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USB host control register."]
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
    #[doc = "USB frame number register."]
    #[inline(always)]
    pub const fn frame_no(self) -> crate::common::Reg<regs::FrameNo, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "USB suspend register."]
    #[inline(always)]
    pub const fn suspend(self) -> crate::common::Reg<regs::Suspend, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "USB actual speed register."]
    #[inline(always)]
    pub const fn spd_type(self) -> crate::common::Reg<regs::SpdType, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "USB miscellaneous status."]
    #[inline(always)]
    pub const fn mis_st(self) -> crate::common::Reg<regs::MisSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "USB interrupt flag."]
    #[inline(always)]
    pub const fn int_fg(self) -> crate::common::Reg<regs::IntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "USB interrupt status."]
    #[inline(always)]
    pub const fn int_st(self) -> crate::common::Reg<regs::IntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0busize) as _) }
    }
    #[doc = "USB receiving length."]
    #[inline(always)]
    pub const fn r6_usb_rx_len(self) -> crate::common::Reg<regs::R6UsbRxLen, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "endpoint 1(9) 4(8,12) mode."]
    #[inline(always)]
    pub const fn uep4_1_mod(self) -> crate::common::Reg<regs::Uep41Mod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "endpoint 2(10) 3(11) mode and USB host endpoint mode control register."]
    #[inline(always)]
    pub const fn uep2_3_mod_uh_ep_mod(
        self,
    ) -> crate::common::Reg<regs::Uep23ModUhEpMod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x11usize) as _) }
    }
    #[doc = "endpoint 5(13) 6(14) mode."]
    #[inline(always)]
    pub const fn uep5_6_mod(self) -> crate::common::Reg<regs::Uep56Mod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x12usize) as _) }
    }
    #[doc = "endpoint 7(15) mode."]
    #[inline(always)]
    pub const fn uep7_mod(self) -> crate::common::Reg<regs::Uep7Mod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13usize) as _) }
    }
    #[doc = "endpoint 0 DMA buffer address."]
    #[inline(always)]
    pub const fn uep0_rt_dma(self) -> crate::common::Reg<regs::Uep0RtDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "endpoint 1 DMA buffer address."]
    #[inline(always)]
    pub const fn uep1_rx_dma(self) -> crate::common::Reg<regs::Uep1RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "endpoint 2 DMA buffer address _ host rx endpoint buffer start address."]
    #[inline(always)]
    pub const fn uep2_rx_dma_uh_rx_dma(
        self,
    ) -> crate::common::Reg<regs::Uep2RxDmaUhRxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "endpoint 3 DMA buffer address;host tx endpoint buffer high address."]
    #[inline(always)]
    pub const fn uep3_rx_dma(self) -> crate::common::Reg<regs::Uep3RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "endpoint 4 DMA buffer address."]
    #[inline(always)]
    pub const fn uep4_rx_dma(self) -> crate::common::Reg<regs::Uep4RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "endpoint 5 DMA buffer address."]
    #[inline(always)]
    pub const fn uep5_rx_dma(self) -> crate::common::Reg<regs::Uep5RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "endpoint 6 DMA buffer address."]
    #[inline(always)]
    pub const fn uep6_rx_dma(self) -> crate::common::Reg<regs::Uep6RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "endpoint 7 DMA buffer address."]
    #[inline(always)]
    pub const fn uep7_rx_dma(self) -> crate::common::Reg<regs::Uep7RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "endpoint 1 DMA TX buffer address."]
    #[inline(always)]
    pub const fn uep1_tx_dma(self) -> crate::common::Reg<regs::Uep1TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "endpoint 2 DMA TX buffer address."]
    #[inline(always)]
    pub const fn uep2_tx_dma(self) -> crate::common::Reg<regs::Uep2TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "endpoint 3 DMA TX buffer address and host tx endpoint buffer start address."]
    #[inline(always)]
    pub const fn uep3_tx_dma_uh_tx_dma(
        self,
    ) -> crate::common::Reg<regs::Uep3TxDmaUhTxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "endpoint 4 DMA TX buffer address."]
    #[inline(always)]
    pub const fn uep4_tx_dma(self) -> crate::common::Reg<regs::Uep4TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "endpoint 5 DMA TX buffer address."]
    #[inline(always)]
    pub const fn uep5_tx_dma(self) -> crate::common::Reg<regs::Uep5TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "endpoint 4 DMA TX buffer address."]
    #[inline(always)]
    pub const fn uep6_tx_dma(self) -> crate::common::Reg<regs::Uep6TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "endpoint 7 DMA TX buffer address."]
    #[inline(always)]
    pub const fn uep7_tx_dma(self) -> crate::common::Reg<regs::Uep7TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "endpoint 0 receive max length."]
    #[inline(always)]
    pub const fn uep0_max_len(self) -> crate::common::Reg<regs::Uep0MaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "endpoint 1 receive max length."]
    #[inline(always)]
    pub const fn uep1_max_len(self) -> crate::common::Reg<regs::Uep1MaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "endpoint 2 receive max length and USB host receive max packet length register."]
    #[inline(always)]
    pub const fn uep2_max_len_uh_max_len(
        self,
    ) -> crate::common::Reg<regs::Uep2MaxLenUhMaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "endpoint 3 receive max length."]
    #[inline(always)]
    pub const fn uep3_max_len(self) -> crate::common::Reg<regs::Uep3MaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "endpoint 4 receive max length."]
    #[inline(always)]
    pub const fn uep4_max_len(self) -> crate::common::Reg<regs::Uep4MaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "endpoint 5 receive max length."]
    #[inline(always)]
    pub const fn uep5_max_len(self) -> crate::common::Reg<regs::Uep5MaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "endpoint 6 receive max length."]
    #[inline(always)]
    pub const fn uep6_max_len(self) -> crate::common::Reg<regs::Uep6MaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "endpoint 7 receive max length."]
    #[inline(always)]
    pub const fn uep7_max_len(self) -> crate::common::Reg<regs::Uep7MaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "endpoint 0 transmittal length."]
    #[inline(always)]
    pub const fn uep0_t_len(self) -> crate::common::Reg<regs::Uep0TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "endpoint 0 tx control."]
    #[inline(always)]
    pub const fn uep0_tx_ctrl(self) -> crate::common::Reg<regs::Uep0TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x72usize) as _) }
    }
    #[doc = "endpoint 0 rx control."]
    #[inline(always)]
    pub const fn uep0_rx_ctrl(self) -> crate::common::Reg<regs::Uep0RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x73usize) as _) }
    }
    #[doc = "endpoint 1 transmittal length."]
    #[inline(always)]
    pub const fn uep1_t_len(self) -> crate::common::Reg<regs::Uep1TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "endpoint 1 tx control."]
    #[inline(always)]
    pub const fn uep1_tx_ctrl(self) -> crate::common::Reg<regs::Uep1TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x76usize) as _) }
    }
    #[doc = "endpoint 1 rx control."]
    #[inline(always)]
    pub const fn uep1_rx_ctrl(self) -> crate::common::Reg<regs::Uep1RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x77usize) as _) }
    }
    #[doc = "endpoint 2 transmittal length and Set usb host token register."]
    #[inline(always)]
    pub const fn uep2_t_len_uh_ep_pid(
        self,
    ) -> crate::common::Reg<regs::Uep2TLenUhEpPid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "endpoint 2 tx control."]
    #[inline(always)]
    pub const fn uep2_tx_ctrl(self) -> crate::common::Reg<regs::Uep2TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7ausize) as _) }
    }
    #[doc = "endpoint 2 rx control and USb host receive endpoint control register."]
    #[inline(always)]
    pub const fn uep2_rx_ctrl_uh_rx_ctrl(
        self,
    ) -> crate::common::Reg<regs::Uep2RxCtrlUhRxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7busize) as _) }
    }
    #[doc = "endpoint 3 transmittal length and host transmittal endpoint transmittal length."]
    #[inline(always)]
    pub const fn uep3_t_len_uh_tx_len(
        self,
    ) -> crate::common::Reg<regs::Uep3TLenUhTxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "endpoint 3 tx control and host transmittal endpoint control."]
    #[inline(always)]
    pub const fn uep3_tx_ctrl_uh_tx_ctrl(
        self,
    ) -> crate::common::Reg<regs::Uep3TxCtrlUhTxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7eusize) as _) }
    }
    #[doc = "endpoint 3 rx control."]
    #[inline(always)]
    pub const fn uep3_rx_ctrl(self) -> crate::common::Reg<regs::Uep3RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7fusize) as _) }
    }
    #[doc = "endpoint 4 transmittal length and USB host Tx SPLIT packet data."]
    #[inline(always)]
    pub const fn uep4_t_len_uh_split_data(
        self,
    ) -> crate::common::Reg<regs::Uep4TLenUhSplitData, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "endpoint 4 tx control."]
    #[inline(always)]
    pub const fn uep4_tx_ctrl(self) -> crate::common::Reg<regs::Uep4TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x82usize) as _) }
    }
    #[doc = "endpoint 4 rx control."]
    #[inline(always)]
    pub const fn uep4_rx_ctrl(self) -> crate::common::Reg<regs::Uep4RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x83usize) as _) }
    }
    #[doc = "endpoint 5 transmittal length."]
    #[inline(always)]
    pub const fn uep5_t_len(self) -> crate::common::Reg<regs::Uep5TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "endpoint 5 tx control."]
    #[inline(always)]
    pub const fn uep5_tx_ctrl(self) -> crate::common::Reg<regs::Uep5TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x86usize) as _) }
    }
    #[doc = "endpoint 5 rx control."]
    #[inline(always)]
    pub const fn uep5_rx_ctrl(self) -> crate::common::Reg<regs::Uep5RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x87usize) as _) }
    }
    #[doc = "endpoint 6 transmittal length."]
    #[inline(always)]
    pub const fn uep6_t_len(self) -> crate::common::Reg<regs::Uep6TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "endpoint 6 tx control."]
    #[inline(always)]
    pub const fn uep6_tx_ctrl(self) -> crate::common::Reg<regs::Uep6TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8ausize) as _) }
    }
    #[doc = "endpoint 6 rx control."]
    #[inline(always)]
    pub const fn uep6_rx_ctrl(self) -> crate::common::Reg<regs::Uep6RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8busize) as _) }
    }
    #[doc = "endpoint 7 transmittal length."]
    #[inline(always)]
    pub const fn uep7_t_len(self) -> crate::common::Reg<regs::Uep7TLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "endpoint 7 tx control."]
    #[inline(always)]
    pub const fn uep7_tx_ctrl(self) -> crate::common::Reg<regs::Uep7TxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8eusize) as _) }
    }
    #[doc = "endpoint 7 rx control."]
    #[inline(always)]
    pub const fn uep7_rx_ctrl(self) -> crate::common::Reg<regs::Uep7RxCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8fusize) as _) }
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
        pub const fn dma_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA enable and DMA interrupt enable for USB."]
        #[inline(always)]
        pub const fn set_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "force clear FIFO and count of USB."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_all(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force clear FIFO and count of USB."]
        #[inline(always)]
        pub const fn set_clr_all(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "force reset USB SIE, need software clear."]
        #[must_use]
        #[inline(always)]
        pub const fn reset_sie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "force reset USB SIE, need software clear."]
        #[inline(always)]
        pub const fn set_reset_sie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid."]
        #[must_use]
        #[inline(always)]
        pub const fn int_busy(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic responding busy for device mode or automatic pause for host mode during interrupt flag UIF_TRANSFER valid."]
        #[inline(always)]
        pub const fn set_int_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB device enable and internal pullup resistance enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dev_pu_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB device enable and internal pullup resistance enable."]
        #[inline(always)]
        pub const fn set_dev_pu_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable USB low speed."]
        #[must_use]
        #[inline(always)]
        pub const fn sptp_mask(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "enable USB low speed."]
        #[inline(always)]
        pub const fn set_sptp_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u8) & 0x03) << 5usize);
        }
        #[doc = "enable USB host mode: 0=device mode, 1=host mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB host mode: 0=device mode, 1=host mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
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
                .field("dma_en", &self.dma_en())
                .field("clr_all", &self.clr_all())
                .field("reset_sie", &self.reset_sie())
                .field("int_busy", &self.int_busy())
                .field("dev_pu_en", &self.dev_pu_en())
                .field("sptp_mask", &self.sptp_mask())
                .field("mode", &self.mode())
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
        pub const fn addr_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "bit mask for USB device address."]
        #[inline(always)]
        pub const fn set_addr_mask(&mut self, val: u8) {
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
                .field("addr_mask", &self.addr_mask())
                .finish()
        }
    }
    #[doc = "USB frame number register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FrameNo(pub u16);
    impl FrameNo {
        #[doc = "USB frame number."]
        #[must_use]
        #[inline(always)]
        pub const fn frame_no(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "USB frame number."]
        #[inline(always)]
        pub const fn set_frame_no(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for FrameNo {
        #[inline(always)]
        fn default() -> FrameNo {
            FrameNo(0)
        }
    }
    impl core::fmt::Debug for FrameNo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FrameNo")
                .field("frame_no", &self.frame_no())
                .finish()
        }
    }
    #[doc = "USB interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u8);
    impl IntEn {
        #[doc = "enable interrupt for USB bus reset event for USB device mode _ enable interrupt for USB device detected event for USB host mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_busrst_rb_ie_detect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for USB bus reset event for USB device mode _ enable interrupt for USB device detected event for USB host mode."]
        #[inline(always)]
        pub const fn set_ie_busrst_rb_ie_detect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable interrupt for USB transfer completion."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_trans(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for USB transfer completion."]
        #[inline(always)]
        pub const fn set_ie_trans(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "enable interrupt for USB suspend or resume event."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_suspend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for USB suspend or resume event."]
        #[inline(always)]
        pub const fn set_ie_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable interrupt for host SOF timer action for USB host mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_sof(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for host SOF timer action for USB host mode."]
        #[inline(always)]
        pub const fn set_ie_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "enable interrupt for FIFO overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_fifoov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for FIFO overflow."]
        #[inline(always)]
        pub const fn set_ie_fifoov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Setup packet end interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_setupact(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Setup packet end interrupt."]
        #[inline(always)]
        pub const fn set_ie_setupact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Synchronous transmission received control token packet interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_isoact(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronous transmission received control token packet interrupt."]
        #[inline(always)]
        pub const fn set_ie_isoact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "enable interrupt for NAK responded for USB device mode."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_dev_nak(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for NAK responded for USB device mode."]
        #[inline(always)]
        pub const fn set_ie_dev_nak(&mut self, val: bool) {
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
                .field("ie_busrst_rb_ie_detect", &self.ie_busrst_rb_ie_detect())
                .field("ie_trans", &self.ie_trans())
                .field("ie_suspend", &self.ie_suspend())
                .field("ie_sof", &self.ie_sof())
                .field("ie_fifoov", &self.ie_fifoov())
                .field("ie_setupact", &self.ie_setupact())
                .field("ie_isoact", &self.ie_isoact())
                .field("ie_dev_nak", &self.ie_dev_nak())
                .finish()
        }
    }
    #[doc = "USB interrupt flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFg(pub u8);
    impl IntFg {
        #[doc = "bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn if_busrst_rb_if_detect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "bus reset event interrupt flag for USB device mode, direct bit address clear or write 1 to clear;device detected event interrupt flag for USB host mode, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn set_if_busrst_rb_if_detect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB transfer completion interrupt flag, direct bit address clear or write 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn if_transfer(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB transfer completion interrupt flag, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn set_if_transfer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn if_suspend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB suspend or resume event interrupt flag, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn set_if_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn if_hst_sof(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "host SOF timer interrupt flag for USB host, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn set_if_hst_sof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear."]
        #[must_use]
        #[inline(always)]
        pub const fn if_fifoov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overflow interrupt flag for USB, direct bit address clear or write 1 to clear."]
        #[inline(always)]
        pub const fn set_if_fifoov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RO, Setup transaction end interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn if_setuoact(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RO, Setup transaction end interrupt flag."]
        #[inline(always)]
        pub const fn set_if_setuoact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RO, Synchronous transmission received control token packet interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn if_isoact(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RO, Synchronous transmission received control token packet interrupt flag."]
        #[inline(always)]
        pub const fn set_if_isoact(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
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
                .field("if_busrst_rb_if_detect", &self.if_busrst_rb_if_detect())
                .field("if_transfer", &self.if_transfer())
                .field("if_suspend", &self.if_suspend())
                .field("if_hst_sof", &self.if_hst_sof())
                .field("if_fifoov", &self.if_fifoov())
                .field("if_setuoact", &self.if_setuoact())
                .field("if_isoact", &self.if_isoact())
                .finish()
        }
    }
    #[doc = "USB interrupt status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntSt(pub u8);
    impl IntSt {
        #[doc = "RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode."]
        #[must_use]
        #[inline(always)]
        pub const fn host_res_mask_rb_dev_endp_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RO, bit mask of current transfer handshake response for USB host mode: 0000=no response, time out from device, others=handshake response PID received;RO, bit mask of current transfer endpoint number for USB device mode."]
        #[inline(always)]
        pub const fn set_host_res_mask_rb_dev_endp_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "RO, bit mask of current token PID code received for USB device mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dev_token_mask(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "RO, bit mask of current token PID code received for USB device mode."]
        #[inline(always)]
        pub const fn set_dev_token_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "RO, indicate current USB transfer toggle is OK."]
        #[must_use]
        #[inline(always)]
        pub const fn st_togok(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate current USB transfer toggle is OK."]
        #[inline(always)]
        pub const fn set_st_togok(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RO, indicate current USB transfer is NAK received for USB device mode."]
        #[must_use]
        #[inline(always)]
        pub const fn st_nak(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate current USB transfer is NAK received for USB device mode."]
        #[inline(always)]
        pub const fn set_st_nak(&mut self, val: bool) {
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
                .field(
                    "host_res_mask_rb_dev_endp_mask",
                    &self.host_res_mask_rb_dev_endp_mask(),
                )
                .field("dev_token_mask", &self.dev_token_mask())
                .field("st_togok", &self.st_togok())
                .field("st_nak", &self.st_nak())
                .finish()
        }
    }
    #[doc = "USB miscellaneous status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MisSt(pub u8);
    impl MisSt {
        #[doc = "RO,indicate host allow SPLIT packet."]
        #[must_use]
        #[inline(always)]
        pub const fn split_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RO,indicate host allow SPLIT packet."]
        #[inline(always)]
        pub const fn set_split_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RO, indicate device attached status on USB host."]
        #[must_use]
        #[inline(always)]
        pub const fn attach(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate device attached status on USB host."]
        #[inline(always)]
        pub const fn set_attach(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RO, indicate USB suspend status."]
        #[must_use]
        #[inline(always)]
        pub const fn usbbus_suspend(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate USB suspend status."]
        #[inline(always)]
        pub const fn set_usbbus_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RO, indicate USB bus reset status."]
        #[must_use]
        #[inline(always)]
        pub const fn usbbus_reset(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate USB bus reset status."]
        #[inline(always)]
        pub const fn set_usbbus_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RO, indicate USB receiving FIFO ready status (not empty)."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_rdy(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate USB receiving FIFO ready status (not empty)."]
        #[inline(always)]
        pub const fn set_fifo_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RO, indicate USB SIE free status."]
        #[must_use]
        #[inline(always)]
        pub const fn sie_free(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate USB SIE free status."]
        #[inline(always)]
        pub const fn set_sie_free(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RO, indicate host SOF timer action status for USB host."]
        #[must_use]
        #[inline(always)]
        pub const fn sof_act(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate host SOF timer action status for USB host."]
        #[inline(always)]
        pub const fn set_sof_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RO, indicate host SOF timer presage status."]
        #[must_use]
        #[inline(always)]
        pub const fn sof_pres(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, indicate host SOF timer presage status."]
        #[inline(always)]
        pub const fn set_sof_pres(&mut self, val: bool) {
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
                .field("split_en", &self.split_en())
                .field("attach", &self.attach())
                .field("usbbus_suspend", &self.usbbus_suspend())
                .field("usbbus_reset", &self.usbbus_reset())
                .field("fifo_rdy", &self.fifo_rdy())
                .field("sie_free", &self.sie_free())
                .field("sof_act", &self.sof_act())
                .field("sof_pres", &self.sof_pres())
                .finish()
        }
    }
    #[doc = "USB receiving length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct R6UsbRxLen(pub u16);
    impl R6UsbRxLen {
        #[doc = "length of received bytes."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "length of received bytes."]
        #[inline(always)]
        pub const fn set_rx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for R6UsbRxLen {
        #[inline(always)]
        fn default() -> R6UsbRxLen {
            R6UsbRxLen(0)
        }
    }
    impl core::fmt::Debug for R6UsbRxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("R6UsbRxLen")
                .field("rx_len", &self.rx_len())
                .finish()
        }
    }
    #[doc = "USB actual speed register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SpdType(pub u8);
    impl SpdType {
        #[doc = "USB actual speed."]
        #[must_use]
        #[inline(always)]
        pub const fn usbspeed_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "USB actual speed."]
        #[inline(always)]
        pub const fn set_usbspeed_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
    }
    impl Default for SpdType {
        #[inline(always)]
        fn default() -> SpdType {
            SpdType(0)
        }
    }
    impl core::fmt::Debug for SpdType {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SpdType")
                .field("usbspeed_mask", &self.usbspeed_mask())
                .finish()
        }
    }
    #[doc = "USB suspend register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Suspend(pub u8);
    impl Suspend {
        #[doc = "Remote wake-up control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dev_wakeup(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Remote wake-up control bit."]
        #[inline(always)]
        pub const fn set_dev_wakeup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
    }
    impl Default for Suspend {
        #[inline(always)]
        fn default() -> Suspend {
            Suspend(0)
        }
    }
    impl core::fmt::Debug for Suspend {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Suspend")
                .field("dev_wakeup", &self.dev_wakeup())
                .finish()
        }
    }
    #[doc = "endpoint 0 receive max length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0MaxLen(pub u16);
    impl Uep0MaxLen {
        #[doc = "endpoint 0 receive max length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep0_max_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 0 receive max length."]
        #[inline(always)]
        pub const fn set_uep0_max_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
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
    #[doc = "endpoint 0 DMA buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0RtDma(pub u32);
    impl Uep0RtDma {
        #[doc = "endpoint 0 DMA buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep0_rt_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "endpoint 0 DMA buffer address."]
        #[inline(always)]
        pub const fn set_uep0_rt_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for Uep0RtDma {
        #[inline(always)]
        fn default() -> Uep0RtDma {
            Uep0RtDma(0)
        }
    }
    impl core::fmt::Debug for Uep0RtDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep0RtDma")
                .field("uep0_rt_dma", &self.uep0_rt_dma())
                .finish()
        }
    }
    #[doc = "endpoint 0 rx control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0RxCtrl(pub u8);
    impl Uep0RxCtrl {
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rres_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[inline(always)]
        pub const fn set_uep_rres_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "prepared no response."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rres_no(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "prepared no response."]
        #[inline(always)]
        pub const fn set_uep_rres_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog_mask(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving."]
        #[inline(always)]
        pub const fn set_uep_r_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_autotog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint."]
        #[inline(always)]
        pub const fn set_uep_r_autotog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
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
                .field("uep_rres_mask", &self.uep_rres_mask())
                .field("uep_rres_no", &self.uep_rres_no())
                .field("uep_r_tog_mask", &self.uep_r_tog_mask())
                .field("uep_r_autotog", &self.uep_r_autotog())
                .finish()
        }
    }
    #[doc = "endpoint 0 transmittal length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0TLen(pub u16);
    impl Uep0TLen {
        #[doc = "endpoint 0 transmittal length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep0_t_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 0 transmittal length."]
        #[inline(always)]
        pub const fn set_uep0_t_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
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
    #[doc = "endpoint 0 tx control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep0TxCtrl(pub u8);
    impl Uep0TxCtrl {
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tres_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[inline(always)]
        pub const fn set_uep_tres_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "expected no response."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tres_no(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "expected no response."]
        #[inline(always)]
        pub const fn set_uep_tres_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog_mask(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal."]
        #[inline(always)]
        pub const fn set_uep_t_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 0."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_autotog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 0."]
        #[inline(always)]
        pub const fn set_uep_t_autotog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
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
                .field("uep_tres_mask", &self.uep_tres_mask())
                .field("uep_tres_no", &self.uep_tres_no())
                .field("uep_t_tog_mask", &self.uep_t_tog_mask())
                .field("uep_t_autotog", &self.uep_t_autotog())
                .finish()
        }
    }
    #[doc = "endpoint 1 receive max length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1MaxLen(pub u16);
    impl Uep1MaxLen {
        #[doc = "endpoint 1 receive max length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep1_max_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 1 receive max length."]
        #[inline(always)]
        pub const fn set_uep1_max_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep1MaxLen {
        #[inline(always)]
        fn default() -> Uep1MaxLen {
            Uep1MaxLen(0)
        }
    }
    impl core::fmt::Debug for Uep1MaxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1MaxLen")
                .field("uep1_max_len", &self.uep1_max_len())
                .finish()
        }
    }
    #[doc = "endpoint 1 rx control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1RxCtrl(pub u8);
    impl Uep1RxCtrl {
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rres_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[inline(always)]
        pub const fn set_uep_rres_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "prepared no response."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rres_no(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "prepared no response."]
        #[inline(always)]
        pub const fn set_uep_rres_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog_mask(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving."]
        #[inline(always)]
        pub const fn set_uep_r_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_autotog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint."]
        #[inline(always)]
        pub const fn set_uep_r_autotog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for Uep1RxCtrl {
        #[inline(always)]
        fn default() -> Uep1RxCtrl {
            Uep1RxCtrl(0)
        }
    }
    impl core::fmt::Debug for Uep1RxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1RxCtrl")
                .field("uep_rres_mask", &self.uep_rres_mask())
                .field("uep_rres_no", &self.uep_rres_no())
                .field("uep_r_tog_mask", &self.uep_r_tog_mask())
                .field("uep_r_autotog", &self.uep_r_autotog())
                .finish()
        }
    }
    #[doc = "endpoint 1 DMA buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1RxDma(pub u32);
    impl Uep1RxDma {
        #[doc = "endpoint 1 DMA buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep1_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "endpoint 1 DMA buffer address."]
        #[inline(always)]
        pub const fn set_uep1_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for Uep1RxDma {
        #[inline(always)]
        fn default() -> Uep1RxDma {
            Uep1RxDma(0)
        }
    }
    impl core::fmt::Debug for Uep1RxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1RxDma")
                .field("uep1_rx_dma", &self.uep1_rx_dma())
                .finish()
        }
    }
    #[doc = "endpoint 1 transmittal length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1TLen(pub u16);
    impl Uep1TLen {
        #[doc = "endpoint 1 transmittal length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep1_t_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 1 transmittal length."]
        #[inline(always)]
        pub const fn set_uep1_t_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
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
    #[doc = "endpoint 1 tx control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1TxCtrl(pub u8);
    impl Uep1TxCtrl {
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tres_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[inline(always)]
        pub const fn set_uep_tres_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "expected no response."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tres_no(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "expected no response."]
        #[inline(always)]
        pub const fn set_uep_tres_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog_mask(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal."]
        #[inline(always)]
        pub const fn set_uep_t_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 0."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_autotog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 0."]
        #[inline(always)]
        pub const fn set_uep_t_autotog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for Uep1TxCtrl {
        #[inline(always)]
        fn default() -> Uep1TxCtrl {
            Uep1TxCtrl(0)
        }
    }
    impl core::fmt::Debug for Uep1TxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1TxCtrl")
                .field("uep_tres_mask", &self.uep_tres_mask())
                .field("uep_tres_no", &self.uep_tres_no())
                .field("uep_t_tog_mask", &self.uep_t_tog_mask())
                .field("uep_t_autotog", &self.uep_t_autotog())
                .finish()
        }
    }
    #[doc = "endpoint 1 DMA TX buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep1TxDma(pub u32);
    impl Uep1TxDma {
        #[doc = "endpoint 1 DMA TX buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep1_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "endpoint 1 DMA TX buffer address."]
        #[inline(always)]
        pub const fn set_uep1_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for Uep1TxDma {
        #[inline(always)]
        fn default() -> Uep1TxDma {
            Uep1TxDma(0)
        }
    }
    impl core::fmt::Debug for Uep1TxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep1TxDma")
                .field("uep1_tx_dma", &self.uep1_tx_dma())
                .finish()
        }
    }
    #[doc = "endpoint 2(10) 3(11) mode and USB host endpoint mode control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep23ModUhEpMod(pub u8);
    impl Uep23ModUhEpMod {
        #[doc = "buffer mode of USB endpoint 2(10) and USB host receive endpoint (IN) enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep2_buf_mod_rb_uh_rx_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint 2(10) and USB host receive endpoint (IN) enable."]
        #[inline(always)]
        pub const fn set_uep2_buf_mod_rb_uh_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable USB endpoint 2(10) transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep2_tx_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 2(10) transmittal (IN)."]
        #[inline(always)]
        pub const fn set_uep2_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable USB endpoint 2(10) receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep2_rx_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 2(10) receiving (OUT)."]
        #[inline(always)]
        pub const fn set_uep2_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "buffer mode of USB endpoint 3(11)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep3_buf_mod(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint 3(11)."]
        #[inline(always)]
        pub const fn set_uep3_buf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable USB endpoint 3(11) transmittal (IN) and USB host send endpoint (SETUP/OUT) enable."]
        #[must_use]
        #[inline(always)]
        pub const fn uep3_tx_en_rb_uh_tx_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 3(11) transmittal (IN) and USB host send endpoint (SETUP/OUT) enable."]
        #[inline(always)]
        pub const fn set_uep3_tx_en_rb_uh_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "enable USB endpoint 3(11) receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep3_rx_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 3(11) receiving (OUT)."]
        #[inline(always)]
        pub const fn set_uep3_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep23ModUhEpMod {
        #[inline(always)]
        fn default() -> Uep23ModUhEpMod {
            Uep23ModUhEpMod(0)
        }
    }
    impl core::fmt::Debug for Uep23ModUhEpMod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep23ModUhEpMod")
                .field("uep2_buf_mod_rb_uh_rx_en", &self.uep2_buf_mod_rb_uh_rx_en())
                .field("uep2_tx_en", &self.uep2_tx_en())
                .field("uep2_rx_en", &self.uep2_rx_en())
                .field("uep3_buf_mod", &self.uep3_buf_mod())
                .field("uep3_tx_en_rb_uh_tx_en", &self.uep3_tx_en_rb_uh_tx_en())
                .field("uep3_rx_en", &self.uep3_rx_en())
                .finish()
        }
    }
    #[doc = "endpoint 2 receive max length and USB host receive max packet length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2MaxLenUhMaxLen(pub u16);
    impl Uep2MaxLenUhMaxLen {
        #[doc = "endpoint 2 receive max length and USB host receive max packet length register."]
        #[must_use]
        #[inline(always)]
        pub const fn uep2_max_len_uh_max_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 2 receive max length and USB host receive max packet length register."]
        #[inline(always)]
        pub const fn set_uep2_max_len_uh_max_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep2MaxLenUhMaxLen {
        #[inline(always)]
        fn default() -> Uep2MaxLenUhMaxLen {
            Uep2MaxLenUhMaxLen(0)
        }
    }
    impl core::fmt::Debug for Uep2MaxLenUhMaxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2MaxLenUhMaxLen")
                .field("uep2_max_len_uh_max_len", &self.uep2_max_len_uh_max_len())
                .finish()
        }
    }
    #[doc = "endpoint 2 rx control and USb host receive endpoint control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2RxCtrlUhRxCtrl(pub u8);
    impl Uep2RxCtrlUhRxCtrl {
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT) and Host reeiver response control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rres_mask_rb_uh_rres_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT) and Host reeiver response control bit."]
        #[inline(always)]
        pub const fn set_uep_rres_mask_rb_uh_rres_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "Prepared no response and Response control bit of host receiver."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rres_no_rb_uh_rres_no(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Prepared no response and Response control bit of host receiver."]
        #[inline(always)]
        pub const fn set_uep_rres_no_rb_uh_rres_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving and expected data toggle flag of host receiving (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog_mask_rb_uh_r_tog_mask(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving and expected data toggle flag of host receiving (IN)."]
        #[inline(always)]
        pub const fn set_uep_r_tog_mask_rb_uh_r_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint and enable automatic toggle after successful receiver completion."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_autotog_rb_uh_r_autotog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint and enable automatic toggle after successful receiver completion."]
        #[inline(always)]
        pub const fn set_uep_r_autotog_rb_uh_r_autotog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "expect no data packet, for high speed hub in host mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_rdata_no(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "expect no data packet, for high speed hub in host mode."]
        #[inline(always)]
        pub const fn set_uh_rdata_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for Uep2RxCtrlUhRxCtrl {
        #[inline(always)]
        fn default() -> Uep2RxCtrlUhRxCtrl {
            Uep2RxCtrlUhRxCtrl(0)
        }
    }
    impl core::fmt::Debug for Uep2RxCtrlUhRxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2RxCtrlUhRxCtrl")
                .field(
                    "uep_rres_mask_rb_uh_rres_mask",
                    &self.uep_rres_mask_rb_uh_rres_mask(),
                )
                .field(
                    "uep_rres_no_rb_uh_rres_no",
                    &self.uep_rres_no_rb_uh_rres_no(),
                )
                .field(
                    "uep_r_tog_mask_rb_uh_r_tog_mask",
                    &self.uep_r_tog_mask_rb_uh_r_tog_mask(),
                )
                .field(
                    "uep_r_autotog_rb_uh_r_autotog",
                    &self.uep_r_autotog_rb_uh_r_autotog(),
                )
                .field("uh_rdata_no", &self.uh_rdata_no())
                .finish()
        }
    }
    #[doc = "endpoint 2 DMA buffer address _ host rx endpoint buffer start address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2RxDmaUhRxDma(pub u32);
    impl Uep2RxDmaUhRxDma {
        #[doc = "endpoint 2 DMA buffer address _ host rx endpoint buffer start address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep2_rx_dma_uh_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "endpoint 2 DMA buffer address _ host rx endpoint buffer start address."]
        #[inline(always)]
        pub const fn set_uep2_rx_dma_uh_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for Uep2RxDmaUhRxDma {
        #[inline(always)]
        fn default() -> Uep2RxDmaUhRxDma {
            Uep2RxDmaUhRxDma(0)
        }
    }
    impl core::fmt::Debug for Uep2RxDmaUhRxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2RxDmaUhRxDma")
                .field("uep2_rx_dma_uh_rx_dma", &self.uep2_rx_dma_uh_rx_dma())
                .finish()
        }
    }
    #[doc = "endpoint 2 transmittal length and Set usb host token register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2TLenUhEpPid(pub u16);
    impl Uep2TLenUhEpPid {
        #[doc = "endpoint 2 transmittal length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep2_t_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 2 transmittal length."]
        #[inline(always)]
        pub const fn set_uep2_t_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
        #[doc = "The endpoint number of the target of this operation."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_epnum_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The endpoint number of the target of this operation."]
        #[inline(always)]
        pub const fn set_uh_epnum_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[doc = "The token PID packet identification of this USB transfer transaction."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_token_mask(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "The token PID packet identification of this USB transfer transaction."]
        #[inline(always)]
        pub const fn set_uh_token_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
        }
    }
    impl Default for Uep2TLenUhEpPid {
        #[inline(always)]
        fn default() -> Uep2TLenUhEpPid {
            Uep2TLenUhEpPid(0)
        }
    }
    impl core::fmt::Debug for Uep2TLenUhEpPid {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2TLenUhEpPid")
                .field("uep2_t_len", &self.uep2_t_len())
                .field("uh_epnum_mask", &self.uh_epnum_mask())
                .field("uh_token_mask", &self.uh_token_mask())
                .finish()
        }
    }
    #[doc = "endpoint 2 tx control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2TxCtrl(pub u8);
    impl Uep2TxCtrl {
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tres_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[inline(always)]
        pub const fn set_uep_tres_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "expected no response."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tres_no(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "expected no response."]
        #[inline(always)]
        pub const fn set_uep_tres_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog_mask(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal."]
        #[inline(always)]
        pub const fn set_uep_t_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 0."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_autotog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 0."]
        #[inline(always)]
        pub const fn set_uep_t_autotog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for Uep2TxCtrl {
        #[inline(always)]
        fn default() -> Uep2TxCtrl {
            Uep2TxCtrl(0)
        }
    }
    impl core::fmt::Debug for Uep2TxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2TxCtrl")
                .field("uep_tres_mask", &self.uep_tres_mask())
                .field("uep_tres_no", &self.uep_tres_no())
                .field("uep_t_tog_mask", &self.uep_t_tog_mask())
                .field("uep_t_autotog", &self.uep_t_autotog())
                .finish()
        }
    }
    #[doc = "endpoint 2 DMA TX buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep2TxDma(pub u32);
    impl Uep2TxDma {
        #[doc = "endpoint 2 DMA TX buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep2_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "endpoint 2 DMA TX buffer address."]
        #[inline(always)]
        pub const fn set_uep2_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for Uep2TxDma {
        #[inline(always)]
        fn default() -> Uep2TxDma {
            Uep2TxDma(0)
        }
    }
    impl core::fmt::Debug for Uep2TxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep2TxDma")
                .field("uep2_tx_dma", &self.uep2_tx_dma())
                .finish()
        }
    }
    #[doc = "endpoint 3 receive max length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3MaxLen(pub u16);
    impl Uep3MaxLen {
        #[doc = "endpoint 3 receive max length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep3_max_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 3 receive max length."]
        #[inline(always)]
        pub const fn set_uep3_max_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep3MaxLen {
        #[inline(always)]
        fn default() -> Uep3MaxLen {
            Uep3MaxLen(0)
        }
    }
    impl core::fmt::Debug for Uep3MaxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3MaxLen")
                .field("uep3_max_len", &self.uep3_max_len())
                .finish()
        }
    }
    #[doc = "endpoint 3 rx control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3RxCtrl(pub u8);
    impl Uep3RxCtrl {
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rres_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[inline(always)]
        pub const fn set_uep_rres_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "prepared no response."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rres_no(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "prepared no response."]
        #[inline(always)]
        pub const fn set_uep_rres_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog_mask(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving."]
        #[inline(always)]
        pub const fn set_uep_r_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_autotog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint."]
        #[inline(always)]
        pub const fn set_uep_r_autotog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for Uep3RxCtrl {
        #[inline(always)]
        fn default() -> Uep3RxCtrl {
            Uep3RxCtrl(0)
        }
    }
    impl core::fmt::Debug for Uep3RxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3RxCtrl")
                .field("uep_rres_mask", &self.uep_rres_mask())
                .field("uep_rres_no", &self.uep_rres_no())
                .field("uep_r_tog_mask", &self.uep_r_tog_mask())
                .field("uep_r_autotog", &self.uep_r_autotog())
                .finish()
        }
    }
    #[doc = "endpoint 3 DMA buffer address;host tx endpoint buffer high address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3RxDma(pub u32);
    impl Uep3RxDma {
        #[doc = "endpoint 3 DMA buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep3_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "endpoint 3 DMA buffer address."]
        #[inline(always)]
        pub const fn set_uep3_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for Uep3RxDma {
        #[inline(always)]
        fn default() -> Uep3RxDma {
            Uep3RxDma(0)
        }
    }
    impl core::fmt::Debug for Uep3RxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3RxDma")
                .field("uep3_rx_dma", &self.uep3_rx_dma())
                .finish()
        }
    }
    #[doc = "endpoint 3 transmittal length and host transmittal endpoint transmittal length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3TLenUhTxLen(pub u16);
    impl Uep3TLenUhTxLen {
        #[doc = "endpoint 3 transmittal length and host transmittal endpoint transmittal length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep3_t_len_uh_tx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 3 transmittal length and host transmittal endpoint transmittal length."]
        #[inline(always)]
        pub const fn set_uep3_t_len_uh_tx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep3TLenUhTxLen {
        #[inline(always)]
        fn default() -> Uep3TLenUhTxLen {
            Uep3TLenUhTxLen(0)
        }
    }
    impl core::fmt::Debug for Uep3TLenUhTxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3TLenUhTxLen")
                .field("uep3_t_len_uh_tx_len", &self.uep3_t_len_uh_tx_len())
                .finish()
        }
    }
    #[doc = "endpoint 3 tx control and host transmittal endpoint control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3TxCtrlUhTxCtrl(pub u8);
    impl Uep3TxCtrlUhTxCtrl {
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN) and expected handshake response type for host transmittal (SETUP/OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tres_mask_rb_uh_tres_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN) and expected handshake response type for host transmittal (SETUP/OUT)."]
        #[inline(always)]
        pub const fn set_uep_tres_mask_rb_uh_tres_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "expected no response and expected no response, 1=enable, 0=disable, for non-zero endpoint isochronous transactions."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tres_no_rb_uh_tres_no(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "expected no response and expected no response, 1=enable, 0=disable, for non-zero endpoint isochronous transactions."]
        #[inline(always)]
        pub const fn set_uep_tres_no_rb_uh_tres_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal and prepared data toggle flag of host transmittal (SETUP/OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog_mask_rb_uh_t_tog_mask(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal and prepared data toggle flag of host transmittal (SETUP/OUT)."]
        #[inline(always)]
        pub const fn set_uep_t_tog_mask_rb_uh_t_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 0 and enable automatic toggle after successful transfer completion."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_autotog_rb_uh_t_autotog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 0 and enable automatic toggle after successful transfer completion."]
        #[inline(always)]
        pub const fn set_uep_t_autotog_rb_uh_t_autotog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "prepared no data packet, for high speed hub in host mode."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_tdata_no(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "prepared no data packet, for high speed hub in host mode."]
        #[inline(always)]
        pub const fn set_uh_tdata_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for Uep3TxCtrlUhTxCtrl {
        #[inline(always)]
        fn default() -> Uep3TxCtrlUhTxCtrl {
            Uep3TxCtrlUhTxCtrl(0)
        }
    }
    impl core::fmt::Debug for Uep3TxCtrlUhTxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3TxCtrlUhTxCtrl")
                .field(
                    "uep_tres_mask_rb_uh_tres_mask",
                    &self.uep_tres_mask_rb_uh_tres_mask(),
                )
                .field(
                    "uep_tres_no_rb_uh_tres_no",
                    &self.uep_tres_no_rb_uh_tres_no(),
                )
                .field(
                    "uep_t_tog_mask_rb_uh_t_tog_mask",
                    &self.uep_t_tog_mask_rb_uh_t_tog_mask(),
                )
                .field(
                    "uep_t_autotog_rb_uh_t_autotog",
                    &self.uep_t_autotog_rb_uh_t_autotog(),
                )
                .field("uh_tdata_no", &self.uh_tdata_no())
                .finish()
        }
    }
    #[doc = "endpoint 3 DMA TX buffer address and host tx endpoint buffer start address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep3TxDmaUhTxDma(pub u32);
    impl Uep3TxDmaUhTxDma {
        #[doc = "endpoint 3 DMA TX buffer address and host tx endpoint buffer start address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep3_tx_dma_uh_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "endpoint 3 DMA TX buffer address and host tx endpoint buffer start address."]
        #[inline(always)]
        pub const fn set_uep3_tx_dma_uh_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for Uep3TxDmaUhTxDma {
        #[inline(always)]
        fn default() -> Uep3TxDmaUhTxDma {
            Uep3TxDmaUhTxDma(0)
        }
    }
    impl core::fmt::Debug for Uep3TxDmaUhTxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep3TxDmaUhTxDma")
                .field("uep3_tx_dma_uh_tx_dma", &self.uep3_tx_dma_uh_tx_dma())
                .finish()
        }
    }
    #[doc = "endpoint 1(9) 4(8,12) mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep41Mod(pub u8);
    impl Uep41Mod {
        #[doc = "buffer mode of USB endpoint 4(8,12)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep4_buf_mod(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint 4(8,12)."]
        #[inline(always)]
        pub const fn set_uep4_buf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable USB endpoint 4(8,12) transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep4_tx_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 4(8,12) transmittal (IN)."]
        #[inline(always)]
        pub const fn set_uep4_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable USB endpoint 4(8,12) receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep4_rx_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 4(8,12) receiving (OUT)."]
        #[inline(always)]
        pub const fn set_uep4_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "buffer mode of USB endpoint 1(9)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep1_buf_mod(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint 1(9)."]
        #[inline(always)]
        pub const fn set_uep1_buf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable USB endpoint 1(9) transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep1_tx_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 1(9) transmittal (IN)."]
        #[inline(always)]
        pub const fn set_uep1_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "enable USB endpoint 1(9) receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep1_rx_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 1(9) receiving (OUT)."]
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
                .field("uep4_buf_mod", &self.uep4_buf_mod())
                .field("uep4_tx_en", &self.uep4_tx_en())
                .field("uep4_rx_en", &self.uep4_rx_en())
                .field("uep1_buf_mod", &self.uep1_buf_mod())
                .field("uep1_tx_en", &self.uep1_tx_en())
                .field("uep1_rx_en", &self.uep1_rx_en())
                .finish()
        }
    }
    #[doc = "endpoint 4 receive max length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4MaxLen(pub u16);
    impl Uep4MaxLen {
        #[doc = "endpoint 4 receive max length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep4_max_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 4 receive max length."]
        #[inline(always)]
        pub const fn set_uep4_max_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep4MaxLen {
        #[inline(always)]
        fn default() -> Uep4MaxLen {
            Uep4MaxLen(0)
        }
    }
    impl core::fmt::Debug for Uep4MaxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4MaxLen")
                .field("uep4_max_len", &self.uep4_max_len())
                .finish()
        }
    }
    #[doc = "endpoint 4 rx control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4RxCtrl(pub u8);
    impl Uep4RxCtrl {
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rres_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[inline(always)]
        pub const fn set_uep_rres_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "prepared no response."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rres_no(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "prepared no response."]
        #[inline(always)]
        pub const fn set_uep_rres_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog_mask(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving."]
        #[inline(always)]
        pub const fn set_uep_r_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_autotog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint."]
        #[inline(always)]
        pub const fn set_uep_r_autotog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for Uep4RxCtrl {
        #[inline(always)]
        fn default() -> Uep4RxCtrl {
            Uep4RxCtrl(0)
        }
    }
    impl core::fmt::Debug for Uep4RxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4RxCtrl")
                .field("uep_rres_mask", &self.uep_rres_mask())
                .field("uep_rres_no", &self.uep_rres_no())
                .field("uep_r_tog_mask", &self.uep_r_tog_mask())
                .field("uep_r_autotog", &self.uep_r_autotog())
                .finish()
        }
    }
    #[doc = "endpoint 4 DMA buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4RxDma(pub u32);
    impl Uep4RxDma {
        #[doc = "endpoint 4 DMA buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep4_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "endpoint 4 DMA buffer address."]
        #[inline(always)]
        pub const fn set_uep4_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for Uep4RxDma {
        #[inline(always)]
        fn default() -> Uep4RxDma {
            Uep4RxDma(0)
        }
    }
    impl core::fmt::Debug for Uep4RxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4RxDma")
                .field("uep4_rx_dma", &self.uep4_rx_dma())
                .finish()
        }
    }
    #[doc = "endpoint 4 transmittal length and USB host Tx SPLIT packet data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4TLenUhSplitData(pub u16);
    impl Uep4TLenUhSplitData {
        #[doc = "endpoint 4 transmittal length and USB host Tx SPLIT packet data."]
        #[must_use]
        #[inline(always)]
        pub const fn uep4_t_len_uh_split_data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 4 transmittal length and USB host Tx SPLIT packet data."]
        #[inline(always)]
        pub const fn set_uep4_t_len_uh_split_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep4TLenUhSplitData {
        #[inline(always)]
        fn default() -> Uep4TLenUhSplitData {
            Uep4TLenUhSplitData(0)
        }
    }
    impl core::fmt::Debug for Uep4TLenUhSplitData {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4TLenUhSplitData")
                .field("uep4_t_len_uh_split_data", &self.uep4_t_len_uh_split_data())
                .finish()
        }
    }
    #[doc = "endpoint 4 tx control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4TxCtrl(pub u8);
    impl Uep4TxCtrl {
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tres_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[inline(always)]
        pub const fn set_uep_tres_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "expected no response."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tres_no(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "expected no response."]
        #[inline(always)]
        pub const fn set_uep_tres_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog_mask(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal."]
        #[inline(always)]
        pub const fn set_uep_t_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 0."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_autotog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 0."]
        #[inline(always)]
        pub const fn set_uep_t_autotog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for Uep4TxCtrl {
        #[inline(always)]
        fn default() -> Uep4TxCtrl {
            Uep4TxCtrl(0)
        }
    }
    impl core::fmt::Debug for Uep4TxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4TxCtrl")
                .field("uep_tres_mask", &self.uep_tres_mask())
                .field("uep_tres_no", &self.uep_tres_no())
                .field("uep_t_tog_mask", &self.uep_t_tog_mask())
                .field("uep_t_autotog", &self.uep_t_autotog())
                .finish()
        }
    }
    #[doc = "endpoint 4 DMA TX buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep4TxDma(pub u32);
    impl Uep4TxDma {
        #[doc = "endpoint 4 DMA TX buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep4_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "endpoint 4 DMA TX buffer address."]
        #[inline(always)]
        pub const fn set_uep4_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for Uep4TxDma {
        #[inline(always)]
        fn default() -> Uep4TxDma {
            Uep4TxDma(0)
        }
    }
    impl core::fmt::Debug for Uep4TxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep4TxDma")
                .field("uep4_tx_dma", &self.uep4_tx_dma())
                .finish()
        }
    }
    #[doc = "endpoint 5(13) 6(14) mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep56Mod(pub u8);
    impl Uep56Mod {
        #[doc = "buffer mode of USB endpoint 5(13)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep5_buf_mod(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint 5(13)."]
        #[inline(always)]
        pub const fn set_uep5_buf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable USB endpoint 5(13) transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep5_tx_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 5(13) transmittal (IN)."]
        #[inline(always)]
        pub const fn set_uep5_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable USB endpoint 5(13) receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep5_rx_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 5(13) receiving (OUT)."]
        #[inline(always)]
        pub const fn set_uep5_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "buffer mode of USB endpoint 6(14)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep6_buf_mod(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint 6(14)."]
        #[inline(always)]
        pub const fn set_uep6_buf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable USB endpoint 6(14) transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep6_tx_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 6(14) transmittal (IN)."]
        #[inline(always)]
        pub const fn set_uep6_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "enable USB endpoint 6(14) receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep6_rx_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 6(14) receiving (OUT)."]
        #[inline(always)]
        pub const fn set_uep6_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Uep56Mod {
        #[inline(always)]
        fn default() -> Uep56Mod {
            Uep56Mod(0)
        }
    }
    impl core::fmt::Debug for Uep56Mod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep56Mod")
                .field("uep5_buf_mod", &self.uep5_buf_mod())
                .field("uep5_tx_en", &self.uep5_tx_en())
                .field("uep5_rx_en", &self.uep5_rx_en())
                .field("uep6_buf_mod", &self.uep6_buf_mod())
                .field("uep6_tx_en", &self.uep6_tx_en())
                .field("uep6_rx_en", &self.uep6_rx_en())
                .finish()
        }
    }
    #[doc = "endpoint 5 receive max length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5MaxLen(pub u16);
    impl Uep5MaxLen {
        #[doc = "endpoint 5 receive max length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep5_max_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 5 receive max length."]
        #[inline(always)]
        pub const fn set_uep5_max_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep5MaxLen {
        #[inline(always)]
        fn default() -> Uep5MaxLen {
            Uep5MaxLen(0)
        }
    }
    impl core::fmt::Debug for Uep5MaxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5MaxLen")
                .field("uep5_max_len", &self.uep5_max_len())
                .finish()
        }
    }
    #[doc = "endpoint 5 rx control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5RxCtrl(pub u8);
    impl Uep5RxCtrl {
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rres_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[inline(always)]
        pub const fn set_uep_rres_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "prepared no response."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rres_no(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "prepared no response."]
        #[inline(always)]
        pub const fn set_uep_rres_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog_mask(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving."]
        #[inline(always)]
        pub const fn set_uep_r_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_autotog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint."]
        #[inline(always)]
        pub const fn set_uep_r_autotog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for Uep5RxCtrl {
        #[inline(always)]
        fn default() -> Uep5RxCtrl {
            Uep5RxCtrl(0)
        }
    }
    impl core::fmt::Debug for Uep5RxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5RxCtrl")
                .field("uep_rres_mask", &self.uep_rres_mask())
                .field("uep_rres_no", &self.uep_rres_no())
                .field("uep_r_tog_mask", &self.uep_r_tog_mask())
                .field("uep_r_autotog", &self.uep_r_autotog())
                .finish()
        }
    }
    #[doc = "endpoint 5 DMA buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5RxDma(pub u32);
    impl Uep5RxDma {
        #[doc = "endpoint 5 DMA buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep5_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "endpoint 5 DMA buffer address."]
        #[inline(always)]
        pub const fn set_uep5_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for Uep5RxDma {
        #[inline(always)]
        fn default() -> Uep5RxDma {
            Uep5RxDma(0)
        }
    }
    impl core::fmt::Debug for Uep5RxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5RxDma")
                .field("uep5_rx_dma", &self.uep5_rx_dma())
                .finish()
        }
    }
    #[doc = "endpoint 5 transmittal length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5TLen(pub u16);
    impl Uep5TLen {
        #[doc = "endpoint 5 transmittal length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep5_t_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 5 transmittal length."]
        #[inline(always)]
        pub const fn set_uep5_t_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
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
    #[doc = "endpoint 5 tx control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5TxCtrl(pub u8);
    impl Uep5TxCtrl {
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tres_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[inline(always)]
        pub const fn set_uep_tres_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "expected no response."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tres_no(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "expected no response."]
        #[inline(always)]
        pub const fn set_uep_tres_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog_mask(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal."]
        #[inline(always)]
        pub const fn set_uep_t_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 0."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_autotog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 0."]
        #[inline(always)]
        pub const fn set_uep_t_autotog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for Uep5TxCtrl {
        #[inline(always)]
        fn default() -> Uep5TxCtrl {
            Uep5TxCtrl(0)
        }
    }
    impl core::fmt::Debug for Uep5TxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5TxCtrl")
                .field("uep_tres_mask", &self.uep_tres_mask())
                .field("uep_tres_no", &self.uep_tres_no())
                .field("uep_t_tog_mask", &self.uep_t_tog_mask())
                .field("uep_t_autotog", &self.uep_t_autotog())
                .finish()
        }
    }
    #[doc = "endpoint 5 DMA TX buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep5TxDma(pub u32);
    impl Uep5TxDma {
        #[doc = "endpoint 5 DMA TX buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep5_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "endpoint 5 DMA TX buffer address."]
        #[inline(always)]
        pub const fn set_uep5_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for Uep5TxDma {
        #[inline(always)]
        fn default() -> Uep5TxDma {
            Uep5TxDma(0)
        }
    }
    impl core::fmt::Debug for Uep5TxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep5TxDma")
                .field("uep5_tx_dma", &self.uep5_tx_dma())
                .finish()
        }
    }
    #[doc = "endpoint 6 receive max length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6MaxLen(pub u16);
    impl Uep6MaxLen {
        #[doc = "endpoint 6 receive max length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep6_max_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 6 receive max length."]
        #[inline(always)]
        pub const fn set_uep6_max_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep6MaxLen {
        #[inline(always)]
        fn default() -> Uep6MaxLen {
            Uep6MaxLen(0)
        }
    }
    impl core::fmt::Debug for Uep6MaxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6MaxLen")
                .field("uep6_max_len", &self.uep6_max_len())
                .finish()
        }
    }
    #[doc = "endpoint 6 rx control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6RxCtrl(pub u8);
    impl Uep6RxCtrl {
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rres_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[inline(always)]
        pub const fn set_uep_rres_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "prepared no response."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rres_no(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "prepared no response."]
        #[inline(always)]
        pub const fn set_uep_rres_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog_mask(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving."]
        #[inline(always)]
        pub const fn set_uep_r_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_autotog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint."]
        #[inline(always)]
        pub const fn set_uep_r_autotog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for Uep6RxCtrl {
        #[inline(always)]
        fn default() -> Uep6RxCtrl {
            Uep6RxCtrl(0)
        }
    }
    impl core::fmt::Debug for Uep6RxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6RxCtrl")
                .field("uep_rres_mask", &self.uep_rres_mask())
                .field("uep_rres_no", &self.uep_rres_no())
                .field("uep_r_tog_mask", &self.uep_r_tog_mask())
                .field("uep_r_autotog", &self.uep_r_autotog())
                .finish()
        }
    }
    #[doc = "endpoint 6 DMA buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6RxDma(pub u32);
    impl Uep6RxDma {
        #[doc = "endpoint 6 DMA buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep6_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "endpoint 6 DMA buffer address."]
        #[inline(always)]
        pub const fn set_uep6_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for Uep6RxDma {
        #[inline(always)]
        fn default() -> Uep6RxDma {
            Uep6RxDma(0)
        }
    }
    impl core::fmt::Debug for Uep6RxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6RxDma")
                .field("uep6_rx_dma", &self.uep6_rx_dma())
                .finish()
        }
    }
    #[doc = "endpoint 6 transmittal length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6TLen(pub u16);
    impl Uep6TLen {
        #[doc = "endpoint 6 transmittal length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep6_t_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 6 transmittal length."]
        #[inline(always)]
        pub const fn set_uep6_t_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
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
    #[doc = "endpoint 6 tx control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6TxCtrl(pub u8);
    impl Uep6TxCtrl {
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tres_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[inline(always)]
        pub const fn set_uep_tres_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "expected no response."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tres_no(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "expected no response."]
        #[inline(always)]
        pub const fn set_uep_tres_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog_mask(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal."]
        #[inline(always)]
        pub const fn set_uep_t_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 0."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_autotog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 0."]
        #[inline(always)]
        pub const fn set_uep_t_autotog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for Uep6TxCtrl {
        #[inline(always)]
        fn default() -> Uep6TxCtrl {
            Uep6TxCtrl(0)
        }
    }
    impl core::fmt::Debug for Uep6TxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6TxCtrl")
                .field("uep_tres_mask", &self.uep_tres_mask())
                .field("uep_tres_no", &self.uep_tres_no())
                .field("uep_t_tog_mask", &self.uep_t_tog_mask())
                .field("uep_t_autotog", &self.uep_t_autotog())
                .finish()
        }
    }
    #[doc = "endpoint 4 DMA TX buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep6TxDma(pub u32);
    impl Uep6TxDma {
        #[doc = "endpoint 6 DMA TX buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep6_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "endpoint 6 DMA TX buffer address."]
        #[inline(always)]
        pub const fn set_uep6_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for Uep6TxDma {
        #[inline(always)]
        fn default() -> Uep6TxDma {
            Uep6TxDma(0)
        }
    }
    impl core::fmt::Debug for Uep6TxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep6TxDma")
                .field("uep6_tx_dma", &self.uep6_tx_dma())
                .finish()
        }
    }
    #[doc = "endpoint 7 receive max length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7MaxLen(pub u16);
    impl Uep7MaxLen {
        #[doc = "endpoint 7 receive max length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep7_max_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 7 receive max length."]
        #[inline(always)]
        pub const fn set_uep7_max_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Uep7MaxLen {
        #[inline(always)]
        fn default() -> Uep7MaxLen {
            Uep7MaxLen(0)
        }
    }
    impl core::fmt::Debug for Uep7MaxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7MaxLen")
                .field("uep7_max_len", &self.uep7_max_len())
                .finish()
        }
    }
    #[doc = "endpoint 7(15) mode."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7Mod(pub u8);
    impl Uep7Mod {
        #[doc = "buffer mode of USB endpoint 7(15)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep7_buf_mod(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "buffer mode of USB endpoint 7(15)."]
        #[inline(always)]
        pub const fn set_uep7_buf_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable USB endpoint 7(15) transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep7_tx_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 7(15) transmittal (IN)."]
        #[inline(always)]
        pub const fn set_uep7_tx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable USB endpoint 7(15) receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep7_rx_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable USB endpoint 7(15) receiving (OUT)."]
        #[inline(always)]
        pub const fn set_uep7_rx_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for Uep7Mod {
        #[inline(always)]
        fn default() -> Uep7Mod {
            Uep7Mod(0)
        }
    }
    impl core::fmt::Debug for Uep7Mod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7Mod")
                .field("uep7_buf_mod", &self.uep7_buf_mod())
                .field("uep7_tx_en", &self.uep7_tx_en())
                .field("uep7_rx_en", &self.uep7_rx_en())
                .finish()
        }
    }
    #[doc = "endpoint 7 rx control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7RxCtrl(pub u8);
    impl Uep7RxCtrl {
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rres_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X receiving (OUT)."]
        #[inline(always)]
        pub const fn set_uep_rres_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "prepared no response."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_rres_no(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "prepared no response."]
        #[inline(always)]
        pub const fn set_uep_rres_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_tog_mask(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "expected data toggle flag of USB endpoint X receiving."]
        #[inline(always)]
        pub const fn set_uep_r_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_r_autotog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint."]
        #[inline(always)]
        pub const fn set_uep_r_autotog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for Uep7RxCtrl {
        #[inline(always)]
        fn default() -> Uep7RxCtrl {
            Uep7RxCtrl(0)
        }
    }
    impl core::fmt::Debug for Uep7RxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7RxCtrl")
                .field("uep_rres_mask", &self.uep_rres_mask())
                .field("uep_rres_no", &self.uep_rres_no())
                .field("uep_r_tog_mask", &self.uep_r_tog_mask())
                .field("uep_r_autotog", &self.uep_r_autotog())
                .finish()
        }
    }
    #[doc = "endpoint 7 DMA buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7RxDma(pub u32);
    impl Uep7RxDma {
        #[doc = "endpoint 7 DMA buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep7_rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "endpoint 7 DMA buffer address."]
        #[inline(always)]
        pub const fn set_uep7_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for Uep7RxDma {
        #[inline(always)]
        fn default() -> Uep7RxDma {
            Uep7RxDma(0)
        }
    }
    impl core::fmt::Debug for Uep7RxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7RxDma")
                .field("uep7_rx_dma", &self.uep7_rx_dma())
                .finish()
        }
    }
    #[doc = "endpoint 7 transmittal length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7TLen(pub u16);
    impl Uep7TLen {
        #[doc = "endpoint 7 transmittal length."]
        #[must_use]
        #[inline(always)]
        pub const fn uep7_t_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "endpoint 7 transmittal length."]
        #[inline(always)]
        pub const fn set_uep7_t_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
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
    #[doc = "endpoint 7 tx control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7TxCtrl(pub u8);
    impl Uep7TxCtrl {
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tres_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "bit mask of handshake response type for USB endpoint X transmittal (IN)."]
        #[inline(always)]
        pub const fn set_uep_tres_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "expected no response."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_tres_no(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "expected no response."]
        #[inline(always)]
        pub const fn set_uep_tres_no(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_tog_mask(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "prepared data toggle flag of USB endpoint X transmittal."]
        #[inline(always)]
        pub const fn set_uep_t_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 0."]
        #[must_use]
        #[inline(always)]
        pub const fn uep_t_autotog(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable automatic toggle after successful transfer completion on endpoint 0."]
        #[inline(always)]
        pub const fn set_uep_t_autotog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for Uep7TxCtrl {
        #[inline(always)]
        fn default() -> Uep7TxCtrl {
            Uep7TxCtrl(0)
        }
    }
    impl core::fmt::Debug for Uep7TxCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7TxCtrl")
                .field("uep_tres_mask", &self.uep_tres_mask())
                .field("uep_tres_no", &self.uep_tres_no())
                .field("uep_t_tog_mask", &self.uep_t_tog_mask())
                .field("uep_t_autotog", &self.uep_t_autotog())
                .finish()
        }
    }
    #[doc = "endpoint 7 DMA TX buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Uep7TxDma(pub u32);
    impl Uep7TxDma {
        #[doc = "endpoint 7 DMA TX buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn uep7_tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "endpoint 7 DMA TX buffer address."]
        #[inline(always)]
        pub const fn set_uep7_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for Uep7TxDma {
        #[inline(always)]
        fn default() -> Uep7TxDma {
            Uep7TxDma(0)
        }
    }
    impl core::fmt::Debug for Uep7TxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Uep7TxDma")
                .field("uep7_tx_dma", &self.uep7_tx_dma())
                .finish()
        }
    }
    #[doc = "USB host control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UhostCtrl(pub u8);
    impl UhostCtrl {
        #[doc = "USB host send bus reset signal."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_bus_reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB host send bus reset signal."]
        #[inline(always)]
        pub const fn set_uh_bus_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB host send bus suspend signal."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_bus_suspend(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB host send bus suspend signal."]
        #[inline(always)]
        pub const fn set_uh_bus_suspend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "USB host suspend state and wake up device."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_bus_resume(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "USB host suspend state and wake up device."]
        #[inline(always)]
        pub const fn set_uh_bus_resume(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "Automatically generate sof packet enable control."]
        #[must_use]
        #[inline(always)]
        pub const fn uh_autosof_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Automatically generate sof packet enable control."]
        #[inline(always)]
        pub const fn set_uh_autosof_en(&mut self, val: bool) {
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
                .field("uh_bus_reset", &self.uh_bus_reset())
                .field("uh_bus_suspend", &self.uh_bus_suspend())
                .field("uh_bus_resume", &self.uh_bus_resume())
                .field("uh_autosof_en", &self.uh_autosof_en())
                .finish()
        }
    }
}
