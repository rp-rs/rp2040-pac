#[doc = "Register `WDSEL` reader"]
pub type R = crate::R<WDSEL_SPEC>;
#[doc = "Register `WDSEL` writer"]
pub type W = crate::W<WDSEL_SPEC>;
#[doc = "Field `ROSC` reader - "]
pub type ROSC_R = crate::BitReader;
#[doc = "Field `ROSC` writer - "]
pub type ROSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSC` reader - "]
pub type XOSC_R = crate::BitReader;
#[doc = "Field `XOSC` writer - "]
pub type XOSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLOCKS` reader - "]
pub type CLOCKS_R = crate::BitReader;
#[doc = "Field `CLOCKS` writer - "]
pub type CLOCKS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETS` reader - "]
pub type RESETS_R = crate::BitReader;
#[doc = "Field `RESETS` writer - "]
pub type RESETS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSFABRIC` reader - "]
pub type BUSFABRIC_R = crate::BitReader;
#[doc = "Field `BUSFABRIC` writer - "]
pub type BUSFABRIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROM` reader - "]
pub type ROM_R = crate::BitReader;
#[doc = "Field `ROM` writer - "]
pub type ROM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM0` reader - "]
pub type SRAM0_R = crate::BitReader;
#[doc = "Field `SRAM0` writer - "]
pub type SRAM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1` reader - "]
pub type SRAM1_R = crate::BitReader;
#[doc = "Field `SRAM1` writer - "]
pub type SRAM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2` reader - "]
pub type SRAM2_R = crate::BitReader;
#[doc = "Field `SRAM2` writer - "]
pub type SRAM2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM3` reader - "]
pub type SRAM3_R = crate::BitReader;
#[doc = "Field `SRAM3` writer - "]
pub type SRAM3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM4` reader - "]
pub type SRAM4_R = crate::BitReader;
#[doc = "Field `SRAM4` writer - "]
pub type SRAM4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM5` reader - "]
pub type SRAM5_R = crate::BitReader;
#[doc = "Field `SRAM5` writer - "]
pub type SRAM5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XIP` reader - "]
pub type XIP_R = crate::BitReader;
#[doc = "Field `XIP` writer - "]
pub type XIP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREG_AND_CHIP_RESET` reader - "]
pub type VREG_AND_CHIP_RESET_R = crate::BitReader;
#[doc = "Field `VREG_AND_CHIP_RESET` writer - "]
pub type VREG_AND_CHIP_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIO` reader - "]
pub type SIO_R = crate::BitReader;
#[doc = "Field `SIO` writer - "]
pub type SIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROC0` reader - "]
pub type PROC0_R = crate::BitReader;
#[doc = "Field `PROC0` writer - "]
pub type PROC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROC1` reader - "]
pub type PROC1_R = crate::BitReader;
#[doc = "Field `PROC1` writer - "]
pub type PROC1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rosc(&self) -> ROSC_R {
        ROSC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn xosc(&self) -> XOSC_R {
        XOSC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clocks(&self) -> CLOCKS_R {
        CLOCKS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn resets(&self) -> RESETS_R {
        RESETS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn busfabric(&self) -> BUSFABRIC_R {
        BUSFABRIC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sram0(&self) -> SRAM0_R {
        SRAM0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sram1(&self) -> SRAM1_R {
        SRAM1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sram2(&self) -> SRAM2_R {
        SRAM2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sram3(&self) -> SRAM3_R {
        SRAM3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sram4(&self) -> SRAM4_R {
        SRAM4_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sram5(&self) -> SRAM5_R {
        SRAM5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn xip(&self) -> XIP_R {
        XIP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn vreg_and_chip_reset(&self) -> VREG_AND_CHIP_RESET_R {
        VREG_AND_CHIP_RESET_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn sio(&self) -> SIO_R {
        SIO_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn proc0(&self) -> PROC0_R {
        PROC0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn proc1(&self) -> PROC1_R {
        PROC1_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rosc(&mut self) -> ROSC_W<WDSEL_SPEC> {
        ROSC_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn xosc(&mut self) -> XOSC_W<WDSEL_SPEC> {
        XOSC_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn clocks(&mut self) -> CLOCKS_W<WDSEL_SPEC> {
        CLOCKS_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn resets(&mut self) -> RESETS_W<WDSEL_SPEC> {
        RESETS_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn busfabric(&mut self) -> BUSFABRIC_W<WDSEL_SPEC> {
        BUSFABRIC_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> ROM_W<WDSEL_SPEC> {
        ROM_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn sram0(&mut self) -> SRAM0_W<WDSEL_SPEC> {
        SRAM0_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn sram1(&mut self) -> SRAM1_W<WDSEL_SPEC> {
        SRAM1_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn sram2(&mut self) -> SRAM2_W<WDSEL_SPEC> {
        SRAM2_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn sram3(&mut self) -> SRAM3_W<WDSEL_SPEC> {
        SRAM3_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn sram4(&mut self) -> SRAM4_W<WDSEL_SPEC> {
        SRAM4_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn sram5(&mut self) -> SRAM5_W<WDSEL_SPEC> {
        SRAM5_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn xip(&mut self) -> XIP_W<WDSEL_SPEC> {
        XIP_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn vreg_and_chip_reset(&mut self) -> VREG_AND_CHIP_RESET_W<WDSEL_SPEC> {
        VREG_AND_CHIP_RESET_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn sio(&mut self) -> SIO_W<WDSEL_SPEC> {
        SIO_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn proc0(&mut self) -> PROC0_W<WDSEL_SPEC> {
        PROC0_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn proc1(&mut self) -> PROC1_W<WDSEL_SPEC> {
        PROC1_W::new(self, 16)
    }
}
#[doc = "Set to 1 if this peripheral should be reset when the watchdog fires.  

You can [`read`](crate::generic::Reg::read) this register and get [`wdsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDSEL_SPEC;
impl crate::RegisterSpec for WDSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdsel::R`](R) reader structure"]
impl crate::Readable for WDSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdsel::W`](W) writer structure"]
impl crate::Writable for WDSEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDSEL to value 0"]
impl crate::Resettable for WDSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
