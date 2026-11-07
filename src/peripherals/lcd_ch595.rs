#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "LCD register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcd {
    ptr: *mut u8,
}
unsafe impl Send for Lcd {}
unsafe impl Sync for Lcd {}
impl Lcd {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RW, LCD Command Register."]
    #[inline(always)]
    pub const fn cmd(self) -> crate::common::Reg<regs::Cmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "RW, LCD RAM0 register."]
    #[inline(always)]
    pub const fn ram0(self) -> crate::common::Reg<regs::Ram0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "RW, LCD RAM1 register."]
    #[inline(always)]
    pub const fn ram1(self) -> crate::common::Reg<regs::Ram1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "RW, LCD RAM2 register."]
    #[inline(always)]
    pub const fn ram2(self) -> crate::common::Reg<regs::Ram2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
}
pub mod regs {
    #[doc = "RW, LCD Command Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u32);
    impl Cmd {
        #[doc = "RW, System Enable."]
        #[inline(always)]
        pub const fn sys_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, System Enable."]
        #[inline(always)]
        pub fn set_sys_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RW, LCD switch."]
        #[inline(always)]
        pub const fn on(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, LCD switch."]
        #[inline(always)]
        pub fn set_on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RW, PWM6 Bias ratio selection enable."]
        #[inline(always)]
        pub const fn bias(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM6 Bias ratio selection enable."]
        #[inline(always)]
        pub fn set_bias(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RW, LCD duty cycle selection."]
        #[inline(always)]
        pub const fn duty(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "RW, LCD duty cycle selection."]
        #[inline(always)]
        pub fn set_duty(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u32) & 0x03) << 3usize);
        }
        #[doc = "RW, Scan clock selection."]
        #[inline(always)]
        pub const fn scan_clk_sel(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "RW, Scan clock selection."]
        #[inline(always)]
        pub fn set_scan_clk_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
        }
        #[doc = "RW, LCD driver voltage selection."]
        #[inline(always)]
        pub const fn vlcd_sel(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, LCD driver voltage selection."]
        #[inline(always)]
        pub fn set_vlcd_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "RW, SEG0-SEG7 section enable."]
        #[inline(always)]
        pub const fn seg_en1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "RW, SEG0-SEG7 section enable."]
        #[inline(always)]
        pub fn set_seg_en1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "RW, SEG8-SEG15 section enable."]
        #[inline(always)]
        pub const fn seg_en2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "RW, SEG8-SEG15 section enable."]
        #[inline(always)]
        pub fn set_seg_en2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "RW, SEG16-SEG19 section enable."]
        #[inline(always)]
        pub const fn seg_en3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG16-SEG19 section enable."]
        #[inline(always)]
        pub fn set_seg_en3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
    }
    impl Default for Cmd {
        #[inline(always)]
        fn default() -> Cmd {
            Cmd(0)
        }
    }
    #[doc = "RW, LCD RAM0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram0(pub u32);
    impl Ram0 {
        #[doc = "RW, SEG0-SEG7 segment data."]
        #[inline(always)]
        pub const fn segx(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW, SEG0-SEG7 segment data."]
        #[inline(always)]
        pub fn set_segx(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ram0 {
        #[inline(always)]
        fn default() -> Ram0 {
            Ram0(0)
        }
    }
    #[doc = "RW, LCD RAM1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram1(pub u32);
    impl Ram1 {
        #[doc = "RW, SEG8-SEG15 segment data."]
        #[inline(always)]
        pub const fn segx(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG8-SEG15 segment data."]
        #[inline(always)]
        pub fn set_segx(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Ram1 {
        #[inline(always)]
        fn default() -> Ram1 {
            Ram1(0)
        }
    }
    #[doc = "RW, LCD RAM2 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram2(pub u32);
    impl Ram2 {
        #[doc = "RW, SEG16-SEG19 segment data."]
        #[inline(always)]
        pub const fn segx(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "RW, SEG16-SEG19 segment data."]
        #[inline(always)]
        pub fn set_segx(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Ram2 {
        #[inline(always)]
        fn default() -> Ram2 {
            Ram2(0)
        }
    }
}
