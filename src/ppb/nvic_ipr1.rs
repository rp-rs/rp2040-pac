#[doc = "Register `NVIC_IPR1` reader"]
pub struct R(crate::R<NVIC_IPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR1` writer"]
pub struct W(crate::W<NVIC_IPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR1_SPEC>;
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
impl From<crate::W<NVIC_IPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IP_7` reader - Priority of interrupt 7"]
pub struct IP_7_R(crate::FieldReader<u8, u8>);
impl IP_7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_7` writer - Priority of interrupt 7"]
pub struct IP_7_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `IP_6` reader - Priority of interrupt 6"]
pub struct IP_6_R(crate::FieldReader<u8, u8>);
impl IP_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_6` writer - Priority of interrupt 6"]
pub struct IP_6_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `IP_5` reader - Priority of interrupt 5"]
pub struct IP_5_R(crate::FieldReader<u8, u8>);
impl IP_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_5` writer - Priority of interrupt 5"]
pub struct IP_5_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `IP_4` reader - Priority of interrupt 4"]
pub struct IP_4_R(crate::FieldReader<u8, u8>);
impl IP_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_4` writer - Priority of interrupt 4"]
pub struct IP_4_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Priority of interrupt 7"]
    #[inline(always)]
    pub fn ip_7(&self) -> IP_7_R {
        IP_7_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 6"]
    #[inline(always)]
    pub fn ip_6(&self) -> IP_6_R {
        IP_6_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 5"]
    #[inline(always)]
    pub fn ip_5(&self) -> IP_5_R {
        IP_5_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Priority of interrupt 4"]
    #[inline(always)]
    pub fn ip_4(&self) -> IP_4_R {
        IP_4_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Priority of interrupt 7"]
    #[inline(always)]
    pub fn ip_7(&mut self) -> IP_7_W {
        IP_7_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of interrupt 6"]
    #[inline(always)]
    pub fn ip_6(&mut self) -> IP_6_W {
        IP_6_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority of interrupt 5"]
    #[inline(always)]
    pub fn ip_5(&mut self) -> IP_5_W {
        IP_5_W { w: self }
    }
    #[doc = "Bits 6:7 - Priority of interrupt 4"]
    #[inline(always)]
    pub fn ip_4(&mut self) -> IP_4_W {
        IP_4_W { w: self }
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

For information about available fields see [nvic_ipr1](index.html) module"]
pub struct NVIC_IPR1_SPEC;
impl crate::RegisterSpec for NVIC_IPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr1::R](R) reader structure"]
impl crate::Readable for NVIC_IPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr1::W](W) writer structure"]
impl crate::Writable for NVIC_IPR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVIC_IPR1 to value 0"]
impl crate::Resettable for NVIC_IPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
