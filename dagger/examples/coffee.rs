use std::cell::RefCell;
use std::rc::Rc;

use dagger::{self, component, inject, module, Builder, Component, Singleton};

/// A coffee maker to brew the coffee.
#[derive(Debug)]
struct CoffeeMaker<H, P> {
    logger: Rc<RefCell<CoffeeLogger>>,
    /// Create a possibly costly heater only when we use it.
    heater: H,
    pump: P,
}

impl<H, P> CoffeeMaker<H, P>
where
    H: Heater,
    P: Pump,
{
    #[inject]
    pub fn new(logger: Rc<RefCell<CoffeeLogger>>, heater: H, pump: P) -> Self {
        CoffeeMaker {
            logger,
            heater,
            pump,
        }
    }

    pub fn brew(&mut self) {
        self.heater.on();
        self.pump.pump();
        self.logger.borrow_mut().log(" [_]P coffee! [_]P ");
        self.heater.off();
    }
}

/// A logger to logs steps while brewing coffee.
#[derive(Debug, Singleton)]
struct CoffeeLogger {
    logs: Vec<String>,
}

impl CoffeeLogger {
    pub fn log<S: Into<String>>(&mut self, msg: S) {
        self.logs.push(msg.into());
    }

    pub fn logs(&self) -> impl Iterator<Item = &str> {
        self.logs.iter().map(|s| s.as_str())
    }
}

/// A heater to heat the coffee.
trait Heater {
    fn on(&mut self);

    fn off(&mut self);

    fn is_hot(&self) -> bool;
}

/// An electric heater to heat the coffee.
#[derive(Debug)]
struct ElectricHeater {
    logger: Rc<RefCell<CoffeeLogger>>,
    heating: bool,
}

impl ElectricHeater {
    #[inject]
    pub fn new(logger: Rc<RefCell<CoffeeLogger>>) -> Self {
        Self {
            logger,
            heating: false,
        }
    }
}

impl Heater for ElectricHeater {
    fn on(&mut self) {
        self.heating = true;
        self.logger.borrow_mut().log("~ ~ ~ heating ~ ~ ~");
    }

    fn off(&mut self) {
        self.heating = false;
    }

    fn is_hot(&self) -> bool {
        self.heating
    }
}

/// A pump to pump the coffee.
trait Pump {
    fn pump(&mut self);
}

#[derive(Debug)]
struct Thermosiphon<H> {
    logger: Rc<RefCell<CoffeeLogger>>,
    heater: H,
}

impl<H> Thermosiphon<H> {
    #[inject]
    pub fn new(logger: Rc<CoffeeLogger>, heater: H) -> Self {
        Self { logger, heater }
    }
}

impl<H> Pump for Thermosiphon<H>
where
    H: Heater,
{
    fn pump(&mut self) {
        if self.heater.is_hot() {
            self.logger.borrow_mut().log("=> => pumping => =>");
        }
    }
}

#[module]
trait HeaterModule {
    #[binds]
    #[singleton]
    fn bind_heater(heater: ElectricHeater) -> impl Heater;
}

#[module]
trait PumpModule {
    #[binds]
    fn provide_pump(pump: Thermosiphon) -> impl Pump;
}

// #[component(modules = [HeaterModule, PumpModule])]
trait CoffeeShop<H, P> {
    fn maker(&mut self) -> CoffeeMaker<H, P>;

    fn logger(&mut self) -> Rc<RefCell<CoffeeLogger>>;
}

struct CoffeeShopInstance {}

impl CoffeeShop<ElectricHeater, Thermosiphon<ElectricHeater>> for CoffeeShopInstance {
    fn maker(&mut self) -> CoffeeMaker<ElectricHeater, Thermosiphon<ElectricHeater>> {
        unimplemented!()
    }

    fn logger(&mut self) -> Rc<RefCell<CoffeeLogger>> {
        unimplemented!()
    }
}

struct CoffeeShopBuilder {}

impl Builder<CoffeeShopInstance> for CoffeeShopBuilder {
    fn build() -> CoffeeShopInstance {
        CoffeeShopInstance {}
    }
}

/// The main function responsible for brewing the coffee and printing the logs.
fn main() {
    let mut coffeeShop = CoffeeShopBuilder::build();
    coffeeShop.maker().brew();
    for log in coffeeShop.logger().borrow().logs() {
        println!("{}", log);
    }
}
