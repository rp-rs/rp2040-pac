#[doc = "Register `STARTUP` reader"]
pub type R = crate::R<STARTUP_SPEC>;
#[doc = "Register `STARTUP` writer"]
pub type W = crate::W<STARTUP_SPEC>;
#[doc = "Field `DELAY` reader - in multiples of 256*xtal_period. The reset value of 0xc4 corresponds to approx 50 000 cycles."]
pub type DELAY_R = crate::FieldReader<u16>;
#[doc = "Field `DELAY` writer - in multiples of 256*xtal_period. The reset value of 0xc4 corresponds to approx 50 000 cycles."]
pub type DELAY_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 14, O, u16>;
#[doc = "Field `X4` reader - Multiplies the startup_delay by 4. This is of little value to the user given that the delay can be programmed directly."]
pub type X4_R = crate::BitReader;
#[doc = "Field `X4` writer - Multiplies the startup_delay by 4. This is of little value to the user given that the delay can be programmed directly."]
pub type X4_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:13 - in multiples of 256*xtal_period. The reset value of 0xc4 corresponds to approx 50 000 cycles."]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 20 - Multiplies the startup_delay by 4. This is of little value to the user given that the delay can be programmed directly."]
    #[inline(always)]
    pub fn x4(&self) -> X4_R {
        X4_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - in multiples of 256*xtal_period. The reset value of 0xc4 corresponds to approx 50 000 cycles."]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<STARTUP_SPEC, 0> {
        DELAY_W::new(self)
    }
    #[doc = "Bit 20 - Multiplies the startup_delay by 4. This is of little value to the user given that the delay can be programmed directly."]
    #[inline(always)]
    #[must_use]
    pub fn x4(&mut self) -> X4_W<STARTUP_SPEC, 20> {
        X4_W::new(self)
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
#[doc = "Controls the startup delay  

You can [`read`](crate::generic::Reg::read) this register and get [`startup::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`startup::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STARTUP_SPEC;
impl crate::RegisterSpec for STARTUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`startup::R`](R) reader structure"]
impl crate::Readable for STARTUP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`startup::W`](W) writer structure"]
impl crate::Writable for STARTUP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STARTUP to value 0xc4"]
impl crate::Resettable for STARTUP_SPEC {
    const RESET_VALUE: Self::Ux = 0xc4;
}
