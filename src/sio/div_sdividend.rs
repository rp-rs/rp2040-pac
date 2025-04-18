#[doc = "Register `DIV_SDIVIDEND` reader"]
pub type R = crate::R<DIV_SDIVIDEND_SPEC>;
#[doc = "Register `DIV_SDIVIDEND` writer"]
pub type W = crate::W<DIV_SDIVIDEND_SPEC>;
#[doc = "Field `DIV_SDIVIDEND` reader - "]
pub type DIV_SDIVIDEND_R = crate::FieldReader<u32>;
#[doc = "Field `DIV_SDIVIDEND` writer - "]
pub type DIV_SDIVIDEND_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn div_sdividend(&self) -> DIV_SDIVIDEND_R {
        DIV_SDIVIDEND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn div_sdividend(&mut self) -> DIV_SDIVIDEND_W<DIV_SDIVIDEND_SPEC> {
        DIV_SDIVIDEND_W::new(self, 0)
    }
}
#[doc = "Divider signed dividend The same as UDIVIDEND, but starts a signed calculation, rather than unsigned.  

You can [`read`](crate::generic::Reg::read) this register and get [`div_sdividend::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_sdividend::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIV_SDIVIDEND_SPEC;
impl crate::RegisterSpec for DIV_SDIVIDEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div_sdividend::R`](R) reader structure"]
impl crate::Readable for DIV_SDIVIDEND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`div_sdividend::W`](W) writer structure"]
impl crate::Writable for DIV_SDIVIDEND_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIV_SDIVIDEND to value 0"]
impl crate::Resettable for DIV_SDIVIDEND_SPEC {
    const RESET_VALUE: u32 = 0;
}
