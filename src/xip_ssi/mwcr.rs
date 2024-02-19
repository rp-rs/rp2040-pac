#[doc = "Register `MWCR` reader"]
pub type R = crate::R<MWCR_SPEC>;
#[doc = "Register `MWCR` writer"]
pub type W = crate::W<MWCR_SPEC>;
#[doc = "Field `MWMOD` reader - Microwire transfer mode"]
pub type MWMOD_R = crate::BitReader;
#[doc = "Field `MWMOD` writer - Microwire transfer mode"]
pub type MWMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDD` reader - Microwire control"]
pub type MDD_R = crate::BitReader;
#[doc = "Field `MDD` writer - Microwire control"]
pub type MDD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MHS` reader - Microwire handshaking"]
pub type MHS_R = crate::BitReader;
#[doc = "Field `MHS` writer - Microwire handshaking"]
pub type MHS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Microwire transfer mode"]
    #[inline(always)]
    pub fn mwmod(&self) -> MWMOD_R {
        MWMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Microwire control"]
    #[inline(always)]
    pub fn mdd(&self) -> MDD_R {
        MDD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Microwire handshaking"]
    #[inline(always)]
    pub fn mhs(&self) -> MHS_R {
        MHS_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Microwire transfer mode"]
    #[inline(always)]
    #[must_use]
    pub fn mwmod(&mut self) -> MWMOD_W<MWCR_SPEC> {
        MWMOD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Microwire control"]
    #[inline(always)]
    #[must_use]
    pub fn mdd(&mut self) -> MDD_W<MWCR_SPEC> {
        MDD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Microwire handshaking"]
    #[inline(always)]
    #[must_use]
    pub fn mhs(&mut self) -> MHS_W<MWCR_SPEC> {
        MHS_W::new(self, 2)
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
#[doc = "Microwire Control  

You can [`read`](crate::generic::Reg::read) this register and get [`mwcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mwcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MWCR_SPEC;
impl crate::RegisterSpec for MWCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mwcr::R`](R) reader structure"]
impl crate::Readable for MWCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mwcr::W`](W) writer structure"]
impl crate::Writable for MWCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MWCR to value 0"]
impl crate::Resettable for MWCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
