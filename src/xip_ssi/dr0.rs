#[doc = "Register `DR0` reader"]
pub type R = crate::R<DR0_SPEC>;
#[doc = "Register `DR0` writer"]
pub type W = crate::W<DR0_SPEC>;
#[doc = "Field `DR` reader - First data register of 36"]
pub type DR_R = crate::FieldReader<u32>;
#[doc = "Field `DR` writer - First data register of 36"]
pub type DR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - First data register of 36"]
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - First data register of 36"]
    #[inline(always)]
    #[must_use]
    pub fn dr(&mut self) -> DR_W<DR0_SPEC, 0> {
        DR_W::new(self)
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
#[doc = "Data Register 0 (of 36)  

You can [`read`](crate::generic::Reg::read) this register and get [`dr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DR0_SPEC;
impl crate::RegisterSpec for DR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr0::R`](R) reader structure"]
impl crate::Readable for DR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dr0::W`](W) writer structure"]
impl crate::Writable for DR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR0 to value 0"]
impl crate::Resettable for DR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
