#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "SPI_FULL register."]
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
    #[doc = "SPI_FULL mode control."]
    #[inline(always)]
    pub const fn ctrl_mod(self) -> crate::common::Reg<regs::CtrlMod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SPI_FULL configuration control."]
    #[inline(always)]
    pub const fn ctrl_cfg(self) -> crate::common::Reg<regs::CtrlCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "SPI_FULL interrupt enable."]
    #[inline(always)]
    pub const fn inter_en(self) -> crate::common::Reg<regs::InterEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "SPI_FULL master clock divisor."]
    #[inline(always)]
    pub const fn clock_div(self) -> crate::common::Reg<regs::ClockDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "SPI_FULL slave preset value."]
    #[inline(always)]
    pub const fn slave_pre(self) -> crate::common::Reg<regs::SlavePre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "SPI_FULL data buffer."]
    #[inline(always)]
    pub const fn buffer(self) -> crate::common::Reg<regs::Buffer, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SPI_FULL work flag."]
    #[inline(always)]
    pub const fn run_flag(self) -> crate::common::Reg<regs::RunFlag, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "SPI_FULL interrupt flag."]
    #[inline(always)]
    pub const fn int_flag(self) -> crate::common::Reg<regs::IntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "SPI_FULL FIFO count status."]
    #[inline(always)]
    pub const fn fifo_count(self) -> crate::common::Reg<regs::FifoCount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "SPI_FULL total byte count, only low 12 bit."]
    #[inline(always)]
    pub const fn total_cnt(self) -> crate::common::Reg<regs::TotalCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "SPI_FULL FIFO register."]
    #[inline(always)]
    pub const fn fifo(self) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "SPI_FULL FIFO count status."]
    #[inline(always)]
    pub const fn fifo_count1(self) -> crate::common::Reg<regs::FifoCount1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x13usize) as _) }
    }
    #[doc = "SPI_FULL DMA current address."]
    #[inline(always)]
    pub const fn dma_now(self) -> crate::common::Reg<regs::DmaNow, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SPI_FULL DMA begin address."]
    #[inline(always)]
    pub const fn dma_beg(self) -> crate::common::Reg<regs::DmaBeg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "SPI_FULL DMA end address."]
    #[inline(always)]
    pub const fn dma_end(self) -> crate::common::Reg<regs::DmaEnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
}
pub mod regs {
    #[doc = "SPI_FULL data buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buffer(pub u8);
    impl Buffer {
        #[doc = "SPI data buffer."]
        #[must_use]
        #[inline(always)]
        pub const fn buffer(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SPI data buffer."]
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
    #[doc = "SPI_FULL master clock divisor."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClockDiv(pub u8);
    impl ClockDiv {
        #[doc = "master clock divisor."]
        #[must_use]
        #[inline(always)]
        pub const fn clock_div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "master clock divisor."]
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
    #[doc = "SPI_FULL configuration control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlCfg(pub u8);
    impl CtrlCfg {
        #[doc = "SPI DMA enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI DMA enable."]
        #[inline(always)]
        pub const fn set_dma_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "SPI DMA address loop enable."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_loop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SPI DMA address loop enable."]
        #[inline(always)]
        pub const fn set_dma_loop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable buffer or FIFO accessing to auto clear RB_SPI_IF_BYTE_END interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn auto_if(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable buffer or FIFO accessing to auto clear RB_SPI_IF_BYTE_END interrupt flag."]
        #[inline(always)]
        pub const fn set_auto_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "SPI bit data order."]
        #[must_use]
        #[inline(always)]
        pub const fn bit_order(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI bit data order."]
        #[inline(always)]
        pub const fn set_bit_order(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
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
                .finish()
        }
    }
    #[doc = "SPI_FULL mode control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlMod(pub u8);
    impl CtrlMod {
        #[doc = "SPI slave mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode_slave(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI slave mode."]
        #[inline(always)]
        pub const fn set_mode_slave(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "force clear SPI FIFO and count."]
        #[must_use]
        #[inline(always)]
        pub const fn all_clear(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force clear SPI FIFO and count."]
        #[inline(always)]
        pub const fn set_all_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "SPI enable 2 wire mode."]
        #[must_use]
        #[inline(always)]
        pub const fn _2wire_mod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SPI enable 2 wire mode."]
        #[inline(always)]
        pub const fn set__2wire_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "SPI master clock mode _SPI slave command mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mst_sck_mod_rb_slv_cmd_mod(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SPI master clock mode _SPI slave command mode."]
        #[inline(always)]
        pub const fn set_mst_sck_mod_rb_slv_cmd_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "SPI FIFO direction."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_dir(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SPI FIFO direction."]
        #[inline(always)]
        pub const fn set_fifo_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "SPI SCK output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sck_oe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI SCK output enable."]
        #[inline(always)]
        pub const fn set_sck_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "SPI MOSI output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mosi_oe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SPI MOSI output enable."]
        #[inline(always)]
        pub const fn set_mosi_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "SPI MISO output enable."]
        #[must_use]
        #[inline(always)]
        pub const fn miso_oe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SPI MISO output enable."]
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
                .field(
                    "mst_sck_mod_rb_slv_cmd_mod",
                    &self.mst_sck_mod_rb_slv_cmd_mod(),
                )
                .field("fifo_dir", &self.fifo_dir())
                .field("sck_oe", &self.sck_oe())
                .field("mosi_oe", &self.mosi_oe())
                .field("miso_oe", &self.miso_oe())
                .finish()
        }
    }
    #[doc = "SPI_FULL DMA begin address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaBeg(pub u32);
    impl DmaBeg {
        #[doc = "SPI DMA begin address."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_beg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "SPI DMA begin address."]
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
    #[doc = "SPI_FULL DMA end address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaEnd(pub u32);
    impl DmaEnd {
        #[doc = "SPI DMA end address."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_end(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "SPI DMA end address."]
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
    #[doc = "SPI_FULL DMA current address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaNow(pub u32);
    impl DmaNow {
        #[doc = "SPI DMA current address."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_now(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "SPI DMA current address."]
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
    #[doc = "SPI_FULL FIFO register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifo(pub u8);
    impl Fifo {
        #[doc = "SPI FIFO register."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SPI FIFO register."]
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
    #[doc = "SPI_FULL FIFO count status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FifoCount(pub u8);
    impl FifoCount {
        #[doc = "SPI FIFO count status."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_count(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SPI FIFO count status."]
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
    #[doc = "SPI_FULL FIFO count status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FifoCount1(pub u8);
    impl FifoCount1 {
        #[doc = "SPI FIFO count statu."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_count1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SPI FIFO count statu."]
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
    #[doc = "SPI_FULL interrupt flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFlag(pub u8);
    impl IntFlag {
        #[doc = "interrupt flag for SPI total byte count end."]
        #[must_use]
        #[inline(always)]
        pub const fn if_cnt_end(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for SPI total byte count end."]
        #[inline(always)]
        pub const fn set_if_cnt_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "interrupt flag for SPI byte exchanged."]
        #[must_use]
        #[inline(always)]
        pub const fn if_byte_end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for SPI byte exchanged."]
        #[inline(always)]
        pub const fn set_if_byte_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "interrupt flag for SPI FIFO half."]
        #[must_use]
        #[inline(always)]
        pub const fn if_fifo_hf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for SPI FIFO half."]
        #[inline(always)]
        pub const fn set_if_fifo_hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "interrupt flag for SPI DMA completion."]
        #[must_use]
        #[inline(always)]
        pub const fn if_dma_end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for SPI DMA completion."]
        #[inline(always)]
        pub const fn set_if_dma_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "interrupt flag for SPI FIFO overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn if_fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for SPI FIFO overflow."]
        #[inline(always)]
        pub const fn set_if_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "current SPI free status."]
        #[must_use]
        #[inline(always)]
        pub const fn free(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "current SPI free status."]
        #[inline(always)]
        pub const fn set_free(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "interrupt flag for SPI slave mode first byte received."]
        #[must_use]
        #[inline(always)]
        pub const fn if_fst_byte(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for SPI slave mode first byte received."]
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
    #[doc = "SPI_FULL interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InterEn(pub u8);
    impl InterEn {
        #[doc = "enable interrupt for SPI total byte count end."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_cnt_end(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for SPI total byte count end."]
        #[inline(always)]
        pub const fn set_ie_cnt_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable interrupt for SPI byte exchanged."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_byte_end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for SPI byte exchanged."]
        #[inline(always)]
        pub const fn set_ie_byte_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "enable interrupt for SPI FIFO half."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_fifo_hf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for SPI FIFO half."]
        #[inline(always)]
        pub const fn set_ie_fifo_hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable interrupt for SPI DMA completion."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_dma_end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for SPI DMA completion."]
        #[inline(always)]
        pub const fn set_ie_dma_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "enable interrupt for SPI FIFO overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for SPI FIFO overflow."]
        #[inline(always)]
        pub const fn set_ie_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable interrupt for SPI slave mode first byte received."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_fst_byte(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for SPI slave mode first byte received."]
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
    #[doc = "SPI_FULL work flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RunFlag(pub u8);
    impl RunFlag {
        #[doc = "SPI slave command flag."]
        #[must_use]
        #[inline(always)]
        pub const fn slv_cmd_act(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SPI slave command flag."]
        #[inline(always)]
        pub const fn set_slv_cmd_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "SPI FIFO ready status."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_ready(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI FIFO ready status."]
        #[inline(always)]
        pub const fn set_fifo_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "SPI slave chip-select loading status."]
        #[must_use]
        #[inline(always)]
        pub const fn slv_cs_load(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SPI slave chip-select loading status."]
        #[inline(always)]
        pub const fn set_slv_cs_load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "SPI slave selection status."]
        #[must_use]
        #[inline(always)]
        pub const fn slv_select(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SPI slave selection status."]
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
    #[doc = "SPI_FULL slave preset value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlavePre(pub u8);
    impl SlavePre {
        #[doc = "slave preset value."]
        #[must_use]
        #[inline(always)]
        pub const fn slave_pre(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "slave preset value."]
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
    #[doc = "SPI_FULL total byte count, only low 12 bit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TotalCnt(pub u16);
    impl TotalCnt {
        #[doc = "SPI total byte count, only low 12 bit."]
        #[must_use]
        #[inline(always)]
        pub const fn total_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "SPI total byte count, only low 12 bit."]
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
