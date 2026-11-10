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
    #[doc = "chip ID register."]
    #[inline(always)]
    pub const fn chip_id(self) -> crate::common::Reg<regs::ChipId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01usize) as _) }
    }
    #[doc = "safe accessing ID register."]
    #[inline(always)]
    pub const fn safe_access_id(self) -> crate::common::Reg<regs::SafeAccessId, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02usize) as _) }
    }
    #[doc = "watch-dog count register."]
    #[inline(always)]
    pub const fn wdog_count(self) -> crate::common::Reg<regs::WdogCount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03usize) as _) }
    }
    #[doc = "flash ROM configuration register."]
    #[inline(always)]
    pub const fn glob_rom_cfg(self) -> crate::common::Reg<regs::GlobRomCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "reset status and boot/debug status."]
    #[inline(always)]
    pub const fn rst_boot_stat(self) -> crate::common::Reg<regs::RstBootStat, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05usize) as _) }
    }
    #[doc = "reset and watch-dog control."]
    #[inline(always)]
    pub const fn rst_wdog_ctrl(self) -> crate::common::Reg<regs::RstWdogCtrl, crate::common::RW> {
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
    #[doc = "chip ID register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChipId(pub u8);
    impl ChipId {
        #[doc = "chip ID."]
        #[must_use]
        #[inline(always)]
        pub const fn chip_id(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "chip ID."]
        #[inline(always)]
        pub const fn set_chip_id(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for ChipId {
        #[inline(always)]
        fn default() -> ChipId {
            ChipId(0)
        }
    }
    impl core::fmt::Debug for ChipId {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ChipId")
                .field("chip_id", &self.chip_id())
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
    #[doc = "flash ROM configuration register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GlobRomCfg(pub u8);
    impl GlobRomCfg {
        #[doc = "enable flash ROM being read by external programmer."]
        #[must_use]
        #[inline(always)]
        pub const fn rom_ext_re(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable flash ROM being read by external programmer."]
        #[inline(always)]
        pub const fn set_rom_ext_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable code RAM being write."]
        #[must_use]
        #[inline(always)]
        pub const fn code_ram_we(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enable code RAM being write."]
        #[inline(always)]
        pub const fn set_code_ram_we(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "enable flash ROM data area being erase/write."]
        #[must_use]
        #[inline(always)]
        pub const fn rom_data_we(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "enable flash ROM data area being erase/write."]
        #[inline(always)]
        pub const fn set_rom_data_we(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "enable flash ROM code and data area being erase or write."]
        #[must_use]
        #[inline(always)]
        pub const fn rom_code_we(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "enable flash ROM code and data area being erase or write."]
        #[inline(always)]
        pub const fn set_rom_code_we(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "Config the start offset address of user code in Flash."]
        #[must_use]
        #[inline(always)]
        pub const fn rom_code_ofs(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Config the start offset address of user code in Flash."]
        #[inline(always)]
        pub const fn set_rom_code_ofs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for GlobRomCfg {
        #[inline(always)]
        fn default() -> GlobRomCfg {
            GlobRomCfg(0)
        }
    }
    impl core::fmt::Debug for GlobRomCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("GlobRomCfg")
                .field("rom_ext_re", &self.rom_ext_re())
                .field("code_ram_we", &self.code_ram_we())
                .field("rom_data_we", &self.rom_data_we())
                .field("rom_code_we", &self.rom_code_we())
                .field("rom_code_ofs", &self.rom_code_ofs())
                .finish()
        }
    }
    #[doc = "reset status and boot/debug status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RstBootStat(pub u8);
    impl RstBootStat {
        #[doc = "recent reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn reset_flag(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "recent reset flag."]
        #[inline(always)]
        pub const fn set_reset_flag(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "manual reset input enable status."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_reset_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "manual reset input enable status."]
        #[inline(always)]
        pub const fn set_cfg_reset_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "boot-loader enable status."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_boot_en(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "boot-loader enable status."]
        #[inline(always)]
        pub const fn set_cfg_boot_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "debug enable status."]
        #[must_use]
        #[inline(always)]
        pub const fn cfg_debug_en(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "debug enable status."]
        #[inline(always)]
        pub const fn set_cfg_debug_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "indicate boot loader status."]
        #[must_use]
        #[inline(always)]
        pub const fn boot_loader(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "indicate boot loader status."]
        #[inline(always)]
        pub const fn set_boot_loader(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
    }
    impl Default for RstBootStat {
        #[inline(always)]
        fn default() -> RstBootStat {
            RstBootStat(0)
        }
    }
    impl core::fmt::Debug for RstBootStat {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RstBootStat")
                .field("reset_flag", &self.reset_flag())
                .field("cfg_reset_en", &self.cfg_reset_en())
                .field("cfg_boot_en", &self.cfg_boot_en())
                .field("cfg_debug_en", &self.cfg_debug_en())
                .field("boot_loader", &self.boot_loader())
                .finish()
        }
    }
    #[doc = "reset and watch-dog control."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RstWdogCtrl(pub u8);
    impl RstWdogCtrl {
        #[doc = "global software reset."]
        #[must_use]
        #[inline(always)]
        pub const fn software_reset(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "global software reset."]
        #[inline(always)]
        pub const fn set_software_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "enable watch-dog reset if watch-dog timer overflow: 0=as timer only, 1=enable reset if timer overflow."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog_rst_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "enable watch-dog reset if watch-dog timer overflow: 0=as timer only, 1=enable reset if timer overflow."]
        #[inline(always)]
        pub const fn set_wdog_rst_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "watch-dog interrupt enable or INT_ID_WDOG interrupt source selection: 0=software interrupt."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog_int_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "watch-dog interrupt enable or INT_ID_WDOG interrupt source selection: 0=software interrupt."]
        #[inline(always)]
        pub const fn set_wdog_int_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "watch-dog timer overflow interrupt flag."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog_int_flag(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "watch-dog timer overflow interrupt flag."]
        #[inline(always)]
        pub const fn set_wdog_int_flag(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for RstWdogCtrl {
        #[inline(always)]
        fn default() -> RstWdogCtrl {
            RstWdogCtrl(0)
        }
    }
    impl core::fmt::Debug for RstWdogCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("RstWdogCtrl")
                .field("software_reset", &self.software_reset())
                .field("wdog_rst_en", &self.wdog_rst_en())
                .field("wdog_int_en", &self.wdog_int_en())
                .field("wdog_int_flag", &self.wdog_int_flag())
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
        #[doc = "current safe accessing mode."]
        #[must_use]
        #[inline(always)]
        pub const fn safe_acc_mode(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "current safe accessing mode."]
        #[inline(always)]
        pub const fn set_safe_acc_mode(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "safe accessing timer bit mask."]
        #[must_use]
        #[inline(always)]
        pub const fn safe_acc_timer(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "safe accessing timer bit mask."]
        #[inline(always)]
        pub const fn set_safe_acc_timer(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u8) & 0x07) << 4usize);
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
                .field("safe_acc_mode", &self.safe_acc_mode())
                .field("safe_acc_timer", &self.safe_acc_timer())
                .finish()
        }
    }
    #[doc = "watch-dog count register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct WdogCount(pub u8);
    impl WdogCount {
        #[doc = "watch-dog count."]
        #[must_use]
        #[inline(always)]
        pub const fn wdog_count(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "watch-dog count."]
        #[inline(always)]
        pub const fn set_wdog_count(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u8) & 0xff) << 0usize);
        }
    }
    impl Default for WdogCount {
        #[inline(always)]
        fn default() -> WdogCount {
            WdogCount(0)
        }
    }
    impl core::fmt::Debug for WdogCount {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("WdogCount")
                .field("wdog_count", &self.wdog_count())
                .finish()
        }
    }
}
