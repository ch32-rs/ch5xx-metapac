#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "System_control register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SystemControl {
    ptr: *mut u8,
}
unsafe impl Send for SystemControl {}
unsafe impl Sync for SystemControl {}
impl SystemControl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "safe accessing sign register."]
    #[inline(always)]
    pub const fn safe_access_sig(
        self,
    ) -> crate::common::Reg<regs::SafeAccessSig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "safe accessing ID register."]
    #[inline(always)]
    pub const fn safe_access_id(self) -> crate::common::Reg<regs::SafeAccessId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "watch-dog clear register."]
    #[inline(always)]
    pub const fn wdog_clear(self) -> crate::common::Reg<regs::WdogClear, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "Memory configuration register."]
    #[inline(always)]
    pub const fn glob_mem_cfg(self) -> crate::common::Reg<regs::GlobMemCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Global port lock configuration register."]
    #[inline(always)]
    pub const fn glob_lock_port(self) -> crate::common::Reg<regs::GlobLockPort, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "Globally reset the configuration register."]
    #[inline(always)]
    pub const fn glob_rst_cfg(self) -> crate::common::Reg<regs::GlobRstCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06usize) as _) }
    }
    #[doc = "value keeper during global reset."]
    #[inline(always)]
    pub const fn glob_reset_keep(
        self,
    ) -> crate::common::Reg<regs::GlobResetKeep, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07usize) as _) }
    }
}
pub mod regs {
    #[doc = "Global port lock configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GlobLockPort(pub u8);
    impl GlobLockPort {
        #[doc = "PA port lock configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn glob_lock_pa(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PA port lock configuration."]
        #[inline(always)]
        pub const fn set_glob_lock_pa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "PB port lock configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn glob_lock_pb(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "PB port lock configuration."]
        #[inline(always)]
        pub const fn set_glob_lock_pb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "PD port lock configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn glob_lock_pd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PD port lock configuration."]
        #[inline(always)]
        pub const fn set_glob_lock_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for GlobLockPort {
        #[inline(always)]
        fn default() -> GlobLockPort {
            GlobLockPort(0)
        }
    }
    impl core::fmt::Debug for GlobLockPort {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GlobLockPort")
                .field("glob_lock_pa", &self.glob_lock_pa())
                .field("glob_lock_pb", &self.glob_lock_pb())
                .field("glob_lock_pd", &self.glob_lock_pd())
                .finish()
        }
    }
    #[doc = "Memory configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GlobMemCfg(pub u8);
    impl GlobMemCfg {
        #[doc = "Memory configuration."]
        #[must_use]
        #[inline(always)]
        pub const fn glob_mem_cfg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Memory configuration."]
        #[inline(always)]
        pub const fn set_glob_mem_cfg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "Memory configuration flag."]
        #[must_use]
        #[inline(always)]
        pub const fn glob_cfg_flag(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Memory configuration flag."]
        #[inline(always)]
        pub const fn set_glob_cfg_flag(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u8) & 0x03) << 6usize);
        }
    }
    impl Default for GlobMemCfg {
        #[inline(always)]
        fn default() -> GlobMemCfg {
            GlobMemCfg(0)
        }
    }
    impl core::fmt::Debug for GlobMemCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GlobMemCfg")
                .field("glob_mem_cfg", &self.glob_mem_cfg())
                .field("glob_cfg_flag", &self.glob_cfg_flag())
                .finish()
        }
    }
    #[doc = "value keeper during global reset."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GlobResetKeep(pub u8);
    impl GlobResetKeep {
        #[doc = "value keeper during global reset."]
        #[must_use]
        #[inline(always)]
        pub const fn glob_reset_keep(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "value keeper during global reset."]
        #[inline(always)]
        pub const fn set_glob_reset_keep(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for GlobResetKeep {
        #[inline(always)]
        fn default() -> GlobResetKeep {
            GlobResetKeep(0)
        }
    }
    impl core::fmt::Debug for GlobResetKeep {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GlobResetKeep")
                .field("glob_reset_keep", &self.glob_reset_keep())
                .finish()
        }
    }
    #[doc = "Globally reset the configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GlobRstCfg(pub u8);
    impl GlobRstCfg {
        #[doc = "enable watch-dog reset."]
        #[must_use]
        #[inline(always)]
        pub const fn glob_wdog_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enable watch-dog reset."]
        #[inline(always)]
        pub const fn set_glob_wdog_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
    }
    impl Default for GlobRstCfg {
        #[inline(always)]
        fn default() -> GlobRstCfg {
            GlobRstCfg(0)
        }
    }
    impl core::fmt::Debug for GlobRstCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GlobRstCfg")
                .field("glob_wdog_en", &self.glob_wdog_en())
                .finish()
        }
    }
    #[doc = "safe accessing ID register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SafeAccessId(pub u8);
    impl SafeAccessId {
        #[doc = "safe accessing ID."]
        #[must_use]
        #[inline(always)]
        pub const fn safe_access_id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "safe accessing ID."]
        #[inline(always)]
        pub const fn set_safe_access_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for SafeAccessId {
        #[inline(always)]
        fn default() -> SafeAccessId {
            SafeAccessId(0)
        }
    }
    impl core::fmt::Debug for SafeAccessId {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SafeAccessId")
                .field("safe_access_id", &self.safe_access_id())
                .finish()
        }
    }
    #[doc = "safe accessing sign register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SafeAccessSig(pub u8);
    impl SafeAccessSig {
        #[doc = "safe accessing sign register."]
        #[must_use]
        #[inline(always)]
        pub const fn safe_access_sig(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "safe accessing sign register."]
        #[inline(always)]
        pub const fn set_safe_access_sig(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for SafeAccessSig {
        #[inline(always)]
        fn default() -> SafeAccessSig {
            SafeAccessSig(0)
        }
    }
    impl core::fmt::Debug for SafeAccessSig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SafeAccessSig")
                .field("safe_access_sig", &self.safe_access_sig())
                .finish()
        }
    }
    #[doc = "watch-dog clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdogClear(pub u8);
    impl WdogClear {
        #[doc = "watch-dog clear."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog_clear(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "watch-dog clear."]
        #[inline(always)]
        pub const fn set_wdog_clear(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for WdogClear {
        #[inline(always)]
        fn default() -> WdogClear {
            WdogClear(0)
        }
    }
    impl core::fmt::Debug for WdogClear {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WdogClear")
                .field("wdog_clear", &self.wdog_clear())
                .finish()
        }
    }
}
