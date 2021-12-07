#[doc = "Register `NVIC_IPR0` reader"]
pub struct R(crate::R<NVIC_IPR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR0` writer"]
pub struct W(crate::W<NVIC_IPR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR0_SPEC>;
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
impl From<crate::W<NVIC_IPR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IP_3` reader - Priority of interrupt 3"]
pub struct IP_3_R(crate::FieldReader<u8, u8>);
impl IP_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_3` writer - Priority of interrupt 3"]
pub struct IP_3_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `IP_2` reader - Priority of interrupt 2"]
pub struct IP_2_R(crate::FieldReader<u8, u8>);
impl IP_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_2` writer - Priority of interrupt 2"]
pub struct IP_2_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `IP_1` reader - Priority of interrupt 1"]
pub struct IP_1_R(crate::FieldReader<u8, u8>);
impl IP_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_1` writer - Priority of interrupt 1"]
pub struct IP_1_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `IP_0` reader - Priority of interrupt 0"]
pub struct IP_0_R(crate::FieldReader<u8, u8>);
impl IP_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IP_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_0` writer - Priority of interrupt 0"]
pub struct IP_0_W<'a> {
    w: &'a mut W,
}
impl<'a> IP_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Priority of interrupt 3"]
    #[inline(always)]
    pub fn ip_3(&self) -> IP_3_R {
        IP_3_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 2"]
    #[inline(always)]
    pub fn ip_2(&self) -> IP_2_R {
        IP_2_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 1"]
    #[inline(always)]
    pub fn ip_1(&self) -> IP_1_R {
        IP_1_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Priority of interrupt 0"]
    #[inline(always)]
    pub fn ip_0(&self) -> IP_0_R {
        IP_0_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Priority of interrupt 3"]
    #[inline(always)]
    pub fn ip_3(&mut self) -> IP_3_W {
        IP_3_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of interrupt 2"]
    #[inline(always)]
    pub fn ip_2(&mut self) -> IP_2_W {
        IP_2_W { w: self }
    }
    #[doc = "Bits 14:15 - Priority of interrupt 1"]
    #[inline(always)]
    pub fn ip_1(&mut self) -> IP_1_W {
        IP_1_W { w: self }
    }
    #[doc = "Bits 6:7 - Priority of interrupt 0"]
    #[inline(always)]
    pub fn ip_0(&mut self) -> IP_0_W {
        IP_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest.  
 Note: Writing 1 to an NVIC_ICPR bit does not affect the active state of the corresponding interrupt.  
 These registers are only word-accessible  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [nvic_ipr0](index.html) module"]
pub struct NVIC_IPR0_SPEC;
impl crate::RegisterSpec for NVIC_IPR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr0::R](R) reader structure"]
impl crate::Readable for NVIC_IPR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr0::W](W) writer structure"]
impl crate::Writable for NVIC_IPR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVIC_IPR0 to value 0"]
impl crate::Resettable for NVIC_IPR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
