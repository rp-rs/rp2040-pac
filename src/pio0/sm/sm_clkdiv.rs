#[doc = "Register `SM_CLKDIV` reader"]
pub struct R(crate::R<SM_CLKDIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SM_CLKDIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SM_CLKDIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SM_CLKDIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SM_CLKDIV` writer"]
pub struct W(crate::W<SM_CLKDIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SM_CLKDIV_SPEC>;
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
impl From<crate::W<SM_CLKDIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SM_CLKDIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRAC` reader - Fractional part of clock divisor"]
pub type FRAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRAC` writer - Fractional part of clock divisor"]
pub type FRAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SM_CLKDIV_SPEC, u8, u8, 8, O>;
#[doc = "Field `INT` reader - Effective frequency is sysclk/(int + frac/256).  
 Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0."]
pub type INT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INT` writer - Effective frequency is sysclk/(int + frac/256).  
 Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0."]
pub type INT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SM_CLKDIV_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 8:15 - Fractional part of clock divisor"]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Effective frequency is sysclk/(int + frac/256).  
 Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0."]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:15 - Fractional part of clock divisor"]
    #[inline(always)]
    #[must_use]
    pub fn frac(&mut self) -> FRAC_W<8> {
        FRAC_W::new(self)
    }
    #[doc = "Bits 16:31 - Effective frequency is sysclk/(int + frac/256).  
 Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0."]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<16> {
        INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock divisor register for state machine 0  
 Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sm_clkdiv](index.html) module"]
pub struct SM_CLKDIV_SPEC;
impl crate::RegisterSpec for SM_CLKDIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sm_clkdiv::R](R) reader structure"]
impl crate::Readable for SM_CLKDIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sm_clkdiv::W](W) writer structure"]
impl crate::Writable for SM_CLKDIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SM_CLKDIV to value 0x0001_0000"]
impl crate::Resettable for SM_CLKDIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
