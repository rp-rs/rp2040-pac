#[doc = "Register `SSPRIS` reader"]
pub type R = crate::R<SSPRIS_SPEC>;
#[doc = "Register `SSPRIS` writer"]
pub type W = crate::W<SSPRIS_SPEC>;
#[doc = "Field `RORRIS` reader - Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt"]
pub type RORRIS_R = crate::BitReader;
#[doc = "Field `RTRIS` reader - Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt"]
pub type RTRIS_R = crate::BitReader;
#[doc = "Field `RXRIS` reader - Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt"]
pub type RXRIS_R = crate::BitReader;
#[doc = "Field `TXRIS` reader - Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt"]
pub type TXRIS_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt"]
    #[inline(always)]
    pub fn rorris(&self) -> RORRIS_R {
        RORRIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt"]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt"]
    #[inline(always)]
    pub fn rxris(&self) -> RXRIS_R {
        RXRIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt"]
    #[inline(always)]
    pub fn txris(&self) -> TXRIS_R {
        TXRIS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {}
#[doc = "Raw interrupt status register, SSPRIS on page 3-10  

You can [`read`](crate::generic::Reg::read) this register and get [`sspris::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspris::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSPRIS_SPEC;
impl crate::RegisterSpec for SSPRIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sspris::R`](R) reader structure"]
impl crate::Readable for SSPRIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sspris::W`](W) writer structure"]
impl crate::Writable for SSPRIS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSPRIS to value 0x08"]
impl crate::Resettable for SSPRIS_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
