#[doc = "Register `SNIFF_CTRL` reader"]
pub struct R(crate::R<SNIFF_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SNIFF_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SNIFF_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SNIFF_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SNIFF_CTRL` writer"]
pub struct W(crate::W<SNIFF_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SNIFF_CTRL_SPEC>;
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
impl From<crate::W<SNIFF_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SNIFF_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable sniffer"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable sniffer"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNIFF_CTRL_SPEC, bool, O>;
#[doc = "Field `DMACH` reader - DMA channel for Sniffer to observe"]
pub type DMACH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMACH` writer - DMA channel for Sniffer to observe"]
pub type DMACH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SNIFF_CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CALC` reader - "]
pub type CALC_R = crate::FieldReader<u8, CALC_A>;
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CALC_A {
    #[doc = "0: Calculate a CRC-32 (IEEE802.3 polynomial)"]
    CRC32 = 0,
    #[doc = "1: Calculate a CRC-32 (IEEE802.3 polynomial) with bit reversed data"]
    CRC32R = 1,
    #[doc = "2: Calculate a CRC-16-CCITT"]
    CRC16 = 2,
    #[doc = "3: Calculate a CRC-16-CCITT with bit reversed data"]
    CRC16R = 3,
    #[doc = "14: XOR reduction over all data. == 1 if the total 1 population count is odd."]
    EVEN = 14,
    #[doc = "15: Calculate a simple 32-bit checksum (addition with a 32 bit accumulator)"]
    SUM = 15,
}
impl From<CALC_A> for u8 {
    #[inline(always)]
    fn from(variant: CALC_A) -> Self {
        variant as _
    }
}
impl CALC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CALC_A> {
        match self.bits {
            0 => Some(CALC_A::CRC32),
            1 => Some(CALC_A::CRC32R),
            2 => Some(CALC_A::CRC16),
            3 => Some(CALC_A::CRC16R),
            14 => Some(CALC_A::EVEN),
            15 => Some(CALC_A::SUM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CRC32`"]
    #[inline(always)]
    pub fn is_crc32(&self) -> bool {
        *self == CALC_A::CRC32
    }
    #[doc = "Checks if the value of the field is `CRC32R`"]
    #[inline(always)]
    pub fn is_crc32r(&self) -> bool {
        *self == CALC_A::CRC32R
    }
    #[doc = "Checks if the value of the field is `CRC16`"]
    #[inline(always)]
    pub fn is_crc16(&self) -> bool {
        *self == CALC_A::CRC16
    }
    #[doc = "Checks if the value of the field is `CRC16R`"]
    #[inline(always)]
    pub fn is_crc16r(&self) -> bool {
        *self == CALC_A::CRC16R
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == CALC_A::EVEN
    }
    #[doc = "Checks if the value of the field is `SUM`"]
    #[inline(always)]
    pub fn is_sum(&self) -> bool {
        *self == CALC_A::SUM
    }
}
#[doc = "Field `CALC` writer - "]
pub type CALC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SNIFF_CTRL_SPEC, u8, CALC_A, 4, O>;
impl<'a, const O: u8> CALC_W<'a, O> {
    #[doc = "Calculate a CRC-32 (IEEE802.3 polynomial)"]
    #[inline(always)]
    pub fn crc32(self) -> &'a mut W {
        self.variant(CALC_A::CRC32)
    }
    #[doc = "Calculate a CRC-32 (IEEE802.3 polynomial) with bit reversed data"]
    #[inline(always)]
    pub fn crc32r(self) -> &'a mut W {
        self.variant(CALC_A::CRC32R)
    }
    #[doc = "Calculate a CRC-16-CCITT"]
    #[inline(always)]
    pub fn crc16(self) -> &'a mut W {
        self.variant(CALC_A::CRC16)
    }
    #[doc = "Calculate a CRC-16-CCITT with bit reversed data"]
    #[inline(always)]
    pub fn crc16r(self) -> &'a mut W {
        self.variant(CALC_A::CRC16R)
    }
    #[doc = "XOR reduction over all data. == 1 if the total 1 population count is odd."]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(CALC_A::EVEN)
    }
    #[doc = "Calculate a simple 32-bit checksum (addition with a 32 bit accumulator)"]
    #[inline(always)]
    pub fn sum(self) -> &'a mut W {
        self.variant(CALC_A::SUM)
    }
}
#[doc = "Field `BSWAP` reader - Locally perform a byte reverse on the sniffed data, before feeding into checksum.  

 Note that the sniff hardware is downstream of the DMA channel byteswap performed in the read master: if channel CTRL_BSWAP and SNIFF_CTRL_BSWAP are both enabled, their effects cancel from the sniffer's point of view."]
pub type BSWAP_R = crate::BitReader<bool>;
#[doc = "Field `BSWAP` writer - Locally perform a byte reverse on the sniffed data, before feeding into checksum.  

 Note that the sniff hardware is downstream of the DMA channel byteswap performed in the read master: if channel CTRL_BSWAP and SNIFF_CTRL_BSWAP are both enabled, their effects cancel from the sniffer's point of view."]
pub type BSWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNIFF_CTRL_SPEC, bool, O>;
#[doc = "Field `OUT_REV` reader - If set, the result appears bit-reversed when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
pub type OUT_REV_R = crate::BitReader<bool>;
#[doc = "Field `OUT_REV` writer - If set, the result appears bit-reversed when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
pub type OUT_REV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNIFF_CTRL_SPEC, bool, O>;
#[doc = "Field `OUT_INV` reader - If set, the result appears inverted (bitwise complement) when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
pub type OUT_INV_R = crate::BitReader<bool>;
#[doc = "Field `OUT_INV` writer - If set, the result appears inverted (bitwise complement) when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
pub type OUT_INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, SNIFF_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable sniffer"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - DMA channel for Sniffer to observe"]
    #[inline(always)]
    pub fn dmach(&self) -> DMACH_R {
        DMACH_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:8"]
    #[inline(always)]
    pub fn calc(&self) -> CALC_R {
        CALC_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Locally perform a byte reverse on the sniffed data, before feeding into checksum.  

 Note that the sniff hardware is downstream of the DMA channel byteswap performed in the read master: if channel CTRL_BSWAP and SNIFF_CTRL_BSWAP are both enabled, their effects cancel from the sniffer's point of view."]
    #[inline(always)]
    pub fn bswap(&self) -> BSWAP_R {
        BSWAP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If set, the result appears bit-reversed when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
    #[inline(always)]
    pub fn out_rev(&self) -> OUT_REV_R {
        OUT_REV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - If set, the result appears inverted (bitwise complement) when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
    #[inline(always)]
    pub fn out_inv(&self) -> OUT_INV_R {
        OUT_INV_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable sniffer"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:4 - DMA channel for Sniffer to observe"]
    #[inline(always)]
    #[must_use]
    pub fn dmach(&mut self) -> DMACH_W<1> {
        DMACH_W::new(self)
    }
    #[doc = "Bits 5:8"]
    #[inline(always)]
    #[must_use]
    pub fn calc(&mut self) -> CALC_W<5> {
        CALC_W::new(self)
    }
    #[doc = "Bit 9 - Locally perform a byte reverse on the sniffed data, before feeding into checksum.  

 Note that the sniff hardware is downstream of the DMA channel byteswap performed in the read master: if channel CTRL_BSWAP and SNIFF_CTRL_BSWAP are both enabled, their effects cancel from the sniffer's point of view."]
    #[inline(always)]
    #[must_use]
    pub fn bswap(&mut self) -> BSWAP_W<9> {
        BSWAP_W::new(self)
    }
    #[doc = "Bit 10 - If set, the result appears bit-reversed when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
    #[inline(always)]
    #[must_use]
    pub fn out_rev(&mut self) -> OUT_REV_W<10> {
        OUT_REV_W::new(self)
    }
    #[doc = "Bit 11 - If set, the result appears inverted (bitwise complement) when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
    #[inline(always)]
    #[must_use]
    pub fn out_inv(&mut self) -> OUT_INV_W<11> {
        OUT_INV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sniffer Control  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [sniff_ctrl](index.html) module"]
pub struct SNIFF_CTRL_SPEC;
impl crate::RegisterSpec for SNIFF_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sniff_ctrl::R](R) reader structure"]
impl crate::Readable for SNIFF_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sniff_ctrl::W](W) writer structure"]
impl crate::Writable for SNIFF_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SNIFF_CTRL to value 0"]
impl crate::Resettable for SNIFF_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
