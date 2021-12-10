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
#[doc = "Field `OUT_INV` reader - If set, the result appears inverted (bitwise complement) when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
pub struct OUT_INV_R(crate::FieldReader<bool, bool>);
impl OUT_INV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_INV` writer - If set, the result appears inverted (bitwise complement) when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
pub struct OUT_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_INV_W<'a> {
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
#[doc = "Field `OUT_REV` reader - If set, the result appears bit-reversed when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
pub struct OUT_REV_R(crate::FieldReader<bool, bool>);
impl OUT_REV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUT_REV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT_REV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT_REV` writer - If set, the result appears bit-reversed when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
pub struct OUT_REV_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT_REV_W<'a> {
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
#[doc = "Field `BSWAP` reader - Locally perform a byte reverse on the sniffed data, before feeding into checksum.  

 Note that the sniff hardware is downstream of the DMA channel byteswap performed in the read master: if channel CTRL_BSWAP and SNIFF_CTRL_BSWAP are both enabled, their effects cancel from the sniffer's point of view."]
pub struct BSWAP_R(crate::FieldReader<bool, bool>);
impl BSWAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BSWAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BSWAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BSWAP` writer - Locally perform a byte reverse on the sniffed data, before feeding into checksum.  

 Note that the sniff hardware is downstream of the DMA channel byteswap performed in the read master: if channel CTRL_BSWAP and SNIFF_CTRL_BSWAP are both enabled, their effects cancel from the sniffer's point of view."]
pub struct BSWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> BSWAP_W<'a> {
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
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `CALC` reader - "]
pub struct CALC_R(crate::FieldReader<u8, CALC_A>);
impl CALC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CALC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == CALC_A::CRC32
    }
    #[doc = "Checks if the value of the field is `CRC32R`"]
    #[inline(always)]
    pub fn is_crc32r(&self) -> bool {
        **self == CALC_A::CRC32R
    }
    #[doc = "Checks if the value of the field is `CRC16`"]
    #[inline(always)]
    pub fn is_crc16(&self) -> bool {
        **self == CALC_A::CRC16
    }
    #[doc = "Checks if the value of the field is `CRC16R`"]
    #[inline(always)]
    pub fn is_crc16r(&self) -> bool {
        **self == CALC_A::CRC16R
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        **self == CALC_A::EVEN
    }
    #[doc = "Checks if the value of the field is `SUM`"]
    #[inline(always)]
    pub fn is_sum(&self) -> bool {
        **self == CALC_A::SUM
    }
}
impl core::ops::Deref for CALC_R {
    type Target = crate::FieldReader<u8, CALC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALC` writer - "]
pub struct CALC_W<'a> {
    w: &'a mut W,
}
impl<'a> CALC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CALC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | ((value as u32 & 0x0f) << 5);
        self.w
    }
}
#[doc = "Field `DMACH` reader - DMA channel for Sniffer to observe"]
pub struct DMACH_R(crate::FieldReader<u8, u8>);
impl DMACH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DMACH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMACH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMACH` writer - DMA channel for Sniffer to observe"]
pub struct DMACH_W<'a> {
    w: &'a mut W,
}
impl<'a> DMACH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | ((value as u32 & 0x0f) << 1);
        self.w
    }
}
#[doc = "Field `EN` reader - Enable sniffer"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Enable sniffer"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
    #[doc = "Bit 11 - If set, the result appears inverted (bitwise complement) when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
    #[inline(always)]
    pub fn out_inv(&self) -> OUT_INV_R {
        OUT_INV_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - If set, the result appears bit-reversed when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
    #[inline(always)]
    pub fn out_rev(&self) -> OUT_REV_R {
        OUT_REV_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Locally perform a byte reverse on the sniffed data, before feeding into checksum.  

 Note that the sniff hardware is downstream of the DMA channel byteswap performed in the read master: if channel CTRL_BSWAP and SNIFF_CTRL_BSWAP are both enabled, their effects cancel from the sniffer's point of view."]
    #[inline(always)]
    pub fn bswap(&self) -> BSWAP_R {
        BSWAP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 5:8"]
    #[inline(always)]
    pub fn calc(&self) -> CALC_R {
        CALC_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 1:4 - DMA channel for Sniffer to observe"]
    #[inline(always)]
    pub fn dmach(&self) -> DMACH_R {
        DMACH_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - Enable sniffer"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - If set, the result appears inverted (bitwise complement) when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
    #[inline(always)]
    pub fn out_inv(&mut self) -> OUT_INV_W {
        OUT_INV_W { w: self }
    }
    #[doc = "Bit 10 - If set, the result appears bit-reversed when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
    #[inline(always)]
    pub fn out_rev(&mut self) -> OUT_REV_W {
        OUT_REV_W { w: self }
    }
    #[doc = "Bit 9 - Locally perform a byte reverse on the sniffed data, before feeding into checksum.  

 Note that the sniff hardware is downstream of the DMA channel byteswap performed in the read master: if channel CTRL_BSWAP and SNIFF_CTRL_BSWAP are both enabled, their effects cancel from the sniffer's point of view."]
    #[inline(always)]
    pub fn bswap(&mut self) -> BSWAP_W {
        BSWAP_W { w: self }
    }
    #[doc = "Bits 5:8"]
    #[inline(always)]
    pub fn calc(&mut self) -> CALC_W {
        CALC_W { w: self }
    }
    #[doc = "Bits 1:4 - DMA channel for Sniffer to observe"]
    #[inline(always)]
    pub fn dmach(&mut self) -> DMACH_W {
        DMACH_W { w: self }
    }
    #[doc = "Bit 0 - Enable sniffer"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
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
}
#[doc = "`reset()` method sets SNIFF_CTRL to value 0"]
impl crate::Resettable for SNIFF_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
