#[doc = "Register `IC_DMA_CR` reader"]
pub struct R(crate::R<IC_DMA_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_DMA_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_DMA_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_DMA_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC_DMA_CR` writer"]
pub struct W(crate::W<IC_DMA_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_DMA_CR_SPEC>;
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
impl From<crate::W<IC_DMA_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_DMA_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDMAE_A {
    #[doc = "0: transmit FIFO DMA channel disabled"]
    DISABLED = 0,
    #[doc = "1: Transmit FIFO DMA channel enabled"]
    ENABLED = 1,
}
impl From<TDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: TDMAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDMAE` reader - Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. Reset value: 0x0"]
pub struct TDMAE_R(crate::FieldReader<bool, TDMAE_A>);
impl TDMAE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TDMAE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDMAE_A {
        match self.bits {
            false => TDMAE_A::DISABLED,
            true => TDMAE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TDMAE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TDMAE_A::ENABLED
    }
}
impl core::ops::Deref for TDMAE_R {
    type Target = crate::FieldReader<bool, TDMAE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TDMAE` writer - Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. Reset value: 0x0"]
pub struct TDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> TDMAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TDMAE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "transmit FIFO DMA channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TDMAE_A::DISABLED)
    }
    #[doc = "Transmit FIFO DMA channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TDMAE_A::ENABLED)
    }
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
#[doc = "Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDMAE_A {
    #[doc = "0: Receive FIFO DMA channel disabled"]
    DISABLED = 0,
    #[doc = "1: Receive FIFO DMA channel enabled"]
    ENABLED = 1,
}
impl From<RDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: RDMAE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDMAE` reader - Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. Reset value: 0x0"]
pub struct RDMAE_R(crate::FieldReader<bool, RDMAE_A>);
impl RDMAE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RDMAE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDMAE_A {
        match self.bits {
            false => RDMAE_A::DISABLED,
            true => RDMAE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RDMAE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RDMAE_A::ENABLED
    }
}
impl core::ops::Deref for RDMAE_R {
    type Target = crate::FieldReader<bool, RDMAE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDMAE` writer - Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. Reset value: 0x0"]
pub struct RDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> RDMAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDMAE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive FIFO DMA channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RDMAE_A::DISABLED)
    }
    #[doc = "Receive FIFO DMA channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RDMAE_A::ENABLED)
    }
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
    #[doc = "Bit 1 - Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. Reset value: 0x0"]
    #[inline(always)]
    pub fn tdmae(&self) -> TDMAE_R {
        TDMAE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. Reset value: 0x0"]
    #[inline(always)]
    pub fn rdmae(&self) -> RDMAE_R {
        RDMAE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. Reset value: 0x0"]
    #[inline(always)]
    pub fn tdmae(&mut self) -> TDMAE_W {
        TDMAE_W { w: self }
    }
    #[doc = "Bit 0 - Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. Reset value: 0x0"]
    #[inline(always)]
    pub fn rdmae(&mut self) -> RDMAE_W {
        RDMAE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Control Register  

 The register is used to enable the DMA Controller interface operation. There is a separate bit for transmit and receive. This can be programmed regardless of the state of IC_ENABLE.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_dma_cr](index.html) module"]
pub struct IC_DMA_CR_SPEC;
impl crate::RegisterSpec for IC_DMA_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_dma_cr::R](R) reader structure"]
impl crate::Readable for IC_DMA_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic_dma_cr::W](W) writer structure"]
impl crate::Writable for IC_DMA_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC_DMA_CR to value 0"]
impl crate::Resettable for IC_DMA_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
