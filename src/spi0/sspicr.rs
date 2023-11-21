#[doc = "Register `SSPICR` reader"]
pub type R = crate::R<SSPICR_SPEC>;
#[doc = "Register `SSPICR` writer"]
pub type W = crate::W<SSPICR_SPEC>;
#[doc = "Field `RORIC` reader - Clears the SSPRORINTR interrupt"]
pub type RORIC_R = crate::BitReader;
#[doc = "Field `RORIC` writer - Clears the SSPRORINTR interrupt"]
pub type RORIC_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
#[doc = "Field `RTIC` reader - Clears the SSPRTINTR interrupt"]
pub type RTIC_R = crate::BitReader;
#[doc = "Field `RTIC` writer - Clears the SSPRTINTR interrupt"]
pub type RTIC_W<'a, REG, const O: u8> = crate::BitWriter1C<'a, REG, O>;
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
    pub fn roric(&mut self) -> RORIC_W<SSPICR_SPEC, 0> {
        RORIC_W::new(self)
    }
    #[doc = "Bit 1 - Clears the SSPRTINTR interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rtic(&mut self) -> RTIC_W<SSPICR_SPEC, 1> {
        RTIC_W::new(self)
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
#[doc = "Interrupt clear register, SSPICR on page 3-11  

You can [`read`](crate::generic::Reg::read) this register and get [`sspicr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspicr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPICR_SPEC;
impl crate::RegisterSpec for SSPICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspicr::R`](R) reader structure"]
impl crate::Readable for SSPICR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sspicr::W`](W) writer structure"]
impl crate::Writable for SSPICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x03;
}
#[doc = "`reset()` method sets SSPICR to value 0"]
impl crate::Resettable for SSPICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
