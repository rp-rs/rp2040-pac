#[doc = "Register `NVIC_IPR6` reader"]
pub struct R(crate::R<NVIC_IPR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVIC_IPR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVIC_IPR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVIC_IPR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVIC_IPR6` writer"]
pub struct W(crate::W<NVIC_IPR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVIC_IPR6_SPEC>;
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
impl From<crate::W<NVIC_IPR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVIC_IPR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IP_24` reader - Priority of interrupt 24"]
pub type IP_24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_24` writer - Priority of interrupt 24"]
pub type IP_24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR6_SPEC, u8, u8, 2, O>;
#[doc = "Field `IP_25` reader - Priority of interrupt 25"]
pub type IP_25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_25` writer - Priority of interrupt 25"]
pub type IP_25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR6_SPEC, u8, u8, 2, O>;
#[doc = "Field `IP_26` reader - Priority of interrupt 26"]
pub type IP_26_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_26` writer - Priority of interrupt 26"]
pub type IP_26_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR6_SPEC, u8, u8, 2, O>;
#[doc = "Field `IP_27` reader - Priority of interrupt 27"]
pub type IP_27_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_27` writer - Priority of interrupt 27"]
pub type IP_27_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR6_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 6:7 - Priority of interrupt 24"]
    #[inline(always)]
    pub fn ip_24(&self) -> IP_24_R {
        IP_24_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 25"]
    #[inline(always)]
    pub fn ip_25(&self) -> IP_25_R {
        IP_25_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 26"]
    #[inline(always)]
    pub fn ip_26(&self) -> IP_26_R {
        IP_26_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 27"]
    #[inline(always)]
    pub fn ip_27(&self) -> IP_27_R {
        IP_27_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Priority of interrupt 24"]
    #[inline(always)]
    #[must_use]
    pub fn ip_24(&mut self) -> IP_24_W<6> {
        IP_24_W::new(self)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 25"]
    #[inline(always)]
    #[must_use]
    pub fn ip_25(&mut self) -> IP_25_W<14> {
        IP_25_W::new(self)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 26"]
    #[inline(always)]
    #[must_use]
    pub fn ip_26(&mut self) -> IP_26_W<22> {
        IP_26_W::new(self)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 27"]
    #[inline(always)]
    #[must_use]
    pub fn ip_27(&mut self) -> IP_27_W<30> {
        IP_27_W::new(self)
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

For information about available fields see [nvic_ipr6](index.html) module"]
pub struct NVIC_IPR6_SPEC;
impl crate::RegisterSpec for NVIC_IPR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvic_ipr6::R](R) reader structure"]
impl crate::Readable for NVIC_IPR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvic_ipr6::W](W) writer structure"]
impl crate::Writable for NVIC_IPR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR6 to value 0"]
impl crate::Resettable for NVIC_IPR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
