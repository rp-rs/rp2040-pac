#[doc = "Register `FC0_INTERVAL` reader"]
pub struct R(crate::R<FC0_INTERVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FC0_INTERVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FC0_INTERVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FC0_INTERVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FC0_INTERVAL` writer"]
pub struct W(crate::W<FC0_INTERVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FC0_INTERVAL_SPEC>;
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
impl From<crate::W<FC0_INTERVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FC0_INTERVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FC0_INTERVAL` reader - "]
pub type FC0_INTERVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FC0_INTERVAL` writer - "]
pub type FC0_INTERVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FC0_INTERVAL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn fc0_interval(&self) -> FC0_INTERVAL_R {
        FC0_INTERVAL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn fc0_interval(&mut self) -> FC0_INTERVAL_W<0> {
        FC0_INTERVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval  
 The default gives a test interval of 250us  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [fc0_interval](index.html) module"]
pub struct FC0_INTERVAL_SPEC;
impl crate::RegisterSpec for FC0_INTERVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fc0_interval::R](R) reader structure"]
impl crate::Readable for FC0_INTERVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fc0_interval::W](W) writer structure"]
impl crate::Writable for FC0_INTERVAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FC0_INTERVAL to value 0x08"]
impl crate::Resettable for FC0_INTERVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
