#[doc = "Register `CLK_USB_DIV` reader"]
pub struct R(crate::R<CLK_USB_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_USB_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_USB_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_USB_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_USB_DIV` writer"]
pub struct W(crate::W<CLK_USB_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_USB_DIV_SPEC>;
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
impl From<crate::W<CLK_USB_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_USB_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT` reader - Integer component of the divisor, 0 -> divide by 2^16"]
pub type INT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INT` writer - Integer component of the divisor, 0 -> divide by 2^16"]
pub type INT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_USB_DIV_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 8:9 - Integer component of the divisor, 0 -> divide by 2^16"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - Integer component of the divisor, 0 -> divide by 2^16"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<8> {
        INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock divisor, can be changed on-the-fly  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [clk_usb_div](index.html) module"]
pub struct CLK_USB_DIV_SPEC;
impl crate::RegisterSpec for CLK_USB_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_usb_div::R](R) reader structure"]
impl crate::Readable for CLK_USB_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_usb_div::W](W) writer structure"]
impl crate::Writable for CLK_USB_DIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_USB_DIV to value 0x0100"]
impl crate::Resettable for CLK_USB_DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
