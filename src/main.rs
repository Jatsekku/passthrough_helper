mod gpu_devices {

    use devices::{DeviceInfo, Devices, Error};
    pub struct GpuDevices;

    impl GpuDevices {
        pub fn get() -> Result<Vec<DeviceInfo>, Error> {
            match Devices::get() {
                Ok(devices) => Ok(devices
                    .into_iter()
                    .filter(|d| d.class().contains("VGA"))
                    .collect()),
                Err(e) => Err(e),
            }
        }
    }
}

mod iommu_groups;

use crate::iommu_groups::*;

mod x_config_gen;
use x_config_gen::device_section::DeviceSection;

fn main() {
    // let devices = IommuMap::get().unwrap().into_iter().enumerate();
    // for (group_idx, devices) in devices {
    //     println!("GROUP: {}", group_idx);
    //     for device in devices {
    //         println!("{:?}", device);
    //     }
    //}
    let devices_section = DeviceSection::new()
            .add_comment("Test comment")
            .add_comment("test2")
            .set_identifier("ok").unwrap()
            .set_driver("Nvidia").unwrap()
            .set_membase(0x200).unwrap()
            .set_board("Someboard").unwrap();
    print!("{:100}", devices_section);
}
