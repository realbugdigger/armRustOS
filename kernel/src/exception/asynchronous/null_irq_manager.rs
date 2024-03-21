//! Null IRQ Manager.

use super::{interface, IRQContext, IRQHandlerDescriptor};


pub struct NullIRQManager;


pub static NULL_IRQ_MANAGER: NullIRQManager = NullIRQManager {};


impl interface::IRQManager for NullIRQManager {
    type IRQNumberType = super::IRQNumber;

    fn register_handler(
        &self,
        _descriptor: IRQHandlerDescriptor<Self::IRQNumberType>,
    ) -> Result<(), &'static str> {
        panic!("No IRQ Manager registered yet");
    }

    fn enable(&self, _irq_number: &Self::IRQNumberType) {
        panic!("No IRQ Manager registered yet");
    }

    fn handle_pending_irqs<'irq_context>(&'irq_context self, _ic: &IRQContext<'irq_context>) {
        panic!("No IRQ Manager registered yet");
    }
}