#[doc = "Register `SPI_CTRLR0` reader"]
pub type R = crate::R<SPI_CTRLR0_SPEC>;
#[doc = "Register `SPI_CTRLR0` writer"]
pub type W = crate::W<SPI_CTRLR0_SPEC>;
#[doc = "Field `TRANS_TYPE` reader - Address and instruction transfer format"]
pub type TRANS_TYPE_R = crate::FieldReader<TRANS_TYPE_A>;
#[doc = "Address and instruction transfer format  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRANS_TYPE_A {
    #[doc = "0: Command and address both in standard SPI frame format"]
    _1C1A = 0,
    #[doc = "1: Command in standard SPI format, address in format specified by FRF"]
    _1C2A = 1,
    #[doc = "2: Command and address both in format specified by FRF (e.g. Dual-SPI)"]
    _2C2A = 2,
}
impl From<TRANS_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: TRANS_TYPE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TRANS_TYPE_A {
    type Ux = u8;
}
impl TRANS_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TRANS_TYPE_A> {
        match self.bits {
            0 => Some(TRANS_TYPE_A::_1C1A),
            1 => Some(TRANS_TYPE_A::_1C2A),
            2 => Some(TRANS_TYPE_A::_2C2A),
            _ => None,
        }
    }
    #[doc = "Command and address both in standard SPI frame format"]
    #[inline(always)]
    pub fn is_1c1a(&self) -> bool {
        *self == TRANS_TYPE_A::_1C1A
    }
    #[doc = "Command in standard SPI format, address in format specified by FRF"]
    #[inline(always)]
    pub fn is_1c2a(&self) -> bool {
        *self == TRANS_TYPE_A::_1C2A
    }
    #[doc = "Command and address both in format specified by FRF (e.g. Dual-SPI)"]
    #[inline(always)]
    pub fn is_2c2a(&self) -> bool {
        *self == TRANS_TYPE_A::_2C2A
    }
}
#[doc = "Field `TRANS_TYPE` writer - Address and instruction transfer format"]
pub type TRANS_TYPE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, TRANS_TYPE_A>;
impl<'a, REG, const O: u8> TRANS_TYPE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Command and address both in standard SPI frame format"]
    #[inline(always)]
    pub fn _1c1a(self) -> &'a mut crate::W<REG> {
        self.variant(TRANS_TYPE_A::_1C1A)
    }
    #[doc = "Command in standard SPI format, address in format specified by FRF"]
    #[inline(always)]
    pub fn _1c2a(self) -> &'a mut crate::W<REG> {
        self.variant(TRANS_TYPE_A::_1C2A)
    }
    #[doc = "Command and address both in format specified by FRF (e.g. Dual-SPI)"]
    #[inline(always)]
    pub fn _2c2a(self) -> &'a mut crate::W<REG> {
        self.variant(TRANS_TYPE_A::_2C2A)
    }
}
#[doc = "Field `ADDR_L` reader - Address length (0b-60b in 4b increments)"]
pub type ADDR_L_R = crate::FieldReader;
#[doc = "Field `ADDR_L` writer - Address length (0b-60b in 4b increments)"]
pub type ADDR_L_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `INST_L` reader - Instruction length (0/4/8/16b)"]
pub type INST_L_R = crate::FieldReader<INST_L_A>;
#[doc = "Instruction length (0/4/8/16b)  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INST_L_A {
    #[doc = "0: No instruction"]
    NONE = 0,
    #[doc = "1: 4-bit instruction"]
    _4B = 1,
    #[doc = "2: 8-bit instruction"]
    _8B = 2,
    #[doc = "3: 16-bit instruction"]
    _16B = 3,
}
impl From<INST_L_A> for u8 {
    #[inline(always)]
    fn from(variant: INST_L_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for INST_L_A {
    type Ux = u8;
}
impl INST_L_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> INST_L_A {
        match self.bits {
            0 => INST_L_A::NONE,
            1 => INST_L_A::_4B,
            2 => INST_L_A::_8B,
            3 => INST_L_A::_16B,
            _ => unreachable!(),
        }
    }
    #[doc = "No instruction"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == INST_L_A::NONE
    }
    #[doc = "4-bit instruction"]
    #[inline(always)]
    pub fn is_4b(&self) -> bool {
        *self == INST_L_A::_4B
    }
    #[doc = "8-bit instruction"]
    #[inline(always)]
    pub fn is_8b(&self) -> bool {
        *self == INST_L_A::_8B
    }
    #[doc = "16-bit instruction"]
    #[inline(always)]
    pub fn is_16b(&self) -> bool {
        *self == INST_L_A::_16B
    }
}
#[doc = "Field `INST_L` writer - Instruction length (0/4/8/16b)"]
pub type INST_L_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, INST_L_A>;
impl<'a, REG, const O: u8> INST_L_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No instruction"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(INST_L_A::NONE)
    }
    #[doc = "4-bit instruction"]
    #[inline(always)]
    pub fn _4b(self) -> &'a mut crate::W<REG> {
        self.variant(INST_L_A::_4B)
    }
    #[doc = "8-bit instruction"]
    #[inline(always)]
    pub fn _8b(self) -> &'a mut crate::W<REG> {
        self.variant(INST_L_A::_8B)
    }
    #[doc = "16-bit instruction"]
    #[inline(always)]
    pub fn _16b(self) -> &'a mut crate::W<REG> {
        self.variant(INST_L_A::_16B)
    }
}
#[doc = "Field `WAIT_CYCLES` reader - Wait cycles between control frame transmit and data reception (in SCLK cycles)"]
pub type WAIT_CYCLES_R = crate::FieldReader;
#[doc = "Field `WAIT_CYCLES` writer - Wait cycles between control frame transmit and data reception (in SCLK cycles)"]
pub type WAIT_CYCLES_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `SPI_DDR_EN` reader - SPI DDR transfer enable"]
pub type SPI_DDR_EN_R = crate::BitReader;
#[doc = "Field `SPI_DDR_EN` writer - SPI DDR transfer enable"]
pub type SPI_DDR_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INST_DDR_EN` reader - Instruction DDR transfer enable"]
pub type INST_DDR_EN_R = crate::BitReader;
#[doc = "Field `INST_DDR_EN` writer - Instruction DDR transfer enable"]
pub type INST_DDR_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SPI_RXDS_EN` reader - Read data strobe enable"]
pub type SPI_RXDS_EN_R = crate::BitReader;
#[doc = "Field `SPI_RXDS_EN` writer - Read data strobe enable"]
pub type SPI_RXDS_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `XIP_CMD` reader - SPI Command to send in XIP mode (INST_L = 8-bit) or to append to Address (INST_L = 0-bit)"]
pub type XIP_CMD_R = crate::FieldReader;
#[doc = "Field `XIP_CMD` writer - SPI Command to send in XIP mode (INST_L = 8-bit) or to append to Address (INST_L = 0-bit)"]
pub type XIP_CMD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:1 - Address and instruction transfer format"]
    #[inline(always)]
    pub fn trans_type(&self) -> TRANS_TYPE_R {
        TRANS_TYPE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - Address length (0b-60b in 4b increments)"]
    #[inline(always)]
    pub fn addr_l(&self) -> ADDR_L_R {
        ADDR_L_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Instruction length (0/4/8/16b)"]
    #[inline(always)]
    pub fn inst_l(&self) -> INST_L_R {
        INST_L_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 11:15 - Wait cycles between control frame transmit and data reception (in SCLK cycles)"]
    #[inline(always)]
    pub fn wait_cycles(&self) -> WAIT_CYCLES_R {
        WAIT_CYCLES_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - SPI DDR transfer enable"]
    #[inline(always)]
    pub fn spi_ddr_en(&self) -> SPI_DDR_EN_R {
        SPI_DDR_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Instruction DDR transfer enable"]
    #[inline(always)]
    pub fn inst_ddr_en(&self) -> INST_DDR_EN_R {
        INST_DDR_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Read data strobe enable"]
    #[inline(always)]
    pub fn spi_rxds_en(&self) -> SPI_RXDS_EN_R {
        SPI_RXDS_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 24:31 - SPI Command to send in XIP mode (INST_L = 8-bit) or to append to Address (INST_L = 0-bit)"]
    #[inline(always)]
    pub fn xip_cmd(&self) -> XIP_CMD_R {
        XIP_CMD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Address and instruction transfer format"]
    #[inline(always)]
    #[must_use]
    pub fn trans_type(&mut self) -> TRANS_TYPE_W<SPI_CTRLR0_SPEC, 0> {
        TRANS_TYPE_W::new(self)
    }
    #[doc = "Bits 2:5 - Address length (0b-60b in 4b increments)"]
    #[inline(always)]
    #[must_use]
    pub fn addr_l(&mut self) -> ADDR_L_W<SPI_CTRLR0_SPEC, 2> {
        ADDR_L_W::new(self)
    }
    #[doc = "Bits 8:9 - Instruction length (0/4/8/16b)"]
    #[inline(always)]
    #[must_use]
    pub fn inst_l(&mut self) -> INST_L_W<SPI_CTRLR0_SPEC, 8> {
        INST_L_W::new(self)
    }
    #[doc = "Bits 11:15 - Wait cycles between control frame transmit and data reception (in SCLK cycles)"]
    #[inline(always)]
    #[must_use]
    pub fn wait_cycles(&mut self) -> WAIT_CYCLES_W<SPI_CTRLR0_SPEC, 11> {
        WAIT_CYCLES_W::new(self)
    }
    #[doc = "Bit 16 - SPI DDR transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi_ddr_en(&mut self) -> SPI_DDR_EN_W<SPI_CTRLR0_SPEC, 16> {
        SPI_DDR_EN_W::new(self)
    }
    #[doc = "Bit 17 - Instruction DDR transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn inst_ddr_en(&mut self) -> INST_DDR_EN_W<SPI_CTRLR0_SPEC, 17> {
        INST_DDR_EN_W::new(self)
    }
    #[doc = "Bit 18 - Read data strobe enable"]
    #[inline(always)]
    #[must_use]
    pub fn spi_rxds_en(&mut self) -> SPI_RXDS_EN_W<SPI_CTRLR0_SPEC, 18> {
        SPI_RXDS_EN_W::new(self)
    }
    #[doc = "Bits 24:31 - SPI Command to send in XIP mode (INST_L = 8-bit) or to append to Address (INST_L = 0-bit)"]
    #[inline(always)]
    #[must_use]
    pub fn xip_cmd(&mut self) -> XIP_CMD_W<SPI_CTRLR0_SPEC, 24> {
        XIP_CMD_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SPI control  

You can [`read`](crate::generic::Reg::read) this register and get [`spi_ctrlr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi_ctrlr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_CTRLR0_SPEC;
impl crate::RegisterSpec for SPI_CTRLR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_ctrlr0::R`](R) reader structure"]
impl crate::Readable for SPI_CTRLR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_ctrlr0::W`](W) writer structure"]
impl crate::Writable for SPI_CTRLR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_CTRLR0 to value 0x0300_0000"]
impl crate::Resettable for SPI_CTRLR0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0300_0000;
}
