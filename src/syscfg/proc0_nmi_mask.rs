#[doc = "Register `PROC0_NMI_MASK` reader"]
pub type R = crate::R<PROC0_NMI_MASK_SPEC>;
#[doc = "Register `PROC0_NMI_MASK` writer"]
pub type W = crate::W<PROC0_NMI_MASK_SPEC>;
#[doc = "Field `PROC0_NMI_MASK` reader - Set a bit high to enable NMI from that IRQ"]
pub type PROC0_NMI_MASK_R = crate::FieldReader<u32>;
#[doc = "Field `PROC0_NMI_MASK` writer - Set a bit high to enable NMI from that IRQ"]
pub type PROC0_NMI_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Set a bit high to enable NMI from that IRQ"]
    #[inline(always)]
    pub fn proc0_nmi_mask(&self) -> PROC0_NMI_MASK_R {
        PROC0_NMI_MASK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Set a bit high to enable NMI from that IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn proc0_nmi_mask(&mut self) -> PROC0_NMI_MASK_W<PROC0_NMI_MASK_SPEC> {
        PROC0_NMI_MASK_W::new(self, 0)
    }
}
#[doc = "Processor core 0 NMI source mask  

You can [`read`](crate::generic::Reg::read) this register and get [`proc0_nmi_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`proc0_nmi_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROC0_NMI_MASK_SPEC;
impl crate::RegisterSpec for PROC0_NMI_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`proc0_nmi_mask::R`](R) reader structure"]
impl crate::Readable for PROC0_NMI_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`proc0_nmi_mask::W`](W) writer structure"]
impl crate::Writable for PROC0_NMI_MASK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PROC0_NMI_MASK to value 0"]
impl crate::Resettable for PROC0_NMI_MASK_SPEC {
    const RESET_VALUE: u32 = 0;
}
