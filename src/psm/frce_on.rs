#[doc = "Register `FRCE_ON` reader"]
pub type R = crate::R<FRCE_ON_SPEC>;
#[doc = "Register `FRCE_ON` writer"]
pub type W = crate::W<FRCE_ON_SPEC>;
#[doc = "Field `rosc` reader - "]
pub type ROSC_R = crate::BitReader;
#[doc = "Field `rosc` writer - "]
pub type ROSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xosc` reader - "]
pub type XOSC_R = crate::BitReader;
#[doc = "Field `xosc` writer - "]
pub type XOSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clocks` reader - "]
pub type CLOCKS_R = crate::BitReader;
#[doc = "Field `clocks` writer - "]
pub type CLOCKS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `resets` reader - "]
pub type RESETS_R = crate::BitReader;
#[doc = "Field `resets` writer - "]
pub type RESETS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `busfabric` reader - "]
pub type BUSFABRIC_R = crate::BitReader;
#[doc = "Field `busfabric` writer - "]
pub type BUSFABRIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rom` reader - "]
pub type ROM_R = crate::BitReader;
#[doc = "Field `rom` writer - "]
pub type ROM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sram0` reader - "]
pub type SRAM0_R = crate::BitReader;
#[doc = "Field `sram0` writer - "]
pub type SRAM0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sram1` reader - "]
pub type SRAM1_R = crate::BitReader;
#[doc = "Field `sram1` writer - "]
pub type SRAM1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sram2` reader - "]
pub type SRAM2_R = crate::BitReader;
#[doc = "Field `sram2` writer - "]
pub type SRAM2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sram3` reader - "]
pub type SRAM3_R = crate::BitReader;
#[doc = "Field `sram3` writer - "]
pub type SRAM3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sram4` reader - "]
pub type SRAM4_R = crate::BitReader;
#[doc = "Field `sram4` writer - "]
pub type SRAM4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sram5` reader - "]
pub type SRAM5_R = crate::BitReader;
#[doc = "Field `sram5` writer - "]
pub type SRAM5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `xip` reader - "]
pub type XIP_R = crate::BitReader;
#[doc = "Field `xip` writer - "]
pub type XIP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vreg_and_chip_reset` reader - "]
pub type VREG_AND_CHIP_RESET_R = crate::BitReader;
#[doc = "Field `vreg_and_chip_reset` writer - "]
pub type VREG_AND_CHIP_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sio` reader - "]
pub type SIO_R = crate::BitReader;
#[doc = "Field `sio` writer - "]
pub type SIO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `proc0` reader - "]
pub type PROC0_R = crate::BitReader;
#[doc = "Field `proc0` writer - "]
pub type PROC0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `proc1` reader - "]
pub type PROC1_R = crate::BitReader;
#[doc = "Field `proc1` writer - "]
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
    pub fn rosc(&mut self) -> ROSC_W<FRCE_ON_SPEC> {
        ROSC_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn xosc(&mut self) -> XOSC_W<FRCE_ON_SPEC> {
        XOSC_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn clocks(&mut self) -> CLOCKS_W<FRCE_ON_SPEC> {
        CLOCKS_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn resets(&mut self) -> RESETS_W<FRCE_ON_SPEC> {
        RESETS_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn busfabric(&mut self) -> BUSFABRIC_W<FRCE_ON_SPEC> {
        BUSFABRIC_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> ROM_W<FRCE_ON_SPEC> {
        ROM_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn sram0(&mut self) -> SRAM0_W<FRCE_ON_SPEC> {
        SRAM0_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn sram1(&mut self) -> SRAM1_W<FRCE_ON_SPEC> {
        SRAM1_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn sram2(&mut self) -> SRAM2_W<FRCE_ON_SPEC> {
        SRAM2_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn sram3(&mut self) -> SRAM3_W<FRCE_ON_SPEC> {
        SRAM3_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn sram4(&mut self) -> SRAM4_W<FRCE_ON_SPEC> {
        SRAM4_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn sram5(&mut self) -> SRAM5_W<FRCE_ON_SPEC> {
        SRAM5_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn xip(&mut self) -> XIP_W<FRCE_ON_SPEC> {
        XIP_W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn vreg_and_chip_reset(&mut self) -> VREG_AND_CHIP_RESET_W<FRCE_ON_SPEC> {
        VREG_AND_CHIP_RESET_W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn sio(&mut self) -> SIO_W<FRCE_ON_SPEC> {
        SIO_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn proc0(&mut self) -> PROC0_W<FRCE_ON_SPEC> {
        PROC0_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn proc1(&mut self) -> PROC1_W<FRCE_ON_SPEC> {
        PROC1_W::new(self, 16)
    }
}
#[doc = "Force block out of reset (i.e. power it on)  

You can [`read`](crate::Reg::read) this register and get [`frce_on::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frce_on::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRCE_ON_SPEC;
impl crate::RegisterSpec for FRCE_ON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frce_on::R`](R) reader structure"]
impl crate::Readable for FRCE_ON_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frce_on::W`](W) writer structure"]
impl crate::Writable for FRCE_ON_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRCE_ON to value 0"]
impl crate::Resettable for FRCE_ON_SPEC {
    const RESET_VALUE: u32 = 0;
}
