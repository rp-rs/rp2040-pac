#[doc = "Reader of register IC_TAR"]
pub type R = crate::R<u32, super::IC_TAR>;
#[doc = "Writer for register IC_TAR"]
pub type W = crate::W<u32, super::IC_TAR>;
#[doc = "Register IC_TAR `reset()`'s with value 0x55"]
impl crate::ResetValue for super::IC_TAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x55
    }
}
#[doc = "This bit indicates whether software performs a Device-ID or General Call or START BYTE command. - 0: ignore bit 10 GC_OR_START and use IC_TAR normally - 1: perform special I2C command as specified in Device_ID or GC_OR_START bit Reset value: 0x0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPECIAL_A {
    #[doc = "0: Disables programming of GENERAL_CALL or START_BYTE transmission"]
    DISABLED = 0,
    #[doc = "1: Enables programming of GENERAL_CALL or START_BYTE transmission"]
    ENABLED = 1,
}
impl From<SPECIAL_A> for bool {
    #[inline(always)]
    fn from(variant: SPECIAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPECIAL`"]
pub type SPECIAL_R = crate::R<bool, SPECIAL_A>;
impl SPECIAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPECIAL_A {
        match self.bits {
            false => SPECIAL_A::DISABLED,
            true => SPECIAL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPECIAL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPECIAL_A::ENABLED
    }
}
#[doc = "Write proxy for field `SPECIAL`"]
pub struct SPECIAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPECIAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPECIAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disables programming of GENERAL_CALL or START_BYTE transmission"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPECIAL_A::DISABLED)
    }
    #[doc = "Enables programming of GENERAL_CALL or START_BYTE transmission"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPECIAL_A::ENABLED)
    }
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
#[doc = "If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then this bit indicates whether a General Call or START byte command is to be performed by the DW_apb_i2c. - 0: General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The DW_apb_i2c remains in General Call mode until the SPECIAL bit value (bit 11) is cleared. - 1: START BYTE Reset value: 0x0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GC_OR_START_A {
    #[doc = "0: GENERAL_CALL byte transmission"]
    GENERAL_CALL = 0,
    #[doc = "1: START byte transmission"]
    START_BYTE = 1,
}
impl From<GC_OR_START_A> for bool {
    #[inline(always)]
    fn from(variant: GC_OR_START_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `GC_OR_START`"]
pub type GC_OR_START_R = crate::R<bool, GC_OR_START_A>;
impl GC_OR_START_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GC_OR_START_A {
        match self.bits {
            false => GC_OR_START_A::GENERAL_CALL,
            true => GC_OR_START_A::START_BYTE,
        }
    }
    #[doc = "Checks if the value of the field is `GENERAL_CALL`"]
    #[inline(always)]
    pub fn is_general_call(&self) -> bool {
        *self == GC_OR_START_A::GENERAL_CALL
    }
    #[doc = "Checks if the value of the field is `START_BYTE`"]
    #[inline(always)]
    pub fn is_start_byte(&self) -> bool {
        *self == GC_OR_START_A::START_BYTE
    }
}
#[doc = "Write proxy for field `GC_OR_START`"]
pub struct GC_OR_START_W<'a> {
    w: &'a mut W,
}
impl<'a> GC_OR_START_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GC_OR_START_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "GENERAL_CALL byte transmission"]
    #[inline(always)]
    pub fn general_call(self) -> &'a mut W {
        self.variant(GC_OR_START_A::GENERAL_CALL)
    }
    #[doc = "START byte transmission"]
    #[inline(always)]
    pub fn start_byte(self) -> &'a mut W {
        self.variant(GC_OR_START_A::START_BYTE)
    }
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
#[doc = "Reader of field `IC_TAR`"]
pub type IC_TAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IC_TAR`"]
pub struct IC_TAR_W<'a> {
    w: &'a mut W,
}
impl<'a> IC_TAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - This bit indicates whether software performs a Device-ID or General Call or START BYTE command. - 0: ignore bit 10 GC_OR_START and use IC_TAR normally - 1: perform special I2C command as specified in Device_ID or GC_OR_START bit Reset value: 0x0"]
    #[inline(always)]
    pub fn special(&self) -> SPECIAL_R {
        SPECIAL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then this bit indicates whether a General Call or START byte command is to be performed by the DW_apb_i2c. - 0: General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The DW_apb_i2c remains in General Call mode until the SPECIAL bit value (bit 11) is cleared. - 1: START BYTE Reset value: 0x0"]
    #[inline(always)]
    pub fn gc_or_start(&self) -> GC_OR_START_R {
        GC_OR_START_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 0:9 - This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits.\\n\\n If the IC_TAR and IC_SAR are the same, loopback exists but the FIFOs are shared between master and slave, so full loopback is not feasible. Only one direction loopback mode is supported (simplex), not duplex. A master cannot transmit to itself; it can transmit to only a slave."]
    #[inline(always)]
    pub fn ic_tar(&self) -> IC_TAR_R {
        IC_TAR_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 11 - This bit indicates whether software performs a Device-ID or General Call or START BYTE command. - 0: ignore bit 10 GC_OR_START and use IC_TAR normally - 1: perform special I2C command as specified in Device_ID or GC_OR_START bit Reset value: 0x0"]
    #[inline(always)]
    pub fn special(&mut self) -> SPECIAL_W {
        SPECIAL_W { w: self }
    }
    #[doc = "Bit 10 - If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then this bit indicates whether a General Call or START byte command is to be performed by the DW_apb_i2c. - 0: General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The DW_apb_i2c remains in General Call mode until the SPECIAL bit value (bit 11) is cleared. - 1: START BYTE Reset value: 0x0"]
    #[inline(always)]
    pub fn gc_or_start(&mut self) -> GC_OR_START_W {
        GC_OR_START_W { w: self }
    }
    #[doc = "Bits 0:9 - This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits.\\n\\n If the IC_TAR and IC_SAR are the same, loopback exists but the FIFOs are shared between master and slave, so full loopback is not feasible. Only one direction loopback mode is supported (simplex), not duplex. A master cannot transmit to itself; it can transmit to only a slave."]
    #[inline(always)]
    pub fn ic_tar(&mut self) -> IC_TAR_W {
        IC_TAR_W { w: self }
    }
}
