#[doc = "Register `SM_PINCTRL` reader"]
pub type R = crate::R<SM_PINCTRL_SPEC>;
#[doc = "Register `SM_PINCTRL` writer"]
pub type W = crate::W<SM_PINCTRL_SPEC>;
#[doc = "Field `OUT_BASE` reader - The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data."]
pub type OUT_BASE_R = crate::FieldReader;
#[doc = "Field `OUT_BASE` writer - The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data."]
pub type OUT_BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SET_BASE` reader - The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data."]
pub type SET_BASE_R = crate::FieldReader;
#[doc = "Field `SET_BASE` writer - The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data."]
pub type SET_BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SIDESET_BASE` reader - The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins."]
pub type SIDESET_BASE_R = crate::FieldReader;
#[doc = "Field `SIDESET_BASE` writer - The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins."]
pub type SIDESET_BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IN_BASE` reader - The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number."]
pub type IN_BASE_R = crate::FieldReader;
#[doc = "Field `IN_BASE` writer - The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number."]
pub type IN_BASE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OUT_COUNT` reader - The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive."]
pub type OUT_COUNT_R = crate::FieldReader;
#[doc = "Field `OUT_COUNT` writer - The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive."]
pub type OUT_COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SET_COUNT` reader - The number of pins asserted by a SET. In the range 0 to 5 inclusive."]
pub type SET_COUNT_R = crate::FieldReader;
#[doc = "Field `SET_COUNT` writer - The number of pins asserted by a SET. In the range 0 to 5 inclusive."]
pub type SET_COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SIDESET_COUNT` reader - The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay)."]
pub type SIDESET_COUNT_R = crate::FieldReader;
#[doc = "Field `SIDESET_COUNT` writer - The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay)."]
pub type SIDESET_COUNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data."]
    #[inline(always)]
    pub fn out_base(&self) -> OUT_BASE_R {
        OUT_BASE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data."]
    #[inline(always)]
    pub fn set_base(&self) -> SET_BASE_R {
        SET_BASE_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins."]
    #[inline(always)]
    pub fn sideset_base(&self) -> SIDESET_BASE_R {
        SIDESET_BASE_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number."]
    #[inline(always)]
    pub fn in_base(&self) -> IN_BASE_R {
        IN_BASE_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:25 - The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive."]
    #[inline(always)]
    pub fn out_count(&self) -> OUT_COUNT_R {
        OUT_COUNT_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:28 - The number of pins asserted by a SET. In the range 0 to 5 inclusive."]
    #[inline(always)]
    pub fn set_count(&self) -> SET_COUNT_R {
        SET_COUNT_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay)."]
    #[inline(always)]
    pub fn sideset_count(&self) -> SIDESET_COUNT_R {
        SIDESET_COUNT_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data."]
    #[inline(always)]
    #[must_use]
    pub fn out_base(&mut self) -> OUT_BASE_W<SM_PINCTRL_SPEC> {
        OUT_BASE_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data."]
    #[inline(always)]
    #[must_use]
    pub fn set_base(&mut self) -> SET_BASE_W<SM_PINCTRL_SPEC> {
        SET_BASE_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins."]
    #[inline(always)]
    #[must_use]
    pub fn sideset_base(&mut self) -> SIDESET_BASE_W<SM_PINCTRL_SPEC> {
        SIDESET_BASE_W::new(self, 10)
    }
    #[doc = "Bits 15:19 - The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number."]
    #[inline(always)]
    #[must_use]
    pub fn in_base(&mut self) -> IN_BASE_W<SM_PINCTRL_SPEC> {
        IN_BASE_W::new(self, 15)
    }
    #[doc = "Bits 20:25 - The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive."]
    #[inline(always)]
    #[must_use]
    pub fn out_count(&mut self) -> OUT_COUNT_W<SM_PINCTRL_SPEC> {
        OUT_COUNT_W::new(self, 20)
    }
    #[doc = "Bits 26:28 - The number of pins asserted by a SET. In the range 0 to 5 inclusive."]
    #[inline(always)]
    #[must_use]
    pub fn set_count(&mut self) -> SET_COUNT_W<SM_PINCTRL_SPEC> {
        SET_COUNT_W::new(self, 26)
    }
    #[doc = "Bits 29:31 - The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay)."]
    #[inline(always)]
    #[must_use]
    pub fn sideset_count(&mut self) -> SIDESET_COUNT_W<SM_PINCTRL_SPEC> {
        SIDESET_COUNT_W::new(self, 29)
    }
}
#[doc = "State machine pin control  

You can [`read`](crate::Reg::read) this register and get [`sm_pinctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sm_pinctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SM_PINCTRL_SPEC;
impl crate::RegisterSpec for SM_PINCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sm_pinctrl::R`](R) reader structure"]
impl crate::Readable for SM_PINCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sm_pinctrl::W`](W) writer structure"]
impl crate::Writable for SM_PINCTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SM_PINCTRL to value 0x1400_0000"]
impl crate::Resettable for SM_PINCTRL_SPEC {
    const RESET_VALUE: u32 = 0x1400_0000;
}
