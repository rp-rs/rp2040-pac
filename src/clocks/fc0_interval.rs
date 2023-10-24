#[doc = "Register `FC0_INTERVAL` reader"]
pub type R = crate::R<FC0_INTERVAL_SPEC>;
#[doc = "Register `FC0_INTERVAL` writer"]
pub type W = crate::W<FC0_INTERVAL_SPEC>;
#[doc = "Field `FC0_INTERVAL` reader - "]
pub type FC0_INTERVAL_R = crate::FieldReader;
#[doc = "Field `FC0_INTERVAL` writer - "]
pub type FC0_INTERVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn fc0_interval(&self) -> FC0_INTERVAL_R {
        FC0_INTERVAL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn fc0_interval(&mut self) -> FC0_INTERVAL_W<FC0_INTERVAL_SPEC, 0> {
        FC0_INTERVAL_W::new(self)
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
#[doc = "The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval  
 The default gives a test interval of 250us  

You can [`read`](crate::generic::Reg::read) this register and get [`fc0_interval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fc0_interval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FC0_INTERVAL_SPEC;
impl crate::RegisterSpec for FC0_INTERVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc0_interval::R`](R) reader structure"]
impl crate::Readable for FC0_INTERVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fc0_interval::W`](W) writer structure"]
impl crate::Writable for FC0_INTERVAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FC0_INTERVAL to value 0x08"]
impl crate::Resettable for FC0_INTERVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
