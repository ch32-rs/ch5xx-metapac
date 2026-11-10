#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Analog to Digital Converter."]
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
    #[doc = "RW, ADC input channel selection."]
    #[inline(always)]
    pub const fn channel(self) -> crate::common::Reg<regs::Channel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "RW, ADC configure."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x59usize) as _) }
    }
    #[doc = "RW, ADC convert control."]
    #[inline(always)]
    pub const fn convert(self) -> crate::common::Reg<regs::Convert, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5ausize) as _) }
    }
    #[doc = "RW, temperature sensor control."]
    #[inline(always)]
    pub const fn tem_sensor(self) -> crate::common::Reg<regs::TemSensor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5busize) as _) }
    }
    #[doc = "RO, ADC data."]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::Data, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "RO, ADC interrupt flag register."]
    #[inline(always)]
    pub const fn int_flag(self) -> crate::common::Reg<regs::IntFlag, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5eusize) as _) }
    }
    #[doc = "RW, ADC DMA control."]
    #[inline(always)]
    pub const fn ctrl_dma(self) -> crate::common::Reg<regs::CtrlDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x61usize) as _) }
    }
    #[doc = "RW1, ADC and DMA interrupt flag registers."]
    #[inline(always)]
    pub const fn dma_if(self) -> crate::common::Reg<regs::DmaIf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x62usize) as _) }
    }
    #[doc = "RW, ADC auto cycle value."]
    #[inline(always)]
    pub const fn auto_cycle(self) -> crate::common::Reg<regs::AutoCycle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x63usize) as _) }
    }
    #[doc = "RO, ADC DMA current address."]
    #[inline(always)]
    pub const fn dma_now(self) -> crate::common::Reg<regs::DmaNow, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "RW, ADC DMA begin address."]
    #[inline(always)]
    pub const fn dma_beg(self) -> crate::common::Reg<regs::DmaBeg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "RW, ADC DMA end address."]
    #[inline(always)]
    pub const fn dma_end(self) -> crate::common::Reg<regs::DmaEnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
}
pub mod regs {
    #[doc = "RO, ADC interrupt flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AutoCycle(pub u8);
    impl AutoCycle {
        #[doc = "auto ADC cycle value, unit is 16 Fsys."]
        #[must_use]
        #[inline(always)]
        pub const fn auto_cycle(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "auto ADC cycle value, unit is 16 Fsys."]
        #[inline(always)]
        pub const fn set_auto_cycle(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for AutoCycle {
        #[inline(always)]
        fn default() -> AutoCycle {
            AutoCycle(0)
        }
    }
    impl core::fmt::Debug for AutoCycle {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AutoCycle")
                .field("auto_cycle", &self.auto_cycle())
                .finish()
        }
    }
    #[doc = "RW, ADC configure."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u8);
    impl Cfg {
        #[doc = "RW, ADC power control: 0=power down, 1=power on."]
        #[must_use]
        #[inline(always)]
        pub const fn power_on(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC power control: 0=power down, 1=power on."]
        #[inline(always)]
        pub const fn set_power_on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, ADC input buffer enable."]
        #[must_use]
        #[inline(always)]
        pub const fn buf_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC input buffer enable."]
        #[inline(always)]
        pub const fn set_buf_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, ADC input channel mode: 0=single-end, 1=differnetial."]
        #[must_use]
        #[inline(always)]
        pub const fn diff_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC input channel mode: 0=single-end, 1=differnetial."]
        #[inline(always)]
        pub const fn set_diff_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, enable ADC offset test mode: 0=normal mode, 1=short to test offset."]
        #[must_use]
        #[inline(always)]
        pub const fn ofs_test(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable ADC offset test mode: 0=normal mode, 1=short to test offset."]
        #[inline(always)]
        pub const fn set_ofs_test(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, set ADC input PGA gain."]
        #[must_use]
        #[inline(always)]
        pub const fn pga_gain10(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "RW, set ADC input PGA gain."]
        #[inline(always)]
        pub const fn set_pga_gain10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "RW, select ADC clock frequency."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_div(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, select ADC clock frequency."]
        #[inline(always)]
        pub const fn set_clk_div(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
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
                .field("power_on", &self.power_on())
                .field("buf_en", &self.buf_en())
                .field("diff_en", &self.diff_en())
                .field("ofs_test", &self.ofs_test())
                .field("pga_gain10", &self.pga_gain10())
                .field("clk_div", &self.clk_div())
                .finish()
        }
    }
    #[doc = "RW, ADC input channel selection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Channel(pub u8);
    impl Channel {
        #[doc = "RW, ADC input channel index."]
        #[must_use]
        #[inline(always)]
        pub const fn ch_inx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, ADC input channel index."]
        #[inline(always)]
        pub const fn set_ch_inx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for Channel {
        #[inline(always)]
        fn default() -> Channel {
            Channel(0)
        }
    }
    impl core::fmt::Debug for Channel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Channel")
                .field("ch_inx", &self.ch_inx())
                .finish()
        }
    }
    #[doc = "RW, ADC convert control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Convert(pub u8);
    impl Convert {
        #[doc = "RW, ADC convert start control: 0=stop ADC convert, 1=start an ADC convert, auto clear."]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC convert start control: 0=stop ADC convert, 1=start an ADC convert, auto clear."]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, ADC sampling period selection."]
        #[must_use]
        #[inline(always)]
        pub const fn sample_time(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "RW, ADC sampling period selection."]
        #[inline(always)]
        pub const fn set_sample_time(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "RO, end of ADC conversion flag."]
        #[must_use]
        #[inline(always)]
        pub const fn eoc_x(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, end of ADC conversion flag."]
        #[inline(always)]
        pub const fn set_eoc_x(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Convert {
        #[inline(always)]
        fn default() -> Convert {
            Convert(0)
        }
    }
    impl core::fmt::Debug for Convert {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Convert")
                .field("start", &self.start())
                .field("sample_time", &self.sample_time())
                .field("eoc_x", &self.eoc_x())
                .finish()
        }
    }
    #[doc = "RW, ADC DMA control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlDma(pub u8);
    impl CtrlDma {
        #[doc = "RW, ADC DMA enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC DMA enable."]
        #[inline(always)]
        pub const fn set_dma_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, ADC DMA address loop enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_loop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC DMA address loop enable."]
        #[inline(always)]
        pub const fn set_dma_loop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, enable interrupt for ADC DMA completion."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_dma_end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable interrupt for ADC DMA completion."]
        #[inline(always)]
        pub const fn set_ie_dma_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, enable interrupt for end of ADC conversion."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_eoc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable interrupt for end of ADC conversion."]
        #[inline(always)]
        pub const fn set_ie_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RW, enable contineous conversion ADC."]
        #[must_use]
        #[inline(always)]
        pub const fn cont_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable contineous conversion ADC."]
        #[inline(always)]
        pub const fn set_cont_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RW, enable auto continuing ADC for DMA."]
        #[must_use]
        #[inline(always)]
        pub const fn auto_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable auto continuing ADC for DMA."]
        #[inline(always)]
        pub const fn set_auto_en(&mut self, val: bool) {
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
                .field("dma_loop", &self.dma_loop())
                .field("ie_dma_end", &self.ie_dma_end())
                .field("ie_eoc", &self.ie_eoc())
                .field("cont_en", &self.cont_en())
                .field("auto_en", &self.auto_en())
                .finish()
        }
    }
    #[doc = "RO, ADC data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data(pub u16);
    impl Data {
        #[doc = "RO, ADC conversion data."]
        #[must_use]
        #[inline(always)]
        pub const fn data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "RO, ADC conversion data."]
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
    #[doc = "RW, ADC DMA begin address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaBeg(pub u16);
    impl DmaBeg {
        #[doc = "ADC DMA begin address."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_beg(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x3fff;
            val as u16
        }
        #[doc = "ADC DMA begin address."]
        #[inline(always)]
        pub const fn set_dma_beg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 1usize)) | (((val as u16) & 0x3fff) << 1usize);
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
    #[doc = "RW, ADC DMA end address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaEnd(pub u16);
    impl DmaEnd {
        #[doc = "ADC DMA end address."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_end(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x3fff;
            val as u16
        }
        #[doc = "ADC DMA end address."]
        #[inline(always)]
        pub const fn set_dma_end(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 1usize)) | (((val as u16) & 0x3fff) << 1usize);
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
    #[doc = "RW1, ADC and DMA interrupt flag registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaIf(pub u8);
    impl DmaIf {
        #[doc = "interrupt flag for ADC DMA completion."]
        #[must_use]
        #[inline(always)]
        pub const fn if_dma_end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for ADC DMA completion."]
        #[inline(always)]
        pub const fn set_if_dma_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "interrupt flag for end of ADC conversion."]
        #[must_use]
        #[inline(always)]
        pub const fn if_end(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for end of ADC conversion."]
        #[inline(always)]
        pub const fn set_if_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for DmaIf {
        #[inline(always)]
        fn default() -> DmaIf {
            DmaIf(0)
        }
    }
    impl core::fmt::Debug for DmaIf {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaIf")
                .field("if_dma_end", &self.if_dma_end())
                .field("if_end", &self.if_end())
                .finish()
        }
    }
    #[doc = "RO, ADC DMA current address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaNow(pub u16);
    impl DmaNow {
        #[doc = "ADC DMA current address."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_now(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x3fff;
            val as u16
        }
        #[doc = "ADC DMA current address."]
        #[inline(always)]
        pub const fn set_dma_now(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 1usize)) | (((val as u16) & 0x3fff) << 1usize);
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
    #[doc = "RO, ADC interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFlag(pub u8);
    impl IntFlag {
        #[doc = "RO, ADC conversion interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn if_eoc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, ADC conversion interrupt flag."]
        #[inline(always)]
        pub const fn set_if_eoc(&mut self, val: bool) {
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
                .field("if_eoc", &self.if_eoc())
                .finish()
        }
    }
    #[doc = "RW, temperature sensor control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TemSensor(pub u8);
    impl TemSensor {
        #[doc = "RW, temperature sensor power control: 0=power down, 1=power on."]
        #[must_use]
        #[inline(always)]
        pub const fn tem_sen_pwr_on(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, temperature sensor power control: 0=power down, 1=power on."]
        #[inline(always)]
        pub const fn set_tem_sen_pwr_on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for TemSensor {
        #[inline(always)]
        fn default() -> TemSensor {
            TemSensor(0)
        }
    }
    impl core::fmt::Debug for TemSensor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TemSensor")
                .field("tem_sen_pwr_on", &self.tem_sen_pwr_on())
                .finish()
        }
    }
}
