#[doc = "Register `UARTLCR_H` reader"]
pub type R = crate::R<UARTLCR_H_SPEC>;
#[doc = "Register `UARTLCR_H` writer"]
pub type W = crate::W<UARTLCR_H_SPEC>;
#[doc = "Field `BRK` reader - Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
pub type BRK_R = crate::BitReader;
#[doc = "Field `BRK` writer - Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
pub type BRK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - Parity enable: 0 = parity is disabled and no parity bit added to the data frame 1 = parity checking and generation is enabled."]
pub type PEN_R = crate::BitReader;
#[doc = "Field `PEN` writer - Parity enable: 0 = parity is disabled and no parity bit added to the data frame 1 = parity checking and generation is enabled."]
pub type PEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPS` reader - Even parity select. Controls the type of parity the UART uses during transmission and reception: 0 = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits. 1 = even parity. The UART generates or checks for an even number of 1s in the data and parity bits. This bit has no effect when the PEN bit disables parity checking and generation."]
pub type EPS_R = crate::BitReader;
#[doc = "Field `EPS` writer - Even parity select. Controls the type of parity the UART uses during transmission and reception: 0 = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits. 1 = even parity. The UART generates or checks for an even number of 1s in the data and parity bits. This bit has no effect when the PEN bit disables parity checking and generation."]
pub type EPS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STP2` reader - Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
pub type STP2_R = crate::BitReader;
#[doc = "Field `STP2` writer - Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
pub type STP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEN` reader - Enable FIFOs: 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers 1 = transmit and receive FIFO buffers are enabled (FIFO mode)."]
pub type FEN_R = crate::BitReader;
#[doc = "Field `FEN` writer - Enable FIFOs: 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers 1 = transmit and receive FIFO buffers are enabled (FIFO mode)."]
pub type FEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WLEN` reader - Word length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits b10 = 7 bits b01 = 6 bits b00 = 5 bits."]
pub type WLEN_R = crate::FieldReader;
#[doc = "Field `WLEN` writer - Word length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits b10 = 7 bits b01 = 6 bits b00 = 5 bits."]
pub type WLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPS` reader - Stick parity select. 0 = stick parity is disabled 1 = either: * if the EPS bit is 0 then the parity bit is transmitted and checked as a 1 * if the EPS bit is 1 then the parity bit is transmitted and checked as a 0. This bit has no effect when the PEN bit disables parity checking and generation."]
pub type SPS_R = crate::BitReader;
#[doc = "Field `SPS` writer - Stick parity select. 0 = stick parity is disabled 1 = either: * if the EPS bit is 0 then the parity bit is transmitted and checked as a 1 * if the EPS bit is 1 then the parity bit is transmitted and checked as a 0. This bit has no effect when the PEN bit disables parity checking and generation."]
pub type SPS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
    #[inline(always)]
    pub fn brk(&self) -> BRK_R {
        BRK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity enable: 0 = parity is disabled and no parity bit added to the data frame 1 = parity checking and generation is enabled."]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Even parity select. Controls the type of parity the UART uses during transmission and reception: 0 = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits. 1 = even parity. The UART generates or checks for an even number of 1s in the data and parity bits. This bit has no effect when the PEN bit disables parity checking and generation."]
    #[inline(always)]
    pub fn eps(&self) -> EPS_R {
        EPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
    #[inline(always)]
    pub fn stp2(&self) -> STP2_R {
        STP2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable FIFOs: 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers 1 = transmit and receive FIFO buffers are enabled (FIFO mode)."]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Word length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits b10 = 7 bits b01 = 6 bits b00 = 5 bits."]
    #[inline(always)]
    pub fn wlen(&self) -> WLEN_R {
        WLEN_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Stick parity select. 0 = stick parity is disabled 1 = either: * if the EPS bit is 0 then the parity bit is transmitted and checked as a 1 * if the EPS bit is 1 then the parity bit is transmitted and checked as a 0. This bit has no effect when the PEN bit disables parity checking and generation."]
    #[inline(always)]
    pub fn sps(&self) -> SPS_R {
        SPS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn brk(&mut self) -> BRK_W<UARTLCR_H_SPEC> {
        BRK_W::new(self, 0)
    }
    #[doc = "Bit 1 - Parity enable: 0 = parity is disabled and no parity bit added to the data frame 1 = parity checking and generation is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PEN_W<UARTLCR_H_SPEC> {
        PEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Even parity select. Controls the type of parity the UART uses during transmission and reception: 0 = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits. 1 = even parity. The UART generates or checks for an even number of 1s in the data and parity bits. This bit has no effect when the PEN bit disables parity checking and generation."]
    #[inline(always)]
    #[must_use]
    pub fn eps(&mut self) -> EPS_W<UARTLCR_H_SPEC> {
        EPS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
    #[inline(always)]
    #[must_use]
    pub fn stp2(&mut self) -> STP2_W<UARTLCR_H_SPEC> {
        STP2_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable FIFOs: 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers 1 = transmit and receive FIFO buffers are enabled (FIFO mode)."]
    #[inline(always)]
    #[must_use]
    pub fn fen(&mut self) -> FEN_W<UARTLCR_H_SPEC> {
        FEN_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Word length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits b10 = 7 bits b01 = 6 bits b00 = 5 bits."]
    #[inline(always)]
    #[must_use]
    pub fn wlen(&mut self) -> WLEN_W<UARTLCR_H_SPEC> {
        WLEN_W::new(self, 5)
    }
    #[doc = "Bit 7 - Stick parity select. 0 = stick parity is disabled 1 = either: * if the EPS bit is 0 then the parity bit is transmitted and checked as a 1 * if the EPS bit is 1 then the parity bit is transmitted and checked as a 0. This bit has no effect when the PEN bit disables parity checking and generation."]
    #[inline(always)]
    #[must_use]
    pub fn sps(&mut self) -> SPS_W<UARTLCR_H_SPEC> {
        SPS_W::new(self, 7)
    }
}
#[doc = "Line Control Register, UARTLCR_H  

You can [`read`](crate::Reg::read) this register and get [`uartlcr_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uartlcr_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UARTLCR_H_SPEC;
impl crate::RegisterSpec for UARTLCR_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uartlcr_h::R`](R) reader structure"]
impl crate::Readable for UARTLCR_H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uartlcr_h::W`](W) writer structure"]
impl crate::Writable for UARTLCR_H_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UARTLCR_H to value 0"]
impl crate::Resettable for UARTLCR_H_SPEC {
    const RESET_VALUE: u32 = 0;
}
