#[doc = "Register `DMATDLR` reader"]
pub type R = crate::R<DMATDLR_SPEC>;
#[doc = "Register `DMATDLR` writer"]
pub type W = crate::W<DMATDLR_SPEC>;
#[doc = "Field `DMATDL` reader - Transmit data watermark level"]
pub type DMATDL_R = crate::FieldReader;
#[doc = "Field `DMATDL` writer - Transmit data watermark level"]
pub type DMATDL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
    pub fn dmatdl(&mut self) -> DMATDL_W<DMATDLR_SPEC> {
        DMATDL_W::new(self, 0)
    }
}
#[doc = "DMA TX data level  

You can [`read`](crate::Reg::read) this register and get [`dmatdlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmatdlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATDLR_SPEC;
impl crate::RegisterSpec for DMATDLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatdlr::R`](R) reader structure"]
impl crate::Readable for DMATDLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmatdlr::W`](W) writer structure"]
impl crate::Writable for DMATDLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMATDLR to value 0"]
impl crate::Resettable for DMATDLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
