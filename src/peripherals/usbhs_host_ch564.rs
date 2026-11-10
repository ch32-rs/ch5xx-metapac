#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "USBHS Host mode functions' registers."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbhsHost {
    ptr: *mut u8,
}
unsafe impl Send for UsbhsHost {}
unsafe impl Sync for UsbhsHost {}
impl UsbhsHost {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "USB host Configuration register."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "USB host interrupt enable register."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "USB host device address register."]
    #[inline(always)]
    pub const fn dev_addr(self) -> crate::common::Reg<regs::DevAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "USB host control register."]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "USB HOST interrupt flag register."]
    #[inline(always)]
    pub const fn int_flag(self) -> crate::common::Reg<regs::IntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "USB host interrupt status."]
    #[inline(always)]
    pub const fn int_st(self) -> crate::common::Reg<regs::IntSt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "USB host miscellaneous status."]
    #[inline(always)]
    pub const fn mis_st(self) -> crate::common::Reg<regs::MisSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "USB host power management data register."]
    #[inline(always)]
    pub const fn lpm(self) -> crate::common::Reg<regs::Lpm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "USB host SPLIT register."]
    #[inline(always)]
    pub const fn split(self) -> crate::common::Reg<regs::Split, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "USB host frame register."]
    #[inline(always)]
    pub const fn frame(self) -> crate::common::Reg<regs::Frame, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "USB host send length register."]
    #[inline(always)]
    pub const fn tx_len(self) -> crate::common::Reg<regs::TxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "USB host receive length register."]
    #[inline(always)]
    pub const fn rx_len(self) -> crate::common::Reg<regs::RxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "USB host receives the maximum length register."]
    #[inline(always)]
    pub const fn rx_max_len(self) -> crate::common::Reg<regs::RxMaxLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "DMA receive address register."]
    #[inline(always)]
    pub const fn rx_dma(self) -> crate::common::Reg<regs::RxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "DMA send address register."]
    #[inline(always)]
    pub const fn tx_dma(self) -> crate::common::Reg<regs::TxDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "USB host port control register."]
    #[inline(always)]
    pub const fn port_ctrl(self) -> crate::common::Reg<regs::PortCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "USB host port Configuration register."]
    #[inline(always)]
    pub const fn port_cfg(self) -> crate::common::Reg<regs::PortCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "USB host port interrupt enable register."]
    #[inline(always)]
    pub const fn port_int_en(self) -> crate::common::Reg<regs::PortIntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x32usize) as _) }
    }
    #[doc = "USB host port test mode register."]
    #[inline(always)]
    pub const fn port_test_ct(self) -> crate::common::Reg<regs::PortTestCt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x33usize) as _) }
    }
    #[doc = "USB host port status register."]
    #[inline(always)]
    pub const fn port_status(self) -> crate::common::Reg<regs::PortStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "USB host BC charging control register."]
    #[inline(always)]
    pub const fn root_bc_ctrl(self) -> crate::common::Reg<regs::RootBcCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
}
pub mod regs {
    #[doc = "USB host Configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u8);
    impl Cfg {
        #[doc = "USB connection controller module resets."]
        #[must_use]
        #[inline(always)]
        pub const fn root_link_reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "USB connection controller module resets."]
        #[inline(always)]
        pub const fn set_root_link_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "USB protocol processor reset."]
        #[must_use]
        #[inline(always)]
        pub const fn root_sie_reset(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "USB protocol processor reset."]
        #[inline(always)]
        pub const fn set_root_sie_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "clear all interrupt flags."]
        #[must_use]
        #[inline(always)]
        pub const fn root_all_clr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "clear all interrupt flags."]
        #[inline(always)]
        pub const fn set_root_all_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "PHY suspend,close utmi clock."]
        #[must_use]
        #[inline(always)]
        pub const fn root_phy_suspendm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PHY suspend,close utmi clock."]
        #[inline(always)]
        pub const fn set_root_phy_suspendm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "DMA transfer enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn root_dma_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "DMA transfer enabled."]
        #[inline(always)]
        pub const fn set_root_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "sof packet sending function enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn root_sof_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "sof packet sending function enabled."]
        #[inline(always)]
        pub const fn set_root_sof_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "forced full speed FS."]
        #[must_use]
        #[inline(always)]
        pub const fn root_force_fs(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "forced full speed FS."]
        #[inline(always)]
        pub const fn set_root_force_fs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "LPM enable."]
        #[must_use]
        #[inline(always)]
        pub const fn root_lpm_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "LPM enable."]
        #[inline(always)]
        pub const fn set_root_lpm_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Cfg {
        #[inline(always)]
        fn default() -> Cfg {
            Cfg(0)
        }
    }
    impl core::fmt::Debug for Cfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfg")
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
    #[doc = "USB host control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Control(pub u32);
    impl Control {
        #[doc = "transaction token packet PID."]
        #[must_use]
        #[inline(always)]
        pub const fn pid_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "transaction token packet PID."]
        #[inline(always)]
        pub const fn set_pid_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "transaction token packet endpoint number."]
        #[must_use]
        #[inline(always)]
        pub const fn endp_mask(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "transaction token packet endpoint number."]
        #[inline(always)]
        pub const fn set_endp_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "send data PID."]
        #[must_use]
        #[inline(always)]
        pub const fn tog_mask(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "send data PID."]
        #[inline(always)]
        pub const fn set_tog_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
        }
        #[doc = "DATA cache control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn buf_mode(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "DATA cache control bit."]
        #[inline(always)]
        pub const fn set_buf_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "host enables the transaction."]
        #[must_use]
        #[inline(always)]
        pub const fn host_action(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "host enables the transaction."]
        #[inline(always)]
        pub const fn set_host_action(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "valid to send LMP packet."]
        #[must_use]
        #[inline(always)]
        pub const fn lpm_valid(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "valid to send LMP packet."]
        #[inline(always)]
        pub const fn set_lpm_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "valid to send SPLIT packet."]
        #[must_use]
        #[inline(always)]
        pub const fn split_valid(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "valid to send SPLIT packet."]
        #[inline(always)]
        pub const fn set_split_valid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "this bit is enabled when the port is operating at full. speed and needs to send low speed packets(PRE)."]
        #[must_use]
        #[inline(always)]
        pub const fn down_lowspd(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "this bit is enabled when the port is operating at full. speed and needs to send low speed packets(PRE)."]
        #[inline(always)]
        pub const fn set_down_lowspd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "OUT or SETUP token packet packets are followed by no data. packets for hign-speed SPLIT packets."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_no_data(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "OUT or SETUP token packet packets are followed by no data. packets for hign-speed SPLIT packets."]
        #[inline(always)]
        pub const fn set_tx_no_data(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "DATA packets are not expected after IN token packets and. are used for high-speed SPLIT packets."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_no_data(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "DATA packets are not expected after IN token packets and. are used for high-speed SPLIT packets."]
        #[inline(always)]
        pub const fn set_rx_no_data(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "OUT or SETUP DATA no reply is extended and is used for. synchronous transmission or hign-speed SPLIT packets."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_no_res(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "OUT or SETUP DATA no reply is extended and is used for. synchronous transmission or hign-speed SPLIT packets."]
        #[inline(always)]
        pub const fn set_tx_no_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "IN-DATA no answer,used for synchronous transfer or. high-speed SPLIT packets."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_no_res(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "IN-DATA no answer,used for synchronous transfer or. high-speed SPLIT packets."]
        #[inline(always)]
        pub const fn set_rx_no_res(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Control {
        #[inline(always)]
        fn default() -> Control {
            Control(0)
        }
    }
    impl core::fmt::Debug for Control {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Control")
                .field("pid_mask", &self.pid_mask())
                .field("endp_mask", &self.endp_mask())
                .field("tog_mask", &self.tog_mask())
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
    #[doc = "USB host device address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DevAddr(pub u8);
    impl DevAddr {
        #[doc = "device address."]
        #[must_use]
        #[inline(always)]
        pub const fn dev_addr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "device address."]
        #[inline(always)]
        pub const fn set_dev_addr(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for DevAddr {
        #[inline(always)]
        fn default() -> DevAddr {
            DevAddr(0)
        }
    }
    impl core::fmt::Debug for DevAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DevAddr")
                .field("dev_addr", &self.dev_addr())
                .finish()
        }
    }
    #[doc = "USB host frame register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Frame(pub u32);
    impl Frame {
        #[doc = "the frame number."]
        #[must_use]
        #[inline(always)]
        pub const fn frame_no(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "the frame number."]
        #[inline(always)]
        pub const fn set_frame_no(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "microfame number."]
        #[must_use]
        #[inline(always)]
        pub const fn micro_frame(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "microfame number."]
        #[inline(always)]
        pub const fn set_micro_frame(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "SOF count enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn sof_cnt_en(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SOF count enabled."]
        #[inline(always)]
        pub const fn set_sof_cnt_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "the SOF count is cleared."]
        #[must_use]
        #[inline(always)]
        pub const fn sof_cnt_clr(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "the SOF count is cleared."]
        #[inline(always)]
        pub const fn set_sof_cnt_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Frame {
        #[inline(always)]
        fn default() -> Frame {
            Frame(0)
        }
    }
    impl core::fmt::Debug for Frame {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Frame")
                .field("frame_no", &self.frame_no())
                .field("micro_frame", &self.micro_frame())
                .field("sof_cnt_en", &self.sof_cnt_en())
                .field("sof_cnt_clr", &self.sof_cnt_clr())
                .finish()
        }
    }
    #[doc = "USB host interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u8);
    impl IntEn {
        #[doc = "Wakeup interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn wkup_act_ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup interrupt enabled."]
        #[inline(always)]
        pub const fn set_wkup_act_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "bus recovery interrupt was enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn resume_act_ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "bus recovery interrupt was enabled."]
        #[inline(always)]
        pub const fn set_resume_act_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB transfer end interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_act_ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB transfer end interrupt enabled."]
        #[inline(always)]
        pub const fn set_usb_act_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "sof packet sending interruption is enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn sof_act_ie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "sof packet sending interruption is enabled."]
        #[inline(always)]
        pub const fn set_sof_act_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "send pause interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_halt_ie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "send pause interrupt enable."]
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
    impl Default for IntEn {
        #[inline(always)]
        fn default() -> IntEn {
            IntEn(0)
        }
    }
    impl core::fmt::Debug for IntEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntEn")
                .field("wkup_act_ie", &self.wkup_act_ie())
                .field("resume_act_ie", &self.resume_act_ie())
                .field("usb_act_ie", &self.usb_act_ie())
                .field("sof_act_ie", &self.sof_act_ie())
                .field("tx_halt_ie", &self.tx_halt_ie())
                .field("fifo_ov_ie", &self.fifo_ov_ie())
                .finish()
        }
    }
    #[doc = "USB HOST interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFlag(pub u8);
    impl IntFlag {
        #[doc = "Wakeup interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wkup_if(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup interrupt flag."]
        #[inline(always)]
        pub const fn set_wkup_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "bus recovery interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn resume_act_if(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "bus recovery interrupt flag."]
        #[inline(always)]
        pub const fn set_resume_act_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "USB transaction transfer completion interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_act_if(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "USB transaction transfer completion interrupt flag."]
        #[inline(always)]
        pub const fn set_usb_act_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "sof packet delivert completion interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn sof_act_if(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "sof packet delivert completion interrupt flag."]
        #[inline(always)]
        pub const fn set_sof_act_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "send pause interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_halt_if(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "send pause interrupt flag."]
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
    impl Default for IntFlag {
        #[inline(always)]
        fn default() -> IntFlag {
            IntFlag(0)
        }
    }
    impl core::fmt::Debug for IntFlag {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntFlag")
                .field("wkup_if", &self.wkup_if())
                .field("resume_act_if", &self.resume_act_if())
                .field("usb_act_if", &self.usb_act_if())
                .field("sof_act_if", &self.sof_act_if())
                .field("tx_halt_if", &self.tx_halt_if())
                .field("fifo_ov_if", &self.fifo_ov_if())
                .finish()
        }
    }
    #[doc = "USB host interrupt status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntSt(pub u8);
    impl IntSt {
        #[doc = "received PID."]
        #[must_use]
        #[inline(always)]
        pub const fn this_pid_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "received PID."]
        #[inline(always)]
        pub const fn set_this_pid_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "A bit of indicates that the port received a wakeup signal."]
        #[must_use]
        #[inline(always)]
        pub const fn port_rx_resume(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "A bit of indicates that the port received a wakeup signal."]
        #[inline(always)]
        pub const fn set_port_rx_resume(&mut self, val: bool) {
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
                .field("this_pid_mask", &self.this_pid_mask())
                .field("port_rx_resume", &self.port_rx_resume())
                .finish()
        }
    }
    #[doc = "USB host power management data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lpm(pub u32);
    impl Lpm {
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
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for Lpm {
        #[inline(always)]
        fn default() -> Lpm {
            Lpm(0)
        }
    }
    impl core::fmt::Debug for Lpm {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Lpm")
                .field("lpm_data", &self.lpm_data())
                .finish()
        }
    }
    #[doc = "USB host miscellaneous status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MisSt(pub u8);
    impl MisSt {
        #[doc = "port enabled state."]
        #[must_use]
        #[inline(always)]
        pub const fn sof_free(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "port enabled state."]
        #[inline(always)]
        pub const fn set_sof_free(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "The state of the USB SOF packet is indicated as follows."]
        #[must_use]
        #[inline(always)]
        pub const fn sof_pre(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "The state of the USB SOF packet is indicated as follows."]
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
        #[doc = "USB bus wakes up."]
        #[must_use]
        #[inline(always)]
        pub const fn usb_wakeup(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "USB bus wakes up."]
        #[inline(always)]
        pub const fn set_usb_wakeup(&mut self, val: bool) {
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
        #[doc = "J on USB bus."]
        #[must_use]
        #[inline(always)]
        pub const fn bus_j(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "J on USB bus."]
        #[inline(always)]
        pub const fn set_bus_j(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "SE0 on USB bus."]
        #[must_use]
        #[inline(always)]
        pub const fn bus_se0(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SE0 on USB bus."]
        #[inline(always)]
        pub const fn set_bus_se0(&mut self, val: bool) {
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
                .field("sof_free", &self.sof_free())
                .field("sof_pre", &self.sof_pre())
                .field("sof_st", &self.sof_st())
                .field("usb_wakeup", &self.usb_wakeup())
                .field("linestate_mask", &self.linestate_mask())
                .field("bus_j", &self.bus_j())
                .field("bus_se0", &self.bus_se0())
                .finish()
        }
    }
    #[doc = "USB host port Configuration register."]
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
        #[doc = "the 15k resistance drop-down function is enable in host. mode."]
        #[must_use]
        #[inline(always)]
        pub const fn port_rpd(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "the 15k resistance drop-down function is enable in host. mode."]
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
    #[doc = "USB host port control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortCtrl(pub u32);
    impl PortCtrl {
        #[doc = "PORT send reset."]
        #[must_use]
        #[inline(always)]
        pub const fn set_port_reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PORT send reset."]
        #[inline(always)]
        pub const fn set_set_port_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "the PORT is in a suspended state."]
        #[must_use]
        #[inline(always)]
        pub const fn set_port_susp(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "the PORT is in a suspended state."]
        #[inline(always)]
        pub const fn set_set_port_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "the PORT exits the suspended state."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_port_susp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "the PORT exits the suspended state."]
        #[inline(always)]
        pub const fn set_clr_port_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "the PORT goes to sleep(LPM)."]
        #[must_use]
        #[inline(always)]
        pub const fn set_port_sleep(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "the PORT goes to sleep(LPM)."]
        #[inline(always)]
        pub const fn set_set_port_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "PORT exits the enabled state and enters the DISABLED state."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_port_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "PORT exits the enabled state and enters the DISABLED state."]
        #[inline(always)]
        pub const fn set_clr_port_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "bring the PORT into the port state."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_port_connect(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "bring the PORT into the port state."]
        #[inline(always)]
        pub const fn set_clr_port_connect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "the PORT exits the sleep state (LPM)."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_port_sleep(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "the PORT exits the sleep state (LPM)."]
        #[inline(always)]
        pub const fn set_clr_port_sleep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "wake-up time control."]
        #[must_use]
        #[inline(always)]
        pub const fn port_sleep_besl(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "wake-up time control."]
        #[inline(always)]
        pub const fn set_port_sleep_besl(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "BUS reset time selection."]
        #[must_use]
        #[inline(always)]
        pub const fn bus_rst_long(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "BUS reset time selection."]
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
    #[doc = "USB host port interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortIntEn(pub u8);
    impl PortIntEn {
        #[doc = "port connection state change interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn port_connect_ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "port connection state change interrupt enabled."]
        #[inline(always)]
        pub const fn set_port_connect_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "port enable state change interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn port_en_ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "port enable state change interrupt enabled."]
        #[inline(always)]
        pub const fn set_port_en_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "port pause state change interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn port_susp_ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "port pause state change interrupt enabled."]
        #[inline(always)]
        pub const fn set_port_susp_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "port reset state change interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn port_reset_ie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "port reset state change interrupt enabled."]
        #[inline(always)]
        pub const fn set_port_reset_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "port sleep state change interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn port_slp_ie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "port sleep state change interrupt enabled."]
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
    #[doc = "USB host port status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortStatus(pub u16);
    impl PortStatus {
        #[doc = "port connection state."]
        #[must_use]
        #[inline(always)]
        pub const fn port_c0nnect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "port connection state."]
        #[inline(always)]
        pub const fn set_port_c0nnect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "port enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn port_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "port enabled."]
        #[inline(always)]
        pub const fn set_port_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "port pause state."]
        #[must_use]
        #[inline(always)]
        pub const fn port_susp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "port pause state."]
        #[inline(always)]
        pub const fn set_port_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "port reset status."]
        #[must_use]
        #[inline(always)]
        pub const fn port_restting(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "port reset status."]
        #[inline(always)]
        pub const fn set_port_restting(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "port sleep."]
        #[must_use]
        #[inline(always)]
        pub const fn port_slp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "port sleep."]
        #[inline(always)]
        pub const fn set_port_slp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "whether the port connection speed is low."]
        #[must_use]
        #[inline(always)]
        pub const fn port_low_spd(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "whether the port connection speed is low."]
        #[inline(always)]
        pub const fn set_port_low_spd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "whether the port connection speed is hign."]
        #[must_use]
        #[inline(always)]
        pub const fn port_high_spd(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "whether the port connection speed is hign."]
        #[inline(always)]
        pub const fn set_port_high_spd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "whether the port is in test mode."]
        #[must_use]
        #[inline(always)]
        pub const fn port_test(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "whether the port is in test mode."]
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
    #[doc = "USB host port test mode register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PortTestCt(pub u8);
    impl PortTestCt {
        #[doc = "test output J."]
        #[must_use]
        #[inline(always)]
        pub const fn test_j(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "test output J."]
        #[inline(always)]
        pub const fn set_test_j(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "test output K."]
        #[must_use]
        #[inline(always)]
        pub const fn test_k(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "test output K."]
        #[inline(always)]
        pub const fn set_test_k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "test mode was enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn test_force_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "test mode was enabled."]
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
    #[doc = "USB host BC charging control register."]
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
        #[doc = "UDP pin BC protocol comparator enables."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_bc_cmpe(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "UDP pin BC protocol comparator enables."]
        #[inline(always)]
        pub const fn set_udp_bc_cmpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "UDM pin BC protocol comparator enables."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_bc_cmpe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "UDM pin BC protocol comparator enables."]
        #[inline(always)]
        pub const fn set_udm_bc_cmpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Automatic mode enables."]
        #[must_use]
        #[inline(always)]
        pub const fn bc_auto_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic mode enables."]
        #[inline(always)]
        pub const fn set_bc_auto_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "UDP pin BC protocol source voltage enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn udp_bc_vsrc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "UDP pin BC protocol source voltage enabled."]
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
        #[doc = "In automatic mode, UDP outputs VBC_SRC ,otherwise it is. controlled by UDM_BC_CMPE."]
        #[must_use]
        #[inline(always)]
        pub const fn udm_vsrc_act(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "In automatic mode, UDP outputs VBC_SRC ,otherwise it is. controlled by UDM_BC_CMPE."]
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
    #[doc = "DMA receive address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxDma(pub u32);
    impl RxDma {
        #[doc = "Received address."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "Received address."]
        #[inline(always)]
        pub const fn set_rx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for RxDma {
        #[inline(always)]
        fn default() -> RxDma {
            RxDma(0)
        }
    }
    impl core::fmt::Debug for RxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxDma")
                .field("rx_dma", &self.rx_dma())
                .finish()
        }
    }
    #[doc = "USB host receive length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxLen(pub u32);
    impl RxLen {
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
    #[doc = "USB host receives the maximum length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxMaxLen(pub u32);
    impl RxMaxLen {
        #[doc = "receives the maximum length."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_max_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "receives the maximum length."]
        #[inline(always)]
        pub const fn set_rx_max_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for RxMaxLen {
        #[inline(always)]
        fn default() -> RxMaxLen {
            RxMaxLen(0)
        }
    }
    impl core::fmt::Debug for RxMaxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxMaxLen")
                .field("rx_max_len", &self.rx_max_len())
                .finish()
        }
    }
    #[doc = "USB host SPLIT register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Split(pub u32);
    impl Split {
        #[doc = "SPLIT management data."]
        #[must_use]
        #[inline(always)]
        pub const fn split_data(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0007_ffff;
            val as u32
        }
        #[doc = "SPLIT management data."]
        #[inline(always)]
        pub const fn set_split_data(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0007_ffff << 0usize)) | (((val as u32) & 0x0007_ffff) << 0usize);
        }
    }
    impl Default for Split {
        #[inline(always)]
        fn default() -> Split {
            Split(0)
        }
    }
    impl core::fmt::Debug for Split {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Split")
                .field("split_data", &self.split_data())
                .finish()
        }
    }
    #[doc = "DMA send address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxDma(pub u32);
    impl TxDma {
        #[doc = "Send address."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_dma(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "Send address."]
        #[inline(always)]
        pub const fn set_tx_dma(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for TxDma {
        #[inline(always)]
        fn default() -> TxDma {
            TxDma(0)
        }
    }
    impl core::fmt::Debug for TxDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxDma")
                .field("tx_dma", &self.tx_dma())
                .finish()
        }
    }
    #[doc = "USB host send length register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxLen(pub u32);
    impl TxLen {
        #[doc = "send data length."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_len(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "send data length."]
        #[inline(always)]
        pub const fn set_tx_len(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for TxLen {
        #[inline(always)]
        fn default() -> TxLen {
            TxLen(0)
        }
    }
    impl core::fmt::Debug for TxLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxLen")
                .field("tx_len", &self.tx_len())
                .finish()
        }
    }
}
