#[doc = "Reader of register CTRLR0"]
pub type R = crate::R<u32, super::CTRLR0>;
#[doc = "Writer for register CTRLR0"]
pub type W = crate::W<u32, super::CTRLR0>;
#[doc = "Register CTRLR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSTE`"]
pub type SSTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSTE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "SPI frame format\n\nValue on reset: 0"]
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
#[doc = "Reader of field `SPI_FRF`"]
pub type SPI_FRF_R = crate::R<u8, SPI_FRF_A>;
impl SPI_FRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPI_FRF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SPI_FRF_A::STD),
            1 => Val(SPI_FRF_A::DUAL),
            2 => Val(SPI_FRF_A::QUAD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `STD`"]
    #[inline(always)]
    pub fn is_std(&self) -> bool {
        *self == SPI_FRF_A::STD
    }
    #[doc = "Checks if the value of the field is `DUAL`"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == SPI_FRF_A::DUAL
    }
    #[doc = "Checks if the value of the field is `QUAD`"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == SPI_FRF_A::QUAD
    }
}
#[doc = "Write proxy for field `SPI_FRF`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `DFS_32`"]
pub type DFS_32_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DFS_32`"]
pub struct DFS_32_W<'a> {
    w: &'a mut W,
}
impl<'a> DFS_32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `CFS`"]
pub type CFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFS`"]
pub struct CFS_W<'a> {
    w: &'a mut W,
}
impl<'a> CFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `SRL`"]
pub type SRL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRL`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SLV_OE`"]
pub type SLV_OE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLV_OE`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Transfer mode\n\nValue on reset: 0"]
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
#[doc = "Reader of field `TMOD`"]
pub type TMOD_R = crate::R<u8, TMOD_A>;
impl TMOD_R {
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
        *self == TMOD_A::TX_AND_RX
    }
    #[doc = "Checks if the value of the field is `TX_ONLY`"]
    #[inline(always)]
    pub fn is_tx_only(&self) -> bool {
        *self == TMOD_A::TX_ONLY
    }
    #[doc = "Checks if the value of the field is `RX_ONLY`"]
    #[inline(always)]
    pub fn is_rx_only(&self) -> bool {
        *self == TMOD_A::RX_ONLY
    }
    #[doc = "Checks if the value of the field is `EEPROM_READ`"]
    #[inline(always)]
    pub fn is_eeprom_read(&self) -> bool {
        *self == TMOD_A::EEPROM_READ
    }
}
#[doc = "Write proxy for field `TMOD`"]
pub struct TMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TMOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMOD_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `SCPOL`"]
pub type SCPOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCPOL`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SCPH`"]
pub type SCPH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCPH`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `FRF`"]
pub type FRF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FRF`"]
pub struct FRF_W<'a> {
    w: &'a mut W,
}
impl<'a> FRF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `DFS`"]
pub type DFS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DFS`"]
pub struct DFS_W<'a> {
    w: &'a mut W,
}
impl<'a> DFS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
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
    #[doc = "Bits 16:20 - Data frame size in 32b transfer mode\\n Value of n -> n+1 clocks per frame."]
    #[inline(always)]
    pub fn dfs_32(&self) -> DFS_32_R {
        DFS_32_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - Control frame size\\n Value of n -> n+1 clocks per frame."]
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
    #[doc = "Bits 16:20 - Data frame size in 32b transfer mode\\n Value of n -> n+1 clocks per frame."]
    #[inline(always)]
    pub fn dfs_32(&mut self) -> DFS_32_W {
        DFS_32_W { w: self }
    }
    #[doc = "Bits 12:15 - Control frame size\\n Value of n -> n+1 clocks per frame."]
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
}
