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
pub struct SER_R(crate::FieldReader<bool, bool>);
impl SER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SER` writer - For each bit:  
 0 -> slave not selected  
 1 -> slave selected"]
pub struct SER_W<'a> {
    w: &'a mut W,
}
impl<'a> SER_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - For each bit:  
 0 -> slave not selected  
 1 -> slave selected"]
    #[inline(always)]
    pub fn ser(&self) -> SER_R {
        SER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - For each bit:  
 0 -> slave not selected  
 1 -> slave selected"]
    #[inline(always)]
    pub fn ser(&mut self) -> SER_W {
        SER_W { w: self }
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
}
#[doc = "`reset()` method sets SER to value 0"]
impl crate::Resettable for SER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
