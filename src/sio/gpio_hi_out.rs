#[doc = "Register `GPIO_HI_OUT` reader"]
pub type R = crate::R<GPIO_HI_OUT_SPEC>;
#[doc = "Register `GPIO_HI_OUT` writer"]
pub type W = crate::W<GPIO_HI_OUT_SPEC>;
#[doc = "Field `GPIO_HI_OUT` reader - Set output level (1/0 -> high/low) for QSPI IO0...5. Reading back gives the last value written, NOT the input value from the pins. If core 0 and core 1 both write to GPIO_HI_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result."]
pub type GPIO_HI_OUT_R = crate::FieldReader;
#[doc = "Field `GPIO_HI_OUT` writer - Set output level (1/0 -> high/low) for QSPI IO0...5. Reading back gives the last value written, NOT the input value from the pins. If core 0 and core 1 both write to GPIO_HI_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result."]
pub type GPIO_HI_OUT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Set output level (1/0 -> high/low) for QSPI IO0...5. Reading back gives the last value written, NOT the input value from the pins. If core 0 and core 1 both write to GPIO_HI_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result."]
    #[inline(always)]
    pub fn gpio_hi_out(&self) -> GPIO_HI_OUT_R {
        GPIO_HI_OUT_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Set output level (1/0 -> high/low) for QSPI IO0...5. Reading back gives the last value written, NOT the input value from the pins. If core 0 and core 1 both write to GPIO_HI_OUT simultaneously (or to a SET/CLR/XOR alias), the result is as though the write from core 0 took place first, and the write from core 1 was then applied to that intermediate result."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_hi_out(&mut self) -> GPIO_HI_OUT_W<GPIO_HI_OUT_SPEC> {
        GPIO_HI_OUT_W::new(self, 0)
    }
}
#[doc = "QSPI output value  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_hi_out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_hi_out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_HI_OUT_SPEC;
impl crate::RegisterSpec for GPIO_HI_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_hi_out::R`](R) reader structure"]
impl crate::Readable for GPIO_HI_OUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_hi_out::W`](W) writer structure"]
impl crate::Writable for GPIO_HI_OUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_HI_OUT to value 0"]
impl crate::Resettable for GPIO_HI_OUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
