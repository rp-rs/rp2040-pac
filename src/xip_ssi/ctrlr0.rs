#[doc = "Register `CTRLR0` reader"]
pub struct R(crate::R<CTRLR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLR0` writer"]
pub struct W(crate::W<CTRLR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLR0_SPEC>;
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
impl From<crate::W<CTRLR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSTE` reader - Slave select toggle enable"]
pub struct SSTE_R(crate::FieldReader<bool, bool>);
impl SSTE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSTE` writer - Slave select toggle enable"]
pub struct SSTE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSTE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "SPI frame format  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPI_FRF_A {
    #[doc = "0: Standard 1-bit SPI frame format; 1 bit per SCK, full-duplex"]
    STD = 0,
    #[doc = "1: Dual-SPI frame format; two bits per SCK, half-duplex"]
    DUAL = 1,
    #[doc = "2: Quad-SPI frame format; four bits per SCK, half-duplex"]
    QUAD = 2,
}
impl From<SPI_FRF_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI_FRF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPI_FRF` reader - SPI frame format"]
pub struct SPI_FRF_R(crate::FieldReader<u8, SPI_FRF_A>);
impl SPI_FRF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_FRF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPI_FRF_A> {
        match self.bits {
            0 => Some(SPI_FRF_A::STD),
            1 => Some(SPI_FRF_A::DUAL),
            2 => Some(SPI_FRF_A::QUAD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STD`"]
    #[inline(always)]
    pub fn is_std(&self) -> bool {
        **self == SPI_FRF_A::STD
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        **self == SPI_FRF_A::DUAL
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        **self == SPI_FRF_A::QUAD
    }
}
impl core::ops::Deref for SPI_FRF_R {
    type Target = crate::FieldReader<u8, SPI_FRF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_FRF` writer - SPI frame format"]
pub struct SPI_FRF_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_FRF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI_FRF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Standard 1-bit SPI frame format; 1 bit per SCK, full-duplex"]
    #[inline(always)]
    pub fn std(self) -> &'a mut W {
        self.variant(SPI_FRF_A::STD)
    }
    #[doc = "Dual-SPI frame format; two bits per SCK, half-duplex"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut W {
        self.variant(SPI_FRF_A::DUAL)
    }
    #[doc = "Quad-SPI frame format; four bits per SCK, half-duplex"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut W {
        self.variant(SPI_FRF_A::QUAD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `DFS_32` reader - Data frame size in 32b transfer mode  
 Value of n -> n+1 clocks per frame."]
pub struct DFS_32_R(crate::FieldReader<u8, u8>);
impl DFS_32_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DFS_32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFS_32_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFS_32` writer - Data frame size in 32b transfer mode  
 Value of n -> n+1 clocks per frame."]
pub struct DFS_32_W<'a> {
    w: &'a mut W,
}
impl<'a> DFS_32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `CFS` reader - Control frame size  
 Value of n -> n+1 clocks per frame."]
pub struct CFS_R(crate::FieldReader<u8, u8>);
impl CFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFS` writer - Control frame size  
 Value of n -> n+1 clocks per frame."]
pub struct CFS_W<'a> {
    w: &'a mut W,
}
impl<'a> CFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `SRL` reader - Shift register loop (test mode)"]
pub struct SRL_R(crate::FieldReader<bool, bool>);
impl SRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRL` writer - Shift register loop (test mode)"]
pub struct SRL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRL_W<'a> {
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
#[doc = "Field `SLV_OE` reader - Slave output enable"]
pub struct SLV_OE_R(crate::FieldReader<bool, bool>);
impl SLV_OE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLV_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLV_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLV_OE` writer - Slave output enable"]
pub struct SLV_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLV_OE_W<'a> {
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
#[doc = "Transfer mode  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMOD_A {
    #[doc = "0: Both transmit and receive"]
    TX_AND_RX = 0,
    #[doc = "1: Transmit only (not for FRF == 0, standard SPI mode)"]
    TX_ONLY = 1,
    #[doc = "2: Receive only (not for FRF == 0, standard SPI mode)"]
    RX_ONLY = 2,
    #[doc = "3: EEPROM read mode (TX then RX; RX starts after control data TX'd)"]
    EEPROM_READ = 3,
}
impl From<TMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: TMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMOD` reader - Transfer mode"]
pub struct TMOD_R(crate::FieldReader<u8, TMOD_A>);
impl TMOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TMOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMOD_A {
        match self.bits {
            0 => TMOD_A::TX_AND_RX,
            1 => TMOD_A::TX_ONLY,
            2 => TMOD_A::RX_ONLY,
            3 => TMOD_A::EEPROM_READ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TX_AND_RX`"]
    #[inline(always)]
    pub fn is_tx_and_rx(&self) -> bool {
        **self == TMOD_A::TX_AND_RX
    }
    #[doc = "Checks if the value of the field is `TX_ONLY`"]
    #[inline(always)]
    pub fn is_tx_only(&self) -> bool {
        **self == TMOD_A::TX_ONLY
    }
    #[doc = "Checks if the value of the field is `RX_ONLY`"]
    #[inline(always)]
    pub fn is_rx_only(&self) -> bool {
        **self == TMOD_A::RX_ONLY
    }
    #[doc = "Checks if the value of the field is `EEPROM_READ`"]
    #[inline(always)]
    pub fn is_eeprom_read(&self) -> bool {
        **self == TMOD_A::EEPROM_READ
    }
}
impl core::ops::Deref for TMOD_R {
    type Target = crate::FieldReader<u8, TMOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMOD` writer - Transfer mode"]
pub struct TMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMOD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Both transmit and receive"]
    #[inline(always)]
    pub fn tx_and_rx(self) -> &'a mut W {
        self.variant(TMOD_A::TX_AND_RX)
    }
    #[doc = "Transmit only (not for FRF == 0, standard SPI mode)"]
    #[inline(always)]
    pub fn tx_only(self) -> &'a mut W {
        self.variant(TMOD_A::TX_ONLY)
    }
    #[doc = "Receive only (not for FRF == 0, standard SPI mode)"]
    #[inline(always)]
    pub fn rx_only(self) -> &'a mut W {
        self.variant(TMOD_A::RX_ONLY)
    }
    #[doc = "EEPROM read mode (TX then RX; RX starts after control data TX'd)"]
    #[inline(always)]
    pub fn eeprom_read(self) -> &'a mut W {
        self.variant(TMOD_A::EEPROM_READ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `SCPOL` reader - Serial clock polarity"]
pub struct SCPOL_R(crate::FieldReader<bool, bool>);
impl SCPOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCPOL` writer - Serial clock polarity"]
pub struct SCPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SCPOL_W<'a> {
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
#[doc = "Field `SCPH` reader - Serial clock phase"]
pub struct SCPH_R(crate::FieldReader<bool, bool>);
impl SCPH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SCPH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCPH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCPH` writer - Serial clock phase"]
pub struct SCPH_W<'a> {
    w: &'a mut W,
}
impl<'a> SCPH_W<'a> {
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
#[doc = "Field `FRF` reader - Frame format"]
pub struct FRF_R(crate::FieldReader<u8, u8>);
impl FRF_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRF` writer - Frame format"]
pub struct FRF_W<'a> {
    w: &'a mut W,
}
impl<'a> FRF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `DFS` reader - Data frame size"]
pub struct DFS_R(crate::FieldReader<u8, u8>);
impl DFS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DFS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFS` writer - Data frame size"]
pub struct DFS_W<'a> {
    w: &'a mut W,
}
impl<'a> DFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bit 24 - Slave select toggle enable"]
    #[inline(always)]
    pub fn sste(&self) -> SSTE_R {
        SSTE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - SPI frame format"]
    #[inline(always)]
    pub fn spi_frf(&self) -> SPI_FRF_R {
        SPI_FRF_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 16:20 - Data frame size in 32b transfer mode  
 Value of n -> n+1 clocks per frame."]
    #[inline(always)]
    pub fn dfs_32(&self) -> DFS_32_R {
        DFS_32_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - Control frame size  
 Value of n -> n+1 clocks per frame."]
    #[inline(always)]
    pub fn cfs(&self) -> CFS_R {
        CFS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - Shift register loop (test mode)"]
    #[inline(always)]
    pub fn srl(&self) -> SRL_R {
        SRL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Slave output enable"]
    #[inline(always)]
    pub fn slv_oe(&self) -> SLV_OE_R {
        SLV_OE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Transfer mode"]
    #[inline(always)]
    pub fn tmod(&self) -> TMOD_R {
        TMOD_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Serial clock polarity"]
    #[inline(always)]
    pub fn scpol(&self) -> SCPOL_R {
        SCPOL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Serial clock phase"]
    #[inline(always)]
    pub fn scph(&self) -> SCPH_R {
        SCPH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Frame format"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3 - Data frame size"]
    #[inline(always)]
    pub fn dfs(&self) -> DFS_R {
        DFS_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - Slave select toggle enable"]
    #[inline(always)]
    pub fn sste(&mut self) -> SSTE_W {
        SSTE_W { w: self }
    }
    #[doc = "Bits 21:22 - SPI frame format"]
    #[inline(always)]
    pub fn spi_frf(&mut self) -> SPI_FRF_W {
        SPI_FRF_W { w: self }
    }
    #[doc = "Bits 16:20 - Data frame size in 32b transfer mode  
 Value of n -> n+1 clocks per frame."]
    #[inline(always)]
    pub fn dfs_32(&mut self) -> DFS_32_W {
        DFS_32_W { w: self }
    }
    #[doc = "Bits 12:15 - Control frame size  
 Value of n -> n+1 clocks per frame."]
    #[inline(always)]
    pub fn cfs(&mut self) -> CFS_W {
        CFS_W { w: self }
    }
    #[doc = "Bit 11 - Shift register loop (test mode)"]
    #[inline(always)]
    pub fn srl(&mut self) -> SRL_W {
        SRL_W { w: self }
    }
    #[doc = "Bit 10 - Slave output enable"]
    #[inline(always)]
    pub fn slv_oe(&mut self) -> SLV_OE_W {
        SLV_OE_W { w: self }
    }
    #[doc = "Bits 8:9 - Transfer mode"]
    #[inline(always)]
    pub fn tmod(&mut self) -> TMOD_W {
        TMOD_W { w: self }
    }
    #[doc = "Bit 7 - Serial clock polarity"]
    #[inline(always)]
    pub fn scpol(&mut self) -> SCPOL_W {
        SCPOL_W { w: self }
    }
    #[doc = "Bit 6 - Serial clock phase"]
    #[inline(always)]
    pub fn scph(&mut self) -> SCPH_W {
        SCPH_W { w: self }
    }
    #[doc = "Bits 4:5 - Frame format"]
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W {
        FRF_W { w: self }
    }
    #[doc = "Bits 0:3 - Data frame size"]
    #[inline(always)]
    pub fn dfs(&mut self) -> DFS_W {
        DFS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 0  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ctrlr0](index.html) module"]
pub struct CTRLR0_SPEC;
impl crate::RegisterSpec for CTRLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrlr0::R](R) reader structure"]
impl crate::Readable for CTRLR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlr0::W](W) writer structure"]
impl crate::Writable for CTRLR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLR0 to value 0"]
impl crate::Resettable for CTRLR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
