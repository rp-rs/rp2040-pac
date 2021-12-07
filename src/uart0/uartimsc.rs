#[doc = "Register `UARTIMSC` reader"]
pub struct R(crate::R<UARTIMSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTIMSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTIMSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTIMSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UARTIMSC` writer"]
pub struct W(crate::W<UARTIMSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UARTIMSC_SPEC>;
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
impl From<crate::W<UARTIMSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UARTIMSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OEIM` reader - Overrun error interrupt mask. A read returns the current mask for the UARTOEINTR interrupt. On a write of 1, the mask of the UARTOEINTR interrupt is set. A write of 0 clears the mask."]
pub struct OEIM_R(crate::FieldReader<bool, bool>);
impl OEIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OEIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEIM` writer - Overrun error interrupt mask. A read returns the current mask for the UARTOEINTR interrupt. On a write of 1, the mask of the UARTOEINTR interrupt is set. A write of 0 clears the mask."]
pub struct OEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> OEIM_W<'a> {
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
#[doc = "Field `BEIM` reader - Break error interrupt mask. A read returns the current mask for the UARTBEINTR interrupt. On a write of 1, the mask of the UARTBEINTR interrupt is set. A write of 0 clears the mask."]
pub struct BEIM_R(crate::FieldReader<bool, bool>);
impl BEIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BEIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEIM` writer - Break error interrupt mask. A read returns the current mask for the UARTBEINTR interrupt. On a write of 1, the mask of the UARTBEINTR interrupt is set. A write of 0 clears the mask."]
pub struct BEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> BEIM_W<'a> {
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
#[doc = "Field `PEIM` reader - Parity error interrupt mask. A read returns the current mask for the UARTPEINTR interrupt. On a write of 1, the mask of the UARTPEINTR interrupt is set. A write of 0 clears the mask."]
pub struct PEIM_R(crate::FieldReader<bool, bool>);
impl PEIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEIM` writer - Parity error interrupt mask. A read returns the current mask for the UARTPEINTR interrupt. On a write of 1, the mask of the UARTPEINTR interrupt is set. A write of 0 clears the mask."]
pub struct PEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PEIM_W<'a> {
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
#[doc = "Field `FEIM` reader - Framing error interrupt mask. A read returns the current mask for the UARTFEINTR interrupt. On a write of 1, the mask of the UARTFEINTR interrupt is set. A write of 0 clears the mask."]
pub struct FEIM_R(crate::FieldReader<bool, bool>);
impl FEIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIM` writer - Framing error interrupt mask. A read returns the current mask for the UARTFEINTR interrupt. On a write of 1, the mask of the UARTFEINTR interrupt is set. A write of 0 clears the mask."]
pub struct FEIM_W<'a> {
    w: &'a mut W,
}
impl<'a> FEIM_W<'a> {
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
#[doc = "Field `RTIM` reader - Receive timeout interrupt mask. A read returns the current mask for the UARTRTINTR interrupt. On a write of 1, the mask of the UARTRTINTR interrupt is set. A write of 0 clears the mask."]
pub struct RTIM_R(crate::FieldReader<bool, bool>);
impl RTIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTIM` writer - Receive timeout interrupt mask. A read returns the current mask for the UARTRTINTR interrupt. On a write of 1, the mask of the UARTRTINTR interrupt is set. A write of 0 clears the mask."]
pub struct RTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTIM_W<'a> {
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
#[doc = "Field `TXIM` reader - Transmit interrupt mask. A read returns the current mask for the UARTTXINTR interrupt. On a write of 1, the mask of the UARTTXINTR interrupt is set. A write of 0 clears the mask."]
pub struct TXIM_R(crate::FieldReader<bool, bool>);
impl TXIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXIM` writer - Transmit interrupt mask. A read returns the current mask for the UARTTXINTR interrupt. On a write of 1, the mask of the UARTTXINTR interrupt is set. A write of 0 clears the mask."]
pub struct TXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIM_W<'a> {
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
#[doc = "Field `RXIM` reader - Receive interrupt mask. A read returns the current mask for the UARTRXINTR interrupt. On a write of 1, the mask of the UARTRXINTR interrupt is set. A write of 0 clears the mask."]
pub struct RXIM_R(crate::FieldReader<bool, bool>);
impl RXIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIM` writer - Receive interrupt mask. A read returns the current mask for the UARTRXINTR interrupt. On a write of 1, the mask of the UARTRXINTR interrupt is set. A write of 0 clears the mask."]
pub struct RXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIM_W<'a> {
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
#[doc = "Field `DSRMIM` reader - nUARTDSR modem interrupt mask. A read returns the current mask for the UARTDSRINTR interrupt. On a write of 1, the mask of the UARTDSRINTR interrupt is set. A write of 0 clears the mask."]
pub struct DSRMIM_R(crate::FieldReader<bool, bool>);
impl DSRMIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSRMIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSRMIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSRMIM` writer - nUARTDSR modem interrupt mask. A read returns the current mask for the UARTDSRINTR interrupt. On a write of 1, the mask of the UARTDSRINTR interrupt is set. A write of 0 clears the mask."]
pub struct DSRMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DSRMIM_W<'a> {
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
#[doc = "Field `DCDMIM` reader - nUARTDCD modem interrupt mask. A read returns the current mask for the UARTDCDINTR interrupt. On a write of 1, the mask of the UARTDCDINTR interrupt is set. A write of 0 clears the mask."]
pub struct DCDMIM_R(crate::FieldReader<bool, bool>);
impl DCDMIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCDMIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDMIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDMIM` writer - nUARTDCD modem interrupt mask. A read returns the current mask for the UARTDCDINTR interrupt. On a write of 1, the mask of the UARTDCDINTR interrupt is set. A write of 0 clears the mask."]
pub struct DCDMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDMIM_W<'a> {
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
#[doc = "Field `CTSMIM` reader - nUARTCTS modem interrupt mask. A read returns the current mask for the UARTCTSINTR interrupt. On a write of 1, the mask of the UARTCTSINTR interrupt is set. A write of 0 clears the mask."]
pub struct CTSMIM_R(crate::FieldReader<bool, bool>);
impl CTSMIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTSMIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTSMIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSMIM` writer - nUARTCTS modem interrupt mask. A read returns the current mask for the UARTCTSINTR interrupt. On a write of 1, the mask of the UARTCTSINTR interrupt is set. A write of 0 clears the mask."]
pub struct CTSMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSMIM_W<'a> {
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
#[doc = "Field `RIMIM` reader - nUARTRI modem interrupt mask. A read returns the current mask for the UARTRIINTR interrupt. On a write of 1, the mask of the UARTRIINTR interrupt is set. A write of 0 clears the mask."]
pub struct RIMIM_R(crate::FieldReader<bool, bool>);
impl RIMIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RIMIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIMIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIMIM` writer - nUARTRI modem interrupt mask. A read returns the current mask for the UARTRIINTR interrupt. On a write of 1, the mask of the UARTRIINTR interrupt is set. A write of 0 clears the mask."]
pub struct RIMIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RIMIM_W<'a> {
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
    #[doc = "Bit 10 - Overrun error interrupt mask. A read returns the current mask for the UARTOEINTR interrupt. On a write of 1, the mask of the UARTOEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn oeim(&self) -> OEIM_R {
        OEIM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Break error interrupt mask. A read returns the current mask for the UARTBEINTR interrupt. On a write of 1, the mask of the UARTBEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn beim(&self) -> BEIM_R {
        BEIM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt mask. A read returns the current mask for the UARTPEINTR interrupt. On a write of 1, the mask of the UARTPEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn peim(&self) -> PEIM_R {
        PEIM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Framing error interrupt mask. A read returns the current mask for the UARTFEINTR interrupt. On a write of 1, the mask of the UARTFEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn feim(&self) -> FEIM_R {
        FEIM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive timeout interrupt mask. A read returns the current mask for the UARTRTINTR interrupt. On a write of 1, the mask of the UARTRTINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit interrupt mask. A read returns the current mask for the UARTTXINTR interrupt. On a write of 1, the mask of the UARTTXINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive interrupt mask. A read returns the current mask for the UARTRXINTR interrupt. On a write of 1, the mask of the UARTRXINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt mask. A read returns the current mask for the UARTDSRINTR interrupt. On a write of 1, the mask of the UARTDSRINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn dsrmim(&self) -> DSRMIM_R {
        DSRMIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt mask. A read returns the current mask for the UARTDCDINTR interrupt. On a write of 1, the mask of the UARTDCDINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn dcdmim(&self) -> DCDMIM_R {
        DCDMIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt mask. A read returns the current mask for the UARTCTSINTR interrupt. On a write of 1, the mask of the UARTCTSINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn ctsmim(&self) -> CTSMIM_R {
        CTSMIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - nUARTRI modem interrupt mask. A read returns the current mask for the UARTRIINTR interrupt. On a write of 1, the mask of the UARTRIINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn rimim(&self) -> RIMIM_R {
        RIMIM_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10 - Overrun error interrupt mask. A read returns the current mask for the UARTOEINTR interrupt. On a write of 1, the mask of the UARTOEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn oeim(&mut self) -> OEIM_W {
        OEIM_W { w: self }
    }
    #[doc = "Bit 9 - Break error interrupt mask. A read returns the current mask for the UARTBEINTR interrupt. On a write of 1, the mask of the UARTBEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn beim(&mut self) -> BEIM_W {
        BEIM_W { w: self }
    }
    #[doc = "Bit 8 - Parity error interrupt mask. A read returns the current mask for the UARTPEINTR interrupt. On a write of 1, the mask of the UARTPEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn peim(&mut self) -> PEIM_W {
        PEIM_W { w: self }
    }
    #[doc = "Bit 7 - Framing error interrupt mask. A read returns the current mask for the UARTFEINTR interrupt. On a write of 1, the mask of the UARTFEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn feim(&mut self) -> FEIM_W {
        FEIM_W { w: self }
    }
    #[doc = "Bit 6 - Receive timeout interrupt mask. A read returns the current mask for the UARTRTINTR interrupt. On a write of 1, the mask of the UARTRTINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn rtim(&mut self) -> RTIM_W {
        RTIM_W { w: self }
    }
    #[doc = "Bit 5 - Transmit interrupt mask. A read returns the current mask for the UARTTXINTR interrupt. On a write of 1, the mask of the UARTTXINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn txim(&mut self) -> TXIM_W {
        TXIM_W { w: self }
    }
    #[doc = "Bit 4 - Receive interrupt mask. A read returns the current mask for the UARTRXINTR interrupt. On a write of 1, the mask of the UARTRXINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn rxim(&mut self) -> RXIM_W {
        RXIM_W { w: self }
    }
    #[doc = "Bit 3 - nUARTDSR modem interrupt mask. A read returns the current mask for the UARTDSRINTR interrupt. On a write of 1, the mask of the UARTDSRINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn dsrmim(&mut self) -> DSRMIM_W {
        DSRMIM_W { w: self }
    }
    #[doc = "Bit 2 - nUARTDCD modem interrupt mask. A read returns the current mask for the UARTDCDINTR interrupt. On a write of 1, the mask of the UARTDCDINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn dcdmim(&mut self) -> DCDMIM_W {
        DCDMIM_W { w: self }
    }
    #[doc = "Bit 1 - nUARTCTS modem interrupt mask. A read returns the current mask for the UARTCTSINTR interrupt. On a write of 1, the mask of the UARTCTSINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn ctsmim(&mut self) -> CTSMIM_W {
        CTSMIM_W { w: self }
    }
    #[doc = "Bit 0 - nUARTRI modem interrupt mask. A read returns the current mask for the UARTRIINTR interrupt. On a write of 1, the mask of the UARTRIINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn rimim(&mut self) -> RIMIM_W {
        RIMIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Set/Clear Register, UARTIMSC  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [uartimsc](index.html) module"]
pub struct UARTIMSC_SPEC;
impl crate::RegisterSpec for UARTIMSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartimsc::R](R) reader structure"]
impl crate::Readable for UARTIMSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uartimsc::W](W) writer structure"]
impl crate::Writable for UARTIMSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UARTIMSC to value 0"]
impl crate::Resettable for UARTIMSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
