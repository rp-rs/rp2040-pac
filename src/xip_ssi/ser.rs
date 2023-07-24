#[doc = "Register `SER` reader"]
pub struct R(crate::R<SER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SER` writer"]
pub struct W(crate::W<SER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SER_SPEC>;
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
impl From<crate::W<SER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SER` reader - For each bit:  
 0 -> slave not selected  
 1 -> slave selected"]
pub type SER_R = crate::BitReader<bool>;
#[doc = "Field `SER` writer - For each bit:  
 0 -> slave not selected  
 1 -> slave selected"]
pub type SER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - For each bit:  
 0 -> slave not selected  
 1 -> slave selected"]
    #[inline(always)]
    pub fn ser(&self) -> SER_R {
        SER_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - For each bit:  
 0 -> slave not selected  
 1 -> slave selected"]
    #[inline(always)]
    #[must_use]
    pub fn ser(&mut self) -> SER_W<0> {
        SER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave enable  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ser](index.html) module"]
pub struct SER_SPEC;
impl crate::RegisterSpec for SER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ser::R](R) reader structure"]
impl crate::Readable for SER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ser::W](W) writer structure"]
impl crate::Writable for SER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SER to value 0"]
impl crate::Resettable for SER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
