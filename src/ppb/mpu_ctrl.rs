#[doc = "Register `MPU_CTRL` reader"]
pub struct R(crate::R<MPU_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MPU_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MPU_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MPU_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MPU_CTRL` writer"]
pub struct W(crate::W<MPU_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MPU_CTRL_SPEC>;
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
impl From<crate::W<MPU_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MPU_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIVDEFENA` reader - Controls whether the default memory map is enabled as a background region for privileged accesses. This bit is ignored when ENABLE is clear.  
 0 = If the MPU is enabled, disables use of the default memory map. Any memory access to a location not  
 covered by any enabled region causes a fault.  
 1 = If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses.  
 When enabled, the background region acts as if it is region number -1. Any region that is defined and enabled has priority over this default map."]
pub struct PRIVDEFENA_R(crate::FieldReader<bool, bool>);
impl PRIVDEFENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRIVDEFENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIVDEFENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIVDEFENA` writer - Controls whether the default memory map is enabled as a background region for privileged accesses. This bit is ignored when ENABLE is clear.  
 0 = If the MPU is enabled, disables use of the default memory map. Any memory access to a location not  
 covered by any enabled region causes a fault.  
 1 = If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses.  
 When enabled, the background region acts as if it is region number -1. Any region that is defined and enabled has priority over this default map."]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `HFNMIENA` reader - Controls the use of the MPU for HardFaults and NMIs. Setting this bit when ENABLE is clear results in UNPREDICTABLE behaviour.  
 When the MPU is enabled:  
 0 = MPU is disabled during HardFault and NMI handlers, regardless of the value of the ENABLE bit.  
 1 = the MPU is enabled during HardFault and NMI handlers."]
pub struct HFNMIENA_R(crate::FieldReader<bool, bool>);
impl HFNMIENA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HFNMIENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFNMIENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFNMIENA` writer - Controls the use of the MPU for HardFaults and NMIs. Setting this bit when ENABLE is clear results in UNPREDICTABLE behaviour.  
 When the MPU is enabled:  
 0 = MPU is disabled during HardFault and NMI handlers, regardless of the value of the ENABLE bit.  
 1 = the MPU is enabled during HardFault and NMI handlers."]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `ENABLE` reader - Enables the MPU. If the MPU is disabled, privileged and unprivileged accesses use the default memory map.  
 0 = MPU disabled.  
 1 = MPU enabled."]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Enables the MPU. If the MPU is disabled, privileged and unprivileged accesses use the default memory map.  
 0 = MPU disabled.  
 1 = MPU enabled."]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Controls whether the default memory map is enabled as a background region for privileged accesses. This bit is ignored when ENABLE is clear.  
 0 = If the MPU is enabled, disables use of the default memory map. Any memory access to a location not  
 covered by any enabled region causes a fault.  
 1 = If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses.  
 When enabled, the background region acts as if it is region number -1. Any region that is defined and enabled has priority over this default map."]
    #[inline(always)]
    pub fn privdefena(&self) -> PRIVDEFENA_R {
        PRIVDEFENA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Controls the use of the MPU for HardFaults and NMIs. Setting this bit when ENABLE is clear results in UNPREDICTABLE behaviour.  
 When the MPU is enabled:  
 0 = MPU is disabled during HardFault and NMI handlers, regardless of the value of the ENABLE bit.  
 1 = the MPU is enabled during HardFault and NMI handlers."]
    #[inline(always)]
    pub fn hfnmiena(&self) -> HFNMIENA_R {
        HFNMIENA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enables the MPU. If the MPU is disabled, privileged and unprivileged accesses use the default memory map.  
 0 = MPU disabled.  
 1 = MPU enabled."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Controls whether the default memory map is enabled as a background region for privileged accesses. This bit is ignored when ENABLE is clear.  
 0 = If the MPU is enabled, disables use of the default memory map. Any memory access to a location not  
 covered by any enabled region causes a fault.  
 1 = If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses.  
 When enabled, the background region acts as if it is region number -1. Any region that is defined and enabled has priority over this default map."]
    #[inline(always)]
    pub fn privdefena(&mut self) -> PRIVDEFENA_W {
        PRIVDEFENA_W { w: self }
    }
    #[doc = "Bit 1 - Controls the use of the MPU for HardFaults and NMIs. Setting this bit when ENABLE is clear results in UNPREDICTABLE behaviour.  
 When the MPU is enabled:  
 0 = MPU is disabled during HardFault and NMI handlers, regardless of the value of the ENABLE bit.  
 1 = the MPU is enabled during HardFault and NMI handlers."]
    #[inline(always)]
    pub fn hfnmiena(&mut self) -> HFNMIENA_W {
        HFNMIENA_W { w: self }
    }
    #[doc = "Bit 0 - Enables the MPU. If the MPU is disabled, privileged and unprivileged accesses use the default memory map.  
 0 = MPU disabled.  
 1 = MPU enabled."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Use the MPU Control Register to enable and disable the MPU, and to control whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults and NMIs.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [mpu_ctrl](index.html) module"]
pub struct MPU_CTRL_SPEC;
impl crate::RegisterSpec for MPU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mpu_ctrl::R](R) reader structure"]
impl crate::Readable for MPU_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mpu_ctrl::W](W) writer structure"]
impl crate::Writable for MPU_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MPU_CTRL to value 0"]
impl crate::Resettable for MPU_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
