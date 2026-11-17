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
    #[doc = "RW, pre-program mode and start."]
    #[inline(always)]
    pub const fn prepg_ctrl(self) -> crate::common::Reg<regs::PrepgCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RW, latch clear and auto-increment load."]
    #[inline(always)]
    pub const fn buf_ctrl(self) -> crate::common::Reg<regs::BufCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "RW, erase mode and start."]
    #[inline(always)]
    pub const fn erase_ctrl(self) -> crate::common::Reg<regs::EraseCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "WO, page program start."]
    #[inline(always)]
    pub const fn program_ctrl(self) -> crate::common::Reg<regs::ProgramCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "RW, flash clock selection and address auto-increment."]
    #[inline(always)]
    pub const fn read_ctrl(self) -> crate::common::Reg<regs::ReadCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
    #[doc = "RW, flash address; auto-increments with latch data."]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "RW, 32-bit latch data."]
    #[inline(always)]
    pub const fn data(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "R, controller status."]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "Latch clear and auto-increment load."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BufCtrl(pub u32);
    impl BufCtrl {
        #[must_use]
        #[inline(always)]
        pub const fn clear(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_clear(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn load_auto(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_load_auto(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
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
                .field("load_auto", &self.load_auto())
                .finish()
        }
    }
    #[doc = "Erase mode and start."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EraseCtrl(pub u32);
    impl EraseCtrl {
        #[doc = "0 = page, 1 = sector, 2 = bank."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "0 = page, 1 = sector, 2 = bank."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
    pub struct PrepgCtrl(pub u32);
    impl PrepgCtrl {
        #[doc = "0 = page, 1 = sector, 2 = bank."]
        #[must_use]
        #[inline(always)]
        pub const fn mode(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x03;
            val as u8
        }
        #[doc = "0 = page, 1 = sector, 2 = bank."]
        #[inline(always)]
        pub const fn set_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 1usize)) | (((val as u32) & 0x03) << 1usize);
        }
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
                .field("mode", &self.mode())
                .field("start", &self.start())
                .finish()
        }
    }
    #[doc = "Page program start."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ProgramCtrl(pub u32);
    impl ProgramCtrl {
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
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
    #[doc = "Flash clock selection and address auto-increment."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ReadCtrl(pub u32);
    impl ReadCtrl {
        #[must_use]
        #[inline(always)]
        pub const fn auto_increment(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub const fn set_auto_increment(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
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
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
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
