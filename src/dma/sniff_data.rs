#[doc = "Register `SNIFF_DATA` reader"]
pub struct R(crate::R<SNIFF_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNIFF_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNIFF_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNIFF_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SNIFF_DATA` writer"]
pub struct W(crate::W<SNIFF_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNIFF_DATA_SPEC>;
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
impl From<crate::W<SNIFF_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNIFF_DATA_SPEC>) -> Self {
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
#[doc = "Data accumulator for sniff hardware  
 Write an initial seed value here before starting a DMA transfer on the channel indicated by SNIFF_CTRL_DMACH. The hardware will update this register each time it observes a read from the indicated channel. Once the channel completes, the final result can be read from this register.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sniff_data](index.html) module"]
pub struct SNIFF_DATA_SPEC;
impl crate::RegisterSpec for SNIFF_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sniff_data::R](R) reader structure"]
impl crate::Readable for SNIFF_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sniff_data::W](W) writer structure"]
impl crate::Writable for SNIFF_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SNIFF_DATA to value 0"]
impl crate::Resettable for SNIFF_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
