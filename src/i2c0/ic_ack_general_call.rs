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
#[doc = "Field `ACK_GEN_CALL` reader - ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe)."]
pub type ACK_GEN_CALL_R = crate::BitReader<ACK_GEN_CALL_A>;
#[doc = "ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe).  

Value on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
impl ACK_GEN_CALL_R {
    #[doc = "Get enumerated values variant"]
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
#[doc = "Field `ACK_GEN_CALL` writer - ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe)."]
pub type ACK_GEN_CALL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, IC_ACK_GENERAL_CALL_SPEC, ACK_GEN_CALL_A, O>;
impl<'a, const O: u8> ACK_GEN_CALL_W<'a, O> {
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
}
impl R {
    #[doc = "Bit 0 - ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe)."]
    #[inline(always)]
    pub fn ack_gen_call(&self) -> ACK_GEN_CALL_R {
        ACK_GEN_CALL_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe)."]
    #[inline(always)]
    #[must_use]
    pub fn ack_gen_call(&mut self) -> ACK_GEN_CALL_W<0> {
        ACK_GEN_CALL_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IC_ACK_GENERAL_CALL to value 0x01"]
impl crate::Resettable for IC_ACK_GENERAL_CALL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
