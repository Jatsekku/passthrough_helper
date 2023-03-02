#[macro_use]
extern crate derive_builder;

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

mod xconf;
use crate::xconf::*;

fn main() {
    let devices = IommuMap::get().unwrap().into_iter().enumerate();
    for (group_idx, devices) in devices {
        println!("GROUP: {}", group_idx);
        for device in devices {
            println!("{:?}", device);
        }
    }

    print!(
        "{:?}",
        XConfDeviceBuilder::default()
            .identifier("test".to_string())
            .build()
    );
}
