#[doc = "Register `GPIO_OE` reader"]
pub type R = crate::R<GPIO_OE_SPEC>;
#[doc = "Register `GPIO_OE` writer"]
pub type W = crate::W<GPIO_OE_SPEC>;
#[doc = "Field `GPIO_OE` reader - Set output enable (1/0 -> output/input) for GPIO0...29. Reading back gives the last value written. If core 0 and core 1 both write to GPIO_OE simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result."]
pub type GPIO_OE_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_OE` writer - Set output enable (1/0 -> output/input) for GPIO0...29. Reading back gives the last value written. If core 0 and core 1 both write to GPIO_OE simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result."]
pub type GPIO_OE_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Set output enable (1/0 -> output/input) for GPIO0...29. Reading back gives the last value written. If core 0 and core 1 both write to GPIO_OE simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result."]
    #[inline(always)]
    pub fn gpio_oe(&self) -> GPIO_OE_R {
        GPIO_OE_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Set output enable (1/0 -> output/input) for GPIO0...29. Reading back gives the last value written. If core 0 and core 1 both write to GPIO_OE simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_oe(&mut self) -> GPIO_OE_W<GPIO_OE_SPEC> {
        GPIO_OE_W::new(self, 0)
    }
}
#[doc = "GPIO output enable  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_oe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_oe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_OE_SPEC;
impl crate::RegisterSpec for GPIO_OE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_oe::R`](R) reader structure"]
impl crate::Readable for GPIO_OE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_oe::W`](W) writer structure"]
impl crate::Writable for GPIO_OE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_OE to value 0"]
impl crate::Resettable for GPIO_OE_SPEC {
    const RESET_VALUE: u32 = 0;
}
