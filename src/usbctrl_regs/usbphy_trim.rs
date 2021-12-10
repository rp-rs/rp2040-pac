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
#[doc = "Field `DM_PULLDN_TRIM` reader - Value to drive to USB PHY  
 DM pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
pub struct DM_PULLDN_TRIM_R(crate::FieldReader<u8, u8>);
impl DM_PULLDN_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DM_PULLDN_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DM_PULLDN_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DM_PULLDN_TRIM` writer - Value to drive to USB PHY  
 DM pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
pub struct DM_PULLDN_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_PULLDN_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `DP_PULLDN_TRIM` reader - Value to drive to USB PHY  
 DP pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
pub struct DP_PULLDN_TRIM_R(crate::FieldReader<u8, u8>);
impl DP_PULLDN_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DP_PULLDN_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DP_PULLDN_TRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DP_PULLDN_TRIM` writer - Value to drive to USB PHY  
 DP pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
pub struct DP_PULLDN_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DP_PULLDN_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:12 - Value to drive to USB PHY  
 DM pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    #[inline(always)]
    pub fn dm_pulldn_trim(&self) -> DM_PULLDN_TRIM_R {
        DM_PULLDN_TRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - Value to drive to USB PHY  
 DP pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    #[inline(always)]
    pub fn dp_pulldn_trim(&self) -> DP_PULLDN_TRIM_R {
        DP_PULLDN_TRIM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - Value to drive to USB PHY  
 DM pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    #[inline(always)]
    pub fn dm_pulldn_trim(&mut self) -> DM_PULLDN_TRIM_W {
        DM_PULLDN_TRIM_W { w: self }
    }
    #[doc = "Bits 0:4 - Value to drive to USB PHY  
 DP pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    #[inline(always)]
    pub fn dp_pulldn_trim(&mut self) -> DP_PULLDN_TRIM_W {
        DP_PULLDN_TRIM_W { w: self }
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
}
#[doc = "`reset()` method sets USBPHY_TRIM to value 0x1f1f"]
impl crate::Resettable for USBPHY_TRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f1f
    }
}
