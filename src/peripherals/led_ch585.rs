#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "LED register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Led {
    ptr: *mut u8,
}
unsafe impl Send for Led {}
unsafe impl Sync for Led {}
impl Led {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RW, LED mode configuration register."]
    #[inline(always)]
    pub const fn ctrl_mod(self) -> crate::common::Reg<regs::CtrlMod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RW, LED serial clock divider register."]
    #[inline(always)]
    pub const fn clock_div(self) -> crate::common::Reg<regs::ClockDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "RW, LED mode configuration register 1."]
    #[inline(always)]
    pub const fn ctrl_mod1(self) -> crate::common::Reg<regs::CtrlMod1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "RW1, LED status register."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "WO, LED data FIFO register."]
    #[inline(always)]
    pub const fn fifo(self) -> crate::common::Reg<regs::Fifo, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "WO, LED DMA transmission length."]
    #[inline(always)]
    pub const fn dma_len(self) -> crate::common::Reg<regs::DmaLen, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "RO, LED DMA remaining count register."]
    #[inline(always)]
    pub const fn dma_cnt(self) -> crate::common::Reg<regs::DmaCnt, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "RW, LED DMA starting address."]
    #[inline(always)]
    pub const fn dma_beg(self) -> crate::common::Reg<regs::DmaBeg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "RO, LED DMA current address."]
    #[inline(always)]
    pub const fn dma_cur(self) -> crate::common::Reg<regs::DmaCur, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "RW, LED serial clock divider register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClockDiv(pub u8);
    impl ClockDiv {
        #[doc = "LED output clock frequency division coefficient."]
        #[must_use]
        #[inline(always)]
        pub const fn clock_div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "LED output clock frequency division coefficient."]
        #[inline(always)]
        pub const fn set_clock_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for ClockDiv {
        #[inline(always)]
        fn default() -> ClockDiv {
            ClockDiv(0)
        }
    }
    impl core::fmt::Debug for ClockDiv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ClockDiv")
                .field("clock_div", &self.clock_div())
                .finish()
        }
    }
    #[doc = "RW, LED mode configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlMod(pub u8);
    impl CtrlMod {
        #[doc = "Selection of LED serial data bit sequence."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_order(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Selection of LED serial data bit sequence."]
        #[inline(always)]
        pub const fn set_bit_order(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "LED FIFO/counter/interrupt flag reset to zero."]
        #[must_use]
        #[inline(always)]
        pub const fn all_clear(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "LED FIFO/counter/interrupt flag reset to zero."]
        #[inline(always)]
        pub const fn set_all_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "LED data output polarity control bit."]
        #[must_use]
        #[inline(always)]
        pub const fn out_polar(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "LED data output polarity control bit."]
        #[inline(always)]
        pub const fn set_out_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "LED signal output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn out_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "LED signal output enable."]
        #[inline(always)]
        pub const fn set_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "LED DMA function and DMA interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "LED DMA function and DMA interrupt enable."]
        #[inline(always)]
        pub const fn set_dma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "FIFO count over half interrupt enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_fifo(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO count over half interrupt enabled."]
        #[inline(always)]
        pub const fn set_ie_fifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "LED channel mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn chan_mod(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "LED channel mode selection."]
        #[inline(always)]
        pub const fn set_chan_mod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for CtrlMod {
        #[inline(always)]
        fn default() -> CtrlMod {
            CtrlMod(0)
        }
    }
    impl core::fmt::Debug for CtrlMod {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CtrlMod")
                .field("bit_order", &self.bit_order())
                .field("all_clear", &self.all_clear())
                .field("out_polar", &self.out_polar())
                .field("out_en", &self.out_en())
                .field("dma_en", &self.dma_en())
                .field("ie_fifo", &self.ie_fifo())
                .field("chan_mod", &self.chan_mod())
                .finish()
        }
    }
    #[doc = "RW, LED mode configuration register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlMod1(pub u8);
    impl CtrlMod1 {
        #[doc = "Enable LED DMA data cycling usage."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_loop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable LED DMA data cycling usage."]
        #[inline(always)]
        pub const fn set_dma_loop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "DMA All Data Transfer End Interrupt Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_send_end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA All Data Transfer End Interrupt Enable."]
        #[inline(always)]
        pub const fn set_ie_send_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
    }
    impl Default for CtrlMod1 {
        #[inline(always)]
        fn default() -> CtrlMod1 {
            CtrlMod1(0)
        }
    }
    impl core::fmt::Debug for CtrlMod1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CtrlMod1")
                .field("dma_loop", &self.dma_loop())
                .field("ie_send_end", &self.ie_send_end())
                .finish()
        }
    }
    #[doc = "RW, LED DMA starting address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaBeg(pub u32);
    impl DmaBeg {
        #[doc = "DMA starting address."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_beg(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x7fff;
            val as u16
        }
        #[doc = "DMA starting address."]
        #[inline(always)]
        pub const fn set_dma_beg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 2usize)) | (((val as u32) & 0x7fff) << 2usize);
        }
    }
    impl Default for DmaBeg {
        #[inline(always)]
        fn default() -> DmaBeg {
            DmaBeg(0)
        }
    }
    impl core::fmt::Debug for DmaBeg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaBeg")
                .field("dma_beg", &self.dma_beg())
                .finish()
        }
    }
    #[doc = "RO, LED DMA remaining count register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaCnt(pub u16);
    impl DmaCnt {
        #[doc = "LED_SMA_maIN Main Buffer Current DMA Remaining Words (16 bits) Count."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "LED_SMA_maIN Main Buffer Current DMA Remaining Words (16 bits) Count."]
        #[inline(always)]
        pub const fn set_dma_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u16) & 0x0fff) << 0usize);
        }
    }
    impl Default for DmaCnt {
        #[inline(always)]
        fn default() -> DmaCnt {
            DmaCnt(0)
        }
    }
    impl core::fmt::Debug for DmaCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaCnt")
                .field("dma_cnt", &self.dma_cnt())
                .finish()
        }
    }
    #[doc = "RO, LED DMA current address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaCur(pub u32);
    impl DmaCur {
        #[doc = "LED DMA current address."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_cur(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x7fff;
            val as u16
        }
        #[doc = "LED DMA current address."]
        #[inline(always)]
        pub const fn set_dma_cur(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 2usize)) | (((val as u32) & 0x7fff) << 2usize);
        }
    }
    impl Default for DmaCur {
        #[inline(always)]
        fn default() -> DmaCur {
            DmaCur(0)
        }
    }
    impl core::fmt::Debug for DmaCur {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaCur")
                .field("dma_cur", &self.dma_cur())
                .finish()
        }
    }
    #[doc = "WO, LED DMA transmission length."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaLen(pub u16);
    impl DmaLen {
        #[doc = "DMA transmission count length."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_length(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "DMA transmission count length."]
        #[inline(always)]
        pub const fn set_dma_length(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u16) & 0x0fff) << 0usize);
        }
    }
    impl Default for DmaLen {
        #[inline(always)]
        fn default() -> DmaLen {
            DmaLen(0)
        }
    }
    impl core::fmt::Debug for DmaLen {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaLen")
                .field("dma_length", &self.dma_length())
                .finish()
        }
    }
    #[doc = "WO, LED data FIFO register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifo(pub u32);
    impl Fifo {
        #[doc = "LED data FIFO input, 32-bit write."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "LED data FIFO input, 32-bit write."]
        #[inline(always)]
        pub const fn set_fifo(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Fifo {
        #[inline(always)]
        fn default() -> Fifo {
            Fifo(0)
        }
    }
    impl core::fmt::Debug for Fifo {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fifo").field("fifo", &self.fifo()).finish()
        }
    }
    #[doc = "RW1, LED status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u16);
    impl Status {
        #[doc = "The byte count value in the current FIFO must be even."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_count(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "The byte count value in the current FIFO must be even."]
        #[inline(always)]
        pub const fn set_fifo_count(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u16) & 0x0f) << 0usize);
        }
        #[doc = "Current LED clock signal level status."]
        #[must_use]
        #[inline(always)]
        pub const fn clock(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Current LED clock signal level status."]
        #[inline(always)]
        pub const fn set_clock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "FIFO count over half interrupt flag, write 1 to reset or write R16_SED_LFO to reset."]
        #[must_use]
        #[inline(always)]
        pub const fn if_fifo(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO count over half interrupt flag, write 1 to reset or write R16_SED_LFO to reset."]
        #[inline(always)]
        pub const fn set_if_fifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "Loading data results."]
        #[must_use]
        #[inline(always)]
        pub const fn load_fail(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Loading data results."]
        #[inline(always)]
        pub const fn set_load_fail(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "DMA completion flag, write 1 to reset or write R16_SED-DMA_CNT to reset."]
        #[must_use]
        #[inline(always)]
        pub const fn if_dma_end(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "DMA completion flag, write 1 to reset or write R16_SED-DMA_CNT to reset."]
        #[inline(always)]
        pub const fn set_if_dma_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "DMA transfer end interrupt flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn if_dma_int(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "DMA transfer end interrupt flag bit."]
        #[inline(always)]
        pub const fn set_if_dma_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
    }
    impl Default for Status {
        #[inline(always)]
        fn default() -> Status {
            Status(0)
        }
    }
    impl core::fmt::Debug for Status {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Status")
                .field("fifo_count", &self.fifo_count())
                .field("clock", &self.clock())
                .field("if_fifo", &self.if_fifo())
                .field("load_fail", &self.load_fail())
                .field("if_dma_end", &self.if_dma_end())
                .field("if_dma_int", &self.if_dma_int())
                .finish()
        }
    }
}
