#[doc = "Register `GPIO_OUT_XOR` reader"]
pub type R = crate::R<GPIO_OUT_XOR_SPEC>;
#[doc = "Register `GPIO_OUT_XOR` writer"]
pub type W = crate::W<GPIO_OUT_XOR_SPEC>;
#[doc = "Field `GPIO_OUT_XOR` writer - Perform an atomic bitwise XOR on GPIO_OUT, i.e. `GPIO_OUT ^= wdata`"]
pub type GPIO_OUT_XOR_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl W {
    #[doc = "Bits 0:29 - Perform an atomic bitwise XOR on GPIO_OUT, i.e. `GPIO_OUT ^= wdata`"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_out_xor(&mut self) -> GPIO_OUT_XOR_W<GPIO_OUT_XOR_SPEC> {
        GPIO_OUT_XOR_W::new(self, 0)
    }
}
#[doc = "GPIO output value XOR  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_out_xor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_out_xor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_OUT_XOR_SPEC;
impl crate::RegisterSpec for GPIO_OUT_XOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_out_xor::R`](R) reader structure"]
impl crate::Readable for GPIO_OUT_XOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_out_xor::W`](W) writer structure"]
impl crate::Writable for GPIO_OUT_XOR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_OUT_XOR to value 0"]
impl crate::Resettable for GPIO_OUT_XOR_SPEC {
    const RESET_VALUE: u32 = 0;
}
