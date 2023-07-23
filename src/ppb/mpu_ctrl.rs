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
#[doc = "Field `ENABLE` reader - Enables the MPU. If the MPU is disabled, privileged and unprivileged accesses use the default memory map.  
 0 = MPU disabled.  
 1 = MPU enabled."]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - Enables the MPU. If the MPU is disabled, privileged and unprivileged accesses use the default memory map.  
 0 = MPU disabled.  
 1 = MPU enabled."]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_CTRL_SPEC, bool, O>;
#[doc = "Field `HFNMIENA` reader - Controls the use of the MPU for HardFaults and NMIs. Setting this bit when ENABLE is clear results in UNPREDICTABLE behaviour.  
 When the MPU is enabled:  
 0 = MPU is disabled during HardFault and NMI handlers, regardless of the value of the ENABLE bit.  
 1 = the MPU is enabled during HardFault and NMI handlers."]
pub type HFNMIENA_R = crate::BitReader<bool>;
#[doc = "Field `HFNMIENA` writer - Controls the use of the MPU for HardFaults and NMIs. Setting this bit when ENABLE is clear results in UNPREDICTABLE behaviour.  
 When the MPU is enabled:  
 0 = MPU is disabled during HardFault and NMI handlers, regardless of the value of the ENABLE bit.  
 1 = the MPU is enabled during HardFault and NMI handlers."]
pub type HFNMIENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_CTRL_SPEC, bool, O>;
#[doc = "Field `PRIVDEFENA` reader - Controls whether the default memory map is enabled as a background region for privileged accesses. This bit is ignored when ENABLE is clear.  
 0 = If the MPU is enabled, disables use of the default memory map. Any memory access to a location not  
 covered by any enabled region causes a fault.  
 1 = If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses.  
 When enabled, the background region acts as if it is region number -1. Any region that is defined and enabled has priority over this default map."]
pub type PRIVDEFENA_R = crate::BitReader<bool>;
#[doc = "Field `PRIVDEFENA` writer - Controls whether the default memory map is enabled as a background region for privileged accesses. This bit is ignored when ENABLE is clear.  
 0 = If the MPU is enabled, disables use of the default memory map. Any memory access to a location not  
 covered by any enabled region causes a fault.  
 1 = If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses.  
 When enabled, the background region acts as if it is region number -1. Any region that is defined and enabled has priority over this default map."]
pub type PRIVDEFENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MPU_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enables the MPU. If the MPU is disabled, privileged and unprivileged accesses use the default memory map.  
 0 = MPU disabled.  
 1 = MPU enabled."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controls the use of the MPU for HardFaults and NMIs. Setting this bit when ENABLE is clear results in UNPREDICTABLE behaviour.  
 When the MPU is enabled:  
 0 = MPU is disabled during HardFault and NMI handlers, regardless of the value of the ENABLE bit.  
 1 = the MPU is enabled during HardFault and NMI handlers."]
    #[inline(always)]
    pub fn hfnmiena(&self) -> HFNMIENA_R {
        HFNMIENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Controls whether the default memory map is enabled as a background region for privileged accesses. This bit is ignored when ENABLE is clear.  
 0 = If the MPU is enabled, disables use of the default memory map. Any memory access to a location not  
 covered by any enabled region causes a fault.  
 1 = If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses.  
 When enabled, the background region acts as if it is region number -1. Any region that is defined and enabled has priority over this default map."]
    #[inline(always)]
    pub fn privdefena(&self) -> PRIVDEFENA_R {
        PRIVDEFENA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the MPU. If the MPU is disabled, privileged and unprivileged accesses use the default memory map.  
 0 = MPU disabled.  
 1 = MPU enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - Controls the use of the MPU for HardFaults and NMIs. Setting this bit when ENABLE is clear results in UNPREDICTABLE behaviour.  
 When the MPU is enabled:  
 0 = MPU is disabled during HardFault and NMI handlers, regardless of the value of the ENABLE bit.  
 1 = the MPU is enabled during HardFault and NMI handlers."]
    #[inline(always)]
    #[must_use]
    pub fn hfnmiena(&mut self) -> HFNMIENA_W<1> {
        HFNMIENA_W::new(self)
    }
    #[doc = "Bit 2 - Controls whether the default memory map is enabled as a background region for privileged accesses. This bit is ignored when ENABLE is clear.  
 0 = If the MPU is enabled, disables use of the default memory map. Any memory access to a location not  
 covered by any enabled region causes a fault.  
 1 = If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses.  
 When enabled, the background region acts as if it is region number -1. Any region that is defined and enabled has priority over this default map."]
    #[inline(always)]
    #[must_use]
    pub fn privdefena(&mut self) -> PRIVDEFENA_W<2> {
        PRIVDEFENA_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MPU_CTRL to value 0"]
impl crate::Resettable for MPU_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
