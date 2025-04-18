#[doc = "Register `MEMPOWERDOWN` reader"]
pub type R = crate::R<MEMPOWERDOWN_SPEC>;
#[doc = "Register `MEMPOWERDOWN` writer"]
pub type W = crate::W<MEMPOWERDOWN_SPEC>;
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
#[doc = "Field `USB` reader - "]
pub type USB_R = crate::BitReader;
#[doc = "Field `USB` writer - "]
pub type USB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROM` reader - "]
pub type ROM_R = crate::BitReader;
#[doc = "Field `ROM` writer - "]
pub type ROM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sram0(&self) -> SRAM0_R {
        SRAM0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sram1(&self) -> SRAM1_R {
        SRAM1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sram2(&self) -> SRAM2_R {
        SRAM2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sram3(&self) -> SRAM3_R {
        SRAM3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sram4(&self) -> SRAM4_R {
        SRAM4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sram5(&self) -> SRAM5_R {
        SRAM5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rom(&self) -> ROM_R {
        ROM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn sram0(&mut self) -> SRAM0_W<MEMPOWERDOWN_SPEC> {
        SRAM0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn sram1(&mut self) -> SRAM1_W<MEMPOWERDOWN_SPEC> {
        SRAM1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn sram2(&mut self) -> SRAM2_W<MEMPOWERDOWN_SPEC> {
        SRAM2_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn sram3(&mut self) -> SRAM3_W<MEMPOWERDOWN_SPEC> {
        SRAM3_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn sram4(&mut self) -> SRAM4_W<MEMPOWERDOWN_SPEC> {
        SRAM4_W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn sram5(&mut self) -> SRAM5_W<MEMPOWERDOWN_SPEC> {
        SRAM5_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn usb(&mut self) -> USB_W<MEMPOWERDOWN_SPEC> {
        USB_W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn rom(&mut self) -> ROM_W<MEMPOWERDOWN_SPEC> {
        ROM_W::new(self, 7)
    }
}
#[doc = "Control power downs to memories. Set high to power down memories. Use with extreme caution  

You can [`read`](crate::generic::Reg::read) this register and get [`mempowerdown::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mempowerdown::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEMPOWERDOWN_SPEC;
impl crate::RegisterSpec for MEMPOWERDOWN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mempowerdown::R`](R) reader structure"]
impl crate::Readable for MEMPOWERDOWN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mempowerdown::W`](W) writer structure"]
impl crate::Writable for MEMPOWERDOWN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMPOWERDOWN to value 0"]
impl crate::Resettable for MEMPOWERDOWN_SPEC {
    const RESET_VALUE: u32 = 0;
}
