#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Ethernet: media access control."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EthernetMac {
    ptr: *mut u8,
}
unsafe impl Send for EthernetMac {}
unsafe impl Sync for EthernetMac {}
impl EthernetMac {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Ethernet MAC configuration register. (ETH_MACCR)."]
    #[inline(always)]
    pub const fn maccr(self) -> crate::common::Reg<regs::Maccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Ethernet MAC frame filter register. (ETH_MACCFFR)."]
    #[inline(always)]
    pub const fn macffr(self) -> crate::common::Reg<regs::Macffr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "Ethernet MAC hash table high. register."]
    #[inline(always)]
    pub const fn machthr(self) -> crate::common::Reg<regs::Machthr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Ethernet MAC hash table low. register."]
    #[inline(always)]
    pub const fn machtlr(self) -> crate::common::Reg<regs::Machtlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Ethernet MAC MII address register. (ETH_MACMIIAR)."]
    #[inline(always)]
    pub const fn macmiiar(self) -> crate::common::Reg<regs::Macmiiar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "Ethernet MAC MII data register. (ETH_MACMIIDR)."]
    #[inline(always)]
    pub const fn macmiidr(self) -> crate::common::Reg<regs::Macmiidr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "Ethernet MAC flow control register. (ETH_MACFCR)."]
    #[inline(always)]
    pub const fn macfcr(self) -> crate::common::Reg<regs::Macfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "Ethernet MAC VLAN tag register. (ETH_MACVLAN)."]
    #[inline(always)]
    pub const fn macvlan(self) -> crate::common::Reg<regs::Macvlan, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Ethernet MAC remote wakeup frame filter. register (ETH_MACRWUFFR)."]
    #[inline(always)]
    pub const fn macrwuffr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x28usize) as _) }
    }
    #[doc = "Ethernet MAC PMT control and status register. (ETH_MACPMTCSR)."]
    #[inline(always)]
    pub const fn macpmtcsr(self) -> crate::common::Reg<regs::Macpmtcsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x2cusize) as _) }
    }
    #[doc = "Ethernet MAC interrupt status register. (ETH_MACSR)."]
    #[inline(always)]
    pub const fn macsr(self) -> crate::common::Reg<regs::Macsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "Ethernet MAC interrupt mask register. (ETH_MACIMR)."]
    #[inline(always)]
    pub const fn macimr(self) -> crate::common::Reg<regs::Macimr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
    #[doc = "Ethernet MAC address 0 high register. (ETH_MACA0HR)."]
    #[inline(always)]
    pub const fn maca0hr(self) -> crate::common::Reg<regs::Maca0hr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Ethernet MAC address 0 low. register."]
    #[inline(always)]
    pub const fn maca0lr(self) -> crate::common::Reg<regs::Maca0lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "Ethernet MAC address 1 high register. (ETH_MACA1HR)."]
    #[inline(always)]
    pub const fn maca1hr(self) -> crate::common::Reg<regs::Maca1hr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Ethernet MAC address1 low. register."]
    #[inline(always)]
    pub const fn maca1lr(self) -> crate::common::Reg<regs::Maca1lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "Ethernet MAC address 2 high register. (ETH_MACA2HR)."]
    #[inline(always)]
    pub const fn maca2hr(self) -> crate::common::Reg<regs::Maca2hr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Ethernet MAC address 2 low. register."]
    #[inline(always)]
    pub const fn maca2lr(self) -> crate::common::Reg<regs::Maca2lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "Ethernet MAC address 3 high register. (ETH_MACA3HR)."]
    #[inline(always)]
    pub const fn maca3hr(self) -> crate::common::Reg<regs::Maca3hr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "Ethernet MAC address 3 low. register."]
    #[inline(always)]
    pub const fn maca3lr(self) -> crate::common::Reg<regs::Maca3lr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "PHY configure register."]
    #[inline(always)]
    pub const fn phy_cr(self) -> crate::common::Reg<regs::PhyCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "CHKSUM configure register."]
    #[inline(always)]
    pub const fn chksum_cr(self) -> crate::common::Reg<regs::ChksumCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "IP packet register."]
    #[inline(always)]
    pub const fn ip_pdr(self) -> crate::common::Reg<regs::IpPdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "IP packet register."]
    #[inline(always)]
    pub const fn chksum_hr(self) -> crate::common::Reg<regs::ChksumHr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "IP data register."]
    #[inline(always)]
    pub const fn chksum_pr(self) -> crate::common::Reg<regs::ChksumPr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
}
pub mod regs {
    #[doc = "CHKSUM configure register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChksumCr(pub u32);
    impl ChksumCr {
        #[doc = "Participate in the ip header checksum to. calculate byte control bits."]
        #[must_use]
        #[inline(always)]
        pub const fn byte_en1(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Participate in the ip header checksum to. calculate byte control bits."]
        #[inline(always)]
        pub const fn set_byte_en1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Participate in the ip header checksum to. calculate byte control bits."]
        #[must_use]
        #[inline(always)]
        pub const fn byte_en2(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Participate in the ip header checksum to. calculate byte control bits."]
        #[inline(always)]
        pub const fn set_byte_en2(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "the software sets 1 at the beginning of frame calculation. and write 0 at the end of frame calculation."]
        #[must_use]
        #[inline(always)]
        pub const fn chksum_en(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "the software sets 1 at the beginning of frame calculation. and write 0 at the end of frame calculation."]
        #[inline(always)]
        pub const fn set_chksum_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for ChksumCr {
        #[inline(always)]
        fn default() -> ChksumCr {
            ChksumCr(0)
        }
    }
    impl core::fmt::Debug for ChksumCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ChksumCr")
                .field("byte_en1", &self.byte_en1())
                .field("byte_en2", &self.byte_en2())
                .field("chksum_en", &self.chksum_en())
                .finish()
        }
    }
    #[doc = "IP packet register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChksumHr(pub u32);
    impl ChksumHr {
        #[doc = "IP header checksum."]
        #[must_use]
        #[inline(always)]
        pub const fn chksum_head_result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "IP header checksum."]
        #[inline(always)]
        pub const fn set_chksum_head_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for ChksumHr {
        #[inline(always)]
        fn default() -> ChksumHr {
            ChksumHr(0)
        }
    }
    impl core::fmt::Debug for ChksumHr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ChksumHr")
                .field("chksum_head_result", &self.chksum_head_result())
                .finish()
        }
    }
    #[doc = "IP data register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChksumPr(pub u32);
    impl ChksumPr {
        #[doc = "IP header checksum."]
        #[must_use]
        #[inline(always)]
        pub const fn chksum_head_result(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "IP header checksum."]
        #[inline(always)]
        pub const fn set_chksum_head_result(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for ChksumPr {
        #[inline(always)]
        fn default() -> ChksumPr {
            ChksumPr(0)
        }
    }
    impl core::fmt::Debug for ChksumPr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ChksumPr")
                .field("chksum_head_result", &self.chksum_head_result())
                .finish()
        }
    }
    #[doc = "IP packet register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IpPdr(pub u32);
    impl IpPdr {
        #[doc = "IP data written by software,need to be used with. ETH_CHKSUM_CR."]
        #[must_use]
        #[inline(always)]
        pub const fn ip_payload(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IP data written by software,need to be used with. ETH_CHKSUM_CR."]
        #[inline(always)]
        pub const fn set_ip_payload(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for IpPdr {
        #[inline(always)]
        fn default() -> IpPdr {
            IpPdr(0)
        }
    }
    impl core::fmt::Debug for IpPdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IpPdr")
                .field("ip_payload", &self.ip_payload())
                .finish()
        }
    }
    #[doc = "Ethernet MAC address 0 high register. (ETH_MACA0HR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca0hr(pub u32);
    impl Maca0hr {
        #[doc = "MAC address0 high."]
        #[must_use]
        #[inline(always)]
        pub const fn maca0h(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC address0 high."]
        #[inline(always)]
        pub const fn set_maca0h(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Always 1."]
        #[must_use]
        #[inline(always)]
        pub const fn mo(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Always 1."]
        #[inline(always)]
        pub const fn set_mo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Maca0hr {
        #[inline(always)]
        fn default() -> Maca0hr {
            Maca0hr(0)
        }
    }
    impl core::fmt::Debug for Maca0hr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca0hr")
                .field("maca0h", &self.maca0h())
                .field("mo", &self.mo())
                .finish()
        }
    }
    #[doc = "Ethernet MAC address 0 low. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca0lr(pub u32);
    impl Maca0lr {
        #[doc = "MAC address0 low."]
        #[must_use]
        #[inline(always)]
        pub const fn maca0l(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC address0 low."]
        #[inline(always)]
        pub const fn set_maca0l(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Maca0lr {
        #[inline(always)]
        fn default() -> Maca0lr {
            Maca0lr(0)
        }
    }
    impl core::fmt::Debug for Maca0lr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca0lr")
                .field("maca0l", &self.maca0l())
                .finish()
        }
    }
    #[doc = "Ethernet MAC address 1 high register. (ETH_MACA1HR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca1hr(pub u32);
    impl Maca1hr {
        #[doc = "MAC address1 high."]
        #[must_use]
        #[inline(always)]
        pub const fn maca1h(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC address1 high."]
        #[inline(always)]
        pub const fn set_maca1h(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Mask byte control."]
        #[must_use]
        #[inline(always)]
        pub const fn mbc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Mask byte control."]
        #[inline(always)]
        pub const fn set_mbc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Source address."]
        #[must_use]
        #[inline(always)]
        pub const fn sa(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Source address."]
        #[inline(always)]
        pub const fn set_sa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Address enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Address enable."]
        #[inline(always)]
        pub const fn set_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Maca1hr {
        #[inline(always)]
        fn default() -> Maca1hr {
            Maca1hr(0)
        }
    }
    impl core::fmt::Debug for Maca1hr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca1hr")
                .field("maca1h", &self.maca1h())
                .field("mbc", &self.mbc())
                .field("sa", &self.sa())
                .field("ae", &self.ae())
                .finish()
        }
    }
    #[doc = "Ethernet MAC address1 low. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca1lr(pub u32);
    impl Maca1lr {
        #[doc = "MAC address1 low."]
        #[must_use]
        #[inline(always)]
        pub const fn maca1l(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC address1 low."]
        #[inline(always)]
        pub const fn set_maca1l(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Maca1lr {
        #[inline(always)]
        fn default() -> Maca1lr {
            Maca1lr(0)
        }
    }
    impl core::fmt::Debug for Maca1lr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca1lr")
                .field("maca1l", &self.maca1l())
                .finish()
        }
    }
    #[doc = "Ethernet MAC address 2 high register. (ETH_MACA2HR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca2hr(pub u32);
    impl Maca2hr {
        #[doc = "Ethernet MAC address 2 high. register."]
        #[must_use]
        #[inline(always)]
        pub const fn maca2h(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Ethernet MAC address 2 high. register."]
        #[inline(always)]
        pub const fn set_maca2h(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Mask byte control."]
        #[must_use]
        #[inline(always)]
        pub const fn mbc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Mask byte control."]
        #[inline(always)]
        pub const fn set_mbc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Source address."]
        #[must_use]
        #[inline(always)]
        pub const fn sa(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Source address."]
        #[inline(always)]
        pub const fn set_sa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Address enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Address enable."]
        #[inline(always)]
        pub const fn set_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Maca2hr {
        #[inline(always)]
        fn default() -> Maca2hr {
            Maca2hr(0)
        }
    }
    impl core::fmt::Debug for Maca2hr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca2hr")
                .field("maca2h", &self.maca2h())
                .field("mbc", &self.mbc())
                .field("sa", &self.sa())
                .field("ae", &self.ae())
                .finish()
        }
    }
    #[doc = "Ethernet MAC address 2 low. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca2lr(pub u32);
    impl Maca2lr {
        #[doc = "MAC address2 low."]
        #[must_use]
        #[inline(always)]
        pub const fn maca2l(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "MAC address2 low."]
        #[inline(always)]
        pub const fn set_maca2l(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 0usize)) | (((val as u32) & 0x7fff_ffff) << 0usize);
        }
    }
    impl Default for Maca2lr {
        #[inline(always)]
        fn default() -> Maca2lr {
            Maca2lr(0)
        }
    }
    impl core::fmt::Debug for Maca2lr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca2lr")
                .field("maca2l", &self.maca2l())
                .finish()
        }
    }
    #[doc = "Ethernet MAC address 3 high register. (ETH_MACA3HR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca3hr(pub u32);
    impl Maca3hr {
        #[doc = "MAC address3 high."]
        #[must_use]
        #[inline(always)]
        pub const fn maca3h(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MAC address3 high."]
        #[inline(always)]
        pub const fn set_maca3h(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Mask byte control."]
        #[must_use]
        #[inline(always)]
        pub const fn mbc(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0x3f;
            val as u8
        }
        #[doc = "Mask byte control."]
        #[inline(always)]
        pub const fn set_mbc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
        }
        #[doc = "Source address."]
        #[must_use]
        #[inline(always)]
        pub const fn sa(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Source address."]
        #[inline(always)]
        pub const fn set_sa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Address enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ae(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Address enable."]
        #[inline(always)]
        pub const fn set_ae(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Maca3hr {
        #[inline(always)]
        fn default() -> Maca3hr {
            Maca3hr(0)
        }
    }
    impl core::fmt::Debug for Maca3hr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca3hr")
                .field("maca3h", &self.maca3h())
                .field("mbc", &self.mbc())
                .field("sa", &self.sa())
                .field("ae", &self.ae())
                .finish()
        }
    }
    #[doc = "Ethernet MAC address 3 low. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maca3lr(pub u32);
    impl Maca3lr {
        #[doc = "MAC address3 low."]
        #[must_use]
        #[inline(always)]
        pub const fn mbca3l(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "MAC address3 low."]
        #[inline(always)]
        pub const fn set_mbca3l(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Maca3lr {
        #[inline(always)]
        fn default() -> Maca3lr {
            Maca3lr(0)
        }
    }
    impl core::fmt::Debug for Maca3lr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maca3lr")
                .field("mbca3l", &self.mbca3l())
                .finish()
        }
    }
    #[doc = "Ethernet MAC configuration register. (ETH_MACCR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Maccr(pub u32);
    impl Maccr {
        #[doc = "Receiver enable."]
        #[must_use]
        #[inline(always)]
        pub const fn re(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver enable."]
        #[inline(always)]
        pub const fn set_re(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Transmitter enable."]
        #[must_use]
        #[inline(always)]
        pub const fn te(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Transmitter enable."]
        #[inline(always)]
        pub const fn set_te(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Automatic pad/CRC. stripping."]
        #[must_use]
        #[inline(always)]
        pub const fn apcs(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Automatic pad/CRC. stripping."]
        #[inline(always)]
        pub const fn set_apcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Retry disable."]
        #[must_use]
        #[inline(always)]
        pub const fn rd(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Retry disable."]
        #[inline(always)]
        pub const fn set_rd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "IPv4 checksum offload."]
        #[must_use]
        #[inline(always)]
        pub const fn ipco(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "IPv4 checksum offload."]
        #[inline(always)]
        pub const fn set_ipco(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Duplex mode."]
        #[must_use]
        #[inline(always)]
        pub const fn dm(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Duplex mode."]
        #[inline(always)]
        pub const fn set_dm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Fast Ethernet speed."]
        #[must_use]
        #[inline(always)]
        pub const fn fes(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Fast Ethernet speed."]
        #[inline(always)]
        pub const fn set_fes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Interframe gap."]
        #[must_use]
        #[inline(always)]
        pub const fn ifg(&self) -> u8 {
            let val = (self.0 >> 17usize) & 0x07;
            val as u8
        }
        #[doc = "Interframe gap."]
        #[inline(always)]
        pub const fn set_ifg(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 17usize)) | (((val as u32) & 0x07) << 17usize);
        }
        #[doc = "Jabber disable."]
        #[must_use]
        #[inline(always)]
        pub const fn jd(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Jabber disable."]
        #[inline(always)]
        pub const fn set_jd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Watchdog disable."]
        #[must_use]
        #[inline(always)]
        pub const fn wd(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Watchdog disable."]
        #[inline(always)]
        pub const fn set_wd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Maccr {
        #[inline(always)]
        fn default() -> Maccr {
            Maccr(0)
        }
    }
    impl core::fmt::Debug for Maccr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Maccr")
                .field("re", &self.re())
                .field("te", &self.te())
                .field("apcs", &self.apcs())
                .field("rd", &self.rd())
                .field("ipco", &self.ipco())
                .field("dm", &self.dm())
                .field("fes", &self.fes())
                .field("ifg", &self.ifg())
                .field("jd", &self.jd())
                .field("wd", &self.wd())
                .finish()
        }
    }
    #[doc = "Ethernet MAC flow control register. (ETH_MACFCR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macfcr(pub u32);
    impl Macfcr {
        #[doc = "Flow control busy."]
        #[must_use]
        #[inline(always)]
        pub const fn fcb(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Flow control busy."]
        #[inline(always)]
        pub const fn set_fcb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Transmit flow control. enable."]
        #[must_use]
        #[inline(always)]
        pub const fn tfce(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Transmit flow control. enable."]
        #[inline(always)]
        pub const fn set_tfce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Receive flow control. enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rfce(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receive flow control. enable."]
        #[inline(always)]
        pub const fn set_rfce(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Unicast pause frame detect."]
        #[must_use]
        #[inline(always)]
        pub const fn upfd(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Unicast pause frame detect."]
        #[inline(always)]
        pub const fn set_upfd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Pass control frames."]
        #[must_use]
        #[inline(always)]
        pub const fn pt(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Pass control frames."]
        #[inline(always)]
        pub const fn set_pt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Macfcr {
        #[inline(always)]
        fn default() -> Macfcr {
            Macfcr(0)
        }
    }
    impl core::fmt::Debug for Macfcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macfcr")
                .field("fcb", &self.fcb())
                .field("tfce", &self.tfce())
                .field("rfce", &self.rfce())
                .field("upfd", &self.upfd())
                .field("pt", &self.pt())
                .finish()
        }
    }
    #[doc = "Ethernet MAC frame filter register. (ETH_MACCFFR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macffr(pub u32);
    impl Macffr {
        #[doc = "Promiscuous mode."]
        #[must_use]
        #[inline(always)]
        pub const fn pm(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Promiscuous mode."]
        #[inline(always)]
        pub const fn set_pm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Hash unicast."]
        #[must_use]
        #[inline(always)]
        pub const fn hu(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Hash unicast."]
        #[inline(always)]
        pub const fn set_hu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Hash multicast."]
        #[must_use]
        #[inline(always)]
        pub const fn hm(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Hash multicast."]
        #[inline(always)]
        pub const fn set_hm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Destination address inverse. filtering."]
        #[must_use]
        #[inline(always)]
        pub const fn daif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Destination address inverse. filtering."]
        #[inline(always)]
        pub const fn set_daif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Pass all multicast."]
        #[must_use]
        #[inline(always)]
        pub const fn pam(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Pass all multicast."]
        #[inline(always)]
        pub const fn set_pam(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Broadcast frames disable."]
        #[must_use]
        #[inline(always)]
        pub const fn bfd(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Broadcast frames disable."]
        #[inline(always)]
        pub const fn set_bfd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Pass control frames."]
        #[must_use]
        #[inline(always)]
        pub const fn pcf(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Pass control frames."]
        #[inline(always)]
        pub const fn set_pcf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Source address inverse. filtering."]
        #[must_use]
        #[inline(always)]
        pub const fn saif(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Source address inverse. filtering."]
        #[inline(always)]
        pub const fn set_saif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Source address filter."]
        #[must_use]
        #[inline(always)]
        pub const fn saf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Source address filter."]
        #[inline(always)]
        pub const fn set_saf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Hash or perfect filter."]
        #[must_use]
        #[inline(always)]
        pub const fn hpf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Hash or perfect filter."]
        #[inline(always)]
        pub const fn set_hpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Receive all."]
        #[must_use]
        #[inline(always)]
        pub const fn ra(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Receive all."]
        #[inline(always)]
        pub const fn set_ra(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macffr {
        #[inline(always)]
        fn default() -> Macffr {
            Macffr(0)
        }
    }
    impl core::fmt::Debug for Macffr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macffr")
                .field("pm", &self.pm())
                .field("hu", &self.hu())
                .field("hm", &self.hm())
                .field("daif", &self.daif())
                .field("pam", &self.pam())
                .field("bfd", &self.bfd())
                .field("pcf", &self.pcf())
                .field("saif", &self.saif())
                .field("saf", &self.saf())
                .field("hpf", &self.hpf())
                .field("ra", &self.ra())
                .finish()
        }
    }
    #[doc = "Ethernet MAC hash table high. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Machthr(pub u32);
    impl Machthr {
        #[doc = "Hash table high."]
        #[must_use]
        #[inline(always)]
        pub const fn hth(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Hash table high."]
        #[inline(always)]
        pub const fn set_hth(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Machthr {
        #[inline(always)]
        fn default() -> Machthr {
            Machthr(0)
        }
    }
    impl core::fmt::Debug for Machthr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Machthr").field("hth", &self.hth()).finish()
        }
    }
    #[doc = "Ethernet MAC hash table low. register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Machtlr(pub u32);
    impl Machtlr {
        #[doc = "Hash table low."]
        #[must_use]
        #[inline(always)]
        pub const fn htl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Hash table low."]
        #[inline(always)]
        pub const fn set_htl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Machtlr {
        #[inline(always)]
        fn default() -> Machtlr {
            Machtlr(0)
        }
    }
    impl core::fmt::Debug for Machtlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Machtlr").field("htl", &self.htl()).finish()
        }
    }
    #[doc = "Ethernet MAC interrupt mask register. (ETH_MACIMR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macimr(pub u32);
    impl Macimr {
        #[doc = "PMT interrupt mask."]
        #[must_use]
        #[inline(always)]
        pub const fn pmtim(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PMT interrupt mask."]
        #[inline(always)]
        pub const fn set_pmtim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Time stamp trigger interrupt. mask."]
        #[must_use]
        #[inline(always)]
        pub const fn tstim(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Time stamp trigger interrupt. mask."]
        #[inline(always)]
        pub const fn set_tstim(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Macimr {
        #[inline(always)]
        fn default() -> Macimr {
            Macimr(0)
        }
    }
    impl core::fmt::Debug for Macimr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macimr")
                .field("pmtim", &self.pmtim())
                .field("tstim", &self.tstim())
                .finish()
        }
    }
    #[doc = "Ethernet MAC MII address register. (ETH_MACMIIAR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macmiiar(pub u32);
    impl Macmiiar {
        #[doc = "MII busy."]
        #[must_use]
        #[inline(always)]
        pub const fn mb(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "MII busy."]
        #[inline(always)]
        pub const fn set_mb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "MII write."]
        #[must_use]
        #[inline(always)]
        pub const fn mw(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "MII write."]
        #[inline(always)]
        pub const fn set_mw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clock range."]
        #[must_use]
        #[inline(always)]
        pub const fn cr(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x07;
            val as u8
        }
        #[doc = "Clock range."]
        #[inline(always)]
        pub const fn set_cr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 2usize)) | (((val as u32) & 0x07) << 2usize);
        }
        #[doc = "MII register."]
        #[must_use]
        #[inline(always)]
        pub const fn mr(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x1f;
            val as u8
        }
        #[doc = "MII register."]
        #[inline(always)]
        pub const fn set_mr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 6usize)) | (((val as u32) & 0x1f) << 6usize);
        }
        #[doc = "PHY address."]
        #[must_use]
        #[inline(always)]
        pub const fn pa(&self) -> u8 {
            let val = (self.0 >> 11usize) & 0x1f;
            val as u8
        }
        #[doc = "PHY address."]
        #[inline(always)]
        pub const fn set_pa(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
        }
    }
    impl Default for Macmiiar {
        #[inline(always)]
        fn default() -> Macmiiar {
            Macmiiar(0)
        }
    }
    impl core::fmt::Debug for Macmiiar {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macmiiar")
                .field("mb", &self.mb())
                .field("mw", &self.mw())
                .field("cr", &self.cr())
                .field("mr", &self.mr())
                .field("pa", &self.pa())
                .finish()
        }
    }
    #[doc = "Ethernet MAC MII data register. (ETH_MACMIIDR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macmiidr(pub u32);
    impl Macmiidr {
        #[doc = "MII data."]
        #[must_use]
        #[inline(always)]
        pub const fn md(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "MII data."]
        #[inline(always)]
        pub const fn set_md(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Macmiidr {
        #[inline(always)]
        fn default() -> Macmiidr {
            Macmiidr(0)
        }
    }
    impl core::fmt::Debug for Macmiidr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macmiidr").field("md", &self.md()).finish()
        }
    }
    #[doc = "Ethernet MAC PMT control and status register. (ETH_MACPMTCSR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macpmtcsr(pub u32);
    impl Macpmtcsr {
        #[doc = "Power down."]
        #[must_use]
        #[inline(always)]
        pub const fn pd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Power down."]
        #[inline(always)]
        pub const fn set_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Magic Packet enable."]
        #[must_use]
        #[inline(always)]
        pub const fn mpe(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Magic Packet enable."]
        #[inline(always)]
        pub const fn set_mpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Wakeup frame enable."]
        #[must_use]
        #[inline(always)]
        pub const fn wfe(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup frame enable."]
        #[inline(always)]
        pub const fn set_wfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "PMEB of PHY enable."]
        #[must_use]
        #[inline(always)]
        pub const fn phye(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PMEB of PHY enable."]
        #[inline(always)]
        pub const fn set_phye(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Magic packet received."]
        #[must_use]
        #[inline(always)]
        pub const fn mpr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Magic packet received."]
        #[inline(always)]
        pub const fn set_mpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Wakeup frame received."]
        #[must_use]
        #[inline(always)]
        pub const fn wfr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup frame received."]
        #[inline(always)]
        pub const fn set_wfr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "wake-up frame flag bit of PHY."]
        #[must_use]
        #[inline(always)]
        pub const fn phyr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "wake-up frame flag bit of PHY."]
        #[inline(always)]
        pub const fn set_phyr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Global unicast."]
        #[must_use]
        #[inline(always)]
        pub const fn gu(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Global unicast."]
        #[inline(always)]
        pub const fn set_gu(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Wakeup frame filter register pointer. reset."]
        #[must_use]
        #[inline(always)]
        pub const fn wffrpr(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup frame filter register pointer. reset."]
        #[inline(always)]
        pub const fn set_wffrpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Macpmtcsr {
        #[inline(always)]
        fn default() -> Macpmtcsr {
            Macpmtcsr(0)
        }
    }
    impl core::fmt::Debug for Macpmtcsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macpmtcsr")
                .field("pd", &self.pd())
                .field("mpe", &self.mpe())
                .field("wfe", &self.wfe())
                .field("phye", &self.phye())
                .field("mpr", &self.mpr())
                .field("wfr", &self.wfr())
                .field("phyr", &self.phyr())
                .field("gu", &self.gu())
                .field("wffrpr", &self.wffrpr())
                .finish()
        }
    }
    #[doc = "Ethernet MAC interrupt status register. (ETH_MACSR)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macsr(pub u32);
    impl Macsr {
        #[doc = "PMT status."]
        #[must_use]
        #[inline(always)]
        pub const fn pmts(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "PMT status."]
        #[inline(always)]
        pub const fn set_pmts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "MMC status."]
        #[must_use]
        #[inline(always)]
        pub const fn mmcs(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "MMC status."]
        #[inline(always)]
        pub const fn set_mmcs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "MMC receive status."]
        #[must_use]
        #[inline(always)]
        pub const fn mmcrs(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "MMC receive status."]
        #[inline(always)]
        pub const fn set_mmcrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "MMC transmit status."]
        #[must_use]
        #[inline(always)]
        pub const fn mmcts(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "MMC transmit status."]
        #[inline(always)]
        pub const fn set_mmcts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Time stamp trigger status."]
        #[must_use]
        #[inline(always)]
        pub const fn tsts(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Time stamp trigger status."]
        #[inline(always)]
        pub const fn set_tsts(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Macsr {
        #[inline(always)]
        fn default() -> Macsr {
            Macsr(0)
        }
    }
    impl core::fmt::Debug for Macsr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macsr")
                .field("pmts", &self.pmts())
                .field("mmcs", &self.mmcs())
                .field("mmcrs", &self.mmcrs())
                .field("mmcts", &self.mmcts())
                .field("tsts", &self.tsts())
                .finish()
        }
    }
    #[doc = "Ethernet MAC VLAN tag register. (ETH_MACVLAN)."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Macvlan(pub u32);
    impl Macvlan {
        #[doc = "VLAN tag identifier (for receive. frames)."]
        #[must_use]
        #[inline(always)]
        pub const fn vlanti(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "VLAN tag identifier (for receive. frames)."]
        #[inline(always)]
        pub const fn set_vlanti(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "12-bit VLAN tag comparison."]
        #[must_use]
        #[inline(always)]
        pub const fn vlantc(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "12-bit VLAN tag comparison."]
        #[inline(always)]
        pub const fn set_vlantc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
    }
    impl Default for Macvlan {
        #[inline(always)]
        fn default() -> Macvlan {
            Macvlan(0)
        }
    }
    impl core::fmt::Debug for Macvlan {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Macvlan")
                .field("vlanti", &self.vlanti())
                .field("vlantc", &self.vlantc())
                .finish()
        }
    }
    #[doc = "PHY configure register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PhyCr(pub u32);
    impl PhyCr {
        #[doc = "when the bit 7 set 1,bit4-0 serves as the address of the. PHY."]
        #[must_use]
        #[inline(always)]
        pub const fn rephyaddr(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x1f;
            val as u8
        }
        #[doc = "when the bit 7 set 1,bit4-0 serves as the address of the. PHY."]
        #[inline(always)]
        pub const fn set_rephyaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
        }
        #[doc = "Reconfigure the PHY address."]
        #[must_use]
        #[inline(always)]
        pub const fn phyaddr_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Reconfigure the PHY address."]
        #[inline(always)]
        pub const fn set_phyaddr_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Even Ethernet speed selection."]
        #[must_use]
        #[inline(always)]
        pub const fn speed(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Even Ethernet speed selection."]
        #[inline(always)]
        pub const fn set_speed(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Even Ethernet mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn duplex(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Even Ethernet mode selection."]
        #[inline(always)]
        pub const fn set_duplex(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "PHY work mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_eee(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "PHY work mode selection."]
        #[inline(always)]
        pub const fn set_phy_eee(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "PHY pd mode selection."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_pd(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "PHY pd mode selection."]
        #[inline(always)]
        pub const fn set_phy_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Ethernet PHY global reset signal."]
        #[must_use]
        #[inline(always)]
        pub const fn phy_rstn(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Ethernet PHY global reset signal."]
        #[inline(always)]
        pub const fn set_phy_rstn(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for PhyCr {
        #[inline(always)]
        fn default() -> PhyCr {
            PhyCr(0)
        }
    }
    impl core::fmt::Debug for PhyCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("PhyCr")
                .field("rephyaddr", &self.rephyaddr())
                .field("phyaddr_en", &self.phyaddr_en())
                .field("speed", &self.speed())
                .field("duplex", &self.duplex())
                .field("phy_eee", &self.phy_eee())
                .field("phy_pd", &self.phy_pd())
                .field("phy_rstn", &self.phy_rstn())
                .finish()
        }
    }
}
