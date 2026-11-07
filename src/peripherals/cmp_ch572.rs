#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Comparer."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmp {
    ptr: *mut u8,
}
unsafe impl Send for Cmp {}
unsafe impl Sync for Cmp {}
impl Cmp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RW, Comparator control register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
}
pub mod regs {
    #[doc = "RW, Comparator control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u32);
    impl Ctrl {
        #[doc = "RW, Comparator enabled."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Comparator enabled."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RW, Timer captures channel signal source selection."]
        #[inline(always)]
        pub const fn cap(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Timer captures channel signal source selection."]
        #[inline(always)]
        pub fn set_cap(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RW, Comparator channel selection."]
        #[inline(always)]
        pub const fn sw(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "RW, Comparator channel selection."]
        #[inline(always)]
        pub fn set_sw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "RW, Selection of negative reference level for comparator."]
        #[inline(always)]
        pub const fn nref_level(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, Selection of negative reference level for comparator."]
        #[inline(always)]
        pub fn set_nref_level(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "RW, Comparator interrupt enable."]
        #[inline(always)]
        pub const fn int(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Comparator interrupt enable."]
        #[inline(always)]
        pub fn set_int(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "RW, Comparator interrupt generates event selection."]
        #[inline(always)]
        pub const fn out_sel(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "RW, Comparator interrupt generates event selection."]
        #[inline(always)]
        pub fn set_out_sel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
        }
        #[doc = "RW1, Comparator interrupt flag, write 1 to reset to zero."]
        #[inline(always)]
        pub const fn if_(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "RW1, Comparator interrupt flag, write 1 to reset to zero."]
        #[inline(always)]
        pub fn set_if_(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "RW, Real time output signal of comparator."]
        #[inline(always)]
        pub const fn apr_out_cmp(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Real time output signal of comparator."]
        #[inline(always)]
        pub fn set_apr_out_cmp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
}
