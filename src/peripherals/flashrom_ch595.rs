#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Page-latch flash controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flashrom {
    ptr: *mut u8,
}
unsafe impl Send for Flashrom {}
unsafe impl Sync for Flashrom {}
impl Flashrom {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RW, pre-program mode, auto-erase and start."]
    #[inline(always)]
    pub const fn prepg_ctrl(self) -> crate::common::Reg<regs::PrepgCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RW, latch clear and load control."]
    #[inline(always)]
    pub const fn buf_ctrl(self) -> crate::common::Reg<regs::BufCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "RW, erase mode and start."]
    #[inline(always)]
    pub const fn erase_ctrl(self) -> crate::common::Reg<regs::EraseCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "RW, page program start."]
    #[inline(always)]
    pub const fn program_ctrl(self) -> crate::common::Reg<regs::ProgramCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "RW, read timing; SDK `R8_FLASH_RD_CTRL`, not in the datasheet table."]
    #[inline(always)]
    pub const fn rd_ctrl(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RW, low-power control; SDK-only, no known consumer."]
    #[inline(always)]
    pub const fn lp_ctrl(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "RW, prefetch control; SDK-only, no known consumer."]
    #[inline(always)]
    pub const fn prefetch_ctrl(self) -> crate::common::Reg<u8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "RW, data direction, read start, clock and address auto-increment."]
    #[inline(always)]
    pub const fn read_ctrl(self) -> crate::common::Reg<regs::ReadCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "WO, read-protect disable key; reads back the handshake state."]
    #[inline(always)]
    pub const fn rdprctdis_ctrl(
        self,
    ) -> crate::common::Reg<regs::RdprctdisCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "R, read-protect, debug and halted state."]
    #[inline(always)]
    pub const fn chip_status(self) -> crate::common::Reg<regs::ChipStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09usize) as _) }
    }
    #[doc = "RW, flash address; auto-increments with latch data."]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "RW, 32-bit latch data; read data when read enable is set."]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "R, second word of an aligned read."]
    #[inline(always)]
    pub const fn data1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "R, third word of an aligned read."]
    #[inline(always)]
    pub const fn data2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "R, fourth word of an aligned read."]
    #[inline(always)]
    pub const fn data3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "R, controller status."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "Latch clear and load control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BufCtrl(pub u8);
    impl BufCtrl {
        #[must_use]
        #[inline(always)]
        pub const fn clear(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Start a manual latch load."]
        #[must_use]
        #[inline(always)]
        pub const fn load_start(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Start a manual latch load."]
        #[inline(always)]
        pub const fn set_load_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "Latch each DATA write automatically."]
        #[must_use]
        #[inline(always)]
        pub const fn load_auto(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Latch each DATA write automatically."]
        #[inline(always)]
        pub const fn set_load_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for BufCtrl {
        #[inline(always)]
        fn default() -> BufCtrl {
            BufCtrl(0)
        }
    }
    impl core::fmt::Debug for BufCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BufCtrl")
                .field("clear", &self.clear())
                .field("load_start", &self.load_start())
                .field("load_auto", &self.load_auto())
                .finish()
        }
    }
    #[doc = "Chip flash state."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChipStatus(pub u8);
    impl ChipStatus {
        #[must_use]
        #[inline(always)]
        pub const fn rd_protect(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_rd_protect(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn dbg_mode(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_dbg_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Flash access blocked by the debugger."]
        #[must_use]
        #[inline(always)]
        pub const fn flash_halted(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Flash access blocked by the debugger."]
        #[inline(always)]
        pub const fn set_flash_halted(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for ChipStatus {
        #[inline(always)]
        fn default() -> ChipStatus {
            ChipStatus(0)
        }
    }
    impl core::fmt::Debug for ChipStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ChipStatus")
                .field("rd_protect", &self.rd_protect())
                .field("dbg_mode", &self.dbg_mode())
                .field("flash_halted", &self.flash_halted())
                .finish()
        }
    }
    #[doc = "Erase mode and start."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EraseCtrl(pub u8);
    impl EraseCtrl {
        #[doc = "0 = page, 1 = sector, 2/3 = bank."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "0 = page, 1 = sector, 2/3 = bank."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for EraseCtrl {
        #[inline(always)]
        fn default() -> EraseCtrl {
            EraseCtrl(0)
        }
    }
    impl core::fmt::Debug for EraseCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EraseCtrl")
                .field("mode", &self.mode())
                .field("start", &self.start())
                .finish()
        }
    }
    #[doc = "Pre-program mode and start."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PrepgCtrl(pub u8);
    impl PrepgCtrl {
        #[doc = "Run the configured erase after pre-programming."]
        #[must_use]
        #[inline(always)]
        pub const fn auto_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Run the configured erase after pre-programming."]
        #[inline(always)]
        pub const fn set_auto_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "0 = page, 1 = sector, 2 = bank with ADDR bit 8 selecting the page, 3 = bank."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "0 = page, 1 = sector, 2 = bank with ADDR bit 8 selecting the page, 3 = bank."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u8) & 0x03) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for PrepgCtrl {
        #[inline(always)]
        fn default() -> PrepgCtrl {
            PrepgCtrl(0)
        }
    }
    impl core::fmt::Debug for PrepgCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PrepgCtrl")
                .field("auto_en", &self.auto_en())
                .field("mode", &self.mode())
                .field("start", &self.start())
                .finish()
        }
    }
    #[doc = "Page program start."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ProgramCtrl(pub u8);
    impl ProgramCtrl {
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
    }
    impl Default for ProgramCtrl {
        #[inline(always)]
        fn default() -> ProgramCtrl {
            ProgramCtrl(0)
        }
    }
    impl core::fmt::Debug for ProgramCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ProgramCtrl")
                .field("start", &self.start())
                .finish()
        }
    }
    #[doc = "Read-protect disable handshake."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RdprctdisCtrl(pub u8);
    impl RdprctdisCtrl {
        #[doc = "Set while the disable sequence is running."]
        #[must_use]
        #[inline(always)]
        pub const fn disable_flag(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Set while the disable sequence is running."]
        #[inline(always)]
        pub const fn set_disable_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "0 = idle, 1-3 = running, 4 = done, 5 = failed."]
        #[must_use]
        #[inline(always)]
        pub const fn disable_state(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "0 = idle, 1-3 = running, 4 = done, 5 = failed."]
        #[inline(always)]
        pub const fn set_disable_state(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u8) & 0x07) << 1usize);
        }
        #[doc = "11b when 0x73 can start or 0x99 can clear the disable."]
        #[must_use]
        #[inline(always)]
        pub const fn disable_start(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "11b when 0x73 can start or 0x99 can clear the disable."]
        #[inline(always)]
        pub const fn set_disable_start(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for RdprctdisCtrl {
        #[inline(always)]
        fn default() -> RdprctdisCtrl {
            RdprctdisCtrl(0)
        }
    }
    impl core::fmt::Debug for RdprctdisCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RdprctdisCtrl")
                .field("disable_flag", &self.disable_flag())
                .field("disable_state", &self.disable_state())
                .field("disable_start", &self.disable_start())
                .finish()
        }
    }
    #[doc = "Data direction, read start, clock and address auto-increment."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ReadCtrl(pub u8);
    impl ReadCtrl {
        #[doc = "1 = DATA reads, 0 = DATA writes."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_rd_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "1 = DATA reads, 0 = DATA writes."]
        #[inline(always)]
        pub const fn set_cfg_rd_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Start a read; clears when the read finishes."]
        #[must_use]
        #[inline(always)]
        pub const fn rd_start(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Start a read; clears when the read finishes."]
        #[inline(always)]
        pub const fn set_rd_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Advance ADDR by four words after each transfer."]
        #[must_use]
        #[inline(always)]
        pub const fn auto_increment(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Advance ADDR by four words after each transfer."]
        #[inline(always)]
        pub const fn set_auto_increment(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "0 = one clock, 3 = two clocks high."]
        #[must_use]
        #[inline(always)]
        pub const fn clk(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "0 = one clock, 3 = two clocks high."]
        #[inline(always)]
        pub const fn set_clk(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u8) & 0x03) << 4usize);
        }
    }
    impl Default for ReadCtrl {
        #[inline(always)]
        fn default() -> ReadCtrl {
            ReadCtrl(0)
        }
    }
    impl core::fmt::Debug for ReadCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ReadCtrl")
                .field("cfg_rd_en", &self.cfg_rd_en())
                .field("rd_start", &self.rd_start())
                .field("auto_increment", &self.auto_increment())
                .field("clk", &self.clk())
                .finish()
        }
    }
    #[doc = "Controller status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Status(pub u32);
    impl Status {
        #[must_use]
        #[inline(always)]
        pub const fn ready(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_ready(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn busy(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_busy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
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
                .field("ready", &self.ready())
                .field("busy", &self.busy())
                .finish()
        }
    }
}
