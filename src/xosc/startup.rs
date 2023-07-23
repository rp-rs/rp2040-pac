#[doc = "Register `STARTUP` reader"]
pub struct R(crate::R<STARTUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STARTUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STARTUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STARTUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STARTUP` writer"]
pub struct W(crate::W<STARTUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STARTUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<STARTUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STARTUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DELAY` reader - in multiples of 256*xtal_period. The reset value of 0xc4 corresponds to approx 50 000 cycles."]
pub type DELAY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DELAY` writer - in multiples of 256*xtal_period. The reset value of 0xc4 corresponds to approx 50 000 cycles."]
pub type DELAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STARTUP_SPEC, u16, u16, 14, O>;
#[doc = "Field `X4` reader - Multiplies the startup_delay by 4. This is of little value to the user given that the delay can be programmed directly."]
pub type X4_R = crate::BitReader<bool>;
#[doc = "Field `X4` writer - Multiplies the startup_delay by 4. This is of little value to the user given that the delay can be programmed directly."]
pub type X4_W<'a, const O: u8> = crate::BitWriter<'a, u32, STARTUP_SPEC, bool, O>;
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
    pub fn delay(&mut self) -> DELAY_W<0> {
        DELAY_W::new(self)
    }
    #[doc = "Bit 20 - Multiplies the startup_delay by 4. This is of little value to the user given that the delay can be programmed directly."]
    #[inline(always)]
    #[must_use]
    pub fn x4(&mut self) -> X4_W<20> {
        X4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls the startup delay  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [startup](index.html) module"]
pub struct STARTUP_SPEC;
impl crate::RegisterSpec for STARTUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [startup::R](R) reader structure"]
impl crate::Readable for STARTUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [startup::W](W) writer structure"]
impl crate::Writable for STARTUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STARTUP to value 0xc4"]
impl crate::Resettable for STARTUP_SPEC {
    const RESET_VALUE: Self::Ux = 0xc4;
}
