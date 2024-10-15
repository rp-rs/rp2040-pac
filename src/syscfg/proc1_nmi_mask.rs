#[doc = "Register `PROC1_NMI_MASK` reader"]
pub type R = crate::R<PROC1_NMI_MASK_SPEC>;
#[doc = "Register `PROC1_NMI_MASK` writer"]
pub type W = crate::W<PROC1_NMI_MASK_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Processor core 1 NMI source mask  
 Set a bit high to enable NMI from that IRQ  

You can [`read`](crate::Reg::read) this register and get [`proc1_nmi_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`proc1_nmi_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PROC1_NMI_MASK_SPEC;
impl crate::RegisterSpec for PROC1_NMI_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`proc1_nmi_mask::R`](R) reader structure"]
impl crate::Readable for PROC1_NMI_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`proc1_nmi_mask::W`](W) writer structure"]
impl crate::Writable for PROC1_NMI_MASK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PROC1_NMI_MASK to value 0"]
impl crate::Resettable for PROC1_NMI_MASK_SPEC {
    const RESET_VALUE: u32 = 0;
}
