#[doc = "Register `GPIO_HI_OE` reader"]
pub struct R(crate::R<GPIO_HI_OE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_HI_OE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_HI_OE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_HI_OE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_HI_OE` writer"]
pub struct W(crate::W<GPIO_HI_OE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_HI_OE_SPEC>;
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
impl From<crate::W<GPIO_HI_OE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_HI_OE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_HI_OE` reader - Set output enable (1/0 -> output/input) for QSPI IO0...5.  
 Reading back gives the last value written.  
 If core 0 and core 1 both write to GPIO_HI_OE simultaneously (or to a SET/CLR/XOR alias),  
 the result is as though the write from core 0 took place first,  
 and the write from core 1 was then applied to that intermediate result."]
pub type GPIO_HI_OE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPIO_HI_OE` writer - Set output enable (1/0 -> output/input) for QSPI IO0...5.  
 Reading back gives the last value written.  
 If core 0 and core 1 both write to GPIO_HI_OE simultaneously (or to a SET/CLR/XOR alias),  
 the result is as though the write from core 0 took place first,  
 and the write from core 1 was then applied to that intermediate result."]
pub type GPIO_HI_OE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPIO_HI_OE_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Set output enable (1/0 -> output/input) for QSPI IO0...5.  
 Reading back gives the last value written.  
 If core 0 and core 1 both write to GPIO_HI_OE simultaneously (or to a SET/CLR/XOR alias),  
 the result is as though the write from core 0 took place first,  
 and the write from core 1 was then applied to that intermediate result."]
    #[inline(always)]
    pub fn gpio_hi_oe(&self) -> GPIO_HI_OE_R {
        GPIO_HI_OE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Set output enable (1/0 -> output/input) for QSPI IO0...5.  
 Reading back gives the last value written.  
 If core 0 and core 1 both write to GPIO_HI_OE simultaneously (or to a SET/CLR/XOR alias),  
 the result is as though the write from core 0 took place first,  
 and the write from core 1 was then applied to that intermediate result."]
    #[inline(always)]
    #[must_use]
    pub fn gpio_hi_oe(&mut self) -> GPIO_HI_OE_W<0> {
        GPIO_HI_OE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI output enable  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [gpio_hi_oe](index.html) module"]
pub struct GPIO_HI_OE_SPEC;
impl crate::RegisterSpec for GPIO_HI_OE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_hi_oe::R](R) reader structure"]
impl crate::Readable for GPIO_HI_OE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_hi_oe::W](W) writer structure"]
impl crate::Writable for GPIO_HI_OE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_HI_OE to value 0"]
impl crate::Resettable for GPIO_HI_OE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
