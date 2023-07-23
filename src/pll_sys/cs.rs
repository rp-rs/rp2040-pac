#[doc = "Register `CS` reader"]
pub struct R(crate::R<CS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS` writer"]
pub struct W(crate::W<CS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CS_SPEC>;
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
impl From<crate::W<CS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFDIV` reader - Divides the PLL input reference clock.  
 Behaviour is undefined for div=0.  
 PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it."]
pub type REFDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REFDIV` writer - Divides the PLL input reference clock.  
 Behaviour is undefined for div=0.  
 PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it."]
pub type REFDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CS_SPEC, u8, u8, 6, O>;
#[doc = "Field `BYPASS` reader - Passes the reference clock to the output instead of the divided VCO. The VCO continues to run so the user can switch between the reference clock and the divided VCO but the output will glitch when doing so."]
pub type BYPASS_R = crate::BitReader<bool>;
#[doc = "Field `BYPASS` writer - Passes the reference clock to the output instead of the divided VCO. The VCO continues to run so the user can switch between the reference clock and the divided VCO but the output will glitch when doing so."]
pub type BYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, bool, O>;
#[doc = "Field `LOCK` reader - PLL is locked"]
pub type LOCK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:5 - Divides the PLL input reference clock.  
 Behaviour is undefined for div=0.  
 PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it."]
    #[inline(always)]
    pub fn refdiv(&self) -> REFDIV_R {
        REFDIV_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Passes the reference clock to the output instead of the divided VCO. The VCO continues to run so the user can switch between the reference clock and the divided VCO but the output will glitch when doing so."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - PLL is locked"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Divides the PLL input reference clock.  
 Behaviour is undefined for div=0.  
 PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it."]
    #[inline(always)]
    #[must_use]
    pub fn refdiv(&mut self) -> REFDIV_W<0> {
        REFDIV_W::new(self)
    }
    #[doc = "Bit 8 - Passes the reference clock to the output instead of the divided VCO. The VCO continues to run so the user can switch between the reference clock and the divided VCO but the output will glitch when doing so."]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BYPASS_W<8> {
        BYPASS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control and Status  
 GENERAL CONSTRAINTS:  
 Reference clock frequency min=5MHz, max=800MHz  
 Feedback divider min=16, max=320  
 VCO frequency min=750MHz, max=1600MHz  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [cs](index.html) module"]
pub struct CS_SPEC;
impl crate::RegisterSpec for CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cs::R](R) reader structure"]
impl crate::Readable for CS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cs::W](W) writer structure"]
impl crate::Writable for CS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CS to value 0x01"]
impl crate::Resettable for CS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
