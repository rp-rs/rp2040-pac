#[doc = "Register `IC_ACK_GENERAL_CALL` reader"]
pub struct R(crate::R<IC_ACK_GENERAL_CALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_ACK_GENERAL_CALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_ACK_GENERAL_CALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_ACK_GENERAL_CALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC_ACK_GENERAL_CALL` writer"]
pub struct W(crate::W<IC_ACK_GENERAL_CALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_ACK_GENERAL_CALL_SPEC>;
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
impl From<crate::W<IC_ACK_GENERAL_CALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_ACK_GENERAL_CALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe).  

Value on reset: 1"]
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
#[doc = "Field `ACK_GEN_CALL` reader - ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe)."]
pub struct ACK_GEN_CALL_R(crate::FieldReader<bool, ACK_GEN_CALL_A>);
impl ACK_GEN_CALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ACK_GEN_CALL_R(crate::FieldReader::new(bits))
    }
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
        **self == ACK_GEN_CALL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ACK_GEN_CALL_A::ENABLED
    }
}
impl core::ops::Deref for ACK_GEN_CALL_R {
    type Target = crate::FieldReader<bool, ACK_GEN_CALL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACK_GEN_CALL` writer - ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe)."]
pub struct ACK_GEN_CALL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_GEN_CALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACK_GEN_CALL_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C ACK General Call Register  

 The register controls whether DW_apb_i2c responds with a ACK or NACK when it receives an I2C General Call address.  

 This register is applicable only when the DW_apb_i2c is in slave mode.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_ack_general_call](index.html) module"]
pub struct IC_ACK_GENERAL_CALL_SPEC;
impl crate::RegisterSpec for IC_ACK_GENERAL_CALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_ack_general_call::R](R) reader structure"]
impl crate::Readable for IC_ACK_GENERAL_CALL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic_ack_general_call::W](W) writer structure"]
impl crate::Writable for IC_ACK_GENERAL_CALL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IC_ACK_GENERAL_CALL to value 0x01"]
impl crate::Resettable for IC_ACK_GENERAL_CALL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
