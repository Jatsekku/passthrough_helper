use std::fmt::{self};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeviceSectionElement {
    Comment(String),    // <0;+inf)
    Identifier(String), // 1 (Required)
    Vendor(String),     // {0;1}
    Board(String),      // {0;1}
    Chipset(String),    // {0;1}
    Card(String),       // {0;1}
    Driver(String),     // 1 (Required)
    RamDac(String),     // {0;1}
    DacSpeed(Vec<i32>), // {0;1}
    VideoRam(i32),      // {0;1}
    //BiosBase(),         // Ignored
    MemBase(u64),       // {0;1}
    IoBase(u64),        // {0;1}
    ClockChip(String),  // {0;1}
    ChipId(i32),        // {0;1}
    ChipRev(i32),       // {0;1}
    Clocks(Vec<i32>),   // {0;1}
    MatchSeat(String),  // {0;1}
    Option(),           // <0;+inf)
    BusId(String),      // {0;1}
    Irq(i32),           // {0;1}
    Screen(i32),        // {0;1}
}

impl fmt::Display for DeviceSectionElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceSectionElement::Comment(value) => write!(f, "# {}\n", value),
            DeviceSectionElement::Identifier(value) => write!(f, "Identifier \"{}\"\n", value),
            DeviceSectionElement::Vendor(value) => write!(f, "Vendor \"{}\"\n", value),
            DeviceSectionElement::Board(value) => write!(f, "Board \"{}\"\n", value),
            DeviceSectionElement::Chipset(value) => write!(f, "Chipset \"{}\"\n", value),
            DeviceSectionElement::Card(value) => write!(f, "Card \"{}\"\n", value),
            DeviceSectionElement::Driver(value) => write!(f, "Driver \"{}\"\n", value),
            DeviceSectionElement::RamDac(value) => write!(f, "Ramdac \"{}\"\n", value),
            DeviceSectionElement::DacSpeed(values) => {
                let values: String = values.into_iter().map(|i| format!("{} ", i)).collect();
                write!(f, "Dacspeed {}\n", values)
            }
            DeviceSectionElement::VideoRam(value) => write!(f, "VideoRam {}\n", value),
            DeviceSectionElement::MemBase(value) => write!(f, "MemBase {}\n", value),
            DeviceSectionElement::IoBase(value) => write!(f, "MemBase {}\n", value),
            DeviceSectionElement::ClockChip(value) => write!(f, "ClockChip \"{}\"\n", value),
            DeviceSectionElement::ChipId(value) => write!(f, "ChipID {}\n", value),
            DeviceSectionElement::ChipRev(value) => write!(f, "ChipRev {}\n", value),
            // DeviceSectionElement::Clocks(values) => {
            //     let mut text = String::new();
            //     for (i, value) in values.enumerate() {
            //         text.push(format!("{} ", value));
            //         if i % 8 == 0 {
            //             text.push("\n")
            //         }
            //     }
            // }
            DeviceSectionElement::MatchSeat(value) => write!(f, "MatchSeat \"{}\"\n", value),
            DeviceSectionElement::BusId(value) => write!(f, "BusID \"{}\"\n", value),
            DeviceSectionElement::Irq(value) => write!(f, "IRQ {}\n", value),
            DeviceSectionElement::Screen(value) => write!(f, "IRQ {}\n", value),
            _ => write!(f, "not supported")
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct DeviceSection {
    content: Vec<DeviceSectionElement>,
    has_identifier: bool,
    has_vendor: bool,
    has_board: bool,
    has_chipset: bool,
    has_card: bool,
    has_driver: bool,
    has_ramdac: bool,
    has_dacspeed: bool,
    has_videoram: bool,
    has_membase: bool,
    has_iobase: bool,
    has_clockchip: bool,
    has_chipid: bool,
    has_chiprev: bool,
    has_clocks: bool,
    has_matchseat: bool,
    has_busid: bool,
    has_irq: bool,
    has_screen: bool,
}

#[allow(dead_code)]
impl DeviceSection {
    pub fn new() -> Self {
        Default::default()
    }

    create_function!(add_comment, &str, DeviceSectionElement::Comment);
    create_function!(set_identifier, &str, DeviceSectionElement::Identifier, has_identifier);
    create_function!(set_vendor, &str, DeviceSectionElement::Vendor, has_vendor);
    create_function!(set_board, &str, DeviceSectionElement::Board, has_board);
    create_function!(set_chipset, &str, DeviceSectionElement::Chipset, has_chipset);
    create_function!(set_card, &str, DeviceSectionElement::Card, has_card);
    create_function!(set_driver, &str, DeviceSectionElement::Driver, has_driver);
    create_function!(set_ramdac, &str, DeviceSectionElement::RamDac, has_ramdac);
    create_function!(set_dacspeed, &Vec<i32>, DeviceSectionElement::DacSpeed, has_dacspeed);
    create_function!(set_videoram, i32, DeviceSectionElement::VideoRam, has_videoram);
    create_function!(set_membase, u64, DeviceSectionElement::MemBase, has_membase);
    create_function!(set_iobase, u64, DeviceSectionElement::IoBase, has_iobase);
    create_function!(set_clockship, &str, DeviceSectionElement::ClockChip, has_clockchip);
    create_function!(set_chipid, i32, DeviceSectionElement::ChipId, has_chipid);
    create_function!(set_chiprev, i32, DeviceSectionElement::ChipRev, has_chiprev);
    create_function!(set_clocks, &Vec<i32>, DeviceSectionElement::Clocks, has_clocks);
    create_function!(set_matchset, &str, DeviceSectionElement::MatchSeat, has_matchseat);
    create_function!(set_busid, &str, DeviceSectionElement::BusId, has_busid);
    create_function!(set_irq, i32, DeviceSectionElement::Irq, has_irq);
    create_function!(set_screen, i32, DeviceSectionElement::Screen, has_screen);
}

impl fmt::Display for DeviceSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content.iter().map(ToString::to_string).collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use crate::x_config_gen::device_section::{DeviceSectionElement, DeviceSection};


    #[test]
    fn add_single_comment() {
        let device_section = DeviceSection::new().add_comment("Test comment");

        assert_eq!(
            device_section,
            DeviceSection {
                content: vec![DeviceSectionElement::Comment(String::from("Test comment"))],
                ..Default::default()
            }
        )
    }

    #[test]
    fn add_identifier_and_videoram() {
        let device_section = DeviceSection::new()
            .set_identifier("someID")
            .unwrap()
            .set_videoram(4)
            .unwrap();

        assert_eq!(
            device_section,
            DeviceSection {
                content: vec![
                    DeviceSectionElement::Identifier(String::from("someID")),
                    DeviceSectionElement::VideoRam(4)
                ],
                has_identifier: true,
                has_videoram: true,
                ..Default::default()
            }
        )
    }
}
