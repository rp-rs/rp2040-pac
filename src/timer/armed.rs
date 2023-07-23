#[doc = "Register `ARMED` reader"]
pub struct R(crate::R<ARMED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ARMED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ARMED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ARMED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ARMED` writer"]
pub struct W(crate::W<ARMED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ARMED_SPEC>;
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
impl From<crate::W<ARMED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ARMED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARMED` reader - "]
pub type ARMED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARMED` writer - "]
pub type ARMED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ARMED_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn armed(&self) -> ARMED_R {
        ARMED_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn armed(&mut self) -> ARMED_W<0> {
        ARMED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Indicates the armed/disarmed status of each alarm.  
 A write to the corresponding ALARMx register arms the alarm.  
 Alarms automatically disarm upon firing, but writing ones here  
 will disarm immediately without waiting to fire.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [armed](index.html) module"]
pub struct ARMED_SPEC;
impl crate::RegisterSpec for ARMED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [armed::R](R) reader structure"]
impl crate::Readable for ARMED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [armed::W](W) writer structure"]
impl crate::Writable for ARMED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0f;
}
#[doc = "`reset()` method sets ARMED to value 0"]
impl crate::Resettable for ARMED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
