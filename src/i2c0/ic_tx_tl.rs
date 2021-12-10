#[doc = "Register `IC_TX_TL` reader"]
pub struct R(crate::R<IC_TX_TL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_TX_TL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_TX_TL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_TX_TL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC_TX_TL` writer"]
pub struct W(crate::W<IC_TX_TL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_TX_TL_SPEC>;
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
impl From<crate::W<IC_TX_TL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_TX_TL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_TL` reader - Transmit FIFO Threshold Level.  

 Controls the level of entries (or below) that trigger the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that it may not be set to value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 0 entries, and a value of 255 sets the threshold for 255 entries."]
pub struct TX_TL_R(crate::FieldReader<u8, u8>);
impl TX_TL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_TL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_TL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TX_TL` writer - Transmit FIFO Threshold Level.  

 Controls the level of entries (or below) that trigger the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that it may not be set to value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 0 entries, and a value of 255 sets the threshold for 255 entries."]
pub struct TX_TL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmit FIFO Threshold Level.  

 Controls the level of entries (or below) that trigger the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that it may not be set to value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 0 entries, and a value of 255 sets the threshold for 255 entries."]
    #[inline(always)]
    pub fn tx_tl(&self) -> TX_TL_R {
        TX_TL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit FIFO Threshold Level.  

 Controls the level of entries (or below) that trigger the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that it may not be set to value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 0 entries, and a value of 255 sets the threshold for 255 entries."]
    #[inline(always)]
    pub fn tx_tl(&mut self) -> TX_TL_W {
        TX_TL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Transmit FIFO Threshold Register  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_tx_tl](index.html) module"]
pub struct IC_TX_TL_SPEC;
impl crate::RegisterSpec for IC_TX_TL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_tx_tl::R](R) reader structure"]
impl crate::Readable for IC_TX_TL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic_tx_tl::W](W) writer structure"]
impl crate::Writable for IC_TX_TL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC_TX_TL to value 0"]
impl crate::Resettable for IC_TX_TL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
