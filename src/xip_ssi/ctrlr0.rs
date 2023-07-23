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
#[doc = "Field `DFS` reader - Data frame size"]
pub type DFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DFS` writer - Data frame size"]
pub type DFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `FRF` reader - Frame format"]
pub type FRF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRF` writer - Frame format"]
pub type FRF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLR0_SPEC, u8, u8, 2, O>;
#[doc = "Field `SCPH` reader - Serial clock phase"]
pub type SCPH_R = crate::BitReader<bool>;
#[doc = "Field `SCPH` writer - Serial clock phase"]
pub type SCPH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLR0_SPEC, bool, O>;
#[doc = "Field `SCPOL` reader - Serial clock polarity"]
pub type SCPOL_R = crate::BitReader<bool>;
#[doc = "Field `SCPOL` writer - Serial clock polarity"]
pub type SCPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLR0_SPEC, bool, O>;
#[doc = "Field `TMOD` reader - Transfer mode"]
pub type TMOD_R = crate::FieldReader<u8, TMOD_A>;
#[doc = "Transfer mode  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl TMOD_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `TMOD` writer - Transfer mode"]
pub type TMOD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CTRLR0_SPEC, u8, TMOD_A, 2, O>;
impl<'a, const O: u8> TMOD_W<'a, O> {
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
}
#[doc = "Field `SLV_OE` reader - Slave output enable"]
pub type SLV_OE_R = crate::BitReader<bool>;
#[doc = "Field `SLV_OE` writer - Slave output enable"]
pub type SLV_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLR0_SPEC, bool, O>;
#[doc = "Field `SRL` reader - Shift register loop (test mode)"]
pub type SRL_R = crate::BitReader<bool>;
#[doc = "Field `SRL` writer - Shift register loop (test mode)"]
pub type SRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLR0_SPEC, bool, O>;
#[doc = "Field `CFS` reader - Control frame size  
 Value of n -> n+1 clocks per frame."]
pub type CFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CFS` writer - Control frame size  
 Value of n -> n+1 clocks per frame."]
pub type CFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `DFS_32` reader - Data frame size in 32b transfer mode  
 Value of n -> n+1 clocks per frame."]
pub type DFS_32_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DFS_32` writer - Data frame size in 32b transfer mode  
 Value of n -> n+1 clocks per frame."]
pub type DFS_32_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLR0_SPEC, u8, u8, 5, O>;
#[doc = "Field `SPI_FRF` reader - SPI frame format"]
pub type SPI_FRF_R = crate::FieldReader<u8, SPI_FRF_A>;
#[doc = "SPI frame format  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl SPI_FRF_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `SPI_FRF` writer - SPI frame format"]
pub type SPI_FRF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLR0_SPEC, u8, SPI_FRF_A, 2, O>;
impl<'a, const O: u8> SPI_FRF_W<'a, O> {
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
}
#[doc = "Field `SSTE` reader - Slave select toggle enable"]
pub type SSTE_R = crate::BitReader<bool>;
#[doc = "Field `SSTE` writer - Slave select toggle enable"]
pub type SSTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLR0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Data frame size"]
    #[inline(always)]
    pub fn dfs(&self) -> DFS_R {
        DFS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Frame format"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Serial clock phase"]
    #[inline(always)]
    pub fn scph(&self) -> SCPH_R {
        SCPH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Serial clock polarity"]
    #[inline(always)]
    pub fn scpol(&self) -> SCPOL_R {
        SCPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Transfer mode"]
    #[inline(always)]
    pub fn tmod(&self) -> TMOD_R {
        TMOD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Slave output enable"]
    #[inline(always)]
    pub fn slv_oe(&self) -> SLV_OE_R {
        SLV_OE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Shift register loop (test mode)"]
    #[inline(always)]
    pub fn srl(&self) -> SRL_R {
        SRL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Control frame size  
 Value of n -> n+1 clocks per frame."]
    #[inline(always)]
    pub fn cfs(&self) -> CFS_R {
        CFS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Data frame size in 32b transfer mode  
 Value of n -> n+1 clocks per frame."]
    #[inline(always)]
    pub fn dfs_32(&self) -> DFS_32_R {
        DFS_32_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - SPI frame format"]
    #[inline(always)]
    pub fn spi_frf(&self) -> SPI_FRF_R {
        SPI_FRF_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - Slave select toggle enable"]
    #[inline(always)]
    pub fn sste(&self) -> SSTE_R {
        SSTE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data frame size"]
    #[inline(always)]
    #[must_use]
    pub fn dfs(&mut self) -> DFS_W<0> {
        DFS_W::new(self)
    }
    #[doc = "Bits 4:5 - Frame format"]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FRF_W<4> {
        FRF_W::new(self)
    }
    #[doc = "Bit 6 - Serial clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn scph(&mut self) -> SCPH_W<6> {
        SCPH_W::new(self)
    }
    #[doc = "Bit 7 - Serial clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn scpol(&mut self) -> SCPOL_W<7> {
        SCPOL_W::new(self)
    }
    #[doc = "Bits 8:9 - Transfer mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmod(&mut self) -> TMOD_W<8> {
        TMOD_W::new(self)
    }
    #[doc = "Bit 10 - Slave output enable"]
    #[inline(always)]
    #[must_use]
    pub fn slv_oe(&mut self) -> SLV_OE_W<10> {
        SLV_OE_W::new(self)
    }
    #[doc = "Bit 11 - Shift register loop (test mode)"]
    #[inline(always)]
    #[must_use]
    pub fn srl(&mut self) -> SRL_W<11> {
        SRL_W::new(self)
    }
    #[doc = "Bits 12:15 - Control frame size  
 Value of n -> n+1 clocks per frame."]
    #[inline(always)]
    #[must_use]
    pub fn cfs(&mut self) -> CFS_W<12> {
        CFS_W::new(self)
    }
    #[doc = "Bits 16:20 - Data frame size in 32b transfer mode  
 Value of n -> n+1 clocks per frame."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_32(&mut self) -> DFS_32_W<16> {
        DFS_32_W::new(self)
    }
    #[doc = "Bits 21:22 - SPI frame format"]
    #[inline(always)]
    #[must_use]
    pub fn spi_frf(&mut self) -> SPI_FRF_W<21> {
        SPI_FRF_W::new(self)
    }
    #[doc = "Bit 24 - Slave select toggle enable"]
    #[inline(always)]
    #[must_use]
    pub fn sste(&mut self) -> SSTE_W<24> {
        SSTE_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLR0 to value 0"]
impl crate::Resettable for CTRLR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
