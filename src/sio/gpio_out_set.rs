#[doc = "Register `GPIO_OUT_SET` writer"]
pub type W = crate::W<GPIO_OUT_SET_SPEC>;
#[doc = "Field `GPIO_OUT_SET` writer - Perform an atomic bit-set on GPIO_OUT, i.e. `GPIO_OUT |= wdata`"]
pub type GPIO_OUT_SET_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl W {
    #[doc = "Bits 0:29 - Perform an atomic bit-set on GPIO_OUT, i.e. `GPIO_OUT |= wdata`"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_out_set(&mut self) -> GPIO_OUT_SET_W<GPIO_OUT_SET_SPEC> {
        GPIO_OUT_SET_W::new(self, 0)
    }
}
#[doc = "GPIO output value set  

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_out_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_OUT_SET_SPEC;
impl crate::RegisterSpec for GPIO_OUT_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpio_out_set::W`](W) writer structure"]
impl crate::Writable for GPIO_OUT_SET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_OUT_SET to value 0"]
impl crate::Resettable for GPIO_OUT_SET_SPEC {
    const RESET_VALUE: u32 = 0;
}
