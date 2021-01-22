#[doc = "Reader of register IC_ACK_GENERAL_CALL"]
pub type R = crate::R<u32, super::IC_ACK_GENERAL_CALL>;
#[doc = "Writer for register IC_ACK_GENERAL_CALL"]
pub type W = crate::W<u32, super::IC_ACK_GENERAL_CALL>;
#[doc = "Register IC_ACK_GENERAL_CALL `reset()`'s with value 0x01"]
impl crate::ResetValue for super::IC_ACK_GENERAL_CALL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACK_GEN_CALL_A {
    #[doc = "0: Generate NACK for a General Call"]
    DISABLED = 0,
    #[doc = "1: Generate ACK for a General Call"]
    ENABLED = 1,
}
impl From<ACK_GEN_CALL_A> for bool {
    #[inline(always)]
    fn from(variant: ACK_GEN_CALL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACK_GEN_CALL`"]
pub type ACK_GEN_CALL_R = crate::R<bool, ACK_GEN_CALL_A>;
impl ACK_GEN_CALL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACK_GEN_CALL_A {
        match self.bits {
            false => ACK_GEN_CALL_A::DISABLED,
            true => ACK_GEN_CALL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ACK_GEN_CALL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ACK_GEN_CALL_A::ENABLED
    }
}
#[doc = "Write proxy for field `ACK_GEN_CALL`"]
pub struct ACK_GEN_CALL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_GEN_CALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACK_GEN_CALL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Generate NACK for a General Call"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ACK_GEN_CALL_A::DISABLED)
    }
    #[doc = "Generate ACK for a General Call"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ACK_GEN_CALL_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe)."]
    #[inline(always)]
    pub fn ack_gen_call(&self) -> ACK_GEN_CALL_R {
        ACK_GEN_CALL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe)."]
    #[inline(always)]
    pub fn ack_gen_call(&mut self) -> ACK_GEN_CALL_W {
        ACK_GEN_CALL_W { w: self }
    }
}
