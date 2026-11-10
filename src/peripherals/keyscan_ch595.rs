#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "KEYSCAN register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Keyscan {
    ptr: *mut u8,
}
unsafe impl Send for Keyscan {}
unsafe impl Sync for Keyscan {}
impl Keyscan {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RW, Key scan control register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RW, Key scan interrupt enable register."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "RW, Key scan interrupt flag register."]
    #[inline(always)]
    pub const fn int_flag(self) -> crate::common::Reg<regs::IntFlag, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "RW, Key scanning key number register."]
    #[inline(always)]
    pub const fn numb(self) -> crate::common::Reg<regs::Numb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
}
pub mod regs {
    #[doc = "RW, Key scan control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u16);
    impl Ctrl {
        #[doc = "RW, Start scan enable."]
        #[must_use]
        #[inline(always)]
        pub const fn scan_start_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Start scan enable."]
        #[inline(always)]
        pub const fn set_scan_start_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "RW, Set the number of times to scan to the same key value."]
        #[must_use]
        #[inline(always)]
        pub const fn scan_cnt_end(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "RW, Set the number of times to scan to the same key value."]
        #[inline(always)]
        pub const fn set_scan_cnt_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u16) & 0x07) << 1usize);
        }
        #[doc = "RW, Scan clock division."]
        #[must_use]
        #[inline(always)]
        pub const fn scan_clk_div(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "RW, Scan clock division."]
        #[inline(always)]
        pub const fn set_scan_clk_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u16) & 0x0f) << 4usize);
        }
        #[doc = "RW, I/O pin enabling participating in key scanning."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_scan_en(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x1f;
            val as u8
        }
        #[doc = "RW, I/O pin enabling participating in key scanning."]
        #[inline(always)]
        pub const fn set_pin_scan_en(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u16) & 0x1f) << 8usize);
        }
        #[doc = "RW, Trigger wake-up signal enabling after a single scan."]
        #[must_use]
        #[inline(always)]
        pub const fn scan_1end_wake_en(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Trigger wake-up signal enabling after a single scan."]
        #[inline(always)]
        pub const fn set_scan_1end_wake_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "RW, Automatically clear wake-up signal enable after wake-up."]
        #[must_use]
        #[inline(always)]
        pub const fn clr_wakeup_en(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Automatically clear wake-up signal enable after wake-up."]
        #[inline(always)]
        pub const fn set_clr_wakeup_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    impl core::fmt::Debug for Ctrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ctrl")
                .field("scan_start_en", &self.scan_start_en())
                .field("scan_cnt_end", &self.scan_cnt_end())
                .field("scan_clk_div", &self.scan_clk_div())
                .field("pin_scan_en", &self.pin_scan_en())
                .field("scan_1end_wake_en", &self.scan_1end_wake_en())
                .field("clr_wakeup_en", &self.clr_wakeup_en())
                .finish()
        }
    }
    #[doc = "RW, Key scan interrupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u8);
    impl IntEn {
        #[doc = "RW, It is detected that the key is pressed and the number of times of the same key value reaches the set value (RB_SCAN_CNT_END) interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn key_pressed_ie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, It is detected that the key is pressed and the number of times of the same key value reaches the set value (RB_SCAN_CNT_END) interrupt enable."]
        #[inline(always)]
        pub const fn set_key_pressed_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, Interrupt enabling after a single scan."]
        #[must_use]
        #[inline(always)]
        pub const fn scan_1end_ie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Interrupt enabling after a single scan."]
        #[inline(always)]
        pub const fn set_scan_1end_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, Key release interrupt enable detected."]
        #[must_use]
        #[inline(always)]
        pub const fn key_released_ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Key release interrupt enable detected."]
        #[inline(always)]
        pub const fn set_key_released_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
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
                .field("key_pressed_ie", &self.key_pressed_ie())
                .field("scan_1end_ie", &self.scan_1end_ie())
                .field("key_released_ie", &self.key_released_ie())
                .finish()
        }
    }
    #[doc = "RW, Key scan interrupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFlag(pub u8);
    impl IntFlag {
        #[doc = "RW, It is detected that the key is pressed and the number of times of the same key value reaches the set value (RB_SCAN_CNT_END) interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn key_pressed_if(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, It is detected that the key is pressed and the number of times of the same key value reaches the set value (RB_SCAN_CNT_END) interrupt flag."]
        #[inline(always)]
        pub const fn set_key_pressed_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RW, Interrupt flag after single scan."]
        #[must_use]
        #[inline(always)]
        pub const fn scan_1end_if(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Interrupt flag after single scan."]
        #[inline(always)]
        pub const fn set_scan_1end_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RW, Key release interrupt flag detected."]
        #[must_use]
        #[inline(always)]
        pub const fn key_released_if(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, Key release interrupt flag detected."]
        #[inline(always)]
        pub const fn set_key_released_if(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
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
                .field("key_pressed_if", &self.key_pressed_if())
                .field("scan_1end_if", &self.scan_1end_if())
                .field("key_released_if", &self.key_released_if())
                .finish()
        }
    }
    #[doc = "RW, Key scanning key number register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Numb(pub u32);
    impl Numb {
        #[doc = "RW, Current key number value."]
        #[must_use]
        #[inline(always)]
        pub const fn numb(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "RW, Current key number value."]
        #[inline(always)]
        pub const fn set_numb(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "RW, Current key scanning times."]
        #[must_use]
        #[inline(always)]
        pub const fn cnt(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x07;
            val as u8
        }
        #[doc = "RW, Current key scanning times."]
        #[inline(always)]
        pub const fn set_cnt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 20usize)) | (((val as u32) & 0x07) << 20usize);
        }
    }
    impl Default for Numb {
        #[inline(always)]
        fn default() -> Numb {
            Numb(0)
        }
    }
    impl core::fmt::Debug for Numb {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Numb")
                .field("numb", &self.numb())
                .field("cnt", &self.cnt())
                .finish()
        }
    }
}
