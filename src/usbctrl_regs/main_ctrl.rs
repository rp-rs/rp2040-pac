#[doc = "Reader of register MAIN_CTRL"]
pub type R = crate::R<u32, super::MAIN_CTRL>;
#[doc = "Writer for register MAIN_CTRL"]
pub type W = crate::W<u32, super::MAIN_CTRL>;
#[doc = "Register MAIN_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MAIN_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SIM_TIMING`"]
pub type SIM_TIMING_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SIM_TIMING`"]
pub struct SIM_TIMING_W<'a> {
    w: &'a mut W,
}
impl<'a> SIM_TIMING_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `HOST_NDEVICE`"]
pub type HOST_NDEVICE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_NDEVICE`"]
pub struct HOST_NDEVICE_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_NDEVICE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `CONTROLLER_EN`"]
pub type CONTROLLER_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CONTROLLER_EN`"]
pub struct CONTROLLER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CONTROLLER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Reduced timings for simulation"]
    #[inline(always)]
    pub fn sim_timing(&self) -> SIM_TIMING_R {
        SIM_TIMING_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Device mode = 0, Host mode = 1"]
    #[inline(always)]
    pub fn host_ndevice(&self) -> HOST_NDEVICE_R {
        HOST_NDEVICE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable controller"]
    #[inline(always)]
    pub fn controller_en(&self) -> CONTROLLER_EN_R {
        CONTROLLER_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Reduced timings for simulation"]
    #[inline(always)]
    pub fn sim_timing(&mut self) -> SIM_TIMING_W {
        SIM_TIMING_W { w: self }
    }
    #[doc = "Bit 1 - Device mode = 0, Host mode = 1"]
    #[inline(always)]
    pub fn host_ndevice(&mut self) -> HOST_NDEVICE_W {
        HOST_NDEVICE_W { w: self }
    }
    #[doc = "Bit 0 - Enable controller"]
    #[inline(always)]
    pub fn controller_en(&mut self) -> CONTROLLER_EN_W {
        CONTROLLER_EN_W { w: self }
    }
}
