#[macro_use]
extern crate bitflags;


bitflags! {
    struct Spices: u32 {
        const SALT       = 0b00000001;
        const PEPPER     = 0b00000010;
        const CHILLY     = 0b00000100;
        const SAFFRON    = 0b00001000;
        const ALL        = Self::SALT.bits
                         | Self::PEPPER.bits
                         | Self::CHILLY.bits
                         | Self::SAFFRON.bits;
    }
}

impl Spices {
    pub fn clear(&mut self) -> &mut Self {
        self.bits = 0;
        self
    }
}



fn main() {
    let classic = Spices::SALT | Spices::PEPPER;
    let spicy = Spices::PEPPER | Spices::CHILLY;
    println!("Classic: {:?}", classic);
    println!("Bits: {:08b}", classic.bits());
    println!("Spicy: {:?}", spicy);
    println!("Bits: {:08b}", spicy.bits());

    println!();

    println!("Union: {:?}", classic | spicy);
    println!("Intersection: {:?}", classic & spicy);
    println!("Difference: {:?}", classic - spicy);
    println!("Complement: {:?}", !classic);


    let mut custom = classic | spicy;
    println!("Custom spice mix: {:?}", custom);
    /* To do: Showcase the following
    The following methods are defined for the generated struct:

    empty: an empty set of flags
    all: the set of all flags
    bits: the raw value of the flags currently stored
    from_bits: convert from underlying bit representation, unless that representation contains bits that do not correspond to a flag
    from_bits_truncate: convert from underlying bit representation, dropping any bits that do not correspond to flags
    is_empty: true if no flags are currently stored
    is_all: true if all flags are currently set
    intersects: true if there are flags common to both self and other
    contains: true all of the flags in other are contained within self
    insert: inserts the specified flags in-place
    remove: removes the specified flags in-place
    toggle: the specified flags will be inserted if not present, and removed if they are.
    set: inserts or removes the specified flags depending on the passed value
    */

    // To do: Show default

    custom.clear();
    println!("Custom spice mix after clearing: {:?}", custom);
}
