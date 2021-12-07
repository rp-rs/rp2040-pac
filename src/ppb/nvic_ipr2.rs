#[doc = "Register `NVIC_IPR2` reader"]
pub struct R(crate::R<NVIC_IPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR2` writer"]
pub struct W(crate::W<NVIC_IPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR2_SPEC>;
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
impl From<crate::W<NVIC_IPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IP_11` reader - Priority of interrupt 11"]
pub struct IP_11_R(crate::FieldReader<u8, u8>);
impl IP_11_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_11` writer - Priority of interrupt 11"]
pub struct IP_11_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `IP_10` reader - Priority of interrupt 10"]
pub struct IP_10_R(crate::FieldReader<u8, u8>);
impl IP_10_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_10` writer - Priority of interrupt 10"]
pub struct IP_10_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `IP_9` reader - Priority of interrupt 9"]
pub struct IP_9_R(crate::FieldReader<u8, u8>);
impl IP_9_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_9` writer - Priority of interrupt 9"]
pub struct IP_9_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `IP_8` reader - Priority of interrupt 8"]
pub struct IP_8_R(crate::FieldReader<u8, u8>);
impl IP_8_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_8` writer - Priority of interrupt 8"]
pub struct IP_8_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Priority of interrupt 11"]
    #[inline(always)]
    pub fn ip_11(&self) -> IP_11_R {
        IP_11_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 10"]
    #[inline(always)]
    pub fn ip_10(&self) -> IP_10_R {
        IP_10_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 9"]
    #[inline(always)]
    pub fn ip_9(&self) -> IP_9_R {
        IP_9_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Priority of interrupt 8"]
    #[inline(always)]
    pub fn ip_8(&self) -> IP_8_R {
        IP_8_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Priority of interrupt 11"]
    #[inline(always)]
    pub fn ip_11(&mut self) -> IP_11_W {
        IP_11_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of interrupt 10"]
    #[inline(always)]
    pub fn ip_10(&mut self) -> IP_10_W {
        IP_10_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority of interrupt 9"]
    #[inline(always)]
    pub fn ip_9(&mut self) -> IP_9_W {
        IP_9_W { w: self }
    }
    #[doc = "Bits 6:7 - Priority of interrupt 8"]
    #[inline(always)]
    pub fn ip_8(&mut self) -> IP_8_W {
        IP_8_W { w: self }
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

For information about available fields see [nvic_ipr2](index.html) module"]
pub struct NVIC_IPR2_SPEC;
impl crate::RegisterSpec for NVIC_IPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr2::R](R) reader structure"]
impl crate::Readable for NVIC_IPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr2::W](W) writer structure"]
impl crate::Writable for NVIC_IPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVIC_IPR2 to value 0"]
impl crate::Resettable for NVIC_IPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
