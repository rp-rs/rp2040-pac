#[doc = "Register `IRQ_INTF` reader"]
pub struct R(crate::R<IRQ_INTF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQ_INTF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQ_INTF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQ_INTF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQ_INTF` writer"]
pub struct W(crate::W<IRQ_INTF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQ_INTF_SPEC>;
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
impl From<crate::W<IRQ_INTF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQ_INTF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SM3` reader - "]
pub struct SM3_R(crate::FieldReader<bool, bool>);
impl SM3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM3` writer - "]
pub struct SM3_W<'a> {
    w: &'a mut W,
}
impl<'a> SM3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `SM2` reader - "]
pub struct SM2_R(crate::FieldReader<bool, bool>);
impl SM2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM2` writer - "]
pub struct SM2_W<'a> {
    w: &'a mut W,
}
impl<'a> SM2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SM1` reader - "]
pub struct SM1_R(crate::FieldReader<bool, bool>);
impl SM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM1` writer - "]
pub struct SM1_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `SM0` reader - "]
pub struct SM0_R(crate::FieldReader<bool, bool>);
impl SM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM0` writer - "]
pub struct SM0_W<'a> {
    w: &'a mut W,
}
impl<'a> SM0_W<'a> {
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
#[doc = "Field `SM3_TXNFULL` reader - "]
pub struct SM3_TXNFULL_R(crate::FieldReader<bool, bool>);
impl SM3_TXNFULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM3_TXNFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM3_TXNFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM3_TXNFULL` writer - "]
pub struct SM3_TXNFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> SM3_TXNFULL_W<'a> {
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
#[doc = "Field `SM2_TXNFULL` reader - "]
pub struct SM2_TXNFULL_R(crate::FieldReader<bool, bool>);
impl SM2_TXNFULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM2_TXNFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM2_TXNFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM2_TXNFULL` writer - "]
pub struct SM2_TXNFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> SM2_TXNFULL_W<'a> {
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
#[doc = "Field `SM1_TXNFULL` reader - "]
pub struct SM1_TXNFULL_R(crate::FieldReader<bool, bool>);
impl SM1_TXNFULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM1_TXNFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM1_TXNFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM1_TXNFULL` writer - "]
pub struct SM1_TXNFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1_TXNFULL_W<'a> {
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
#[doc = "Field `SM0_TXNFULL` reader - "]
pub struct SM0_TXNFULL_R(crate::FieldReader<bool, bool>);
impl SM0_TXNFULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM0_TXNFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM0_TXNFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM0_TXNFULL` writer - "]
pub struct SM0_TXNFULL_W<'a> {
    w: &'a mut W,
}
impl<'a> SM0_TXNFULL_W<'a> {
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
#[doc = "Field `SM3_RXNEMPTY` reader - "]
pub struct SM3_RXNEMPTY_R(crate::FieldReader<bool, bool>);
impl SM3_RXNEMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM3_RXNEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM3_RXNEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM3_RXNEMPTY` writer - "]
pub struct SM3_RXNEMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> SM3_RXNEMPTY_W<'a> {
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
#[doc = "Field `SM2_RXNEMPTY` reader - "]
pub struct SM2_RXNEMPTY_R(crate::FieldReader<bool, bool>);
impl SM2_RXNEMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM2_RXNEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM2_RXNEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM2_RXNEMPTY` writer - "]
pub struct SM2_RXNEMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> SM2_RXNEMPTY_W<'a> {
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
#[doc = "Field `SM1_RXNEMPTY` reader - "]
pub struct SM1_RXNEMPTY_R(crate::FieldReader<bool, bool>);
impl SM1_RXNEMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM1_RXNEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM1_RXNEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM1_RXNEMPTY` writer - "]
pub struct SM1_RXNEMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> SM1_RXNEMPTY_W<'a> {
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
#[doc = "Field `SM0_RXNEMPTY` reader - "]
pub struct SM0_RXNEMPTY_R(crate::FieldReader<bool, bool>);
impl SM0_RXNEMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SM0_RXNEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SM0_RXNEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SM0_RXNEMPTY` writer - "]
pub struct SM0_RXNEMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> SM0_RXNEMPTY_W<'a> {
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
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sm3(&self) -> SM3_R {
        SM3_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sm2(&self) -> SM2_R {
        SM2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sm1(&self) -> SM1_R {
        SM1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sm0(&self) -> SM0_R {
        SM0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sm3_txnfull(&self) -> SM3_TXNFULL_R {
        SM3_TXNFULL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sm2_txnfull(&self) -> SM2_TXNFULL_R {
        SM2_TXNFULL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sm1_txnfull(&self) -> SM1_TXNFULL_R {
        SM1_TXNFULL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sm0_txnfull(&self) -> SM0_TXNFULL_R {
        SM0_TXNFULL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sm3_rxnempty(&self) -> SM3_RXNEMPTY_R {
        SM3_RXNEMPTY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sm2_rxnempty(&self) -> SM2_RXNEMPTY_R {
        SM2_RXNEMPTY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sm1_rxnempty(&self) -> SM1_RXNEMPTY_R {
        SM1_RXNEMPTY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sm0_rxnempty(&self) -> SM0_RXNEMPTY_R {
        SM0_RXNEMPTY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sm3(&mut self) -> SM3_W {
        SM3_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sm2(&mut self) -> SM2_W {
        SM2_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sm1(&mut self) -> SM1_W {
        SM1_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sm0(&mut self) -> SM0_W {
        SM0_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sm3_txnfull(&mut self) -> SM3_TXNFULL_W {
        SM3_TXNFULL_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sm2_txnfull(&mut self) -> SM2_TXNFULL_W {
        SM2_TXNFULL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sm1_txnfull(&mut self) -> SM1_TXNFULL_W {
        SM1_TXNFULL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sm0_txnfull(&mut self) -> SM0_TXNFULL_W {
        SM0_TXNFULL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sm3_rxnempty(&mut self) -> SM3_RXNEMPTY_W {
        SM3_RXNEMPTY_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sm2_rxnempty(&mut self) -> SM2_RXNEMPTY_W {
        SM2_RXNEMPTY_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sm1_rxnempty(&mut self) -> SM1_RXNEMPTY_W {
        SM1_RXNEMPTY_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sm0_rxnempty(&mut self) -> SM0_RXNEMPTY_W {
        SM0_RXNEMPTY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Force for irq0  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [irq_intf](index.html) module"]
pub struct IRQ_INTF_SPEC;
impl crate::RegisterSpec for IRQ_INTF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irq_intf::R](R) reader structure"]
impl crate::Readable for IRQ_INTF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irq_intf::W](W) writer structure"]
impl crate::Writable for IRQ_INTF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRQ_INTF to value 0"]
impl crate::Resettable for IRQ_INTF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
