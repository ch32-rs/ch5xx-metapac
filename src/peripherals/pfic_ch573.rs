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
    #[doc = "RO,Interrupt Status Register."]
    #[inline(always)]
    pub const fn isr1(self) -> crate::common::Reg<regs::Isr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "RO,Interrupt Status Register."]
    #[inline(always)]
    pub const fn isr2(self) -> crate::common::Reg<regs::Isr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "RO,Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipr1(self) -> crate::common::Reg<regs::Ipr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "RO,Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipr2(self) -> crate::common::Reg<regs::Ipr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "RW,Interrupt Priority Register."]
    #[inline(always)]
    pub const fn ithresdr(self) -> crate::common::Reg<regs::Ithresdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "RW,Interrupt Fast Address Register."]
    #[inline(always)]
    pub const fn fibaddrr(self) -> crate::common::Reg<regs::Fibaddrr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Interrupt Config Register."]
    #[inline(always)]
    pub const fn cfgr(self) -> crate::common::Reg<regs::Cfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Interrupt Global Register."]
    #[inline(always)]
    pub const fn gisr(self) -> crate::common::Reg<regs::Gisr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Interrupt 0 address Register."]
    #[inline(always)]
    pub const fn fifoaddrr0(self) -> crate::common::Reg<regs::Fifoaddrr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Interrupt 1 address Register."]
    #[inline(always)]
    pub const fn fifoaddrr1(self) -> crate::common::Reg<regs::Fifoaddrr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Interrupt 2 address Register."]
    #[inline(always)]
    pub const fn fifoaddrr2(self) -> crate::common::Reg<regs::Fifoaddrr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "Interrupt 3 address Register."]
    #[inline(always)]
    pub const fn fifoaddrr3(self) -> crate::common::Reg<regs::Fifoaddrr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "Interrupt Setting Register."]
    #[inline(always)]
    pub const fn ienr1(self) -> crate::common::Reg<regs::Ienr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Interrupt Setting Register."]
    #[inline(always)]
    pub const fn ienr2(self) -> crate::common::Reg<regs::Ienr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Interrupt Clear Register."]
    #[inline(always)]
    pub const fn irer1(self) -> crate::common::Reg<regs::Irer1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Interrupt Clear Register."]
    #[inline(always)]
    pub const fn irer2(self) -> crate::common::Reg<regs::Irer2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipsr1(self) -> crate::common::Reg<regs::Ipsr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "Interrupt Pending Register."]
    #[inline(always)]
    pub const fn ipsr2(self) -> crate::common::Reg<regs::Ipsr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[inline(always)]
    pub const fn iprr1(self) -> crate::common::Reg<regs::Iprr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize) as _) }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[inline(always)]
    pub const fn iprr2(self) -> crate::common::Reg<regs::Iprr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0284usize) as _) }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[inline(always)]
    pub const fn iactr1(self) -> crate::common::Reg<regs::Iactr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[inline(always)]
    pub const fn iactr2(self) -> crate::common::Reg<regs::Iactr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior0(self) -> crate::common::Reg<regs::Iprior0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior1(self) -> crate::common::Reg<regs::Iprior1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior2(self) -> crate::common::Reg<regs::Iprior2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior3(self) -> crate::common::Reg<regs::Iprior3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0460usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior4(self) -> crate::common::Reg<regs::Iprior4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior5(self) -> crate::common::Reg<regs::Iprior5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04a0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior6(self) -> crate::common::Reg<regs::Iprior6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior7(self) -> crate::common::Reg<regs::Iprior7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04e0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior8(self) -> crate::common::Reg<regs::Iprior8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior9(self) -> crate::common::Reg<regs::Iprior9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior10(self) -> crate::common::Reg<regs::Iprior10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior11(self) -> crate::common::Reg<regs::Iprior11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0560usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior12(self) -> crate::common::Reg<regs::Iprior12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0580usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior13(self) -> crate::common::Reg<regs::Iprior13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior14(self) -> crate::common::Reg<regs::Iprior14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior15(self) -> crate::common::Reg<regs::Iprior15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05e0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior16(self) -> crate::common::Reg<regs::Iprior16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior17(self) -> crate::common::Reg<regs::Iprior17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0620usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior18(self) -> crate::common::Reg<regs::Iprior18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0640usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior19(self) -> crate::common::Reg<regs::Iprior19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0660usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior20(self) -> crate::common::Reg<regs::Iprior20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0680usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior21(self) -> crate::common::Reg<regs::Iprior21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06a0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior22(self) -> crate::common::Reg<regs::Iprior22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06c0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior23(self) -> crate::common::Reg<regs::Iprior23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06e0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior24(self) -> crate::common::Reg<regs::Iprior24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0700usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior25(self) -> crate::common::Reg<regs::Iprior25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0720usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior26(self) -> crate::common::Reg<regs::Iprior26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0740usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior27(self) -> crate::common::Reg<regs::Iprior27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0760usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior28(self) -> crate::common::Reg<regs::Iprior28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0780usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior29(self) -> crate::common::Reg<regs::Iprior29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07a0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior30(self) -> crate::common::Reg<regs::Iprior30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07c0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior31(self) -> crate::common::Reg<regs::Iprior31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07e0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior32(self) -> crate::common::Reg<regs::Iprior32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior33(self) -> crate::common::Reg<regs::Iprior33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0820usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior34(self) -> crate::common::Reg<regs::Iprior34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0840usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior35(self) -> crate::common::Reg<regs::Iprior35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0860usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior36(self) -> crate::common::Reg<regs::Iprior36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0880usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior37(self) -> crate::common::Reg<regs::Iprior37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08a0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior38(self) -> crate::common::Reg<regs::Iprior38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08c0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior39(self) -> crate::common::Reg<regs::Iprior39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08e0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior40(self) -> crate::common::Reg<regs::Iprior40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0900usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior41(self) -> crate::common::Reg<regs::Iprior41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0920usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior42(self) -> crate::common::Reg<regs::Iprior42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0940usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior43(self) -> crate::common::Reg<regs::Iprior43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0960usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior44(self) -> crate::common::Reg<regs::Iprior44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0980usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior45(self) -> crate::common::Reg<regs::Iprior45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09a0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior46(self) -> crate::common::Reg<regs::Iprior46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09c0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior47(self) -> crate::common::Reg<regs::Iprior47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x09e0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior48(self) -> crate::common::Reg<regs::Iprior48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a00usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior49(self) -> crate::common::Reg<regs::Iprior49, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a20usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior50(self) -> crate::common::Reg<regs::Iprior50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a40usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior51(self) -> crate::common::Reg<regs::Iprior51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a60usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior52(self) -> crate::common::Reg<regs::Iprior52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0a80usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior53(self) -> crate::common::Reg<regs::Iprior53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0aa0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior54(self) -> crate::common::Reg<regs::Iprior54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ad0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior55(self) -> crate::common::Reg<regs::Iprior55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ae0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior56(self) -> crate::common::Reg<regs::Iprior56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b00usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior57(self) -> crate::common::Reg<regs::Iprior57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b20usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior58(self) -> crate::common::Reg<regs::Iprior58, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b40usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior59(self) -> crate::common::Reg<regs::Iprior59, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b60usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior60(self) -> crate::common::Reg<regs::Iprior60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0b80usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior61(self) -> crate::common::Reg<regs::Iprior61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0ba0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior62(self) -> crate::common::Reg<regs::Iprior62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0be0usize) as _) }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[inline(always)]
    pub const fn iprior63(self) -> crate::common::Reg<regs::Iprior63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0c00usize) as _) }
    }
    #[doc = "System Control Register."]
    #[inline(always)]
    pub const fn sctlr(self) -> crate::common::Reg<regs::Sctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d10usize) as _) }
    }
    #[doc = "System Control Register."]
    #[inline(always)]
    pub const fn vtctlr(self) -> crate::common::Reg<regs::Vtctlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0d14usize) as _) }
    }
}
pub mod regs {
    #[doc = "Interrupt Config Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfgr(pub u32);
    impl Cfgr {
        #[doc = "RW,HWSTKCTRL."]
        #[inline(always)]
        pub const fn hwstkctrl(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RW,HWSTKCTRL."]
        #[inline(always)]
        pub fn set_hwstkctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "RW,NESTCTRL."]
        #[inline(always)]
        pub const fn nestctrl(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW,NESTCTRL."]
        #[inline(always)]
        pub fn set_nestctrl(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "WO,NMISET."]
        #[inline(always)]
        pub const fn nmiset(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "WO,NMISET."]
        #[inline(always)]
        pub fn set_nmiset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "WO,NMIRESET."]
        #[inline(always)]
        pub const fn nmireset(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "WO,NMIRESET."]
        #[inline(always)]
        pub fn set_nmireset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "WO,EXCSET."]
        #[inline(always)]
        pub const fn excset(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "WO,EXCSET."]
        #[inline(always)]
        pub fn set_excset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "WO,EXCRESET."]
        #[inline(always)]
        pub const fn excreset(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "WO,EXCRESET."]
        #[inline(always)]
        pub fn set_excreset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "WO,PFICRSET."]
        #[inline(always)]
        pub const fn pficreset(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "WO,PFICRSET."]
        #[inline(always)]
        pub fn set_pficreset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "WO,SYSRESET."]
        #[inline(always)]
        pub const fn sysreset(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "WO,SYSRESET."]
        #[inline(always)]
        pub fn set_sysreset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "WO,KEYCODE."]
        #[inline(always)]
        pub const fn keycode(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "WO,KEYCODE."]
        #[inline(always)]
        pub fn set_keycode(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Cfgr {
        #[inline(always)]
        fn default() -> Cfgr {
            Cfgr(0)
        }
    }
    #[doc = "RW,Interrupt Fast Address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fibaddrr(pub u32);
    impl Fibaddrr {
        #[doc = "BASEADDR."]
        #[inline(always)]
        pub const fn baseaddr(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x0f;
            val as u8
        }
        #[doc = "BASEADDR."]
        #[inline(always)]
        pub fn set_baseaddr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
        }
    }
    impl Default for Fibaddrr {
        #[inline(always)]
        fn default() -> Fibaddrr {
            Fibaddrr(0)
        }
    }
    #[doc = "Interrupt 0 address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifoaddrr0(pub u32);
    impl Fifoaddrr0 {
        #[doc = "RW,OFFADDR0."]
        #[inline(always)]
        pub const fn offaddr0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "RW,OFFADDR0."]
        #[inline(always)]
        pub fn set_offaddr0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "RW,IRQID0."]
        #[inline(always)]
        pub const fn irqid0(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "RW,IRQID0."]
        #[inline(always)]
        pub fn set_irqid0(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Fifoaddrr0 {
        #[inline(always)]
        fn default() -> Fifoaddrr0 {
            Fifoaddrr0(0)
        }
    }
    #[doc = "Interrupt 1 address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifoaddrr1(pub u32);
    impl Fifoaddrr1 {
        #[doc = "RW,OFFADDR1."]
        #[inline(always)]
        pub const fn offaddr1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "RW,OFFADDR1."]
        #[inline(always)]
        pub fn set_offaddr1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "RW,IRQID1."]
        #[inline(always)]
        pub const fn irqid1(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "RW,IRQID1."]
        #[inline(always)]
        pub fn set_irqid1(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Fifoaddrr1 {
        #[inline(always)]
        fn default() -> Fifoaddrr1 {
            Fifoaddrr1(0)
        }
    }
    #[doc = "Interrupt 2 address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifoaddrr2(pub u32);
    impl Fifoaddrr2 {
        #[doc = "RW,OFFADDR2."]
        #[inline(always)]
        pub const fn offaddr2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "RW,OFFADDR2."]
        #[inline(always)]
        pub fn set_offaddr2(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "RW,IRQID2."]
        #[inline(always)]
        pub const fn irqid2(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "RW,IRQID2."]
        #[inline(always)]
        pub fn set_irqid2(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Fifoaddrr2 {
        #[inline(always)]
        fn default() -> Fifoaddrr2 {
            Fifoaddrr2(0)
        }
    }
    #[doc = "Interrupt 3 address Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Fifoaddrr3(pub u32);
    impl Fifoaddrr3 {
        #[doc = "RW,OFFADDR3."]
        #[inline(always)]
        pub const fn offaddr3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "RW,OFFADDR3."]
        #[inline(always)]
        pub fn set_offaddr3(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "RW,IRQID3."]
        #[inline(always)]
        pub const fn irqid3(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "RW,IRQID3."]
        #[inline(always)]
        pub fn set_irqid3(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Fifoaddrr3 {
        #[inline(always)]
        fn default() -> Fifoaddrr3 {
            Fifoaddrr3(0)
        }
    }
    #[doc = "Interrupt Global Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gisr(pub u32);
    impl Gisr {
        #[doc = "RO,NESTSTA."]
        #[inline(always)]
        pub const fn neststa(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RO,NESTSTA."]
        #[inline(always)]
        pub fn set_neststa(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "RO,GACTSTA."]
        #[inline(always)]
        pub const fn gactsta(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "RO,GACTSTA."]
        #[inline(always)]
        pub fn set_gactsta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "RO,GPENDSTA."]
        #[inline(always)]
        pub const fn gpendsta(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "RO,GPENDSTA."]
        #[inline(always)]
        pub fn set_gpendsta(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Gisr {
        #[inline(always)]
        fn default() -> Gisr {
            Gisr(0)
        }
    }
    #[doc = "Interrupt ACTIVE Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iactr1(pub u32);
    impl Iactr1 {
        #[doc = "RW1,IACTS."]
        #[inline(always)]
        pub const fn iacts(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "RW1,IACTS."]
        #[inline(always)]
        pub fn set_iacts(&mut self, val: u32) {
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
    #[doc = "Interrupt ACTIVE Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iactr2(pub u32);
    impl Iactr2 {
        #[doc = "RW1,IACTS."]
        #[inline(always)]
        pub const fn iacts(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RW1,IACTS."]
        #[inline(always)]
        pub fn set_iacts(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Iactr2 {
        #[inline(always)]
        fn default() -> Iactr2 {
            Iactr2(0)
        }
    }
    #[doc = "Interrupt Setting Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ienr1(pub u32);
    impl Ienr1 {
        #[doc = "RW1,INTEN."]
        #[inline(always)]
        pub const fn inten(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "RW1,INTEN."]
        #[inline(always)]
        pub fn set_inten(&mut self, val: u32) {
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
    #[doc = "Interrupt Setting Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ienr2(pub u32);
    impl Ienr2 {
        #[doc = "RW1,INTEN."]
        #[inline(always)]
        pub const fn inten(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RW1,INTEN."]
        #[inline(always)]
        pub fn set_inten(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Ienr2 {
        #[inline(always)]
        fn default() -> Ienr2 {
            Ienr2(0)
        }
    }
    #[doc = "RO,Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipr1(pub u32);
    impl Ipr1 {
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub const fn pendsta(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub fn set_pendsta(&mut self, val: u32) {
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
    #[doc = "RO,Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipr2(pub u32);
    impl Ipr2 {
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub const fn pendsta(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "PENDSTA."]
        #[inline(always)]
        pub fn set_pendsta(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Ipr2 {
        #[inline(always)]
        fn default() -> Ipr2 {
            Ipr2(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior0(pub u32);
    impl Iprior0 {
        #[doc = "RW,IPRIOR0."]
        #[inline(always)]
        pub const fn iprior0(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR0."]
        #[inline(always)]
        pub fn set_iprior0(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior0 {
        #[inline(always)]
        fn default() -> Iprior0 {
            Iprior0(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior1(pub u32);
    impl Iprior1 {
        #[doc = ">RW,IPRIOR1."]
        #[inline(always)]
        pub const fn iprior1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,IPRIOR1."]
        #[inline(always)]
        pub fn set_iprior1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior1 {
        #[inline(always)]
        fn default() -> Iprior1 {
            Iprior1(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior10(pub u32);
    impl Iprior10 {
        #[doc = ">RW,IPRIOR10."]
        #[inline(always)]
        pub const fn iprior10(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,IPRIOR10."]
        #[inline(always)]
        pub fn set_iprior10(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior10 {
        #[inline(always)]
        fn default() -> Iprior10 {
            Iprior10(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior11(pub u32);
    impl Iprior11 {
        #[doc = "RW,IPRIOR11."]
        #[inline(always)]
        pub const fn iprior11(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR11."]
        #[inline(always)]
        pub fn set_iprior11(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior11 {
        #[inline(always)]
        fn default() -> Iprior11 {
            Iprior11(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior12(pub u32);
    impl Iprior12 {
        #[doc = "RW,IPRIOR12."]
        #[inline(always)]
        pub const fn iprior12(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR12."]
        #[inline(always)]
        pub fn set_iprior12(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior12 {
        #[inline(always)]
        fn default() -> Iprior12 {
            Iprior12(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior13(pub u32);
    impl Iprior13 {
        #[doc = "RW,IPRIOR13."]
        #[inline(always)]
        pub const fn iprior13(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR13."]
        #[inline(always)]
        pub fn set_iprior13(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior13 {
        #[inline(always)]
        fn default() -> Iprior13 {
            Iprior13(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior14(pub u32);
    impl Iprior14 {
        #[doc = "RW,IPRIOR14."]
        #[inline(always)]
        pub const fn iprior14(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR14."]
        #[inline(always)]
        pub fn set_iprior14(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior14 {
        #[inline(always)]
        fn default() -> Iprior14 {
            Iprior14(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior15(pub u32);
    impl Iprior15 {
        #[doc = "RW,IPRIOR15."]
        #[inline(always)]
        pub const fn iprior15(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR15."]
        #[inline(always)]
        pub fn set_iprior15(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior15 {
        #[inline(always)]
        fn default() -> Iprior15 {
            Iprior15(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior16(pub u32);
    impl Iprior16 {
        #[doc = "RW,IPRIOR16."]
        #[inline(always)]
        pub const fn iprior16(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR16."]
        #[inline(always)]
        pub fn set_iprior16(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior16 {
        #[inline(always)]
        fn default() -> Iprior16 {
            Iprior16(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior17(pub u32);
    impl Iprior17 {
        #[doc = "RW,IPRIOR17."]
        #[inline(always)]
        pub const fn iprior17(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR17."]
        #[inline(always)]
        pub fn set_iprior17(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior17 {
        #[inline(always)]
        fn default() -> Iprior17 {
            Iprior17(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior18(pub u32);
    impl Iprior18 {
        #[doc = "RW,IPRIOR18."]
        #[inline(always)]
        pub const fn iprior18(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR18."]
        #[inline(always)]
        pub fn set_iprior18(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior18 {
        #[inline(always)]
        fn default() -> Iprior18 {
            Iprior18(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior19(pub u32);
    impl Iprior19 {
        #[doc = "RW,IPRIOR19."]
        #[inline(always)]
        pub const fn iprior19(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR19."]
        #[inline(always)]
        pub fn set_iprior19(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior19 {
        #[inline(always)]
        fn default() -> Iprior19 {
            Iprior19(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior2(pub u32);
    impl Iprior2 {
        #[doc = ">RW,IPRIOR2."]
        #[inline(always)]
        pub const fn iprior2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,IPRIOR2."]
        #[inline(always)]
        pub fn set_iprior2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior2 {
        #[inline(always)]
        fn default() -> Iprior2 {
            Iprior2(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior20(pub u32);
    impl Iprior20 {
        #[doc = "RW,IPRIOR20."]
        #[inline(always)]
        pub const fn iprior20(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR20."]
        #[inline(always)]
        pub fn set_iprior20(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior20 {
        #[inline(always)]
        fn default() -> Iprior20 {
            Iprior20(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior21(pub u32);
    impl Iprior21 {
        #[doc = "RW,IPRIOR21."]
        #[inline(always)]
        pub const fn iprior21(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR21."]
        #[inline(always)]
        pub fn set_iprior21(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior21 {
        #[inline(always)]
        fn default() -> Iprior21 {
            Iprior21(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior22(pub u32);
    impl Iprior22 {
        #[doc = "RW,IPRIOR22."]
        #[inline(always)]
        pub const fn iprior22(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR22."]
        #[inline(always)]
        pub fn set_iprior22(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior22 {
        #[inline(always)]
        fn default() -> Iprior22 {
            Iprior22(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior23(pub u32);
    impl Iprior23 {
        #[doc = "RW,IPRIOR23."]
        #[inline(always)]
        pub const fn iprior23(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR23."]
        #[inline(always)]
        pub fn set_iprior23(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior23 {
        #[inline(always)]
        fn default() -> Iprior23 {
            Iprior23(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior24(pub u32);
    impl Iprior24 {
        #[doc = "RW,IPRIOR24."]
        #[inline(always)]
        pub const fn iprior24(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR24."]
        #[inline(always)]
        pub fn set_iprior24(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior24 {
        #[inline(always)]
        fn default() -> Iprior24 {
            Iprior24(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior25(pub u32);
    impl Iprior25 {
        #[doc = "RW,IPRIOR25."]
        #[inline(always)]
        pub const fn iprior25(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR25."]
        #[inline(always)]
        pub fn set_iprior25(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior25 {
        #[inline(always)]
        fn default() -> Iprior25 {
            Iprior25(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior26(pub u32);
    impl Iprior26 {
        #[doc = "RW,IPRIOR26."]
        #[inline(always)]
        pub const fn iprior26(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR26."]
        #[inline(always)]
        pub fn set_iprior26(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior26 {
        #[inline(always)]
        fn default() -> Iprior26 {
            Iprior26(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior27(pub u32);
    impl Iprior27 {
        #[doc = "RW,IPRIOR27."]
        #[inline(always)]
        pub const fn iprior27(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR27."]
        #[inline(always)]
        pub fn set_iprior27(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior27 {
        #[inline(always)]
        fn default() -> Iprior27 {
            Iprior27(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior28(pub u32);
    impl Iprior28 {
        #[doc = "RW,IPRIOR28."]
        #[inline(always)]
        pub const fn iprior28(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR28."]
        #[inline(always)]
        pub fn set_iprior28(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior28 {
        #[inline(always)]
        fn default() -> Iprior28 {
            Iprior28(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior29(pub u32);
    impl Iprior29 {
        #[doc = "RW,IPRIOR29."]
        #[inline(always)]
        pub const fn iprior29(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR29."]
        #[inline(always)]
        pub fn set_iprior29(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior29 {
        #[inline(always)]
        fn default() -> Iprior29 {
            Iprior29(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior3(pub u32);
    impl Iprior3 {
        #[doc = ">RW,IPRIOR3."]
        #[inline(always)]
        pub const fn iprior3(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,IPRIOR3."]
        #[inline(always)]
        pub fn set_iprior3(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior3 {
        #[inline(always)]
        fn default() -> Iprior3 {
            Iprior3(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior30(pub u32);
    impl Iprior30 {
        #[doc = "RW,IPRIOR30."]
        #[inline(always)]
        pub const fn iprior30(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR30."]
        #[inline(always)]
        pub fn set_iprior30(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior30 {
        #[inline(always)]
        fn default() -> Iprior30 {
            Iprior30(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior31(pub u32);
    impl Iprior31 {
        #[doc = "RW,IPRIOR31."]
        #[inline(always)]
        pub const fn iprior31(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR31."]
        #[inline(always)]
        pub fn set_iprior31(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior31 {
        #[inline(always)]
        fn default() -> Iprior31 {
            Iprior31(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior32(pub u32);
    impl Iprior32 {
        #[doc = "RW,IPRIOR32."]
        #[inline(always)]
        pub const fn iprior32(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR32."]
        #[inline(always)]
        pub fn set_iprior32(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior32 {
        #[inline(always)]
        fn default() -> Iprior32 {
            Iprior32(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior33(pub u32);
    impl Iprior33 {
        #[doc = "RW,IPRIOR33."]
        #[inline(always)]
        pub const fn iprior33(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR33."]
        #[inline(always)]
        pub fn set_iprior33(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior33 {
        #[inline(always)]
        fn default() -> Iprior33 {
            Iprior33(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior34(pub u32);
    impl Iprior34 {
        #[doc = "RW,IPRIOR34."]
        #[inline(always)]
        pub const fn iprior34(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR34."]
        #[inline(always)]
        pub fn set_iprior34(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior34 {
        #[inline(always)]
        fn default() -> Iprior34 {
            Iprior34(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior35(pub u32);
    impl Iprior35 {
        #[doc = "RW,IPRIOR35."]
        #[inline(always)]
        pub const fn iprior35(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR35."]
        #[inline(always)]
        pub fn set_iprior35(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior35 {
        #[inline(always)]
        fn default() -> Iprior35 {
            Iprior35(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior36(pub u32);
    impl Iprior36 {
        #[doc = "RW,IPRIOR36."]
        #[inline(always)]
        pub const fn iprior36(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR36."]
        #[inline(always)]
        pub fn set_iprior36(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior36 {
        #[inline(always)]
        fn default() -> Iprior36 {
            Iprior36(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior37(pub u32);
    impl Iprior37 {
        #[doc = "RW,IPRIOR37."]
        #[inline(always)]
        pub const fn iprior37(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR37."]
        #[inline(always)]
        pub fn set_iprior37(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior37 {
        #[inline(always)]
        fn default() -> Iprior37 {
            Iprior37(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior38(pub u32);
    impl Iprior38 {
        #[doc = "RW,IPRIOR38."]
        #[inline(always)]
        pub const fn iprior38(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR38."]
        #[inline(always)]
        pub fn set_iprior38(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior38 {
        #[inline(always)]
        fn default() -> Iprior38 {
            Iprior38(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior39(pub u32);
    impl Iprior39 {
        #[doc = "RW,IPRIOR39."]
        #[inline(always)]
        pub const fn iprior39(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR39."]
        #[inline(always)]
        pub fn set_iprior39(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior39 {
        #[inline(always)]
        fn default() -> Iprior39 {
            Iprior39(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior4(pub u32);
    impl Iprior4 {
        #[doc = ">RW,IPRIOR4."]
        #[inline(always)]
        pub const fn iprior4(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,IPRIOR4."]
        #[inline(always)]
        pub fn set_iprior4(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior4 {
        #[inline(always)]
        fn default() -> Iprior4 {
            Iprior4(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior40(pub u32);
    impl Iprior40 {
        #[doc = "RW,IPRIOR40."]
        #[inline(always)]
        pub const fn iprior40(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR40."]
        #[inline(always)]
        pub fn set_iprior40(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior40 {
        #[inline(always)]
        fn default() -> Iprior40 {
            Iprior40(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior41(pub u32);
    impl Iprior41 {
        #[doc = "RW,IPRIOR41."]
        #[inline(always)]
        pub const fn iprior41(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR41."]
        #[inline(always)]
        pub fn set_iprior41(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior41 {
        #[inline(always)]
        fn default() -> Iprior41 {
            Iprior41(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior42(pub u32);
    impl Iprior42 {
        #[doc = "RW,IPRIOR42."]
        #[inline(always)]
        pub const fn iprior42(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR42."]
        #[inline(always)]
        pub fn set_iprior42(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior42 {
        #[inline(always)]
        fn default() -> Iprior42 {
            Iprior42(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior43(pub u32);
    impl Iprior43 {
        #[doc = "RW,IPRIOR43."]
        #[inline(always)]
        pub const fn iprior43(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR43."]
        #[inline(always)]
        pub fn set_iprior43(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior43 {
        #[inline(always)]
        fn default() -> Iprior43 {
            Iprior43(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior44(pub u32);
    impl Iprior44 {
        #[doc = "RW,IPRIOR44."]
        #[inline(always)]
        pub const fn iprior44(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR44."]
        #[inline(always)]
        pub fn set_iprior44(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior44 {
        #[inline(always)]
        fn default() -> Iprior44 {
            Iprior44(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior45(pub u32);
    impl Iprior45 {
        #[doc = "RW,IPRIOR45."]
        #[inline(always)]
        pub const fn iprior45(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR45."]
        #[inline(always)]
        pub fn set_iprior45(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior45 {
        #[inline(always)]
        fn default() -> Iprior45 {
            Iprior45(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior46(pub u32);
    impl Iprior46 {
        #[doc = "RW,IPRIOR46."]
        #[inline(always)]
        pub const fn iprior46(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR46."]
        #[inline(always)]
        pub fn set_iprior46(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior46 {
        #[inline(always)]
        fn default() -> Iprior46 {
            Iprior46(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior47(pub u32);
    impl Iprior47 {
        #[doc = "RW,IPRIOR47."]
        #[inline(always)]
        pub const fn iprior47(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR47."]
        #[inline(always)]
        pub fn set_iprior47(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior47 {
        #[inline(always)]
        fn default() -> Iprior47 {
            Iprior47(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior48(pub u32);
    impl Iprior48 {
        #[doc = "RW,IPRIOR48."]
        #[inline(always)]
        pub const fn iprior48(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR48."]
        #[inline(always)]
        pub fn set_iprior48(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior48 {
        #[inline(always)]
        fn default() -> Iprior48 {
            Iprior48(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior49(pub u32);
    impl Iprior49 {
        #[doc = "RW,IPRIOR49."]
        #[inline(always)]
        pub const fn iprior49(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR49."]
        #[inline(always)]
        pub fn set_iprior49(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior49 {
        #[inline(always)]
        fn default() -> Iprior49 {
            Iprior49(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior5(pub u32);
    impl Iprior5 {
        #[doc = ">RW,IPRIOR5."]
        #[inline(always)]
        pub const fn iprior5(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,IPRIOR5."]
        #[inline(always)]
        pub fn set_iprior5(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior5 {
        #[inline(always)]
        fn default() -> Iprior5 {
            Iprior5(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior50(pub u32);
    impl Iprior50 {
        #[doc = "RW,IPRIOR50."]
        #[inline(always)]
        pub const fn iprior50(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR50."]
        #[inline(always)]
        pub fn set_iprior50(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior50 {
        #[inline(always)]
        fn default() -> Iprior50 {
            Iprior50(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior51(pub u32);
    impl Iprior51 {
        #[doc = "RW,IPRIOR51."]
        #[inline(always)]
        pub const fn iprior51(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR51."]
        #[inline(always)]
        pub fn set_iprior51(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior51 {
        #[inline(always)]
        fn default() -> Iprior51 {
            Iprior51(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior52(pub u32);
    impl Iprior52 {
        #[doc = "RW,IPRIOR52."]
        #[inline(always)]
        pub const fn iprior52(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR52."]
        #[inline(always)]
        pub fn set_iprior52(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior52 {
        #[inline(always)]
        fn default() -> Iprior52 {
            Iprior52(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior53(pub u32);
    impl Iprior53 {
        #[doc = "RW,IPRIOR53."]
        #[inline(always)]
        pub const fn iprior53(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR53."]
        #[inline(always)]
        pub fn set_iprior53(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior53 {
        #[inline(always)]
        fn default() -> Iprior53 {
            Iprior53(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior54(pub u32);
    impl Iprior54 {
        #[doc = "RW,IPRIOR54."]
        #[inline(always)]
        pub const fn iprior54(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR54."]
        #[inline(always)]
        pub fn set_iprior54(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior54 {
        #[inline(always)]
        fn default() -> Iprior54 {
            Iprior54(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior55(pub u32);
    impl Iprior55 {
        #[doc = "RW,IPRIOR55."]
        #[inline(always)]
        pub const fn iprior55(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR55."]
        #[inline(always)]
        pub fn set_iprior55(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior55 {
        #[inline(always)]
        fn default() -> Iprior55 {
            Iprior55(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior56(pub u32);
    impl Iprior56 {
        #[doc = "RW,IPRIOR56."]
        #[inline(always)]
        pub const fn iprior56(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR56."]
        #[inline(always)]
        pub fn set_iprior56(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior56 {
        #[inline(always)]
        fn default() -> Iprior56 {
            Iprior56(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior57(pub u32);
    impl Iprior57 {
        #[doc = "RW,IPRIOR57."]
        #[inline(always)]
        pub const fn iprior57(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR57."]
        #[inline(always)]
        pub fn set_iprior57(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior57 {
        #[inline(always)]
        fn default() -> Iprior57 {
            Iprior57(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior58(pub u32);
    impl Iprior58 {
        #[doc = "RW,IPRIOR58."]
        #[inline(always)]
        pub const fn iprior58(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR58."]
        #[inline(always)]
        pub fn set_iprior58(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior58 {
        #[inline(always)]
        fn default() -> Iprior58 {
            Iprior58(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior59(pub u32);
    impl Iprior59 {
        #[doc = "RW,IPRIOR59."]
        #[inline(always)]
        pub const fn iprior59(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR59."]
        #[inline(always)]
        pub fn set_iprior59(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior59 {
        #[inline(always)]
        fn default() -> Iprior59 {
            Iprior59(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior6(pub u32);
    impl Iprior6 {
        #[doc = ">RW,IPRIOR6."]
        #[inline(always)]
        pub const fn iprior6(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,IPRIOR6."]
        #[inline(always)]
        pub fn set_iprior6(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior6 {
        #[inline(always)]
        fn default() -> Iprior6 {
            Iprior6(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior60(pub u32);
    impl Iprior60 {
        #[doc = "RW,IPRIOR60."]
        #[inline(always)]
        pub const fn iprior60(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR60."]
        #[inline(always)]
        pub fn set_iprior60(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior60 {
        #[inline(always)]
        fn default() -> Iprior60 {
            Iprior60(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior61(pub u32);
    impl Iprior61 {
        #[doc = "RW,IPRIOR61."]
        #[inline(always)]
        pub const fn iprior61(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR61."]
        #[inline(always)]
        pub fn set_iprior61(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior61 {
        #[inline(always)]
        fn default() -> Iprior61 {
            Iprior61(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior62(pub u32);
    impl Iprior62 {
        #[doc = "RW,IPRIOR62."]
        #[inline(always)]
        pub const fn iprior62(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR62."]
        #[inline(always)]
        pub fn set_iprior62(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior62 {
        #[inline(always)]
        fn default() -> Iprior62 {
            Iprior62(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior63(pub u32);
    impl Iprior63 {
        #[doc = "RW,IPRIOR63."]
        #[inline(always)]
        pub const fn iprior63(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "RW,IPRIOR63."]
        #[inline(always)]
        pub fn set_iprior63(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior63 {
        #[inline(always)]
        fn default() -> Iprior63 {
            Iprior63(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior7(pub u32);
    impl Iprior7 {
        #[doc = ">RW,IPRIOR7."]
        #[inline(always)]
        pub const fn iprior7(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,IPRIOR7."]
        #[inline(always)]
        pub fn set_iprior7(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior7 {
        #[inline(always)]
        fn default() -> Iprior7 {
            Iprior7(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior8(pub u32);
    impl Iprior8 {
        #[doc = ">RW,IPRIOR8."]
        #[inline(always)]
        pub const fn iprior8(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,IPRIOR8."]
        #[inline(always)]
        pub fn set_iprior8(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior8 {
        #[inline(always)]
        fn default() -> Iprior8 {
            Iprior8(0)
        }
    }
    #[doc = "Interrupt Priority configuration Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprior9(pub u32);
    impl Iprior9 {
        #[doc = ">RW,IPRIOR9."]
        #[inline(always)]
        pub const fn iprior9(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = ">RW,IPRIOR9."]
        #[inline(always)]
        pub fn set_iprior9(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Iprior9 {
        #[inline(always)]
        fn default() -> Iprior9 {
            Iprior9(0)
        }
    }
    #[doc = "Interrupt Pending Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprr1(pub u32);
    impl Iprr1 {
        #[doc = "RW1,PENDRESET."]
        #[inline(always)]
        pub const fn pendreset(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "RW1,PENDRESET."]
        #[inline(always)]
        pub fn set_pendreset(&mut self, val: u32) {
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
    #[doc = "Interrupt Pending Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Iprr2(pub u32);
    impl Iprr2 {
        #[doc = "RW1,PENDRESET."]
        #[inline(always)]
        pub const fn pendreset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RW1,PENDRESET."]
        #[inline(always)]
        pub fn set_pendreset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Iprr2 {
        #[inline(always)]
        fn default() -> Iprr2 {
            Iprr2(0)
        }
    }
    #[doc = "Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipsr1(pub u32);
    impl Ipsr1 {
        #[doc = "RW1,PENDSET."]
        #[inline(always)]
        pub const fn pendset(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "RW1,PENDSET."]
        #[inline(always)]
        pub fn set_pendset(&mut self, val: u32) {
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
    #[doc = "Interrupt Pending Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ipsr2(pub u32);
    impl Ipsr2 {
        #[doc = "RW1,PENDSET."]
        #[inline(always)]
        pub const fn pendset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RW1,PENDSET."]
        #[inline(always)]
        pub fn set_pendset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Ipsr2 {
        #[inline(always)]
        fn default() -> Ipsr2 {
            Ipsr2(0)
        }
    }
    #[doc = "Interrupt Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irer1(pub u32);
    impl Irer1 {
        #[doc = "RW1,INTRESET."]
        #[inline(always)]
        pub const fn intreset(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "RW1,INTRESET."]
        #[inline(always)]
        pub fn set_intreset(&mut self, val: u32) {
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
    #[doc = "Interrupt Clear Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Irer2(pub u32);
    impl Irer2 {
        #[doc = "RW1,INTRESET."]
        #[inline(always)]
        pub const fn intreset(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "RW1,INTRESET."]
        #[inline(always)]
        pub fn set_intreset(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Irer2 {
        #[inline(always)]
        fn default() -> Irer2 {
            Irer2(0)
        }
    }
    #[doc = "RO,Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr1(pub u32);
    impl Isr1 {
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub const fn intensta(&self) -> u32 {
            let val = (self.0 >> 12usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub fn set_intensta(&mut self, val: u32) {
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
    #[doc = "RO,Interrupt Status Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr2(pub u32);
    impl Isr2 {
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub const fn intensta(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Interrupt ID Status."]
        #[inline(always)]
        pub fn set_intensta(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
    }
    impl Default for Isr2 {
        #[inline(always)]
        fn default() -> Isr2 {
            Isr2(0)
        }
    }
    #[doc = "RW,Interrupt Priority Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ithresdr(pub u32);
    impl Ithresdr {
        #[doc = "RW,THRESHOLD."]
        #[inline(always)]
        pub const fn threshold(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RW,THRESHOLD."]
        #[inline(always)]
        pub fn set_threshold(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Ithresdr {
        #[inline(always)]
        fn default() -> Ithresdr {
            Ithresdr(0)
        }
    }
    #[doc = "System Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sctlr(pub u32);
    impl Sctlr {
        #[doc = "RW,SLEEPONEXIT."]
        #[inline(always)]
        pub const fn sleeponexit(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "RW,SLEEPONEXIT."]
        #[inline(always)]
        pub fn set_sleeponexit(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "RW,SLEEPDEEP."]
        #[inline(always)]
        pub const fn sleepdeep(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "RW,SLEEPDEEP."]
        #[inline(always)]
        pub fn set_sleepdeep(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "RW,WFITOWFE."]
        #[inline(always)]
        pub const fn wfitowfe(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "RW,WFITOWFE."]
        #[inline(always)]
        pub fn set_wfitowfe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "RW,SEVONPEND."]
        #[inline(always)]
        pub const fn sevonpend(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "RW,SEVONPEND."]
        #[inline(always)]
        pub fn set_sevonpend(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "WO,SETEVENT."]
        #[inline(always)]
        pub const fn setevent(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "WO,SETEVENT."]
        #[inline(always)]
        pub fn set_setevent(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Sctlr {
        #[inline(always)]
        fn default() -> Sctlr {
            Sctlr(0)
        }
    }
    #[doc = "System Control Register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Vtctlr(pub u32);
    impl Vtctlr {
        #[doc = "VTADDR."]
        #[inline(always)]
        pub const fn vtaddr(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "VTADDR."]
        #[inline(always)]
        pub fn set_vtaddr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Vtctlr {
        #[inline(always)]
        fn default() -> Vtctlr {
            Vtctlr(0)
        }
    }
}
