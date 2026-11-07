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
    pub const fn ctrl_mod(self) -> crate::common::Reg<regs::CtrlModFull, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "SPI_FULL DMA control."]
    #[inline(always)]
    pub const fn ctrl_dma(self) -> crate::common::Reg<regs::CtrlDmaFull, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "SPI_FULL interrupt enable."]
    #[inline(always)]
    pub const fn inter_en(self) -> crate::common::Reg<regs::InterEnFull, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "SPI_FULL master clock divisor."]
    #[inline(always)]
    pub const fn clock_div(self) -> crate::common::Reg<regs::ClockDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "SPI_FULL slave preset value."]
    #[inline(always)]
    pub const fn slave_pre(self) -> crate::common::Reg<regs::SlavePre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "SPI_FULL data buffer."]
    #[inline(always)]
    pub const fn buffer(self) -> crate::common::Reg<regs::Buffer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "SPI_FULL work flag."]
    #[inline(always)]
    pub const fn run_flag(self) -> crate::common::Reg<regs::RunFlagFull, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "SPI_FULL interrupt flag."]
    #[inline(always)]
    pub const fn int_flag(self) -> crate::common::Reg<regs::IntFlagFull, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "SPI_FULL FIFO count status."]
    #[inline(always)]
    pub const fn fifo_count(self) -> crate::common::Reg<regs::FifoCount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
    }
    #[doc = "SPI_FULL reset command word register."]
    #[inline(always)]
    pub const fn reset_cmd(self) -> crate::common::Reg<regs::ResetCmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "SPI_FULL busy flag register."]
    #[inline(always)]
    pub const fn busy(self) -> crate::common::Reg<regs::Busy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09usize) as _) }
    }
    #[doc = "SPI_FULL total byte count, only low 12 bit."]
    #[inline(always)]
    pub const fn total_cnt(self) -> crate::common::Reg<regs::TotalCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "SPI_FULL FIFO register."]
    #[inline(always)]
    pub const fn fifo(self) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "SPI_FULL FIFO count status."]
    #[inline(always)]
    pub const fn fifo_count1(self) -> crate::common::Reg<regs::FifoCount1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x13usize) as _) }
    }
    #[doc = "SPI_FULL DMA current address."]
    #[inline(always)]
    pub const fn dma_now(self) -> crate::common::Reg<regs::DmaNow, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "SPI_FULL DMA begin address."]
    #[inline(always)]
    pub const fn dma_beg(self) -> crate::common::Reg<regs::DmaBeg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "SPI_FULL DMA end address."]
    #[inline(always)]
    pub const fn dma_end(self) -> crate::common::Reg<regs::DmaEnd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
}
#[doc = "SPI_NODMA register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiNodma {
    ptr: *mut u8,
}
unsafe impl Send for SpiNodma {}
unsafe impl Sync for SpiNodma {}
impl SpiNodma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SPI_NODMA mode control."]
    #[inline(always)]
    pub const fn ctrl_mod(self) -> crate::common::Reg<regs::CtrlModNodma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "SPI_NODMA DMA control."]
    #[inline(always)]
    pub const fn ctrl_dma(self) -> crate::common::Reg<regs::CtrlDmaNodma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01usize) as _) }
    }
    #[doc = "SPI_NODMA interrupt enable."]
    #[inline(always)]
    pub const fn inter_en(self) -> crate::common::Reg<regs::InterEnNodma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "SPI_NODMA master clock divisor."]
    #[inline(always)]
    pub const fn clock_div(self) -> crate::common::Reg<regs::ClockDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03usize) as _) }
    }
    #[doc = "SPI_NODMA data buffer."]
    #[inline(always)]
    pub const fn buffer(self) -> crate::common::Reg<regs::Buffer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "SPI_NODMA work flag."]
    #[inline(always)]
    pub const fn run_flag(self) -> crate::common::Reg<regs::RunFlagNodma, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05usize) as _) }
    }
    #[doc = "SPI_NODMA interrupt flag."]
    #[inline(always)]
    pub const fn int_flag(self) -> crate::common::Reg<regs::IntFlagNodma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "SPI_NODMA FIFO count status."]
    #[inline(always)]
    pub const fn fifo_count(self) -> crate::common::Reg<regs::FifoCount, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07usize) as _) }
    }
    #[doc = "SPI_NODMA total byte count, only low 12 bit."]
    #[inline(always)]
    pub const fn total_cnt(self) -> crate::common::Reg<regs::TotalCnt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "SPI_NODMA FIFO register."]
    #[inline(always)]
    pub const fn fifo(self) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "SPI_NODMA FIFO count status."]
    #[inline(always)]
    pub const fn fifo_count1(self) -> crate::common::Reg<regs::FifoCount1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x13usize) as _) }
    }
}
pub mod regs {
    #[doc = "SPI_FULL data buffer."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Buffer(pub u8);
    impl Buffer {
        #[doc = "SPI data buffer."]
        #[inline(always)]
        pub const fn buffer(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SPI data buffer."]
        #[inline(always)]
        pub fn set_buffer(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Buffer {
        #[inline(always)]
        fn default() -> Buffer {
            Buffer(0)
        }
    }
    #[doc = "SPI_FULL busy flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Busy(pub u8);
    impl Busy {
        #[doc = "busy flag bit."]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "busy flag bit."]
        #[inline(always)]
        pub fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for Busy {
        #[inline(always)]
        fn default() -> Busy {
            Busy(0)
        }
    }
    #[doc = "SPI_FULL master clock divisor."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClockDiv(pub u8);
    impl ClockDiv {
        #[doc = "master clock divisor."]
        #[inline(always)]
        pub const fn clock_div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "master clock divisor."]
        #[inline(always)]
        pub fn set_clock_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for ClockDiv {
        #[inline(always)]
        fn default() -> ClockDiv {
            ClockDiv(0)
        }
    }
    #[doc = "SPI_FULL DMA control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlDmaFull(pub u8);
    impl CtrlDmaFull {
        #[doc = "SPI DMA enable."]
        #[inline(always)]
        pub const fn dma_enable(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI DMA enable."]
        #[inline(always)]
        pub fn set_dma_enable(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "DMA block operates on the enable bit."]
        #[inline(always)]
        pub const fn dma_burst(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "DMA block operates on the enable bit."]
        #[inline(always)]
        pub fn set_dma_burst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "SPI DMA address loop enable."]
        #[inline(always)]
        pub const fn dma_loop(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SPI DMA address loop enable."]
        #[inline(always)]
        pub fn set_dma_loop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "High speed host receive mode control bit."]
        #[inline(always)]
        pub const fn hs_host(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "High speed host receive mode control bit."]
        #[inline(always)]
        pub fn set_hs_host(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for CtrlDmaFull {
        #[inline(always)]
        fn default() -> CtrlDmaFull {
            CtrlDmaFull(0)
        }
    }
    #[doc = "SPI_NODMA DMA control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlDmaNodma(pub u8);
    impl CtrlDmaNodma {
        #[doc = "High speed host receive mode control bit."]
        #[inline(always)]
        pub const fn hs_host(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "High speed host receive mode control bit."]
        #[inline(always)]
        pub fn set_hs_host(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for CtrlDmaNodma {
        #[inline(always)]
        fn default() -> CtrlDmaNodma {
            CtrlDmaNodma(0)
        }
    }
    #[doc = "SPI_FULL mode control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlModFull(pub u8);
    impl CtrlModFull {
        #[doc = "SPI slave mode."]
        #[inline(always)]
        pub const fn mode_slave(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "SPI slave mode."]
        #[inline(always)]
        pub fn set_mode_slave(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "force clear SPI FIFO and count."]
        #[inline(always)]
        pub const fn all_clear(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force clear SPI FIFO and count."]
        #[inline(always)]
        pub fn set_all_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "SPI enable 2 wire mode."]
        #[inline(always)]
        pub const fn _2wire_mod(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "SPI enable 2 wire mode."]
        #[inline(always)]
        pub fn set__2wire_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "SPI master clock mode."]
        #[inline(always)]
        pub const fn mst_sck_mod(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SPI master clock mode."]
        #[inline(always)]
        pub fn set_mst_sck_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "SPI slave command mode."]
        #[inline(always)]
        pub const fn slv_cmd_mod(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SPI slave command mode."]
        #[inline(always)]
        pub fn set_slv_cmd_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "SPI FIFO direction."]
        #[inline(always)]
        pub const fn fifo_dir(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SPI FIFO direction."]
        #[inline(always)]
        pub fn set_fifo_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "SPI SCK output enable."]
        #[inline(always)]
        pub const fn sck_oe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI SCK output enable."]
        #[inline(always)]
        pub fn set_sck_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "SPI MOSI output enable."]
        #[inline(always)]
        pub const fn mosi_oe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SPI MOSI output enable."]
        #[inline(always)]
        pub fn set_mosi_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "SPI MISO output enable."]
        #[inline(always)]
        pub const fn miso_oe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SPI MISO output enable."]
        #[inline(always)]
        pub fn set_miso_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for CtrlModFull {
        #[inline(always)]
        fn default() -> CtrlModFull {
            CtrlModFull(0)
        }
    }
    #[doc = "SPI_NODMA mode control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrlModNodma(pub u8);
    impl CtrlModNodma {
        #[doc = "force clear SPI FIFO and count."]
        #[inline(always)]
        pub const fn all_clear(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "force clear SPI FIFO and count."]
        #[inline(always)]
        pub fn set_all_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "SPI slave command mode."]
        #[inline(always)]
        pub const fn slv_cmd_mod(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SPI slave command mode."]
        #[inline(always)]
        pub fn set_slv_cmd_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "SPI FIFO direction."]
        #[inline(always)]
        pub const fn fifo_dir(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SPI FIFO direction."]
        #[inline(always)]
        pub fn set_fifo_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "SPI SCK output enable."]
        #[inline(always)]
        pub const fn sck_oe(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI SCK output enable."]
        #[inline(always)]
        pub fn set_sck_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "SPI MOSI output enable."]
        #[inline(always)]
        pub const fn mosi_oe(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SPI MOSI output enable."]
        #[inline(always)]
        pub fn set_mosi_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "SPI 2 wire or 3 wire SPI mode configuration bits."]
        #[inline(always)]
        pub const fn miso_oe(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SPI 2 wire or 3 wire SPI mode configuration bits."]
        #[inline(always)]
        pub fn set_miso_oe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for CtrlModNodma {
        #[inline(always)]
        fn default() -> CtrlModNodma {
            CtrlModNodma(0)
        }
    }
    #[doc = "SPI_FULL DMA begin address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaBeg(pub u32);
    impl DmaBeg {
        #[doc = "SPI DMA begin address."]
        #[inline(always)]
        pub const fn dma_beg(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "SPI DMA begin address."]
        #[inline(always)]
        pub fn set_dma_beg(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
    }
    impl Default for DmaBeg {
        #[inline(always)]
        fn default() -> DmaBeg {
            DmaBeg(0)
        }
    }
    #[doc = "SPI_FULL DMA end address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaEnd(pub u32);
    impl DmaEnd {
        #[doc = "SPI DMA end address."]
        #[inline(always)]
        pub const fn dma_end(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "SPI DMA end address."]
        #[inline(always)]
        pub fn set_dma_end(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
    }
    impl Default for DmaEnd {
        #[inline(always)]
        fn default() -> DmaEnd {
            DmaEnd(0)
        }
    }
    #[doc = "SPI_FULL DMA current address."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaNow(pub u32);
    impl DmaNow {
        #[doc = "SPI DMA current address."]
        #[inline(always)]
        pub const fn dma_now(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0003_ffff;
            val as u32
        }
        #[doc = "SPI DMA current address."]
        #[inline(always)]
        pub fn set_dma_now(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
        }
    }
    impl Default for DmaNow {
        #[inline(always)]
        fn default() -> DmaNow {
            DmaNow(0)
        }
    }
    #[doc = "SPI_FULL FIFO register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifo(pub u8);
    impl Fifo {
        #[doc = "SPI FIFO register."]
        #[inline(always)]
        pub const fn fifo(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SPI FIFO register."]
        #[inline(always)]
        pub fn set_fifo(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for Fifo {
        #[inline(always)]
        fn default() -> Fifo {
            Fifo(0)
        }
    }
    #[doc = "SPI_FULL FIFO count status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FifoCount(pub u8);
    impl FifoCount {
        #[doc = "SPI FIFO count status."]
        #[inline(always)]
        pub const fn fifo_count(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SPI FIFO count status."]
        #[inline(always)]
        pub fn set_fifo_count(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for FifoCount {
        #[inline(always)]
        fn default() -> FifoCount {
            FifoCount(0)
        }
    }
    #[doc = "SPI_FULL FIFO count status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FifoCount1(pub u8);
    impl FifoCount1 {
        #[doc = "SPI FIFO count statu."]
        #[inline(always)]
        pub const fn fifo_count1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "SPI FIFO count statu."]
        #[inline(always)]
        pub fn set_fifo_count1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for FifoCount1 {
        #[inline(always)]
        fn default() -> FifoCount1 {
            FifoCount1(0)
        }
    }
    #[doc = "SPI_FULL interrupt flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFlagFull(pub u8);
    impl IntFlagFull {
        #[doc = "interrupt flag for SPI total byte count end."]
        #[inline(always)]
        pub const fn if_cnt_end(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for SPI total byte count end."]
        #[inline(always)]
        pub fn set_if_cnt_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "interrupt flag for SPI byte exchanged."]
        #[inline(always)]
        pub const fn if_byte_end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for SPI byte exchanged."]
        #[inline(always)]
        pub fn set_if_byte_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "interrupt flag for SPI FIFO half."]
        #[inline(always)]
        pub const fn if_fifo_hf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for SPI FIFO half."]
        #[inline(always)]
        pub fn set_if_fifo_hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "interrupt flag for SPI DMA completion."]
        #[inline(always)]
        pub const fn if_dma_end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for SPI DMA completion."]
        #[inline(always)]
        pub fn set_if_dma_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "interrupt flag for SPI FIFO overflow."]
        #[inline(always)]
        pub const fn if_fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for SPI FIFO overflow."]
        #[inline(always)]
        pub fn set_if_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "interrupt flag for SPI DMA transfer error."]
        #[inline(always)]
        pub const fn if_dma_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for SPI DMA transfer error."]
        #[inline(always)]
        pub fn set_if_dma_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "current SPI free status."]
        #[inline(always)]
        pub const fn free(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "current SPI free status."]
        #[inline(always)]
        pub fn set_free(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "interrupt flag for SPI slave mode first byte received."]
        #[inline(always)]
        pub const fn if_fst_byte(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for SPI slave mode first byte received."]
        #[inline(always)]
        pub fn set_if_fst_byte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for IntFlagFull {
        #[inline(always)]
        fn default() -> IntFlagFull {
            IntFlagFull(0)
        }
    }
    #[doc = "SPI_NODMA interrupt flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFlagNodma(pub u8);
    impl IntFlagNodma {
        #[doc = "interrupt flag for SPI total byte count end."]
        #[inline(always)]
        pub const fn if_cnt_end(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for SPI total byte count end."]
        #[inline(always)]
        pub fn set_if_cnt_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "interrupt flag for SPI byte exchanged."]
        #[inline(always)]
        pub const fn if_byte_end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for SPI byte exchanged."]
        #[inline(always)]
        pub fn set_if_byte_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "interrupt flag for SPI FIFO half."]
        #[inline(always)]
        pub const fn if_fifo_hf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for SPI FIFO half."]
        #[inline(always)]
        pub fn set_if_fifo_hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "interrupt flag for SPI FIFO overflow."]
        #[inline(always)]
        pub const fn if_fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt flag for SPI FIFO overflow."]
        #[inline(always)]
        pub fn set_if_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "current SPI free status."]
        #[inline(always)]
        pub const fn free(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "current SPI free status."]
        #[inline(always)]
        pub fn set_free(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for IntFlagNodma {
        #[inline(always)]
        fn default() -> IntFlagNodma {
            IntFlagNodma(0)
        }
    }
    #[doc = "SPI_FULL interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InterEnFull(pub u8);
    impl InterEnFull {
        #[doc = "enable interrupt for SPI total byte count end."]
        #[inline(always)]
        pub const fn ie_cnt_end(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for SPI total byte count end."]
        #[inline(always)]
        pub fn set_ie_cnt_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable interrupt for SPI single byte exchanged."]
        #[inline(always)]
        pub const fn ie_byte_end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for SPI single byte exchanged."]
        #[inline(always)]
        pub fn set_ie_byte_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "enable interrupt for SPI FIFO half."]
        #[inline(always)]
        pub const fn ie_fifo_hf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for SPI FIFO half."]
        #[inline(always)]
        pub fn set_ie_fifo_hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable interrupt for SPI DMA completion."]
        #[inline(always)]
        pub const fn ie_dma_end(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for SPI DMA completion."]
        #[inline(always)]
        pub fn set_ie_dma_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "enable interrupt for SPI FIFO overflow."]
        #[inline(always)]
        pub const fn ie_fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for SPI FIFO overflow."]
        #[inline(always)]
        pub fn set_ie_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "enable interrupt for DMA transfer error."]
        #[inline(always)]
        pub const fn ie_dma_err(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for DMA transfer error."]
        #[inline(always)]
        pub fn set_ie_dma_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "enable interrupt for SPI slave mode first byte received."]
        #[inline(always)]
        pub const fn ie_fst_byte(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for SPI slave mode first byte received."]
        #[inline(always)]
        pub fn set_ie_fst_byte(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for InterEnFull {
        #[inline(always)]
        fn default() -> InterEnFull {
            InterEnFull(0)
        }
    }
    #[doc = "SPI_NODMA interrupt enable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct InterEnNodma(pub u8);
    impl InterEnNodma {
        #[doc = "enable interrupt for SPI total byte count end."]
        #[inline(always)]
        pub const fn ie_cnt_end(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for SPI total byte count end."]
        #[inline(always)]
        pub fn set_ie_cnt_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable interrupt for SPI single byte exchanged."]
        #[inline(always)]
        pub const fn ie_byte_end(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for SPI single byte exchanged."]
        #[inline(always)]
        pub fn set_ie_byte_end(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "enable interrupt for SPI FIFO half."]
        #[inline(always)]
        pub const fn ie_fifo_hf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for SPI FIFO half."]
        #[inline(always)]
        pub fn set_ie_fifo_hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable interrupt for SPI FIFO overflow."]
        #[inline(always)]
        pub const fn ie_fifo_ov(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable interrupt for SPI FIFO overflow."]
        #[inline(always)]
        pub fn set_ie_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for InterEnNodma {
        #[inline(always)]
        fn default() -> InterEnNodma {
            InterEnNodma(0)
        }
    }
    #[doc = "SPI_FULL reset command word register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResetCmd(pub u8);
    impl ResetCmd {
        #[doc = "A global reset is generated when this command is received."]
        #[inline(always)]
        pub const fn reset_cmd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "A global reset is generated when this command is received."]
        #[inline(always)]
        pub fn set_reset_cmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for ResetCmd {
        #[inline(always)]
        fn default() -> ResetCmd {
            ResetCmd(0)
        }
    }
    #[doc = "SPI_FULL work flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RunFlagFull(pub u8);
    impl RunFlagFull {
        #[doc = "SPI slave command flag."]
        #[inline(always)]
        pub const fn slv_cmd_act(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "SPI slave command flag."]
        #[inline(always)]
        pub fn set_slv_cmd_act(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "SPI FIFO ready status."]
        #[inline(always)]
        pub const fn fifo_ready(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI FIFO ready status."]
        #[inline(always)]
        pub fn set_fifo_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "SPI slave chip-select loading status."]
        #[inline(always)]
        pub const fn slv_cs_load(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "SPI slave chip-select loading status."]
        #[inline(always)]
        pub fn set_slv_cs_load(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "SPI slave selection status."]
        #[inline(always)]
        pub const fn slv_select(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "SPI slave selection status."]
        #[inline(always)]
        pub fn set_slv_select(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for RunFlagFull {
        #[inline(always)]
        fn default() -> RunFlagFull {
            RunFlagFull(0)
        }
    }
    #[doc = "SPI_NODMA work flag."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RunFlagNodma(pub u8);
    impl RunFlagNodma {
        #[doc = "SPI FIFO ready status."]
        #[inline(always)]
        pub const fn fifo_ready(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "SPI FIFO ready status."]
        #[inline(always)]
        pub fn set_fifo_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for RunFlagNodma {
        #[inline(always)]
        fn default() -> RunFlagNodma {
            RunFlagNodma(0)
        }
    }
    #[doc = "SPI_FULL slave preset value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlavePre(pub u8);
    impl SlavePre {
        #[doc = "slave preset value."]
        #[inline(always)]
        pub const fn slave_pre(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "slave preset value."]
        #[inline(always)]
        pub fn set_slave_pre(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for SlavePre {
        #[inline(always)]
        fn default() -> SlavePre {
            SlavePre(0)
        }
    }
    #[doc = "SPI_FULL total byte count, only low 12 bit."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TotalCnt(pub u16);
    impl TotalCnt {
        #[doc = "SPI total byte count, only low 12 bit."]
        #[inline(always)]
        pub const fn total_cnt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "SPI total byte count, only low 12 bit."]
        #[inline(always)]
        pub fn set_total_cnt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u16) & 0x0fff) << 0usize);
        }
    }
    impl Default for TotalCnt {
        #[inline(always)]
        fn default() -> TotalCnt {
            TotalCnt(0)
        }
    }
}
