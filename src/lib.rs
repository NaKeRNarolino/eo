pub mod notifiers;
pub mod logger;
pub mod events;

pub use macros::event;
pub use macros::event_init;
pub use macros::infix;
pub use macros::notifier;
pub use macros::reactive_value;

#[cfg(test)]
mod tests {
    use colored::Colorize;
    extern crate self as eo;

    use crate::logger::EoLogger;
    use eo::notifier;
    use macros::{event_init, infix, reactive_value};
    use std::sync::RwLock;

    #[test]
    fn test() {
        log::set_logger(&EoLogger).unwrap();
        log::set_max_level(log::LevelFilter::Debug);
        
        notifier!(a = 0);
        
        reactive_value!(b = $a + 1);
        
        reactive_value!(c = $b as f64 / 2.0);

        a.set(10);
        
        println!("b second = {}, c = {}", b.get().to_string().red(), c.get());

        // event!(event i32);

        let event = event_init!(i32);

        infix! {
            event subscribe |x| {
                println!("Got {x}!")
            };
            a listen |x| println!("`a` was set to {x}");
            a modify |x| {
                let buf: i32 = x + 3;

                buf.abs() / 2
            };
            a listen |_| {};
            event notify 10
        }
    }
}