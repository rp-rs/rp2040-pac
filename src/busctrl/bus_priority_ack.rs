#[doc = "Reader of register BUS_PRIORITY_ACK"]
pub type R = crate::R<u32, super::BUS_PRIORITY_ACK>;
#[doc = "Reader of field `BUS_PRIORITY_ACK`"]
pub type BUS_PRIORITY_ACK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Goes to 1 once all arbiters have registered the new global priority levels.\\n Arbiters update their local priority when servicing a new nonsequential access.\\n In normal circumstances this will happen almost immediately."]
    #[inline(always)]
    pub fn bus_priority_ack(&self) -> BUS_PRIORITY_ACK_R {
        BUS_PRIORITY_ACK_R::new((self.bits & 0x01) != 0)
    }
}
