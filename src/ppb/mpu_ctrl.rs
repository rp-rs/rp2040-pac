#[doc = "Reader of register MPU_CTRL"]
pub type R = crate::R<u32, super::MPU_CTRL>;
#[doc = "Writer for register MPU_CTRL"]
pub type W = crate::W<u32, super::MPU_CTRL>;
#[doc = "Register MPU_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::MPU_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRIVDEFENA`"]
pub type PRIVDEFENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIVDEFENA`"]
pub struct PRIVDEFENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIVDEFENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `HFNMIENA`"]
pub type HFNMIENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HFNMIENA`"]
pub struct HFNMIENA_W<'a> {
    w: &'a mut W,
}
impl<'a> HFNMIENA_W<'a> {
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
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
    #[doc = "Bit 2 - Controls whether the default memory map is enabled as a background region for privileged accesses. This bit is ignored when ENABLE is clear.\\n 0 = If the MPU is enabled, disables use of the default memory map. Any memory access to a location not\\n covered by any enabled region causes a fault.\\n 1 = If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses.\\n When enabled, the background region acts as if it is region number -1. Any region that is defined and enabled has priority over this default map."]
    #[inline(always)]
    pub fn privdefena(&self) -> PRIVDEFENA_R {
        PRIVDEFENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Controls the use of the MPU for HardFaults and NMIs. Setting this bit when ENABLE is clear results in UNPREDICTABLE behaviour.\\n When the MPU is enabled:\\n 0 = MPU is disabled during HardFault and NMI handlers, regardless of the value of the ENABLE bit.\\n 1 = the MPU is enabled during HardFault and NMI handlers."]
    #[inline(always)]
    pub fn hfnmiena(&self) -> HFNMIENA_R {
        HFNMIENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enables the MPU. If the MPU is disabled, privileged and unprivileged accesses use the default memory map.\\n 0 = MPU disabled.\\n 1 = MPU enabled."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Controls whether the default memory map is enabled as a background region for privileged accesses. This bit is ignored when ENABLE is clear.\\n 0 = If the MPU is enabled, disables use of the default memory map. Any memory access to a location not\\n covered by any enabled region causes a fault.\\n 1 = If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses.\\n When enabled, the background region acts as if it is region number -1. Any region that is defined and enabled has priority over this default map."]
    #[inline(always)]
    pub fn privdefena(&mut self) -> PRIVDEFENA_W {
        PRIVDEFENA_W { w: self }
    }
    #[doc = "Bit 1 - Controls the use of the MPU for HardFaults and NMIs. Setting this bit when ENABLE is clear results in UNPREDICTABLE behaviour.\\n When the MPU is enabled:\\n 0 = MPU is disabled during HardFault and NMI handlers, regardless of the value of the ENABLE bit.\\n 1 = the MPU is enabled during HardFault and NMI handlers."]
    #[inline(always)]
    pub fn hfnmiena(&mut self) -> HFNMIENA_W {
        HFNMIENA_W { w: self }
    }
    #[doc = "Bit 0 - Enables the MPU. If the MPU is disabled, privileged and unprivileged accesses use the default memory map.\\n 0 = MPU disabled.\\n 1 = MPU enabled."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
