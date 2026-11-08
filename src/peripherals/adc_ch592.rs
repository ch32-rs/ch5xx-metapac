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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "RW, ADC configure."]
    #[inline(always)]
    pub const fn cfg(self) -> crate::common::Reg<regs::Cfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x59usize) as _) }
    }
    #[doc = "RW, ADC convert control."]
    #[inline(always)]
    pub const fn convert(self) -> crate::common::Reg<regs::Convert, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5ausize) as _) }
    }
    #[doc = "RW, temperature sensor control."]
    #[inline(always)]
    pub const fn tem_sensor(self) -> crate::common::Reg<regs::TemSensor, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5busize) as _) }
    }
    #[doc = "RO, ADC data."]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<regs::Data, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "RO, ADC interrupt flag register."]
    #[inline(always)]
    pub const fn int_flag(self) -> crate::common::Reg<regs::IntFlag, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5eusize) as _) }
    }
    #[doc = "RW, ADC DMA control."]
    #[inline(always)]
    pub const fn ctrl_dma(self) -> crate::common::Reg<regs::CtrlDma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x61usize) as _) }
    }
    #[doc = "RW1, ADC and DMA interrupt flag registers."]
    #[inline(always)]
    pub const fn dma_if(self) -> crate::common::Reg<regs::DmaIf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x62usize) as _) }
    }
    #[doc = "RW, ADC auto cycle value."]
    #[inline(always)]
    pub const fn auto_cycle(self) -> crate::common::Reg<regs::AutoCycle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x63usize) as _) }
    }
    #[doc = "RO, ADC DMA current address."]
    #[inline(always)]
    pub const fn dma_now(self) -> crate::common::Reg<regs::DmaNow, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "RW, ADC DMA begin address."]
    #[inline(always)]
    pub const fn dma_beg(self) -> crate::common::Reg<regs::DmaBeg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "RW, ADC DMA end address."]
    #[inline(always)]
    pub const fn dma_end(self) -> crate::common::Reg<regs::DmaEnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "RW, ADC scan sequence, channels 2 through 9."]
    #[inline(always)]
    pub const fn scan_cfg1(self) -> crate::common::Reg<regs::ScanCfg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "RW, ADC scan sequence, channels 10 through 12, and scan control."]
    #[inline(always)]
    pub const fn scan_cfg2(self) -> crate::common::Reg<regs::ScanCfg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
}
pub mod regs {
    #[doc = "RW, auto ADC cycle value, unit is 16 Fsys."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AutoCycle(pub u8);
    impl AutoCycle {
        #[doc = "auto ADC cycle value, unit is 16 Fsys."]
        #[inline(always)]
        pub const fn auto_cycle(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "auto ADC cycle value, unit is 16 Fsys."]
        #[inline(always)]
        pub fn set_auto_cycle(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for AutoCycle {
        #[inline(always)]
        fn default() -> AutoCycle {
            AutoCycle(0)
        }
    }
    #[doc = "RW, ADC configure."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfg(pub u8);
    impl Cfg {
        #[doc = "RW, ADC power control: 0=power down, 1=power on."]
        #[inline(always)]
        pub const fn power_on(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC power control: 0=power down, 1=power on."]
        #[inline(always)]
        pub fn set_power_on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, ADC input buffer enable."]
        #[inline(always)]
        pub const fn buf_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC input buffer enable."]
        #[inline(always)]
        pub fn set_buf_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, ADC input channel mode: 0=single-end, 1=differnetial."]
        #[inline(always)]
        pub const fn diff_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC input channel mode: 0=single-end, 1=differnetial."]
        #[inline(always)]
        pub fn set_diff_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, enable ADC offset test mode: 0=normal mode, 1=short to test offset."]
        #[inline(always)]
        pub const fn ofs_test(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable ADC offset test mode: 0=normal mode, 1=short to test offset."]
        #[inline(always)]
        pub fn set_ofs_test(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, set ADC input PGA gain: 00=-12dB, 01=-6dB, 10=0dB, 11=6dB."]
        #[inline(always)]
        pub const fn pga_gain10(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "RW, set ADC input PGA gain: 00=-12dB, 01=-6dB, 10=0dB, 11=6dB."]
        #[inline(always)]
        pub fn set_pga_gain10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
        #[doc = "RW, select ADC clock frequency: 00=3.2MHz, 01=2.67MHz, 10=5.33MHz, 11=4MHz."]
        #[inline(always)]
        pub const fn clk_div(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "RW, select ADC clock frequency: 00=3.2MHz, 01=2.67MHz, 10=5.33MHz, 11=4MHz."]
        #[inline(always)]
        pub fn set_clk_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for Cfg {
        #[inline(always)]
        fn default() -> Cfg {
            Cfg(0)
        }
    }
    #[doc = "RW, ADC input channel selection."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Channel(pub u8);
    impl Channel {
        #[doc = "RW, ADC input channel index."]
        #[inline(always)]
        pub const fn ch_inx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, ADC input channel index."]
        #[inline(always)]
        pub fn set_ch_inx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for Channel {
        #[inline(always)]
        fn default() -> Channel {
            Channel(0)
        }
    }
    #[doc = "RW, ADC convert control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Convert(pub u8);
    impl Convert {
        #[doc = "RW, ADC convert start control: 0=stop ADC convert, 1=start an ADC convert, auto clear."]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC convert start control: 0=stop ADC convert, 1=start an ADC convert, auto clear."]
        #[inline(always)]
        pub fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, ADC input PGA gain direction selection, combined with RB_ ADC_ PGA_ GAIN \\[1:0\\]
usage."]
        #[inline(always)]
        pub const fn pga_gain2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC input PGA gain direction selection, combined with RB_ ADC_ PGA_ GAIN \\[1:0\\]
usage."]
        #[inline(always)]
        pub fn set_pga_gain2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RO, end of ADC conversion flag."]
        #[inline(always)]
        pub const fn eoc_x(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, end of ADC conversion flag."]
        #[inline(always)]
        pub fn set_eoc_x(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Convert {
        #[inline(always)]
        fn default() -> Convert {
            Convert(0)
        }
    }
    #[doc = "RW, ADC DMA control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlDma(pub u8);
    impl CtrlDma {
        #[doc = "RW, ADC DMA enable."]
        #[inline(always)]
        pub const fn dma_enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC DMA enable."]
        #[inline(always)]
        pub fn set_dma_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, ADC DMA address loop enable."]
        #[inline(always)]
        pub const fn dma_loop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC DMA address loop enable."]
        #[inline(always)]
        pub fn set_dma_loop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, enable interrupt for ADC DMA completion."]
        #[inline(always)]
        pub const fn ie_dma_end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable interrupt for ADC DMA completion."]
        #[inline(always)]
        pub fn set_ie_dma_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, enable interrupt for end of ADC conversion."]
        #[inline(always)]
        pub const fn ie_eoc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable interrupt for end of ADC conversion."]
        #[inline(always)]
        pub fn set_ie_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RW, enable contineous conversion ADC."]
        #[inline(always)]
        pub const fn cont_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable contineous conversion ADC."]
        #[inline(always)]
        pub fn set_cont_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RW, enable auto continuing ADC for DMA."]
        #[inline(always)]
        pub const fn auto_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable auto continuing ADC for DMA."]
        #[inline(always)]
        pub fn set_auto_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for CtrlDma {
        #[inline(always)]
        fn default() -> CtrlDma {
            CtrlDma(0)
        }
    }
    #[doc = "RO, ADC data."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Data(pub u16);
    impl Data {
        #[doc = "RO, ADC conversion data."]
        #[inline(always)]
        pub const fn data(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "RO, ADC conversion data."]
        #[inline(always)]
        pub fn set_data(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u16) & 0x0fff) << 0usize);
        }
    }
    impl Default for Data {
        #[inline(always)]
        fn default() -> Data {
            Data(0)
        }
    }
    #[doc = "RW, ADC DMA begin address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaBeg(pub u16);
    impl DmaBeg {
        #[doc = "ADC DMA begin address."]
        #[inline(always)]
        pub const fn dma_beg(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x3fff;
            val as u16
        }
        #[doc = "ADC DMA begin address."]
        #[inline(always)]
        pub fn set_dma_beg(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 1usize)) | (((val as u16) & 0x3fff) << 1usize);
        }
    }
    impl Default for DmaBeg {
        #[inline(always)]
        fn default() -> DmaBeg {
            DmaBeg(0)
        }
    }
    #[doc = "RW, ADC DMA end address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaEnd(pub u16);
    impl DmaEnd {
        #[doc = "ADC DMA end address."]
        #[inline(always)]
        pub const fn dma_end(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x3fff;
            val as u16
        }
        #[doc = "ADC DMA end address."]
        #[inline(always)]
        pub fn set_dma_end(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 1usize)) | (((val as u16) & 0x3fff) << 1usize);
        }
    }
    impl Default for DmaEnd {
        #[inline(always)]
        fn default() -> DmaEnd {
            DmaEnd(0)
        }
    }
    #[doc = "RW1, ADC and DMA interrupt flag registers."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaIf(pub u8);
    impl DmaIf {
        #[doc = "interrupt flag for ADC DMA completion."]
        #[inline(always)]
        pub const fn if_dma_end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for ADC DMA completion."]
        #[inline(always)]
        pub fn set_if_dma_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "interrupt flag for end of ADC conversion."]
        #[inline(always)]
        pub const fn if_end(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for end of ADC conversion."]
        #[inline(always)]
        pub fn set_if_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for DmaIf {
        #[inline(always)]
        fn default() -> DmaIf {
            DmaIf(0)
        }
    }
    #[doc = "RO, ADC DMA current address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaNow(pub u16);
    impl DmaNow {
        #[doc = "ADC DMA current address."]
        #[inline(always)]
        pub const fn dma_now(&self) -> u16 {
            let val = (self.0 >> 1usize) & 0x3fff;
            val as u16
        }
        #[doc = "ADC DMA current address."]
        #[inline(always)]
        pub fn set_dma_now(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 1usize)) | (((val as u16) & 0x3fff) << 1usize);
        }
    }
    impl Default for DmaNow {
        #[inline(always)]
        fn default() -> DmaNow {
            DmaNow(0)
        }
    }
    #[doc = "RO, ADC interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFlag(pub u8);
    impl IntFlag {
        #[doc = "RO, ADC conversion interrupt flag."]
        #[inline(always)]
        pub const fn if_eoc(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, ADC conversion interrupt flag."]
        #[inline(always)]
        pub fn set_if_eoc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for IntFlag {
        #[inline(always)]
        fn default() -> IntFlag {
            IntFlag(0)
        }
    }
    #[doc = "RW, ADC scan sequence, channels 2 through 9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ScanCfg1(pub u32);
    impl ScanCfg1 {
        #[doc = "Scan sequence entry; entry 0 holds the second channel in the sequence."]
        #[inline(always)]
        pub const fn ch(&self, n: usize) -> u8 {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "Scan sequence entry; entry 0 holds the second channel in the sequence."]
        #[inline(always)]
        pub fn set_ch(&mut self, n: usize, val: u8) {
            assert!(n < 8usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
    }
    impl Default for ScanCfg1 {
        #[inline(always)]
        fn default() -> ScanCfg1 {
            ScanCfg1(0)
        }
    }
    #[doc = "RW, ADC scan sequence, channels 10 through 12, and scan control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ScanCfg2(pub u32);
    impl ScanCfg2 {
        #[doc = "Scan sequence entry; entry 0 holds the tenth channel in the sequence."]
        #[inline(always)]
        pub const fn ch(&self, n: usize) -> u8 {
            assert!(n < 3usize);
            let offs = 0usize + n * 4usize;
            let val = (self.0 >> offs) & 0x0f;
            val as u8
        }
        #[doc = "Scan sequence entry; entry 0 holds the tenth channel in the sequence."]
        #[inline(always)]
        pub fn set_ch(&mut self, n: usize, val: u8) {
            assert!(n < 3usize);
            let offs = 0usize + n * 4usize;
            self.0 = (self.0 & !(0x0f << offs)) | (((val as u32) & 0x0f) << offs);
        }
        #[doc = "Number of scanned channels minus one."]
        #[inline(always)]
        pub const fn scan_num(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of scanned channels minus one."]
        #[inline(always)]
        pub fn set_scan_num(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "Scan source: 0 selects the touch-key block, 1 selects the ADC."]
        #[inline(always)]
        pub const fn scan_sel(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Scan source: 0 selects the touch-key block, 1 selects the ADC."]
        #[inline(always)]
        pub fn set_scan_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Enable an interrupt when one scan pass ends."]
        #[inline(always)]
        pub const fn ie_scan_end(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Enable an interrupt when one scan pass ends."]
        #[inline(always)]
        pub fn set_ie_scan_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "Scan pass end flag; write 1 to clear."]
        #[inline(always)]
        pub const fn if_scan_end(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Scan pass end flag; write 1 to clear."]
        #[inline(always)]
        pub fn set_if_scan_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for ScanCfg2 {
        #[inline(always)]
        fn default() -> ScanCfg2 {
            ScanCfg2(0)
        }
    }
    #[doc = "RW, temperature sensor control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TemSensor(pub u8);
    impl TemSensor {
        #[doc = "RW, temperature sensor power control: 0=power down, 1=power on."]
        #[inline(always)]
        pub const fn tem_sen_pwr_on(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, temperature sensor power control: 0=power down, 1=power on."]
        #[inline(always)]
        pub fn set_tem_sen_pwr_on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for TemSensor {
        #[inline(always)]
        fn default() -> TemSensor {
            TemSensor(0)
        }
    }
}
