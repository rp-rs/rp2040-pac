#[doc = "Register `UARTRSR` reader"]
pub struct R(crate::R<UARTRSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTRSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTRSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTRSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UARTRSR` writer"]
pub struct W(crate::W<UARTRSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UARTRSR_SPEC>;
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
impl From<crate::W<UARTRSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UARTRSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FE` reader - Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
pub type FE_R = crate::BitReader<bool>;
#[doc = "Field `FE` writer - Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
pub type FE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, UARTRSR_SPEC, bool, O>;
#[doc = "Field `PE` reader - Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `PE` writer - Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
pub type PE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, UARTRSR_SPEC, bool, O>;
#[doc = "Field `BE` reader - Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity, and stop bits). This bit is cleared to 0 after a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received."]
pub type BE_R = crate::BitReader<bool>;
#[doc = "Field `BE` writer - Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity, and stop bits). This bit is cleared to 0 after a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received."]
pub type BE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, UARTRSR_SPEC, bool, O>;
#[doc = "Field `OE` reader - Overrun error. This bit is set to 1 if data is received and the FIFO is already full. This bit is cleared to 0 by a write to UARTECR. The FIFO contents remain valid because no more data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must now read the data, to empty the FIFO."]
pub type OE_R = crate::BitReader<bool>;
#[doc = "Field `OE` writer - Overrun error. This bit is set to 1 if data is received and the FIFO is already full. This bit is cleared to 0 by a write to UARTECR. The FIFO contents remain valid because no more data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must now read the data, to empty the FIFO."]
pub type OE_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, UARTRSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity, and stop bits). This bit is cleared to 0 after a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received."]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error. This bit is set to 1 if data is received and the FIFO is already full. This bit is cleared to 0 by a write to UARTECR. The FIFO contents remain valid because no more data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must now read the data, to empty the FIFO."]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FE_W<0> {
        FE_W::new(self)
    }
    #[doc = "Bit 1 - Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<1> {
        PE_W::new(self)
    }
    #[doc = "Bit 2 - Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity, and stop bits). This bit is cleared to 0 after a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received."]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BE_W<2> {
        BE_W::new(self)
    }
    #[doc = "Bit 3 - Overrun error. This bit is set to 1 if data is received and the FIFO is already full. This bit is cleared to 0 by a write to UARTECR. The FIFO contents remain valid because no more data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must now read the data, to empty the FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn oe(&mut self) -> OE_W<3> {
        OE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Status Register/Error Clear Register, UARTRSR/UARTECR  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [uartrsr](index.html) module"]
pub struct UARTRSR_SPEC;
impl crate::RegisterSpec for UARTRSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartrsr::R](R) reader structure"]
impl crate::Readable for UARTRSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uartrsr::W](W) writer structure"]
impl crate::Writable for UARTRSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x0f;
}
#[doc = "`reset()` method sets UARTRSR to value 0"]
impl crate::Resettable for UARTRSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
