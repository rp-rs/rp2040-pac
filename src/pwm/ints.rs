#[doc = "Register `INTS` reader"]
pub type R = crate::R<INTS_SPEC>;
#[doc = "Register `INTS` writer"]
pub type W = crate::W<INTS_SPEC>;
#[doc = "Field `CH0` reader - "]
pub type CH0_R = crate::BitReader;
#[doc = "Field `CH1` reader - "]
pub type CH1_R = crate::BitReader;
#[doc = "Field `CH2` reader - "]
pub type CH2_R = crate::BitReader;
#[doc = "Field `CH3` reader - "]
pub type CH3_R = crate::BitReader;
#[doc = "Field `CH4` reader - "]
pub type CH4_R = crate::BitReader;
#[doc = "Field `CH5` reader - "]
pub type CH5_R = crate::BitReader;
#[doc = "Field `CH6` reader - "]
pub type CH6_R = crate::BitReader;
#[doc = "Field `CH7` reader - "]
pub type CH7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {}
#[doc = "Interrupt status after masking &amp; forcing  

You can [`read`](crate::generic::Reg::read) this register and get [`ints::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ints::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTS_SPEC;
impl crate::RegisterSpec for INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ints::R`](R) reader structure"]
impl crate::Readable for INTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ints::W`](W) writer structure"]
impl crate::Writable for INTS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTS to value 0"]
impl crate::Resettable for INTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
