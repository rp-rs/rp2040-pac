#[doc = "Register `FREQA` reader"]
pub type R = crate::R<FREQA_SPEC>;
#[doc = "Register `FREQA` writer"]
pub type W = crate::W<FREQA_SPEC>;
#[doc = "Field `DS0` reader - Stage 0 drive strength"]
pub type DS0_R = crate::FieldReader;
#[doc = "Field `DS0` writer - Stage 0 drive strength"]
pub type DS0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DS1` reader - Stage 1 drive strength"]
pub type DS1_R = crate::FieldReader;
#[doc = "Field `DS1` writer - Stage 1 drive strength"]
pub type DS1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DS2` reader - Stage 2 drive strength"]
pub type DS2_R = crate::FieldReader;
#[doc = "Field `DS2` writer - Stage 2 drive strength"]
pub type DS2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DS3` reader - Stage 3 drive strength"]
pub type DS3_R = crate::FieldReader;
#[doc = "Field `DS3` writer - Stage 3 drive strength"]
pub type DS3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Set to 0x9696 to apply the settings Any other value in this field will set all drive strengths to 0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum PASSWD_A {
    #[doc = "38550: `1001011010010110`"]
    PASS = 38550,
}
impl From<PASSWD_A> for u16 {
    #[inline(always)]
    fn from(variant: PASSWD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PASSWD_A {
    type Ux = u16;
}
#[doc = "Field `PASSWD` reader - Set to 0x9696 to apply the settings Any other value in this field will set all drive strengths to 0"]
pub type PASSWD_R = crate::FieldReader<PASSWD_A>;
impl PASSWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PASSWD_A> {
        match self.bits {
            38550 => Some(PASSWD_A::PASS),
            _ => None,
        }
    }
    #[doc = "`1001011010010110`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        *self == PASSWD_A::PASS
    }
}
#[doc = "Field `PASSWD` writer - Set to 0x9696 to apply the settings Any other value in this field will set all drive strengths to 0"]
pub type PASSWD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, PASSWD_A>;
impl<'a, REG> PASSWD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`1001011010010110`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut crate::W<REG> {
        self.variant(PASSWD_A::PASS)
    }
}
impl R {
    #[doc = "Bits 0:2 - Stage 0 drive strength"]
    #[inline(always)]
    pub fn ds0(&self) -> DS0_R {
        DS0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Stage 1 drive strength"]
    #[inline(always)]
    pub fn ds1(&self) -> DS1_R {
        DS1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Stage 2 drive strength"]
    #[inline(always)]
    pub fn ds2(&self) -> DS2_R {
        DS2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Stage 3 drive strength"]
    #[inline(always)]
    pub fn ds3(&self) -> DS3_R {
        DS3_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:31 - Set to 0x9696 to apply the settings Any other value in this field will set all drive strengths to 0"]
    #[inline(always)]
    pub fn passwd(&self) -> PASSWD_R {
        PASSWD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Stage 0 drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn ds0(&mut self) -> DS0_W<FREQA_SPEC> {
        DS0_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Stage 1 drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn ds1(&mut self) -> DS1_W<FREQA_SPEC> {
        DS1_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Stage 2 drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn ds2(&mut self) -> DS2_W<FREQA_SPEC> {
        DS2_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Stage 3 drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn ds3(&mut self) -> DS3_W<FREQA_SPEC> {
        DS3_W::new(self, 12)
    }
    #[doc = "Bits 16:31 - Set to 0x9696 to apply the settings Any other value in this field will set all drive strengths to 0"]
    #[inline(always)]
    #[must_use]
    pub fn passwd(&mut self) -> PASSWD_W<FREQA_SPEC> {
        PASSWD_W::new(self, 16)
    }
}
#[doc = "The FREQA &amp; FREQB registers control the frequency by controlling the drive strength of each stage The drive strength has 4 levels determined by the number of bits set Increasing the number of bits set increases the drive strength and increases the oscillation frequency 0 bits set is the default drive strength 1 bit set doubles the drive strength 2 bits set triples drive strength 3 bits set quadruples drive strength  

You can [`read`](crate::generic::Reg::read) this register and get [`freqa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freqa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FREQA_SPEC;
impl crate::RegisterSpec for FREQA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`freqa::R`](R) reader structure"]
impl crate::Readable for FREQA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`freqa::W`](W) writer structure"]
impl crate::Writable for FREQA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FREQA to value 0"]
impl crate::Resettable for FREQA_SPEC {
    const RESET_VALUE: u32 = 0;
}
