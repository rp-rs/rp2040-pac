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
#[doc = "Field `IP_12` reader - Priority of interrupt 12"]
pub type IP_12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_12` writer - Priority of interrupt 12"]
pub type IP_12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `IP_13` reader - Priority of interrupt 13"]
pub type IP_13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_13` writer - Priority of interrupt 13"]
pub type IP_13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `IP_14` reader - Priority of interrupt 14"]
pub type IP_14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_14` writer - Priority of interrupt 14"]
pub type IP_14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `IP_15` reader - Priority of interrupt 15"]
pub type IP_15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_15` writer - Priority of interrupt 15"]
pub type IP_15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR3_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 6:7 - Priority of interrupt 12"]
    #[inline(always)]
    pub fn ip_12(&self) -> IP_12_R {
        IP_12_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 13"]
    #[inline(always)]
    pub fn ip_13(&self) -> IP_13_R {
        IP_13_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 14"]
    #[inline(always)]
    pub fn ip_14(&self) -> IP_14_R {
        IP_14_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 15"]
    #[inline(always)]
    pub fn ip_15(&self) -> IP_15_R {
        IP_15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Priority of interrupt 12"]
    #[inline(always)]
    #[must_use]
    pub fn ip_12(&mut self) -> IP_12_W<6> {
        IP_12_W::new(self)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 13"]
    #[inline(always)]
    #[must_use]
    pub fn ip_13(&mut self) -> IP_13_W<14> {
        IP_13_W::new(self)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 14"]
    #[inline(always)]
    #[must_use]
    pub fn ip_14(&mut self) -> IP_14_W<22> {
        IP_14_W::new(self)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 15"]
    #[inline(always)]
    #[must_use]
    pub fn ip_15(&mut self) -> IP_15_W<30> {
        IP_15_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR3 to value 0"]
impl crate::Resettable for NVIC_IPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
