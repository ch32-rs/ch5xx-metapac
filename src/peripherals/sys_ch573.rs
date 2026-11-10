#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "SYS register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sys {
    ptr: *mut u8,
}
unsafe impl Send for Sys {}
unsafe impl Sync for Sys {}
impl Sys {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RWA, flash ROM configuration."]
    #[inline(always)]
    pub const fn glob_rom_cfg(self) -> crate::common::Reg<regs::GlobRomCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RO, reset status."]
    #[inline(always)]
    pub const fn reset_status(self) -> crate::common::Reg<regs::ResetStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RWA, system clock configuration, SAM."]
    #[inline(always)]
    pub const fn clk_cfg(self) -> crate::common::Reg<regs::ClkCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "RWA, high frequency clock module power control, SAM."]
    #[inline(always)]
    pub const fn hfck_pwr_ctrl(self) -> crate::common::Reg<regs::HfckPwrCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ausize) as _) }
    }
    #[doc = "RWA, sleep clock off control byte 0, SAM."]
    #[inline(always)]
    pub const fn slp_clk_off0(self) -> crate::common::Reg<regs::SlpClkOff0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "RWA, sleep clock off control byte 1, SAM."]
    #[inline(always)]
    pub const fn slp_clk_off1(self) -> crate::common::Reg<regs::SlpClkOff1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0dusize) as _) }
    }
    #[doc = "RWA, wake control, SAM."]
    #[inline(always)]
    pub const fn slp_wake_ctrl(self) -> crate::common::Reg<regs::SlpWakeCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0eusize) as _) }
    }
    #[doc = "RWA, peripherals power down control, SAM."]
    #[inline(always)]
    pub const fn slp_power_ctrl(self) -> crate::common::Reg<regs::SlpPowerCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0fusize) as _) }
    }
    #[doc = "RW, function pin alternate configuration."]
    #[inline(always)]
    pub const fn pin_alternate(self) -> crate::common::Reg<regs::PinAlternate, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "RW, analog pin enable and digital input disable."]
    #[inline(always)]
    pub const fn pin_analog_ie(self) -> crate::common::Reg<regs::PinAnalogIe, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1ausize) as _) }
    }
    #[doc = "RWA, power plan before sleep instruction, SAM."]
    #[inline(always)]
    pub const fn power_plan(self) -> crate::common::Reg<regs::PowerPlan, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "RWA, aux power adjust control, SAM."]
    #[inline(always)]
    pub const fn aux_power_adj(self) -> crate::common::Reg<regs::AuxPowerAdj, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x22usize) as _) }
    }
    #[doc = "RWA, battery voltage detector control, SAM."]
    #[inline(always)]
    pub const fn bat_det_ctrl(self) -> crate::common::Reg<regs::BatDetCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "RWA, battery voltage detector configuration, SAM."]
    #[inline(always)]
    pub const fn bat_det_cfg(self) -> crate::common::Reg<regs::BatDetCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x25usize) as _) }
    }
    #[doc = "RO, battery status."]
    #[inline(always)]
    pub const fn bat_status(self) -> crate::common::Reg<regs::BatStatus, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x26usize) as _) }
    }
    #[doc = "RWA, internal 32KHz oscillator tune control, SAM."]
    #[inline(always)]
    pub const fn int32k_tune(self) -> crate::common::Reg<regs::Int32kTune, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "RWA, external 32KHz oscillator tune control, SAM."]
    #[inline(always)]
    pub const fn xt32k_tune(self) -> crate::common::Reg<regs::Xt32kTune, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2eusize) as _) }
    }
    #[doc = "RWA, 32KHz oscillator configure."]
    #[inline(always)]
    pub const fn ck32k_config(self) -> crate::common::Reg<regs::Ck32kConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2fusize) as _) }
    }
}
pub mod regs {
    #[doc = "RWA, aux power adjust control, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AuxPowerAdj(pub u8);
    impl AuxPowerAdj {
        #[doc = "RWA, Ultra-Low-Power LDO voltage adjust."]
        #[must_use]
        #[inline(always)]
        pub const fn ulpldo_adj(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "RWA, Ultra-Low-Power LDO voltage adjust."]
        #[inline(always)]
        pub const fn set_ulpldo_adj(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
    }
    impl Default for AuxPowerAdj {
        #[inline(always)]
        fn default() -> AuxPowerAdj {
            AuxPowerAdj(0)
        }
    }
    impl core::fmt::Debug for AuxPowerAdj {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("AuxPowerAdj")
                .field("ulpldo_adj", &self.ulpldo_adj())
                .finish()
        }
    }
    #[doc = "RWA, battery voltage detector configuration, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BatDetCfg(pub u8);
    impl BatDetCfg {
        #[doc = "RWA, select threshold voltage of battery voltage low."]
        #[must_use]
        #[inline(always)]
        pub const fn bat_low_vth(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "RWA, select threshold voltage of battery voltage low."]
        #[inline(always)]
        pub const fn set_bat_low_vth(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
    }
    impl Default for BatDetCfg {
        #[inline(always)]
        fn default() -> BatDetCfg {
            BatDetCfg(0)
        }
    }
    impl core::fmt::Debug for BatDetCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BatDetCfg")
                .field("bat_low_vth", &self.bat_low_vth())
                .finish()
        }
    }
    #[doc = "RWA, battery voltage detector control, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BatDetCtrl(pub u8);
    impl BatDetCtrl {
        #[doc = "RWA, battery voltage detector enable/select monitor threshold voltage."]
        #[must_use]
        #[inline(always)]
        pub const fn bat_det_en_rb_bat_low_vthx(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, battery voltage detector enable/select monitor threshold voltage."]
        #[inline(always)]
        pub const fn set_bat_det_en_rb_bat_low_vthx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RWA, battery voltage monitor enable under sleep mode."]
        #[must_use]
        #[inline(always)]
        pub const fn bat_mon_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, battery voltage monitor enable under sleep mode."]
        #[inline(always)]
        pub const fn set_bat_mon_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RWA, interrupt enable for battery lower voltage."]
        #[must_use]
        #[inline(always)]
        pub const fn bat_lower_ie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, interrupt enable for battery lower voltage."]
        #[inline(always)]
        pub const fn set_bat_lower_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RWA, interrupt enable for battery low voltage."]
        #[must_use]
        #[inline(always)]
        pub const fn bat_low_ie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, interrupt enable for battery low voltage."]
        #[inline(always)]
        pub const fn set_bat_low_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
    }
    impl Default for BatDetCtrl {
        #[inline(always)]
        fn default() -> BatDetCtrl {
            BatDetCtrl(0)
        }
    }
    impl core::fmt::Debug for BatDetCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BatDetCtrl")
                .field(
                    "bat_det_en_rb_bat_low_vthx",
                    &self.bat_det_en_rb_bat_low_vthx(),
                )
                .field("bat_mon_en", &self.bat_mon_en())
                .field("bat_lower_ie", &self.bat_lower_ie())
                .field("bat_low_ie", &self.bat_low_ie())
                .finish()
        }
    }
    #[doc = "RO, battery status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BatStatus(pub u8);
    impl BatStatus {
        #[doc = "RO, battery lower voltage status, high action."]
        #[must_use]
        #[inline(always)]
        pub const fn bat_stat_lower(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RO, battery lower voltage status, high action."]
        #[inline(always)]
        pub const fn set_bat_stat_lower(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RO, battery low voltage status, high action."]
        #[must_use]
        #[inline(always)]
        pub const fn bat_stat_low(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RO, battery low voltage status, high action."]
        #[inline(always)]
        pub const fn set_bat_stat_low(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
    }
    impl Default for BatStatus {
        #[inline(always)]
        fn default() -> BatStatus {
            BatStatus(0)
        }
    }
    impl core::fmt::Debug for BatStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BatStatus")
                .field("bat_stat_lower", &self.bat_stat_lower())
                .field("bat_stat_low", &self.bat_stat_low())
                .finish()
        }
    }
    #[doc = "RWA, 32KHz oscillator configure."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ck32kConfig(pub u8);
    impl Ck32kConfig {
        #[doc = "RWA, external 32KHz oscillator power on."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_xt32k_pon(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, external 32KHz oscillator power on."]
        #[inline(always)]
        pub const fn set_clk_xt32k_pon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RWA, internal 32KHz oscillator power on."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_int32k_pon(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, internal 32KHz oscillator power on."]
        #[inline(always)]
        pub const fn set_clk_int32k_pon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RWA, 32KHz oscillator source selection: 0=RC, 1=XT."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_osc32k_xt(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, 32KHz oscillator source selection: 0=RC, 1=XT."]
        #[inline(always)]
        pub const fn set_clk_osc32k_xt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RO, 32KHz oscillator clock pin status."]
        #[must_use]
        #[inline(always)]
        pub const fn _32k_clk_pin(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RO, 32KHz oscillator clock pin status."]
        #[inline(always)]
        pub const fn set__32k_clk_pin(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for Ck32kConfig {
        #[inline(always)]
        fn default() -> Ck32kConfig {
            Ck32kConfig(0)
        }
    }
    impl core::fmt::Debug for Ck32kConfig {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ck32kConfig")
                .field("clk_xt32k_pon", &self.clk_xt32k_pon())
                .field("clk_int32k_pon", &self.clk_int32k_pon())
                .field("clk_osc32k_xt", &self.clk_osc32k_xt())
                .field("_32k_clk_pin", &self._32k_clk_pin())
                .finish()
        }
    }
    #[doc = "RWA, system clock configuration, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClkCfg(pub u16);
    impl ClkCfg {
        #[doc = "RWA, output clock divider from PLL or CK32M."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_pll_div(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "RWA, output clock divider from PLL or CK32M."]
        #[inline(always)]
        pub const fn set_clk_pll_div(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u16) & 0x1f) << 0usize);
        }
        #[doc = "RWA, system clock source mode: 00=divided from 32MHz, 01=divided from PLL-480MHz, 10=directly from 32MHz, 11=directly from 32KHz."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_sys_mod(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "RWA, system clock source mode: 00=divided from 32MHz, 01=divided from PLL-480MHz, 10=directly from 32MHz, 11=directly from 32KHz."]
        #[inline(always)]
        pub const fn set_clk_sys_mod(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u16) & 0x03) << 6usize);
        }
    }
    impl Default for ClkCfg {
        #[inline(always)]
        fn default() -> ClkCfg {
            ClkCfg(0)
        }
    }
    impl core::fmt::Debug for ClkCfg {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ClkCfg")
                .field("clk_pll_div", &self.clk_pll_div())
                .field("clk_sys_mod", &self.clk_sys_mod())
                .finish()
        }
    }
    #[doc = "RWA, flash ROM configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GlobRomCfg(pub u8);
    impl GlobRomCfg {
        #[doc = "RWA, code offset address selection in Flash ROM: 0=start address 0x000000, 1=start address 0x008000."]
        #[must_use]
        #[inline(always)]
        pub const fn rom_code_ofs(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, code offset address selection in Flash ROM: 0=start address 0x000000, 1=start address 0x008000."]
        #[inline(always)]
        pub const fn set_rom_code_ofs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RWA, enable flash ROM control interface enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rom_ctrl_en(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, enable flash ROM control interface enable."]
        #[inline(always)]
        pub const fn set_rom_ctrl_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RWA,enable flash ROM data and code area being erase/write."]
        #[must_use]
        #[inline(always)]
        pub const fn rom_data_we(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RWA,enable flash ROM data and code area being erase/write."]
        #[inline(always)]
        pub const fn set_rom_data_we(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RWA, enable flash ROM code area being erase or write."]
        #[must_use]
        #[inline(always)]
        pub const fn rom_code_we(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, enable flash ROM code area being erase or write."]
        #[inline(always)]
        pub const fn set_rom_code_we(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
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
                .field("rom_code_ofs", &self.rom_code_ofs())
                .field("rom_ctrl_en", &self.rom_ctrl_en())
                .field("rom_data_we", &self.rom_data_we())
                .field("rom_code_we", &self.rom_code_we())
                .finish()
        }
    }
    #[doc = "RWA, high frequency clock module power control, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HfckPwrCtrl(pub u8);
    impl HfckPwrCtrl {
        #[doc = "RWA, external 32MHz oscillator power control: 0=power down, 1-power on."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_xt32m_pon(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, external 32MHz oscillator power control: 0=power down, 1-power on."]
        #[inline(always)]
        pub const fn set_clk_xt32m_pon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RWA, PLL power control: 0=power down, 1-power on."]
        #[must_use]
        #[inline(always)]
        pub const fn clk_pll_pon(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, PLL power control: 0=power down, 1-power on."]
        #[inline(always)]
        pub const fn set_clk_pll_pon(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
    }
    impl Default for HfckPwrCtrl {
        #[inline(always)]
        fn default() -> HfckPwrCtrl {
            HfckPwrCtrl(0)
        }
    }
    impl core::fmt::Debug for HfckPwrCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HfckPwrCtrl")
                .field("clk_xt32m_pon", &self.clk_xt32m_pon())
                .field("clk_pll_pon", &self.clk_pll_pon())
                .finish()
        }
    }
    #[doc = "RWA, internal 32KHz oscillator tune control, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Int32kTune(pub u16);
    impl Int32kTune {
        #[doc = "RWA, internal 32KHz oscillator frequency tune."]
        #[must_use]
        #[inline(always)]
        pub const fn int32k_tune(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "RWA, internal 32KHz oscillator frequency tune."]
        #[inline(always)]
        pub const fn set_int32k_tune(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u16) & 0x0fff) << 0usize);
        }
    }
    impl Default for Int32kTune {
        #[inline(always)]
        fn default() -> Int32kTune {
            Int32kTune(0)
        }
    }
    impl core::fmt::Debug for Int32kTune {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Int32kTune")
                .field("int32k_tune", &self.int32k_tune())
                .finish()
        }
    }
    #[doc = "RW, function pin alternate configuration."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PinAlternate(pub u16);
    impl PinAlternate {
        #[doc = "RW, TMR0 alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_tmr0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW, TMR0 alternate pin enable."]
        #[inline(always)]
        pub const fn set_pin_tmr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "RW, TMR1 alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_tmr1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW, TMR1 alternate pin enable."]
        #[inline(always)]
        pub const fn set_pin_tmr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "RW, TMR2 alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_tmr2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW, TMR2 alternate pin enable."]
        #[inline(always)]
        pub const fn set_pin_tmr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "RW, RXD0/TXD0 alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_uart0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW, RXD0/TXD0 alternate pin enable."]
        #[inline(always)]
        pub const fn set_pin_uart0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "RW, RXD1/TXD1 alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_uart1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RW, RXD1/TXD1 alternate pin enable."]
        #[inline(always)]
        pub const fn set_pin_uart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u16) & 0x01) << 5usize);
        }
        #[doc = "RW, SCS/SCK0/MOSI/MISO alternate pin enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_spi0(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RW, SCS/SCK0/MOSI/MISO alternate pin enable."]
        #[inline(always)]
        pub const fn set_pin_spi0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
    }
    impl Default for PinAlternate {
        #[inline(always)]
        fn default() -> PinAlternate {
            PinAlternate(0)
        }
    }
    impl core::fmt::Debug for PinAlternate {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PinAlternate")
                .field("pin_tmr0", &self.pin_tmr0())
                .field("pin_tmr1", &self.pin_tmr1())
                .field("pin_tmr2", &self.pin_tmr2())
                .field("pin_uart0", &self.pin_uart0())
                .field("pin_uart1", &self.pin_uart1())
                .field("pin_spi0", &self.pin_spi0())
                .finish()
        }
    }
    #[doc = "RW, analog pin enable and digital input disable."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PinAnalogIe(pub u16);
    impl PinAnalogIe {
        #[doc = "RW,USB UDP internal pullup resistance enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_usb_dp_pu(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RW,USB UDP internal pullup resistance enable."]
        #[inline(always)]
        pub const fn set_pin_usb_dp_pu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u16) & 0x01) << 6usize);
        }
        #[doc = "RW, USB analog I/O enable: 0=analog I/O disable, 1=analog I/O enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_usb_ie(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RW, USB analog I/O enable: 0=analog I/O disable, 1=analog I/O enable."]
        #[inline(always)]
        pub const fn set_pin_usb_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "RW, ADC/TouchKey channel 9/8 digital input disable: 0=digital input enable, 1=digital input disable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_adc8_9_ie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC/TouchKey channel 9/8 digital input disable: 0=digital input enable, 1=digital input disable."]
        #[inline(always)]
        pub const fn set_pin_adc8_9_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "RW, ADC/TouchKey channel0 digital input disable: 0=digital input enable, 1=digital input disable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_adc0_ie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC/TouchKey channel0 digital input disable: 0=digital input enable, 1=digital input disable."]
        #[inline(always)]
        pub const fn set_pin_adc0_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "RW, ADC/TouchKey channel1 digital input disable: 0=digital input enable, 1=digital input disable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_adc1_ie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC/TouchKey channel1 digital input disable: 0=digital input enable, 1=digital input disable."]
        #[inline(always)]
        pub const fn set_pin_adc1_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "RW, ADC/TouchKey channel12 digital input disable: 0=digital input enable, 1=digital input disable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_adc12_ie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC/TouchKey channel12 digital input disable: 0=digital input enable, 1=digital input disable."]
        #[inline(always)]
        pub const fn set_pin_adc12_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u16) & 0x01) << 11usize);
        }
        #[doc = "RW, ADC/TouchKey channel13 digital input disable: 0=digital input enable, 1=digital input disable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_adc13_ie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC/TouchKey channel13 digital input disable: 0=digital input enable, 1=digital input disable."]
        #[inline(always)]
        pub const fn set_pin_adc13_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u16) & 0x01) << 12usize);
        }
        #[doc = "RW, external 32KHz oscillator digital input disable: 0=digital input enable, 1=digital input disable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_xt32k_ie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "RW, external 32KHz oscillator digital input disable: 0=digital input enable, 1=digital input disable."]
        #[inline(always)]
        pub const fn set_pin_xt32k_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
        #[doc = "RW, ADC/TouchKey channel 2/3 digital input disable: 0=digital input enable, 1=digital input disable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_adc2_3_ie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC/TouchKey channel 2/3 digital input disable: 0=digital input enable, 1=digital input disable."]
        #[inline(always)]
        pub const fn set_pin_adc2_3_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u16) & 0x01) << 14usize);
        }
        #[doc = "RW, ADC/TouchKey channel 4/5 digital input disable: 0=digital input enable, 1=digital input disable."]
        #[must_use]
        #[inline(always)]
        pub const fn pin_adc4_5_ie(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RW, ADC/TouchKey channel 4/5 digital input disable: 0=digital input enable, 1=digital input disable."]
        #[inline(always)]
        pub const fn set_pin_adc4_5_ie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for PinAnalogIe {
        #[inline(always)]
        fn default() -> PinAnalogIe {
            PinAnalogIe(0)
        }
    }
    impl core::fmt::Debug for PinAnalogIe {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PinAnalogIe")
                .field("pin_usb_dp_pu", &self.pin_usb_dp_pu())
                .field("pin_usb_ie", &self.pin_usb_ie())
                .field("pin_adc8_9_ie", &self.pin_adc8_9_ie())
                .field("pin_adc0_ie", &self.pin_adc0_ie())
                .field("pin_adc1_ie", &self.pin_adc1_ie())
                .field("pin_adc12_ie", &self.pin_adc12_ie())
                .field("pin_adc13_ie", &self.pin_adc13_ie())
                .field("pin_xt32k_ie", &self.pin_xt32k_ie())
                .field("pin_adc2_3_ie", &self.pin_adc2_3_ie())
                .field("pin_adc4_5_ie", &self.pin_adc4_5_ie())
                .finish()
        }
    }
    #[doc = "RWA, power plan before sleep instruction, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PowerPlan(pub u16);
    impl PowerPlan {
        #[doc = "RWA, power for retention 2KB SRAM."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_xrom(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, power for retention 2KB SRAM."]
        #[inline(always)]
        pub const fn set_pwr_xrom(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "RWA, power for retention 2KB SRAM."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_ram2k(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, power for retention 2KB SRAM."]
        #[inline(always)]
        pub const fn set_pwr_ram2k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "RWA, power retention for core and base peripherals."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_core(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, power retention for core and base peripherals."]
        #[inline(always)]
        pub const fn set_pwr_core(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "RWA, power retention for USB and BLE."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_extend(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, power retention for USB and BLE."]
        #[inline(always)]
        pub const fn set_pwr_extend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "RWA, power for main SRAM."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_ram16k(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, power for main SRAM."]
        #[inline(always)]
        pub const fn set_pwr_ram16k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u16) & 0x01) << 4usize);
        }
        #[doc = "RWA, power for system."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_sys_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, power for system."]
        #[inline(always)]
        pub const fn set_pwr_sys_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "RWA, LDO enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_ldo_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, LDO enable."]
        #[inline(always)]
        pub const fn set_pwr_ldo_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "RWA, DC/DC converter enable: 0=DC/DC disable and bypass, 1=DC/DC enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_dcdc_en(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, DC/DC converter enable: 0=DC/DC disable and bypass, 1=DC/DC enable."]
        #[inline(always)]
        pub const fn set_pwr_dcdc_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "RWA, DC/DC converter pre-enable."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_dcdc_pre(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, DC/DC converter pre-enable."]
        #[inline(always)]
        pub const fn set_pwr_dcdc_pre(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u16) & 0x01) << 10usize);
        }
        #[doc = "RWA, power plan enable, auto clear after sleep executed."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_must_0010(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x0f;
            val as u8
        }
        #[doc = "RWA, power plan enable, auto clear after sleep executed."]
        #[inline(always)]
        pub const fn set_pwr_must_0010(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u16) & 0x0f) << 11usize);
        }
        #[doc = "RWA, must write 0010."]
        #[must_use]
        #[inline(always)]
        pub const fn pwr_plan_en(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, must write 0010."]
        #[inline(always)]
        pub const fn set_pwr_plan_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
        }
    }
    impl Default for PowerPlan {
        #[inline(always)]
        fn default() -> PowerPlan {
            PowerPlan(0)
        }
    }
    impl core::fmt::Debug for PowerPlan {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PowerPlan")
                .field("pwr_xrom", &self.pwr_xrom())
                .field("pwr_ram2k", &self.pwr_ram2k())
                .field("pwr_core", &self.pwr_core())
                .field("pwr_extend", &self.pwr_extend())
                .field("pwr_ram16k", &self.pwr_ram16k())
                .field("pwr_sys_en", &self.pwr_sys_en())
                .field("pwr_ldo_en", &self.pwr_ldo_en())
                .field("pwr_dcdc_en", &self.pwr_dcdc_en())
                .field("pwr_dcdc_pre", &self.pwr_dcdc_pre())
                .field("pwr_must_0010", &self.pwr_must_0010())
                .field("pwr_plan_en", &self.pwr_plan_en())
                .finish()
        }
    }
    #[doc = "RWA, reset status."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ResetStatus(pub u8);
    impl ResetStatus {
        #[doc = "RO, recent reset flag."]
        #[must_use]
        #[inline(always)]
        pub const fn reset_flag(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "RO, recent reset flag."]
        #[inline(always)]
        pub const fn set_reset_flag(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u8) & 0x07) << 0usize);
        }
    }
    impl Default for ResetStatus {
        #[inline(always)]
        fn default() -> ResetStatus {
            ResetStatus(0)
        }
    }
    impl core::fmt::Debug for ResetStatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ResetStatus")
                .field("reset_flag", &self.reset_flag())
                .finish()
        }
    }
    #[doc = "RWA, sleep clock off control byte 0, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpClkOff0(pub u8);
    impl SlpClkOff0 {
        #[doc = "RWA, close TMR0 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_tmr0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close TMR0 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_tmr0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RWA, close TMR1 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_tmr1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close TMR1 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_tmr1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "RWA, close TMR2 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_tmr2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close TMR2 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_tmr2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RWA, close TMR3 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_tmr3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close TMR3 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_tmr3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RWA, close UART0 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_uart0(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close UART0 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_uart0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RWA, close UART1 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_uart1(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close UART1 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_uart1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RWA, close UART2 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_uart2(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close UART2 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_uart2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RWA, close UART3 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_uart3(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close UART3 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_uart3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for SlpClkOff0 {
        #[inline(always)]
        fn default() -> SlpClkOff0 {
            SlpClkOff0(0)
        }
    }
    impl core::fmt::Debug for SlpClkOff0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SlpClkOff0")
                .field("slp_clk_tmr0", &self.slp_clk_tmr0())
                .field("slp_clk_tmr1", &self.slp_clk_tmr1())
                .field("slp_clk_tmr2", &self.slp_clk_tmr2())
                .field("slp_clk_tmr3", &self.slp_clk_tmr3())
                .field("slp_clk_uart0", &self.slp_clk_uart0())
                .field("slp_clk_uart1", &self.slp_clk_uart1())
                .field("slp_clk_uart2", &self.slp_clk_uart2())
                .field("slp_clk_uart3", &self.slp_clk_uart3())
                .finish()
        }
    }
    #[doc = "RWA, sleep clock off control byte 1, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpClkOff1(pub u8);
    impl SlpClkOff1 {
        #[doc = "RWA, close SPI0 clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_spi0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close SPI0 clock."]
        #[inline(always)]
        pub const fn set_slp_clk_spi0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RWA, close PWMx clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_pwmx(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close PWMx clock."]
        #[inline(always)]
        pub const fn set_slp_clk_pwmx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
        #[doc = "RWA, close USB clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_usb(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close USB clock."]
        #[inline(always)]
        pub const fn set_slp_clk_usb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RWA, close BLE clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_ble(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close BLE clock."]
        #[inline(always)]
        pub const fn set_slp_clk_ble(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for SlpClkOff1 {
        #[inline(always)]
        fn default() -> SlpClkOff1 {
            SlpClkOff1(0)
        }
    }
    impl core::fmt::Debug for SlpClkOff1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SlpClkOff1")
                .field("slp_clk_spi0", &self.slp_clk_spi0())
                .field("slp_clk_pwmx", &self.slp_clk_pwmx())
                .field("slp_clk_usb", &self.slp_clk_usb())
                .field("slp_clk_ble", &self.slp_clk_ble())
                .finish()
        }
    }
    #[doc = "RWA, peripherals power down control, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpPowerCtrl(pub u8);
    impl SlpPowerCtrl {
        #[doc = "RWA, close main SRAM clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_ramx(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close main SRAM clock."]
        #[inline(always)]
        pub const fn set_slp_clk_ramx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RWA, close retention 2KB SRAM clock."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_clk_ram2k(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, close retention 2KB SRAM clock."]
        #[inline(always)]
        pub const fn set_slp_clk_ram2k(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RWA, SRAM retention voltage selection."]
        #[must_use]
        #[inline(always)]
        pub const fn ram_ret_lv(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, SRAM retention voltage selection."]
        #[inline(always)]
        pub const fn set_ram_ret_lv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
    }
    impl Default for SlpPowerCtrl {
        #[inline(always)]
        fn default() -> SlpPowerCtrl {
            SlpPowerCtrl(0)
        }
    }
    impl core::fmt::Debug for SlpPowerCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SlpPowerCtrl")
                .field("slp_clk_ramx", &self.slp_clk_ramx())
                .field("slp_clk_ram2k", &self.slp_clk_ram2k())
                .field("ram_ret_lv", &self.ram_ret_lv())
                .finish()
        }
    }
    #[doc = "RWA, wake control, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SlpWakeCtrl(pub u8);
    impl SlpWakeCtrl {
        #[doc = "RWA, enable USB waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_usb_wake(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, enable USB waking."]
        #[inline(always)]
        pub const fn set_slp_usb_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "RWA, enable RTC waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_rtc_wake(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, enable RTC waking."]
        #[inline(always)]
        pub const fn set_slp_rtc_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u8) & 0x01) << 3usize);
        }
        #[doc = "RWA, enable GPIO waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_gpio_wake(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, enable GPIO waking."]
        #[inline(always)]
        pub const fn set_slp_gpio_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u8) & 0x01) << 4usize);
        }
        #[doc = "RWA, enable BAT waking."]
        #[must_use]
        #[inline(always)]
        pub const fn slp_bat_wake(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, enable BAT waking."]
        #[inline(always)]
        pub const fn set_slp_bat_wake(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u8) & 0x01) << 5usize);
        }
        #[doc = "RWA, event wakeup mode."]
        #[must_use]
        #[inline(always)]
        pub const fn wake_ev_mode(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, event wakeup mode."]
        #[inline(always)]
        pub const fn set_wake_ev_mode(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u8) & 0x01) << 6usize);
        }
        #[doc = "RWA, wakeup delay."]
        #[must_use]
        #[inline(always)]
        pub const fn wake_delay(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "RWA, wakeup delay."]
        #[inline(always)]
        pub const fn set_wake_delay(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u8) & 0x01) << 7usize);
        }
    }
    impl Default for SlpWakeCtrl {
        #[inline(always)]
        fn default() -> SlpWakeCtrl {
            SlpWakeCtrl(0)
        }
    }
    impl core::fmt::Debug for SlpWakeCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("SlpWakeCtrl")
                .field("slp_usb_wake", &self.slp_usb_wake())
                .field("slp_rtc_wake", &self.slp_rtc_wake())
                .field("slp_gpio_wake", &self.slp_gpio_wake())
                .field("slp_bat_wake", &self.slp_bat_wake())
                .field("wake_ev_mode", &self.wake_ev_mode())
                .field("wake_delay", &self.wake_delay())
                .finish()
        }
    }
    #[doc = "RWA, external 32KHz oscillator tune control, SAM."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Xt32kTune(pub u8);
    impl Xt32kTune {
        #[doc = "RWA, external 32KHz oscillator current tune: 00=75% current, 01=standard current, 10=150% current, 11=200% current."]
        #[must_use]
        #[inline(always)]
        pub const fn xt32k_i_tune(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "RWA, external 32KHz oscillator current tune: 00=75% current, 01=standard current, 10=150% current, 11=200% current."]
        #[inline(always)]
        pub const fn set_xt32k_i_tune(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u8) & 0x03) << 0usize);
        }
        #[doc = "RWA, external 32KHz oscillator load capacitor tune: Cap = RB_XT32K_C_LOAD + 12pF."]
        #[must_use]
        #[inline(always)]
        pub const fn xt32k_c_load(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "RWA, external 32KHz oscillator load capacitor tune: Cap = RB_XT32K_C_LOAD + 12pF."]
        #[inline(always)]
        pub const fn set_xt32k_c_load(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u8) & 0x0f) << 4usize);
        }
    }
    impl Default for Xt32kTune {
        #[inline(always)]
        fn default() -> Xt32kTune {
            Xt32kTune(0)
        }
    }
    impl core::fmt::Debug for Xt32kTune {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Xt32kTune")
                .field("xt32k_i_tune", &self.xt32k_i_tune())
                .field("xt32k_c_load", &self.xt32k_c_load())
                .finish()
        }
    }
}
