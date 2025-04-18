#[doc = "Register `GPIO_OUT_CLR` reader"]
pub type R = crate::R<GPIO_OUT_CLR_SPEC>;
#[doc = "Register `GPIO_OUT_CLR` writer"]
pub type W = crate::W<GPIO_OUT_CLR_SPEC>;
#[doc = "Field `GPIO_OUT_CLR` writer - Perform an atomic bit-clear on GPIO_OUT, i.e. `GPIO_OUT &amp;= ~wdata`"]
pub type GPIO_OUT_CLR_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl W {
    #[doc = "Bits 0:29 - Perform an atomic bit-clear on GPIO_OUT, i.e. `GPIO_OUT &amp;= ~wdata`"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_out_clr(&mut self) -> GPIO_OUT_CLR_W<GPIO_OUT_CLR_SPEC> {
        GPIO_OUT_CLR_W::new(self, 0)
    }
}
#[doc = "GPIO output value clear  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_out_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_out_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_OUT_CLR_SPEC;
impl crate::RegisterSpec for GPIO_OUT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_out_clr::R`](R) reader structure"]
impl crate::Readable for GPIO_OUT_CLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_out_clr::W`](W) writer structure"]
impl crate::Writable for GPIO_OUT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_OUT_CLR to value 0"]
impl crate::Resettable for GPIO_OUT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
