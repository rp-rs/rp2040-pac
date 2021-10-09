#[doc = "Register `CH_AL3_READ_ADDR_TRIG` reader"]
pub struct R(crate::R<CH_AL3_READ_ADDR_TRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_AL3_READ_ADDR_TRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_AL3_READ_ADDR_TRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_AL3_READ_ADDR_TRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH_AL3_READ_ADDR_TRIG` writer"]
pub struct W(crate::W<CH_AL3_READ_ADDR_TRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_AL3_READ_ADDR_TRIG_SPEC>;
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
impl From<crate::W<CH_AL3_READ_ADDR_TRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_AL3_READ_ADDR_TRIG_SPEC>) -> Self {
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
#[doc = "Alias for channel 0 READ_ADDR register  
 This is a trigger register (0xc). Writing a nonzero value will  
 reload the channel counter and start the channel.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ch_al3_read_addr_trig](index.html) module"]
pub struct CH_AL3_READ_ADDR_TRIG_SPEC;
impl crate::RegisterSpec for CH_AL3_READ_ADDR_TRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_al3_read_addr_trig::R](R) reader structure"]
impl crate::Readable for CH_AL3_READ_ADDR_TRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_al3_read_addr_trig::W](W) writer structure"]
impl crate::Writable for CH_AL3_READ_ADDR_TRIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH_AL3_READ_ADDR_TRIG to value 0"]
impl crate::Resettable for CH_AL3_READ_ADDR_TRIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
