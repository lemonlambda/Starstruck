use ash::{Instance, Entry, Device, vk::{ApplicationInfo, make_api_version, InstanceCreateInfo, PhysicalDevice, DeviceCreateInfo, DeviceQueueCreateInfo, DebugUtilsMessengerCreateInfoEXT, DebugUtilsMessageTypeFlagsEXT, DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessengerEXT, DebugUtilsMessengerCallbackDataEXT, Bool32}, extensions::ext::DebugUtils};
use ash::extensions::khr::Surface;
use winit::window::Window;
use std::ffi::{CString, CStr};
use std::borrow::Cow;

pub struct Renderer {
    entry: Entry,
    instance: Option<Instance>,
    debug_callback: Option<DebugUtilsMessengerEXT>,
    physical_device: Option<PhysicalDevice>,
    device_family_index: Option<u32>,
    logical_device: Option<Device>,
    surface: Option<Surface>,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            entry: unsafe { Entry::load().unwrap() },
            instance: None,
            debug_callback: None,
            physical_device: None,
            device_family_index: None,
            logical_device: None,
            surface: None
        }
    }

    /// Creates a VulkanInstance
    pub fn create_instance<S: Into<String>>(&mut self, application_name: S, window: &Window) -> &mut Self {
        let app_info = ApplicationInfo {
            p_application_name: CString::new(application_name.into()).unwrap().into_raw(),
            p_engine_name: CString::new(String::from("Starstruck")).unwrap().into_raw(),
            api_version: make_api_version(0, 1, 0, 0),
            ..Default::default()
        };
        let mut extensions = vec!["VK_LAYER_KHRONOS_validation"];
        // extensions.push(DebugUtils::NAME);

        let extensions_raw = extensions.into_iter().map(|x| CString::new(String::from(x)).unwrap().into_raw() as *const i8).collect::<Vec<_>>();
        let create_info = InstanceCreateInfo {
            p_application_info: &app_info,
            enabled_extension_count: extensions_raw.len() as u32,
            pp_enabled_extension_names: extensions_raw.as_ptr(),
            ..Default::default()
        };
        self.instance = unsafe { Some(self.entry.create_instance(&create_info, None).unwrap()) };
        self
    }

    pub fn create_debug(&mut self) -> &mut Self {
        let debug_create_info = DebugUtilsMessengerCreateInfoEXT::builder().message_type(
            DebugUtilsMessageTypeFlagsEXT::GENERAL | DebugUtilsMessageTypeFlagsEXT::VALIDATION | DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
        ).message_severity(
            DebugUtilsMessageSeverityFlagsEXT::INFO | DebugUtilsMessageSeverityFlagsEXT::WARNING | DebugUtilsMessageSeverityFlagsEXT::ERROR
        ).pfn_user_callback(Some(vulkan_debug_callback)).build();

        let debug_utils_loader = DebugUtils::new(&self.entry, self.instance.as_ref().unwrap());
        self.debug_callback = unsafe { Some(debug_utils_loader.create_debug_utils_messenger(&debug_create_info, None).unwrap()) };
        self
    }

    /// Creates physical and logical devices
    pub fn create_physical_device(&mut self) -> &mut Self {
        let instance_ref = self.instance.as_ref().unwrap();
        let physical_devices = unsafe { instance_ref.enumerate_physical_devices().unwrap() };
        // Get the best device to use
        let mut best_device = (None, None);

        // Check if we need to find the best device
        if physical_devices.len() == 1 {
            best_device = (Some(physical_devices[0]), Some(0));
        } else {
            for (i, device) in physical_devices.into_iter().enumerate() {
                match best_device.0 {
                    Some(best) => {
                        if get_suitable_device(instance_ref, best) > get_suitable_device(instance_ref, device) {
                            best_device = (Some(device), Some(i as u32));
                        } else { 
                            continue; 
                        }
                    },
                    None => {},
                };
            }
        }

        self.physical_device = best_device.0;
        self.device_family_index = best_device.1;

        self
    }

    fn create_logical_device(&mut self) -> &mut Self {
        let device_queue_create_info = DeviceQueueCreateInfo {
            queue_family_index: self.device_family_index.unwrap(),
            queue_count: 1,
            ..Default::default()
        };
        
        let device_create_info = DeviceCreateInfo {
            p_queue_create_infos: &device_queue_create_info,
            ..Default::default()
        };

        let instance_ref = self.instance.as_ref().unwrap();
        self.logical_device = unsafe { Some(instance_ref.create_device(self.physical_device.unwrap(), &device_create_info, None).unwrap()) };

        self
    }
}

fn get_suitable_device(instance: &Instance, device: PhysicalDevice) -> i32 {
    let properties = unsafe { instance.get_physical_device_features(device) };
    let mut score = 0;
    if properties.geometry_shader != 0 {
        score += 1;
    }
    if properties.robust_buffer_access != 0 {
        score += 1;
    }
    return score;
}

unsafe extern "system" fn vulkan_debug_callback(
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_type: DebugUtilsMessageTypeFlagsEXT,
    callback_data: *const DebugUtilsMessengerCallbackDataEXT,
    _user_date: *mut std::os::raw::c_void,
) -> Bool32 {
    let callback_data = *callback_data;
    let message_id_number = callback_data.message_id_number;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!("{message_severity:?}:\n{message_type:?} [{message_id_name} {message_id_number}]: {message}");

    0
}