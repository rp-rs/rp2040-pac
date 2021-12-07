#[doc = "Register `UARTLCR_H` reader"]
pub struct R(crate::R<UARTLCR_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTLCR_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTLCR_H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTLCR_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UARTLCR_H` writer"]
pub struct W(crate::W<UARTLCR_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UARTLCR_H_SPEC>;
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
impl From<crate::W<UARTLCR_H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UARTLCR_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPS` reader - Stick parity select. 0 = stick parity is disabled 1 = either: * if the EPS bit is 0 then the parity bit is transmitted and checked as a 1 * if the EPS bit is 1 then the parity bit is transmitted and checked as a 0. This bit has no effect when the PEN bit disables parity checking and generation."]
pub struct SPS_R(crate::FieldReader<bool, bool>);
impl SPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPS` writer - Stick parity select. 0 = stick parity is disabled 1 = either: * if the EPS bit is 0 then the parity bit is transmitted and checked as a 1 * if the EPS bit is 1 then the parity bit is transmitted and checked as a 0. This bit has no effect when the PEN bit disables parity checking and generation."]
pub struct SPS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPS_W<'a> {
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
#[doc = "Field `WLEN` reader - Word length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits b10 = 7 bits b01 = 6 bits b00 = 5 bits."]
pub struct WLEN_R(crate::FieldReader<u8, u8>);
impl WLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WLEN` writer - Word length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits b10 = 7 bits b01 = 6 bits b00 = 5 bits."]
pub struct WLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `FEN` reader - Enable FIFOs: 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers 1 = transmit and receive FIFO buffers are enabled (FIFO mode)."]
pub struct FEN_R(crate::FieldReader<bool, bool>);
impl FEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEN` writer - Enable FIFOs: 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers 1 = transmit and receive FIFO buffers are enabled (FIFO mode)."]
pub struct FEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FEN_W<'a> {
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
#[doc = "Field `STP2` reader - Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
pub struct STP2_R(crate::FieldReader<bool, bool>);
impl STP2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        STP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STP2` writer - Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
pub struct STP2_W<'a> {
    w: &'a mut W,
}
impl<'a> STP2_W<'a> {
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
#[doc = "Field `EPS` reader - Even parity select. Controls the type of parity the UART uses during transmission and reception: 0 = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits. 1 = even parity. The UART generates or checks for an even number of 1s in the data and parity bits. This bit has no effect when the PEN bit disables parity checking and generation."]
pub struct EPS_R(crate::FieldReader<bool, bool>);
impl EPS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPS` writer - Even parity select. Controls the type of parity the UART uses during transmission and reception: 0 = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits. 1 = even parity. The UART generates or checks for an even number of 1s in the data and parity bits. This bit has no effect when the PEN bit disables parity checking and generation."]
pub struct EPS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPS_W<'a> {
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
#[doc = "Field `PEN` reader - Parity enable: 0 = parity is disabled and no parity bit added to the data frame 1 = parity checking and generation is enabled."]
pub struct PEN_R(crate::FieldReader<bool, bool>);
impl PEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN` writer - Parity enable: 0 = parity is disabled and no parity bit added to the data frame 1 = parity checking and generation is enabled."]
pub struct PEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN_W<'a> {
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
#[doc = "Field `BRK` reader - Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
pub struct BRK_R(crate::FieldReader<bool, bool>);
impl BRK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BRK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRK` writer - Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
pub struct BRK_W<'a> {
    w: &'a mut W,
}
impl<'a> BRK_W<'a> {
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
    #[doc = "Bit 7 - Stick parity select. 0 = stick parity is disabled 1 = either: * if the EPS bit is 0 then the parity bit is transmitted and checked as a 1 * if the EPS bit is 1 then the parity bit is transmitted and checked as a 0. This bit has no effect when the PEN bit disables parity checking and generation."]
    #[inline(always)]
    pub fn sps(&self) -> SPS_R {
        SPS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Word length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits b10 = 7 bits b01 = 6 bits b00 = 5 bits."]
    #[inline(always)]
    pub fn wlen(&self) -> WLEN_R {
        WLEN_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Enable FIFOs: 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers 1 = transmit and receive FIFO buffers are enabled (FIFO mode)."]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
    #[inline(always)]
    pub fn stp2(&self) -> STP2_R {
        STP2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Even parity select. Controls the type of parity the UART uses during transmission and reception: 0 = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits. 1 = even parity. The UART generates or checks for an even number of 1s in the data and parity bits. This bit has no effect when the PEN bit disables parity checking and generation."]
    #[inline(always)]
    pub fn eps(&self) -> EPS_R {
        EPS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Parity enable: 0 = parity is disabled and no parity bit added to the data frame 1 = parity checking and generation is enabled."]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
    #[inline(always)]
    pub fn brk(&self) -> BRK_R {
        BRK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Stick parity select. 0 = stick parity is disabled 1 = either: * if the EPS bit is 0 then the parity bit is transmitted and checked as a 1 * if the EPS bit is 1 then the parity bit is transmitted and checked as a 0. This bit has no effect when the PEN bit disables parity checking and generation."]
    #[inline(always)]
    pub fn sps(&mut self) -> SPS_W {
        SPS_W { w: self }
    }
    #[doc = "Bits 5:6 - Word length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits b10 = 7 bits b01 = 6 bits b00 = 5 bits."]
    #[inline(always)]
    pub fn wlen(&mut self) -> WLEN_W {
        WLEN_W { w: self }
    }
    #[doc = "Bit 4 - Enable FIFOs: 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers 1 = transmit and receive FIFO buffers are enabled (FIFO mode)."]
    #[inline(always)]
    pub fn fen(&mut self) -> FEN_W {
        FEN_W { w: self }
    }
    #[doc = "Bit 3 - Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
    #[inline(always)]
    pub fn stp2(&mut self) -> STP2_W {
        STP2_W { w: self }
    }
    #[doc = "Bit 2 - Even parity select. Controls the type of parity the UART uses during transmission and reception: 0 = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits. 1 = even parity. The UART generates or checks for an even number of 1s in the data and parity bits. This bit has no effect when the PEN bit disables parity checking and generation."]
    #[inline(always)]
    pub fn eps(&mut self) -> EPS_W {
        EPS_W { w: self }
    }
    #[doc = "Bit 1 - Parity enable: 0 = parity is disabled and no parity bit added to the data frame 1 = parity checking and generation is enabled."]
    #[inline(always)]
    pub fn pen(&mut self) -> PEN_W {
        PEN_W { w: self }
    }
    #[doc = "Bit 0 - Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
    #[inline(always)]
    pub fn brk(&mut self) -> BRK_W {
        BRK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line Control Register, UARTLCR_H  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [uartlcr_h](index.html) module"]
pub struct UARTLCR_H_SPEC;
impl crate::RegisterSpec for UARTLCR_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartlcr_h::R](R) reader structure"]
impl crate::Readable for UARTLCR_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uartlcr_h::W](W) writer structure"]
impl crate::Writable for UARTLCR_H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UARTLCR_H to value 0"]
impl crate::Resettable for UARTLCR_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
