#[doc = "Register `GPIO_OUT_CLR` writer"]
pub struct W(crate::W<GPIO_OUT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_OUT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GPIO_OUT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_OUT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_OUT_CLR` writer - Perform an atomic bit-clear on GPIO_OUT, i.e. `GPIO_OUT &= ~wdata`"]
pub type GPIO_OUT_CLR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO_OUT_CLR_SPEC, u32, u32, 30, O>;
impl W {
    #[doc = "Bits 0:29 - Perform an atomic bit-clear on GPIO_OUT, i.e. `GPIO_OUT &= ~wdata`"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_out_clr(&mut self) -> GPIO_OUT_CLR_W<0> {
        GPIO_OUT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO output value clear  

This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [gpio_out_clr](index.html) module"]
pub struct GPIO_OUT_CLR_SPEC;
impl crate::RegisterSpec for GPIO_OUT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpio_out_clr::W](W) writer structure"]
impl crate::Writable for GPIO_OUT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_OUT_CLR to value 0"]
impl crate::Resettable for GPIO_OUT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
