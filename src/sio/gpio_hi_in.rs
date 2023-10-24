#[doc = "Register `GPIO_HI_IN` reader"]
pub type R = crate::R<GPIO_HI_IN_SPEC>;
#[doc = "Field `GPIO_HI_IN` reader - Input value on QSPI IO in order 0..5: SCLK, SSn, SD0, SD1, SD2, SD3"]
pub type GPIO_HI_IN_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Input value on QSPI IO in order 0..5: SCLK, SSn, SD0, SD1, SD2, SD3"]
    #[inline(always)]
    pub fn gpio_hi_in(&self) -> GPIO_HI_IN_R {
        GPIO_HI_IN_R::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Input value for QSPI pins  

You can [`read`](crate::generic::Reg::read) this register and get [`gpio_hi_in::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_HI_IN_SPEC;
impl crate::RegisterSpec for GPIO_HI_IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_hi_in::R`](R) reader structure"]
impl crate::Readable for GPIO_HI_IN_SPEC {}
#[doc = "`reset()` method sets GPIO_HI_IN to value 0"]
impl crate::Resettable for GPIO_HI_IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
