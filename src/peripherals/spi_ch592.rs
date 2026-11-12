#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "SPI register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiFull {
    ptr: *mut u8,
}
unsafe impl Send for SpiFull {}
unsafe impl Sync for SpiFull {}
impl SpiFull {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RW, SPI mode control."]
    #[inline(always)]
    pub const fn ctrl_mod(self) -> crate::common::Reg<regs::CtrlMod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RW, SPI configuration control."]
    #[inline(always)]
    pub const fn ctrl_cfg(self) -> crate::common::Reg<regs::CtrlCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "RW, SPI interrupt enable."]
    #[inline(always)]
    pub const fn inter_en(self) -> crate::common::Reg<regs::InterEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "RW, SPI master clock divisor."]
    #[inline(always)]
    pub const fn clock_div(self) -> crate::common::Reg<regs::ClockDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "RW, SPI slave preset value."]
    #[inline(always)]
    pub const fn slave_pre(self) -> crate::common::Reg<regs::SlavePre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "RW, SPI data buffer."]
    #[inline(always)]
    pub const fn buffer(self) -> crate::common::Reg<regs::Buffer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RO, SPI work flag."]
    #[inline(always)]
    pub const fn run_flag(self) -> crate::common::Reg<regs::RunFlag, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "RW1, SPI interrupt flag."]
    #[inline(always)]
    pub const fn int_flag(self) -> crate::common::Reg<regs::IntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "RW, SPI FIFO count status."]
    #[inline(always)]
    pub const fn fifo_count(self) -> crate::common::Reg<regs::FifoCount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "RW, SPI mode control register 1."]
    #[inline(always)]
    pub const fn ctrl_mod1(self) -> crate::common::Reg<regs::CtrlMod1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RW, SPI mode control register 2."]
    #[inline(always)]
    pub const fn ctrl_mod2(self) -> crate::common::Reg<regs::CtrlMod2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "RW, SPI total byte count, only low 12 bit."]
    #[inline(always)]
    pub const fn total_cnt(self) -> crate::common::Reg<regs::TotalCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "RO/WO, SPI FIFO register."]
    #[inline(always)]
    pub const fn fifo(self) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "RO, SPI FIFO count status."]
    #[inline(always)]
    pub const fn fifo_count1(self) -> crate::common::Reg<regs::FifoCount1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13usize) as _) }
    }
    #[doc = "RW, SPI DMA current address."]
    #[inline(always)]
    pub const fn dma_now(self) -> crate::common::Reg<regs::DmaNow, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "RW, SPI DMA begin address."]
    #[inline(always)]
    pub const fn dma_beg(self) -> crate::common::Reg<regs::DmaBeg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "RW, SPI DMA end address."]
    #[inline(always)]
    pub const fn dma_end(self) -> crate::common::Reg<regs::DmaEnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "RW, SPI data buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buffer(pub u8);
    impl Buffer {
        #[doc = "RW, SPI data buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn buffer(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW, SPI data buffer."]
        #[inline(always)]
        pub const fn set_buffer(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Buffer {
        #[inline(always)]
        fn default() -> Buffer {
            Buffer(0)
        }
    }
    impl core::fmt::Debug for Buffer {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Buffer")
                .field("buffer", &self.buffer())
                .finish()
        }
    }
    #[doc = "RW, SPI master clock divisor."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClockDiv(pub u8);
    impl ClockDiv {
        #[doc = "RW, SPI master clock divisor."]
        #[must_use]
        #[inline(always)]
        pub const fn clock_div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW, SPI master clock divisor."]
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
    #[doc = "RW, SPI configuration control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlCfg(pub u8);
    impl CtrlCfg {
        #[doc = "RW, SPI DMA enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, SPI DMA enable."]
        #[inline(always)]
        pub const fn set_dma_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, SPI DMA address loop enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_loop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, SPI DMA address loop enable."]
        #[inline(always)]
        pub const fn set_dma_loop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, enable buffer/FIFO accessing to auto clear RB_SPI_IF_BYTE_END interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn auto_if(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable buffer/FIFO accessing to auto clear RB_SPI_IF_BYTE_END interrupt flag."]
        #[inline(always)]
        pub const fn set_auto_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RW, SPI bit data order: 0=MSB first, 1=LSB first."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_order(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RW, SPI bit data order: 0=MSB first, 1=LSB first."]
        #[inline(always)]
        pub const fn set_bit_order(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RW, SPI master input delay enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mst_dly_en(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, SPI master input delay enable."]
        #[inline(always)]
        pub const fn set_mst_dly_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for CtrlCfg {
        #[inline(always)]
        fn default() -> CtrlCfg {
            CtrlCfg(0)
        }
    }
    impl core::fmt::Debug for CtrlCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CtrlCfg")
                .field("dma_enable", &self.dma_enable())
                .field("dma_loop", &self.dma_loop())
                .field("auto_if", &self.auto_if())
                .field("bit_order", &self.bit_order())
                .field("mst_dly_en", &self.mst_dly_en())
                .finish()
        }
    }
    #[doc = "RW, SPI mode control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlMod(pub u8);
    impl CtrlMod {
        #[doc = "RW, SPI slave mode: 0=master or host, 1=slave or device."]
        #[must_use]
        #[inline(always)]
        pub const fn mode_slave(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, SPI slave mode: 0=master or host, 1=slave or device."]
        #[inline(always)]
        pub const fn set_mode_slave(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, force clear SPI FIFO and count."]
        #[must_use]
        #[inline(always)]
        pub const fn all_clear(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, force clear SPI FIFO and count."]
        #[inline(always)]
        pub const fn set_all_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, SPI enable 2 wire mode for slave: 0=3wire(SCK0,MOSI,MISO), 1=2wire(SCK0,MISO=MXSX)."]
        #[must_use]
        #[inline(always)]
        pub const fn _2wire_mod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, SPI enable 2 wire mode for slave: 0=3wire(SCK0,MOSI,MISO), 1=2wire(SCK0,MISO=MXSX)."]
        #[inline(always)]
        pub const fn set__2wire_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, SPI master clock mode: 0=mode 0, 1=mode 3."]
        #[must_use]
        #[inline(always)]
        pub const fn mst_sck_mod(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, SPI master clock mode: 0=mode 0, 1=mode 3."]
        #[inline(always)]
        pub const fn set_mst_sck_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, SPI slave command mode: 0=byte stream, 1=first byte command."]
        #[must_use]
        #[inline(always)]
        pub const fn slv_cmd_mod(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, SPI slave command mode: 0=byte stream, 1=first byte command."]
        #[inline(always)]
        pub const fn set_slv_cmd_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, SPI FIFO direction: 0=out(write @master mode), 1=in(read @master mode)."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_dir(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, SPI FIFO direction: 0=out(write @master mode), 1=in(read @master mode)."]
        #[inline(always)]
        pub const fn set_fifo_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RW, SPI SCK output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sck_oe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RW, SPI SCK output enable."]
        #[inline(always)]
        pub const fn set_sck_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RW, SPI MOSI output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mosi_oe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW, SPI MOSI output enable."]
        #[inline(always)]
        pub const fn set_mosi_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RW, SPI MISO output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn miso_oe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, SPI MISO output enable."]
        #[inline(always)]
        pub const fn set_miso_oe(&mut self, val: bool) {
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
                .field("mode_slave", &self.mode_slave())
                .field("all_clear", &self.all_clear())
                .field("_2wire_mod", &self._2wire_mod())
                .field("mst_sck_mod", &self.mst_sck_mod())
                .field("slv_cmd_mod", &self.slv_cmd_mod())
                .field("fifo_dir", &self.fifo_dir())
                .field("sck_oe", &self.sck_oe())
                .field("mosi_oe", &self.mosi_oe())
                .field("miso_oe", &self.miso_oe())
                .finish()
        }
    }
    #[doc = "RW, SPI mode control register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlMod1(pub u8);
    impl CtrlMod1 {
        #[doc = "RW, master clock polarity select while MODE_SLAVE is clear: 1 inverts the polarity, 0 keeps it. In slave mode selects the two-wire input/output direction: 1 input, 0 output."]
        #[must_use]
        #[inline(always)]
        pub const fn mst_clk_sel(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, master clock polarity select while MODE_SLAVE is clear: 1 inverts the polarity, 0 keeps it. In slave mode selects the two-wire input/output direction: 1 input, 0 output."]
        #[inline(always)]
        pub const fn set_mst_clk_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
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
                .field("mst_clk_sel", &self.mst_clk_sel())
                .finish()
        }
    }
    #[doc = "RW, SPI mode control register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlMod2(pub u8);
    impl CtrlMod2 {
        #[doc = "RW, slave fast mode for higher clock rates: 1 enabled, 0 default mode."]
        #[must_use]
        #[inline(always)]
        pub const fn slv_fast_mod(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, slave fast mode for higher clock rates: 1 enabled, 0 default mode."]
        #[inline(always)]
        pub const fn set_slv_fast_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for CtrlMod2 {
        #[inline(always)]
        fn default() -> CtrlMod2 {
            CtrlMod2(0)
        }
    }
    impl core::fmt::Debug for CtrlMod2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CtrlMod2")
                .field("slv_fast_mod", &self.slv_fast_mod())
                .finish()
        }
    }
    #[doc = "RW, SPI DMA begin address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaBeg(pub u16);
    impl DmaBeg {
        #[doc = "RW, SPI DMA begin address."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_beg(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RW, SPI DMA begin address."]
        #[inline(always)]
        pub const fn set_dma_beg(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
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
    #[doc = "RW, SPI DMA end address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaEnd(pub u16);
    impl DmaEnd {
        #[doc = "RW, SPI DMA end address."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_end(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RW, SPI DMA end address."]
        #[inline(always)]
        pub const fn set_dma_end(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
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
    #[doc = "RW, SPI DMA current address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaNow(pub u16);
    impl DmaNow {
        #[doc = "RW, SPI DMA current address."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_now(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RW, SPI DMA current address."]
        #[inline(always)]
        pub const fn set_dma_now(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
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
    #[doc = "RO/WO, SPI FIFO register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifo(pub u8);
    impl Fifo {
        #[doc = "RO/WO, SPI FIFO register."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RO/WO, SPI FIFO register."]
        #[inline(always)]
        pub const fn set_fifo(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
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
    #[doc = "RW, SPI FIFO count status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FifoCount(pub u8);
    impl FifoCount {
        #[doc = "RW, SPI FIFO count status."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_count(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SPI FIFO count status."]
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
    #[doc = "RO, SPI FIFO count status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FifoCount1(pub u8);
    impl FifoCount1 {
        #[doc = "RO, SPI FIFO count status."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_count1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RO, SPI FIFO count status."]
        #[inline(always)]
        pub const fn set_fifo_count1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for FifoCount1 {
        #[inline(always)]
        fn default() -> FifoCount1 {
            FifoCount1(0)
        }
    }
    impl core::fmt::Debug for FifoCount1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("FifoCount1")
                .field("fifo_count1", &self.fifo_count1())
                .finish()
        }
    }
    #[doc = "RW1, SPI interrupt flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFlag(pub u8);
    impl IntFlag {
        #[doc = "RW1, interrupt flag for SPI total byte count end."]
        #[must_use]
        #[inline(always)]
        pub const fn if_cnt_end(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW1, interrupt flag for SPI total byte count end."]
        #[inline(always)]
        pub const fn set_if_cnt_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW1, interrupt flag for SPI byte exchanged."]
        #[must_use]
        #[inline(always)]
        pub const fn if_byte_end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW1, interrupt flag for SPI byte exchanged."]
        #[inline(always)]
        pub const fn set_if_byte_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW1, interrupt flag for SPI FIFO half."]
        #[must_use]
        #[inline(always)]
        pub const fn if_fifo_hf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW1, interrupt flag for SPI FIFO half."]
        #[inline(always)]
        pub const fn set_if_fifo_hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW1, interrupt flag for SPI DMA completion."]
        #[must_use]
        #[inline(always)]
        pub const fn if_dma_end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW1, interrupt flag for SPI DMA completion."]
        #[inline(always)]
        pub const fn set_if_dma_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW1, interrupt flag for SPI FIFO overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn if_fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW1, interrupt flag for SPI FIFO overflow."]
        #[inline(always)]
        pub const fn set_if_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RO, current SPI free status."]
        #[must_use]
        #[inline(always)]
        pub const fn free(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RO, current SPI free status."]
        #[inline(always)]
        pub const fn set_free(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RW1, interrupt flag for SPI slave mode first byte received."]
        #[must_use]
        #[inline(always)]
        pub const fn if_fst_byte(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW1, interrupt flag for SPI slave mode first byte received."]
        #[inline(always)]
        pub const fn set_if_fst_byte(&mut self, val: bool) {
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
                .field("if_cnt_end", &self.if_cnt_end())
                .field("if_byte_end", &self.if_byte_end())
                .field("if_fifo_hf", &self.if_fifo_hf())
                .field("if_dma_end", &self.if_dma_end())
                .field("if_fifo_ov", &self.if_fifo_ov())
                .field("free", &self.free())
                .field("if_fst_byte", &self.if_fst_byte())
                .finish()
        }
    }
    #[doc = "RW, SPI interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InterEn(pub u8);
    impl InterEn {
        #[doc = "RW, enable interrupt for SPI total byte count end."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_cnt_end(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable interrupt for SPI total byte count end."]
        #[inline(always)]
        pub const fn set_ie_cnt_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, enable interrupt for SPI byte exchanged."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_byte_end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable interrupt for SPI byte exchanged."]
        #[inline(always)]
        pub const fn set_ie_byte_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, enable interrupt for SPI FIFO half."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_fifo_hf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable interrupt for SPI FIFO half."]
        #[inline(always)]
        pub const fn set_ie_fifo_hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, enable interrupt for SPI DMA completion."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_dma_end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable interrupt for SPI DMA completion."]
        #[inline(always)]
        pub const fn set_ie_dma_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RW, enable interrupt for SPI FIFO overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable interrupt for SPI FIFO overflow."]
        #[inline(always)]
        pub const fn set_ie_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RW, enable interrupt for SPI slave mode first byte received."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_fst_byte(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, enable interrupt for SPI slave mode first byte received."]
        #[inline(always)]
        pub const fn set_ie_fst_byte(&mut self, val: bool) {
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
                .field("ie_cnt_end", &self.ie_cnt_end())
                .field("ie_byte_end", &self.ie_byte_end())
                .field("ie_fifo_hf", &self.ie_fifo_hf())
                .field("ie_dma_end", &self.ie_dma_end())
                .field("ie_fifo_ov", &self.ie_fifo_ov())
                .field("ie_fst_byte", &self.ie_fst_byte())
                .finish()
        }
    }
    #[doc = "RO, SPI work flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RunFlag(pub u8);
    impl RunFlag {
        #[doc = "RO, SPI slave first byte or command flag."]
        #[must_use]
        #[inline(always)]
        pub const fn slv_cmd_act(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RO, SPI slave first byte or command flag."]
        #[inline(always)]
        pub const fn set_slv_cmd_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RO, SPI FIFO ready status."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_ready(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RO, SPI FIFO ready status."]
        #[inline(always)]
        pub const fn set_fifo_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RO, SPI slave chip-select loading status."]
        #[must_use]
        #[inline(always)]
        pub const fn slv_cs_load(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RO, SPI slave chip-select loading status."]
        #[inline(always)]
        pub const fn set_slv_cs_load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RO, SPI slave selection status."]
        #[must_use]
        #[inline(always)]
        pub const fn slv_select(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, SPI slave selection status."]
        #[inline(always)]
        pub const fn set_slv_select(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for RunFlag {
        #[inline(always)]
        fn default() -> RunFlag {
            RunFlag(0)
        }
    }
    impl core::fmt::Debug for RunFlag {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RunFlag")
                .field("slv_cmd_act", &self.slv_cmd_act())
                .field("fifo_ready", &self.fifo_ready())
                .field("slv_cs_load", &self.slv_cs_load())
                .field("slv_select", &self.slv_select())
                .finish()
        }
    }
    #[doc = "RW, SPI slave preset value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlavePre(pub u8);
    impl SlavePre {
        #[doc = "RW, SPI slave preset value."]
        #[must_use]
        #[inline(always)]
        pub const fn slave_pre(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW, SPI slave preset value."]
        #[inline(always)]
        pub const fn set_slave_pre(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for SlavePre {
        #[inline(always)]
        fn default() -> SlavePre {
            SlavePre(0)
        }
    }
    impl core::fmt::Debug for SlavePre {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SlavePre")
                .field("slave_pre", &self.slave_pre())
                .finish()
        }
    }
    #[doc = "RW, SPI total byte count, only low 12 bit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TotalCnt(pub u16);
    impl TotalCnt {
        #[doc = "RW, SPI total byte count, only low 12 bit."]
        #[must_use]
        #[inline(always)]
        pub const fn total_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RW, SPI total byte count, only low 12 bit."]
        #[inline(always)]
        pub const fn set_total_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u16) & 0xffff) << 0usize);
        }
    }
    impl Default for TotalCnt {
        #[inline(always)]
        fn default() -> TotalCnt {
            TotalCnt(0)
        }
    }
    impl core::fmt::Debug for TotalCnt {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TotalCnt")
                .field("total_cnt", &self.total_cnt())
                .finish()
        }
    }
}
