#[doc = "Register `MAIN_CTRL` reader"]
pub type R = crate::R<MAIN_CTRL_SPEC>;
#[doc = "Register `MAIN_CTRL` writer"]
pub type W = crate::W<MAIN_CTRL_SPEC>;
#[doc = "Field `CONTROLLER_EN` reader - Enable controller"]
pub type CONTROLLER_EN_R = crate::BitReader;
#[doc = "Field `CONTROLLER_EN` writer - Enable controller"]
pub type CONTROLLER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST_NDEVICE` reader - Device mode = 0, Host mode = 1"]
pub type HOST_NDEVICE_R = crate::BitReader;
#[doc = "Field `HOST_NDEVICE` writer - Device mode = 0, Host mode = 1"]
pub type HOST_NDEVICE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIM_TIMING` reader - Reduced timings for simulation"]
pub type SIM_TIMING_R = crate::BitReader;
#[doc = "Field `SIM_TIMING` writer - Reduced timings for simulation"]
pub type SIM_TIMING_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable controller"]
    #[inline(always)]
    pub fn controller_en(&self) -> CONTROLLER_EN_R {
        CONTROLLER_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Device mode = 0, Host mode = 1"]
    #[inline(always)]
    pub fn host_ndevice(&self) -> HOST_NDEVICE_R {
        HOST_NDEVICE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - Reduced timings for simulation"]
    #[inline(always)]
    pub fn sim_timing(&self) -> SIM_TIMING_R {
        SIM_TIMING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable controller"]
    #[inline(always)]
    #[must_use]
    pub fn controller_en(&mut self) -> CONTROLLER_EN_W<MAIN_CTRL_SPEC> {
        CONTROLLER_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Device mode = 0, Host mode = 1"]
    #[inline(always)]
    #[must_use]
    pub fn host_ndevice(&mut self) -> HOST_NDEVICE_W<MAIN_CTRL_SPEC> {
        HOST_NDEVICE_W::new(self, 1)
    }
    #[doc = "Bit 31 - Reduced timings for simulation"]
    #[inline(always)]
    #[must_use]
    pub fn sim_timing(&mut self) -> SIM_TIMING_W<MAIN_CTRL_SPEC> {
        SIM_TIMING_W::new(self, 31)
    }
}
#[doc = "Main control register  

You can [`read`](crate::generic::Reg::read) this register and get [`main_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`main_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MAIN_CTRL_SPEC;
impl crate::RegisterSpec for MAIN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`main_ctrl::R`](R) reader structure"]
impl crate::Readable for MAIN_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`main_ctrl::W`](W) writer structure"]
impl crate::Writable for MAIN_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MAIN_CTRL to value 0"]
impl crate::Resettable for MAIN_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
