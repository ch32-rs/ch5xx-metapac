#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "EMMC register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emmc {
    ptr: *mut u8,
}
unsafe impl Send for Emmc {}
unsafe impl Sync for Emmc {}
impl Emmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SD 32bits command argument register."]
    #[inline(always)]
    pub const fn argument(self) -> crate::common::Reg<regs::Argument, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SD 16bits cmd setting register."]
    #[inline(always)]
    pub const fn cmd_set(self) -> crate::common::Reg<regs::CmdSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SD 128bits response register, \\[31:0\\]
32bits."]
    #[inline(always)]
    pub const fn response0(self) -> crate::common::Reg<regs::Response0, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "SD 128bits response register, \\[63:32\\]
32bits."]
    #[inline(always)]
    pub const fn response1(self) -> crate::common::Reg<regs::Response1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "SD 128bits response register, \\[95:64\\]
32bits."]
    #[inline(always)]
    pub const fn response2(self) -> crate::common::Reg<regs::Response2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "SD 128bits response register, \\[127:96\\]
32bits."]
    #[inline(always)]
    pub const fn response3(self) -> crate::common::Reg<regs::Response3, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Multiplexing register of the EMMC_RESPONSE3,\\[127:96\\]
32bits."]
    #[inline(always)]
    pub const fn write_cont(self) -> crate::common::Reg<regs::WriteCont, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SD 8bits control register."]
    #[inline(always)]
    pub const fn control(self) -> crate::common::Reg<regs::Control, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "SD 8bits data timeout value."]
    #[inline(always)]
    pub const fn timeout(self) -> crate::common::Reg<regs::Timeout, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "SD status."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "SD 16bits interrupt flag register."]
    #[inline(always)]
    pub const fn int_fg(self) -> crate::common::Reg<regs::IntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "SD 16bits interrupt enable register."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "SD 16bits DMA start address register when to operate."]
    #[inline(always)]
    pub const fn dma_beg1(self) -> crate::common::Reg<regs::DmaBeg1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "SD 32bits data counter, \\[15:0\\]
number of blocks this time will tran/recv, \\[27:16\\]
block sise(byte number) of every block in this time tran/recv."]
    #[inline(always)]
    pub const fn block_cfg(self) -> crate::common::Reg<regs::BlockCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "SD TRANSFER MODE register."]
    #[inline(always)]
    pub const fn tran_mode(self) -> crate::common::Reg<regs::TranMode, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "SD clock divider register."]
    #[inline(always)]
    pub const fn clk_div(self) -> crate::common::Reg<regs::ClkDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "SD 16bits DMA start address register when to operate."]
    #[inline(always)]
    pub const fn dma_beg2(self) -> crate::common::Reg<regs::DmaBeg2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
}
pub mod regs {
    #[doc = "SD 32bits command argument register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Argument(pub u32);
    impl Argument {
        #[doc = "32 bit command parameter register."]
        #[must_use]
        #[inline(always)]
        pub const fn argument(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "32 bit command parameter register."]
        #[inline(always)]
        pub const fn set_argument(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Argument {
        #[inline(always)]
        fn default() -> Argument {
            Argument(0)
        }
    }
    impl core::fmt::Debug for Argument {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Argument")
                .field("argument", &self.argument())
                .finish()
        }
    }
    #[doc = "SD 32bits data counter, \\[15:0\\]
number of blocks this time will tran/recv, \\[27:16\\]
block sise(byte number) of every block in this time tran/recv."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BlockCfg(pub u32);
    impl BlockCfg {
        #[doc = "the number of blocks to be transferred."]
        #[must_use]
        #[inline(always)]
        pub const fn bknum_mask(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the number of blocks to be transferred."]
        #[inline(always)]
        pub const fn set_bknum_mask(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "single block transfer size."]
        #[must_use]
        #[inline(always)]
        pub const fn bksize_mask(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "single block transfer size."]
        #[inline(always)]
        pub const fn set_bksize_mask(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for BlockCfg {
        #[inline(always)]
        fn default() -> BlockCfg {
            BlockCfg(0)
        }
    }
    impl core::fmt::Debug for BlockCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BlockCfg")
                .field("bknum_mask", &self.bknum_mask())
                .field("bksize_mask", &self.bksize_mask())
                .finish()
        }
    }
    #[doc = "SD clock divider register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClkDiv(pub u16);
    impl ClkDiv {
        #[doc = "clk div."]
        #[must_use]
        #[inline(always)]
        pub const fn div_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "clk div."]
        #[inline(always)]
        pub const fn set_div_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "chip output sdclk oe."]
        #[must_use]
        #[inline(always)]
        pub const fn clkoe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "chip output sdclk oe."]
        #[inline(always)]
        pub const fn set_clkoe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "EMMC clock frequency mode selection bit."]
        #[must_use]
        #[inline(always)]
        pub const fn clkmode(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "EMMC clock frequency mode selection bit."]
        #[inline(always)]
        pub const fn set_clkmode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "invert chip output sdclk phase."]
        #[must_use]
        #[inline(always)]
        pub const fn phaseinv(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "invert chip output sdclk phase."]
        #[inline(always)]
        pub const fn set_phaseinv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
    }
    impl Default for ClkDiv {
        #[inline(always)]
        fn default() -> ClkDiv {
            ClkDiv(0)
        }
    }
    impl core::fmt::Debug for ClkDiv {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ClkDiv")
                .field("div_mask", &self.div_mask())
                .field("clkoe", &self.clkoe())
                .field("clkmode", &self.clkmode())
                .field("phaseinv", &self.phaseinv())
                .finish()
        }
    }
    #[doc = "SD 16bits cmd setting register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CmdSet(pub u16);
    impl CmdSet {
        #[doc = "the index number of the currently sent command."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdidx_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x3f;
            val as u8
        }
        #[doc = "the index number of the currently sent command."]
        #[inline(always)]
        pub const fn set_cmdidx_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
        }
        #[doc = "current respone type."]
        #[must_use]
        #[inline(always)]
        pub const fn rpty_mask(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x03;
            val as u8
        }
        #[doc = "current respone type."]
        #[inline(always)]
        pub const fn set_rpty_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u16) & 0x03) << 8usize);
        }
        #[doc = "check the response CRC."]
        #[must_use]
        #[inline(always)]
        pub const fn ckcrc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "check the response CRC."]
        #[inline(always)]
        pub const fn set_ckcrc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "check the response command index."]
        #[must_use]
        #[inline(always)]
        pub const fn ckidx(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "check the response command index."]
        #[inline(always)]
        pub const fn set_ckidx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
    }
    impl Default for CmdSet {
        #[inline(always)]
        fn default() -> CmdSet {
            CmdSet(0)
        }
    }
    impl core::fmt::Debug for CmdSet {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("CmdSet")
                .field("cmdidx_mask", &self.cmdidx_mask())
                .field("rpty_mask", &self.rpty_mask())
                .field("ckcrc", &self.ckcrc())
                .field("ckidx", &self.ckidx())
                .finish()
        }
    }
    #[doc = "SD 8bits control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Control(pub u8);
    impl Control {
        #[doc = "effctive data width for sending or receiving data."]
        #[must_use]
        #[inline(always)]
        pub const fn lw_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "effctive data width for sending or receiving data."]
        #[inline(always)]
        pub const fn set_lw_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "reset all the inner logic, default is valid."]
        #[must_use]
        #[inline(always)]
        pub const fn all_clr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "reset all the inner logic, default is valid."]
        #[inline(always)]
        pub const fn set_all_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable the dma."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaen(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable the dma."]
        #[inline(always)]
        pub const fn set_dmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "reset the data tran/recv logic."]
        #[must_use]
        #[inline(always)]
        pub const fn rst_lgc(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "reset the data tran/recv logic."]
        #[inline(always)]
        pub const fn set_rst_lgc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "controller use nagedge sample cmd."]
        #[must_use]
        #[inline(always)]
        pub const fn negsmp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "controller use nagedge sample cmd."]
        #[inline(always)]
        pub const fn set_negsmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for Control {
        #[inline(always)]
        fn default() -> Control {
            Control(0)
        }
    }
    impl core::fmt::Debug for Control {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Control")
                .field("lw_mask", &self.lw_mask())
                .field("all_clr", &self.all_clr())
                .field("dmaen", &self.dmaen())
                .field("rst_lgc", &self.rst_lgc())
                .field("negsmp", &self.negsmp())
                .finish()
        }
    }
    #[doc = "SD 16bits DMA start address register when to operate."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaBeg1(pub u32);
    impl DmaBeg1 {
        #[doc = "start address of read-write data buffer,the lower 4 bits are fixed to 0."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaad1_mask(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "start address of read-write data buffer,the lower 4 bits are fixed to 0."]
        #[inline(always)]
        pub const fn set_dmaad1_mask(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for DmaBeg1 {
        #[inline(always)]
        fn default() -> DmaBeg1 {
            DmaBeg1(0)
        }
    }
    impl core::fmt::Debug for DmaBeg1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaBeg1")
                .field("dmaad1_mask", &self.dmaad1_mask())
                .finish()
        }
    }
    #[doc = "SD 16bits DMA start address register when to operate."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DmaBeg2(pub u32);
    impl DmaBeg2 {
        #[doc = "block DMA start address register."]
        #[must_use]
        #[inline(always)]
        pub const fn dmaad2_mask(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "block DMA start address register."]
        #[inline(always)]
        pub const fn set_dmaad2_mask(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for DmaBeg2 {
        #[inline(always)]
        fn default() -> DmaBeg2 {
            DmaBeg2(0)
        }
    }
    impl core::fmt::Debug for DmaBeg2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("DmaBeg2")
                .field("dmaad2_mask", &self.dmaad2_mask())
                .finish()
        }
    }
    #[doc = "SD 16bits interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u16);
    impl IntEn {
        #[doc = "command response timeout interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_re_tmout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "command response timeout interrupt enable."]
        #[inline(always)]
        pub const fn set_ie_re_tmout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "response CRC check error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_recrc_wr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "response CRC check error interrupt enable."]
        #[inline(always)]
        pub const fn set_ie_recrc_wr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "response index check error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_reidx_er(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "response index check error interrupt enable."]
        #[inline(always)]
        pub const fn set_ie_reidx_er(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "command completion interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_cmddone(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "command completion interrupt enable."]
        #[inline(always)]
        pub const fn set_ie_cmddone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "data timeout interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_dattmo(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "data timeout interrupt enable."]
        #[inline(always)]
        pub const fn set_ie_dattmo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "blocks transfer CRC error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_tranerr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "blocks transfer CRC error interrupt enable."]
        #[inline(always)]
        pub const fn set_ie_tranerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "all blocks transfer complete interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_trandone(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "all blocks transfer complete interrupt enable."]
        #[inline(always)]
        pub const fn set_ie_trandone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "single block transmission completion interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_bkgap(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "single block transmission completion interrupt enable."]
        #[inline(always)]
        pub const fn set_ie_bkgap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "FIFO overflow interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_fifo_ov(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO overflow interrupt enable."]
        #[inline(always)]
        pub const fn set_ie_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "SDIO card interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ie_sdioint(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SDIO card interrupt enable."]
        #[inline(always)]
        pub const fn set_ie_sdioint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
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
                .field("ie_re_tmout", &self.ie_re_tmout())
                .field("ie_recrc_wr", &self.ie_recrc_wr())
                .field("ie_reidx_er", &self.ie_reidx_er())
                .field("ie_cmddone", &self.ie_cmddone())
                .field("ie_dattmo", &self.ie_dattmo())
                .field("ie_tranerr", &self.ie_tranerr())
                .field("ie_trandone", &self.ie_trandone())
                .field("ie_bkgap", &self.ie_bkgap())
                .field("ie_fifo_ov", &self.ie_fifo_ov())
                .field("ie_sdioint", &self.ie_sdioint())
                .finish()
        }
    }
    #[doc = "SD 16bits interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFg(pub u16);
    impl IntFg {
        #[doc = "indicate when expect the response, timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn if_re_tmout(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "indicate when expect the response, timeout."]
        #[inline(always)]
        pub const fn set_if_re_tmout(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "indicate CRC error of the response."]
        #[must_use]
        #[inline(always)]
        pub const fn if_recrc_wr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "indicate CRC error of the response."]
        #[inline(always)]
        pub const fn set_if_recrc_wr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "indicate INDEX error of the response."]
        #[must_use]
        #[inline(always)]
        pub const fn if_reidx_er(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "indicate INDEX error of the response."]
        #[inline(always)]
        pub const fn set_if_reidx_er(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "when cmd hasn't response, indicate cmd has been sent, when cmd has a response, indicate cmd has bee sent and has received the response."]
        #[must_use]
        #[inline(always)]
        pub const fn if_cmddone(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "when cmd hasn't response, indicate cmd has been sent, when cmd has a response, indicate cmd has bee sent and has received the response."]
        #[inline(always)]
        pub const fn set_if_cmddone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "data line busy timeout."]
        #[must_use]
        #[inline(always)]
        pub const fn if_dattmo(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "data line busy timeout."]
        #[inline(always)]
        pub const fn set_if_dattmo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "last block have encountered a CRC error."]
        #[must_use]
        #[inline(always)]
        pub const fn if_tranerr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "last block have encountered a CRC error."]
        #[inline(always)]
        pub const fn set_if_tranerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "all the blocks have been tran/recv successfully."]
        #[must_use]
        #[inline(always)]
        pub const fn if_trandone(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "all the blocks have been tran/recv successfully."]
        #[inline(always)]
        pub const fn set_if_trandone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "every block gap interrupt when multiple read or write, allow drive change the DMA address at this moment."]
        #[must_use]
        #[inline(always)]
        pub const fn if_bkgap(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "every block gap interrupt when multiple read or write, allow drive change the DMA address at this moment."]
        #[inline(always)]
        pub const fn set_if_bkgap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "fifo overflow, when write sd, indicate empty overflow, when read sd, indicate full overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn if_fifo_ov(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "fifo overflow, when write sd, indicate empty overflow, when read sd, indicate full overflow."]
        #[inline(always)]
        pub const fn set_if_fifo_ov(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "interrupt from SDIO card inside."]
        #[must_use]
        #[inline(always)]
        pub const fn if_sdioint(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "interrupt from SDIO card inside."]
        #[inline(always)]
        pub const fn set_if_sdioint(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
    }
    impl Default for IntFg {
        #[inline(always)]
        fn default() -> IntFg {
            IntFg(0)
        }
    }
    impl core::fmt::Debug for IntFg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntFg")
                .field("if_re_tmout", &self.if_re_tmout())
                .field("if_recrc_wr", &self.if_recrc_wr())
                .field("if_reidx_er", &self.if_reidx_er())
                .field("if_cmddone", &self.if_cmddone())
                .field("if_dattmo", &self.if_dattmo())
                .field("if_tranerr", &self.if_tranerr())
                .field("if_trandone", &self.if_trandone())
                .field("if_bkgap", &self.if_bkgap())
                .field("if_fifo_ov", &self.if_fifo_ov())
                .field("if_sdioint", &self.if_sdioint())
                .finish()
        }
    }
    #[doc = "SD 128bits response register, \\[31:0\\]
32bits."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Response0(pub u32);
    impl Response0 {
        #[doc = "response parameter register."]
        #[must_use]
        #[inline(always)]
        pub const fn response0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "response parameter register."]
        #[inline(always)]
        pub const fn set_response0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Response0 {
        #[inline(always)]
        fn default() -> Response0 {
            Response0(0)
        }
    }
    impl core::fmt::Debug for Response0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Response0")
                .field("response0", &self.response0())
                .finish()
        }
    }
    #[doc = "SD 128bits response register, \\[63:32\\]
32bits."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Response1(pub u32);
    impl Response1 {
        #[doc = "response parameter register."]
        #[must_use]
        #[inline(always)]
        pub const fn response1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "response parameter register."]
        #[inline(always)]
        pub const fn set_response1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Response1 {
        #[inline(always)]
        fn default() -> Response1 {
            Response1(0)
        }
    }
    impl core::fmt::Debug for Response1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Response1")
                .field("response1", &self.response1())
                .finish()
        }
    }
    #[doc = "SD 128bits response register, \\[95:64\\]
32bits."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Response2(pub u32);
    impl Response2 {
        #[doc = "response parameter register."]
        #[must_use]
        #[inline(always)]
        pub const fn response2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "response parameter register."]
        #[inline(always)]
        pub const fn set_response2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Response2 {
        #[inline(always)]
        fn default() -> Response2 {
            Response2(0)
        }
    }
    impl core::fmt::Debug for Response2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Response2")
                .field("response2", &self.response2())
                .finish()
        }
    }
    #[doc = "SD 128bits response register, \\[127:96\\]
32bits."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Response3(pub u32);
    impl Response3 {
        #[doc = "response parameter register."]
        #[must_use]
        #[inline(always)]
        pub const fn response3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "response parameter register."]
        #[inline(always)]
        pub const fn set_response3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Response3 {
        #[inline(always)]
        fn default() -> Response3 {
            Response3(0)
        }
    }
    impl core::fmt::Debug for Response3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Response3")
                .field("response3", &self.response3())
                .finish()
        }
    }
    #[doc = "SD status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[doc = "the number of blocks successfully transmitted in the current multi-block transmission."]
        #[must_use]
        #[inline(always)]
        pub const fn mask_block_num(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "the number of blocks successfully transmitted in the current multi-block transmission."]
        #[inline(always)]
        pub const fn set_mask_block_num(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "indicate cmd line is high level now."]
        #[must_use]
        #[inline(always)]
        pub const fn cmdsta(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "indicate cmd line is high level now."]
        #[inline(always)]
        pub const fn set_cmdsta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "indicate dat\\[0\\]
line is high level now."]
        #[must_use]
        #[inline(always)]
        pub const fn dat0sta(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "indicate dat\\[0\\]
line is high level now."]
        #[inline(always)]
        pub const fn set_dat0sta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
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
                .field("mask_block_num", &self.mask_block_num())
                .field("cmdsta", &self.cmdsta())
                .field("dat0sta", &self.dat0sta())
                .finish()
        }
    }
    #[doc = "SD 8bits data timeout value."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timeout(pub u8);
    impl Timeout {
        #[doc = "response /data timeout configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn tocnt_mask(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "response /data timeout configuration."]
        #[inline(always)]
        pub const fn set_tocnt_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u8) & 0x0f) << 0usize);
        }
    }
    impl Default for Timeout {
        #[inline(always)]
        fn default() -> Timeout {
            Timeout(0)
        }
    }
    impl core::fmt::Debug for Timeout {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Timeout")
                .field("tocnt_mask", &self.tocnt_mask())
                .finish()
        }
    }
    #[doc = "SD TRANSFER MODE register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TranMode(pub u32);
    impl TranMode {
        #[doc = "set DMA direction is controller to emmc card."]
        #[must_use]
        #[inline(always)]
        pub const fn dma_dir(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "set DMA direction is controller to emmc card."]
        #[inline(always)]
        pub const fn set_dma_dir(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "clock stop mode after block completion."]
        #[must_use]
        #[inline(always)]
        pub const fn gap_stop(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "clock stop mode after block completion."]
        #[inline(always)]
        pub const fn set_gap_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "enable emmc boot mode."]
        #[must_use]
        #[inline(always)]
        pub const fn mode_boot(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable emmc boot mode."]
        #[inline(always)]
        pub const fn set_mode_boot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "enable auto set bTM_GAP_STOP when tran start."]
        #[must_use]
        #[inline(always)]
        pub const fn autogapstop(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "enable auto set bTM_GAP_STOP when tran start."]
        #[inline(always)]
        pub const fn set_autogapstop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "FIFO ready select signal when writing EMMC."]
        #[must_use]
        #[inline(always)]
        pub const fn fifo_rdy(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "FIFO ready select signal when writing EMMC."]
        #[inline(always)]
        pub const fn set_fifo_rdy(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "in double buffer mode,set the block count value of buffer switch."]
        #[must_use]
        #[inline(always)]
        pub const fn dmatn_cnt(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x7f;
            val as u8
        }
        #[doc = "in double buffer mode,set the block count value of buffer switch."]
        #[inline(always)]
        pub const fn set_dmatn_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 8usize)) | (((val as u32) & 0x7f) << 8usize);
        }
        #[doc = "enable double buffer dma."]
        #[must_use]
        #[inline(always)]
        pub const fn duledma_en(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "enable double buffer dma."]
        #[inline(always)]
        pub const fn set_duledma_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for TranMode {
        #[inline(always)]
        fn default() -> TranMode {
            TranMode(0)
        }
    }
    impl core::fmt::Debug for TranMode {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("TranMode")
                .field("dma_dir", &self.dma_dir())
                .field("gap_stop", &self.gap_stop())
                .field("mode_boot", &self.mode_boot())
                .field("autogapstop", &self.autogapstop())
                .field("fifo_rdy", &self.fifo_rdy())
                .field("dmatn_cnt", &self.dmatn_cnt())
                .field("duledma_en", &self.duledma_en())
                .finish()
        }
    }
    #[doc = "Multiplexing register of the EMMC_RESPONSE3,\\[127:96\\]
32bits."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WriteCont(pub u32);
    impl WriteCont {
        #[doc = "response parameter register."]
        #[must_use]
        #[inline(always)]
        pub const fn write_cont(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "response parameter register."]
        #[inline(always)]
        pub const fn set_write_cont(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for WriteCont {
        #[inline(always)]
        fn default() -> WriteCont {
            WriteCont(0)
        }
    }
    impl core::fmt::Debug for WriteCont {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WriteCont")
                .field("write_cont", &self.write_cont())
                .finish()
        }
    }
}
