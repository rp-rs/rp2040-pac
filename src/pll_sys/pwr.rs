#[doc = "Register `PWR` reader"]
pub struct R(crate::R<PWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR` writer"]
pub struct W(crate::W<PWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_SPEC>;
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
impl From<crate::W<PWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD` reader - PLL powerdown  
 To save power set high when PLL output not required."]
pub type PD_R = crate::BitReader<bool>;
#[doc = "Field `PD` writer - PLL powerdown  
 To save power set high when PLL output not required."]
pub type PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SPEC, bool, O>;
#[doc = "Field `DSMPD` reader - PLL DSM powerdown  
 Nothing is achieved by setting this low."]
pub type DSMPD_R = crate::BitReader<bool>;
#[doc = "Field `DSMPD` writer - PLL DSM powerdown  
 Nothing is achieved by setting this low."]
pub type DSMPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SPEC, bool, O>;
#[doc = "Field `POSTDIVPD` reader - PLL post divider powerdown  
 To save power set high when PLL output not required or bypass=1."]
pub type POSTDIVPD_R = crate::BitReader<bool>;
#[doc = "Field `POSTDIVPD` writer - PLL post divider powerdown  
 To save power set high when PLL output not required or bypass=1."]
pub type POSTDIVPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SPEC, bool, O>;
#[doc = "Field `VCOPD` reader - PLL VCO powerdown  
 To save power set high when PLL output not required or bypass=1."]
pub type VCOPD_R = crate::BitReader<bool>;
#[doc = "Field `VCOPD` writer - PLL VCO powerdown  
 To save power set high when PLL output not required or bypass=1."]
pub type VCOPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PWR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PLL powerdown  
 To save power set high when PLL output not required."]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PLL DSM powerdown  
 Nothing is achieved by setting this low."]
    #[inline(always)]
    pub fn dsmpd(&self) -> DSMPD_R {
        DSMPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PLL post divider powerdown  
 To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    pub fn postdivpd(&self) -> POSTDIVPD_R {
        POSTDIVPD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL VCO powerdown  
 To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    pub fn vcopd(&self) -> VCOPD_R {
        VCOPD_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL powerdown  
 To save power set high when PLL output not required."]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PD_W<0> {
        PD_W::new(self)
    }
    #[doc = "Bit 2 - PLL DSM powerdown  
 Nothing is achieved by setting this low."]
    #[inline(always)]
    #[must_use]
    pub fn dsmpd(&mut self) -> DSMPD_W<2> {
        DSMPD_W::new(self)
    }
    #[doc = "Bit 3 - PLL post divider powerdown  
 To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    #[must_use]
    pub fn postdivpd(&mut self) -> POSTDIVPD_W<3> {
        POSTDIVPD_W::new(self)
    }
    #[doc = "Bit 5 - PLL VCO powerdown  
 To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    #[must_use]
    pub fn vcopd(&mut self) -> VCOPD_W<5> {
        VCOPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the PLL power modes.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [pwr](index.html) module"]
pub struct PWR_SPEC;
impl crate::RegisterSpec for PWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr::R](R) reader structure"]
impl crate::Readable for PWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr::W](W) writer structure"]
impl crate::Writable for PWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWR to value 0x2d"]
impl crate::Resettable for PWR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2d;
}
