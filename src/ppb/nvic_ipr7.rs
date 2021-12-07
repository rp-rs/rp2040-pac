#[doc = "Register `NVIC_IPR7` reader"]
pub struct R(crate::R<NVIC_IPR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR7` writer"]
pub struct W(crate::W<NVIC_IPR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR7_SPEC>;
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
impl From<crate::W<NVIC_IPR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IP_31` reader - Priority of interrupt 31"]
pub struct IP_31_R(crate::FieldReader<u8, u8>);
impl IP_31_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_31_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_31` writer - Priority of interrupt 31"]
pub struct IP_31_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_31_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `IP_30` reader - Priority of interrupt 30"]
pub struct IP_30_R(crate::FieldReader<u8, u8>);
impl IP_30_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_30_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_30` writer - Priority of interrupt 30"]
pub struct IP_30_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_30_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `IP_29` reader - Priority of interrupt 29"]
pub struct IP_29_R(crate::FieldReader<u8, u8>);
impl IP_29_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_29_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_29` writer - Priority of interrupt 29"]
pub struct IP_29_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_29_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `IP_28` reader - Priority of interrupt 28"]
pub struct IP_28_R(crate::FieldReader<u8, u8>);
impl IP_28_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_28_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_28` writer - Priority of interrupt 28"]
pub struct IP_28_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Priority of interrupt 31"]
    #[inline(always)]
    pub fn ip_31(&self) -> IP_31_R {
        IP_31_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 30"]
    #[inline(always)]
    pub fn ip_30(&self) -> IP_30_R {
        IP_30_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 29"]
    #[inline(always)]
    pub fn ip_29(&self) -> IP_29_R {
        IP_29_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Priority of interrupt 28"]
    #[inline(always)]
    pub fn ip_28(&self) -> IP_28_R {
        IP_28_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Priority of interrupt 31"]
    #[inline(always)]
    pub fn ip_31(&mut self) -> IP_31_W {
        IP_31_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of interrupt 30"]
    #[inline(always)]
    pub fn ip_30(&mut self) -> IP_30_W {
        IP_30_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority of interrupt 29"]
    #[inline(always)]
    pub fn ip_29(&mut self) -> IP_29_W {
        IP_29_W { w: self }
    }
    #[doc = "Bits 6:7 - Priority of interrupt 28"]
    #[inline(always)]
    pub fn ip_28(&mut self) -> IP_28_W {
        IP_28_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [nvic_ipr7](index.html) module"]
pub struct NVIC_IPR7_SPEC;
impl crate::RegisterSpec for NVIC_IPR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr7::R](R) reader structure"]
impl crate::Readable for NVIC_IPR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr7::W](W) writer structure"]
impl crate::Writable for NVIC_IPR7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVIC_IPR7 to value 0"]
impl crate::Resettable for NVIC_IPR7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
