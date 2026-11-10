#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "ADC register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc {
    ptr: *mut u8,
}
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ADC mode control register."]
    #[inline(always)]
    pub const fn ctrl_mod(self) -> crate::common::Reg<regs::CtrlMod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "DMA control register."]
    #[inline(always)]
    pub const fn ctrl_dma(self) -> crate::common::Reg<regs::CtrlDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "ADC interrupt the enabling register."]
    #[inline(always)]
    pub const fn inter_en(self) -> crate::common::Reg<regs::InterEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "ADC clock frequency partition coefficient register."]
    #[inline(always)]
    pub const fn clock_div(self) -> crate::common::Reg<regs::ClockDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "Convert the data register."]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::Data, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Interrupt the status register."]
    #[inline(always)]
    pub const fn int_flag(self) -> crate::common::Reg<regs::IntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "FIFO count register."]
    #[inline(always)]
    pub const fn fifo_count(self) -> crate::common::Reg<regs::FifoCount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "ADC control register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "The comparison mode reference value register."]
    #[inline(always)]
    pub const fn cmp_value(self) -> crate::common::Reg<regs::CmpValue, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "ADC FIFO register."]
    #[inline(always)]
    pub const fn fifo(self) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "DMA current buffer address register."]
    #[inline(always)]
    pub const fn dma_now(self) -> crate::common::Reg<regs::DmaNow, crate::common::RW> {
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
pub mod regs {
    #[doc = "ADC clock frequency partition coefficient register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClockDiv(pub u8);
    impl ClockDiv {
        #[doc = "sampling the clock frequency division coefficient."]
        #[must_use]
        #[inline(always)]
        pub const fn clock_div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "sampling the clock frequency division coefficient."]
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
    #[doc = "The comparison mode reference value register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmpValue(pub u16);
    impl CmpValue {
        #[doc = "The comparison mode reference value."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp_value(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "The comparison mode reference value."]
        #[inline(always)]
        pub const fn set_cmp_value(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u16) & 0x0fff) << 0usize);
        }
    }
    impl Default for CmpValue {
        #[inline(always)]
        fn default() -> CmpValue {
            CmpValue(0)
        }
    }
    impl core::fmt::Debug for CmpValue {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmpValue")
                .field("cmp_value", &self.cmp_value())
                .finish()
        }
    }
    #[doc = "ADC control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "Corresponding channel."]
        #[must_use]
        #[inline(always)]
        pub const fn ctl_mod1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Corresponding channel."]
        #[inline(always)]
        pub const fn set_ctl_mod1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "ADC sampling and calibration time."]
        #[must_use]
        #[inline(always)]
        pub const fn smaple_time(&self) -> u32 {
            let val = (self.0 >> 4usize) & 0x001f_ffff;
            val as u32
        }
        #[doc = "ADC sampling and calibration time."]
        #[inline(always)]
        pub const fn set_smaple_time(&mut self, val: u32) {
            self.0 = (self.0 & !(0x001f_ffff << 4usize)) | (((val as u32) & 0x001f_ffff) << 4usize);
        }
        #[doc = "Position 4-6 of the automatic sampling period."]
        #[must_use]
        #[inline(always)]
        pub const fn cycle_bit_4_6(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x07;
            val as u8
        }
        #[doc = "Position 4-6 of the automatic sampling period."]
        #[inline(always)]
        pub const fn set_cycle_bit_4_6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 25usize)) | (((val as u32) & 0x07) << 25usize);
        }
        #[doc = "ADC resolution selection bit."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_mode(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "ADC resolution selection bit."]
        #[inline(always)]
        pub const fn set_bit_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
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
                .field("ctl_mod1", &self.ctl_mod1())
                .field("smaple_time", &self.smaple_time())
                .field("cycle_bit_4_6", &self.cycle_bit_4_6())
                .field("bit_mode", &self.bit_mode())
                .finish()
        }
    }
    #[doc = "DMA control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlDma(pub u8);
    impl CtrlDma {
        #[doc = "DMA function enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "DMA function enable bit."]
        #[inline(always)]
        pub const fn set_dma_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "DMA block operation enable bit."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_burst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA block operation enable bit."]
        #[inline(always)]
        pub const fn set_dma_burst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "DMA address loop function enables."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_loop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "DMA address loop function enables."]
        #[inline(always)]
        pub const fn set_dma_loop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "ADC channel controls the ADCS output enable bits."]
        #[must_use]
        #[inline(always)]
        pub const fn chan_oe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ADC channel controls the ADCS output enable bits."]
        #[inline(always)]
        pub const fn set_chan_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "ADC manually sampled the control bits."]
        #[must_use]
        #[inline(always)]
        pub const fn man_sample(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "ADC manually sampled the control bits."]
        #[inline(always)]
        pub const fn set_man_sample(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("chan_oe", &self.chan_oe())
                .field("man_sample", &self.man_sample())
                .finish()
        }
    }
    #[doc = "ADC mode control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlMod(pub u8);
    impl CtrlMod {
        #[doc = "ADC clock sampling mode setting bit."]
        #[must_use]
        #[inline(always)]
        pub const fn cycle_clk(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "ADC clock sampling mode setting bit."]
        #[inline(always)]
        pub const fn set_cycle_clk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
        #[doc = "ADC channel selection bit."]
        #[must_use]
        #[inline(always)]
        pub const fn chan_mod(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "ADC channel selection bit."]
        #[inline(always)]
        pub const fn set_chan_mod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "ADC automatically samples the pulse width settings bit."]
        #[must_use]
        #[inline(always)]
        pub const fn sample_wid(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ADC automatically samples the pulse width settings bit."]
        #[inline(always)]
        pub const fn set_sample_wid(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "ADC function enable."]
        #[must_use]
        #[inline(always)]
        pub const fn power_on(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "ADC function enable."]
        #[inline(always)]
        pub const fn set_power_on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("cycle_clk", &self.cycle_clk())
                .field("chan_mod", &self.chan_mod())
                .field("sample_wid", &self.sample_wid())
                .field("power_on", &self.power_on())
                .finish()
        }
    }
    #[doc = "Convert the data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data(pub u16);
    impl Data {
        #[doc = "ADC collected the data content."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ADC collected the data content."]
        #[inline(always)]
        pub const fn set_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u16) & 0x0fff) << 0usize);
        }
    }
    impl Default for Data {
        #[inline(always)]
        fn default() -> Data {
            Data(0)
        }
    }
    impl core::fmt::Debug for Data {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Data").field("data", &self.data()).finish()
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
    #[doc = "ADC FIFO register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifo(pub u16);
    impl Fifo {
        #[doc = "ADC Current conversion value."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "ADC Current conversion value."]
        #[inline(always)]
        pub const fn set_fifo(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u16) & 0x0fff) << 0usize);
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
    #[doc = "FIFO count register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FifoCount(pub u8);
    impl FifoCount {
        #[doc = "number of ADC converted data already stored in the FIFO."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_count(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "number of ADC converted data already stored in the FIFO."]
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
    #[doc = "Interrupt the status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFlag(pub u8);
    impl IntFlag {
        #[doc = "ADC activated interrupt flag bits comparing the ADC current. value with the reference."]
        #[must_use]
        #[inline(always)]
        pub const fn if_cmp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "ADC activated interrupt flag bits comparing the ADC current. value with the reference."]
        #[inline(always)]
        pub const fn set_if_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "The ADC transition-end flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn if_end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "The ADC transition-end flag bit."]
        #[inline(always)]
        pub const fn set_if_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "FIFO receives half interrupt flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn if_fifo_hf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO receives half interrupt flag bit."]
        #[inline(always)]
        pub const fn set_if_fifo_hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "DMA end flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn if_dma_end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DMA end flag bit."]
        #[inline(always)]
        pub const fn set_if_dma_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "FIFO overflow interrupt flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn if_fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overflow interrupt flag bit."]
        #[inline(always)]
        pub const fn set_if_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "DMA transfer error interrupt flag bit."]
        #[must_use]
        #[inline(always)]
        pub const fn if_dma_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DMA transfer error interrupt flag bit."]
        #[inline(always)]
        pub const fn set_if_dma_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "The ADC transition-end state bit."]
        #[must_use]
        #[inline(always)]
        pub const fn eoc_flag(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "The ADC transition-end state bit."]
        #[inline(always)]
        pub const fn set_eoc_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "The ADC current channel in autoconversion mode."]
        #[must_use]
        #[inline(always)]
        pub const fn chan_index(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "The ADC current channel in autoconversion mode."]
        #[inline(always)]
        pub const fn set_chan_index(&mut self, val: bool) {
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
                .field("if_cmp", &self.if_cmp())
                .field("if_end", &self.if_end())
                .field("if_fifo_hf", &self.if_fifo_hf())
                .field("if_dma_end", &self.if_dma_end())
                .field("if_fifo_ov", &self.if_fifo_ov())
                .field("if_dma_err", &self.if_dma_err())
                .field("eoc_flag", &self.eoc_flag())
                .field("chan_index", &self.chan_index())
                .finish()
        }
    }
    #[doc = "ADC interrupt the enabling register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InterEn(pub u8);
    impl InterEn {
        #[doc = "current ADC comparison activates the interrupt enable bits."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_cmp(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "current ADC comparison activates the interrupt enable bits."]
        #[inline(always)]
        pub const fn set_ie_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "ADC conversion complete interrupt enable bits."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "ADC conversion complete interrupt enable bits."]
        #[inline(always)]
        pub const fn set_ie_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "FIFO receives half interrupt enable bits."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_fifo_hf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO receives half interrupt enable bits."]
        #[inline(always)]
        pub const fn set_ie_fifo_hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "DMA ends the interrupt enable bits."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_dma_end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "DMA ends the interrupt enable bits."]
        #[inline(always)]
        pub const fn set_ie_dma_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "FIFO overflow interruption enable bits."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overflow interruption enable bits."]
        #[inline(always)]
        pub const fn set_ie_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "DMA transfer process error response interrupt enable bits."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_dma_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "DMA transfer process error response interrupt enable bits."]
        #[inline(always)]
        pub const fn set_ie_dma_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "ADC reference value equal comparable enable bits."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp_mod_eq(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "ADC reference value equal comparable enable bits."]
        #[inline(always)]
        pub const fn set_cmp_mod_eq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "ADC reference size to compare enable bits."]
        #[must_use]
        #[inline(always)]
        pub const fn cmp_mod_gt(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "ADC reference size to compare enable bits."]
        #[inline(always)]
        pub const fn set_cmp_mod_gt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("ie_cmp", &self.ie_cmp())
                .field("ie_end", &self.ie_end())
                .field("ie_fifo_hf", &self.ie_fifo_hf())
                .field("ie_dma_end", &self.ie_dma_end())
                .field("ie_fifo_ov", &self.ie_fifo_ov())
                .field("ie_dma_err", &self.ie_dma_err())
                .field("cmp_mod_eq", &self.cmp_mod_eq())
                .field("cmp_mod_gt", &self.cmp_mod_gt())
                .finish()
        }
    }
}
