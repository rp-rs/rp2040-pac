#[doc = "Register `FIFO_WR` writer"]
pub struct W(crate::W<FIFO_WR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_WR_SPEC>;
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
impl From<crate::W<FIFO_WR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_WR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write access to this core's TX FIFO  

This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [fifo_wr](index.html) module"]
pub struct FIFO_WR_SPEC;
impl crate::RegisterSpec for FIFO_WR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fifo_wr::W](W) writer structure"]
impl crate::Writable for FIFO_WR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFO_WR to value 0"]
impl crate::Resettable for FIFO_WR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
