#[doc = "Register `BUS_PRIORITY` reader"]
pub struct R(crate::R<BUS_PRIORITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_PRIORITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_PRIORITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_PRIORITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUS_PRIORITY` writer"]
pub struct W(crate::W<BUS_PRIORITY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUS_PRIORITY_SPEC>;
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
impl From<crate::W<BUS_PRIORITY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUS_PRIORITY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_W` reader - 0 - low priority, 1 - high priority"]
pub struct DMA_W_R(crate::FieldReader<bool, bool>);
impl DMA_W_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_W_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_W` writer - 0 - low priority, 1 - high priority"]
pub struct DMA_W_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `DMA_R` reader - 0 - low priority, 1 - high priority"]
pub struct DMA_R_R(crate::FieldReader<bool, bool>);
impl DMA_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMA_R` writer - 0 - low priority, 1 - high priority"]
pub struct DMA_R_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_R_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `PROC1` reader - 0 - low priority, 1 - high priority"]
pub struct PROC1_R(crate::FieldReader<bool, bool>);
impl PROC1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROC1` writer - 0 - low priority, 1 - high priority"]
pub struct PROC1_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `PROC0` reader - 0 - low priority, 1 - high priority"]
pub struct PROC0_R(crate::FieldReader<bool, bool>);
impl PROC0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PROC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROC0` writer - 0 - low priority, 1 - high priority"]
pub struct PROC0_W<'a> {
    w: &'a mut W,
}
impl<'a> PROC0_W<'a> {
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
    #[doc = "Bit 12 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn dma_w(&self) -> DMA_W_R {
        DMA_W_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn dma_r(&self) -> DMA_R_R {
        DMA_R_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn proc1(&self) -> PROC1_R {
        PROC1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn proc0(&self) -> PROC0_R {
        PROC0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn dma_w(&mut self) -> DMA_W_W {
        DMA_W_W { w: self }
    }
    #[doc = "Bit 8 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn dma_r(&mut self) -> DMA_R_W {
        DMA_R_W { w: self }
    }
    #[doc = "Bit 4 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn proc1(&mut self) -> PROC1_W {
        PROC1_W { w: self }
    }
    #[doc = "Bit 0 - 0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn proc0(&mut self) -> PROC0_W {
        PROC0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set the priority of each master for bus arbitration.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [bus_priority](index.html) module"]
pub struct BUS_PRIORITY_SPEC;
impl crate::RegisterSpec for BUS_PRIORITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_priority::R](R) reader structure"]
impl crate::Readable for BUS_PRIORITY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bus_priority::W](W) writer structure"]
impl crate::Writable for BUS_PRIORITY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUS_PRIORITY to value 0"]
impl crate::Resettable for BUS_PRIORITY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
