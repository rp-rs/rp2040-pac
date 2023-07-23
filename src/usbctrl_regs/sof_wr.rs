#[doc = "Register `SOF_WR` writer"]
pub struct W(crate::W<SOF_WR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOF_WR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SOF_WR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOF_WR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` writer - "]
pub type COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SOF_WR_SPEC, u16, u16, 11, O>;
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn count(&mut self) -> COUNT_W<0> {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time.  

This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sof_wr](index.html) module"]
pub struct SOF_WR_SPEC;
impl crate::RegisterSpec for SOF_WR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sof_wr::W](W) writer structure"]
impl crate::Writable for SOF_WR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOF_WR to value 0"]
impl crate::Resettable for SOF_WR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
