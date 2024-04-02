#[doc = "Register `PERFSEL2` reader"]
pub type R = crate::R<PERFSEL2_SPEC>;
#[doc = "Register `PERFSEL2` writer"]
pub type W = crate::W<PERFSEL2_SPEC>;
#[doc = "Select an event for PERFCTR2. Count either contested accesses, or all accesses, on a downstream port of the main crossbar.  

Value on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PERFSEL2_A {
    #[doc = "0: `0`"]
    APB_CONTESTED = 0,
    #[doc = "1: `1`"]
    APB = 1,
    #[doc = "2: `10`"]
    FASTPERI_CONTESTED = 2,
    #[doc = "3: `11`"]
    FASTPERI = 3,
    #[doc = "4: `100`"]
    SRAM5_CONTESTED = 4,
    #[doc = "5: `101`"]
    SRAM5 = 5,
    #[doc = "6: `110`"]
    SRAM4_CONTESTED = 6,
    #[doc = "7: `111`"]
    SRAM4 = 7,
    #[doc = "8: `1000`"]
    SRAM3_CONTESTED = 8,
    #[doc = "9: `1001`"]
    SRAM3 = 9,
    #[doc = "10: `1010`"]
    SRAM2_CONTESTED = 10,
    #[doc = "11: `1011`"]
    SRAM2 = 11,
    #[doc = "12: `1100`"]
    SRAM1_CONTESTED = 12,
    #[doc = "13: `1101`"]
    SRAM1 = 13,
    #[doc = "14: `1110`"]
    SRAM0_CONTESTED = 14,
    #[doc = "15: `1111`"]
    SRAM0 = 15,
    #[doc = "16: `10000`"]
    XIP_MAIN_CONTESTED = 16,
    #[doc = "17: `10001`"]
    XIP_MAIN = 17,
    #[doc = "18: `10010`"]
    ROM_CONTESTED = 18,
    #[doc = "19: `10011`"]
    ROM = 19,
}
impl From<PERFSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: PERFSEL2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PERFSEL2_A {
    type Ux = u8;
}
impl crate::IsEnum for PERFSEL2_A {}
#[doc = "Field `PERFSEL2` reader - Select an event for PERFCTR2. Count either contested accesses, or all accesses, on a downstream port of the main crossbar."]
pub type PERFSEL2_R = crate::FieldReader<PERFSEL2_A>;
impl PERFSEL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PERFSEL2_A> {
        match self.bits {
            0 => Some(PERFSEL2_A::APB_CONTESTED),
            1 => Some(PERFSEL2_A::APB),
            2 => Some(PERFSEL2_A::FASTPERI_CONTESTED),
            3 => Some(PERFSEL2_A::FASTPERI),
            4 => Some(PERFSEL2_A::SRAM5_CONTESTED),
            5 => Some(PERFSEL2_A::SRAM5),
            6 => Some(PERFSEL2_A::SRAM4_CONTESTED),
            7 => Some(PERFSEL2_A::SRAM4),
            8 => Some(PERFSEL2_A::SRAM3_CONTESTED),
            9 => Some(PERFSEL2_A::SRAM3),
            10 => Some(PERFSEL2_A::SRAM2_CONTESTED),
            11 => Some(PERFSEL2_A::SRAM2),
            12 => Some(PERFSEL2_A::SRAM1_CONTESTED),
            13 => Some(PERFSEL2_A::SRAM1),
            14 => Some(PERFSEL2_A::SRAM0_CONTESTED),
            15 => Some(PERFSEL2_A::SRAM0),
            16 => Some(PERFSEL2_A::XIP_MAIN_CONTESTED),
            17 => Some(PERFSEL2_A::XIP_MAIN),
            18 => Some(PERFSEL2_A::ROM_CONTESTED),
            19 => Some(PERFSEL2_A::ROM),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_apb_contested(&self) -> bool {
        *self == PERFSEL2_A::APB_CONTESTED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == PERFSEL2_A::APB
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_fastperi_contested(&self) -> bool {
        *self == PERFSEL2_A::FASTPERI_CONTESTED
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_fastperi(&self) -> bool {
        *self == PERFSEL2_A::FASTPERI
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_sram5_contested(&self) -> bool {
        *self == PERFSEL2_A::SRAM5_CONTESTED
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_sram5(&self) -> bool {
        *self == PERFSEL2_A::SRAM5
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_sram4_contested(&self) -> bool {
        *self == PERFSEL2_A::SRAM4_CONTESTED
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn is_sram4(&self) -> bool {
        *self == PERFSEL2_A::SRAM4
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn is_sram3_contested(&self) -> bool {
        *self == PERFSEL2_A::SRAM3_CONTESTED
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn is_sram3(&self) -> bool {
        *self == PERFSEL2_A::SRAM3
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn is_sram2_contested(&self) -> bool {
        *self == PERFSEL2_A::SRAM2_CONTESTED
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn is_sram2(&self) -> bool {
        *self == PERFSEL2_A::SRAM2
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn is_sram1_contested(&self) -> bool {
        *self == PERFSEL2_A::SRAM1_CONTESTED
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn is_sram1(&self) -> bool {
        *self == PERFSEL2_A::SRAM1
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_sram0_contested(&self) -> bool {
        *self == PERFSEL2_A::SRAM0_CONTESTED
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_sram0(&self) -> bool {
        *self == PERFSEL2_A::SRAM0
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn is_xip_main_contested(&self) -> bool {
        *self == PERFSEL2_A::XIP_MAIN_CONTESTED
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn is_xip_main(&self) -> bool {
        *self == PERFSEL2_A::XIP_MAIN
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn is_rom_contested(&self) -> bool {
        *self == PERFSEL2_A::ROM_CONTESTED
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn is_rom(&self) -> bool {
        *self == PERFSEL2_A::ROM
    }
}
#[doc = "Field `PERFSEL2` writer - Select an event for PERFCTR2. Count either contested accesses, or all accesses, on a downstream port of the main crossbar."]
pub type PERFSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PERFSEL2_A>;
impl<'a, REG> PERFSEL2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn apb_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::APB_CONTESTED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::APB)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn fastperi_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::FASTPERI_CONTESTED)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn fastperi(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::FASTPERI)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn sram5_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::SRAM5_CONTESTED)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn sram5(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::SRAM5)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn sram4_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::SRAM4_CONTESTED)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn sram4(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::SRAM4)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn sram3_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::SRAM3_CONTESTED)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn sram3(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::SRAM3)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn sram2_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::SRAM2_CONTESTED)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn sram2(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::SRAM2)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn sram1_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::SRAM1_CONTESTED)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn sram1(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::SRAM1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn sram0_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::SRAM0_CONTESTED)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn sram0(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::SRAM0)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn xip_main_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::XIP_MAIN_CONTESTED)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn xip_main(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::XIP_MAIN)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn rom_contested(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::ROM_CONTESTED)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn rom(self) -> &'a mut crate::W<REG> {
        self.variant(PERFSEL2_A::ROM)
    }
}
impl R {
    #[doc = "Bits 0:4 - Select an event for PERFCTR2. Count either contested accesses, or all accesses, on a downstream port of the main crossbar."]
    #[inline(always)]
    pub fn perfsel2(&self) -> PERFSEL2_R {
        PERFSEL2_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select an event for PERFCTR2. Count either contested accesses, or all accesses, on a downstream port of the main crossbar."]
    #[inline(always)]
    #[must_use]
    pub fn perfsel2(&mut self) -> PERFSEL2_W<PERFSEL2_SPEC> {
        PERFSEL2_W::new(self, 0)
    }
}
#[doc = "Bus fabric performance event select for PERFCTR2  

You can [`read`](crate::Reg::read) this register and get [`perfsel2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perfsel2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERFSEL2_SPEC;
impl crate::RegisterSpec for PERFSEL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perfsel2::R`](R) reader structure"]
impl crate::Readable for PERFSEL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perfsel2::W`](W) writer structure"]
impl crate::Writable for PERFSEL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERFSEL2 to value 0x1f"]
impl crate::Resettable for PERFSEL2_SPEC {
    const RESET_VALUE: u32 = 0x1f;
}
