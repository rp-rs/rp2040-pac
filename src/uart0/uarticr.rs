#[doc = "Register `UARTICR` reader"]
pub struct R(crate::R<UARTICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UARTICR` writer"]
pub struct W(crate::W<UARTICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UARTICR_SPEC>;
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
impl From<crate::W<UARTICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UARTICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OEIC` reader - Overrun error interrupt clear. Clears the UARTOEINTR interrupt."]
pub struct OEIC_R(crate::FieldReader<bool, bool>);
impl OEIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OEIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEIC` writer - Overrun error interrupt clear. Clears the UARTOEINTR interrupt."]
pub struct OEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> OEIC_W<'a> {
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
#[doc = "Field `BEIC` reader - Break error interrupt clear. Clears the UARTBEINTR interrupt."]
pub struct BEIC_R(crate::FieldReader<bool, bool>);
impl BEIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BEIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEIC` writer - Break error interrupt clear. Clears the UARTBEINTR interrupt."]
pub struct BEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIC_W<'a> {
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
#[doc = "Field `PEIC` reader - Parity error interrupt clear. Clears the UARTPEINTR interrupt."]
pub struct PEIC_R(crate::FieldReader<bool, bool>);
impl PEIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEIC` writer - Parity error interrupt clear. Clears the UARTPEINTR interrupt."]
pub struct PEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIC_W<'a> {
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
#[doc = "Field `FEIC` reader - Framing error interrupt clear. Clears the UARTFEINTR interrupt."]
pub struct FEIC_R(crate::FieldReader<bool, bool>);
impl FEIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIC` writer - Framing error interrupt clear. Clears the UARTFEINTR interrupt."]
pub struct FEIC_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIC_W<'a> {
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
#[doc = "Field `RTIC` reader - Receive timeout interrupt clear. Clears the UARTRTINTR interrupt."]
pub struct RTIC_R(crate::FieldReader<bool, bool>);
impl RTIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTIC` writer - Receive timeout interrupt clear. Clears the UARTRTINTR interrupt."]
pub struct RTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIC_W<'a> {
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
#[doc = "Field `TXIC` reader - Transmit interrupt clear. Clears the UARTTXINTR interrupt."]
pub struct TXIC_R(crate::FieldReader<bool, bool>);
impl TXIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXIC` writer - Transmit interrupt clear. Clears the UARTTXINTR interrupt."]
pub struct TXIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIC_W<'a> {
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
#[doc = "Field `RXIC` reader - Receive interrupt clear. Clears the UARTRXINTR interrupt."]
pub struct RXIC_R(crate::FieldReader<bool, bool>);
impl RXIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIC` writer - Receive interrupt clear. Clears the UARTRXINTR interrupt."]
pub struct RXIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIC_W<'a> {
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
#[doc = "Field `DSRMIC` reader - nUARTDSR modem interrupt clear. Clears the UARTDSRINTR interrupt."]
pub struct DSRMIC_R(crate::FieldReader<bool, bool>);
impl DSRMIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSRMIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSRMIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSRMIC` writer - nUARTDSR modem interrupt clear. Clears the UARTDSRINTR interrupt."]
pub struct DSRMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRMIC_W<'a> {
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
#[doc = "Field `DCDMIC` reader - nUARTDCD modem interrupt clear. Clears the UARTDCDINTR interrupt."]
pub struct DCDMIC_R(crate::FieldReader<bool, bool>);
impl DCDMIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCDMIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDMIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDMIC` writer - nUARTDCD modem interrupt clear. Clears the UARTDCDINTR interrupt."]
pub struct DCDMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDMIC_W<'a> {
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
#[doc = "Field `CTSMIC` reader - nUARTCTS modem interrupt clear. Clears the UARTCTSINTR interrupt."]
pub struct CTSMIC_R(crate::FieldReader<bool, bool>);
impl CTSMIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTSMIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTSMIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSMIC` writer - nUARTCTS modem interrupt clear. Clears the UARTCTSINTR interrupt."]
pub struct CTSMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSMIC_W<'a> {
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
#[doc = "Field `RIMIC` reader - nUARTRI modem interrupt clear. Clears the UARTRIINTR interrupt."]
pub struct RIMIC_R(crate::FieldReader<bool, bool>);
impl RIMIC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RIMIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIMIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIMIC` writer - nUARTRI modem interrupt clear. Clears the UARTRIINTR interrupt."]
pub struct RIMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RIMIC_W<'a> {
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
    #[doc = "Bit 10 - Overrun error interrupt clear. Clears the UARTOEINTR interrupt."]
    #[inline(always)]
    pub fn oeic(&self) -> OEIC_R {
        OEIC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Break error interrupt clear. Clears the UARTBEINTR interrupt."]
    #[inline(always)]
    pub fn beic(&self) -> BEIC_R {
        BEIC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt clear. Clears the UARTPEINTR interrupt."]
    #[inline(always)]
    pub fn peic(&self) -> PEIC_R {
        PEIC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Framing error interrupt clear. Clears the UARTFEINTR interrupt."]
    #[inline(always)]
    pub fn feic(&self) -> FEIC_R {
        FEIC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive timeout interrupt clear. Clears the UARTRTINTR interrupt."]
    #[inline(always)]
    pub fn rtic(&self) -> RTIC_R {
        RTIC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit interrupt clear. Clears the UARTTXINTR interrupt."]
    #[inline(always)]
    pub fn txic(&self) -> TXIC_R {
        TXIC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive interrupt clear. Clears the UARTRXINTR interrupt."]
    #[inline(always)]
    pub fn rxic(&self) -> RXIC_R {
        RXIC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt clear. Clears the UARTDSRINTR interrupt."]
    #[inline(always)]
    pub fn dsrmic(&self) -> DSRMIC_R {
        DSRMIC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt clear. Clears the UARTDCDINTR interrupt."]
    #[inline(always)]
    pub fn dcdmic(&self) -> DCDMIC_R {
        DCDMIC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt clear. Clears the UARTCTSINTR interrupt."]
    #[inline(always)]
    pub fn ctsmic(&self) -> CTSMIC_R {
        CTSMIC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - nUARTRI modem interrupt clear. Clears the UARTRIINTR interrupt."]
    #[inline(always)]
    pub fn rimic(&self) -> RIMIC_R {
        RIMIC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Overrun error interrupt clear. Clears the UARTOEINTR interrupt."]
    #[inline(always)]
    pub fn oeic(&mut self) -> OEIC_W {
        OEIC_W { w: self }
    }
    #[doc = "Bit 9 - Break error interrupt clear. Clears the UARTBEINTR interrupt."]
    #[inline(always)]
    pub fn beic(&mut self) -> BEIC_W {
        BEIC_W { w: self }
    }
    #[doc = "Bit 8 - Parity error interrupt clear. Clears the UARTPEINTR interrupt."]
    #[inline(always)]
    pub fn peic(&mut self) -> PEIC_W {
        PEIC_W { w: self }
    }
    #[doc = "Bit 7 - Framing error interrupt clear. Clears the UARTFEINTR interrupt."]
    #[inline(always)]
    pub fn feic(&mut self) -> FEIC_W {
        FEIC_W { w: self }
    }
    #[doc = "Bit 6 - Receive timeout interrupt clear. Clears the UARTRTINTR interrupt."]
    #[inline(always)]
    pub fn rtic(&mut self) -> RTIC_W {
        RTIC_W { w: self }
    }
    #[doc = "Bit 5 - Transmit interrupt clear. Clears the UARTTXINTR interrupt."]
    #[inline(always)]
    pub fn txic(&mut self) -> TXIC_W {
        TXIC_W { w: self }
    }
    #[doc = "Bit 4 - Receive interrupt clear. Clears the UARTRXINTR interrupt."]
    #[inline(always)]
    pub fn rxic(&mut self) -> RXIC_W {
        RXIC_W { w: self }
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt clear. Clears the UARTDSRINTR interrupt."]
    #[inline(always)]
    pub fn dsrmic(&mut self) -> DSRMIC_W {
        DSRMIC_W { w: self }
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt clear. Clears the UARTDCDINTR interrupt."]
    #[inline(always)]
    pub fn dcdmic(&mut self) -> DCDMIC_W {
        DCDMIC_W { w: self }
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt clear. Clears the UARTCTSINTR interrupt."]
    #[inline(always)]
    pub fn ctsmic(&mut self) -> CTSMIC_W {
        CTSMIC_W { w: self }
    }
    #[doc = "Bit 0 - nUARTRI modem interrupt clear. Clears the UARTRIINTR interrupt."]
    #[inline(always)]
    pub fn rimic(&mut self) -> RIMIC_W {
        RIMIC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Clear Register, UARTICR  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [uarticr](index.html) module"]
pub struct UARTICR_SPEC;
impl crate::RegisterSpec for UARTICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uarticr::R](R) reader structure"]
impl crate::Readable for UARTICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uarticr::W](W) writer structure"]
impl crate::Writable for UARTICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UARTICR to value 0"]
impl crate::Resettable for UARTICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
