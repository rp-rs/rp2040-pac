#[doc = "Reader of register USBPHY_TRIM"]
pub type R = crate::R<u32, super::USBPHY_TRIM>;
#[doc = "Writer for register USBPHY_TRIM"]
pub type W = crate::W<u32, super::USBPHY_TRIM>;
#[doc = "Register USBPHY_TRIM `reset()`'s with value 0x1f1f"]
impl crate::ResetValue for super::USBPHY_TRIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f1f
    }
}
#[doc = "Reader of field `DM_PULLDN_TRIM`"]
pub type DM_PULLDN_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DM_PULLDN_TRIM`"]
pub struct DM_PULLDN_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DM_PULLDN_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DP_PULLDN_TRIM`"]
pub type DP_PULLDN_TRIM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DP_PULLDN_TRIM`"]
pub struct DP_PULLDN_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DP_PULLDN_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:12 - Value to drive to USB PHY\\n DM pulldown resistor trim control\\n Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    #[inline(always)]
    pub fn dm_pulldn_trim(&self) -> DM_PULLDN_TRIM_R {
        DM_PULLDN_TRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - Value to drive to USB PHY\\n DP pulldown resistor trim control\\n Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    #[inline(always)]
    pub fn dp_pulldn_trim(&self) -> DP_PULLDN_TRIM_R {
        DP_PULLDN_TRIM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - Value to drive to USB PHY\\n DM pulldown resistor trim control\\n Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    #[inline(always)]
    pub fn dm_pulldn_trim(&mut self) -> DM_PULLDN_TRIM_W {
        DM_PULLDN_TRIM_W { w: self }
    }
    #[doc = "Bits 0:4 - Value to drive to USB PHY\\n DP pulldown resistor trim control\\n Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    #[inline(always)]
    pub fn dp_pulldn_trim(&mut self) -> DP_PULLDN_TRIM_W {
        DP_PULLDN_TRIM_W { w: self }
    }
}
