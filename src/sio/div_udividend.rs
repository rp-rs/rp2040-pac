#[doc = "Register `DIV_UDIVIDEND` reader"]
pub type R = crate::R<DIV_UDIVIDEND_SPEC>;
#[doc = "Register `DIV_UDIVIDEND` writer"]
pub type W = crate::W<DIV_UDIVIDEND_SPEC>;
#[doc = "Field `DIV_UDIVIDEND` reader - "]
pub type DIV_UDIVIDEND_R = crate::FieldReader<u32>;
#[doc = "Field `DIV_UDIVIDEND` writer - "]
pub type DIV_UDIVIDEND_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn div_udividend(&self) -> DIV_UDIVIDEND_R {
        DIV_UDIVIDEND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn div_udividend(&mut self) -> DIV_UDIVIDEND_W<DIV_UDIVIDEND_SPEC> {
        DIV_UDIVIDEND_W::new(self, 0)
    }
}
#[doc = "Divider unsigned dividend Write to the DIVIDEND operand of the divider, i.e. the p in `p / q`. Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER. UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an unsigned calculation, and the S alias starts a signed calculation.  

You can [`read`](crate::generic::Reg::read) this register and get [`div_udividend::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div_udividend::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIV_UDIVIDEND_SPEC;
impl crate::RegisterSpec for DIV_UDIVIDEND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div_udividend::R`](R) reader structure"]
impl crate::Readable for DIV_UDIVIDEND_SPEC {}
#[doc = "`write(|w| ..)` method takes [`div_udividend::W`](W) writer structure"]
impl crate::Writable for DIV_UDIVIDEND_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIV_UDIVIDEND to value 0"]
impl crate::Resettable for DIV_UDIVIDEND_SPEC {
    const RESET_VALUE: u32 = 0;
}
