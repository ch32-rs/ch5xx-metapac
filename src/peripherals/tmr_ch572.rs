#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Timer register."]
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
    #[doc = "RW, TMR mode control."]
    #[inline(always)]
    pub const fn ctrl_mod(self) -> crate::common::Reg<regs::CtrlMod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RO/WO, DMA control register."]
    #[inline(always)]
    pub const fn ctrl_dma(self) -> crate::common::Reg<regs::CtrlDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "RW, TMR interrupt enable."]
    #[inline(always)]
    pub const fn inter_en(self) -> crate::common::Reg<regs::InterEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "RW1, TMR interrupt flag."]
    #[inline(always)]
    pub const fn int_flag(self) -> crate::common::Reg<regs::IntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "RO, TMR FIFO count status."]
    #[inline(always)]
    pub const fn fifo_count(self) -> crate::common::Reg<regs::FifoCount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "RO, TMR current count."]
    #[inline(always)]
    pub const fn count(self) -> crate::common::Reg<regs::Count, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RW, TMR end count value, only low 26 bit."]
    #[inline(always)]
    pub const fn cnt_end(self) -> crate::common::Reg<regs::CntEnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "RO/WO, TMR FIFO register, only low 26 bit."]
    #[inline(always)]
    pub const fn fifo(self) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "RO, DMA current buffer address."]
    #[inline(always)]
    pub const fn dma_now(self) -> crate::common::Reg<regs::DmaNow, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "RW, DMA start buffer address."]
    #[inline(always)]
    pub const fn dma_beg(self) -> crate::common::Reg<regs::DmaBeg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "RW, DMA end buffer address."]
    #[inline(always)]
    pub const fn dma_end(self) -> crate::common::Reg<regs::DmaEnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "RW, TMR end count value, only low 26 bit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CntEnd(pub u32);
    impl CntEnd {
        #[doc = "RW1,TMR end count value."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt_end(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW1,TMR end count value."]
        #[inline(always)]
        pub const fn set_cnt_end(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
    #[doc = "RO, TMR current count."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Count(pub u32);
    impl Count {
        #[doc = "RW1,TMR current count."]
        #[must_use]
        #[inline(always)]
        pub const fn count(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "RW1,TMR current count."]
        #[inline(always)]
        pub const fn set_count(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
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
    #[doc = "RO/WO, DMA control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlDma(pub u8);
    impl CtrlDma {
        #[doc = "RW, DMA function enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, DMA function enable bit."]
        #[inline(always)]
        pub const fn set_dma_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, Enable bit for DMA address loop function."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_loop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Enable bit for DMA address loop function."]
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
                .field("dma_loop", &self.dma_loop())
                .finish()
        }
    }
    #[doc = "RW, TMR mode control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlMod(pub u8);
    impl CtrlMod {
        #[doc = "RW, timer in mode: 0=timer/PWM, 1=capture/count."]
        #[must_use]
        #[inline(always)]
        pub const fn mode_in(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, timer in mode: 0=timer/PWM, 1=capture/count."]
        #[inline(always)]
        pub const fn set_mode_in(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, force clear timer FIFO and count."]
        #[must_use]
        #[inline(always)]
        pub const fn all_clear(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, force clear timer FIFO and count."]
        #[inline(always)]
        pub const fn set_all_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, timer count enable."]
        #[must_use]
        #[inline(always)]
        pub const fn count_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, timer count enable."]
        #[inline(always)]
        pub const fn set_count_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, timer output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn out_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, timer output enable."]
        #[inline(always)]
        pub const fn set_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, count sub-mode if TMR_MODE_IN=1: 0=capture, 1=count."]
        #[must_use]
        #[inline(always)]
        pub const fn cap_count(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, count sub-mode if TMR_MODE_IN=1: 0=capture, 1=count."]
        #[inline(always)]
        pub const fn set_cap_count(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RW, timer PWM output polarity: 0=default low and high action, 1=default high and low action."]
        #[must_use]
        #[inline(always)]
        pub const fn out_polar(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, timer PWM output polarity: 0=default low and high action, 1=default high and low action."]
        #[inline(always)]
        pub const fn set_out_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RW, timer capture edge mode: 00=disable, 01=edge change, 10=fall to fall, 11-rise to rise."]
        #[must_use]
        #[inline(always)]
        pub const fn cap_edge(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "RW, timer capture edge mode: 00=disable, 01=edge change, 10=fall to fall, 11-rise to rise."]
        #[inline(always)]
        pub const fn set_cap_edge(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
        #[doc = "RW, timer PWM repeat mode: 00=1, 01=4, 10=8, 11-16."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm_repeat(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "RW, timer PWM repeat mode: 00=1, 01=4, 10=8, 11-16."]
        #[inline(always)]
        pub const fn set_pwm_repeat(&mut self, val: u8) {
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
                .field("mode_in", &self.mode_in())
                .field("all_clear", &self.all_clear())
                .field("count_en", &self.count_en())
                .field("out_en", &self.out_en())
                .field("cap_count", &self.cap_count())
                .field("out_polar", &self.out_polar())
                .field("cap_edge", &self.cap_edge())
                .field("pwm_repeat", &self.pwm_repeat())
                .finish()
        }
    }
    #[doc = "RW, DMA start buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaBeg(pub u32);
    impl DmaBeg {
        #[doc = "RW, The starting address of the DMA data buffer must be aligned with 4 bytes."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_beg(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x1fff;
            val as u16
        }
        #[doc = "RW, The starting address of the DMA data buffer must be aligned with 4 bytes."]
        #[inline(always)]
        pub const fn set_dma_beg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 2usize)) | (((val as u32) & 0x1fff) << 2usize);
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
    #[doc = "RW, DMA end buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaEnd(pub u32);
    impl DmaEnd {
        #[doc = "RW, DMA data buffer end address (excluding), the address must be aligned with 4 bytes."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_end(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x1fff;
            val as u16
        }
        #[doc = "RW, DMA data buffer end address (excluding), the address must be aligned with 4 bytes."]
        #[inline(always)]
        pub const fn set_dma_end(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 2usize)) | (((val as u32) & 0x1fff) << 2usize);
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
    #[doc = "RO, DMA current buffer address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaNow(pub u32);
    impl DmaNow {
        #[doc = "RW, DMA data buffer current address."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_now(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x1fff;
            val as u16
        }
        #[doc = "RW, DMA data buffer current address."]
        #[inline(always)]
        pub const fn set_dma_now(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 2usize)) | (((val as u32) & 0x1fff) << 2usize);
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
    #[doc = "RO/WO, TMR FIFO register, only low 26 bit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifo(pub u32);
    impl Fifo {
        #[doc = "RW1,TMR FIFO register."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW1,TMR FIFO register."]
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
    #[doc = "RO, TMR FIFO count status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FifoCount(pub u8);
    impl FifoCount {
        #[doc = "R0,TMR FIFO count status."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_count(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "R0,TMR FIFO count status."]
        #[inline(always)]
        pub const fn set_fifo_count(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
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
    #[doc = "RW1, TMR interrupt flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFlag(pub u8);
    impl IntFlag {
        #[doc = "RW1, interrupt flag for timer capture count timeout or PWM cycle end."]
        #[must_use]
        #[inline(always)]
        pub const fn if_cyc_end(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW1, interrupt flag for timer capture count timeout or PWM cycle end."]
        #[inline(always)]
        pub const fn set_if_cyc_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW1, interrupt flag for timer capture input action or PWM trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn if_data_act(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW1, interrupt flag for timer capture input action or PWM trigger."]
        #[inline(always)]
        pub const fn set_if_data_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW1, interrupt flag for timer FIFO half (capture fifo >=4 or PWM fifo less than 3."]
        #[must_use]
        #[inline(always)]
        pub const fn if_fifo_hf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW1, interrupt flag for timer FIFO half (capture fifo >=4 or PWM fifo less than 3."]
        #[inline(always)]
        pub const fn set_if_fifo_hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW1, interrupt flag for timer1/2 DMA completion."]
        #[must_use]
        #[inline(always)]
        pub const fn if_dma_end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW1, interrupt flag for timer1/2 DMA completion."]
        #[inline(always)]
        pub const fn set_if_dma_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW1, interrupt flag for timer FIFO overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn if_fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW1, interrupt flag for timer FIFO overflow."]
        #[inline(always)]
        pub const fn set_if_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
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
                .finish()
        }
    }
    #[doc = "RW, TMR interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InterEn(pub u8);
    impl InterEn {
        #[doc = "RW, enable interrupt for timer capture count timeout or PWM cycle end."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_cyc_end(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable interrupt for timer capture count timeout or PWM cycle end."]
        #[inline(always)]
        pub const fn set_ie_cyc_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, enable interrupt for timer capture input action or PWM trigger."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_data_act(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable interrupt for timer capture input action or PWM trigger."]
        #[inline(always)]
        pub const fn set_ie_data_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, enable interrupt for timer FIFO half (capture fifo >=4 or PWM fifo less than 3)."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_fifo_hf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable interrupt for timer FIFO half (capture fifo >=4 or PWM fifo less than 3)."]
        #[inline(always)]
        pub const fn set_ie_fifo_hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, enable interrupt for timer1/2 DMA completion."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_dma_end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable interrupt for timer1/2 DMA completion."]
        #[inline(always)]
        pub const fn set_ie_dma_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, enable interrupt for timer FIFO overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable interrupt for timer FIFO overflow."]
        #[inline(always)]
        pub const fn set_ie_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for InterEn {
        #[inline(always)]
        fn default() -> InterEn {
            InterEn(0)
        }
    }
    impl core::fmt::Debug for InterEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("InterEn")
                .field("ie_cyc_end", &self.ie_cyc_end())
                .field("ie_data_act", &self.ie_data_act())
                .field("ie_fifo_hf", &self.ie_fifo_hf())
                .field("ie_dma_end", &self.ie_dma_end())
                .field("ie_fifo_ov", &self.ie_fifo_ov())
                .finish()
        }
    }
}
