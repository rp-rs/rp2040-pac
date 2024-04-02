#[doc = "Register `SSIENR` reader"]
pub type R = crate::R<SSIENR_SPEC>;
#[doc = "Register `SSIENR` writer"]
pub type W = crate::W<SSIENR_SPEC>;
#[doc = "Field `SSI_EN` reader - SSI enable"]
pub type SSI_EN_R = crate::BitReader;
#[doc = "Field `SSI_EN` writer - SSI enable"]
pub type SSI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SSI enable"]
    #[inline(always)]
    pub fn ssi_en(&self) -> SSI_EN_R {
        SSI_EN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssi_en(&mut self) -> SSI_EN_W<SSIENR_SPEC> {
        SSI_EN_W::new(self, 0)
    }
}
#[doc = "SSI Enable  

You can [`read`](crate::Reg::read) this register and get [`ssienr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssienr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSIENR_SPEC;
impl crate::RegisterSpec for SSIENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssienr::R`](R) reader structure"]
impl crate::Readable for SSIENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ssienr::W`](W) writer structure"]
impl crate::Writable for SSIENR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSIENR to value 0"]
impl crate::Resettable for SSIENR_SPEC {
    const RESET_VALUE: u32 = 0;
}
