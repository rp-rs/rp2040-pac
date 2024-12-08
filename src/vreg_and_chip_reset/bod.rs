#[doc = "Register `BOD` reader"]
pub type R = crate::R<BOD_SPEC>;
#[doc = "Register `BOD` writer"]
pub type W = crate::W<BOD_SPEC>;
#[doc = "Field `EN` reader - enable 0=not enabled, 1=enabled"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - enable 0=not enabled, 1=enabled"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSEL` reader - threshold select 0000 - 0.473V 0001 - 0.516V 0010 - 0.559V 0011 - 0.602V 0100 - 0.645V 0101 - 0.688V 0110 - 0.731V 0111 - 0.774V 1000 - 0.817V 1001 - 0.860V (default) 1010 - 0.903V 1011 - 0.946V 1100 - 0.989V 1101 - 1.032V 1110 - 1.075V 1111 - 1.118V"]
pub type VSEL_R = crate::FieldReader;
#[doc = "Field `VSEL` writer - threshold select 0000 - 0.473V 0001 - 0.516V 0010 - 0.559V 0011 - 0.602V 0100 - 0.645V 0101 - 0.688V 0110 - 0.731V 0111 - 0.774V 1000 - 0.817V 1001 - 0.860V (default) 1010 - 0.903V 1011 - 0.946V 1100 - 0.989V 1101 - 1.032V 1110 - 1.075V 1111 - 1.118V"]
pub type VSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - enable 0=not enabled, 1=enabled"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - threshold select 0000 - 0.473V 0001 - 0.516V 0010 - 0.559V 0011 - 0.602V 0100 - 0.645V 0101 - 0.688V 0110 - 0.731V 0111 - 0.774V 1000 - 0.817V 1001 - 0.860V (default) 1010 - 0.903V 1011 - 0.946V 1100 - 0.989V 1101 - 1.032V 1110 - 1.075V 1111 - 1.118V"]
    #[inline(always)]
    pub fn vsel(&self) -> VSEL_R {
        VSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - enable 0=not enabled, 1=enabled"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<BOD_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - threshold select 0000 - 0.473V 0001 - 0.516V 0010 - 0.559V 0011 - 0.602V 0100 - 0.645V 0101 - 0.688V 0110 - 0.731V 0111 - 0.774V 1000 - 0.817V 1001 - 0.860V (default) 1010 - 0.903V 1011 - 0.946V 1100 - 0.989V 1101 - 1.032V 1110 - 1.075V 1111 - 1.118V"]
    #[inline(always)]
    #[must_use]
    pub fn vsel(&mut self) -> VSEL_W<BOD_SPEC> {
        VSEL_W::new(self, 4)
    }
}
#[doc = "brown-out detection control  

You can [`read`](crate::generic::Reg::read) this register and get [`bod::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bod::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOD_SPEC;
impl crate::RegisterSpec for BOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bod::R`](R) reader structure"]
impl crate::Readable for BOD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bod::W`](W) writer structure"]
impl crate::Writable for BOD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOD to value 0x91"]
impl crate::Resettable for BOD_SPEC {
    const RESET_VALUE: u32 = 0x91;
}
