#[doc = "Register `SM_ADDR` reader"]
pub type R = crate::R<SM_ADDR_SPEC>;
#[doc = "Register `SM_ADDR` writer"]
pub type W = crate::W<SM_ADDR_SPEC>;
#[doc = "Field `SM0_ADDR` reader - "]
pub type SM0_ADDR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn sm0_addr(&self) -> SM0_ADDR_R {
        SM0_ADDR_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {}
#[doc = "Current instruction address of state machine 0  

You can [`read`](crate::generic::Reg::read) this register and get [`sm_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sm_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SM_ADDR_SPEC;
impl crate::RegisterSpec for SM_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sm_addr::R`](R) reader structure"]
impl crate::Readable for SM_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sm_addr::W`](W) writer structure"]
impl crate::Writable for SM_ADDR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SM_ADDR to value 0"]
impl crate::Resettable for SM_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
