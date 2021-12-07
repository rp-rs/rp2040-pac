#[doc = "Register `NVIC_IPR5` reader"]
pub struct R(crate::R<NVIC_IPR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR5` writer"]
pub struct W(crate::W<NVIC_IPR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR5_SPEC>;
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
impl From<crate::W<NVIC_IPR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IP_23` reader - Priority of interrupt 23"]
pub struct IP_23_R(crate::FieldReader<u8, u8>);
impl IP_23_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_23_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_23` writer - Priority of interrupt 23"]
pub struct IP_23_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `IP_22` reader - Priority of interrupt 22"]
pub struct IP_22_R(crate::FieldReader<u8, u8>);
impl IP_22_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_22_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_22` writer - Priority of interrupt 22"]
pub struct IP_22_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `IP_21` reader - Priority of interrupt 21"]
pub struct IP_21_R(crate::FieldReader<u8, u8>);
impl IP_21_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_21_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_21` writer - Priority of interrupt 21"]
pub struct IP_21_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `IP_20` reader - Priority of interrupt 20"]
pub struct IP_20_R(crate::FieldReader<u8, u8>);
impl IP_20_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_20_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_20` writer - Priority of interrupt 20"]
pub struct IP_20_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Priority of interrupt 23"]
    #[inline(always)]
    pub fn ip_23(&self) -> IP_23_R {
        IP_23_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 22"]
    #[inline(always)]
    pub fn ip_22(&self) -> IP_22_R {
        IP_22_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 21"]
    #[inline(always)]
    pub fn ip_21(&self) -> IP_21_R {
        IP_21_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Priority of interrupt 20"]
    #[inline(always)]
    pub fn ip_20(&self) -> IP_20_R {
        IP_20_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Priority of interrupt 23"]
    #[inline(always)]
    pub fn ip_23(&mut self) -> IP_23_W {
        IP_23_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of interrupt 22"]
    #[inline(always)]
    pub fn ip_22(&mut self) -> IP_22_W {
        IP_22_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority of interrupt 21"]
    #[inline(always)]
    pub fn ip_21(&mut self) -> IP_21_W {
        IP_21_W { w: self }
    }
    #[doc = "Bits 6:7 - Priority of interrupt 20"]
    #[inline(always)]
    pub fn ip_20(&mut self) -> IP_20_W {
        IP_20_W { w: self }
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

For information about available fields see [nvic_ipr5](index.html) module"]
pub struct NVIC_IPR5_SPEC;
impl crate::RegisterSpec for NVIC_IPR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr5::R](R) reader structure"]
impl crate::Readable for NVIC_IPR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr5::W](W) writer structure"]
impl crate::Writable for NVIC_IPR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVIC_IPR5 to value 0"]
impl crate::Resettable for NVIC_IPR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
