#[doc = "Register `DMATDLR` reader"]
pub type R = crate::R<DMATDLR_SPEC>;
#[doc = "Register `DMATDLR` writer"]
pub type W = crate::W<DMATDLR_SPEC>;
#[doc = "Field `DMATDL` reader - Transmit data watermark level"]
pub type DMATDL_R = crate::FieldReader;
#[doc = "Field `DMATDL` writer - Transmit data watermark level"]
pub type DMATDL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Transmit data watermark level"]
    #[inline(always)]
    pub fn dmatdl(&self) -> DMATDL_R {
        DMATDL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit data watermark level"]
    #[inline(always)]
    #[must_use]
    pub fn dmatdl(&mut self) -> DMATDL_W<DMATDLR_SPEC, 0> {
        DMATDL_W::new(self)
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
#[doc = "DMA TX data level  

You can [`read`](crate::generic::Reg::read) this register and get [`dmatdlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatdlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATDLR_SPEC;
impl crate::RegisterSpec for DMATDLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatdlr::R`](R) reader structure"]
impl crate::Readable for DMATDLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmatdlr::W`](W) writer structure"]
impl crate::Writable for DMATDLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMATDLR to value 0"]
impl crate::Resettable for DMATDLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
