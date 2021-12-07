#[doc = "Register `GPIO_HI_OUT` reader"]
pub struct R(crate::R<GPIO_HI_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_HI_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_HI_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_HI_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_HI_OUT` writer"]
pub struct W(crate::W<GPIO_HI_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_HI_OUT_SPEC>;
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
impl From<crate::W<GPIO_HI_OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_HI_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_HI_OUT` reader - Set output level (1/0 -> high/low) for QSPI IO0...5.  
 Reading back gives the last value written, NOT the input value from the pins.  
 If core 0 and core 1 both write to GPIO_HI_OUT simultaneously (or to a SET/CLR/XOR alias),  
 the result is as though the write from core 0 took place first,  
 and the write from core 1 was then applied to that intermediate result."]
pub struct GPIO_HI_OUT_R(crate::FieldReader<u8, u8>);
impl GPIO_HI_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GPIO_HI_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_HI_OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_HI_OUT` writer - Set output level (1/0 -> high/low) for QSPI IO0...5.  
 Reading back gives the last value written, NOT the input value from the pins.  
 If core 0 and core 1 both write to GPIO_HI_OUT simultaneously (or to a SET/CLR/XOR alias),  
 the result is as though the write from core 0 took place first,  
 and the write from core 1 was then applied to that intermediate result."]
pub struct GPIO_HI_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_HI_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Set output level (1/0 -> high/low) for QSPI IO0...5.  
 Reading back gives the last value written, NOT the input value from the pins.  
 If core 0 and core 1 both write to GPIO_HI_OUT simultaneously (or to a SET/CLR/XOR alias),  
 the result is as though the write from core 0 took place first,  
 and the write from core 1 was then applied to that intermediate result."]
    #[inline(always)]
    pub fn gpio_hi_out(&self) -> GPIO_HI_OUT_R {
        GPIO_HI_OUT_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Set output level (1/0 -> high/low) for QSPI IO0...5.  
 Reading back gives the last value written, NOT the input value from the pins.  
 If core 0 and core 1 both write to GPIO_HI_OUT simultaneously (or to a SET/CLR/XOR alias),  
 the result is as though the write from core 0 took place first,  
 and the write from core 1 was then applied to that intermediate result."]
    #[inline(always)]
    pub fn gpio_hi_out(&mut self) -> GPIO_HI_OUT_W {
        GPIO_HI_OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QSPI output value  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [gpio_hi_out](index.html) module"]
pub struct GPIO_HI_OUT_SPEC;
impl crate::RegisterSpec for GPIO_HI_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_hi_out::R](R) reader structure"]
impl crate::Readable for GPIO_HI_OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_hi_out::W](W) writer structure"]
impl crate::Writable for GPIO_HI_OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_HI_OUT to value 0"]
impl crate::Resettable for GPIO_HI_OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
