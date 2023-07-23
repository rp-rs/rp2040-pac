#[doc = "Register `NVIC_IPR4` reader"]
pub struct R(crate::R<NVIC_IPR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR4` writer"]
pub struct W(crate::W<NVIC_IPR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR4_SPEC>;
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
impl From<crate::W<NVIC_IPR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IP_16` reader - Priority of interrupt 16"]
pub type IP_16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_16` writer - Priority of interrupt 16"]
pub type IP_16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR4_SPEC, u8, u8, 2, O>;
#[doc = "Field `IP_17` reader - Priority of interrupt 17"]
pub type IP_17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_17` writer - Priority of interrupt 17"]
pub type IP_17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR4_SPEC, u8, u8, 2, O>;
#[doc = "Field `IP_18` reader - Priority of interrupt 18"]
pub type IP_18_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_18` writer - Priority of interrupt 18"]
pub type IP_18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR4_SPEC, u8, u8, 2, O>;
#[doc = "Field `IP_19` reader - Priority of interrupt 19"]
pub type IP_19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_19` writer - Priority of interrupt 19"]
pub type IP_19_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR4_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 6:7 - Priority of interrupt 16"]
    #[inline(always)]
    pub fn ip_16(&self) -> IP_16_R {
        IP_16_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 17"]
    #[inline(always)]
    pub fn ip_17(&self) -> IP_17_R {
        IP_17_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 18"]
    #[inline(always)]
    pub fn ip_18(&self) -> IP_18_R {
        IP_18_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 19"]
    #[inline(always)]
    pub fn ip_19(&self) -> IP_19_R {
        IP_19_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Priority of interrupt 16"]
    #[inline(always)]
    #[must_use]
    pub fn ip_16(&mut self) -> IP_16_W<6> {
        IP_16_W::new(self)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 17"]
    #[inline(always)]
    #[must_use]
    pub fn ip_17(&mut self) -> IP_17_W<14> {
        IP_17_W::new(self)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 18"]
    #[inline(always)]
    #[must_use]
    pub fn ip_18(&mut self) -> IP_18_W<22> {
        IP_18_W::new(self)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 19"]
    #[inline(always)]
    #[must_use]
    pub fn ip_19(&mut self) -> IP_19_W<30> {
        IP_19_W::new(self)
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

For information about available fields see [nvic_ipr4](index.html) module"]
pub struct NVIC_IPR4_SPEC;
impl crate::RegisterSpec for NVIC_IPR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr4::R](R) reader structure"]
impl crate::Readable for NVIC_IPR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr4::W](W) writer structure"]
impl crate::Writable for NVIC_IPR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR4 to value 0"]
impl crate::Resettable for NVIC_IPR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
