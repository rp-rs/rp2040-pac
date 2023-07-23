#[doc = "Register `UARTDR` reader"]
pub struct R(crate::R<UARTDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UARTDR` writer"]
pub struct W(crate::W<UARTDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UARTDR_SPEC>;
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
impl From<crate::W<UARTDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UARTDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Receive (read) data character. Transmit (write) data character."]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - Receive (read) data character. Transmit (write) data character."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UARTDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `FE` reader - Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). In FIFO mode, this error is associated with the character at the top of the FIFO."]
pub type FE_R = crate::BitReader<bool>;
#[doc = "Field `PE` reader - Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H. In FIFO mode, this error is associated with the character at the top of the FIFO."]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `BE` reader - Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state), and the next valid start bit is received."]
pub type BE_R = crate::BitReader<bool>;
#[doc = "Field `OE` reader - Overrun error. This bit is set to 1 if data is received and the receive FIFO is already full. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it."]
pub type OE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:7 - Receive (read) data character. Transmit (write) data character."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state), and the next valid start bit is received."]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Overrun error. This bit is set to 1 if data is received and the receive FIFO is already full. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it."]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive (read) data character. Transmit (write) data character."]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Register, UARTDR  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [uartdr](index.html) module"]
pub struct UARTDR_SPEC;
impl crate::RegisterSpec for UARTDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartdr::R](R) reader structure"]
impl crate::Readable for UARTDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uartdr::W](W) writer structure"]
impl crate::Writable for UARTDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UARTDR to value 0"]
impl crate::Resettable for UARTDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
