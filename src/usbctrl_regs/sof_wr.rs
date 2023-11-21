#[doc = "Register `SOF_WR` writer"]
pub type W = crate::W<SOF_WR_SPEC>;
#[doc = "Field `COUNT` writer - "]
pub type COUNT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<SOF_WR_SPEC, 0> {
        COUNT_W::new(self)
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
#[doc = "Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time.  

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sof_wr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOF_WR_SPEC;
impl crate::RegisterSpec for SOF_WR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sof_wr::W`](W) writer structure"]
impl crate::Writable for SOF_WR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOF_WR to value 0"]
impl crate::Resettable for SOF_WR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
