#[doc = "Reader of register GPIO_IN"]
pub type R = crate::R<u32, super::GPIO_IN>;
#[doc = "Reader of field `GPIO_IN`"]
pub type GPIO_IN_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:29 - Input value for GPIO0...29"]
    #[inline(always)]
    pub fn gpio_in(&self) -> GPIO_IN_R {
        GPIO_IN_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
