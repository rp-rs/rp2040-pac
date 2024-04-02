#[doc = "Register `DIV` reader"]
pub type R = crate::R<DIV_SPEC>;
#[doc = "Register `DIV` writer"]
pub type W = crate::W<DIV_SPEC>;
#[doc = "Field `FRAC` reader - Fractional part of clock divisor. First-order delta-sigma."]
pub type FRAC_R = crate::FieldReader;
#[doc = "Field `FRAC` writer - Fractional part of clock divisor. First-order delta-sigma."]
pub type FRAC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INT` reader - Integer part of clock divisor."]
pub type INT_R = crate::FieldReader<u16>;
#[doc = "Field `INT` writer - Integer part of clock divisor."]
pub type INT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - Fractional part of clock divisor. First-order delta-sigma."]
    #[inline(always)]
    pub fn frac(&self) -> FRAC_R {
        FRAC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:23 - Integer part of clock divisor."]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fractional part of clock divisor. First-order delta-sigma."]
    #[inline(always)]
    #[must_use]
    pub fn frac(&mut self) -> FRAC_W<DIV_SPEC> {
        FRAC_W::new(self, 0)
    }
    #[doc = "Bits 8:23 - Integer part of clock divisor."]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<DIV_SPEC> {
        INT_W::new(self, 8)
    }
}
#[doc = "Clock divider. If non-zero, CS_START_MANY will start conversions  
 at regular intervals rather than back-to-back.  
 The divider is reset when either of these fields are written.  
 Total period is 1 + INT + FRAC / 256  

You can [`read`](crate::Reg::read) this register and get [`div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIV_SPEC;
impl crate::RegisterSpec for DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div::R`](R) reader structure"]
impl crate::Readable for DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`div::W`](W) writer structure"]
impl crate::Writable for DIV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIV to value 0"]
impl crate::Resettable for DIV_SPEC {
    const RESET_VALUE: u32 = 0;
}
