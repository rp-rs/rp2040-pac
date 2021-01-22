#[doc = "Reader of register SPI_CTRLR0"]
pub type R = crate::R<u32, super::SPI_CTRLR0>;
#[doc = "Writer for register SPI_CTRLR0"]
pub type W = crate::W<u32, super::SPI_CTRLR0>;
#[doc = "Register SPI_CTRLR0 `reset()`'s with value 0x0300_0000"]
impl crate::ResetValue for super::SPI_CTRLR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0300_0000
    }
}
#[doc = "Reader of field `XIP_CMD`"]
pub type XIP_CMD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `XIP_CMD`"]
pub struct XIP_CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> XIP_CMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `SPI_RXDS_EN`"]
pub type SPI_RXDS_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_RXDS_EN`"]
pub struct SPI_RXDS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_RXDS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `INST_DDR_EN`"]
pub type INST_DDR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INST_DDR_EN`"]
pub struct INST_DDR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INST_DDR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `SPI_DDR_EN`"]
pub type SPI_DDR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_DDR_EN`"]
pub struct SPI_DDR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_DDR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `WAIT_CYCLES`"]
pub type WAIT_CYCLES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WAIT_CYCLES`"]
pub struct WAIT_CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 11)) | (((value as u32) & 0x1f) << 11);
        self.w
    }
}
#[doc = "Instruction length (0/4/8/16b)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `INST_L`"]
pub type INST_L_R = crate::R<u8, INST_L_A>;
impl INST_L_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INST_L_A {
        match self.bits {
            0 => INST_L_A::NONE,
            1 => INST_L_A::_4B,
            2 => INST_L_A::_8B,
            3 => INST_L_A::_16B,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == INST_L_A::NONE
    }
    #[doc = "Checks if the value of the field is `_4B`"]
    #[inline(always)]
    pub fn is_4b(&self) -> bool {
        *self == INST_L_A::_4B
    }
    #[doc = "Checks if the value of the field is `_8B`"]
    #[inline(always)]
    pub fn is_8b(&self) -> bool {
        *self == INST_L_A::_8B
    }
    #[doc = "Checks if the value of the field is `_16B`"]
    #[inline(always)]
    pub fn is_16b(&self) -> bool {
        *self == INST_L_A::_16B
    }
}
#[doc = "Write proxy for field `INST_L`"]
pub struct INST_L_W<'a> {
    w: &'a mut W,
}
impl<'a> INST_L_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INST_L_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No instruction"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(INST_L_A::NONE)
    }
    #[doc = "4-bit instruction"]
    #[inline(always)]
    pub fn _4b(self) -> &'a mut W {
        self.variant(INST_L_A::_4B)
    }
    #[doc = "8-bit instruction"]
    #[inline(always)]
    pub fn _8b(self) -> &'a mut W {
        self.variant(INST_L_A::_8B)
    }
    #[doc = "16-bit instruction"]
    #[inline(always)]
    pub fn _16b(self) -> &'a mut W {
        self.variant(INST_L_A::_16B)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `ADDR_L`"]
pub type ADDR_L_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDR_L`"]
pub struct ADDR_L_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 2)) | (((value as u32) & 0x0f) << 2);
        self.w
    }
}
#[doc = "Address and instruction transfer format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `TRANS_TYPE`"]
pub type TRANS_TYPE_R = crate::R<u8, TRANS_TYPE_A>;
impl TRANS_TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRANS_TYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRANS_TYPE_A::_1C1A),
            1 => Val(TRANS_TYPE_A::_1C2A),
            2 => Val(TRANS_TYPE_A::_2C2A),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1C1A`"]
    #[inline(always)]
    pub fn is_1c1a(&self) -> bool {
        *self == TRANS_TYPE_A::_1C1A
    }
    #[doc = "Checks if the value of the field is `_1C2A`"]
    #[inline(always)]
    pub fn is_1c2a(&self) -> bool {
        *self == TRANS_TYPE_A::_1C2A
    }
    #[doc = "Checks if the value of the field is `_2C2A`"]
    #[inline(always)]
    pub fn is_2c2a(&self) -> bool {
        *self == TRANS_TYPE_A::_2C2A
    }
}
#[doc = "Write proxy for field `TRANS_TYPE`"]
pub struct TRANS_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANS_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRANS_TYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Command and address both in standard SPI frame format"]
    #[inline(always)]
    pub fn _1c1a(self) -> &'a mut W {
        self.variant(TRANS_TYPE_A::_1C1A)
    }
    #[doc = "Command in standard SPI format, address in format specified by FRF"]
    #[inline(always)]
    pub fn _1c2a(self) -> &'a mut W {
        self.variant(TRANS_TYPE_A::_1C2A)
    }
    #[doc = "Command and address both in format specified by FRF (e.g. Dual-SPI)"]
    #[inline(always)]
    pub fn _2c2a(self) -> &'a mut W {
        self.variant(TRANS_TYPE_A::_2C2A)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - SPI Command to send in XIP mode (INST_L = 8-bit) or to append to Address (INST_L = 0-bit)"]
    #[inline(always)]
    pub fn xip_cmd(&self) -> XIP_CMD_R {
        XIP_CMD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 18 - Read data strobe enable"]
    #[inline(always)]
    pub fn spi_rxds_en(&self) -> SPI_RXDS_EN_R {
        SPI_RXDS_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Instruction DDR transfer enable"]
    #[inline(always)]
    pub fn inst_ddr_en(&self) -> INST_DDR_EN_R {
        INST_DDR_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SPI DDR transfer enable"]
    #[inline(always)]
    pub fn spi_ddr_en(&self) -> SPI_DDR_EN_R {
        SPI_DDR_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 11:15 - Wait cycles between control frame transmit and data reception (in SCLK cycles)"]
    #[inline(always)]
    pub fn wait_cycles(&self) -> WAIT_CYCLES_R {
        WAIT_CYCLES_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Instruction length (0/4/8/16b)"]
    #[inline(always)]
    pub fn inst_l(&self) -> INST_L_R {
        INST_L_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 2:5 - Address length (0b-60b in 4b increments)"]
    #[inline(always)]
    pub fn addr_l(&self) -> ADDR_L_R {
        ADDR_L_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 0:1 - Address and instruction transfer format"]
    #[inline(always)]
    pub fn trans_type(&self) -> TRANS_TYPE_R {
        TRANS_TYPE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - SPI Command to send in XIP mode (INST_L = 8-bit) or to append to Address (INST_L = 0-bit)"]
    #[inline(always)]
    pub fn xip_cmd(&mut self) -> XIP_CMD_W {
        XIP_CMD_W { w: self }
    }
    #[doc = "Bit 18 - Read data strobe enable"]
    #[inline(always)]
    pub fn spi_rxds_en(&mut self) -> SPI_RXDS_EN_W {
        SPI_RXDS_EN_W { w: self }
    }
    #[doc = "Bit 17 - Instruction DDR transfer enable"]
    #[inline(always)]
    pub fn inst_ddr_en(&mut self) -> INST_DDR_EN_W {
        INST_DDR_EN_W { w: self }
    }
    #[doc = "Bit 16 - SPI DDR transfer enable"]
    #[inline(always)]
    pub fn spi_ddr_en(&mut self) -> SPI_DDR_EN_W {
        SPI_DDR_EN_W { w: self }
    }
    #[doc = "Bits 11:15 - Wait cycles between control frame transmit and data reception (in SCLK cycles)"]
    #[inline(always)]
    pub fn wait_cycles(&mut self) -> WAIT_CYCLES_W {
        WAIT_CYCLES_W { w: self }
    }
    #[doc = "Bits 8:9 - Instruction length (0/4/8/16b)"]
    #[inline(always)]
    pub fn inst_l(&mut self) -> INST_L_W {
        INST_L_W { w: self }
    }
    #[doc = "Bits 2:5 - Address length (0b-60b in 4b increments)"]
    #[inline(always)]
    pub fn addr_l(&mut self) -> ADDR_L_W {
        ADDR_L_W { w: self }
    }
    #[doc = "Bits 0:1 - Address and instruction transfer format"]
    #[inline(always)]
    pub fn trans_type(&mut self) -> TRANS_TYPE_W {
        TRANS_TYPE_W { w: self }
    }
}
