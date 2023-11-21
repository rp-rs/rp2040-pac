#[doc = "Register `PHASE` reader"]
pub type R = crate::R<PHASE_SPEC>;
#[doc = "Register `PHASE` writer"]
pub type W = crate::W<PHASE_SPEC>;
#[doc = "Field `SHIFT` reader - phase shift the phase-shifted output by SHIFT input clocks  
 this can be changed on-the-fly  
 must be set to 0 before setting div=1"]
pub type SHIFT_R = crate::FieldReader;
#[doc = "Field `SHIFT` writer - phase shift the phase-shifted output by SHIFT input clocks  
 this can be changed on-the-fly  
 must be set to 0 before setting div=1"]
pub type SHIFT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `FLIP` reader - invert the phase-shifted output  
 this is ignored when div=1"]
pub type FLIP_R = crate::BitReader;
#[doc = "Field `FLIP` writer - invert the phase-shifted output  
 this is ignored when div=1"]
pub type FLIP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ENABLE` reader - enable the phase-shifted output  
 this can be changed on-the-fly"]
pub type ENABLE_R = crate::BitReader;
#[doc = "Field `ENABLE` writer - enable the phase-shifted output  
 this can be changed on-the-fly"]
pub type ENABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PASSWD` reader - set to 0xaa  
 any other value enables the output with shift=0"]
pub type PASSWD_R = crate::FieldReader;
#[doc = "Field `PASSWD` writer - set to 0xaa  
 any other value enables the output with shift=0"]
pub type PASSWD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:1 - phase shift the phase-shifted output by SHIFT input clocks  
 this can be changed on-the-fly  
 must be set to 0 before setting div=1"]
    #[inline(always)]
    pub fn shift(&self) -> SHIFT_R {
        SHIFT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - invert the phase-shifted output  
 this is ignored when div=1"]
    #[inline(always)]
    pub fn flip(&self) -> FLIP_R {
        FLIP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable the phase-shifted output  
 this can be changed on-the-fly"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:11 - set to 0xaa  
 any other value enables the output with shift=0"]
    #[inline(always)]
    pub fn passwd(&self) -> PASSWD_R {
        PASSWD_R::new(((self.bits >> 4) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - phase shift the phase-shifted output by SHIFT input clocks  
 this can be changed on-the-fly  
 must be set to 0 before setting div=1"]
    #[inline(always)]
    #[must_use]
    pub fn shift(&mut self) -> SHIFT_W<PHASE_SPEC, 0> {
        SHIFT_W::new(self)
    }
    #[doc = "Bit 2 - invert the phase-shifted output  
 this is ignored when div=1"]
    #[inline(always)]
    #[must_use]
    pub fn flip(&mut self) -> FLIP_W<PHASE_SPEC, 2> {
        FLIP_W::new(self)
    }
    #[doc = "Bit 3 - enable the phase-shifted output  
 this can be changed on-the-fly"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<PHASE_SPEC, 3> {
        ENABLE_W::new(self)
    }
    #[doc = "Bits 4:11 - set to 0xaa  
 any other value enables the output with shift=0"]
    #[inline(always)]
    #[must_use]
    pub fn passwd(&mut self) -> PASSWD_W<PHASE_SPEC, 4> {
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
#[doc = "Controls the phase shifted output  

You can [`read`](crate::generic::Reg::read) this register and get [`phase::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phase::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PHASE_SPEC;
impl crate::RegisterSpec for PHASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phase::R`](R) reader structure"]
impl crate::Readable for PHASE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`phase::W`](W) writer structure"]
impl crate::Writable for PHASE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PHASE to value 0x08"]
impl crate::Resettable for PHASE_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
