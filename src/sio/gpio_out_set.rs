#[doc = "Register `GPIO_OUT_SET` writer"]
pub type W = crate::W<GPIO_OUT_SET_SPEC>;
#[doc = "Field `GPIO_OUT_SET` writer - Perform an atomic bit-set on GPIO_OUT, i.e. `GPIO_OUT |= wdata`"]
pub type GPIO_OUT_SET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
impl W {
    #[doc = "Bits 0:29 - Perform an atomic bit-set on GPIO_OUT, i.e. `GPIO_OUT |= wdata`"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_out_set(&mut self) -> GPIO_OUT_SET_W<GPIO_OUT_SET_SPEC, 0> {
        GPIO_OUT_SET_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPIO output value set  

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_out_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_OUT_SET_SPEC;
impl crate::RegisterSpec for GPIO_OUT_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpio_out_set::W`](W) writer structure"]
impl crate::Writable for GPIO_OUT_SET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_OUT_SET to value 0"]
impl crate::Resettable for GPIO_OUT_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
