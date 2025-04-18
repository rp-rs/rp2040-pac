#[doc = "Register `GPIO_OUT` reader"]
pub type R = crate::R<GPIO_OUT_SPEC>;
#[doc = "Register `GPIO_OUT` writer"]
pub type W = crate::W<GPIO_OUT_SPEC>;
#[doc = "Field `GPIO_OUT` reader - Set output level (1/0 -> high/low) for GPIO0...29. Reading back gives the last value written, NOT the input value from the pins. If core 0 and core 1 both write to GPIO_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result."]
pub type GPIO_OUT_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_OUT` writer - Set output level (1/0 -> high/low) for GPIO0...29. Reading back gives the last value written, NOT the input value from the pins. If core 0 and core 1 both write to GPIO_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result."]
pub type GPIO_OUT_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Set output level (1/0 -> high/low) for GPIO0...29. Reading back gives the last value written, NOT the input value from the pins. If core 0 and core 1 both write to GPIO_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result."]
    #[inline(always)]
    pub fn gpio_out(&self) -> GPIO_OUT_R {
        GPIO_OUT_R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Set output level (1/0 -> high/low) for GPIO0...29. Reading back gives the last value written, NOT the input value from the pins. If core 0 and core 1 both write to GPIO_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_out(&mut self) -> GPIO_OUT_W<GPIO_OUT_SPEC> {
        GPIO_OUT_W::new(self, 0)
    }
}
#[doc = "GPIO output value  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_OUT_SPEC;
impl crate::RegisterSpec for GPIO_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_out::R`](R) reader structure"]
impl crate::Readable for GPIO_OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_out::W`](W) writer structure"]
impl crate::Writable for GPIO_OUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_OUT to value 0"]
impl crate::Resettable for GPIO_OUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
