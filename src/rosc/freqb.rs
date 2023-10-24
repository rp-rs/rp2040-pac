#[doc = "Register `FREQB` reader"]
pub type R = crate::R<FREQB_SPEC>;
#[doc = "Register `FREQB` writer"]
pub type W = crate::W<FREQB_SPEC>;
#[doc = "Field `DS4` reader - Stage 4 drive strength"]
pub type DS4_R = crate::FieldReader;
#[doc = "Field `DS4` writer - Stage 4 drive strength"]
pub type DS4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DS5` reader - Stage 5 drive strength"]
pub type DS5_R = crate::FieldReader;
#[doc = "Field `DS5` writer - Stage 5 drive strength"]
pub type DS5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DS6` reader - Stage 6 drive strength"]
pub type DS6_R = crate::FieldReader;
#[doc = "Field `DS6` writer - Stage 6 drive strength"]
pub type DS6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `DS7` reader - Stage 7 drive strength"]
pub type DS7_R = crate::FieldReader;
#[doc = "Field `DS7` writer - Stage 7 drive strength"]
pub type DS7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `PASSWD` reader - Set to 0x9696 to apply the settings  
 Any other value in this field will set all drive strengths to 0"]
pub type PASSWD_R = crate::FieldReader<PASSWD_A>;
#[doc = "Set to 0x9696 to apply the settings  
 Any other value in this field will set all drive strengths to 0  

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
#[doc = "Field `PASSWD` writer - Set to 0x9696 to apply the settings  
 Any other value in this field will set all drive strengths to 0"]
pub type PASSWD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, PASSWD_A>;
impl<'a, REG, const O: u8> PASSWD_W<'a, REG, O>
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
    #[doc = "Bits 0:2 - Stage 4 drive strength"]
    #[inline(always)]
    pub fn ds4(&self) -> DS4_R {
        DS4_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Stage 5 drive strength"]
    #[inline(always)]
    pub fn ds5(&self) -> DS5_R {
        DS5_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Stage 6 drive strength"]
    #[inline(always)]
    pub fn ds6(&self) -> DS6_R {
        DS6_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Stage 7 drive strength"]
    #[inline(always)]
    pub fn ds7(&self) -> DS7_R {
        DS7_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:31 - Set to 0x9696 to apply the settings  
 Any other value in this field will set all drive strengths to 0"]
    #[inline(always)]
    pub fn passwd(&self) -> PASSWD_R {
        PASSWD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Stage 4 drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn ds4(&mut self) -> DS4_W<FREQB_SPEC, 0> {
        DS4_W::new(self)
    }
    #[doc = "Bits 4:6 - Stage 5 drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn ds5(&mut self) -> DS5_W<FREQB_SPEC, 4> {
        DS5_W::new(self)
    }
    #[doc = "Bits 8:10 - Stage 6 drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn ds6(&mut self) -> DS6_W<FREQB_SPEC, 8> {
        DS6_W::new(self)
    }
    #[doc = "Bits 12:14 - Stage 7 drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn ds7(&mut self) -> DS7_W<FREQB_SPEC, 12> {
        DS7_W::new(self)
    }
    #[doc = "Bits 16:31 - Set to 0x9696 to apply the settings  
 Any other value in this field will set all drive strengths to 0"]
    #[inline(always)]
    #[must_use]
    pub fn passwd(&mut self) -> PASSWD_W<FREQB_SPEC, 16> {
        PASSWD_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "For a detailed description see freqa register  

You can [`read`](crate::generic::Reg::read) this register and get [`freqb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`freqb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FREQB_SPEC;
impl crate::RegisterSpec for FREQB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`freqb::R`](R) reader structure"]
impl crate::Readable for FREQB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`freqb::W`](W) writer structure"]
impl crate::Writable for FREQB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FREQB to value 0"]
impl crate::Resettable for FREQB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
