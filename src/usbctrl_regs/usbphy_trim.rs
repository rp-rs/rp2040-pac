#[doc = "Register `USBPHY_TRIM` reader"]
pub struct R(crate::R<USBPHY_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBPHY_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBPHY_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBPHY_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBPHY_TRIM` writer"]
pub struct W(crate::W<USBPHY_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBPHY_TRIM_SPEC>;
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
impl From<crate::W<USBPHY_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBPHY_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DP_PULLDN_TRIM` reader - Value to drive to USB PHY  
 DP pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
pub type DP_PULLDN_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DP_PULLDN_TRIM` writer - Value to drive to USB PHY  
 DP pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
pub type DP_PULLDN_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USBPHY_TRIM_SPEC, u8, u8, 5, O>;
#[doc = "Field `DM_PULLDN_TRIM` reader - Value to drive to USB PHY  
 DM pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
pub type DM_PULLDN_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DM_PULLDN_TRIM` writer - Value to drive to USB PHY  
 DM pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
pub type DM_PULLDN_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USBPHY_TRIM_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Value to drive to USB PHY  
 DP pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    #[inline(always)]
    pub fn dp_pulldn_trim(&self) -> DP_PULLDN_TRIM_R {
        DP_PULLDN_TRIM_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Value to drive to USB PHY  
 DM pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    #[inline(always)]
    pub fn dm_pulldn_trim(&self) -> DM_PULLDN_TRIM_R {
        DM_PULLDN_TRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Value to drive to USB PHY  
 DP pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    #[inline(always)]
    #[must_use]
    pub fn dp_pulldn_trim(&mut self) -> DP_PULLDN_TRIM_W<0> {
        DP_PULLDN_TRIM_W::new(self)
    }
    #[doc = "Bits 8:12 - Value to drive to USB PHY  
 DM pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    #[inline(always)]
    #[must_use]
    pub fn dm_pulldn_trim(&mut self) -> DM_PULLDN_TRIM_W<8> {
        DM_PULLDN_TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Used to adjust trim values of USB phy pull down resistors.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [usbphy_trim](index.html) module"]
pub struct USBPHY_TRIM_SPEC;
impl crate::RegisterSpec for USBPHY_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbphy_trim::R](R) reader structure"]
impl crate::Readable for USBPHY_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbphy_trim::W](W) writer structure"]
impl crate::Writable for USBPHY_TRIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBPHY_TRIM to value 0x1f1f"]
impl crate::Resettable for USBPHY_TRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f1f;
}
