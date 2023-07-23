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
#[doc = "Field `IP_20` reader - Priority of interrupt 20"]
pub type IP_20_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_20` writer - Priority of interrupt 20"]
pub type IP_20_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR5_SPEC, u8, u8, 2, O>;
#[doc = "Field `IP_21` reader - Priority of interrupt 21"]
pub type IP_21_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_21` writer - Priority of interrupt 21"]
pub type IP_21_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR5_SPEC, u8, u8, 2, O>;
#[doc = "Field `IP_22` reader - Priority of interrupt 22"]
pub type IP_22_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_22` writer - Priority of interrupt 22"]
pub type IP_22_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR5_SPEC, u8, u8, 2, O>;
#[doc = "Field `IP_23` reader - Priority of interrupt 23"]
pub type IP_23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IP_23` writer - Priority of interrupt 23"]
pub type IP_23_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NVIC_IPR5_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 6:7 - Priority of interrupt 20"]
    #[inline(always)]
    pub fn ip_20(&self) -> IP_20_R {
        IP_20_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 21"]
    #[inline(always)]
    pub fn ip_21(&self) -> IP_21_R {
        IP_21_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 22"]
    #[inline(always)]
    pub fn ip_22(&self) -> IP_22_R {
        IP_22_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 23"]
    #[inline(always)]
    pub fn ip_23(&self) -> IP_23_R {
        IP_23_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Priority of interrupt 20"]
    #[inline(always)]
    #[must_use]
    pub fn ip_20(&mut self) -> IP_20_W<6> {
        IP_20_W::new(self)
    }
    #[doc = "Bits 14:15 - Priority of interrupt 21"]
    #[inline(always)]
    #[must_use]
    pub fn ip_21(&mut self) -> IP_21_W<14> {
        IP_21_W::new(self)
    }
    #[doc = "Bits 22:23 - Priority of interrupt 22"]
    #[inline(always)]
    #[must_use]
    pub fn ip_22(&mut self) -> IP_22_W<22> {
        IP_22_W::new(self)
    }
    #[doc = "Bits 30:31 - Priority of interrupt 23"]
    #[inline(always)]
    #[must_use]
    pub fn ip_23(&mut self) -> IP_23_W<30> {
        IP_23_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NVIC_IPR5 to value 0"]
impl crate::Resettable for NVIC_IPR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
