#[doc = "Register `FBDIV_INT` reader"]
pub type R = crate::R<FBDIV_INT_SPEC>;
#[doc = "Register `FBDIV_INT` writer"]
pub type W = crate::W<FBDIV_INT_SPEC>;
#[doc = "Field `FBDIV_INT` reader - see ctrl reg description for constraints"]
pub type FBDIV_INT_R = crate::FieldReader<u16>;
#[doc = "Field `FBDIV_INT` writer - see ctrl reg description for constraints"]
pub type FBDIV_INT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - see ctrl reg description for constraints"]
    #[inline(always)]
    pub fn fbdiv_int(&self) -> FBDIV_INT_R {
        FBDIV_INT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - see ctrl reg description for constraints"]
    #[inline(always)]
    #[must_use]
    pub fn fbdiv_int(&mut self) -> FBDIV_INT_W<FBDIV_INT_SPEC, 0> {
        FBDIV_INT_W::new(self)
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
#[doc = "Feedback divisor  
 (note: this PLL does not support fractional division)  

You can [`read`](crate::generic::Reg::read) this register and get [`fbdiv_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbdiv_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FBDIV_INT_SPEC;
impl crate::RegisterSpec for FBDIV_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fbdiv_int::R`](R) reader structure"]
impl crate::Readable for FBDIV_INT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fbdiv_int::W`](W) writer structure"]
impl crate::Writable for FBDIV_INT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FBDIV_INT to value 0"]
impl crate::Resettable for FBDIV_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
