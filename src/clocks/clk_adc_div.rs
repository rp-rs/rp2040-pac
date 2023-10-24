#[doc = "Register `CLK_ADC_DIV` reader"]
pub type R = crate::R<CLK_ADC_DIV_SPEC>;
#[doc = "Register `CLK_ADC_DIV` writer"]
pub type W = crate::W<CLK_ADC_DIV_SPEC>;
#[doc = "Field `INT` reader - Integer component of the divisor, 0 -> divide by 2^16"]
pub type INT_R = crate::FieldReader;
#[doc = "Field `INT` writer - Integer component of the divisor, 0 -> divide by 2^16"]
pub type INT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 8:9 - Integer component of the divisor, 0 -> divide by 2^16"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - Integer component of the divisor, 0 -> divide by 2^16"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<CLK_ADC_DIV_SPEC, 8> {
        INT_W::new(self)
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
#[doc = "Clock divisor, can be changed on-the-fly  

You can [`read`](crate::generic::Reg::read) this register and get [`clk_adc_div::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_adc_div::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_ADC_DIV_SPEC;
impl crate::RegisterSpec for CLK_ADC_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_adc_div::R`](R) reader structure"]
impl crate::Readable for CLK_ADC_DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_adc_div::W`](W) writer structure"]
impl crate::Writable for CLK_ADC_DIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_ADC_DIV to value 0x0100"]
impl crate::Resettable for CLK_ADC_DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
