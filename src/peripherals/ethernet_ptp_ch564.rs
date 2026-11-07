#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Ethernet: Precision time protocol."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EthernetPtp {
    ptr: *mut u8,
}
unsafe impl Send for EthernetPtp {}
unsafe impl Sync for EthernetPtp {}
impl EthernetPtp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Ethernet PTP time stamp control register. (ETH_PTPTSCR)."]
    #[inline(always)]
    pub const fn ptptscr(self) -> crate::common::Reg<regs::Ptptscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Ethernet PTP subsecond increment. register."]
    #[inline(always)]
    pub const fn ptpssir(self) -> crate::common::Reg<regs::Ptpssir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Ethernet PTP time stamp high. register."]
    #[inline(always)]
    pub const fn ptptshr(self) -> crate::common::Reg<regs::Ptptshr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Ethernet PTP time stamp low register. (ETH_PTPTSLR)."]
    #[inline(always)]
    pub const fn ptptslr(self) -> crate::common::Reg<regs::Ptptslr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Ethernet PTP time stamp high update. register."]
    #[inline(always)]
    pub const fn ptptshur(self) -> crate::common::Reg<regs::Ptptshur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Ethernet PTP time stamp low update register. (ETH_PTPTSLUR)."]
    #[inline(always)]
    pub const fn ptptslur(self) -> crate::common::Reg<regs::Ptptslur, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Ethernet PTP time stamp addend. register."]
    #[inline(always)]
    pub const fn ptptsar(self) -> crate::common::Reg<regs::Ptptsar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Ethernet PTP target time high. register."]
    #[inline(always)]
    pub const fn ptptthr(self) -> crate::common::Reg<regs::Ptptthr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Ethernet PTP target time low. register."]
    #[inline(always)]
    pub const fn ptpttlr(self) -> crate::common::Reg<regs::Ptpttlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
}
pub mod regs {
    #[doc = "Ethernet PTP subsecond increment. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptpssir(pub u32);
    impl Ptpssir {
        #[doc = "System time subsecond. increment."]
        #[inline(always)]
        pub const fn stssi(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "System time subsecond. increment."]
        #[inline(always)]
        pub fn set_stssi(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ptpssir {
        #[inline(always)]
        fn default() -> Ptpssir {
            Ptpssir(0)
        }
    }
    #[doc = "Ethernet PTP time stamp addend. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptsar(pub u32);
    impl Ptptsar {
        #[doc = "Time stamp addend."]
        #[inline(always)]
        pub const fn tsa(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Time stamp addend."]
        #[inline(always)]
        pub fn set_tsa(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ptptsar {
        #[inline(always)]
        fn default() -> Ptptsar {
            Ptptsar(0)
        }
    }
    #[doc = "Ethernet PTP time stamp control register. (ETH_PTPTSCR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptscr(pub u32);
    impl Ptptscr {
        #[doc = "Time stamp enable."]
        #[inline(always)]
        pub const fn tse(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Time stamp enable."]
        #[inline(always)]
        pub fn set_tse(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Time stamp fine or coarse. update."]
        #[inline(always)]
        pub const fn tsfcu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Time stamp fine or coarse. update."]
        #[inline(always)]
        pub fn set_tsfcu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Time stamp system time. initialize."]
        #[inline(always)]
        pub const fn tssti(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Time stamp system time. initialize."]
        #[inline(always)]
        pub fn set_tssti(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Time stamp system time. update."]
        #[inline(always)]
        pub const fn tsstu(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Time stamp system time. update."]
        #[inline(always)]
        pub fn set_tsstu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Time stamp interrupt trigger. enable."]
        #[inline(always)]
        pub const fn tsite(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Time stamp interrupt trigger. enable."]
        #[inline(always)]
        pub fn set_tsite(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Time stamp addend register. update."]
        #[inline(always)]
        pub const fn tsaru(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Time stamp addend register. update."]
        #[inline(always)]
        pub fn set_tsaru(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Ptptscr {
        #[inline(always)]
        fn default() -> Ptptscr {
            Ptptscr(0)
        }
    }
    #[doc = "Ethernet PTP time stamp high. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptshr(pub u32);
    impl Ptptshr {
        #[doc = "PTP System time value."]
        #[inline(always)]
        pub const fn tshr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "PTP System time value."]
        #[inline(always)]
        pub fn set_tshr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ptptshr {
        #[inline(always)]
        fn default() -> Ptptshr {
            Ptptshr(0)
        }
    }
    #[doc = "Ethernet PTP time stamp high update. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptshur(pub u32);
    impl Ptptshur {
        #[doc = "Time stamp update second."]
        #[inline(always)]
        pub const fn tsus(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Time stamp update second."]
        #[inline(always)]
        pub fn set_tsus(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ptptshur {
        #[inline(always)]
        fn default() -> Ptptshur {
            Ptptshur(0)
        }
    }
    #[doc = "Ethernet PTP time stamp low register. (ETH_PTPTSLR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptslr(pub u32);
    impl Ptptslr {
        #[doc = "System time subseconds."]
        #[inline(always)]
        pub const fn stss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "System time subseconds."]
        #[inline(always)]
        pub fn set_stss(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
        #[doc = "System time positive or negative. sign."]
        #[inline(always)]
        pub const fn stpns(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "System time positive or negative. sign."]
        #[inline(always)]
        pub fn set_stpns(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Ptptslr {
        #[inline(always)]
        fn default() -> Ptptslr {
            Ptptslr(0)
        }
    }
    #[doc = "Ethernet PTP time stamp low update register. (ETH_PTPTSLUR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptslur(pub u32);
    impl Ptptslur {
        #[doc = "Time stamp update. subseconds."]
        #[inline(always)]
        pub const fn tsuss(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Time stamp update. subseconds."]
        #[inline(always)]
        pub fn set_tsuss(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ptptslur {
        #[inline(always)]
        fn default() -> Ptptslur {
            Ptptslur(0)
        }
    }
    #[doc = "Ethernet PTP target time high. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptptthr(pub u32);
    impl Ptptthr {
        #[doc = "Target time stamp high."]
        #[inline(always)]
        pub const fn ttsh(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Target time stamp high."]
        #[inline(always)]
        pub fn set_ttsh(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ptptthr {
        #[inline(always)]
        fn default() -> Ptptthr {
            Ptptthr(0)
        }
    }
    #[doc = "Ethernet PTP target time low. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ptpttlr(pub u32);
    impl Ptpttlr {
        #[doc = "Target time stamp low."]
        #[inline(always)]
        pub const fn ttsl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Target time stamp low."]
        #[inline(always)]
        pub fn set_ttsl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Ptpttlr {
        #[inline(always)]
        fn default() -> Ptpttlr {
            Ptpttlr(0)
        }
    }
}
