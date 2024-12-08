#[doc = "Register `SOF_RD` reader"]
pub type R = crate::R<SOF_RD_SPEC>;
#[doc = "Register `SOF_RD` writer"]
pub type W = crate::W<SOF_RD_SPEC>;
#[doc = "Field `COUNT` reader - "]
pub type COUNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {}
#[doc = "Read the last SOF (Start of Frame) frame number seen. In device mode the last SOF received from the host. In host mode the last SOF sent by the host.  

You can [`read`](crate::generic::Reg::read) this register and get [`sof_rd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sof_rd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOF_RD_SPEC;
impl crate::RegisterSpec for SOF_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sof_rd::R`](R) reader structure"]
impl crate::Readable for SOF_RD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sof_rd::W`](W) writer structure"]
impl crate::Writable for SOF_RD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOF_RD to value 0"]
impl crate::Resettable for SOF_RD_SPEC {
    const RESET_VALUE: u32 = 0;
}
