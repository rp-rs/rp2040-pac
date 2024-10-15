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
pub type DP_PULLDN_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DM_PULLDN_TRIM` reader - Value to drive to USB PHY  
 DM pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
pub type DM_PULLDN_TRIM_R = crate::FieldReader;
#[doc = "Field `DM_PULLDN_TRIM` writer - Value to drive to USB PHY  
 DM pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
pub type DM_PULLDN_TRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
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
    pub fn dp_pulldn_trim(&mut self) -> DP_PULLDN_TRIM_W<USBPHY_TRIM_SPEC> {
        DP_PULLDN_TRIM_W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Value to drive to USB PHY  
 DM pulldown resistor trim control  
 Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    #[inline(always)]
    #[must_use]
    pub fn dm_pulldn_trim(&mut self) -> DM_PULLDN_TRIM_W<USBPHY_TRIM_SPEC> {
        DM_PULLDN_TRIM_W::new(self, 8)
    }
}
#[doc = "Used to adjust trim values of USB phy pull down resistors.  

You can [`read`](crate::Reg::read) this register and get [`usbphy_trim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbphy_trim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBPHY_TRIM_SPEC;
impl crate::RegisterSpec for USBPHY_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbphy_trim::R`](R) reader structure"]
impl crate::Readable for USBPHY_TRIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usbphy_trim::W`](W) writer structure"]
impl crate::Writable for USBPHY_TRIM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBPHY_TRIM to value 0x1f1f"]
impl crate::Resettable for USBPHY_TRIM_SPEC {
    const RESET_VALUE: u32 = 0x1f1f;
}
