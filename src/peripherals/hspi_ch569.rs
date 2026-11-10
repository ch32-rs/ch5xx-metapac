#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "HSPI register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hspi {
    ptr: *mut u8,
}
unsafe impl Send for Hspi {}
unsafe impl Sync for Hspi {}
impl Hspi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "parallel if tx or rx cfg."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "parallel if tx or rx control."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "parallel if interrupt enable register."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "parallel if aux."]
    #[inline(always)]
    pub const fn aux(self) -> crate::common::Reg<regs::Aux, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "parallel if dma tx addr0."]
    #[inline(always)]
    pub const fn tx_addr0(self) -> crate::common::Reg<regs::TxAddr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "parallel if dma tx addr1."]
    #[inline(always)]
    pub const fn tx_addr1(self) -> crate::common::Reg<regs::TxAddr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "parallel if dma rx addr0."]
    #[inline(always)]
    pub const fn rx_addr0(self) -> crate::common::Reg<regs::RxAddr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "parallel if dma rx addr1."]
    #[inline(always)]
    pub const fn rx_addr1(self) -> crate::common::Reg<regs::RxAddr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "parallel if dma length0."]
    #[inline(always)]
    pub const fn dma_len0(self) -> crate::common::Reg<regs::DmaLen0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "parallel if receive length0."]
    #[inline(always)]
    pub const fn rx_len0(self) -> crate::common::Reg<regs::RxLen0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x16usize) as _) }
    }
    #[doc = "parallel if dma length1."]
    #[inline(always)]
    pub const fn dma_len1(self) -> crate::common::Reg<regs::DmaLen1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "parallel if receive length1."]
    #[inline(always)]
    pub const fn rx_len1(self) -> crate::common::Reg<regs::RxLen1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "parallel if tx burst config register."]
    #[inline(always)]
    pub const fn burst_cfg(self) -> crate::common::Reg<regs::BurstCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "parallel if tx burst count."]
    #[inline(always)]
    pub const fn burst_cnt(self) -> crate::common::Reg<regs::BurstCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1eusize) as _) }
    }
    #[doc = "parallel if user defined field 0 register."]
    #[inline(always)]
    pub const fn udf0(self) -> crate::common::Reg<regs::Udf0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "parallel if user defined field 1 register."]
    #[inline(always)]
    pub const fn udf1(self) -> crate::common::Reg<regs::Udf1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "parallel if interrupt flag."]
    #[inline(always)]
    pub const fn int_flag(self) -> crate::common::Reg<regs::IntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "parallel rtx status."]
    #[inline(always)]
    pub const fn rtx_status(self) -> crate::common::Reg<regs::RtxStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x29usize) as _) }
    }
    #[doc = "parallel TX sequence ctrl."]
    #[inline(always)]
    pub const fn tx_sc(self) -> crate::common::Reg<regs::TxSc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2ausize) as _) }
    }
    #[doc = "parallel RX sequence ctrl."]
    #[inline(always)]
    pub const fn rx_sc(self) -> crate::common::Reg<regs::RxSc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2busize) as _) }
    }
}
pub mod regs {
    #[doc = "parallel if aux."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Aux(pub u8);
    impl Aux {
        #[doc = "parallel if tx clk polar control."]
        #[must_use]
        #[inline(always)]
        pub const fn tck_mod(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "parallel if tx clk polar control."]
        #[inline(always)]
        pub const fn set_tck_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "parallel if rx clk polar control."]
        #[must_use]
        #[inline(always)]
        pub const fn rck_mod(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "parallel if rx clk polar control."]
        #[inline(always)]
        pub const fn set_rck_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "parallel if tx ack mode cfg."]
        #[must_use]
        #[inline(always)]
        pub const fn ack_tx_mod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "parallel if tx ack mode cfg."]
        #[inline(always)]
        pub const fn set_ack_tx_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "delay time of parallel if send ack when receive done."]
        #[must_use]
        #[inline(always)]
        pub const fn ack_cnt_sel(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "delay time of parallel if send ack when receive done."]
        #[inline(always)]
        pub const fn set_ack_cnt_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
        }
    }
    impl Default for Aux {
        #[inline(always)]
        fn default() -> Aux {
            Aux(0)
        }
    }
    impl core::fmt::Debug for Aux {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Aux")
                .field("tck_mod", &self.tck_mod())
                .field("rck_mod", &self.rck_mod())
                .field("ack_tx_mod", &self.ack_tx_mod())
                .field("ack_cnt_sel", &self.ack_cnt_sel())
                .finish()
        }
    }
    #[doc = "parallel if tx burst config register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BurstCfg(pub u16);
    impl BurstCfg {
        #[doc = "burst transmit enable."]
        #[must_use]
        #[inline(always)]
        pub const fn burst_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "burst transmit enable."]
        #[inline(always)]
        pub const fn set_burst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "burst transmit length."]
        #[must_use]
        #[inline(always)]
        pub const fn burst_len(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "burst transmit length."]
        #[inline(always)]
        pub const fn set_burst_len(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u16) & 0xff) << 8usize);
        }
    }
    impl Default for BurstCfg {
        #[inline(always)]
        fn default() -> BurstCfg {
            BurstCfg(0)
        }
    }
    impl core::fmt::Debug for BurstCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BurstCfg")
                .field("burst_en", &self.burst_en())
                .field("burst_len", &self.burst_len())
                .finish()
        }
    }
    #[doc = "parallel if tx burst count."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BurstCnt(pub u8);
    impl BurstCnt {
        #[doc = "parallel if tx burst count."]
        #[must_use]
        #[inline(always)]
        pub const fn burst_cnt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "parallel if tx burst count."]
        #[inline(always)]
        pub const fn set_burst_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for BurstCnt {
        #[inline(always)]
        fn default() -> BurstCnt {
            BurstCnt(0)
        }
    }
    impl core::fmt::Debug for BurstCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BurstCnt")
                .field("burst_cnt", &self.burst_cnt())
                .finish()
        }
    }
    #[doc = "parallel if tx or rx cfg."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u8);
    impl Cfg {
        #[doc = "parallel if mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "parallel if mode."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "parallel if dualdma mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dualdma(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "parallel if dualdma mode enable."]
        #[inline(always)]
        pub const fn set_dualdma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "parallel if data mode."]
        #[must_use]
        #[inline(always)]
        pub const fn msk_size(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "parallel if data mode."]
        #[inline(always)]
        pub const fn set_msk_size(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u8) & 0x03) << 2usize);
        }
        #[doc = "parallel if tx addr toggle enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_tog_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "parallel if tx addr toggle enable."]
        #[inline(always)]
        pub const fn set_tx_tog_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "parallel if rx addr toggle enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_tog_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "parallel if rx addr toggle enable."]
        #[inline(always)]
        pub const fn set_rx_tog_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "parallel if tx ack by hardware."]
        #[must_use]
        #[inline(always)]
        pub const fn hw_ack(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "parallel if tx ack by hardware."]
        #[inline(always)]
        pub const fn set_hw_ack(&mut self, val: bool) {
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
                .field("mode", &self.mode())
                .field("dualdma", &self.dualdma())
                .field("msk_size", &self.msk_size())
                .field("tx_tog_en", &self.tx_tog_en())
                .field("rx_tog_en", &self.rx_tog_en())
                .field("hw_ack", &self.hw_ack())
                .finish()
        }
    }
    #[doc = "parallel if tx or rx control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u8);
    impl Ctrl {
        #[doc = "parallel if enable."]
        #[must_use]
        #[inline(always)]
        pub const fn enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "parallel if enable."]
        #[inline(always)]
        pub const fn set_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "parallel if dma enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "parallel if dma enable."]
        #[inline(always)]
        pub const fn set_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "parallel if transmit software trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn sw_act(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "parallel if transmit software trigger."]
        #[inline(always)]
        pub const fn set_sw_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "parallel if all clear."]
        #[must_use]
        #[inline(always)]
        pub const fn all_clr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "parallel if all clear."]
        #[inline(always)]
        pub const fn set_all_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "parallel if tx and rx logic clear, high action."]
        #[must_use]
        #[inline(always)]
        pub const fn trx_rst(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "parallel if tx and rx logic clear, high action."]
        #[inline(always)]
        pub const fn set_trx_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
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
                .field("enable", &self.enable())
                .field("dma_en", &self.dma_en())
                .field("sw_act", &self.sw_act())
                .field("all_clr", &self.all_clr())
                .field("trx_rst", &self.trx_rst())
                .finish()
        }
    }
    #[doc = "parallel if dma length0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaLen0(pub u16);
    impl DmaLen0 {
        #[doc = "parallel if dma length0."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_len0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "parallel if dma length0."]
        #[inline(always)]
        pub const fn set_dma_len0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u16) & 0x0fff) << 0usize);
        }
    }
    impl Default for DmaLen0 {
        #[inline(always)]
        fn default() -> DmaLen0 {
            DmaLen0(0)
        }
    }
    impl core::fmt::Debug for DmaLen0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaLen0")
                .field("dma_len0", &self.dma_len0())
                .finish()
        }
    }
    #[doc = "parallel if dma length1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaLen1(pub u16);
    impl DmaLen1 {
        #[doc = "parallel if dma length1."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_len1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "parallel if dma length1."]
        #[inline(always)]
        pub const fn set_dma_len1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u16) & 0x0fff) << 0usize);
        }
    }
    impl Default for DmaLen1 {
        #[inline(always)]
        fn default() -> DmaLen1 {
            DmaLen1(0)
        }
    }
    impl core::fmt::Debug for DmaLen1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaLen1")
                .field("dma_len1", &self.dma_len1())
                .finish()
        }
    }
    #[doc = "parallel if interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u8);
    impl IntEn {
        #[doc = "parallel if transmit done interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_t_done(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "parallel if transmit done interrupt enable."]
        #[inline(always)]
        pub const fn set_ie_t_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "parallel if receive done interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_r_done(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "parallel if receive done interrupt enable."]
        #[inline(always)]
        pub const fn set_ie_r_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "parallel if fifo overflow interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_fifo_ov(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "parallel if fifo overflow interrupt enable."]
        #[inline(always)]
        pub const fn set_ie_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "parallel if tx burst done interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_b_done(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "parallel if tx burst done interrupt enable."]
        #[inline(always)]
        pub const fn set_ie_b_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
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
                .field("ie_t_done", &self.ie_t_done())
                .field("ie_r_done", &self.ie_r_done())
                .field("ie_fifo_ov", &self.ie_fifo_ov())
                .field("ie_b_done", &self.ie_b_done())
                .finish()
        }
    }
    #[doc = "parallel if interrupt flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFlag(pub u8);
    impl IntFlag {
        #[doc = "interrupt flag for parallel if transmit done."]
        #[must_use]
        #[inline(always)]
        pub const fn if_t_done(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for parallel if transmit done."]
        #[inline(always)]
        pub const fn set_if_t_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "interrupt flag for parallel if receive done."]
        #[must_use]
        #[inline(always)]
        pub const fn if_r_done(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for parallel if receive done."]
        #[inline(always)]
        pub const fn set_if_r_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "interrupt flag for parallel if FIFO overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn if_fifo_ov(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for parallel if FIFO overflow."]
        #[inline(always)]
        pub const fn set_if_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "interrupt flag for parallel if tx burst done."]
        #[must_use]
        #[inline(always)]
        pub const fn if_b_done(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for parallel if tx burst done."]
        #[inline(always)]
        pub const fn set_if_b_done(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
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
                .field("if_t_done", &self.if_t_done())
                .field("if_r_done", &self.if_r_done())
                .field("if_fifo_ov", &self.if_fifo_ov())
                .field("if_b_done", &self.if_b_done())
                .finish()
        }
    }
    #[doc = "parallel rtx status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RtxStatus(pub u8);
    impl RtxStatus {
        #[doc = "CRC error occur."]
        #[must_use]
        #[inline(always)]
        pub const fn crc_err(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "CRC error occur."]
        #[inline(always)]
        pub const fn set_crc_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "rx and tx sequence number mismatch."]
        #[must_use]
        #[inline(always)]
        pub const fn num_mis(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "rx and tx sequence number mismatch."]
        #[inline(always)]
        pub const fn set_num_mis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
    }
    impl Default for RtxStatus {
        #[inline(always)]
        fn default() -> RtxStatus {
            RtxStatus(0)
        }
    }
    impl core::fmt::Debug for RtxStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RtxStatus")
                .field("crc_err", &self.crc_err())
                .field("num_mis", &self.num_mis())
                .finish()
        }
    }
    #[doc = "parallel if dma rx addr0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxAddr0(pub u32);
    impl RxAddr0 {
        #[doc = "parallel if dma rx addr0."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_addr0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "parallel if dma rx addr0."]
        #[inline(always)]
        pub const fn set_rx_addr0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for RxAddr0 {
        #[inline(always)]
        fn default() -> RxAddr0 {
            RxAddr0(0)
        }
    }
    impl core::fmt::Debug for RxAddr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxAddr0")
                .field("rx_addr0", &self.rx_addr0())
                .finish()
        }
    }
    #[doc = "parallel if dma rx addr1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxAddr1(pub u32);
    impl RxAddr1 {
        #[doc = "parallel if dma rx addr1."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_addr1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "parallel if dma rx addr1."]
        #[inline(always)]
        pub const fn set_rx_addr1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for RxAddr1 {
        #[inline(always)]
        fn default() -> RxAddr1 {
            RxAddr1(0)
        }
    }
    impl core::fmt::Debug for RxAddr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxAddr1")
                .field("rx_addr1", &self.rx_addr1())
                .finish()
        }
    }
    #[doc = "parallel if receive length0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxLen0(pub u16);
    impl RxLen0 {
        #[doc = "parallel if dma length0."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_len0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "parallel if dma length0."]
        #[inline(always)]
        pub const fn set_rx_len0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u16) & 0x0fff) << 0usize);
        }
    }
    impl Default for RxLen0 {
        #[inline(always)]
        fn default() -> RxLen0 {
            RxLen0(0)
        }
    }
    impl core::fmt::Debug for RxLen0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxLen0")
                .field("rx_len0", &self.rx_len0())
                .finish()
        }
    }
    #[doc = "parallel if receive length1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxLen1(pub u16);
    impl RxLen1 {
        #[doc = "parallel if dma length1."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_len1(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "parallel if dma length1."]
        #[inline(always)]
        pub const fn set_rx_len1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u16) & 0x0fff) << 0usize);
        }
    }
    impl Default for RxLen1 {
        #[inline(always)]
        fn default() -> RxLen1 {
            RxLen1(0)
        }
    }
    impl core::fmt::Debug for RxLen1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxLen1")
                .field("rx_len1", &self.rx_len1())
                .finish()
        }
    }
    #[doc = "parallel RX sequence ctrl."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RxSc(pub u8);
    impl RxSc {
        #[doc = "parallel if rx sequence num."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "parallel if rx sequence num."]
        #[inline(always)]
        pub const fn set_rx_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "parallel if rx addr toggle flag."]
        #[must_use]
        #[inline(always)]
        pub const fn rx_tog(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "parallel if rx addr toggle flag."]
        #[inline(always)]
        pub const fn set_rx_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for RxSc {
        #[inline(always)]
        fn default() -> RxSc {
            RxSc(0)
        }
    }
    impl core::fmt::Debug for RxSc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RxSc")
                .field("rx_num", &self.rx_num())
                .field("rx_tog", &self.rx_tog())
                .finish()
        }
    }
    #[doc = "parallel if dma tx addr0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxAddr0(pub u32);
    impl TxAddr0 {
        #[doc = "parallel if dma tx addr0."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_addr0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "parallel if dma tx addr0."]
        #[inline(always)]
        pub const fn set_tx_addr0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for TxAddr0 {
        #[inline(always)]
        fn default() -> TxAddr0 {
            TxAddr0(0)
        }
    }
    impl core::fmt::Debug for TxAddr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxAddr0")
                .field("tx_addr0", &self.tx_addr0())
                .finish()
        }
    }
    #[doc = "parallel if dma tx addr1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxAddr1(pub u32);
    impl TxAddr1 {
        #[doc = "parallel if dma tx addr1."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_addr1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "parallel if dma tx addr1."]
        #[inline(always)]
        pub const fn set_tx_addr1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for TxAddr1 {
        #[inline(always)]
        fn default() -> TxAddr1 {
            TxAddr1(0)
        }
    }
    impl core::fmt::Debug for TxAddr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxAddr1")
                .field("tx_addr1", &self.tx_addr1())
                .finish()
        }
    }
    #[doc = "parallel TX sequence ctrl."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxSc(pub u8);
    impl TxSc {
        #[doc = "parallel if tx sequence num."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_num(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "parallel if tx sequence num."]
        #[inline(always)]
        pub const fn set_tx_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "parallel if tx addr toggle flag."]
        #[must_use]
        #[inline(always)]
        pub const fn tx_tog(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "parallel if tx addr toggle flag."]
        #[inline(always)]
        pub const fn set_tx_tog(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for TxSc {
        #[inline(always)]
        fn default() -> TxSc {
            TxSc(0)
        }
    }
    impl core::fmt::Debug for TxSc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TxSc")
                .field("tx_num", &self.tx_num())
                .field("tx_tog", &self.tx_tog())
                .finish()
        }
    }
    #[doc = "parallel if user defined field 0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Udf0(pub u32);
    impl Udf0 {
        #[doc = "parallel if user defined field 0 register."]
        #[must_use]
        #[inline(always)]
        pub const fn udf0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "parallel if user defined field 0 register."]
        #[inline(always)]
        pub const fn set_udf0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
        }
    }
    impl Default for Udf0 {
        #[inline(always)]
        fn default() -> Udf0 {
            Udf0(0)
        }
    }
    impl core::fmt::Debug for Udf0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Udf0").field("udf0", &self.udf0()).finish()
        }
    }
    #[doc = "parallel if user defined field 1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Udf1(pub u32);
    impl Udf1 {
        #[doc = "parallel if user defined field 1 register."]
        #[must_use]
        #[inline(always)]
        pub const fn udf1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "parallel if user defined field 1 register."]
        #[inline(always)]
        pub const fn set_udf1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
        }
    }
    impl Default for Udf1 {
        #[inline(always)]
        fn default() -> Udf1 {
            Udf1(0)
        }
    }
    impl core::fmt::Debug for Udf1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Udf1").field("udf1", &self.udf1()).finish()
        }
    }
}
