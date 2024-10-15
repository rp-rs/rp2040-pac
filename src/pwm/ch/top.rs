#[doc = "Register `TOP` reader"]
pub type R = crate::R<TOP_SPEC>;
#[doc = "Register `TOP` writer"]
pub type W = crate::W<TOP_SPEC>;
#[doc = "Field `TOP` reader - "]
pub type TOP_R = crate::FieldReader<u16>;
#[doc = "Field `TOP` writer - "]
pub type TOP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn top(&self) -> TOP_R {
        TOP_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn top(&mut self) -> TOP_W<TOP_SPEC> {
        TOP_W::new(self, 0)
    }
}
#[doc = "Counter wrap value  

You can [`read`](crate::Reg::read) this register and get [`top::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`top::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOP_SPEC;
impl crate::RegisterSpec for TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`top::R`](R) reader structure"]
impl crate::Readable for TOP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`top::W`](W) writer structure"]
impl crate::Writable for TOP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOP to value 0xffff"]
impl crate::Resettable for TOP_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
