#[doc = "Register `USBPHY_TRIM` reader"]
pub type R = crate::R<USBPHY_TRIM_SPEC>;
#[doc = "Register `USBPHY_TRIM` writer"]
pub type W = crate::W<USBPHY_TRIM_SPEC>;
#[doc = "Field `DP_PULLDN_TRIM` reader - Value to drive to USB PHY  
 DP pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
pub type DP_PULLDN_TRIM_R = crate::FieldReader;
#[doc = "Field `DP_PULLDN_TRIM` writer - Value to drive to USB PHY  
 DP pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
pub type DP_PULLDN_TRIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `DM_PULLDN_TRIM` reader - Value to drive to USB PHY  
 DM pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
pub type DM_PULLDN_TRIM_R = crate::FieldReader;
#[doc = "Field `DM_PULLDN_TRIM` writer - Value to drive to USB PHY  
 DM pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
pub type DM_PULLDN_TRIM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
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
    pub fn dp_pulldn_trim(&mut self) -> DP_PULLDN_TRIM_W<USBPHY_TRIM_SPEC, 0> {
        DP_PULLDN_TRIM_W::new(self)
    }
    #[doc = "Bits 8:12 - Value to drive to USB PHY  
 DM pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    #[inline(always)]
    #[must_use]
    pub fn dm_pulldn_trim(&mut self) -> DM_PULLDN_TRIM_W<USBPHY_TRIM_SPEC, 8> {
        DM_PULLDN_TRIM_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Used to adjust trim values of USB phy pull down resistors.  

You can [`read`](crate::generic::Reg::read) this register and get [`usbphy_trim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphy_trim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBPHY_TRIM_SPEC;
impl crate::RegisterSpec for USBPHY_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbphy_trim::R`](R) reader structure"]
impl crate::Readable for USBPHY_TRIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbphy_trim::W`](W) writer structure"]
impl crate::Writable for USBPHY_TRIM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBPHY_TRIM to value 0x1f1f"]
impl crate::Resettable for USBPHY_TRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f1f;
}
