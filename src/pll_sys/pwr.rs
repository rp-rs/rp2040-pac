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
#[doc = "Field `VCOPD` reader - PLL VCO powerdown  
 To save power set high when PLL output not required or bypass=1."]
pub struct VCOPD_R(crate::FieldReader<bool, bool>);
impl VCOPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCOPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VCOPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCOPD` writer - PLL VCO powerdown  
 To save power set high when PLL output not required or bypass=1."]
pub struct VCOPD_W<'a> {
    w: &'a mut W,
}
impl<'a> VCOPD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `POSTDIVPD` reader - PLL post divider powerdown  
 To save power set high when PLL output not required or bypass=1."]
pub struct POSTDIVPD_R(crate::FieldReader<bool, bool>);
impl POSTDIVPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POSTDIVPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POSTDIVPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POSTDIVPD` writer - PLL post divider powerdown  
 To save power set high when PLL output not required or bypass=1."]
pub struct POSTDIVPD_W<'a> {
    w: &'a mut W,
}
impl<'a> POSTDIVPD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DSMPD` reader - PLL DSM powerdown  
 Nothing is achieved by setting this low."]
pub struct DSMPD_R(crate::FieldReader<bool, bool>);
impl DSMPD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSMPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSMPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSMPD` writer - PLL DSM powerdown  
 Nothing is achieved by setting this low."]
pub struct DSMPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DSMPD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PD` reader - PLL powerdown  
 To save power set high when PLL output not required."]
pub struct PD_R(crate::FieldReader<bool, bool>);
impl PD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD` writer - PLL powerdown  
 To save power set high when PLL output not required."]
pub struct PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - PLL VCO powerdown  
 To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    pub fn vcopd(&self) -> VCOPD_R {
        VCOPD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PLL post divider powerdown  
 To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    pub fn postdivpd(&self) -> POSTDIVPD_R {
        POSTDIVPD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PLL DSM powerdown  
 Nothing is achieved by setting this low."]
    #[inline(always)]
    pub fn dsmpd(&self) -> DSMPD_R {
        DSMPD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PLL powerdown  
 To save power set high when PLL output not required."]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - PLL VCO powerdown  
 To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    pub fn vcopd(&mut self) -> VCOPD_W {
        VCOPD_W { w: self }
    }
    #[doc = "Bit 3 - PLL post divider powerdown  
 To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    pub fn postdivpd(&mut self) -> POSTDIVPD_W {
        POSTDIVPD_W { w: self }
    }
    #[doc = "Bit 2 - PLL DSM powerdown  
 Nothing is achieved by setting this low."]
    #[inline(always)]
    pub fn dsmpd(&mut self) -> DSMPD_W {
        DSMPD_W { w: self }
    }
    #[doc = "Bit 0 - PLL powerdown  
 To save power set high when PLL output not required."]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W { w: self }
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
}
#[doc = "`reset()` method sets PWR to value 0x2d"]
impl crate::Resettable for PWR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2d
    }
}
