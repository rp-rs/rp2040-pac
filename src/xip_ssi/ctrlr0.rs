#[doc = "Register `CTRLR0` reader"]
pub type R = crate::R<CTRLR0_SPEC>;
#[doc = "Register `CTRLR0` writer"]
pub type W = crate::W<CTRLR0_SPEC>;
#[doc = "Field `DFS` reader - Data frame size"]
pub type DFS_R = crate::FieldReader;
#[doc = "Field `DFS` writer - Data frame size"]
pub type DFS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FRF` reader - Frame format"]
pub type FRF_R = crate::FieldReader;
#[doc = "Field `FRF` writer - Frame format"]
pub type FRF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCPH` reader - Serial clock phase"]
pub type SCPH_R = crate::BitReader;
#[doc = "Field `SCPH` writer - Serial clock phase"]
pub type SCPH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCPOL` reader - Serial clock polarity"]
pub type SCPOL_R = crate::BitReader;
#[doc = "Field `SCPOL` writer - Serial clock polarity"]
pub type SCPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl crate::FieldSpec for TMOD_A {
    type Ux = u8;
}
impl crate::IsEnum for TMOD_A {}
#[doc = "Field `TMOD` reader - Transfer mode"]
pub type TMOD_R = crate::FieldReader<TMOD_A>;
impl TMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TMOD_A {
        match self.bits {
            0 => TMOD_A::TX_AND_RX,
            1 => TMOD_A::TX_ONLY,
            2 => TMOD_A::RX_ONLY,
            3 => TMOD_A::EEPROM_READ,
            _ => unreachable!(),
        }
    }
    #[doc = "Both transmit and receive"]
    #[inline(always)]
    pub fn is_tx_and_rx(&self) -> bool {
        *self == TMOD_A::TX_AND_RX
    }
    #[doc = "Transmit only (not for FRF == 0, standard SPI mode)"]
    #[inline(always)]
    pub fn is_tx_only(&self) -> bool {
        *self == TMOD_A::TX_ONLY
    }
    #[doc = "Receive only (not for FRF == 0, standard SPI mode)"]
    #[inline(always)]
    pub fn is_rx_only(&self) -> bool {
        *self == TMOD_A::RX_ONLY
    }
    #[doc = "EEPROM read mode (TX then RX; RX starts after control data TX'd)"]
    #[inline(always)]
    pub fn is_eeprom_read(&self) -> bool {
        *self == TMOD_A::EEPROM_READ
    }
}
#[doc = "Field `TMOD` writer - Transfer mode"]
pub type TMOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TMOD_A, crate::Safe>;
impl<'a, REG> TMOD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Both transmit and receive"]
    #[inline(always)]
    pub fn tx_and_rx(self) -> &'a mut crate::W<REG> {
        self.variant(TMOD_A::TX_AND_RX)
    }
    #[doc = "Transmit only (not for FRF == 0, standard SPI mode)"]
    #[inline(always)]
    pub fn tx_only(self) -> &'a mut crate::W<REG> {
        self.variant(TMOD_A::TX_ONLY)
    }
    #[doc = "Receive only (not for FRF == 0, standard SPI mode)"]
    #[inline(always)]
    pub fn rx_only(self) -> &'a mut crate::W<REG> {
        self.variant(TMOD_A::RX_ONLY)
    }
    #[doc = "EEPROM read mode (TX then RX; RX starts after control data TX'd)"]
    #[inline(always)]
    pub fn eeprom_read(self) -> &'a mut crate::W<REG> {
        self.variant(TMOD_A::EEPROM_READ)
    }
}
#[doc = "Field `SLV_OE` reader - Slave output enable"]
pub type SLV_OE_R = crate::BitReader;
#[doc = "Field `SLV_OE` writer - Slave output enable"]
pub type SLV_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRL` reader - Shift register loop (test mode)"]
pub type SRL_R = crate::BitReader;
#[doc = "Field `SRL` writer - Shift register loop (test mode)"]
pub type SRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFS` reader - Control frame size  
 Value of n -> n+1 clocks per frame."]
pub type CFS_R = crate::FieldReader;
#[doc = "Field `CFS` writer - Control frame size  
 Value of n -> n+1 clocks per frame."]
pub type CFS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DFS_32` reader - Data frame size in 32b transfer mode  
 Value of n -> n+1 clocks per frame."]
pub type DFS_32_R = crate::FieldReader;
#[doc = "Field `DFS_32` writer - Data frame size in 32b transfer mode  
 Value of n -> n+1 clocks per frame."]
pub type DFS_32_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
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
impl crate::FieldSpec for SPI_FRF_A {
    type Ux = u8;
}
impl crate::IsEnum for SPI_FRF_A {}
#[doc = "Field `SPI_FRF` reader - SPI frame format"]
pub type SPI_FRF_R = crate::FieldReader<SPI_FRF_A>;
impl SPI_FRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SPI_FRF_A> {
        match self.bits {
            0 => Some(SPI_FRF_A::STD),
            1 => Some(SPI_FRF_A::DUAL),
            2 => Some(SPI_FRF_A::QUAD),
            _ => None,
        }
    }
    #[doc = "Standard 1-bit SPI frame format; 1 bit per SCK, full-duplex"]
    #[inline(always)]
    pub fn is_std(&self) -> bool {
        *self == SPI_FRF_A::STD
    }
    #[doc = "Dual-SPI frame format; two bits per SCK, half-duplex"]
    #[inline(always)]
    pub fn is_dual(&self) -> bool {
        *self == SPI_FRF_A::DUAL
    }
    #[doc = "Quad-SPI frame format; four bits per SCK, half-duplex"]
    #[inline(always)]
    pub fn is_quad(&self) -> bool {
        *self == SPI_FRF_A::QUAD
    }
}
#[doc = "Field `SPI_FRF` writer - SPI frame format"]
pub type SPI_FRF_W<'a, REG> = crate::FieldWriter<'a, REG, 2, SPI_FRF_A>;
impl<'a, REG> SPI_FRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Standard 1-bit SPI frame format; 1 bit per SCK, full-duplex"]
    #[inline(always)]
    pub fn std(self) -> &'a mut crate::W<REG> {
        self.variant(SPI_FRF_A::STD)
    }
    #[doc = "Dual-SPI frame format; two bits per SCK, half-duplex"]
    #[inline(always)]
    pub fn dual(self) -> &'a mut crate::W<REG> {
        self.variant(SPI_FRF_A::DUAL)
    }
    #[doc = "Quad-SPI frame format; four bits per SCK, half-duplex"]
    #[inline(always)]
    pub fn quad(self) -> &'a mut crate::W<REG> {
        self.variant(SPI_FRF_A::QUAD)
    }
}
#[doc = "Field `SSTE` reader - Slave select toggle enable"]
pub type SSTE_R = crate::BitReader;
#[doc = "Field `SSTE` writer - Slave select toggle enable"]
pub type SSTE_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn dfs(&mut self) -> DFS_W<CTRLR0_SPEC> {
        DFS_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Frame format"]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FRF_W<CTRLR0_SPEC> {
        FRF_W::new(self, 4)
    }
    #[doc = "Bit 6 - Serial clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn scph(&mut self) -> SCPH_W<CTRLR0_SPEC> {
        SCPH_W::new(self, 6)
    }
    #[doc = "Bit 7 - Serial clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn scpol(&mut self) -> SCPOL_W<CTRLR0_SPEC> {
        SCPOL_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Transfer mode"]
    #[inline(always)]
    #[must_use]
    pub fn tmod(&mut self) -> TMOD_W<CTRLR0_SPEC> {
        TMOD_W::new(self, 8)
    }
    #[doc = "Bit 10 - Slave output enable"]
    #[inline(always)]
    #[must_use]
    pub fn slv_oe(&mut self) -> SLV_OE_W<CTRLR0_SPEC> {
        SLV_OE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Shift register loop (test mode)"]
    #[inline(always)]
    #[must_use]
    pub fn srl(&mut self) -> SRL_W<CTRLR0_SPEC> {
        SRL_W::new(self, 11)
    }
    #[doc = "Bits 12:15 - Control frame size  
 Value of n -> n+1 clocks per frame."]
    #[inline(always)]
    #[must_use]
    pub fn cfs(&mut self) -> CFS_W<CTRLR0_SPEC> {
        CFS_W::new(self, 12)
    }
    #[doc = "Bits 16:20 - Data frame size in 32b transfer mode  
 Value of n -> n+1 clocks per frame."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_32(&mut self) -> DFS_32_W<CTRLR0_SPEC> {
        DFS_32_W::new(self, 16)
    }
    #[doc = "Bits 21:22 - SPI frame format"]
    #[inline(always)]
    #[must_use]
    pub fn spi_frf(&mut self) -> SPI_FRF_W<CTRLR0_SPEC> {
        SPI_FRF_W::new(self, 21)
    }
    #[doc = "Bit 24 - Slave select toggle enable"]
    #[inline(always)]
    #[must_use]
    pub fn sste(&mut self) -> SSTE_W<CTRLR0_SPEC> {
        SSTE_W::new(self, 24)
    }
}
#[doc = "Control register 0  

You can [`read`](crate::Reg::read) this register and get [`ctrlr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLR0_SPEC;
impl crate::RegisterSpec for CTRLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlr0::R`](R) reader structure"]
impl crate::Readable for CTRLR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlr0::W`](W) writer structure"]
impl crate::Writable for CTRLR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLR0 to value 0"]
impl crate::Resettable for CTRLR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
