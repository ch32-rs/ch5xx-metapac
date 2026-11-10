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
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RW, LCD RAM0 register."]
    #[inline(always)]
    pub const fn ram0(self) -> crate::common::Reg<regs::Ram0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RW, LCD RAM1 register."]
    #[inline(always)]
    pub const fn ram1(self) -> crate::common::Reg<regs::Ram1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RW, LCD RAM2 register."]
    #[inline(always)]
    pub const fn ram2(self) -> crate::common::Reg<regs::Ram2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "RW, LCD RAM3 register."]
    #[inline(always)]
    pub const fn ram3(self) -> crate::common::Reg<regs::Ram3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "RW, LCD segment enable register."]
    #[inline(always)]
    pub const fn seg_en(self) -> crate::common::Reg<regs::SegEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
}
pub mod regs {
    #[doc = "RW, LCD Command Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cmd(pub u8);
    impl Cmd {
        #[doc = "RW, System Enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sys_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, System Enable."]
        #[inline(always)]
        pub const fn set_sys_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, LCD switch."]
        #[must_use]
        #[inline(always)]
        pub const fn on(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, LCD switch."]
        #[inline(always)]
        pub const fn set_on(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, PWM6 Bias ratio selection enable."]
        #[must_use]
        #[inline(always)]
        pub const fn bias(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, PWM6 Bias ratio selection enable."]
        #[inline(always)]
        pub const fn set_bias(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RW, LCD duty cycle selection."]
        #[must_use]
        #[inline(always)]
        pub const fn duty(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x03;
            val as u8
        }
        #[doc = "RW, LCD duty cycle selection."]
        #[inline(always)]
        pub const fn set_duty(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 3usize)) | (((val as u8) & 0x03) << 3usize);
        }
        #[doc = "RW, Scan clock selection."]
        #[must_use]
        #[inline(always)]
        pub const fn scan_clk_sel(&self) -> u8 {
            let val = (self.0 >> 5usize) & 0x03;
            val as u8
        }
        #[doc = "RW, Scan clock selection."]
        #[inline(always)]
        pub const fn set_scan_clk_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u8) & 0x03) << 5usize);
        }
        #[doc = "RW, LCD driver voltage selection."]
        #[must_use]
        #[inline(always)]
        pub const fn vlcd_sel(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, LCD driver voltage selection."]
        #[inline(always)]
        pub const fn set_vlcd_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Cmd {
        #[inline(always)]
        fn default() -> Cmd {
            Cmd(0)
        }
    }
    impl core::fmt::Debug for Cmd {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cmd")
                .field("sys_en", &self.sys_en())
                .field("on", &self.on())
                .field("bias", &self.bias())
                .field("duty", &self.duty())
                .field("scan_clk_sel", &self.scan_clk_sel())
                .field("vlcd_sel", &self.vlcd_sel())
                .finish()
        }
    }
    #[doc = "RW, LCD RAM0 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram0(pub u32);
    impl Ram0 {
        #[doc = "RW, SEG0 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG0 segment data."]
        #[inline(always)]
        pub const fn set_seg0(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "RW, SEG1 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG1 segment data."]
        #[inline(always)]
        pub const fn set_seg1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "RW, SEG2 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg2(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG2 segment data."]
        #[inline(always)]
        pub const fn set_seg2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "RW, SEG3 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg3(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG3 segment data."]
        #[inline(always)]
        pub const fn set_seg3(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "RW, SEG4 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg4(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG4 segment data."]
        #[inline(always)]
        pub const fn set_seg4(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "RW, SEG5 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg5(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG5 segment data."]
        #[inline(always)]
        pub const fn set_seg5(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "RW, SEG6 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg6(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG6 segment data."]
        #[inline(always)]
        pub const fn set_seg6(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "RW, SEG7 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg7(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG7 segment data."]
        #[inline(always)]
        pub const fn set_seg7(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Ram0 {
        #[inline(always)]
        fn default() -> Ram0 {
            Ram0(0)
        }
    }
    impl core::fmt::Debug for Ram0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram0")
                .field("seg0", &self.seg0())
                .field("seg1", &self.seg1())
                .field("seg2", &self.seg2())
                .field("seg3", &self.seg3())
                .field("seg4", &self.seg4())
                .field("seg5", &self.seg5())
                .field("seg6", &self.seg6())
                .field("seg7", &self.seg7())
                .finish()
        }
    }
    #[doc = "RW, LCD RAM1 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram1(pub u32);
    impl Ram1 {
        #[doc = "RW, SEG8 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg8(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG8 segment data."]
        #[inline(always)]
        pub const fn set_seg8(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "RW, SEG9 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg9(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG9 segment data."]
        #[inline(always)]
        pub const fn set_seg9(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "RW, SEG10 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg10(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG10 segment data."]
        #[inline(always)]
        pub const fn set_seg10(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "RW, SEG11 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg11(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG11 segment data."]
        #[inline(always)]
        pub const fn set_seg11(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "RW, SEG12 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg12(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG12 segment data."]
        #[inline(always)]
        pub const fn set_seg12(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "RW, SEG13 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg13(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG13 segment data."]
        #[inline(always)]
        pub const fn set_seg13(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "RW, SEG14 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg14(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG14 segment data."]
        #[inline(always)]
        pub const fn set_seg14(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "RW, SEG15 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg15(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG15 segment data."]
        #[inline(always)]
        pub const fn set_seg15(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Ram1 {
        #[inline(always)]
        fn default() -> Ram1 {
            Ram1(0)
        }
    }
    impl core::fmt::Debug for Ram1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram1")
                .field("seg8", &self.seg8())
                .field("seg9", &self.seg9())
                .field("seg10", &self.seg10())
                .field("seg11", &self.seg11())
                .field("seg12", &self.seg12())
                .field("seg13", &self.seg13())
                .field("seg14", &self.seg14())
                .field("seg15", &self.seg15())
                .finish()
        }
    }
    #[doc = "RW, LCD RAM2 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram2(pub u32);
    impl Ram2 {
        #[doc = "RW, SEG16 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg16(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG16 segment data."]
        #[inline(always)]
        pub const fn set_seg16(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "RW, SEG17 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg17(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG17 segment data."]
        #[inline(always)]
        pub const fn set_seg17(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "RW, SEG18 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg18(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG18 segment data."]
        #[inline(always)]
        pub const fn set_seg18(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "RW, SEG19 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg19(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG19 segment data."]
        #[inline(always)]
        pub const fn set_seg19(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
        #[doc = "RW, SEG20 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg20(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG20 segment data."]
        #[inline(always)]
        pub const fn set_seg20(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
        }
        #[doc = "RW, SEG21 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg21(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG21 segment data."]
        #[inline(always)]
        pub const fn set_seg21(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
        }
        #[doc = "RW, SEG22 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg22(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG22 segment data."]
        #[inline(always)]
        pub const fn set_seg22(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
        }
        #[doc = "RW, SEG23 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg23(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG23 segment data."]
        #[inline(always)]
        pub const fn set_seg23(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Ram2 {
        #[inline(always)]
        fn default() -> Ram2 {
            Ram2(0)
        }
    }
    impl core::fmt::Debug for Ram2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram2")
                .field("seg16", &self.seg16())
                .field("seg17", &self.seg17())
                .field("seg18", &self.seg18())
                .field("seg19", &self.seg19())
                .field("seg20", &self.seg20())
                .field("seg21", &self.seg21())
                .field("seg22", &self.seg22())
                .field("seg23", &self.seg23())
                .finish()
        }
    }
    #[doc = "RW, LCD RAM3 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ram3(pub u32);
    impl Ram3 {
        #[doc = "RW, SEG24 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg24(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG24 segment data."]
        #[inline(always)]
        pub const fn set_seg24(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "RW, SEG25 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg25(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG25 segment data."]
        #[inline(always)]
        pub const fn set_seg25(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "RW, SEG26 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg26(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG26 segment data."]
        #[inline(always)]
        pub const fn set_seg26(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "RW, SEG27 segment data."]
        #[must_use]
        #[inline(always)]
        pub const fn seg27(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, SEG27 segment data."]
        #[inline(always)]
        pub const fn set_seg27(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for Ram3 {
        #[inline(always)]
        fn default() -> Ram3 {
            Ram3(0)
        }
    }
    impl core::fmt::Debug for Ram3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ram3")
                .field("seg24", &self.seg24())
                .field("seg25", &self.seg25())
                .field("seg26", &self.seg26())
                .field("seg27", &self.seg27())
                .finish()
        }
    }
    #[doc = "RW, LCD segment enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SegEn(pub u32);
    impl SegEn {
        #[doc = "RW, Enable SEG0-SEG27 segment, high level effective."]
        #[must_use]
        #[inline(always)]
        pub const fn seg_en(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0fff_ffff;
            val as u32
        }
        #[doc = "RW, Enable SEG0-SEG27 segment, high level effective."]
        #[inline(always)]
        pub const fn set_seg_en(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
        }
    }
    impl Default for SegEn {
        #[inline(always)]
        fn default() -> SegEn {
            SegEn(0)
        }
    }
    impl core::fmt::Debug for SegEn {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SegEn")
                .field("seg_en", &self.seg_en())
                .finish()
        }
    }
}
