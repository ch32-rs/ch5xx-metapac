#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "TMR_FULL register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TmrFull {
    ptr: *mut u8,
}
unsafe impl Send for TmrFull {}
unsafe impl Sync for TmrFull {}
impl TmrFull {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TMR_FULL mode control."]
    #[inline(always)]
    pub const fn ctrl_mod(self) -> crate::common::Reg<regs::CtrlModFull, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "TMR_FULL DMA control register."]
    #[inline(always)]
    pub const fn ctrl_dma(self) -> crate::common::Reg<regs::CtrlDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "TMR_FULL interrupt enable."]
    #[inline(always)]
    pub const fn inter_en(self) -> crate::common::Reg<regs::InterEnFull, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "TMR_FULL interrupt flag."]
    #[inline(always)]
    pub const fn int_flag(self) -> crate::common::Reg<regs::IntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "TMR_FULL FIFO count status."]
    #[inline(always)]
    pub const fn fifo_count(self) -> crate::common::Reg<regs::FifoCount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "TMR_FULL current count."]
    #[inline(always)]
    pub const fn count(self) -> crate::common::Reg<regs::Count, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "TMR_FULL end count value, only low 28 bit."]
    #[inline(always)]
    pub const fn cnt_end(self) -> crate::common::Reg<regs::CntEnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "TMR_FULL FIFO register, only low 28 bit."]
    #[inline(always)]
    pub const fn fifo(self) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "DMA current buffer address register."]
    #[inline(always)]
    pub const fn dma_now(self) -> crate::common::Reg<regs::DmaNow, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "DMA start buffer address register."]
    #[inline(always)]
    pub const fn dma_beg(self) -> crate::common::Reg<regs::DmaBeg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "DMA end buffer address register."]
    #[inline(always)]
    pub const fn dma_end(self) -> crate::common::Reg<regs::DmaEnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
#[doc = "TMR_NODMA register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TmrNodma {
    ptr: *mut u8,
}
unsafe impl Send for TmrNodma {}
unsafe impl Sync for TmrNodma {}
impl TmrNodma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "TMR_NODMA mode control."]
    #[inline(always)]
    pub const fn ctrl_mod(self) -> crate::common::Reg<regs::CtrlModNodma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "TMR_NODMA interrupt enable."]
    #[inline(always)]
    pub const fn inter_en(self) -> crate::common::Reg<regs::InterEnNodma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "TMR_NODMA interrupt flag."]
    #[inline(always)]
    pub const fn int_flag(self) -> crate::common::Reg<regs::IntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "TMR_NODMA FIFO count status."]
    #[inline(always)]
    pub const fn fifo_count(self) -> crate::common::Reg<regs::FifoCount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "TMR_NODMA current count."]
    #[inline(always)]
    pub const fn count(self) -> crate::common::Reg<regs::Count, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "TMR_NODMA end count value, only low 28 bit."]
    #[inline(always)]
    pub const fn cnt_end(self) -> crate::common::Reg<regs::CntEnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "TMR_NODMA FIFO register, only low 28 bit."]
    #[inline(always)]
    pub const fn fifo(self) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
pub mod regs {
    #[doc = "TMR_FULL end count value, only low 28 bit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CntEnd(pub u32);
    impl CntEnd {
        #[doc = "The number of timer ticks in timer mode."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt_end(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "The number of timer ticks in timer mode."]
        #[inline(always)]
        pub const fn set_cnt_end(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
    }
    impl Default for CntEnd {
        #[inline(always)]
        fn default() -> CntEnd {
            CntEnd(0)
        }
    }
    impl core::fmt::Debug for CntEnd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CntEnd")
                .field("cnt_end", &self.cnt_end())
                .finish()
        }
    }
    #[doc = "TMR_FULL current count."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Count(pub u32);
    impl Count {
        #[doc = "TMR_FULL current count."]
        #[must_use]
        #[inline(always)]
        pub const fn count(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "TMR_FULL current count."]
        #[inline(always)]
        pub const fn set_count(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
    }
    impl Default for Count {
        #[inline(always)]
        fn default() -> Count {
            Count(0)
        }
    }
    impl core::fmt::Debug for Count {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Count")
                .field("count", &self.count())
                .finish()
        }
    }
    #[doc = "TMR_FULL DMA control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlDma(pub u8);
    impl CtrlDma {
        #[doc = "DMA function enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA function enable."]
        #[inline(always)]
        pub const fn set_dma_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "DMA burst operation."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_burst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA burst operation."]
        #[inline(always)]
        pub const fn set_dma_burst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "DMA address round-robin mode enabled."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_loop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DMA address round-robin mode enabled."]
        #[inline(always)]
        pub const fn set_dma_loop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
    }
    impl Default for CtrlDma {
        #[inline(always)]
        fn default() -> CtrlDma {
            CtrlDma(0)
        }
    }
    impl core::fmt::Debug for CtrlDma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CtrlDma")
                .field("dma_enable", &self.dma_enable())
                .field("dma_burst", &self.dma_burst())
                .field("dma_loop", &self.dma_loop())
                .finish()
        }
    }
    #[doc = "TMR_FULL mode control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlModFull(pub u8);
    impl CtrlModFull {
        #[doc = "timer in mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode_in(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "timer in mode."]
        #[inline(always)]
        pub const fn set_mode_in(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "force clear timer FIFO and count."]
        #[must_use]
        #[inline(always)]
        pub const fn all_clear(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force clear timer FIFO and count."]
        #[inline(always)]
        pub const fn set_all_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "timer count enable."]
        #[must_use]
        #[inline(always)]
        pub const fn count_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "timer count enable."]
        #[inline(always)]
        pub const fn set_count_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "timer output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn out_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "timer output enable."]
        #[inline(always)]
        pub const fn set_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "timer PWM output polarity / Capture pulse width."]
        #[must_use]
        #[inline(always)]
        pub const fn out_polar_rb_cat_width(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "timer PWM output polarity / Capture pulse width."]
        #[inline(always)]
        pub const fn set_out_polar_rb_cat_width(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "timer serial codec mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode_nrzi(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "timer serial codec mode."]
        #[inline(always)]
        pub const fn set_mode_nrzi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "timer PWM repeat mode / timer capture edge mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm_repeat_rb_catch_edge(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "timer PWM repeat mode / timer capture edge mode."]
        #[inline(always)]
        pub const fn set_pwm_repeat_rb_catch_edge(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for CtrlModFull {
        #[inline(always)]
        fn default() -> CtrlModFull {
            CtrlModFull(0)
        }
    }
    impl core::fmt::Debug for CtrlModFull {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CtrlModFull")
                .field("mode_in", &self.mode_in())
                .field("all_clear", &self.all_clear())
                .field("count_en", &self.count_en())
                .field("out_en", &self.out_en())
                .field("out_polar_rb_cat_width", &self.out_polar_rb_cat_width())
                .field("mode_nrzi", &self.mode_nrzi())
                .field("pwm_repeat_rb_catch_edge", &self.pwm_repeat_rb_catch_edge())
                .finish()
        }
    }
    #[doc = "TMR_NODMA mode control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlModNodma(pub u8);
    impl CtrlModNodma {
        #[doc = "timer in mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode_in(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "timer in mode."]
        #[inline(always)]
        pub const fn set_mode_in(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "force clear timer FIFO and count."]
        #[must_use]
        #[inline(always)]
        pub const fn all_clear(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force clear timer FIFO and count."]
        #[inline(always)]
        pub const fn set_all_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "timer count enable."]
        #[must_use]
        #[inline(always)]
        pub const fn count_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "timer count enable."]
        #[inline(always)]
        pub const fn set_count_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "timer output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn out_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "timer output enable."]
        #[inline(always)]
        pub const fn set_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "timer PWM output polarity / Capture pulse width."]
        #[must_use]
        #[inline(always)]
        pub const fn out_polar_rb_cap_width(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "timer PWM output polarity / Capture pulse width."]
        #[inline(always)]
        pub const fn set_out_polar_rb_cap_width(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "external clock counter mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode_count(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "external clock counter mode."]
        #[inline(always)]
        pub const fn set_mode_count(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "timer PWM repeat mode / timer capture edge mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm_repeat_rb_cap_edge(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "timer PWM repeat mode / timer capture edge mode."]
        #[inline(always)]
        pub const fn set_pwm_repeat_rb_cap_edge(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for CtrlModNodma {
        #[inline(always)]
        fn default() -> CtrlModNodma {
            CtrlModNodma(0)
        }
    }
    impl core::fmt::Debug for CtrlModNodma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CtrlModNodma")
                .field("mode_in", &self.mode_in())
                .field("all_clear", &self.all_clear())
                .field("count_en", &self.count_en())
                .field("out_en", &self.out_en())
                .field("out_polar_rb_cap_width", &self.out_polar_rb_cap_width())
                .field("mode_count", &self.mode_count())
                .field("pwm_repeat_rb_cap_edge", &self.pwm_repeat_rb_cap_edge())
                .finish()
        }
    }
    #[doc = "DMA start buffer address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaBeg(pub u32);
    impl DmaBeg {
        #[doc = "DMA start buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_beg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "DMA start buffer address."]
        #[inline(always)]
        pub const fn set_dma_beg(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
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
    #[doc = "DMA end buffer address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaEnd(pub u32);
    impl DmaEnd {
        #[doc = "DMA end buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_end(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "DMA end buffer address."]
        #[inline(always)]
        pub const fn set_dma_end(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
    }
    impl Default for DmaEnd {
        #[inline(always)]
        fn default() -> DmaEnd {
            DmaEnd(0)
        }
    }
    impl core::fmt::Debug for DmaEnd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaEnd")
                .field("dma_end", &self.dma_end())
                .finish()
        }
    }
    #[doc = "DMA current buffer address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaNow(pub u32);
    impl DmaNow {
        #[doc = "DMA current buffer address."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_now(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "DMA current buffer address."]
        #[inline(always)]
        pub const fn set_dma_now(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
    }
    impl Default for DmaNow {
        #[inline(always)]
        fn default() -> DmaNow {
            DmaNow(0)
        }
    }
    impl core::fmt::Debug for DmaNow {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaNow")
                .field("dma_now", &self.dma_now())
                .finish()
        }
    }
    #[doc = "TMR_FULL FIFO register, only low 28 bit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifo(pub u32);
    impl Fifo {
        #[doc = "TMR_FULL FIFO current count."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "TMR_FULL FIFO current count."]
        #[inline(always)]
        pub const fn set_fifo(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
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
    #[doc = "TMR_FULL FIFO count status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FifoCount(pub u8);
    impl FifoCount {
        #[doc = "TMR_FULL FIFO count status."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_count(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "TMR_FULL FIFO count status."]
        #[inline(always)]
        pub const fn set_fifo_count(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for FifoCount {
        #[inline(always)]
        fn default() -> FifoCount {
            FifoCount(0)
        }
    }
    impl core::fmt::Debug for FifoCount {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FifoCount")
                .field("fifo_count", &self.fifo_count())
                .finish()
        }
    }
    #[doc = "TMR_FULL interrupt flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFlag(pub u8);
    impl IntFlag {
        #[doc = "interrupt flag for timer capture count timeout or PWM cycle. end."]
        #[must_use]
        #[inline(always)]
        pub const fn if_cyc_end(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for timer capture count timeout or PWM cycle. end."]
        #[inline(always)]
        pub const fn set_if_cyc_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "interrupt flag for timer capture input action or PWM. trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn if_data_act(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for timer capture input action or PWM. trigger."]
        #[inline(always)]
        pub const fn set_if_data_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "interrupt flag for timer FIFO half."]
        #[must_use]
        #[inline(always)]
        pub const fn if_fifo_hf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for timer FIFO half."]
        #[inline(always)]
        pub const fn set_if_fifo_hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "interrupt flag for timer DMA completion."]
        #[must_use]
        #[inline(always)]
        pub const fn if_dma_end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for timer DMA completion."]
        #[inline(always)]
        pub const fn set_if_dma_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "interrupt flag for timer FIFO overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn if_fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for timer FIFO overflow."]
        #[inline(always)]
        pub const fn set_if_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "DMA error responde flag."]
        #[must_use]
        #[inline(always)]
        pub const fn if_dma_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DMA error responde flag."]
        #[inline(always)]
        pub const fn set_if_dma_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
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
                .field("if_cyc_end", &self.if_cyc_end())
                .field("if_data_act", &self.if_data_act())
                .field("if_fifo_hf", &self.if_fifo_hf())
                .field("if_dma_end", &self.if_dma_end())
                .field("if_fifo_ov", &self.if_fifo_ov())
                .field("if_dma_err", &self.if_dma_err())
                .finish()
        }
    }
    #[doc = "TMR_FULL interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InterEnFull(pub u8);
    impl InterEnFull {
        #[doc = "enable interrupt for timer capture count timeout or PWM. cycle end."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_cyc_end(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for timer capture count timeout or PWM. cycle end."]
        #[inline(always)]
        pub const fn set_ie_cyc_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable interrupt for timer capture input action or PWM. trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_data_act(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for timer capture input action or PWM. trigger."]
        #[inline(always)]
        pub const fn set_ie_data_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "enable interrupt for timer FIFO half (capture fifo >=4 or. PWM fifo lower than3)."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_fifo_hf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for timer FIFO half (capture fifo >=4 or. PWM fifo lower than3)."]
        #[inline(always)]
        pub const fn set_ie_fifo_hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable interrupt for timer DMA completion."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_dma_end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for timer DMA completion."]
        #[inline(always)]
        pub const fn set_ie_dma_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "enable interrupt for timer FIFO overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for timer FIFO overflow."]
        #[inline(always)]
        pub const fn set_ie_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable DMA error responde interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_dma_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable DMA error responde interrupt."]
        #[inline(always)]
        pub const fn set_ie_dma_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for InterEnFull {
        #[inline(always)]
        fn default() -> InterEnFull {
            InterEnFull(0)
        }
    }
    impl core::fmt::Debug for InterEnFull {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("InterEnFull")
                .field("ie_cyc_end", &self.ie_cyc_end())
                .field("ie_data_act", &self.ie_data_act())
                .field("ie_fifo_hf", &self.ie_fifo_hf())
                .field("ie_dma_end", &self.ie_dma_end())
                .field("ie_fifo_ov", &self.ie_fifo_ov())
                .field("ie_dma_err", &self.ie_dma_err())
                .finish()
        }
    }
    #[doc = "TMR_NODMA interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InterEnNodma(pub u8);
    impl InterEnNodma {
        #[doc = "enable interrupt for timer capture count timeout or PWM. cycle end."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_cyc_end(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for timer capture count timeout or PWM. cycle end."]
        #[inline(always)]
        pub const fn set_ie_cyc_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable interrupt for timer capture input action or PWM. trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_data_act(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for timer capture input action or PWM. trigger."]
        #[inline(always)]
        pub const fn set_ie_data_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "enable interrupt for timer FIFO half (capture fifo >=4 or. PWM fifo lower than3)."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_fifo_hf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for timer FIFO half (capture fifo >=4 or. PWM fifo lower than3)."]
        #[inline(always)]
        pub const fn set_ie_fifo_hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable interrupt for timer DMA completion."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_dma_end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for timer DMA completion."]
        #[inline(always)]
        pub const fn set_ie_dma_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "enable interrupt for timer FIFO overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for timer FIFO overflow."]
        #[inline(always)]
        pub const fn set_ie_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable DMA error responde interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_dma_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable DMA error responde interrupt."]
        #[inline(always)]
        pub const fn set_ie_dma_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "Forcibly enable TIMER0,TIMER1 and TIMER2 at the same time."]
        #[must_use]
        #[inline(always)]
        pub const fn force_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Forcibly enable TIMER0,TIMER1 and TIMER2 at the same time."]
        #[inline(always)]
        pub const fn set_force_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for InterEnNodma {
        #[inline(always)]
        fn default() -> InterEnNodma {
            InterEnNodma(0)
        }
    }
    impl core::fmt::Debug for InterEnNodma {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("InterEnNodma")
                .field("ie_cyc_end", &self.ie_cyc_end())
                .field("ie_data_act", &self.ie_data_act())
                .field("ie_fifo_hf", &self.ie_fifo_hf())
                .field("ie_dma_end", &self.ie_dma_end())
                .field("ie_fifo_ov", &self.ie_fifo_ov())
                .field("ie_dma_err", &self.ie_dma_err())
                .field("force_en", &self.force_en())
                .finish()
        }
    }
}
