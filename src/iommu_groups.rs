use devices::{DeviceInfo, DevicePath, Devices};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{Error, ErrorKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IommuMap;
const IOMMU_PATH: &str = "/sys/kernel/iommu_groups/";

impl IommuMap {
    fn get_paths() -> Result<(usize, Vec<String>), Error> {
        let mut devices_paths: Vec<String> = vec![];
        let mut iommu_groups_count: usize = 0;

        let iommu_groups_dirs = fs::read_dir(IOMMU_PATH)?;
        for dir in iommu_groups_dirs {
            iommu_groups_count += 1;
            let mut iommu_group_path = dir?.path();
            iommu_group_path.push("devices/");
            for device_path in fs::read_dir(iommu_group_path)? {
                let device_path = device_path?.path().display().to_string().clone();
                match device_path.strip_prefix(IOMMU_PATH) {
                    None => return Err(Error::from(ErrorKind::InvalidData)),
                    Some(s) => devices_paths.push(s.to_string()),
                };
            }
        }

        Ok((iommu_groups_count, devices_paths))
    }

    fn get_groups() -> Result<Vec<HashSet<DevicePath>>, Error> {
        let (iommu_groups_count, device_paths) = IommuMap::get_paths()?;
        let mut iommu_groups = vec![HashSet::new(); iommu_groups_count];

        device_paths.into_iter().for_each(|device_path| {
            let device_path: Vec<&str> = device_path.split_terminator('/').collect();
            let (iommu_idx, device_addr) =
                (device_path[0].parse::<usize>().unwrap(), device_path[2]);
            let device_addr: Vec<&str> = device_addr.split_terminator(&['.', ':']).collect();
            iommu_groups[iommu_idx].insert(DevicePath::PCI {
                bus: u8::from_str_radix(device_addr[1], 16).unwrap(),
                slot: u8::from_str_radix(device_addr[2], 16).unwrap(),
                function: u8::from_str_radix(device_addr[3], 16).unwrap(),
            });
        });

        Ok(iommu_groups)
    }

    pub fn get() -> Result<Vec<HashSet<DeviceInfo>>, ()> {
        let devices_infos: HashMap<DevicePath, DeviceInfo> = Devices::get()
            .unwrap()
            .into_iter()
            .map(|device| (*device.path(), device))
            .collect();
        let iommu_groups = IommuMap::get_groups().unwrap();

        let detailed_groups: Vec<HashSet<DeviceInfo>> = iommu_groups
            .into_iter()
            .map(|iommu_group| {
                let detailed_group: HashSet<DeviceInfo> = iommu_group
                    .into_iter()
                    .map(|device_path| match devices_infos.get(&device_path) {
                        Some(device_info) => device_info.clone(),
                        None => panic!("No entry"),
                    })
                    .collect();
                detailed_group
            })
            .collect();
        Ok(detailed_groups)
    }
}
