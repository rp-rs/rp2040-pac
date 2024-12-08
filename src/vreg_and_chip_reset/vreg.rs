#[doc = "Register `VREG` reader"]
pub type R = crate::R<VREG_SPEC>;
#[doc = "Register `VREG` writer"]
pub type W = crate::W<VREG_SPEC>;
#[doc = "Field `EN` reader - enable 0=not enabled, 1=enabled"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - enable 0=not enabled, 1=enabled"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HIZ` reader - high impedance mode select 0=not in high impedance mode, 1=in high impedance mode"]
pub type HIZ_R = crate::BitReader;
#[doc = "Field `HIZ` writer - high impedance mode select 0=not in high impedance mode, 1=in high impedance mode"]
pub type HIZ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Output voltage select for on-chip voltage regulator.  

Value on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VSEL_A {
    #[doc = "5: 0.80V"]
    VOLTAGE0_80 = 5,
    #[doc = "6: 0.85V"]
    VOLTAGE0_85 = 6,
    #[doc = "7: 0.90V"]
    VOLTAGE0_90 = 7,
    #[doc = "8: 0.95V"]
    VOLTAGE0_95 = 8,
    #[doc = "9: 1.00V"]
    VOLTAGE1_00 = 9,
    #[doc = "10: 1.05V"]
    VOLTAGE1_05 = 10,
    #[doc = "11: 1.10V (default)"]
    VOLTAGE1_10 = 11,
    #[doc = "12: 1.15V"]
    VOLTAGE1_15 = 12,
    #[doc = "13: 1.20V"]
    VOLTAGE1_20 = 13,
    #[doc = "14: 1.25V"]
    VOLTAGE1_25 = 14,
    #[doc = "15: 1.30V"]
    VOLTAGE1_30 = 15,
}
impl From<VSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: VSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VSEL_A {
    type Ux = u8;
}
#[doc = "Field `VSEL` reader - Output voltage select for on-chip voltage regulator."]
pub type VSEL_R = crate::FieldReader<VSEL_A>;
impl VSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VSEL_A> {
        match self.bits {
            5 => Some(VSEL_A::VOLTAGE0_80),
            6 => Some(VSEL_A::VOLTAGE0_85),
            7 => Some(VSEL_A::VOLTAGE0_90),
            8 => Some(VSEL_A::VOLTAGE0_95),
            9 => Some(VSEL_A::VOLTAGE1_00),
            10 => Some(VSEL_A::VOLTAGE1_05),
            11 => Some(VSEL_A::VOLTAGE1_10),
            12 => Some(VSEL_A::VOLTAGE1_15),
            13 => Some(VSEL_A::VOLTAGE1_20),
            14 => Some(VSEL_A::VOLTAGE1_25),
            15 => Some(VSEL_A::VOLTAGE1_30),
            _ => None,
        }
    }
    #[doc = "0.80V"]
    #[inline(always)]
    pub fn is_voltage0_80(&self) -> bool {
        *self == VSEL_A::VOLTAGE0_80
    }
    #[doc = "0.85V"]
    #[inline(always)]
    pub fn is_voltage0_85(&self) -> bool {
        *self == VSEL_A::VOLTAGE0_85
    }
    #[doc = "0.90V"]
    #[inline(always)]
    pub fn is_voltage0_90(&self) -> bool {
        *self == VSEL_A::VOLTAGE0_90
    }
    #[doc = "0.95V"]
    #[inline(always)]
    pub fn is_voltage0_95(&self) -> bool {
        *self == VSEL_A::VOLTAGE0_95
    }
    #[doc = "1.00V"]
    #[inline(always)]
    pub fn is_voltage1_00(&self) -> bool {
        *self == VSEL_A::VOLTAGE1_00
    }
    #[doc = "1.05V"]
    #[inline(always)]
    pub fn is_voltage1_05(&self) -> bool {
        *self == VSEL_A::VOLTAGE1_05
    }
    #[doc = "1.10V (default)"]
    #[inline(always)]
    pub fn is_voltage1_10(&self) -> bool {
        *self == VSEL_A::VOLTAGE1_10
    }
    #[doc = "1.15V"]
    #[inline(always)]
    pub fn is_voltage1_15(&self) -> bool {
        *self == VSEL_A::VOLTAGE1_15
    }
    #[doc = "1.20V"]
    #[inline(always)]
    pub fn is_voltage1_20(&self) -> bool {
        *self == VSEL_A::VOLTAGE1_20
    }
    #[doc = "1.25V"]
    #[inline(always)]
    pub fn is_voltage1_25(&self) -> bool {
        *self == VSEL_A::VOLTAGE1_25
    }
    #[doc = "1.30V"]
    #[inline(always)]
    pub fn is_voltage1_30(&self) -> bool {
        *self == VSEL_A::VOLTAGE1_30
    }
}
#[doc = "Field `VSEL` writer - Output voltage select for on-chip voltage regulator."]
pub type VSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, VSEL_A>;
impl<'a, REG> VSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.80V"]
    #[inline(always)]
    pub fn voltage0_80(self) -> &'a mut crate::W<REG> {
        self.variant(VSEL_A::VOLTAGE0_80)
    }
    #[doc = "0.85V"]
    #[inline(always)]
    pub fn voltage0_85(self) -> &'a mut crate::W<REG> {
        self.variant(VSEL_A::VOLTAGE0_85)
    }
    #[doc = "0.90V"]
    #[inline(always)]
    pub fn voltage0_90(self) -> &'a mut crate::W<REG> {
        self.variant(VSEL_A::VOLTAGE0_90)
    }
    #[doc = "0.95V"]
    #[inline(always)]
    pub fn voltage0_95(self) -> &'a mut crate::W<REG> {
        self.variant(VSEL_A::VOLTAGE0_95)
    }
    #[doc = "1.00V"]
    #[inline(always)]
    pub fn voltage1_00(self) -> &'a mut crate::W<REG> {
        self.variant(VSEL_A::VOLTAGE1_00)
    }
    #[doc = "1.05V"]
    #[inline(always)]
    pub fn voltage1_05(self) -> &'a mut crate::W<REG> {
        self.variant(VSEL_A::VOLTAGE1_05)
    }
    #[doc = "1.10V (default)"]
    #[inline(always)]
    pub fn voltage1_10(self) -> &'a mut crate::W<REG> {
        self.variant(VSEL_A::VOLTAGE1_10)
    }
    #[doc = "1.15V"]
    #[inline(always)]
    pub fn voltage1_15(self) -> &'a mut crate::W<REG> {
        self.variant(VSEL_A::VOLTAGE1_15)
    }
    #[doc = "1.20V"]
    #[inline(always)]
    pub fn voltage1_20(self) -> &'a mut crate::W<REG> {
        self.variant(VSEL_A::VOLTAGE1_20)
    }
    #[doc = "1.25V"]
    #[inline(always)]
    pub fn voltage1_25(self) -> &'a mut crate::W<REG> {
        self.variant(VSEL_A::VOLTAGE1_25)
    }
    #[doc = "1.30V"]
    #[inline(always)]
    pub fn voltage1_30(self) -> &'a mut crate::W<REG> {
        self.variant(VSEL_A::VOLTAGE1_30)
    }
}
#[doc = "Field `ROK` reader - regulation status 0=not in regulation, 1=in regulation"]
pub type ROK_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - enable 0=not enabled, 1=enabled"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - high impedance mode select 0=not in high impedance mode, 1=in high impedance mode"]
    #[inline(always)]
    pub fn hiz(&self) -> HIZ_R {
        HIZ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Output voltage select for on-chip voltage regulator."]
    #[inline(always)]
    pub fn vsel(&self) -> VSEL_R {
        VSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - regulation status 0=not in regulation, 1=in regulation"]
    #[inline(always)]
    pub fn rok(&self) -> ROK_R {
        ROK_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable 0=not enabled, 1=enabled"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<VREG_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - high impedance mode select 0=not in high impedance mode, 1=in high impedance mode"]
    #[inline(always)]
    #[must_use]
    pub fn hiz(&mut self) -> HIZ_W<VREG_SPEC> {
        HIZ_W::new(self, 1)
    }
    #[doc = "Bits 4:7 - Output voltage select for on-chip voltage regulator."]
    #[inline(always)]
    #[must_use]
    pub fn vsel(&mut self) -> VSEL_W<VREG_SPEC> {
        VSEL_W::new(self, 4)
    }
}
#[doc = "Voltage regulator control and status  

You can [`read`](crate::generic::Reg::read) this register and get [`vreg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vreg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VREG_SPEC;
impl crate::RegisterSpec for VREG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vreg::R`](R) reader structure"]
impl crate::Readable for VREG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`vreg::W`](W) writer structure"]
impl crate::Writable for VREG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREG to value 0xb1"]
impl crate::Resettable for VREG_SPEC {
    const RESET_VALUE: u32 = 0xb1;
}
