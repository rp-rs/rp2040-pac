#[doc = "Register `FC0_MAX_KHZ` reader"]
pub struct R(crate::R<FC0_MAX_KHZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FC0_MAX_KHZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FC0_MAX_KHZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FC0_MAX_KHZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FC0_MAX_KHZ` writer"]
pub struct W(crate::W<FC0_MAX_KHZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FC0_MAX_KHZ_SPEC>;
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
impl From<crate::W<FC0_MAX_KHZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FC0_MAX_KHZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FC0_MAX_KHZ` reader - "]
pub struct FC0_MAX_KHZ_R(crate::FieldReader<u32, u32>);
impl FC0_MAX_KHZ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        FC0_MAX_KHZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC0_MAX_KHZ_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC0_MAX_KHZ` writer - "]
pub struct FC0_MAX_KHZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FC0_MAX_KHZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | (value as u32 & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn fc0_max_khz(&self) -> FC0_MAX_KHZ_R {
        FC0_MAX_KHZ_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24"]
    #[inline(always)]
    pub fn fc0_max_khz(&mut self) -> FC0_MAX_KHZ_W {
        FC0_MAX_KHZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Maximum pass frequency in kHz. This is optional. Set to 0x1ffffff if you are not using the pass/fail flags  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [fc0_max_khz](index.html) module"]
pub struct FC0_MAX_KHZ_SPEC;
impl crate::RegisterSpec for FC0_MAX_KHZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fc0_max_khz::R](R) reader structure"]
impl crate::Readable for FC0_MAX_KHZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fc0_max_khz::W](W) writer structure"]
impl crate::Writable for FC0_MAX_KHZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FC0_MAX_KHZ to value 0x01ff_ffff"]
impl crate::Resettable for FC0_MAX_KHZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01ff_ffff
    }
}
