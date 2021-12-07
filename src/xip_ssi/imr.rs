#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMR` writer"]
pub struct W(crate::W<IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMR_SPEC>;
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
impl From<crate::W<IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSTIM` reader - Multi-master contention interrupt mask"]
pub struct MSTIM_R(crate::FieldReader<bool, bool>);
impl MSTIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MSTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTIM` writer - Multi-master contention interrupt mask"]
pub struct MSTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTIM_W<'a> {
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
#[doc = "Field `RXFIM` reader - Receive FIFO full interrupt mask"]
pub struct RXFIM_R(crate::FieldReader<bool, bool>);
impl RXFIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXFIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIM` writer - Receive FIFO full interrupt mask"]
pub struct RXFIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIM_W<'a> {
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
#[doc = "Field `RXOIM` reader - Receive FIFO overflow interrupt mask"]
pub struct RXOIM_R(crate::FieldReader<bool, bool>);
impl RXOIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXOIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOIM` writer - Receive FIFO overflow interrupt mask"]
pub struct RXOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOIM_W<'a> {
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
#[doc = "Field `RXUIM` reader - Receive FIFO underflow interrupt mask"]
pub struct RXUIM_R(crate::FieldReader<bool, bool>);
impl RXUIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXUIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXUIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXUIM` writer - Receive FIFO underflow interrupt mask"]
pub struct RXUIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUIM_W<'a> {
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
#[doc = "Field `TXOIM` reader - Transmit FIFO overflow interrupt mask"]
pub struct TXOIM_R(crate::FieldReader<bool, bool>);
impl TXOIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXOIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOIM` writer - Transmit FIFO overflow interrupt mask"]
pub struct TXOIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOIM_W<'a> {
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
#[doc = "Field `TXEIM` reader - Transmit FIFO empty interrupt mask"]
pub struct TXEIM_R(crate::FieldReader<bool, bool>);
impl TXEIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXEIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEIM` writer - Transmit FIFO empty interrupt mask"]
pub struct TXEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEIM_W<'a> {
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
    #[doc = "Bit 5 - Multi-master contention interrupt mask"]
    #[inline(always)]
    pub fn mstim(&self) -> MSTIM_R {
        MSTIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO full interrupt mask"]
    #[inline(always)]
    pub fn rxfim(&self) -> RXFIM_R {
        RXFIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO overflow interrupt mask"]
    #[inline(always)]
    pub fn rxoim(&self) -> RXOIM_R {
        RXOIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO underflow interrupt mask"]
    #[inline(always)]
    pub fn rxuim(&self) -> RXUIM_R {
        RXUIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO overflow interrupt mask"]
    #[inline(always)]
    pub fn txoim(&self) -> TXOIM_R {
        TXOIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Transmit FIFO empty interrupt mask"]
    #[inline(always)]
    pub fn txeim(&self) -> TXEIM_R {
        TXEIM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Multi-master contention interrupt mask"]
    #[inline(always)]
    pub fn mstim(&mut self) -> MSTIM_W {
        MSTIM_W { w: self }
    }
    #[doc = "Bit 4 - Receive FIFO full interrupt mask"]
    #[inline(always)]
    pub fn rxfim(&mut self) -> RXFIM_W {
        RXFIM_W { w: self }
    }
    #[doc = "Bit 3 - Receive FIFO overflow interrupt mask"]
    #[inline(always)]
    pub fn rxoim(&mut self) -> RXOIM_W {
        RXOIM_W { w: self }
    }
    #[doc = "Bit 2 - Receive FIFO underflow interrupt mask"]
    #[inline(always)]
    pub fn rxuim(&mut self) -> RXUIM_W {
        RXUIM_W { w: self }
    }
    #[doc = "Bit 1 - Transmit FIFO overflow interrupt mask"]
    #[inline(always)]
    pub fn txoim(&mut self) -> TXOIM_W {
        TXOIM_W { w: self }
    }
    #[doc = "Bit 0 - Transmit FIFO empty interrupt mask"]
    #[inline(always)]
    pub fn txeim(&mut self) -> TXEIM_W {
        TXEIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt mask  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imr::W](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
