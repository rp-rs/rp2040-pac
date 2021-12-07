#[doc = "Register `NVIC_IPR3` reader"]
pub struct R(crate::R<NVIC_IPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR3` writer"]
pub struct W(crate::W<NVIC_IPR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR3_SPEC>;
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
impl From<crate::W<NVIC_IPR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IP_15` reader - Priority of interrupt 15"]
pub struct IP_15_R(crate::FieldReader<u8, u8>);
impl IP_15_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_15` writer - Priority of interrupt 15"]
pub struct IP_15_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `IP_14` reader - Priority of interrupt 14"]
pub struct IP_14_R(crate::FieldReader<u8, u8>);
impl IP_14_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_14` writer - Priority of interrupt 14"]
pub struct IP_14_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `IP_13` reader - Priority of interrupt 13"]
pub struct IP_13_R(crate::FieldReader<u8, u8>);
impl IP_13_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_13` writer - Priority of interrupt 13"]
pub struct IP_13_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `IP_12` reader - Priority of interrupt 12"]
pub struct IP_12_R(crate::FieldReader<u8, u8>);
impl IP_12_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_12` writer - Priority of interrupt 12"]
pub struct IP_12_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Priority of interrupt 15"]
    #[inline(always)]
    pub fn ip_15(&self) -> IP_15_R {
        IP_15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 14"]
    #[inline(always)]
    pub fn ip_14(&self) -> IP_14_R {
        IP_14_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 13"]
    #[inline(always)]
    pub fn ip_13(&self) -> IP_13_R {
        IP_13_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Priority of interrupt 12"]
    #[inline(always)]
    pub fn ip_12(&self) -> IP_12_R {
        IP_12_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Priority of interrupt 15"]
    #[inline(always)]
    pub fn ip_15(&mut self) -> IP_15_W {
        IP_15_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of interrupt 14"]
    #[inline(always)]
    pub fn ip_14(&mut self) -> IP_14_W {
        IP_14_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority of interrupt 13"]
    #[inline(always)]
    pub fn ip_13(&mut self) -> IP_13_W {
        IP_13_W { w: self }
    }
    #[doc = "Bits 6:7 - Priority of interrupt 12"]
    #[inline(always)]
    pub fn ip_12(&mut self) -> IP_12_W {
        IP_12_W { w: self }
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

For information about available fields see [nvic_ipr3](index.html) module"]
pub struct NVIC_IPR3_SPEC;
impl crate::RegisterSpec for NVIC_IPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr3::R](R) reader structure"]
impl crate::Readable for NVIC_IPR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr3::W](W) writer structure"]
impl crate::Writable for NVIC_IPR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVIC_IPR3 to value 0"]
impl crate::Resettable for NVIC_IPR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
