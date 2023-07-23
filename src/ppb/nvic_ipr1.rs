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
#[doc = "Field `IP_4` reader - Priority of interrupt 4"]
pub type IP_4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_4` writer - Priority of interrupt 4"]
pub type IP_4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `IP_5` reader - Priority of interrupt 5"]
pub type IP_5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_5` writer - Priority of interrupt 5"]
pub type IP_5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `IP_6` reader - Priority of interrupt 6"]
pub type IP_6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_6` writer - Priority of interrupt 6"]
pub type IP_6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `IP_7` reader - Priority of interrupt 7"]
pub type IP_7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_7` writer - Priority of interrupt 7"]
pub type IP_7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR1_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 6:7 - Priority of interrupt 4"]
    #[inline(always)]
    pub fn ip_4(&self) -> IP_4_R {
        IP_4_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 5"]
    #[inline(always)]
    pub fn ip_5(&self) -> IP_5_R {
        IP_5_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 6"]
    #[inline(always)]
    pub fn ip_6(&self) -> IP_6_R {
        IP_6_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 7"]
    #[inline(always)]
    pub fn ip_7(&self) -> IP_7_R {
        IP_7_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Priority of interrupt 4"]
    #[inline(always)]
    #[must_use]
    pub fn ip_4(&mut self) -> IP_4_W<6> {
        IP_4_W::new(self)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 5"]
    #[inline(always)]
    #[must_use]
    pub fn ip_5(&mut self) -> IP_5_W<14> {
        IP_5_W::new(self)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 6"]
    #[inline(always)]
    #[must_use]
    pub fn ip_6(&mut self) -> IP_6_W<22> {
        IP_6_W::new(self)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 7"]
    #[inline(always)]
    #[must_use]
    pub fn ip_7(&mut self) -> IP_7_W<30> {
        IP_7_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR1 to value 0"]
impl crate::Resettable for NVIC_IPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
