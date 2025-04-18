#[doc = "Register `INTERP0_BASE_1AND0` reader"]
pub type R = crate::R<INTERP0_BASE_1AND0_SPEC>;
#[doc = "Register `INTERP0_BASE_1AND0` writer"]
pub type W = crate::W<INTERP0_BASE_1AND0_SPEC>;
#[doc = "Field `INTERP0_BASE_1AND0` writer - "]
pub type INTERP0_BASE_1AND0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn interp0_base_1and0(&mut self) -> INTERP0_BASE_1AND0_W<INTERP0_BASE_1AND0_SPEC> {
        INTERP0_BASE_1AND0_W::new(self, 0)
    }
}
#[doc = "On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously. Each half is sign-extended to 32 bits if that lane's SIGNED flag is set.  

You can [`read`](crate::generic::Reg::read) this register and get [`interp0_base_1and0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interp0_base_1and0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP0_BASE_1AND0_SPEC;
impl crate::RegisterSpec for INTERP0_BASE_1AND0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interp0_base_1and0::R`](R) reader structure"]
impl crate::Readable for INTERP0_BASE_1AND0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interp0_base_1and0::W`](W) writer structure"]
impl crate::Writable for INTERP0_BASE_1AND0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERP0_BASE_1AND0 to value 0"]
impl crate::Resettable for INTERP0_BASE_1AND0_SPEC {
    const RESET_VALUE: u32 = 0;
}
