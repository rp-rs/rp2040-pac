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
#[doc = "Field `IP_8` reader - Priority of interrupt 8"]
pub type IP_8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_8` writer - Priority of interrupt 8"]
pub type IP_8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `IP_9` reader - Priority of interrupt 9"]
pub type IP_9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_9` writer - Priority of interrupt 9"]
pub type IP_9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `IP_10` reader - Priority of interrupt 10"]
pub type IP_10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_10` writer - Priority of interrupt 10"]
pub type IP_10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `IP_11` reader - Priority of interrupt 11"]
pub type IP_11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_11` writer - Priority of interrupt 11"]
pub type IP_11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 6:7 - Priority of interrupt 8"]
    #[inline(always)]
    pub fn ip_8(&self) -> IP_8_R {
        IP_8_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 9"]
    #[inline(always)]
    pub fn ip_9(&self) -> IP_9_R {
        IP_9_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 10"]
    #[inline(always)]
    pub fn ip_10(&self) -> IP_10_R {
        IP_10_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 11"]
    #[inline(always)]
    pub fn ip_11(&self) -> IP_11_R {
        IP_11_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Priority of interrupt 8"]
    #[inline(always)]
    #[must_use]
    pub fn ip_8(&mut self) -> IP_8_W<6> {
        IP_8_W::new(self)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 9"]
    #[inline(always)]
    #[must_use]
    pub fn ip_9(&mut self) -> IP_9_W<14> {
        IP_9_W::new(self)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 10"]
    #[inline(always)]
    #[must_use]
    pub fn ip_10(&mut self) -> IP_10_W<22> {
        IP_10_W::new(self)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 11"]
    #[inline(always)]
    #[must_use]
    pub fn ip_11(&mut self) -> IP_11_W<30> {
        IP_11_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR2 to value 0"]
impl crate::Resettable for NVIC_IPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
