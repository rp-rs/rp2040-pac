#[doc = "Register `SOF_WR` reader"]
pub type R = crate::R<SOF_WR_SPEC>;
#[doc = "Register `SOF_WR` writer"]
pub type W = crate::W<SOF_WR_SPEC>;
#[doc = "Field `COUNT` writer - "]
pub type COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<SOF_WR_SPEC> {
        COUNT_W::new(self, 0)
    }
}
#[doc = "Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time.  

You can [`read`](crate::generic::Reg::read) this register and get [`sof_wr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sof_wr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOF_WR_SPEC;
impl crate::RegisterSpec for SOF_WR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sof_wr::R`](R) reader structure"]
impl crate::Readable for SOF_WR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sof_wr::W`](W) writer structure"]
impl crate::Writable for SOF_WR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOF_WR to value 0"]
impl crate::Resettable for SOF_WR_SPEC {
    const RESET_VALUE: u32 = 0;
}
