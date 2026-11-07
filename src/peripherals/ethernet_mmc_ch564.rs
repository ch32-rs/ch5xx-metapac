#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Ethernet: MAC management counters."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EthernetMmc {
    ptr: *mut u8,
}
unsafe impl Send for EthernetMmc {}
unsafe impl Sync for EthernetMmc {}
impl EthernetMmc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Ethernet MMC control register. (ETH_MMCCR)."]
    #[inline(always)]
    pub const fn mmccr(self) -> crate::common::Reg<regs::Mmccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Ethernet MMC receive interrupt register. (ETH_MMCRIR)."]
    #[inline(always)]
    pub const fn mmcrir(self) -> crate::common::Reg<regs::Mmcrir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Ethernet MMC transmit interrupt register. (ETH_MMCTIR)."]
    #[inline(always)]
    pub const fn mmctir(self) -> crate::common::Reg<regs::Mmctir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Ethernet MMC receive interrupt mask register. (ETH_MMCRIMR)."]
    #[inline(always)]
    pub const fn mmcrimr(self) -> crate::common::Reg<regs::Mmcrimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Ethernet MMC transmit interrupt mask. register (ETH_MMCTIMR)."]
    #[inline(always)]
    pub const fn mmctimr(self) -> crate::common::Reg<regs::Mmctimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Ethernet MMC transmitted good frames after a. single collision counter."]
    #[inline(always)]
    pub const fn mmctgfsccr(self) -> crate::common::Reg<regs::Mmctgfsccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Ethernet MMC transmitted good frames after. more than a single collision."]
    #[inline(always)]
    pub const fn mmctgfmsccr(self) -> crate::common::Reg<regs::Mmctgfmsccr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Ethernet MMC transmitted good frames counter. register."]
    #[inline(always)]
    pub const fn mmctgfcr(self) -> crate::common::Reg<regs::Mmctgfcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "Ethernet MMC received frames with CRC error. counter register."]
    #[inline(always)]
    pub const fn mmcrfcecr(self) -> crate::common::Reg<regs::Mmcrfcecr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "Ethernet MMC received frames with alignment. error counter register."]
    #[inline(always)]
    pub const fn mmcrfaecr(self) -> crate::common::Reg<regs::Mmcrfaecr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "MMC receive the frame count register."]
    #[inline(always)]
    pub const fn mmcrafcr(self) -> crate::common::Reg<regs::Mmcrafcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "Ethernet state machine register."]
    #[inline(always)]
    pub const fn state(self) -> crate::common::Reg<regs::State, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "MMC received good unicast frames counter. register."]
    #[inline(always)]
    pub const fn mmcrgufcr(self) -> crate::common::Reg<regs::Mmcrgufcr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
}
pub mod regs {
    #[doc = "Ethernet MMC control register. (ETH_MMCCR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmccr(pub u32);
    impl Mmccr {
        #[doc = "Counter reset."]
        #[inline(always)]
        pub const fn cr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Counter reset."]
        #[inline(always)]
        pub fn set_cr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Counter stop rollover."]
        #[inline(always)]
        pub const fn csr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Counter stop rollover."]
        #[inline(always)]
        pub fn set_csr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Reset on read."]
        #[inline(always)]
        pub const fn ror(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Reset on read."]
        #[inline(always)]
        pub fn set_ror(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "MMC counter freeze."]
        #[inline(always)]
        pub const fn mcf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "MMC counter freeze."]
        #[inline(always)]
        pub fn set_mcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Mmccr {
        #[inline(always)]
        fn default() -> Mmccr {
            Mmccr(0)
        }
    }
    #[doc = "MMC receive the frame count register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmcrafcr(pub u32);
    impl Mmcrafcr {
        #[doc = "The number of all Ethernet MMC frame received."]
        #[inline(always)]
        pub const fn rafcr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "The number of all Ethernet MMC frame received."]
        #[inline(always)]
        pub fn set_rafcr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmcrafcr {
        #[inline(always)]
        fn default() -> Mmcrafcr {
            Mmcrafcr(0)
        }
    }
    #[doc = "Ethernet MMC received frames with alignment. error counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmcrfaecr(pub u32);
    impl Mmcrfaecr {
        #[doc = "Received frames with alignment error. counter."]
        #[inline(always)]
        pub const fn rfaec(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Received frames with alignment error. counter."]
        #[inline(always)]
        pub fn set_rfaec(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmcrfaecr {
        #[inline(always)]
        fn default() -> Mmcrfaecr {
            Mmcrfaecr(0)
        }
    }
    #[doc = "Ethernet MMC received frames with CRC error. counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmcrfcecr(pub u32);
    impl Mmcrfcecr {
        #[doc = "Received frames with CRC error. counter."]
        #[inline(always)]
        pub const fn rfcfc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Received frames with CRC error. counter."]
        #[inline(always)]
        pub fn set_rfcfc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmcrfcecr {
        #[inline(always)]
        fn default() -> Mmcrfcecr {
            Mmcrfcecr(0)
        }
    }
    #[doc = "MMC received good unicast frames counter. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmcrgufcr(pub u32);
    impl Mmcrgufcr {
        #[doc = "Received good unicast frames. counter."]
        #[inline(always)]
        pub const fn rgufc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Received good unicast frames. counter."]
        #[inline(always)]
        pub fn set_rgufc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmcrgufcr {
        #[inline(always)]
        fn default() -> Mmcrgufcr {
            Mmcrgufcr(0)
        }
    }
    #[doc = "Ethernet MMC receive interrupt mask register. (ETH_MMCRIMR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmcrimr(pub u32);
    impl Mmcrimr {
        #[doc = "Received frame CRC error. mask."]
        #[inline(always)]
        pub const fn rfcem(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Received frame CRC error. mask."]
        #[inline(always)]
        pub fn set_rfcem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Received frames alignment error. mask."]
        #[inline(always)]
        pub const fn rfaem(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Received frames alignment error. mask."]
        #[inline(always)]
        pub fn set_rfaem(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Received good unicast frames. mask."]
        #[inline(always)]
        pub const fn rgufm(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Received good unicast frames. mask."]
        #[inline(always)]
        pub fn set_rgufm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Mmcrimr {
        #[inline(always)]
        fn default() -> Mmcrimr {
            Mmcrimr(0)
        }
    }
    #[doc = "Ethernet MMC receive interrupt register. (ETH_MMCRIR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmcrir(pub u32);
    impl Mmcrir {
        #[doc = "Received frames CRC error. status."]
        #[inline(always)]
        pub const fn rfces(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Received frames CRC error. status."]
        #[inline(always)]
        pub fn set_rfces(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Received Good Unicast Frames. Status."]
        #[inline(always)]
        pub const fn rgufs(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Received Good Unicast Frames. Status."]
        #[inline(always)]
        pub fn set_rgufs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Mmcrir {
        #[inline(always)]
        fn default() -> Mmcrir {
            Mmcrir(0)
        }
    }
    #[doc = "Ethernet MMC transmitted good frames counter. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmctgfcr(pub u32);
    impl Mmctgfcr {
        #[doc = "Transmitted good frames. counter."]
        #[inline(always)]
        pub const fn tgfc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Transmitted good frames. counter."]
        #[inline(always)]
        pub fn set_tgfc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmctgfcr {
        #[inline(always)]
        fn default() -> Mmctgfcr {
            Mmctgfcr(0)
        }
    }
    #[doc = "Ethernet MMC transmitted good frames after. more than a single collision."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmctgfmsccr(pub u32);
    impl Mmctgfmsccr {
        #[doc = "Transmitted good frames after more than. a single collision counter."]
        #[inline(always)]
        pub const fn tgfmscc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Transmitted good frames after more than. a single collision counter."]
        #[inline(always)]
        pub fn set_tgfmscc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmctgfmsccr {
        #[inline(always)]
        fn default() -> Mmctgfmsccr {
            Mmctgfmsccr(0)
        }
    }
    #[doc = "Ethernet MMC transmitted good frames after a. single collision counter."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmctgfsccr(pub u32);
    impl Mmctgfsccr {
        #[doc = "Transmitted good frames after a single. collision counter."]
        #[inline(always)]
        pub const fn tgfscc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Transmitted good frames after a single. collision counter."]
        #[inline(always)]
        pub fn set_tgfscc(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Mmctgfsccr {
        #[inline(always)]
        fn default() -> Mmctgfsccr {
            Mmctgfsccr(0)
        }
    }
    #[doc = "Ethernet MMC transmit interrupt mask. register (ETH_MMCTIMR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmctimr(pub u32);
    impl Mmctimr {
        #[doc = "Transmitted good frames single collision. mask."]
        #[inline(always)]
        pub const fn tgfscm(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitted good frames single collision. mask."]
        #[inline(always)]
        pub fn set_tgfscm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Transmitted good frames more single. collision mask."]
        #[inline(always)]
        pub const fn tgfmscm(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitted good frames more single. collision mask."]
        #[inline(always)]
        pub fn set_tgfmscm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Transmitted good frames. mask."]
        #[inline(always)]
        pub const fn tgfm(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitted good frames. mask."]
        #[inline(always)]
        pub fn set_tgfm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Mmctimr {
        #[inline(always)]
        fn default() -> Mmctimr {
            Mmctimr(0)
        }
    }
    #[doc = "Ethernet MMC transmit interrupt register. (ETH_MMCTIR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mmctir(pub u32);
    impl Mmctir {
        #[doc = "Transmitted good frames. status."]
        #[inline(always)]
        pub const fn tgfs(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitted good frames. status."]
        #[inline(always)]
        pub fn set_tgfs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Mmctir {
        #[inline(always)]
        fn default() -> Mmctir {
            Mmctir(0)
        }
    }
    #[doc = "Ethernet state machine register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct State(pub u32);
    impl State {
        #[doc = "Ethernet receiving state machine."]
        #[inline(always)]
        pub const fn etx_state(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Ethernet receiving state machine."]
        #[inline(always)]
        pub fn set_etx_state(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Ethernet sending state machine."]
        #[inline(always)]
        pub const fn erx_state(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Ethernet sending state machine."]
        #[inline(always)]
        pub fn set_erx_state(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
    }
    impl Default for State {
        #[inline(always)]
        fn default() -> State {
            State(0)
        }
    }
}
