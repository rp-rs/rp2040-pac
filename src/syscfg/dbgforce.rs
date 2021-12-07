#[doc = "Register `DBGFORCE` reader"]
pub struct R(crate::R<DBGFORCE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGFORCE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGFORCE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGFORCE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBGFORCE` writer"]
pub struct W(crate::W<DBGFORCE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBGFORCE_SPEC>;
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
impl From<crate::W<DBGFORCE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBGFORCE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROC1_ATTACH` reader - Attach processor 1 debug port to syscfg controls, and disconnect it from external SWD pads."]
pub struct PROC1_ATTACH_R(crate::FieldReader<bool, bool>);
impl PROC1_ATTACH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROC1_ATTACH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC1_ATTACH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROC1_ATTACH` writer - Attach processor 1 debug port to syscfg controls, and disconnect it from external SWD pads."]
pub struct PROC1_ATTACH_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC1_ATTACH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `PROC1_SWCLK` reader - Directly drive processor 1 SWCLK, if PROC1_ATTACH is set"]
pub struct PROC1_SWCLK_R(crate::FieldReader<bool, bool>);
impl PROC1_SWCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROC1_SWCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC1_SWCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROC1_SWCLK` writer - Directly drive processor 1 SWCLK, if PROC1_ATTACH is set"]
pub struct PROC1_SWCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC1_SWCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PROC1_SWDI` reader - Directly drive processor 1 SWDIO input, if PROC1_ATTACH is set"]
pub struct PROC1_SWDI_R(crate::FieldReader<bool, bool>);
impl PROC1_SWDI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROC1_SWDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC1_SWDI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROC1_SWDI` writer - Directly drive processor 1 SWDIO input, if PROC1_ATTACH is set"]
pub struct PROC1_SWDI_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC1_SWDI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `PROC1_SWDO` reader - Observe the value of processor 1 SWDIO output."]
pub struct PROC1_SWDO_R(crate::FieldReader<bool, bool>);
impl PROC1_SWDO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROC1_SWDO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC1_SWDO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROC0_ATTACH` reader - Attach processor 0 debug port to syscfg controls, and disconnect it from external SWD pads."]
pub struct PROC0_ATTACH_R(crate::FieldReader<bool, bool>);
impl PROC0_ATTACH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROC0_ATTACH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC0_ATTACH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROC0_ATTACH` writer - Attach processor 0 debug port to syscfg controls, and disconnect it from external SWD pads."]
pub struct PROC0_ATTACH_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC0_ATTACH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `PROC0_SWCLK` reader - Directly drive processor 0 SWCLK, if PROC0_ATTACH is set"]
pub struct PROC0_SWCLK_R(crate::FieldReader<bool, bool>);
impl PROC0_SWCLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROC0_SWCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC0_SWCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROC0_SWCLK` writer - Directly drive processor 0 SWCLK, if PROC0_ATTACH is set"]
pub struct PROC0_SWCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC0_SWCLK_W<'a> {
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
#[doc = "Field `PROC0_SWDI` reader - Directly drive processor 0 SWDIO input, if PROC0_ATTACH is set"]
pub struct PROC0_SWDI_R(crate::FieldReader<bool, bool>);
impl PROC0_SWDI_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROC0_SWDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC0_SWDI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROC0_SWDI` writer - Directly drive processor 0 SWDIO input, if PROC0_ATTACH is set"]
pub struct PROC0_SWDI_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC0_SWDI_W<'a> {
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
#[doc = "Field `PROC0_SWDO` reader - Observe the value of processor 0 SWDIO output."]
pub struct PROC0_SWDO_R(crate::FieldReader<bool, bool>);
impl PROC0_SWDO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROC0_SWDO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC0_SWDO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 7 - Attach processor 1 debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    pub fn proc1_attach(&self) -> PROC1_ATTACH_R {
        PROC1_ATTACH_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Directly drive processor 1 SWCLK, if PROC1_ATTACH is set"]
    #[inline(always)]
    pub fn proc1_swclk(&self) -> PROC1_SWCLK_R {
        PROC1_SWCLK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Directly drive processor 1 SWDIO input, if PROC1_ATTACH is set"]
    #[inline(always)]
    pub fn proc1_swdi(&self) -> PROC1_SWDI_R {
        PROC1_SWDI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Observe the value of processor 1 SWDIO output."]
    #[inline(always)]
    pub fn proc1_swdo(&self) -> PROC1_SWDO_R {
        PROC1_SWDO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Attach processor 0 debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    pub fn proc0_attach(&self) -> PROC0_ATTACH_R {
        PROC0_ATTACH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Directly drive processor 0 SWCLK, if PROC0_ATTACH is set"]
    #[inline(always)]
    pub fn proc0_swclk(&self) -> PROC0_SWCLK_R {
        PROC0_SWCLK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Directly drive processor 0 SWDIO input, if PROC0_ATTACH is set"]
    #[inline(always)]
    pub fn proc0_swdi(&self) -> PROC0_SWDI_R {
        PROC0_SWDI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Observe the value of processor 0 SWDIO output."]
    #[inline(always)]
    pub fn proc0_swdo(&self) -> PROC0_SWDO_R {
        PROC0_SWDO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Attach processor 1 debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    pub fn proc1_attach(&mut self) -> PROC1_ATTACH_W {
        PROC1_ATTACH_W { w: self }
    }
    #[doc = "Bit 6 - Directly drive processor 1 SWCLK, if PROC1_ATTACH is set"]
    #[inline(always)]
    pub fn proc1_swclk(&mut self) -> PROC1_SWCLK_W {
        PROC1_SWCLK_W { w: self }
    }
    #[doc = "Bit 5 - Directly drive processor 1 SWDIO input, if PROC1_ATTACH is set"]
    #[inline(always)]
    pub fn proc1_swdi(&mut self) -> PROC1_SWDI_W {
        PROC1_SWDI_W { w: self }
    }
    #[doc = "Bit 3 - Attach processor 0 debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    pub fn proc0_attach(&mut self) -> PROC0_ATTACH_W {
        PROC0_ATTACH_W { w: self }
    }
    #[doc = "Bit 2 - Directly drive processor 0 SWCLK, if PROC0_ATTACH is set"]
    #[inline(always)]
    pub fn proc0_swclk(&mut self) -> PROC0_SWCLK_W {
        PROC0_SWCLK_W { w: self }
    }
    #[doc = "Bit 1 - Directly drive processor 0 SWDIO input, if PROC0_ATTACH is set"]
    #[inline(always)]
    pub fn proc0_swdi(&mut self) -> PROC0_SWDI_W {
        PROC0_SWDI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Directly control the SWD debug port of either processor  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [dbgforce](index.html) module"]
pub struct DBGFORCE_SPEC;
impl crate::RegisterSpec for DBGFORCE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbgforce::R](R) reader structure"]
impl crate::Readable for DBGFORCE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbgforce::W](W) writer structure"]
impl crate::Writable for DBGFORCE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBGFORCE to value 0x66"]
impl crate::Resettable for DBGFORCE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x66
    }
}
