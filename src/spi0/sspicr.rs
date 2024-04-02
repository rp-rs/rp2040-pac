#[doc = "Register `SSPICR` reader"]
pub type R = crate::R<SSPICR_SPEC>;
#[doc = "Register `SSPICR` writer"]
pub type W = crate::W<SSPICR_SPEC>;
#[doc = "Field `RORIC` reader - Clears the SSPRORINTR interrupt"]
pub type RORIC_R = crate::BitReader;
#[doc = "Field `RORIC` writer - Clears the SSPRORINTR interrupt"]
pub type RORIC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RTIC` reader - Clears the SSPRTINTR interrupt"]
pub type RTIC_R = crate::BitReader;
#[doc = "Field `RTIC` writer - Clears the SSPRTINTR interrupt"]
pub type RTIC_W<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clears the SSPRORINTR interrupt"]
    #[inline(always)]
    pub fn roric(&self) -> RORIC_R {
        RORIC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clears the SSPRTINTR interrupt"]
    #[inline(always)]
    pub fn rtic(&self) -> RTIC_R {
        RTIC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clears the SSPRORINTR interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn roric(&mut self) -> RORIC_W<SSPICR_SPEC> {
        RORIC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clears the SSPRTINTR interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rtic(&mut self) -> RTIC_W<SSPICR_SPEC> {
        RTIC_W::new(self, 1)
    }
}
#[doc = "Interrupt clear register, SSPICR on page 3-11  

You can [`read`](crate::Reg::read) this register and get [`sspicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sspicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPICR_SPEC;
impl crate::RegisterSpec for SSPICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspicr::R`](R) reader structure"]
impl crate::Readable for SSPICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sspicr::W`](W) writer structure"]
impl crate::Writable for SSPICR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
#[doc = "`reset()` method sets SSPICR to value 0"]
impl crate::Resettable for SSPICR_SPEC {
    const RESET_VALUE: u32 = 0;
}
