#[doc = "Register `PRIM` reader"]
pub type R = crate::R<PRIM_SPEC>;
#[doc = "Register `PRIM` writer"]
pub type W = crate::W<PRIM_SPEC>;
#[doc = "Field `POSTDIV2` reader - divide by 1-7"]
pub type POSTDIV2_R = crate::FieldReader;
#[doc = "Field `POSTDIV2` writer - divide by 1-7"]
pub type POSTDIV2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `POSTDIV1` reader - divide by 1-7"]
pub type POSTDIV1_R = crate::FieldReader;
#[doc = "Field `POSTDIV1` writer - divide by 1-7"]
pub type POSTDIV1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 12:14 - divide by 1-7"]
    #[inline(always)]
    pub fn postdiv2(&self) -> POSTDIV2_R {
        POSTDIV2_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - divide by 1-7"]
    #[inline(always)]
    pub fn postdiv1(&self) -> POSTDIV1_R {
        POSTDIV1_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 12:14 - divide by 1-7"]
    #[inline(always)]
    #[must_use]
    pub fn postdiv2(&mut self) -> POSTDIV2_W<PRIM_SPEC> {
        POSTDIV2_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - divide by 1-7"]
    #[inline(always)]
    #[must_use]
    pub fn postdiv1(&mut self) -> POSTDIV1_W<PRIM_SPEC> {
        POSTDIV1_W::new(self, 16)
    }
}
#[doc = "Controls the PLL post dividers for the primary output  
 (note: this PLL does not have a secondary output)  
 the primary output is driven from VCO divided by postdiv1*postdiv2  

You can [`read`](crate::Reg::read) this register and get [`prim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIM_SPEC;
impl crate::RegisterSpec for PRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prim::R`](R) reader structure"]
impl crate::Readable for PRIM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prim::W`](W) writer structure"]
impl crate::Writable for PRIM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIM to value 0x0007_7000"]
impl crate::Resettable for PRIM_SPEC {
    const RESET_VALUE: u32 = 0x0007_7000;
}
