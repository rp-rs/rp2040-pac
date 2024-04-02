#[doc = "Register `DMACR` reader"]
pub type R = crate::R<DMACR_SPEC>;
#[doc = "Register `DMACR` writer"]
pub type W = crate::W<DMACR_SPEC>;
#[doc = "Field `RDMAE` reader - Receive DMA enable"]
pub type RDMAE_R = crate::BitReader;
#[doc = "Field `RDMAE` writer - Receive DMA enable"]
pub type RDMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDMAE` reader - Transmit DMA enable"]
pub type TDMAE_R = crate::BitReader;
#[doc = "Field `TDMAE` writer - Transmit DMA enable"]
pub type TDMAE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive DMA enable"]
    #[inline(always)]
    pub fn rdmae(&self) -> RDMAE_R {
        RDMAE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA enable"]
    #[inline(always)]
    pub fn tdmae(&self) -> TDMAE_R {
        TDMAE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdmae(&mut self) -> RDMAE_W<DMACR_SPEC> {
        RDMAE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdmae(&mut self) -> TDMAE_W<DMACR_SPEC> {
        TDMAE_W::new(self, 1)
    }
}
#[doc = "DMA control  

You can [`read`](crate::Reg::read) this register and get [`dmacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACR_SPEC;
impl crate::RegisterSpec for DMACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacr::R`](R) reader structure"]
impl crate::Readable for DMACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmacr::W`](W) writer structure"]
impl crate::Writable for DMACR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACR to value 0"]
impl crate::Resettable for DMACR_SPEC {
    const RESET_VALUE: u32 = 0;
}
