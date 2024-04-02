#[doc = "Register `VTOR` reader"]
pub type R = crate::R<VTOR_SPEC>;
#[doc = "Register `VTOR` writer"]
pub type W = crate::W<VTOR_SPEC>;
#[doc = "Field `TBLOFF` reader - Bits \\[31:8\\]
of the indicate the vector table offset address."]
pub type TBLOFF_R = crate::FieldReader<u32>;
#[doc = "Field `TBLOFF` writer - Bits \\[31:8\\]
of the indicate the vector table offset address."]
pub type TBLOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 8:31 - Bits \\[31:8\\]
of the indicate the vector table offset address."]
    #[inline(always)]
    pub fn tbloff(&self) -> TBLOFF_R {
        TBLOFF_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 8:31 - Bits \\[31:8\\]
of the indicate the vector table offset address."]
    #[inline(always)]
    #[must_use]
    pub fn tbloff(&mut self) -> TBLOFF_W<VTOR_SPEC> {
        TBLOFF_W::new(self, 8)
    }
}
#[doc = "The VTOR holds the vector table offset address.  

You can [`read`](crate::Reg::read) this register and get [`vtor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vtor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VTOR_SPEC;
impl crate::RegisterSpec for VTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vtor::R`](R) reader structure"]
impl crate::Readable for VTOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vtor::W`](W) writer structure"]
impl crate::Writable for VTOR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VTOR to value 0"]
impl crate::Resettable for VTOR_SPEC {
    const RESET_VALUE: u32 = 0;
}
