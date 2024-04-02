#[doc = "Register `IC_ACK_GENERAL_CALL` reader"]
pub type R = crate::R<IC_ACK_GENERAL_CALL_SPEC>;
#[doc = "Register `IC_ACK_GENERAL_CALL` writer"]
pub type W = crate::W<IC_ACK_GENERAL_CALL_SPEC>;
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
#[doc = "Field `ACK_GEN_CALL` reader - ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe)."]
pub type ACK_GEN_CALL_R = crate::BitReader<ACK_GEN_CALL_A>;
impl ACK_GEN_CALL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACK_GEN_CALL_A {
        match self.bits {
            false => ACK_GEN_CALL_A::DISABLED,
            true => ACK_GEN_CALL_A::ENABLED,
        }
    }
    #[doc = "Generate NACK for a General Call"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ACK_GEN_CALL_A::DISABLED
    }
    #[doc = "Generate ACK for a General Call"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ACK_GEN_CALL_A::ENABLED
    }
}
#[doc = "Field `ACK_GEN_CALL` writer - ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe)."]
pub type ACK_GEN_CALL_W<'a, REG> = crate::BitWriter<'a, REG, ACK_GEN_CALL_A>;
impl<'a, REG> ACK_GEN_CALL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Generate NACK for a General Call"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ACK_GEN_CALL_A::DISABLED)
    }
    #[doc = "Generate ACK for a General Call"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
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
    pub fn ack_gen_call(&mut self) -> ACK_GEN_CALL_W<IC_ACK_GENERAL_CALL_SPEC> {
        ACK_GEN_CALL_W::new(self, 0)
    }
}
#[doc = "I2C ACK General Call Register  

 The register controls whether DW_apb_i2c responds with a ACK or NACK when it receives an I2C General Call address.  

 This register is applicable only when the DW_apb_i2c is in slave mode.  

You can [`read`](crate::Reg::read) this register and get [`ic_ack_general_call::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_ack_general_call::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IC_ACK_GENERAL_CALL_SPEC;
impl crate::RegisterSpec for IC_ACK_GENERAL_CALL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_ack_general_call::R`](R) reader structure"]
impl crate::Readable for IC_ACK_GENERAL_CALL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ic_ack_general_call::W`](W) writer structure"]
impl crate::Writable for IC_ACK_GENERAL_CALL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IC_ACK_GENERAL_CALL to value 0x01"]
impl crate::Resettable for IC_ACK_GENERAL_CALL_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
