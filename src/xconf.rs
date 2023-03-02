pub struct FilesSection {
    font_path: Option<String>,
    module_path: Option<String>,
    xkb_dir: Option<String>,
}

pub struct ServerFlagsSection {
    //custom setter?
    options: Option<Vec<(String, String)>>,
}

pub struct ModuleSection {
    load: Option<String>,
    disable: Option<String>,
    load_driver:
}

pub struct XConfExtensions;

pub struct XConfInputDevice;

pub struct XConfInputClass;

#[derive(Builder, Debug)]
pub struct DeviceSection {
    identifier: String,
    driver: String,
    screen: Option<u32>,
}

pub struct MonitorSection {
    identifier: String,
    vender_name: Option<String>,
    model_name: Option<String>,
    use_modes:
    mode_line:
    display_size: (u32, u32)
    horiz_sync: 
    vert_refresh:

}

pub struct XConfModes {
    
}

pub struct XConfScreen {

}

pub struct XConfDisplay {
    
}

pub struct  XConfServerLayout {

}

pub struct XConfDRI {

}