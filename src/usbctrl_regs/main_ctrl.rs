#[doc = "Register `MAIN_CTRL` reader"]
pub struct R(crate::R<MAIN_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAIN_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAIN_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAIN_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAIN_CTRL` writer"]
pub struct W(crate::W<MAIN_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAIN_CTRL_SPEC>;
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
impl From<crate::W<MAIN_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAIN_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIM_TIMING` reader - Reduced timings for simulation"]
pub struct SIM_TIMING_R(crate::FieldReader<bool, bool>);
impl SIM_TIMING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIM_TIMING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIM_TIMING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIM_TIMING` writer - Reduced timings for simulation"]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `HOST_NDEVICE` reader - Device mode = 0, Host mode = 1"]
pub struct HOST_NDEVICE_R(crate::FieldReader<bool, bool>);
impl HOST_NDEVICE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_NDEVICE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOST_NDEVICE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_NDEVICE` writer - Device mode = 0, Host mode = 1"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CONTROLLER_EN` reader - Enable controller"]
pub struct CONTROLLER_EN_R(crate::FieldReader<bool, bool>);
impl CONTROLLER_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONTROLLER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONTROLLER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONTROLLER_EN` writer - Enable controller"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main control register  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [main_ctrl](index.html) module"]
pub struct MAIN_CTRL_SPEC;
impl crate::RegisterSpec for MAIN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [main_ctrl::R](R) reader structure"]
impl crate::Readable for MAIN_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [main_ctrl::W](W) writer structure"]
impl crate::Writable for MAIN_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAIN_CTRL to value 0"]
impl crate::Resettable for MAIN_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
