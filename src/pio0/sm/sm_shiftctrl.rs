#[doc = "Register `SM_SHIFTCTRL` reader"]
pub struct R(crate::R<SM_SHIFTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SM_SHIFTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SM_SHIFTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SM_SHIFTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SM_SHIFTCTRL` writer"]
pub struct W(crate::W<SM_SHIFTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SM_SHIFTCTRL_SPEC>;
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
impl From<crate::W<SM_SHIFTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SM_SHIFTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FJOIN_RX` reader - When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.  
 TX FIFO is disabled as a result (always reads as both full and empty).  
 FIFOs are flushed when this bit is changed."]
pub struct FJOIN_RX_R(crate::FieldReader<bool, bool>);
impl FJOIN_RX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FJOIN_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FJOIN_RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FJOIN_RX` writer - When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.  
 TX FIFO is disabled as a result (always reads as both full and empty).  
 FIFOs are flushed when this bit is changed."]
pub struct FJOIN_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> FJOIN_RX_W<'a> {
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
#[doc = "Field `FJOIN_TX` reader - When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.  
 RX FIFO is disabled as a result (always reads as both full and empty).  
 FIFOs are flushed when this bit is changed."]
pub struct FJOIN_TX_R(crate::FieldReader<bool, bool>);
impl FJOIN_TX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FJOIN_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FJOIN_TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FJOIN_TX` writer - When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.  
 RX FIFO is disabled as a result (always reads as both full and empty).  
 FIFOs are flushed when this bit is changed."]
pub struct FJOIN_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> FJOIN_TX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `PULL_THRESH` reader - Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.  
 Write 0 for value of 32."]
pub struct PULL_THRESH_R(crate::FieldReader<u8, u8>);
impl PULL_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PULL_THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULL_THRESH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULL_THRESH` writer - Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.  
 Write 0 for value of 32."]
pub struct PULL_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> PULL_THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | ((value as u32 & 0x1f) << 25);
        self.w
    }
}
#[doc = "Field `PUSH_THRESH` reader - Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.  
 Write 0 for value of 32."]
pub struct PUSH_THRESH_R(crate::FieldReader<u8, u8>);
impl PUSH_THRESH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PUSH_THRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PUSH_THRESH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PUSH_THRESH` writer - Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.  
 Write 0 for value of 32."]
pub struct PUSH_THRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> PUSH_THRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
#[doc = "Field `OUT_SHIFTDIR` reader - 1 = shift out of output shift register to right. 0 = to left."]
pub struct OUT_SHIFTDIR_R(crate::FieldReader<bool, bool>);
impl OUT_SHIFTDIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_SHIFTDIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_SHIFTDIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_SHIFTDIR` writer - 1 = shift out of output shift register to right. 0 = to left."]
pub struct OUT_SHIFTDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_SHIFTDIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `IN_SHIFTDIR` reader - 1 = shift input shift register to right (data enters from left). 0 = to left."]
pub struct IN_SHIFTDIR_R(crate::FieldReader<bool, bool>);
impl IN_SHIFTDIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IN_SHIFTDIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN_SHIFTDIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN_SHIFTDIR` writer - 1 = shift input shift register to right (data enters from left). 0 = to left."]
pub struct IN_SHIFTDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> IN_SHIFTDIR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `AUTOPULL` reader - Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH."]
pub struct AUTOPULL_R(crate::FieldReader<bool, bool>);
impl AUTOPULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTOPULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTOPULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOPULL` writer - Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH."]
pub struct AUTOPULL_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOPULL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `AUTOPUSH` reader - Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH."]
pub struct AUTOPUSH_R(crate::FieldReader<bool, bool>);
impl AUTOPUSH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTOPUSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTOPUSH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUTOPUSH` writer - Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH."]
pub struct AUTOPUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTOPUSH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.  
 TX FIFO is disabled as a result (always reads as both full and empty).  
 FIFOs are flushed when this bit is changed."]
    #[inline(always)]
    pub fn fjoin_rx(&self) -> FJOIN_RX_R {
        FJOIN_RX_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.  
 RX FIFO is disabled as a result (always reads as both full and empty).  
 FIFOs are flushed when this bit is changed."]
    #[inline(always)]
    pub fn fjoin_tx(&self) -> FJOIN_TX_R {
        FJOIN_TX_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 25:29 - Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.  
 Write 0 for value of 32."]
    #[inline(always)]
    pub fn pull_thresh(&self) -> PULL_THRESH_R {
        PULL_THRESH_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.  
 Write 0 for value of 32."]
    #[inline(always)]
    pub fn push_thresh(&self) -> PUSH_THRESH_R {
        PUSH_THRESH_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 19 - 1 = shift out of output shift register to right. 0 = to left."]
    #[inline(always)]
    pub fn out_shiftdir(&self) -> OUT_SHIFTDIR_R {
        OUT_SHIFTDIR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 1 = shift input shift register to right (data enters from left). 0 = to left."]
    #[inline(always)]
    pub fn in_shiftdir(&self) -> IN_SHIFTDIR_R {
        IN_SHIFTDIR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH."]
    #[inline(always)]
    pub fn autopull(&self) -> AUTOPULL_R {
        AUTOPULL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH."]
    #[inline(always)]
    pub fn autopush(&self) -> AUTOPUSH_R {
        AUTOPUSH_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep.  
 TX FIFO is disabled as a result (always reads as both full and empty).  
 FIFOs are flushed when this bit is changed."]
    #[inline(always)]
    pub fn fjoin_rx(&mut self) -> FJOIN_RX_W {
        FJOIN_RX_W { w: self }
    }
    #[doc = "Bit 30 - When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep.  
 RX FIFO is disabled as a result (always reads as both full and empty).  
 FIFOs are flushed when this bit is changed."]
    #[inline(always)]
    pub fn fjoin_tx(&mut self) -> FJOIN_TX_W {
        FJOIN_TX_W { w: self }
    }
    #[doc = "Bits 25:29 - Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place.  
 Write 0 for value of 32."]
    #[inline(always)]
    pub fn pull_thresh(&mut self) -> PULL_THRESH_W {
        PULL_THRESH_W { w: self }
    }
    #[doc = "Bits 20:24 - Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place.  
 Write 0 for value of 32."]
    #[inline(always)]
    pub fn push_thresh(&mut self) -> PUSH_THRESH_W {
        PUSH_THRESH_W { w: self }
    }
    #[doc = "Bit 19 - 1 = shift out of output shift register to right. 0 = to left."]
    #[inline(always)]
    pub fn out_shiftdir(&mut self) -> OUT_SHIFTDIR_W {
        OUT_SHIFTDIR_W { w: self }
    }
    #[doc = "Bit 18 - 1 = shift input shift register to right (data enters from left). 0 = to left."]
    #[inline(always)]
    pub fn in_shiftdir(&mut self) -> IN_SHIFTDIR_W {
        IN_SHIFTDIR_W { w: self }
    }
    #[doc = "Bit 17 - Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH."]
    #[inline(always)]
    pub fn autopull(&mut self) -> AUTOPULL_W {
        AUTOPULL_W { w: self }
    }
    #[doc = "Bit 16 - Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH."]
    #[inline(always)]
    pub fn autopush(&mut self) -> AUTOPUSH_W {
        AUTOPUSH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control behaviour of the input/output shift registers for state machine 0  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sm_shiftctrl](index.html) module"]
pub struct SM_SHIFTCTRL_SPEC;
impl crate::RegisterSpec for SM_SHIFTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sm_shiftctrl::R](R) reader structure"]
impl crate::Readable for SM_SHIFTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sm_shiftctrl::W](W) writer structure"]
impl crate::Writable for SM_SHIFTCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SM_SHIFTCTRL to value 0x000c_0000"]
impl crate::Resettable for SM_SHIFTCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000c_0000
    }
}
