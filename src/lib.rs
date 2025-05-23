pub mod notifiers;

pub use macros::infix;
pub use macros::notifier;
pub use macros::reactive_value;

#[cfg(test)]
mod tests {
    extern crate self as eo;

    use std::sync::Arc;
    use eo::notifier;
    use macros::{infix, reactive_value};
    use crate::notifiers::{Notifier, ReactiveValue};
    
    #[test]
    fn test() {
        notifier!(a = 0);
        
        reactive_value!(b = $a + 1);
        
        reactive_value!(c = $b as f64 / 2.0);
        
        println!("b first = {}", b.get());
        
        a.set(10);
        
        println!("b second = {}, c = {}", b.get(), c.get());
        
        infix! {
            a listen |x| println!("`a` was set to {x}");
            a modify |x| {
                let buf: i32 = x + 3;
        
                buf.abs() / 2
            };
            a listen |_| {}
        }
    }
}