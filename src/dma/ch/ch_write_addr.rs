#[doc = "Register `CH_WRITE_ADDR` reader"]
pub struct R(crate::R<CH_WRITE_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_WRITE_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_WRITE_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_WRITE_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH_WRITE_ADDR` writer"]
pub struct W(crate::W<CH_WRITE_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_WRITE_ADDR_SPEC>;
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
impl From<crate::W<CH_WRITE_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_WRITE_ADDR_SPEC>) -> Self {
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
#[doc = "DMA Channel 0 Write Address pointer  
 This register updates automatically each time a write completes. The current value is the next address to be written by this channel.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ch_write_addr](index.html) module"]
pub struct CH_WRITE_ADDR_SPEC;
impl crate::RegisterSpec for CH_WRITE_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_write_addr::R](R) reader structure"]
impl crate::Readable for CH_WRITE_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_write_addr::W](W) writer structure"]
impl crate::Writable for CH_WRITE_ADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH_WRITE_ADDR to value 0"]
impl crate::Resettable for CH_WRITE_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
