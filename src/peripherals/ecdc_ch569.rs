#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "ECDC register."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ecdc {
    ptr: *mut u8,
}
unsafe impl Send for Ecdc {}
unsafe impl Sync for Ecdc {}
impl Ecdc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "ECDC AES/SM4 register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Interupt enable register."]
    #[inline(always)]
    pub const fn int_en(self) -> crate::common::Reg<regs::IntEn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02usize) as _) }
    }
    #[doc = "Interupt flag register."]
    #[inline(always)]
    pub const fn int_fg(self) -> crate::common::Reg<regs::IntFg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06usize) as _) }
    }
    #[doc = "User key 224-255 register."]
    #[inline(always)]
    pub const fn key_255t224(self) -> crate::common::Reg<regs::Key255t224, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "User key 192-223 register."]
    #[inline(always)]
    pub const fn key_223t192(self) -> crate::common::Reg<regs::Key223t192, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "User key 160-191 register."]
    #[inline(always)]
    pub const fn key_191t160(self) -> crate::common::Reg<regs::Key191t160, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "User key 128-159 register."]
    #[inline(always)]
    pub const fn key_159t128(self) -> crate::common::Reg<regs::Key159t128, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "User key 96-127 register."]
    #[inline(always)]
    pub const fn key_127t96(self) -> crate::common::Reg<regs::Key127t96, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "User key 64-95 register."]
    #[inline(always)]
    pub const fn key_95t64(self) -> crate::common::Reg<regs::Key95t64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "User key 32-63 register."]
    #[inline(always)]
    pub const fn key_63t32(self) -> crate::common::Reg<regs::Key63t32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "User key 0-31 register."]
    #[inline(always)]
    pub const fn key_31t0(self) -> crate::common::Reg<regs::Key31t0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "CTR mode count 96-127 register."]
    #[inline(always)]
    pub const fn iv_127t96(self) -> crate::common::Reg<regs::Iv127t96, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "CTR mode count 64-95 register."]
    #[inline(always)]
    pub const fn iv_95t64(self) -> crate::common::Reg<regs::Iv95t64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "CTR mode count 32-63 register."]
    #[inline(always)]
    pub const fn iv_63t32(self) -> crate::common::Reg<regs::Iv63t32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "CTR mode count 0-31 register."]
    #[inline(always)]
    pub const fn iv_31t0(self) -> crate::common::Reg<regs::Iv31t0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Single encryption and decryption of original data 96-127 register."]
    #[inline(always)]
    pub const fn sgsd_127t96(self) -> crate::common::Reg<regs::Sgsd127t96, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Single encryption and decryption of original data 64-95 register."]
    #[inline(always)]
    pub const fn sgsd_95t64(self) -> crate::common::Reg<regs::Sgsd95t64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Single encryption and decryption of original data 32-63 register."]
    #[inline(always)]
    pub const fn sgsd_63t32(self) -> crate::common::Reg<regs::Sgsd63t32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Single encryption and decryption of original data 0-31 register."]
    #[inline(always)]
    pub const fn sgsd_31t0(self) -> crate::common::Reg<regs::Sgsd31t0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Single encryption and decryption result 96-127 register."]
    #[inline(always)]
    pub const fn sgrt_127t96(self) -> crate::common::Reg<regs::Sgrt127t96, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Single encryption and decryption result 64-95 register."]
    #[inline(always)]
    pub const fn sgrt_95t64(self) -> crate::common::Reg<regs::Sgrt95t64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Single encryption and decryption result 0-31 register."]
    #[inline(always)]
    pub const fn sgrt_63t32(self) -> crate::common::Reg<regs::Sgrt63t32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Single encryption and decryption result 0-31 register."]
    #[inline(always)]
    pub const fn sgrt_31t0(self) -> crate::common::Reg<regs::Sgrt31t0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "encryption and decryption sram start address register."]
    #[inline(always)]
    pub const fn sram_addr(self) -> crate::common::Reg<regs::SramAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "encryption and decryption sram size register."]
    #[inline(always)]
    pub const fn sram_len(self) -> crate::common::Reg<regs::SramLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
}
pub mod regs {
    #[doc = "ECDC AES/SM4 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ctrl(pub u16);
    impl Ctrl {
        #[doc = "enable key expansion."]
        #[inline(always)]
        pub const fn keyex_en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable key expansion."]
        #[inline(always)]
        pub fn set_keyex_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u16) & 0x01) << 0usize);
        }
        #[doc = "when write data to dma."]
        #[inline(always)]
        pub const fn rdperi_en(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "when write data to dma."]
        #[inline(always)]
        pub fn set_rdperi_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u16) & 0x01) << 1usize);
        }
        #[doc = "when read data from dma."]
        #[inline(always)]
        pub const fn wrperi_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "when read data from dma."]
        #[inline(always)]
        pub fn set_wrperi_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u16) & 0x01) << 2usize);
        }
        #[doc = "ECDC mode select."]
        #[inline(always)]
        pub const fn mode_sel(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "ECDC mode select."]
        #[inline(always)]
        pub fn set_mode_sel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u16) & 0x01) << 3usize);
        }
        #[doc = "Clock divide factor."]
        #[inline(always)]
        pub const fn clkdiv_mask(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Clock divide factor."]
        #[inline(always)]
        pub fn set_clkdiv_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u16) & 0x07) << 4usize);
        }
        #[doc = "module dma enable."]
        #[inline(always)]
        pub const fn wrsram_en(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "module dma enable."]
        #[inline(always)]
        pub fn set_wrsram_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
        }
        #[doc = "Encryption and decryption algorithm mode selection."]
        #[inline(always)]
        pub const fn algrm_mod(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Encryption and decryption algorithm mode selection."]
        #[inline(always)]
        pub fn set_algrm_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
        }
        #[doc = "Block cipher mode selection."]
        #[inline(always)]
        pub const fn cipher_mod(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Block cipher mode selection."]
        #[inline(always)]
        pub fn set_cipher_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u16) & 0x01) << 9usize);
        }
        #[doc = "Key length setting."]
        #[inline(always)]
        pub const fn klen_mask(&self) -> u8 {
            let val = (self.0 >> 10usize) & 0x03;
            val as u8
        }
        #[doc = "Key length setting."]
        #[inline(always)]
        pub fn set_klen_mask(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u16) & 0x03) << 10usize);
        }
        #[doc = "source data and result data is bit endian."]
        #[inline(always)]
        pub const fn dat_mod(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "source data and result data is bit endian."]
        #[inline(always)]
        pub fn set_dat_mod(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u16) & 0x01) << 13usize);
        }
    }
    impl Default for Ctrl {
        #[inline(always)]
        fn default() -> Ctrl {
            Ctrl(0)
        }
    }
    #[doc = "Interupt enable register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntEn(pub u8);
    impl IntEn {
        #[doc = "Key extension completion interrupt enable."]
        #[inline(always)]
        pub const fn ie_ekdone(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Key extension completion interrupt enable."]
        #[inline(always)]
        pub fn set_ie_ekdone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Single encryption and decryption completion interrupt enable."]
        #[inline(always)]
        pub const fn ie_single(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Single encryption and decryption completion interrupt enable."]
        #[inline(always)]
        pub fn set_ie_single(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Memory to memory encryption and decryption completion interrupt enable."]
        #[inline(always)]
        pub const fn ie_wrsram(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Memory to memory encryption and decryption completion interrupt enable."]
        #[inline(always)]
        pub fn set_ie_wrsram(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
    }
    impl Default for IntEn {
        #[inline(always)]
        fn default() -> IntEn {
            IntEn(0)
        }
    }
    #[doc = "Interupt flag register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntFg(pub u8);
    impl IntFg {
        #[doc = "Key extension completion interrupt flag."]
        #[inline(always)]
        pub const fn if_ekdone(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Key extension completion interrupt flag."]
        #[inline(always)]
        pub fn set_if_ekdone(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u8) & 0x01) << 0usize);
        }
        #[doc = "Single encryption and decryption completion interrupt flag."]
        #[inline(always)]
        pub const fn if_single(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Single encryption and decryption completion interrupt flag."]
        #[inline(always)]
        pub fn set_if_single(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u8) & 0x01) << 1usize);
        }
        #[doc = "Memory to memory encryption and decryption completion interrupt flag."]
        #[inline(always)]
        pub const fn if_wrsram(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Memory to memory encryption and decryption completion interrupt flag."]
        #[inline(always)]
        pub fn set_if_wrsram(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u8) & 0x01) << 2usize);
        }
    }
    impl Default for IntFg {
        #[inline(always)]
        fn default() -> IntFg {
            IntFg(0)
        }
    }
    #[doc = "CTR mode count 96-127 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iv127t96(pub u32);
    impl Iv127t96 {
        #[doc = "CTR mode count 96-127 register."]
        #[inline(always)]
        pub const fn iv_127t96(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CTR mode count 96-127 register."]
        #[inline(always)]
        pub fn set_iv_127t96(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iv127t96 {
        #[inline(always)]
        fn default() -> Iv127t96 {
            Iv127t96(0)
        }
    }
    #[doc = "CTR mode count 0-31 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iv31t0(pub u32);
    impl Iv31t0 {
        #[doc = "CTR mode count 0-31 register."]
        #[inline(always)]
        pub const fn iv_31t0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CTR mode count 0-31 register."]
        #[inline(always)]
        pub fn set_iv_31t0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iv31t0 {
        #[inline(always)]
        fn default() -> Iv31t0 {
            Iv31t0(0)
        }
    }
    #[doc = "CTR mode count 32-63 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iv63t32(pub u32);
    impl Iv63t32 {
        #[doc = "CTR mode count 32-63 register."]
        #[inline(always)]
        pub const fn iv_63t32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CTR mode count 32-63 register."]
        #[inline(always)]
        pub fn set_iv_63t32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iv63t32 {
        #[inline(always)]
        fn default() -> Iv63t32 {
            Iv63t32(0)
        }
    }
    #[doc = "CTR mode count 64-95 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iv95t64(pub u32);
    impl Iv95t64 {
        #[doc = "CTR mode count 64-95 register."]
        #[inline(always)]
        pub const fn iv_95t64(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "CTR mode count 64-95 register."]
        #[inline(always)]
        pub fn set_iv_95t64(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iv95t64 {
        #[inline(always)]
        fn default() -> Iv95t64 {
            Iv95t64(0)
        }
    }
    #[doc = "User key 96-127 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key127t96(pub u32);
    impl Key127t96 {
        #[doc = "User key 96-127 register."]
        #[inline(always)]
        pub const fn key_127t96(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "User key 96-127 register."]
        #[inline(always)]
        pub fn set_key_127t96(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key127t96 {
        #[inline(always)]
        fn default() -> Key127t96 {
            Key127t96(0)
        }
    }
    #[doc = "User key 128-159 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key159t128(pub u32);
    impl Key159t128 {
        #[doc = "User key 128-159 register."]
        #[inline(always)]
        pub const fn key_159t128(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "User key 128-159 register."]
        #[inline(always)]
        pub fn set_key_159t128(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key159t128 {
        #[inline(always)]
        fn default() -> Key159t128 {
            Key159t128(0)
        }
    }
    #[doc = "User key 160-191 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key191t160(pub u32);
    impl Key191t160 {
        #[doc = "User key 160-191 register."]
        #[inline(always)]
        pub const fn key_191t160(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "User key 160-191 register."]
        #[inline(always)]
        pub fn set_key_191t160(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key191t160 {
        #[inline(always)]
        fn default() -> Key191t160 {
            Key191t160(0)
        }
    }
    #[doc = "User key 192-223 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key223t192(pub u32);
    impl Key223t192 {
        #[doc = "User key 192-223 register."]
        #[inline(always)]
        pub const fn key_223t192(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "User key 192-223 register."]
        #[inline(always)]
        pub fn set_key_223t192(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key223t192 {
        #[inline(always)]
        fn default() -> Key223t192 {
            Key223t192(0)
        }
    }
    #[doc = "User key 224-255 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key255t224(pub u32);
    impl Key255t224 {
        #[doc = "User key 224-255 register."]
        #[inline(always)]
        pub const fn key_255t224(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "User key 224-255 register."]
        #[inline(always)]
        pub fn set_key_255t224(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key255t224 {
        #[inline(always)]
        fn default() -> Key255t224 {
            Key255t224(0)
        }
    }
    #[doc = "User key 0-31 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key31t0(pub u32);
    impl Key31t0 {
        #[doc = "User key 0-31 register."]
        #[inline(always)]
        pub const fn key_31t0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "User key 0-31 register."]
        #[inline(always)]
        pub fn set_key_31t0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key31t0 {
        #[inline(always)]
        fn default() -> Key31t0 {
            Key31t0(0)
        }
    }
    #[doc = "User key 32-63 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key63t32(pub u32);
    impl Key63t32 {
        #[doc = "User key 32-63 register."]
        #[inline(always)]
        pub const fn key_63t32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "User key 32-63 register."]
        #[inline(always)]
        pub fn set_key_63t32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key63t32 {
        #[inline(always)]
        fn default() -> Key63t32 {
            Key63t32(0)
        }
    }
    #[doc = "User key 64-95 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Key95t64(pub u32);
    impl Key95t64 {
        #[doc = "User key 64-95 register."]
        #[inline(always)]
        pub const fn key_95t64(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "User key 64-95 register."]
        #[inline(always)]
        pub fn set_key_95t64(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Key95t64 {
        #[inline(always)]
        fn default() -> Key95t64 {
            Key95t64(0)
        }
    }
    #[doc = "Single encryption and decryption result 96-127 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sgrt127t96(pub u32);
    impl Sgrt127t96 {
        #[doc = "Single encryption and decryption result 96-127 register."]
        #[inline(always)]
        pub const fn sgrt_127t96(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Single encryption and decryption result 96-127 register."]
        #[inline(always)]
        pub fn set_sgrt_127t96(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sgrt127t96 {
        #[inline(always)]
        fn default() -> Sgrt127t96 {
            Sgrt127t96(0)
        }
    }
    #[doc = "Single encryption and decryption result 0-31 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sgrt31t0(pub u32);
    impl Sgrt31t0 {
        #[doc = "Single encryption and decryption result 0-31 register."]
        #[inline(always)]
        pub const fn sgrt_31t0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Single encryption and decryption result 0-31 register."]
        #[inline(always)]
        pub fn set_sgrt_31t0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sgrt31t0 {
        #[inline(always)]
        fn default() -> Sgrt31t0 {
            Sgrt31t0(0)
        }
    }
    #[doc = "Single encryption and decryption result 0-31 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sgrt63t32(pub u32);
    impl Sgrt63t32 {
        #[doc = "Single encryption and decryption result 0-31 register."]
        #[inline(always)]
        pub const fn sgrt_63t32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Single encryption and decryption result 0-31 register."]
        #[inline(always)]
        pub fn set_sgrt_63t32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sgrt63t32 {
        #[inline(always)]
        fn default() -> Sgrt63t32 {
            Sgrt63t32(0)
        }
    }
    #[doc = "Single encryption and decryption result 64-95 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sgrt95t64(pub u32);
    impl Sgrt95t64 {
        #[doc = "Single encryption and decryption result 64-95 register."]
        #[inline(always)]
        pub const fn sgrt_95t64(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Single encryption and decryption result 64-95 register."]
        #[inline(always)]
        pub fn set_sgrt_95t64(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sgrt95t64 {
        #[inline(always)]
        fn default() -> Sgrt95t64 {
            Sgrt95t64(0)
        }
    }
    #[doc = "Single encryption and decryption of original data 96-127 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sgsd127t96(pub u32);
    impl Sgsd127t96 {
        #[doc = "Single encryption and decryption of original data 96-127 register."]
        #[inline(always)]
        pub const fn sgsd_127t96(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Single encryption and decryption of original data 96-127 register."]
        #[inline(always)]
        pub fn set_sgsd_127t96(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sgsd127t96 {
        #[inline(always)]
        fn default() -> Sgsd127t96 {
            Sgsd127t96(0)
        }
    }
    #[doc = "Single encryption and decryption of original data 0-31 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sgsd31t0(pub u32);
    impl Sgsd31t0 {
        #[doc = "Single encryption and decryption of original data 0-31 register."]
        #[inline(always)]
        pub const fn sgsd_31t0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Single encryption and decryption of original data 0-31 register."]
        #[inline(always)]
        pub fn set_sgsd_31t0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sgsd31t0 {
        #[inline(always)]
        fn default() -> Sgsd31t0 {
            Sgsd31t0(0)
        }
    }
    #[doc = "Single encryption and decryption of original data 32-63 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sgsd63t32(pub u32);
    impl Sgsd63t32 {
        #[doc = "Single encryption and decryption of original data 32-63 register."]
        #[inline(always)]
        pub const fn sgsd_63t32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Single encryption and decryption of original data 32-63 register."]
        #[inline(always)]
        pub fn set_sgsd_63t32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sgsd63t32 {
        #[inline(always)]
        fn default() -> Sgsd63t32 {
            Sgsd63t32(0)
        }
    }
    #[doc = "Single encryption and decryption of original data 64-95 register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sgsd95t64(pub u32);
    impl Sgsd95t64 {
        #[doc = "Single encryption and decryption of original data 64-95 register."]
        #[inline(always)]
        pub const fn sgsd_95t64(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Single encryption and decryption of original data 64-95 register."]
        #[inline(always)]
        pub fn set_sgsd_95t64(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Sgsd95t64 {
        #[inline(always)]
        fn default() -> Sgsd95t64 {
            Sgsd95t64(0)
        }
    }
    #[doc = "encryption and decryption sram start address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SramAddr(pub u32);
    impl SramAddr {
        #[doc = "encryption and decryption sram start address register."]
        #[inline(always)]
        pub const fn sram_addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "encryption and decryption sram start address register."]
        #[inline(always)]
        pub fn set_sram_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for SramAddr {
        #[inline(always)]
        fn default() -> SramAddr {
            SramAddr(0)
        }
    }
    #[doc = "encryption and decryption sram size register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SramLen(pub u32);
    impl SramLen {
        #[doc = "encryption and decryption sram size register."]
        #[inline(always)]
        pub const fn sram_len(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x0001_ffff;
            val as u32
        }
        #[doc = "encryption and decryption sram size register."]
        #[inline(always)]
        pub fn set_sram_len(&mut self, val: u32) {
            self.0 = (self.0 & !(0x0001_ffff << 0usize)) | (((val as u32) & 0x0001_ffff) << 0usize);
        }
    }
    impl Default for SramLen {
        #[inline(always)]
        fn default() -> SramLen {
            SramLen(0)
        }
    }
}
