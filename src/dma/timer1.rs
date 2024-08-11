#[doc = "Register `TIMER1` reader"]
pub type R = crate::R<TIMER1_SPEC>;
#[doc = "Register `TIMER1` writer"]
pub type W = crate::W<TIMER1_SPEC>;
#[doc = "Field `Y` reader - Pacing Timer Divisor. Specifies the Y value for the (X/Y) fractional timer."]
pub type Y_R = crate::FieldReader<u16>;
#[doc = "Field `Y` writer - Pacing Timer Divisor. Specifies the Y value for the (X/Y) fractional timer."]
pub type Y_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `X` reader - Pacing Timer Dividend. Specifies the X value for the (X/Y) fractional timer."]
pub type X_R = crate::FieldReader<u16>;
#[doc = "Field `X` writer - Pacing Timer Dividend. Specifies the X value for the (X/Y) fractional timer."]
pub type X_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Pacing Timer Divisor. Specifies the Y value for the (X/Y) fractional timer."]
    #[inline(always)]
    pub fn y(&self) -> Y_R {
        Y_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Pacing Timer Dividend. Specifies the X value for the (X/Y) fractional timer."]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pacing Timer Divisor. Specifies the Y value for the (X/Y) fractional timer."]
    #[inline(always)]
    #[must_use]
    pub fn y(&mut self) -> Y_W<TIMER1_SPEC> {
        Y_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Pacing Timer Dividend. Specifies the X value for the (X/Y) fractional timer."]
    #[inline(always)]
    #[must_use]
    pub fn x(&mut self) -> X_W<TIMER1_SPEC> {
        X_W::new(self, 16)
    }
}
#[doc = "Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less.  

You can [`read`](crate::generic::Reg::read) this register and get [`timer1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER1_SPEC;
impl crate::RegisterSpec for TIMER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer1::R`](R) reader structure"]
impl crate::Readable for TIMER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer1::W`](W) writer structure"]
impl crate::Writable for TIMER1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER1 to value 0"]
impl crate::Resettable for TIMER1_SPEC {
    const RESET_VALUE: u32 = 0;
}
