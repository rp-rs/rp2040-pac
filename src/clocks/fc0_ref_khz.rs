#[doc = "Register `FC0_REF_KHZ` reader"]
pub type R = crate::R<FC0_REF_KHZ_SPEC>;
#[doc = "Register `FC0_REF_KHZ` writer"]
pub type W = crate::W<FC0_REF_KHZ_SPEC>;
#[doc = "Field `FC0_REF_KHZ` reader - "]
pub type FC0_REF_KHZ_R = crate::FieldReader<u32>;
#[doc = "Field `FC0_REF_KHZ` writer - "]
pub type FC0_REF_KHZ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn fc0_ref_khz(&self) -> FC0_REF_KHZ_R {
        FC0_REF_KHZ_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn fc0_ref_khz(&mut self) -> FC0_REF_KHZ_W<FC0_REF_KHZ_SPEC, 0> {
        FC0_REF_KHZ_W::new(self)
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
#[doc = "Reference clock frequency in kHz  

You can [`read`](crate::generic::Reg::read) this register and get [`fc0_ref_khz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc0_ref_khz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FC0_REF_KHZ_SPEC;
impl crate::RegisterSpec for FC0_REF_KHZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc0_ref_khz::R`](R) reader structure"]
impl crate::Readable for FC0_REF_KHZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fc0_ref_khz::W`](W) writer structure"]
impl crate::Writable for FC0_REF_KHZ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FC0_REF_KHZ to value 0"]
impl crate::Resettable for FC0_REF_KHZ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
