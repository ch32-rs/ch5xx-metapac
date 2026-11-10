#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PWMx register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwmx {
    ptr: *mut u8,
}
unsafe impl Send for Pwmx {}
unsafe impl Sync for Pwmx {}
impl Pwmx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RW, PWM output enable control."]
    #[inline(always)]
    pub const fn out_en(self) -> crate::common::Reg<regs::OutEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RW, PWM output polarity control."]
    #[inline(always)]
    pub const fn polar(self) -> crate::common::Reg<regs::Polar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "RW, PWM configuration."]
    #[inline(always)]
    pub const fn config(self) -> crate::common::Reg<regs::Config, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "RW, PWMx DMA control register."]
    #[inline(always)]
    pub const fn dma_ctrl(self) -> crate::common::Reg<regs::DmaCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "RW, PWM data retention register group 1."]
    #[inline(always)]
    pub const fn pwm1_3_data(self) -> crate::common::Reg<regs::Pwm13Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RW, PWM data retention register group 2."]
    #[inline(always)]
    pub const fn pwm3_data(self) -> crate::common::Reg<regs::Pwm3Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RW, PWMx interrupt enable register."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "RW1, PWMx interrupt flag register."]
    #[inline(always)]
    pub const fn int_flag(self) -> crate::common::Reg<regs::IntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dusize) as _) }
    }
    #[doc = "RW, PWM data retention register group 3."]
    #[inline(always)]
    pub const fn pwm4_5_data(self) -> crate::common::Reg<regs::Pwm45Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "RW, PWM1/2/3 cycles end register."]
    #[inline(always)]
    pub const fn cyc_value(self) -> crate::common::Reg<regs::CycValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "RW, PWM reference clock frequency division coefficient."]
    #[inline(always)]
    pub const fn clock_div(self) -> crate::common::Reg<regs::ClockDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "RW, Current address of PWMx DMA buffer."]
    #[inline(always)]
    pub const fn dma_now(self) -> crate::common::Reg<regs::DmaNow, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "RW, Starting address of PWMx DMA buffer."]
    #[inline(always)]
    pub const fn dma_beg(self) -> crate::common::Reg<regs::DmaBeg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "RW, End of PWMx DMA buffer."]
    #[inline(always)]
    pub const fn dma_end(self) -> crate::common::Reg<regs::DmaEnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
}
pub mod regs {
    #[doc = "RW, PWM reference clock frequency division coefficient."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClockDiv(pub u16);
    impl ClockDiv {
        #[doc = "RW, DMA data buffer current address."]
        #[must_use]
        #[inline(always)]
        pub const fn clock_div(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RW, DMA data buffer current address."]
        #[inline(always)]
        pub const fn set_clock_div(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
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
    #[doc = "RW, PWM configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config(pub u8);
    impl Config {
        #[doc = "RW, PWM cycle selection: 0=256;128;64;32 clocks, 1=255;127;63;31 clocks."]
        #[must_use]
        #[inline(always)]
        pub const fn cycle_sel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM cycle selection: 0=256;128;64;32 clocks, 1=255;127;63;31 clocks."]
        #[inline(always)]
        pub const fn set_cycle_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, PWM data width mode: 00=8 bits data, 01=7 bits data, 10=6 bits data, 11=5 bits data."]
        #[must_use]
        #[inline(always)]
        pub const fn cyc_mod(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "RW, PWM data width mode: 00=8 bits data, 01=7 bits data, 10=6 bits data, 11=5 bits data."]
        #[inline(always)]
        pub const fn set_cyc_mod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u8) & 0x03) << 1usize);
        }
        #[doc = "RW, PWM4/5 stagger output enable: 0=independent output, 1=stagger output."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm4_5_stag_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM4/5 stagger output enable: 0=independent output, 1=stagger output."]
        #[inline(always)]
        pub const fn set_pwm4_5_stag_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, PWM4/5 channel output status."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm4_5_ch(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM4/5 channel output status."]
        #[inline(always)]
        pub const fn set_pwm4_5_ch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RW, Sync Start."]
        #[must_use]
        #[inline(always)]
        pub const fn sync_start(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Sync Start."]
        #[inline(always)]
        pub const fn set_sync_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RW, Synchronization enablement allows."]
        #[must_use]
        #[inline(always)]
        pub const fn sync_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Synchronization enablement allows."]
        #[inline(always)]
        pub const fn set_sync_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Config {
        #[inline(always)]
        fn default() -> Config {
            Config(0)
        }
    }
    impl core::fmt::Debug for Config {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Config")
                .field("cycle_sel", &self.cycle_sel())
                .field("cyc_mod", &self.cyc_mod())
                .field("pwm4_5_stag_en", &self.pwm4_5_stag_en())
                .field("pwm4_5_ch", &self.pwm4_5_ch())
                .field("sync_start", &self.sync_start())
                .field("sync_en", &self.sync_en())
                .finish()
        }
    }
    #[doc = "RW, PWM1/2/3 cycles end register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CycValue(pub u16);
    impl CycValue {
        #[doc = "RW, PWM1/2/3 clock cycle at 16 data widths."]
        #[must_use]
        #[inline(always)]
        pub const fn cyc_value(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RW, PWM1/2/3 clock cycle at 16 data widths."]
        #[inline(always)]
        pub const fn set_cyc_value(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for CycValue {
        #[inline(always)]
        fn default() -> CycValue {
            CycValue(0)
        }
    }
    impl core::fmt::Debug for CycValue {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CycValue")
                .field("cyc_value", &self.cyc_value())
                .finish()
        }
    }
    #[doc = "RW, Starting address of PWMx DMA buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaBeg(pub u32);
    impl DmaBeg {
        #[doc = "RW, Starting address of PWMx DMA buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_beg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW, Starting address of PWMx DMA buffer."]
        #[inline(always)]
        pub const fn set_dma_beg(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
    #[doc = "RW, PWMx DMA control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaCtrl(pub u8);
    impl DmaCtrl {
        #[doc = "RW, DMA enabled (only valid when the data width is 16)."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, DMA enabled (only valid when the data width is 16)."]
        #[inline(always)]
        pub const fn set_dma_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, DMA Loop Output Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_addr_loop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, DMA Loop Output Enable."]
        #[inline(always)]
        pub const fn set_dma_addr_loop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, DMA output channel selection when RB_PWM_SYNC_EN=0."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_sel(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, DMA output channel selection when RB_PWM_SYNC_EN=0."]
        #[inline(always)]
        pub const fn set_dma_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
    }
    impl Default for DmaCtrl {
        #[inline(always)]
        fn default() -> DmaCtrl {
            DmaCtrl(0)
        }
    }
    impl core::fmt::Debug for DmaCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaCtrl")
                .field("dma_enable", &self.dma_enable())
                .field("dma_addr_loop", &self.dma_addr_loop())
                .field("dma_sel", &self.dma_sel())
                .finish()
        }
    }
    #[doc = "RW, End of PWMx DMA buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaEnd(pub u32);
    impl DmaEnd {
        #[doc = "RW, End of PWMx DMA buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_end(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW, End of PWMx DMA buffer."]
        #[inline(always)]
        pub const fn set_dma_end(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
    #[doc = "RW, Current address of PWMx DMA buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaNow(pub u32);
    impl DmaNow {
        #[doc = "RW, Current address of PWMx DMA buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_now(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW, Current address of PWMx DMA buffer."]
        #[inline(always)]
        pub const fn set_dma_now(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
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
    #[doc = "RW, PWMx interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u8);
    impl IntEn {
        #[doc = "RW, End of PWM1/2/3 cycle interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_cyc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, End of PWM1/2/3 cycle interrupt enable bit."]
        #[inline(always)]
        pub const fn set_ie_cyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, Select the interrupt time point at the end of the PWM cycle."]
        #[must_use]
        #[inline(always)]
        pub const fn cyc_pre(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Select the interrupt time point at the end of the PWM cycle."]
        #[inline(always)]
        pub const fn set_cyc_pre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, End of PWM4/5 cycle interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm1_ie_cyc(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, End of PWM4/5 cycle interrupt enable bit."]
        #[inline(always)]
        pub const fn set_pwm1_ie_cyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, FIFO data less than 4 interrupt enable bits."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_fifo(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, FIFO data less than 4 interrupt enable bits."]
        #[inline(always)]
        pub const fn set_ie_fifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RW, DMA transfer end interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_dma(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RW, DMA transfer end interrupt enable bit."]
        #[inline(always)]
        pub const fn set_ie_dma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RW, FIFO read air interrupt enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_over(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, FIFO read air interrupt enable bit."]
        #[inline(always)]
        pub const fn set_ie_over(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
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
                .field("ie_cyc", &self.ie_cyc())
                .field("cyc_pre", &self.cyc_pre())
                .field("pwm1_ie_cyc", &self.pwm1_ie_cyc())
                .field("ie_fifo", &self.ie_fifo())
                .field("ie_dma", &self.ie_dma())
                .field("ie_over", &self.ie_over())
                .finish()
        }
    }
    #[doc = "RW1, PWMx interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFlag(pub u8);
    impl IntFlag {
        #[doc = "RW1, PWM1/2/3 cycle end flag."]
        #[must_use]
        #[inline(always)]
        pub const fn if_cyc(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW1, PWM1/2/3 cycle end flag."]
        #[inline(always)]
        pub const fn set_if_cyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW1, PWM4/5 cycle end flag."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm1_if_cyc(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW1, PWM4/5 cycle end flag."]
        #[inline(always)]
        pub const fn set_pwm1_if_cyc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW1, FIFO data less than 4 interrupt flag bits."]
        #[must_use]
        #[inline(always)]
        pub const fn if_fifo(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW1, FIFO data less than 4 interrupt flag bits."]
        #[inline(always)]
        pub const fn set_if_fifo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW1, DMA transfer end interrupt flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn if_dma(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW1, DMA transfer end interrupt flag bit."]
        #[inline(always)]
        pub const fn set_if_dma(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW1, FIFO overflow interrupt flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn if_over(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW1, FIFO overflow interrupt flag bit."]
        #[inline(always)]
        pub const fn set_if_over(&mut self, val: bool) {
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
                .field("if_cyc", &self.if_cyc())
                .field("pwm1_if_cyc", &self.pwm1_if_cyc())
                .field("if_fifo", &self.if_fifo())
                .field("if_dma", &self.if_dma())
                .field("if_over", &self.if_over())
                .finish()
        }
    }
    #[doc = "RW, PWM output enable control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OutEn(pub u8);
    impl OutEn {
        #[doc = "RW, PWM1 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm1_out_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM1 output enable."]
        #[inline(always)]
        pub const fn set_pwm1_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, PWM2 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm2_out_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM2 output enable."]
        #[inline(always)]
        pub const fn set_pwm2_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, PWM3 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm3_out_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM3 output enable."]
        #[inline(always)]
        pub const fn set_pwm3_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, PWM4 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm4_out_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM4 output enable."]
        #[inline(always)]
        pub const fn set_pwm4_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, PWM5 output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm5_out_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM5 output enable."]
        #[inline(always)]
        pub const fn set_pwm5_out_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for OutEn {
        #[inline(always)]
        fn default() -> OutEn {
            OutEn(0)
        }
    }
    impl core::fmt::Debug for OutEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("OutEn")
                .field("pwm1_out_en", &self.pwm1_out_en())
                .field("pwm2_out_en", &self.pwm2_out_en())
                .field("pwm3_out_en", &self.pwm3_out_en())
                .field("pwm4_out_en", &self.pwm4_out_en())
                .field("pwm5_out_en", &self.pwm5_out_en())
                .finish()
        }
    }
    #[doc = "RW, PWM output polarity control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Polar(pub u8);
    impl Polar {
        #[doc = "RW, PWM1 output polarity: 0=default low and high action, 1=default high and low action."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm1_polar(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM1 output polarity: 0=default low and high action, 1=default high and low action."]
        #[inline(always)]
        pub const fn set_pwm1_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, PWM2 output polarity: 0=default low and high action, 1=default high and low action."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm2_polar(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM2 output polarity: 0=default low and high action, 1=default high and low action."]
        #[inline(always)]
        pub const fn set_pwm2_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, PWM3 output polarity: 0=default low and high action, 1=default high and low action."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm3_polar(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM3 output polarity: 0=default low and high action, 1=default high and low action."]
        #[inline(always)]
        pub const fn set_pwm3_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, PWM4 output polarity: 0=default low and high action, 1=default high and low action."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm4_polar(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM4 output polarity: 0=default low and high action, 1=default high and low action."]
        #[inline(always)]
        pub const fn set_pwm4_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, PWM5 output polarity: 0=default low and high action, 1=default high and low action."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm5_polar(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM5 output polarity: 0=default low and high action, 1=default high and low action."]
        #[inline(always)]
        pub const fn set_pwm5_polar(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for Polar {
        #[inline(always)]
        fn default() -> Polar {
            Polar(0)
        }
    }
    impl core::fmt::Debug for Polar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Polar")
                .field("pwm1_polar", &self.pwm1_polar())
                .field("pwm2_polar", &self.pwm2_polar())
                .field("pwm3_polar", &self.pwm3_polar())
                .field("pwm4_polar", &self.pwm4_polar())
                .field("pwm5_polar", &self.pwm5_polar())
                .finish()
        }
    }
    #[doc = "RW, PWM data retention register group 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm13Data(pub u32);
    impl Pwm13Data {
        #[doc = "RW, PWM1 data holding or PWM1 data holding(16bits wide)."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm1_data__pwm1_data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW, PWM1 data holding or PWM1 data holding(16bits wide)."]
        #[inline(always)]
        pub const fn set_pwm1_data__pwm1_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "RW, PWM2 data holding or PWM1 data holding(16bits wide)."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm2_data__pwm1_data(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "RW, PWM2 data holding or PWM1 data holding(16bits wide)."]
        #[inline(always)]
        pub const fn set_pwm2_data__pwm1_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "RW, PWM3 data holding or PWM2 data holding(16bits wide)."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm3_data__pwm2_data(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "RW, PWM3 data holding or PWM2 data holding(16bits wide)."]
        #[inline(always)]
        pub const fn set_pwm3_data__pwm2_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "RW, PWM2 data holding(16bits wide)."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm2_data(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "RW, PWM2 data holding(16bits wide)."]
        #[inline(always)]
        pub const fn set_pwm2_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Pwm13Data {
        #[inline(always)]
        fn default() -> Pwm13Data {
            Pwm13Data(0)
        }
    }
    impl core::fmt::Debug for Pwm13Data {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwm13Data")
                .field("pwm1_data__pwm1_data", &self.pwm1_data__pwm1_data())
                .field("pwm2_data__pwm1_data", &self.pwm2_data__pwm1_data())
                .field("pwm3_data__pwm2_data", &self.pwm3_data__pwm2_data())
                .field("pwm2_data", &self.pwm2_data())
                .finish()
        }
    }
    #[doc = "RW, PWM data retention register group 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm3Data(pub u16);
    impl Pwm3Data {
        #[doc = "RW, PWM3 data retention register."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm3_data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RW, PWM3 data retention register."]
        #[inline(always)]
        pub const fn set_pwm3_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for Pwm3Data {
        #[inline(always)]
        fn default() -> Pwm3Data {
            Pwm3Data(0)
        }
    }
    impl core::fmt::Debug for Pwm3Data {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwm3Data")
                .field("pwm3_data", &self.pwm3_data())
                .finish()
        }
    }
    #[doc = "RW, PWM data retention register group 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwm45Data(pub u32);
    impl Pwm45Data {
        #[doc = "RW, PWM4 data holding or PWM4 data holding(16bits wide)."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm4_data__pwm4_data(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW, PWM4 data holding or PWM4 data holding(16bits wide)."]
        #[inline(always)]
        pub const fn set_pwm4_data__pwm4_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "RW, PWM5 data holding or PWM4 data holding(16bits wide)."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm5_data__pwm4_data(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "RW, PWM5 data holding or PWM4 data holding(16bits wide)."]
        #[inline(always)]
        pub const fn set_pwm5_data__pwm4_data(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "RW, PWM5 data holding(16bits wide)."]
        #[must_use]
        #[inline(always)]
        pub const fn pwm5_data(&self) -> u32 {
            let val = (self.0 >> 16usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "RW, PWM5 data holding(16bits wide)."]
        #[inline(always)]
        pub const fn set_pwm5_data(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x0001_ffff << 16usize)) | (((val as u32) & 0x0001_ffff) << 16usize);
        }
    }
    impl Default for Pwm45Data {
        #[inline(always)]
        fn default() -> Pwm45Data {
            Pwm45Data(0)
        }
    }
    impl core::fmt::Debug for Pwm45Data {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Pwm45Data")
                .field("pwm4_data__pwm4_data", &self.pwm4_data__pwm4_data())
                .field("pwm5_data__pwm4_data", &self.pwm5_data__pwm4_data())
                .field("pwm5_data", &self.pwm5_data())
                .finish()
        }
    }
}
