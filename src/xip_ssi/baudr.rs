#[doc = "Register `BAUDR` reader"]
pub type R = crate::R<BAUDR_SPEC>;
#[doc = "Register `BAUDR` writer"]
pub type W = crate::W<BAUDR_SPEC>;
#[doc = "Field `SCKDV` reader - SSI clock divider"]
pub type SCKDV_R = crate::FieldReader<u16>;
#[doc = "Field `SCKDV` writer - SSI clock divider"]
pub type SCKDV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SSI clock divider"]
    #[inline(always)]
    pub fn sckdv(&self) -> SCKDV_R {
        SCKDV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SSI clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn sckdv(&mut self) -> SCKDV_W<BAUDR_SPEC> {
        SCKDV_W::new(self, 0)
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
#[doc = "Baud rate  

You can [`read`](crate::generic::Reg::read) this register and get [`baudr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`baudr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BAUDR_SPEC;
impl crate::RegisterSpec for BAUDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baudr::R`](R) reader structure"]
impl crate::Readable for BAUDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`baudr::W`](W) writer structure"]
impl crate::Writable for BAUDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BAUDR to value 0"]
impl crate::Resettable for BAUDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
