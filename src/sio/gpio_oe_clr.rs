#[doc = "Register `GPIO_OE_CLR` writer"]
pub type W = crate::W<GPIO_OE_CLR_SPEC>;
#[doc = "Field `GPIO_OE_CLR` writer - Perform an atomic bit-clear on GPIO_OE, i.e. `GPIO_OE &amp;= ~wdata`"]
pub type GPIO_OE_CLR_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl W {
    #[doc = "Bits 0:29 - Perform an atomic bit-clear on GPIO_OE, i.e. `GPIO_OE &amp;= ~wdata`"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_oe_clr(&mut self) -> GPIO_OE_CLR_W<GPIO_OE_CLR_SPEC> {
        GPIO_OE_CLR_W::new(self, 0)
    }
}
#[doc = "GPIO output enable clear  

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_oe_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_OE_CLR_SPEC;
impl crate::RegisterSpec for GPIO_OE_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpio_oe_clr::W`](W) writer structure"]
impl crate::Writable for GPIO_OE_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_OE_CLR to value 0"]
impl crate::Resettable for GPIO_OE_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
