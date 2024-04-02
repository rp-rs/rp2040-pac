#[doc = "Register `CSR` reader"]
pub type R = crate::R<CSR_SPEC>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CSR_SPEC>;
#[doc = "Field `EN` reader - Enable the PWM channel."]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Enable the PWM channel."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PH_CORRECT` reader - 1: Enable phase-correct modulation. 0: Trailing-edge"]
pub type PH_CORRECT_R = crate::BitReader;
#[doc = "Field `PH_CORRECT` writer - 1: Enable phase-correct modulation. 0: Trailing-edge"]
pub type PH_CORRECT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A_INV` reader - Invert output A"]
pub type A_INV_R = crate::BitReader;
#[doc = "Field `A_INV` writer - Invert output A"]
pub type A_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B_INV` reader - Invert output B"]
pub type B_INV_R = crate::BitReader;
#[doc = "Field `B_INV` writer - Invert output B"]
pub type B_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVMODE_A {
    #[doc = "0: Free-running counting at rate dictated by fractional divider"]
    DIV = 0,
    #[doc = "1: Fractional divider operation is gated by the PWM B pin."]
    LEVEL = 1,
    #[doc = "2: Counter advances with each rising edge of the PWM B pin."]
    RISE = 2,
    #[doc = "3: Counter advances with each falling edge of the PWM B pin."]
    FALL = 3,
}
impl From<DIVMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIVMODE_A {
    type Ux = u8;
}
impl crate::IsEnum for DIVMODE_A {}
#[doc = "Field `DIVMODE` reader - "]
pub type DIVMODE_R = crate::FieldReader<DIVMODE_A>;
impl DIVMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIVMODE_A {
        match self.bits {
            0 => DIVMODE_A::DIV,
            1 => DIVMODE_A::LEVEL,
            2 => DIVMODE_A::RISE,
            3 => DIVMODE_A::FALL,
            _ => unreachable!(),
        }
    }
    #[doc = "Free-running counting at rate dictated by fractional divider"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        *self == DIVMODE_A::DIV
    }
    #[doc = "Fractional divider operation is gated by the PWM B pin."]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == DIVMODE_A::LEVEL
    }
    #[doc = "Counter advances with each rising edge of the PWM B pin."]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == DIVMODE_A::RISE
    }
    #[doc = "Counter advances with each falling edge of the PWM B pin."]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == DIVMODE_A::FALL
    }
}
#[doc = "Field `DIVMODE` writer - "]
pub type DIVMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DIVMODE_A, crate::Safe>;
impl<'a, REG> DIVMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Free-running counting at rate dictated by fractional divider"]
    #[inline(always)]
    pub fn div(self) -> &'a mut crate::W<REG> {
        self.variant(DIVMODE_A::DIV)
    }
    #[doc = "Fractional divider operation is gated by the PWM B pin."]
    #[inline(always)]
    pub fn level(self) -> &'a mut crate::W<REG> {
        self.variant(DIVMODE_A::LEVEL)
    }
    #[doc = "Counter advances with each rising edge of the PWM B pin."]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(DIVMODE_A::RISE)
    }
    #[doc = "Counter advances with each falling edge of the PWM B pin."]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(DIVMODE_A::FALL)
    }
}
#[doc = "Field `PH_RET` reader - Retard the phase of the counter by 1 count, while it is running.  
 Self-clearing. Write a 1, and poll until low. Counter must be running."]
pub type PH_RET_R = crate::BitReader;
#[doc = "Field `PH_RET` writer - Retard the phase of the counter by 1 count, while it is running.  
 Self-clearing. Write a 1, and poll until low. Counter must be running."]
pub type PH_RET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PH_ADV` reader - Advance the phase of the counter by 1 count, while it is running.  
 Self-clearing. Write a 1, and poll until low. Counter must be running  
 at less than full speed (div_int + div_frac / 16 > 1)"]
pub type PH_ADV_R = crate::BitReader;
#[doc = "Field `PH_ADV` writer - Advance the phase of the counter by 1 count, while it is running.  
 Self-clearing. Write a 1, and poll until low. Counter must be running  
 at less than full speed (div_int + div_frac / 16 > 1)"]
pub type PH_ADV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable the PWM channel."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    pub fn ph_correct(&self) -> PH_CORRECT_R {
        PH_CORRECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Invert output A"]
    #[inline(always)]
    pub fn a_inv(&self) -> A_INV_R {
        A_INV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Invert output B"]
    #[inline(always)]
    pub fn b_inv(&self) -> B_INV_R {
        B_INV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn divmode(&self) -> DIVMODE_R {
        DIVMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Retard the phase of the counter by 1 count, while it is running.  
 Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    pub fn ph_ret(&self) -> PH_RET_R {
        PH_RET_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Advance the phase of the counter by 1 count, while it is running.  
 Self-clearing. Write a 1, and poll until low. Counter must be running  
 at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    pub fn ph_adv(&self) -> PH_ADV_R {
        PH_ADV_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable the PWM channel."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CSR_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    #[must_use]
    pub fn ph_correct(&mut self) -> PH_CORRECT_W<CSR_SPEC> {
        PH_CORRECT_W::new(self, 1)
    }
    #[doc = "Bit 2 - Invert output A"]
    #[inline(always)]
    #[must_use]
    pub fn a_inv(&mut self) -> A_INV_W<CSR_SPEC> {
        A_INV_W::new(self, 2)
    }
    #[doc = "Bit 3 - Invert output B"]
    #[inline(always)]
    #[must_use]
    pub fn b_inv(&mut self) -> B_INV_W<CSR_SPEC> {
        B_INV_W::new(self, 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn divmode(&mut self) -> DIVMODE_W<CSR_SPEC> {
        DIVMODE_W::new(self, 4)
    }
    #[doc = "Bit 6 - Retard the phase of the counter by 1 count, while it is running.  
 Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    #[must_use]
    pub fn ph_ret(&mut self) -> PH_RET_W<CSR_SPEC> {
        PH_RET_W::new(self, 6)
    }
    #[doc = "Bit 7 - Advance the phase of the counter by 1 count, while it is running.  
 Self-clearing. Write a 1, and poll until low. Counter must be running  
 at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    #[must_use]
    pub fn ph_adv(&mut self) -> PH_ADV_W<CSR_SPEC> {
        PH_ADV_W::new(self, 7)
    }
}
#[doc = "Control and status register  

You can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
