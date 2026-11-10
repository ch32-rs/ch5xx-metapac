#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Program Fast Interrupt Controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfic {
    ptr: *mut u8,
}
unsafe impl Send for Pfic {}
unsafe impl Sync for Pfic {}
impl Pfic {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "RO,Interrupt Status Register 1."]
    #[inline(always)]
    pub const fn isr1(self) -> crate::common::Reg<regs::Isr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "RO,Interrupt Status Register 2."]
    #[inline(always)]
    pub const fn isr2(self) -> crate::common::Reg<regs::Isr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "RO,Interrupt Pending Register 1."]
    #[inline(always)]
    pub const fn ipr1(self) -> crate::common::Reg<regs::Ipr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
    #[doc = "RO,Interrupt Pending Register 2."]
    #[inline(always)]
    pub const fn ipr2(self) -> crate::common::Reg<regs::Ipr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize) as _) }
    }
    #[doc = "RW,Interrupt Priority Register."]
    #[inline(always)]
    pub const fn ithresdr(self) -> crate::common::Reg<regs::Ithresdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "Interrupt Config Register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x48usize) as _) }
    }
    #[doc = "Interrupt Global Register."]
    #[inline(always)]
    pub const fn gisr(self) -> crate::common::Reg<regs::Gisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x4cusize) as _) }
    }
    #[doc = "RW,Interrupt Fast ID Config Register."]
    #[inline(always)]
    pub const fn idcfgr(self) -> crate::common::Reg<regs::Idcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "Interrupt 0 address Register."]
    #[inline(always)]
    pub const fn fiaddrr0(self) -> crate::common::Reg<regs::Fiaddrr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "Interrupt 1 address Register."]
    #[inline(always)]
    pub const fn fiaddrr1(self) -> crate::common::Reg<regs::Fiaddrr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "Interrupt 2 address Register."]
    #[inline(always)]
    pub const fn fiaddrr2(self) -> crate::common::Reg<regs::Fiaddrr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "Interrupt 3 address Register."]
    #[inline(always)]
    pub const fn fiaddrr3(self) -> crate::common::Reg<regs::Fiaddrr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "Interrupt Setting Register."]
    #[inline(always)]
    pub const fn ienr1(self) -> crate::common::Reg<regs::Ienr1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "Interrupt Setting Register."]
    #[inline(always)]
    pub const fn ienr2(self) -> crate::common::Reg<regs::Ienr2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "Interrupt Clear Register."]
    #[inline(always)]
    pub const fn irer1(self) -> crate::common::Reg<regs::Irer1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "Interrupt Clear Register."]
    #[inline(always)]
    pub const fn irer2(self) -> crate::common::Reg<regs::Irer2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipsr1(self) -> crate::common::Reg<regs::Ipsr1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipsr2(self) -> crate::common::Reg<regs::Ipsr2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[inline(always)]
    pub const fn iprr1(self) -> crate::common::Reg<regs::Iprr1, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[inline(always)]
    pub const fn iprr2(self) -> crate::common::Reg<regs::Iprr2, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[inline(always)]
    pub const fn iactr1(self) -> crate::common::Reg<regs::Iactr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[inline(always)]
    pub const fn iactr2(self) -> crate::common::Reg<regs::Iactr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 0."]
    #[inline(always)]
    pub const fn iprior0(self) -> crate::common::Reg<regs::Iprior0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 1."]
    #[inline(always)]
    pub const fn iprior1(self) -> crate::common::Reg<regs::Iprior1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 2."]
    #[inline(always)]
    pub const fn iprior2(self) -> crate::common::Reg<regs::Iprior2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 3."]
    #[inline(always)]
    pub const fn iprior3(self) -> crate::common::Reg<regs::Iprior3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0460usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 4."]
    #[inline(always)]
    pub const fn iprior4(self) -> crate::common::Reg<regs::Iprior4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 5."]
    #[inline(always)]
    pub const fn iprior5(self) -> crate::common::Reg<regs::Iprior5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 6."]
    #[inline(always)]
    pub const fn iprior6(self) -> crate::common::Reg<regs::Iprior6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 7."]
    #[inline(always)]
    pub const fn iprior7(self) -> crate::common::Reg<regs::Iprior7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 8."]
    #[inline(always)]
    pub const fn iprior8(self) -> crate::common::Reg<regs::Iprior8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 9."]
    #[inline(always)]
    pub const fn iprior9(self) -> crate::common::Reg<regs::Iprior9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 10."]
    #[inline(always)]
    pub const fn iprior10(self) -> crate::common::Reg<regs::Iprior10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 11."]
    #[inline(always)]
    pub const fn iprior11(self) -> crate::common::Reg<regs::Iprior11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 12."]
    #[inline(always)]
    pub const fn iprior12(self) -> crate::common::Reg<regs::Iprior12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 13."]
    #[inline(always)]
    pub const fn iprior13(self) -> crate::common::Reg<regs::Iprior13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 14."]
    #[inline(always)]
    pub const fn iprior14(self) -> crate::common::Reg<regs::Iprior14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 15."]
    #[inline(always)]
    pub const fn iprior15(self) -> crate::common::Reg<regs::Iprior15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 16."]
    #[inline(always)]
    pub const fn iprior16(self) -> crate::common::Reg<regs::Iprior16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 17."]
    #[inline(always)]
    pub const fn iprior17(self) -> crate::common::Reg<regs::Iprior17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 18."]
    #[inline(always)]
    pub const fn iprior18(self) -> crate::common::Reg<regs::Iprior18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0640usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 19."]
    #[inline(always)]
    pub const fn iprior19(self) -> crate::common::Reg<regs::Iprior19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0660usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 20."]
    #[inline(always)]
    pub const fn iprior20(self) -> crate::common::Reg<regs::Iprior20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0680usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 21."]
    #[inline(always)]
    pub const fn iprior21(self) -> crate::common::Reg<regs::Iprior21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06a0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 22."]
    #[inline(always)]
    pub const fn iprior22(self) -> crate::common::Reg<regs::Iprior22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 23."]
    #[inline(always)]
    pub const fn iprior23(self) -> crate::common::Reg<regs::Iprior23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 24."]
    #[inline(always)]
    pub const fn iprior24(self) -> crate::common::Reg<regs::Iprior24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 25."]
    #[inline(always)]
    pub const fn iprior25(self) -> crate::common::Reg<regs::Iprior25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0720usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 26."]
    #[inline(always)]
    pub const fn iprior26(self) -> crate::common::Reg<regs::Iprior26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0740usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 27."]
    #[inline(always)]
    pub const fn iprior27(self) -> crate::common::Reg<regs::Iprior27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0760usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 28."]
    #[inline(always)]
    pub const fn iprior28(self) -> crate::common::Reg<regs::Iprior28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0780usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 29."]
    #[inline(always)]
    pub const fn iprior29(self) -> crate::common::Reg<regs::Iprior29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 30."]
    #[inline(always)]
    pub const fn iprior30(self) -> crate::common::Reg<regs::Iprior30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07c0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 31."]
    #[inline(always)]
    pub const fn iprior31(self) -> crate::common::Reg<regs::Iprior31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07e0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 32."]
    #[inline(always)]
    pub const fn iprior32(self) -> crate::common::Reg<regs::Iprior32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0800usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 33."]
    #[inline(always)]
    pub const fn iprior33(self) -> crate::common::Reg<regs::Iprior33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0820usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 34."]
    #[inline(always)]
    pub const fn iprior34(self) -> crate::common::Reg<regs::Iprior34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0840usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 35."]
    #[inline(always)]
    pub const fn iprior35(self) -> crate::common::Reg<regs::Iprior35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0860usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 36."]
    #[inline(always)]
    pub const fn iprior36(self) -> crate::common::Reg<regs::Iprior36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0880usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 37."]
    #[inline(always)]
    pub const fn iprior37(self) -> crate::common::Reg<regs::Iprior37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08a0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 38."]
    #[inline(always)]
    pub const fn iprior38(self) -> crate::common::Reg<regs::Iprior38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08c0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 39."]
    #[inline(always)]
    pub const fn iprior39(self) -> crate::common::Reg<regs::Iprior39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08e0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 40."]
    #[inline(always)]
    pub const fn iprior40(self) -> crate::common::Reg<regs::Iprior40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0900usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 41."]
    #[inline(always)]
    pub const fn iprior41(self) -> crate::common::Reg<regs::Iprior41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0920usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 42."]
    #[inline(always)]
    pub const fn iprior42(self) -> crate::common::Reg<regs::Iprior42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0940usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 43."]
    #[inline(always)]
    pub const fn iprior43(self) -> crate::common::Reg<regs::Iprior43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0960usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 44."]
    #[inline(always)]
    pub const fn iprior44(self) -> crate::common::Reg<regs::Iprior44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0980usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 45."]
    #[inline(always)]
    pub const fn iprior45(self) -> crate::common::Reg<regs::Iprior45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09a0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 46."]
    #[inline(always)]
    pub const fn iprior46(self) -> crate::common::Reg<regs::Iprior46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09c0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 47."]
    #[inline(always)]
    pub const fn iprior47(self) -> crate::common::Reg<regs::Iprior47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x09e0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 48."]
    #[inline(always)]
    pub const fn iprior48(self) -> crate::common::Reg<regs::Iprior48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a00usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 49."]
    #[inline(always)]
    pub const fn iprior49(self) -> crate::common::Reg<regs::Iprior49, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a20usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 50."]
    #[inline(always)]
    pub const fn iprior50(self) -> crate::common::Reg<regs::Iprior50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a40usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 51."]
    #[inline(always)]
    pub const fn iprior51(self) -> crate::common::Reg<regs::Iprior51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a60usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 52."]
    #[inline(always)]
    pub const fn iprior52(self) -> crate::common::Reg<regs::Iprior52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0a80usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 53."]
    #[inline(always)]
    pub const fn iprior53(self) -> crate::common::Reg<regs::Iprior53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0aa0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 54."]
    #[inline(always)]
    pub const fn iprior54(self) -> crate::common::Reg<regs::Iprior54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ad0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 55."]
    #[inline(always)]
    pub const fn iprior55(self) -> crate::common::Reg<regs::Iprior55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ae0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 56."]
    #[inline(always)]
    pub const fn iprior56(self) -> crate::common::Reg<regs::Iprior56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b00usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 57."]
    #[inline(always)]
    pub const fn iprior57(self) -> crate::common::Reg<regs::Iprior57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b20usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 58."]
    #[inline(always)]
    pub const fn iprior58(self) -> crate::common::Reg<regs::Iprior58, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b40usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 59."]
    #[inline(always)]
    pub const fn iprior59(self) -> crate::common::Reg<regs::Iprior59, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b60usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 60."]
    #[inline(always)]
    pub const fn iprior60(self) -> crate::common::Reg<regs::Iprior60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0b80usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 61."]
    #[inline(always)]
    pub const fn iprior61(self) -> crate::common::Reg<regs::Iprior61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0ba0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 62."]
    #[inline(always)]
    pub const fn iprior62(self) -> crate::common::Reg<regs::Iprior62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0be0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register 63."]
    #[inline(always)]
    pub const fn iprior63(self) -> crate::common::Reg<regs::Iprior63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0c00usize) as _) }
    }
    #[doc = "System Control Register."]
    #[inline(always)]
    pub const fn sctlr(self) -> crate::common::Reg<regs::Sctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0d10usize) as _) }
    }
}
pub mod regs {
    #[doc = "Interrupt Config Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "WO,RESETSYS."]
        #[must_use]
        #[inline(always)]
        pub const fn resetsys(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "WO,RESETSYS."]
        #[inline(always)]
        pub const fn set_resetsys(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "WO,KEYCODE."]
        #[must_use]
        #[inline(always)]
        pub const fn keycode(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "WO,KEYCODE."]
        #[inline(always)]
        pub const fn set_keycode(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    impl core::fmt::Debug for Cfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cfgr")
                .field("resetsys", &self.resetsys())
                .field("keycode", &self.keycode())
                .finish()
        }
    }
    #[doc = "Interrupt 0 address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fiaddrr0(pub u32);
    impl Fiaddrr0 {
        #[doc = "RW,FI0EN."]
        #[must_use]
        #[inline(always)]
        pub const fn fi0en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW,FI0EN."]
        #[inline(always)]
        pub const fn set_fi0en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RW,ADDR0."]
        #[must_use]
        #[inline(always)]
        pub const fn addr0(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "RW,ADDR0."]
        #[inline(always)]
        pub const fn set_addr0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for Fiaddrr0 {
        #[inline(always)]
        fn default() -> Fiaddrr0 {
            Fiaddrr0(0)
        }
    }
    impl core::fmt::Debug for Fiaddrr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fiaddrr0")
                .field("fi0en", &self.fi0en())
                .field("addr0", &self.addr0())
                .finish()
        }
    }
    #[doc = "Interrupt 1 address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fiaddrr1(pub u32);
    impl Fiaddrr1 {
        #[doc = "RW,FI1EN."]
        #[must_use]
        #[inline(always)]
        pub const fn fi1en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW,FI1EN."]
        #[inline(always)]
        pub const fn set_fi1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RW,ADDR1."]
        #[must_use]
        #[inline(always)]
        pub const fn addr1(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "RW,ADDR1."]
        #[inline(always)]
        pub const fn set_addr1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for Fiaddrr1 {
        #[inline(always)]
        fn default() -> Fiaddrr1 {
            Fiaddrr1(0)
        }
    }
    impl core::fmt::Debug for Fiaddrr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fiaddrr1")
                .field("fi1en", &self.fi1en())
                .field("addr1", &self.addr1())
                .finish()
        }
    }
    #[doc = "Interrupt 2 address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fiaddrr2(pub u32);
    impl Fiaddrr2 {
        #[doc = "RW,FI2EN."]
        #[must_use]
        #[inline(always)]
        pub const fn fi2en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW,FI2EN."]
        #[inline(always)]
        pub const fn set_fi2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RW,ADDR2."]
        #[must_use]
        #[inline(always)]
        pub const fn addr2(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "RW,ADDR2."]
        #[inline(always)]
        pub const fn set_addr2(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for Fiaddrr2 {
        #[inline(always)]
        fn default() -> Fiaddrr2 {
            Fiaddrr2(0)
        }
    }
    impl core::fmt::Debug for Fiaddrr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fiaddrr2")
                .field("fi2en", &self.fi2en())
                .field("addr2", &self.addr2())
                .finish()
        }
    }
    #[doc = "Interrupt 3 address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fiaddrr3(pub u32);
    impl Fiaddrr3 {
        #[doc = "RW,FI3EN."]
        #[must_use]
        #[inline(always)]
        pub const fn fi3en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW,FI3EN."]
        #[inline(always)]
        pub const fn set_fi3en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RW,ADDR3."]
        #[must_use]
        #[inline(always)]
        pub const fn addr3(&self) -> u32 {
            let val = (self.0 >> 1usize) & 0x7fff_ffff;
            val as u32
        }
        #[doc = "RW,ADDR3."]
        #[inline(always)]
        pub const fn set_addr3(&mut self, val: u32) {
            self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
        }
    }
    impl Default for Fiaddrr3 {
        #[inline(always)]
        fn default() -> Fiaddrr3 {
            Fiaddrr3(0)
        }
    }
    impl core::fmt::Debug for Fiaddrr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Fiaddrr3")
                .field("fi3en", &self.fi3en())
                .field("addr3", &self.addr3())
                .finish()
        }
    }
    #[doc = "Interrupt Global Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gisr(pub u32);
    impl Gisr {
        #[doc = "RO,NESTSTA."]
        #[must_use]
        #[inline(always)]
        pub const fn neststa(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RO,NESTSTA."]
        #[inline(always)]
        pub const fn set_neststa(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "RO,GACTSTA."]
        #[must_use]
        #[inline(always)]
        pub const fn gactsta(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RO,GACTSTA."]
        #[inline(always)]
        pub const fn set_gactsta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "RO,GPENDSTA."]
        #[must_use]
        #[inline(always)]
        pub const fn gpendsta(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RO,GPENDSTA."]
        #[inline(always)]
        pub const fn set_gpendsta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Gisr {
        #[inline(always)]
        fn default() -> Gisr {
            Gisr(0)
        }
    }
    impl core::fmt::Debug for Gisr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Gisr")
                .field("neststa", &self.neststa())
                .field("gactsta", &self.gactsta())
                .field("gpendsta", &self.gpendsta())
                .finish()
        }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iactr1(pub u32);
    impl Iactr1 {
        #[doc = "RW1,IACTS."]
        #[must_use]
        #[inline(always)]
        pub const fn iacts(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "RW1,IACTS."]
        #[inline(always)]
        pub const fn set_iacts(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Iactr1 {
        #[inline(always)]
        fn default() -> Iactr1 {
            Iactr1(0)
        }
    }
    impl core::fmt::Debug for Iactr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iactr1")
                .field("iacts", &self.iacts())
                .finish()
        }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iactr2(pub u32);
    impl Iactr2 {
        #[doc = "RW1,IACTS."]
        #[must_use]
        #[inline(always)]
        pub const fn iacts(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RW1,IACTS."]
        #[inline(always)]
        pub const fn set_iacts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Iactr2 {
        #[inline(always)]
        fn default() -> Iactr2 {
            Iactr2(0)
        }
    }
    impl core::fmt::Debug for Iactr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iactr2")
                .field("iacts", &self.iacts())
                .finish()
        }
    }
    #[doc = "RW,Interrupt Fast ID Config Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Idcfgr(pub u32);
    impl Idcfgr {
        #[doc = "RW,FIID0."]
        #[must_use]
        #[inline(always)]
        pub const fn fiid0(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW,FIID0."]
        #[inline(always)]
        pub const fn set_fiid0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "RW,FIID1."]
        #[must_use]
        #[inline(always)]
        pub const fn fiid1(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "RW,FIID1."]
        #[inline(always)]
        pub const fn set_fiid1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "RW,FIID2."]
        #[must_use]
        #[inline(always)]
        pub const fn fiid2(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "RW,FIID2."]
        #[inline(always)]
        pub const fn set_fiid2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "RW,FIID3."]
        #[must_use]
        #[inline(always)]
        pub const fn fiid3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "RW,FIID3."]
        #[inline(always)]
        pub const fn set_fiid3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Idcfgr {
        #[inline(always)]
        fn default() -> Idcfgr {
            Idcfgr(0)
        }
    }
    impl core::fmt::Debug for Idcfgr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Idcfgr")
                .field("fiid0", &self.fiid0())
                .field("fiid1", &self.fiid1())
                .field("fiid2", &self.fiid2())
                .field("fiid3", &self.fiid3())
                .finish()
        }
    }
    #[doc = "Interrupt Setting Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ienr1(pub u32);
    impl Ienr1 {
        #[doc = "RW1,INTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn inten(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "RW1,INTEN."]
        #[inline(always)]
        pub const fn set_inten(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Ienr1 {
        #[inline(always)]
        fn default() -> Ienr1 {
            Ienr1(0)
        }
    }
    impl core::fmt::Debug for Ienr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ienr1")
                .field("inten", &self.inten())
                .finish()
        }
    }
    #[doc = "Interrupt Setting Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ienr2(pub u32);
    impl Ienr2 {
        #[doc = "RW1,INTEN."]
        #[must_use]
        #[inline(always)]
        pub const fn inten(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RW1,INTEN."]
        #[inline(always)]
        pub const fn set_inten(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Ienr2 {
        #[inline(always)]
        fn default() -> Ienr2 {
            Ienr2(0)
        }
    }
    impl core::fmt::Debug for Ienr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ienr2")
                .field("inten", &self.inten())
                .finish()
        }
    }
    #[doc = "RO,Interrupt Pending Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipr1(pub u32);
    impl Ipr1 {
        #[doc = "PENDSTA."]
        #[must_use]
        #[inline(always)]
        pub const fn pendsta(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub const fn set_pendsta(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Ipr1 {
        #[inline(always)]
        fn default() -> Ipr1 {
            Ipr1(0)
        }
    }
    impl core::fmt::Debug for Ipr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipr1")
                .field("pendsta", &self.pendsta())
                .finish()
        }
    }
    #[doc = "RO,Interrupt Pending Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipr2(pub u32);
    impl Ipr2 {
        #[doc = "PENDSTA."]
        #[must_use]
        #[inline(always)]
        pub const fn pendsta(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub const fn set_pendsta(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Ipr2 {
        #[inline(always)]
        fn default() -> Ipr2 {
            Ipr2(0)
        }
    }
    impl core::fmt::Debug for Ipr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipr2")
                .field("pendsta", &self.pendsta())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 0."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior0(pub u32);
    impl Iprior0 {
        #[doc = "RW,Interrupt priority for number 0-3."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 0-3."]
        #[inline(always)]
        pub const fn set_iprior0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior0 {
        #[inline(always)]
        fn default() -> Iprior0 {
            Iprior0(0)
        }
    }
    impl core::fmt::Debug for Iprior0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior0")
                .field("iprior0", &self.iprior0())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior1(pub u32);
    impl Iprior1 {
        #[doc = ">RW,Interrupt priority for number 4-7."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,Interrupt priority for number 4-7."]
        #[inline(always)]
        pub const fn set_iprior1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior1 {
        #[inline(always)]
        fn default() -> Iprior1 {
            Iprior1(0)
        }
    }
    impl core::fmt::Debug for Iprior1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior1")
                .field("iprior1", &self.iprior1())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 10."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior10(pub u32);
    impl Iprior10 {
        #[doc = ">RW,Interrupt priority for number 40-43."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior10(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,Interrupt priority for number 40-43."]
        #[inline(always)]
        pub const fn set_iprior10(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior10 {
        #[inline(always)]
        fn default() -> Iprior10 {
            Iprior10(0)
        }
    }
    impl core::fmt::Debug for Iprior10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior10")
                .field("iprior10", &self.iprior10())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 11."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior11(pub u32);
    impl Iprior11 {
        #[doc = "RW,Interrupt priority for number 44-47."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior11(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 44-47."]
        #[inline(always)]
        pub const fn set_iprior11(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior11 {
        #[inline(always)]
        fn default() -> Iprior11 {
            Iprior11(0)
        }
    }
    impl core::fmt::Debug for Iprior11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior11")
                .field("iprior11", &self.iprior11())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 12."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior12(pub u32);
    impl Iprior12 {
        #[doc = "RW,Interrupt priority for number 48-51."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior12(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 48-51."]
        #[inline(always)]
        pub const fn set_iprior12(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior12 {
        #[inline(always)]
        fn default() -> Iprior12 {
            Iprior12(0)
        }
    }
    impl core::fmt::Debug for Iprior12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior12")
                .field("iprior12", &self.iprior12())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 13."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior13(pub u32);
    impl Iprior13 {
        #[doc = "RW,Interrupt priority for number 52-55."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior13(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 52-55."]
        #[inline(always)]
        pub const fn set_iprior13(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior13 {
        #[inline(always)]
        fn default() -> Iprior13 {
            Iprior13(0)
        }
    }
    impl core::fmt::Debug for Iprior13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior13")
                .field("iprior13", &self.iprior13())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 14."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior14(pub u32);
    impl Iprior14 {
        #[doc = "RW,Interrupt priority for number 56-59."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior14(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 56-59."]
        #[inline(always)]
        pub const fn set_iprior14(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior14 {
        #[inline(always)]
        fn default() -> Iprior14 {
            Iprior14(0)
        }
    }
    impl core::fmt::Debug for Iprior14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior14")
                .field("iprior14", &self.iprior14())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 15."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior15(pub u32);
    impl Iprior15 {
        #[doc = "RW,Interrupt priority for number 60-63."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior15(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 60-63."]
        #[inline(always)]
        pub const fn set_iprior15(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior15 {
        #[inline(always)]
        fn default() -> Iprior15 {
            Iprior15(0)
        }
    }
    impl core::fmt::Debug for Iprior15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior15")
                .field("iprior15", &self.iprior15())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 16."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior16(pub u32);
    impl Iprior16 {
        #[doc = "RW,Interrupt priority for number 64-67."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior16(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 64-67."]
        #[inline(always)]
        pub const fn set_iprior16(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior16 {
        #[inline(always)]
        fn default() -> Iprior16 {
            Iprior16(0)
        }
    }
    impl core::fmt::Debug for Iprior16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior16")
                .field("iprior16", &self.iprior16())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 17."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior17(pub u32);
    impl Iprior17 {
        #[doc = "RW,Interrupt priority for number 68-71."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior17(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 68-71."]
        #[inline(always)]
        pub const fn set_iprior17(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior17 {
        #[inline(always)]
        fn default() -> Iprior17 {
            Iprior17(0)
        }
    }
    impl core::fmt::Debug for Iprior17 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior17")
                .field("iprior17", &self.iprior17())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 18."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior18(pub u32);
    impl Iprior18 {
        #[doc = "RW,Interrupt priority for number 72-75."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior18(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 72-75."]
        #[inline(always)]
        pub const fn set_iprior18(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior18 {
        #[inline(always)]
        fn default() -> Iprior18 {
            Iprior18(0)
        }
    }
    impl core::fmt::Debug for Iprior18 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior18")
                .field("iprior18", &self.iprior18())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 19."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior19(pub u32);
    impl Iprior19 {
        #[doc = "RW,Interrupt priority for number 76-79."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior19(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 76-79."]
        #[inline(always)]
        pub const fn set_iprior19(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior19 {
        #[inline(always)]
        fn default() -> Iprior19 {
            Iprior19(0)
        }
    }
    impl core::fmt::Debug for Iprior19 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior19")
                .field("iprior19", &self.iprior19())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior2(pub u32);
    impl Iprior2 {
        #[doc = ">RW,Interrupt priority for number 8-11."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,Interrupt priority for number 8-11."]
        #[inline(always)]
        pub const fn set_iprior2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior2 {
        #[inline(always)]
        fn default() -> Iprior2 {
            Iprior2(0)
        }
    }
    impl core::fmt::Debug for Iprior2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior2")
                .field("iprior2", &self.iprior2())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 20."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior20(pub u32);
    impl Iprior20 {
        #[doc = "RW,RW,Interrupt priority for number 80-83."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior20(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,RW,Interrupt priority for number 80-83."]
        #[inline(always)]
        pub const fn set_iprior20(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior20 {
        #[inline(always)]
        fn default() -> Iprior20 {
            Iprior20(0)
        }
    }
    impl core::fmt::Debug for Iprior20 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior20")
                .field("iprior20", &self.iprior20())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 21."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior21(pub u32);
    impl Iprior21 {
        #[doc = "RW,Interrupt priority for number 84-87."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior21(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 84-87."]
        #[inline(always)]
        pub const fn set_iprior21(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior21 {
        #[inline(always)]
        fn default() -> Iprior21 {
            Iprior21(0)
        }
    }
    impl core::fmt::Debug for Iprior21 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior21")
                .field("iprior21", &self.iprior21())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 22."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior22(pub u32);
    impl Iprior22 {
        #[doc = "RW,Interrupt priority for number 88-91."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior22(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 88-91."]
        #[inline(always)]
        pub const fn set_iprior22(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior22 {
        #[inline(always)]
        fn default() -> Iprior22 {
            Iprior22(0)
        }
    }
    impl core::fmt::Debug for Iprior22 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior22")
                .field("iprior22", &self.iprior22())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 23."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior23(pub u32);
    impl Iprior23 {
        #[doc = "RW,Interrupt priority for number 92-95."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior23(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 92-95."]
        #[inline(always)]
        pub const fn set_iprior23(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior23 {
        #[inline(always)]
        fn default() -> Iprior23 {
            Iprior23(0)
        }
    }
    impl core::fmt::Debug for Iprior23 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior23")
                .field("iprior23", &self.iprior23())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 24."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior24(pub u32);
    impl Iprior24 {
        #[doc = "RW,Interrupt priority for number 96-99."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior24(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 96-99."]
        #[inline(always)]
        pub const fn set_iprior24(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior24 {
        #[inline(always)]
        fn default() -> Iprior24 {
            Iprior24(0)
        }
    }
    impl core::fmt::Debug for Iprior24 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior24")
                .field("iprior24", &self.iprior24())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 25."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior25(pub u32);
    impl Iprior25 {
        #[doc = "RW,Interrupt priority for number 100-103."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior25(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 100-103."]
        #[inline(always)]
        pub const fn set_iprior25(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior25 {
        #[inline(always)]
        fn default() -> Iprior25 {
            Iprior25(0)
        }
    }
    impl core::fmt::Debug for Iprior25 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior25")
                .field("iprior25", &self.iprior25())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 26."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior26(pub u32);
    impl Iprior26 {
        #[doc = "RW,Interrupt priority for number 104-107."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior26(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 104-107."]
        #[inline(always)]
        pub const fn set_iprior26(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior26 {
        #[inline(always)]
        fn default() -> Iprior26 {
            Iprior26(0)
        }
    }
    impl core::fmt::Debug for Iprior26 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior26")
                .field("iprior26", &self.iprior26())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 27."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior27(pub u32);
    impl Iprior27 {
        #[doc = "RW,Interrupt priority for number 108-111."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior27(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 108-111."]
        #[inline(always)]
        pub const fn set_iprior27(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior27 {
        #[inline(always)]
        fn default() -> Iprior27 {
            Iprior27(0)
        }
    }
    impl core::fmt::Debug for Iprior27 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior27")
                .field("iprior27", &self.iprior27())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 28."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior28(pub u32);
    impl Iprior28 {
        #[doc = "RW,Interrupt priority for number 112-115."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior28(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 112-115."]
        #[inline(always)]
        pub const fn set_iprior28(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior28 {
        #[inline(always)]
        fn default() -> Iprior28 {
            Iprior28(0)
        }
    }
    impl core::fmt::Debug for Iprior28 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior28")
                .field("iprior28", &self.iprior28())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 29."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior29(pub u32);
    impl Iprior29 {
        #[doc = "RW,Interrupt priority for number 116-119."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior29(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 116-119."]
        #[inline(always)]
        pub const fn set_iprior29(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior29 {
        #[inline(always)]
        fn default() -> Iprior29 {
            Iprior29(0)
        }
    }
    impl core::fmt::Debug for Iprior29 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior29")
                .field("iprior29", &self.iprior29())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 3."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior3(pub u32);
    impl Iprior3 {
        #[doc = ">RW,Interrupt priority for number 12-15."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,Interrupt priority for number 12-15."]
        #[inline(always)]
        pub const fn set_iprior3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior3 {
        #[inline(always)]
        fn default() -> Iprior3 {
            Iprior3(0)
        }
    }
    impl core::fmt::Debug for Iprior3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior3")
                .field("iprior3", &self.iprior3())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 30."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior30(pub u32);
    impl Iprior30 {
        #[doc = "RW,Interrupt priority for number 120-123."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior30(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 120-123."]
        #[inline(always)]
        pub const fn set_iprior30(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior30 {
        #[inline(always)]
        fn default() -> Iprior30 {
            Iprior30(0)
        }
    }
    impl core::fmt::Debug for Iprior30 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior30")
                .field("iprior30", &self.iprior30())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 31."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior31(pub u32);
    impl Iprior31 {
        #[doc = "RW,Interrupt priority for number 124-127."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior31(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 124-127."]
        #[inline(always)]
        pub const fn set_iprior31(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior31 {
        #[inline(always)]
        fn default() -> Iprior31 {
            Iprior31(0)
        }
    }
    impl core::fmt::Debug for Iprior31 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior31")
                .field("iprior31", &self.iprior31())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 32."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior32(pub u32);
    impl Iprior32 {
        #[doc = "RW,Interrupt priority for number 128-131."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 128-131."]
        #[inline(always)]
        pub const fn set_iprior32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior32 {
        #[inline(always)]
        fn default() -> Iprior32 {
            Iprior32(0)
        }
    }
    impl core::fmt::Debug for Iprior32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior32")
                .field("iprior32", &self.iprior32())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 33."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior33(pub u32);
    impl Iprior33 {
        #[doc = "RW,Interrupt priority for number 132-135."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior33(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 132-135."]
        #[inline(always)]
        pub const fn set_iprior33(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior33 {
        #[inline(always)]
        fn default() -> Iprior33 {
            Iprior33(0)
        }
    }
    impl core::fmt::Debug for Iprior33 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior33")
                .field("iprior33", &self.iprior33())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 34."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior34(pub u32);
    impl Iprior34 {
        #[doc = "RW,Interrupt priority for number 136-139."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior34(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 136-139."]
        #[inline(always)]
        pub const fn set_iprior34(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior34 {
        #[inline(always)]
        fn default() -> Iprior34 {
            Iprior34(0)
        }
    }
    impl core::fmt::Debug for Iprior34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior34")
                .field("iprior34", &self.iprior34())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 35."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior35(pub u32);
    impl Iprior35 {
        #[doc = "RW,Interrupt priority for number 140-143."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior35(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 140-143."]
        #[inline(always)]
        pub const fn set_iprior35(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior35 {
        #[inline(always)]
        fn default() -> Iprior35 {
            Iprior35(0)
        }
    }
    impl core::fmt::Debug for Iprior35 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior35")
                .field("iprior35", &self.iprior35())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 36."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior36(pub u32);
    impl Iprior36 {
        #[doc = "RW,Interrupt priority for number 144-147."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior36(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 144-147."]
        #[inline(always)]
        pub const fn set_iprior36(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior36 {
        #[inline(always)]
        fn default() -> Iprior36 {
            Iprior36(0)
        }
    }
    impl core::fmt::Debug for Iprior36 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior36")
                .field("iprior36", &self.iprior36())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 37."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior37(pub u32);
    impl Iprior37 {
        #[doc = "RW,Interrupt priority for number 148-151."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior37(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 148-151."]
        #[inline(always)]
        pub const fn set_iprior37(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior37 {
        #[inline(always)]
        fn default() -> Iprior37 {
            Iprior37(0)
        }
    }
    impl core::fmt::Debug for Iprior37 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior37")
                .field("iprior37", &self.iprior37())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 38."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior38(pub u32);
    impl Iprior38 {
        #[doc = "RW,Interrupt priority for number 152-155."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior38(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 152-155."]
        #[inline(always)]
        pub const fn set_iprior38(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior38 {
        #[inline(always)]
        fn default() -> Iprior38 {
            Iprior38(0)
        }
    }
    impl core::fmt::Debug for Iprior38 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior38")
                .field("iprior38", &self.iprior38())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 39."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior39(pub u32);
    impl Iprior39 {
        #[doc = "RW,Interrupt priority for number 156-159."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior39(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 156-159."]
        #[inline(always)]
        pub const fn set_iprior39(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior39 {
        #[inline(always)]
        fn default() -> Iprior39 {
            Iprior39(0)
        }
    }
    impl core::fmt::Debug for Iprior39 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior39")
                .field("iprior39", &self.iprior39())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 4."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior4(pub u32);
    impl Iprior4 {
        #[doc = ">RW,Interrupt priority for number 16-19."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior4(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,Interrupt priority for number 16-19."]
        #[inline(always)]
        pub const fn set_iprior4(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior4 {
        #[inline(always)]
        fn default() -> Iprior4 {
            Iprior4(0)
        }
    }
    impl core::fmt::Debug for Iprior4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior4")
                .field("iprior4", &self.iprior4())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 40."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior40(pub u32);
    impl Iprior40 {
        #[doc = "RW,Interrupt priority for number 160-163."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior40(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 160-163."]
        #[inline(always)]
        pub const fn set_iprior40(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior40 {
        #[inline(always)]
        fn default() -> Iprior40 {
            Iprior40(0)
        }
    }
    impl core::fmt::Debug for Iprior40 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior40")
                .field("iprior40", &self.iprior40())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 41."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior41(pub u32);
    impl Iprior41 {
        #[doc = "RW,Interrupt priority for number 164-167."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior41(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 164-167."]
        #[inline(always)]
        pub const fn set_iprior41(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior41 {
        #[inline(always)]
        fn default() -> Iprior41 {
            Iprior41(0)
        }
    }
    impl core::fmt::Debug for Iprior41 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior41")
                .field("iprior41", &self.iprior41())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 42."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior42(pub u32);
    impl Iprior42 {
        #[doc = "RW,Interrupt priority for number 168-171."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior42(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 168-171."]
        #[inline(always)]
        pub const fn set_iprior42(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior42 {
        #[inline(always)]
        fn default() -> Iprior42 {
            Iprior42(0)
        }
    }
    impl core::fmt::Debug for Iprior42 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior42")
                .field("iprior42", &self.iprior42())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 43."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior43(pub u32);
    impl Iprior43 {
        #[doc = "RW,Interrupt priority for number 172-175."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior43(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 172-175."]
        #[inline(always)]
        pub const fn set_iprior43(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior43 {
        #[inline(always)]
        fn default() -> Iprior43 {
            Iprior43(0)
        }
    }
    impl core::fmt::Debug for Iprior43 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior43")
                .field("iprior43", &self.iprior43())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 44."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior44(pub u32);
    impl Iprior44 {
        #[doc = "RW,Interrupt priority for number 176-179."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior44(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 176-179."]
        #[inline(always)]
        pub const fn set_iprior44(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior44 {
        #[inline(always)]
        fn default() -> Iprior44 {
            Iprior44(0)
        }
    }
    impl core::fmt::Debug for Iprior44 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior44")
                .field("iprior44", &self.iprior44())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 45."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior45(pub u32);
    impl Iprior45 {
        #[doc = "RW,Interrupt priority for number 180-183."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior45(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 180-183."]
        #[inline(always)]
        pub const fn set_iprior45(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior45 {
        #[inline(always)]
        fn default() -> Iprior45 {
            Iprior45(0)
        }
    }
    impl core::fmt::Debug for Iprior45 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior45")
                .field("iprior45", &self.iprior45())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 46."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior46(pub u32);
    impl Iprior46 {
        #[doc = "RW,Interrupt priority for number 184-187."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior46(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 184-187."]
        #[inline(always)]
        pub const fn set_iprior46(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior46 {
        #[inline(always)]
        fn default() -> Iprior46 {
            Iprior46(0)
        }
    }
    impl core::fmt::Debug for Iprior46 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior46")
                .field("iprior46", &self.iprior46())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 47."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior47(pub u32);
    impl Iprior47 {
        #[doc = "RW,Interrupt priority for number 188-191."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior47(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 188-191."]
        #[inline(always)]
        pub const fn set_iprior47(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior47 {
        #[inline(always)]
        fn default() -> Iprior47 {
            Iprior47(0)
        }
    }
    impl core::fmt::Debug for Iprior47 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior47")
                .field("iprior47", &self.iprior47())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 48."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior48(pub u32);
    impl Iprior48 {
        #[doc = "RW,Interrupt priority for number 192-195."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior48(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 192-195."]
        #[inline(always)]
        pub const fn set_iprior48(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior48 {
        #[inline(always)]
        fn default() -> Iprior48 {
            Iprior48(0)
        }
    }
    impl core::fmt::Debug for Iprior48 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior48")
                .field("iprior48", &self.iprior48())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 49."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior49(pub u32);
    impl Iprior49 {
        #[doc = "RW,Interrupt priority for number 196-199."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior49(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 196-199."]
        #[inline(always)]
        pub const fn set_iprior49(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior49 {
        #[inline(always)]
        fn default() -> Iprior49 {
            Iprior49(0)
        }
    }
    impl core::fmt::Debug for Iprior49 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior49")
                .field("iprior49", &self.iprior49())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 5."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior5(pub u32);
    impl Iprior5 {
        #[doc = ">RW,Interrupt priority for number 20-23."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior5(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,Interrupt priority for number 20-23."]
        #[inline(always)]
        pub const fn set_iprior5(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior5 {
        #[inline(always)]
        fn default() -> Iprior5 {
            Iprior5(0)
        }
    }
    impl core::fmt::Debug for Iprior5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior5")
                .field("iprior5", &self.iprior5())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 50."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior50(pub u32);
    impl Iprior50 {
        #[doc = "RW,Interrupt priority for number 200-203."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior50(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 200-203."]
        #[inline(always)]
        pub const fn set_iprior50(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior50 {
        #[inline(always)]
        fn default() -> Iprior50 {
            Iprior50(0)
        }
    }
    impl core::fmt::Debug for Iprior50 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior50")
                .field("iprior50", &self.iprior50())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 51."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior51(pub u32);
    impl Iprior51 {
        #[doc = "RW,Interrupt priority for number 204-207."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior51(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 204-207."]
        #[inline(always)]
        pub const fn set_iprior51(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior51 {
        #[inline(always)]
        fn default() -> Iprior51 {
            Iprior51(0)
        }
    }
    impl core::fmt::Debug for Iprior51 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior51")
                .field("iprior51", &self.iprior51())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 52."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior52(pub u32);
    impl Iprior52 {
        #[doc = "RW,Interrupt priority for number 208-211."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior52(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 208-211."]
        #[inline(always)]
        pub const fn set_iprior52(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior52 {
        #[inline(always)]
        fn default() -> Iprior52 {
            Iprior52(0)
        }
    }
    impl core::fmt::Debug for Iprior52 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior52")
                .field("iprior52", &self.iprior52())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 53."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior53(pub u32);
    impl Iprior53 {
        #[doc = "RW,Interrupt priority for number 212-215."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior53(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 212-215."]
        #[inline(always)]
        pub const fn set_iprior53(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior53 {
        #[inline(always)]
        fn default() -> Iprior53 {
            Iprior53(0)
        }
    }
    impl core::fmt::Debug for Iprior53 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior53")
                .field("iprior53", &self.iprior53())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 54."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior54(pub u32);
    impl Iprior54 {
        #[doc = "RW,Interrupt priority for number 216-219."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior54(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 216-219."]
        #[inline(always)]
        pub const fn set_iprior54(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior54 {
        #[inline(always)]
        fn default() -> Iprior54 {
            Iprior54(0)
        }
    }
    impl core::fmt::Debug for Iprior54 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior54")
                .field("iprior54", &self.iprior54())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 55."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior55(pub u32);
    impl Iprior55 {
        #[doc = "RW,Interrupt priority for number 220-223."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior55(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 220-223."]
        #[inline(always)]
        pub const fn set_iprior55(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior55 {
        #[inline(always)]
        fn default() -> Iprior55 {
            Iprior55(0)
        }
    }
    impl core::fmt::Debug for Iprior55 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior55")
                .field("iprior55", &self.iprior55())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 56."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior56(pub u32);
    impl Iprior56 {
        #[doc = "RW,Interrupt priority for number 224-227."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior56(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 224-227."]
        #[inline(always)]
        pub const fn set_iprior56(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior56 {
        #[inline(always)]
        fn default() -> Iprior56 {
            Iprior56(0)
        }
    }
    impl core::fmt::Debug for Iprior56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior56")
                .field("iprior56", &self.iprior56())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 57."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior57(pub u32);
    impl Iprior57 {
        #[doc = "RW,Interrupt priority for number 228-231."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior57(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 228-231."]
        #[inline(always)]
        pub const fn set_iprior57(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior57 {
        #[inline(always)]
        fn default() -> Iprior57 {
            Iprior57(0)
        }
    }
    impl core::fmt::Debug for Iprior57 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior57")
                .field("iprior57", &self.iprior57())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 58."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior58(pub u32);
    impl Iprior58 {
        #[doc = "RW,Interrupt priority for number 232-235."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior58(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 232-235."]
        #[inline(always)]
        pub const fn set_iprior58(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior58 {
        #[inline(always)]
        fn default() -> Iprior58 {
            Iprior58(0)
        }
    }
    impl core::fmt::Debug for Iprior58 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior58")
                .field("iprior58", &self.iprior58())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 59."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior59(pub u32);
    impl Iprior59 {
        #[doc = "RW,Interrupt priority for number 236-239."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior59(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 236-239."]
        #[inline(always)]
        pub const fn set_iprior59(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior59 {
        #[inline(always)]
        fn default() -> Iprior59 {
            Iprior59(0)
        }
    }
    impl core::fmt::Debug for Iprior59 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior59")
                .field("iprior59", &self.iprior59())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 6."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior6(pub u32);
    impl Iprior6 {
        #[doc = ">RW,Interrupt priority for number 24-27."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior6(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,Interrupt priority for number 24-27."]
        #[inline(always)]
        pub const fn set_iprior6(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior6 {
        #[inline(always)]
        fn default() -> Iprior6 {
            Iprior6(0)
        }
    }
    impl core::fmt::Debug for Iprior6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior6")
                .field("iprior6", &self.iprior6())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 60."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior60(pub u32);
    impl Iprior60 {
        #[doc = "RW,Interrupt priority for number 240-243."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior60(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 240-243."]
        #[inline(always)]
        pub const fn set_iprior60(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior60 {
        #[inline(always)]
        fn default() -> Iprior60 {
            Iprior60(0)
        }
    }
    impl core::fmt::Debug for Iprior60 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior60")
                .field("iprior60", &self.iprior60())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 61."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior61(pub u32);
    impl Iprior61 {
        #[doc = "RW,Interrupt priority for number 244-247."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior61(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 244-247."]
        #[inline(always)]
        pub const fn set_iprior61(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior61 {
        #[inline(always)]
        fn default() -> Iprior61 {
            Iprior61(0)
        }
    }
    impl core::fmt::Debug for Iprior61 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior61")
                .field("iprior61", &self.iprior61())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 62."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior62(pub u32);
    impl Iprior62 {
        #[doc = "RW,Interrupt priority for number 248-251."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior62(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 248-251."]
        #[inline(always)]
        pub const fn set_iprior62(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior62 {
        #[inline(always)]
        fn default() -> Iprior62 {
            Iprior62(0)
        }
    }
    impl core::fmt::Debug for Iprior62 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior62")
                .field("iprior62", &self.iprior62())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 63."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior63(pub u32);
    impl Iprior63 {
        #[doc = "RW,Interrupt priority for number 252-255."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior63(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,Interrupt priority for number 252-255."]
        #[inline(always)]
        pub const fn set_iprior63(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior63 {
        #[inline(always)]
        fn default() -> Iprior63 {
            Iprior63(0)
        }
    }
    impl core::fmt::Debug for Iprior63 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior63")
                .field("iprior63", &self.iprior63())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 7."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior7(pub u32);
    impl Iprior7 {
        #[doc = ">RW,Interrupt priority for number 28-31."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior7(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,Interrupt priority for number 28-31."]
        #[inline(always)]
        pub const fn set_iprior7(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior7 {
        #[inline(always)]
        fn default() -> Iprior7 {
            Iprior7(0)
        }
    }
    impl core::fmt::Debug for Iprior7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior7")
                .field("iprior7", &self.iprior7())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 8."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior8(pub u32);
    impl Iprior8 {
        #[doc = ">RW,Interrupt priority for number 32-35."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior8(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,Interrupt priority for number 32-35."]
        #[inline(always)]
        pub const fn set_iprior8(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior8 {
        #[inline(always)]
        fn default() -> Iprior8 {
            Iprior8(0)
        }
    }
    impl core::fmt::Debug for Iprior8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior8")
                .field("iprior8", &self.iprior8())
                .finish()
        }
    }
    #[doc = "Interrupt Priority configuration Register 9."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior9(pub u32);
    impl Iprior9 {
        #[doc = ">RW,Interrupt priority for number 36-39."]
        #[must_use]
        #[inline(always)]
        pub const fn iprior9(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,Interrupt priority for number 36-39."]
        #[inline(always)]
        pub const fn set_iprior9(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior9 {
        #[inline(always)]
        fn default() -> Iprior9 {
            Iprior9(0)
        }
    }
    impl core::fmt::Debug for Iprior9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprior9")
                .field("iprior9", &self.iprior9())
                .finish()
        }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprr1(pub u32);
    impl Iprr1 {
        #[doc = "RW1,PENDRESET."]
        #[must_use]
        #[inline(always)]
        pub const fn pendreset(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "RW1,PENDRESET."]
        #[inline(always)]
        pub const fn set_pendreset(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Iprr1 {
        #[inline(always)]
        fn default() -> Iprr1 {
            Iprr1(0)
        }
    }
    impl core::fmt::Debug for Iprr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprr1")
                .field("pendreset", &self.pendreset())
                .finish()
        }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprr2(pub u32);
    impl Iprr2 {
        #[doc = "RW1,PENDRESET."]
        #[must_use]
        #[inline(always)]
        pub const fn pendreset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RW1,PENDRESET."]
        #[inline(always)]
        pub const fn set_pendreset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Iprr2 {
        #[inline(always)]
        fn default() -> Iprr2 {
            Iprr2(0)
        }
    }
    impl core::fmt::Debug for Iprr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Iprr2")
                .field("pendreset", &self.pendreset())
                .finish()
        }
    }
    #[doc = "Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipsr1(pub u32);
    impl Ipsr1 {
        #[doc = "RW1,PENDSET."]
        #[must_use]
        #[inline(always)]
        pub const fn pendset(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "RW1,PENDSET."]
        #[inline(always)]
        pub const fn set_pendset(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Ipsr1 {
        #[inline(always)]
        fn default() -> Ipsr1 {
            Ipsr1(0)
        }
    }
    impl core::fmt::Debug for Ipsr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipsr1")
                .field("pendset", &self.pendset())
                .finish()
        }
    }
    #[doc = "Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipsr2(pub u32);
    impl Ipsr2 {
        #[doc = "RW1,PENDSET."]
        #[must_use]
        #[inline(always)]
        pub const fn pendset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RW1,PENDSET."]
        #[inline(always)]
        pub const fn set_pendset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Ipsr2 {
        #[inline(always)]
        fn default() -> Ipsr2 {
            Ipsr2(0)
        }
    }
    impl core::fmt::Debug for Ipsr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ipsr2")
                .field("pendset", &self.pendset())
                .finish()
        }
    }
    #[doc = "Interrupt Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irer1(pub u32);
    impl Irer1 {
        #[doc = "RW1,INTRESET."]
        #[must_use]
        #[inline(always)]
        pub const fn intreset(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "RW1,INTRESET."]
        #[inline(always)]
        pub const fn set_intreset(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Irer1 {
        #[inline(always)]
        fn default() -> Irer1 {
            Irer1(0)
        }
    }
    impl core::fmt::Debug for Irer1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Irer1")
                .field("intreset", &self.intreset())
                .finish()
        }
    }
    #[doc = "Interrupt Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irer2(pub u32);
    impl Irer2 {
        #[doc = "RW1,INTRESET."]
        #[must_use]
        #[inline(always)]
        pub const fn intreset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RW1,INTRESET."]
        #[inline(always)]
        pub const fn set_intreset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Irer2 {
        #[inline(always)]
        fn default() -> Irer2 {
            Irer2(0)
        }
    }
    impl core::fmt::Debug for Irer2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Irer2")
                .field("intreset", &self.intreset())
                .finish()
        }
    }
    #[doc = "RO,Interrupt Status Register 1."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr1(pub u32);
    impl Isr1 {
        #[doc = "Interrupt ID Status."]
        #[must_use]
        #[inline(always)]
        pub const fn intensta(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub const fn set_intensta(&mut self, val: u32) {
            self.0 =
                (self.0 & !(0x000f_ffff << 12usize)) | (((val as u32) & 0x000f_ffff) << 12usize);
        }
    }
    impl Default for Isr1 {
        #[inline(always)]
        fn default() -> Isr1 {
            Isr1(0)
        }
    }
    impl core::fmt::Debug for Isr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr1")
                .field("intensta", &self.intensta())
                .finish()
        }
    }
    #[doc = "RO,Interrupt Status Register 2."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr2(pub u32);
    impl Isr2 {
        #[doc = "Interrupt ID Status."]
        #[must_use]
        #[inline(always)]
        pub const fn intensta(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub const fn set_intensta(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Isr2 {
        #[inline(always)]
        fn default() -> Isr2 {
            Isr2(0)
        }
    }
    impl core::fmt::Debug for Isr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Isr2")
                .field("intensta", &self.intensta())
                .finish()
        }
    }
    #[doc = "RW,Interrupt Priority Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ithresdr(pub u32);
    impl Ithresdr {
        #[doc = "RW,THRESHOLD."]
        #[must_use]
        #[inline(always)]
        pub const fn threshold(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW,THRESHOLD."]
        #[inline(always)]
        pub const fn set_threshold(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ithresdr {
        #[inline(always)]
        fn default() -> Ithresdr {
            Ithresdr(0)
        }
    }
    impl core::fmt::Debug for Ithresdr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ithresdr")
                .field("threshold", &self.threshold())
                .finish()
        }
    }
    #[doc = "System Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sctlr(pub u32);
    impl Sctlr {
        #[doc = "RW,SLEEPONEXIT."]
        #[must_use]
        #[inline(always)]
        pub const fn sleeponexit(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW,SLEEPONEXIT."]
        #[inline(always)]
        pub const fn set_sleeponexit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RW,SLEEPDEEP."]
        #[must_use]
        #[inline(always)]
        pub const fn sleepdeep(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW,SLEEPDEEP."]
        #[inline(always)]
        pub const fn set_sleepdeep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RW,WFITOWFE."]
        #[must_use]
        #[inline(always)]
        pub const fn wfitowfe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW,WFITOWFE."]
        #[inline(always)]
        pub const fn set_wfitowfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "RW,SEVONPEND."]
        #[must_use]
        #[inline(always)]
        pub const fn sevonpend(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW,SEVONPEND."]
        #[inline(always)]
        pub const fn set_sevonpend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "WO,SETEVENT."]
        #[must_use]
        #[inline(always)]
        pub const fn setevent(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "WO,SETEVENT."]
        #[inline(always)]
        pub const fn set_setevent(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Sctlr {
        #[inline(always)]
        fn default() -> Sctlr {
            Sctlr(0)
        }
    }
    impl core::fmt::Debug for Sctlr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sctlr")
                .field("sleeponexit", &self.sleeponexit())
                .field("sleepdeep", &self.sleepdeep())
                .field("wfitowfe", &self.wfitowfe())
                .field("sevonpend", &self.sevonpend())
                .field("setevent", &self.setevent())
                .finish()
        }
    }
}
