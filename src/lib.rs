use std::ffi::{c_char, c_void, CStr};

#[repr(transparent)]
pub struct FunctionName(*const c_char);

#[repr(transparent)]
pub struct LayerName(*const c_char);

#[repr(transparent)]
pub struct InstanceHandle(*mut c_void);

type GeneralFn = unsafe extern "C" fn();

#[repr(i32)]
pub enum StructureType {
    ApplicationInformation = 0,
    InstanceCreateInformation = 1,
}

#[repr(C)]
pub struct ApplicationInformation {
    structure_type: StructureType,
    next_structure: *const c_void,
    application_name: *const c_char,
    application_version: u32,
    engine_name: *const c_char,
    engine_version: u32,
    api_version: u32,
}

#[repr(i32)]
pub enum InstanceCreateFlags {
    None = 0,
}

#[repr(C)]
pub struct InstanceCreateInformation {
    structure_type: StructureType,
    next_structure: *const c_void,
    flags: InstanceCreateFlags,
    application_information: *const ApplicationInformation,
    enabled_layer_count: u32,
    enabled_layer_names: *const *const c_char,
    enabled_extension_count: u32,
    enabled_extension_names: *const *const c_char,
}

pub type AllocationFunction = unsafe extern "C" fn();
pub type ReallocationFunction = unsafe extern "C" fn();
pub type FreeFunction = unsafe extern "C" fn();
pub type InternalAllocationNotification = unsafe extern "C" fn();
pub type InternalFreeNotification = unsafe extern "C" fn();

#[repr(C)]
pub struct AllocationCallbacks {
    user_data: *const c_void,
    allocation: AllocationFunction,
    reallocation: ReallocationFunction,
    free: FreeFunction,
    internal_allocation: InternalAllocationNotification,
    internal_free: InternalFreeNotification,
}

#[repr(C)]
pub struct ExtensionProperties {
    name: [c_char; 256],
    specification_version: u32,
}

#[repr(i32)]
pub enum Result {
    Success = 0,
    NotReady = 1,
    Timeout = 2,
    EventSet = 3,
    EventReset = 4,
    Incomplete = 5,
    ErrorOutOfHostMemory = -1,
    ErrorOutOfDeviceMemory = -2,
    ErrorInitializationFailed = -3,
    ErrorDeviceLost = -4,
    ErrorMemoryMapFailed = -5,
    ErrorLayerNotPresent = -6,
    ErrorExtensionNotPresent = -7,
    ErrorFeatureNotPresent = -8,
    ErrorIncompatibleDriver = -9,
    ErrorTooManyObject = -10,
    ErrorFormatNotSupported = -11,
    ErrorFragmentedPool = -12,
    ErrorUnknown = -13,
}

struct Instance {}

impl Instance {
    fn new() -> Self {
        Self {}
    }
}

impl InstanceHandle {
    unsafe fn as_instance(&self) -> &'static mut Instance {
        let ptr = self.0 as *mut Instance;

        ptr.as_mut().unwrap()
    }

    unsafe fn into_instance(self) -> Box<Instance> {
        let ptr = self.0 as *mut Instance;

        Box::from_raw(ptr)
    }

    fn from_instance(instance: Instance) -> Self {
        let reference = Box::leak(Box::new(instance));
        let ptr = reference as *mut Instance;

        Self(ptr as _)
    }
}

#[no_mangle]
pub unsafe extern "C" fn vkCreateInstance(
    _create_information: *const InstanceCreateInformation,
    _allocator: *const AllocationCallbacks,
    instance_handle: *mut InstanceHandle,
) -> Result {
    if instance_handle.is_null() {
        Result::ErrorInitializationFailed
    } else {
        let instance = Instance::new();

        *instance_handle = InstanceHandle::from_instance(instance);

        Result::Success
    }
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumerateInstanceExtensionProperties(
    _layer_name: LayerName,
    _property_count: *mut u32,
    _properties: *mut ExtensionProperties,
) -> Result {
    Result::Success
}

#[no_mangle]
pub unsafe extern "C" fn vkDestroyInstance(
    instance_handle: InstanceHandle,
    _allocator: *const AllocationCallbacks,
) {
    if instance_handle.0.is_null() == false {
        instance_handle.into_instance();
    }
}

#[no_mangle]
pub unsafe extern "C" fn vk_icdGetInstanceProcAddr(
    _instance: InstanceHandle,
    function_name: FunctionName,
) -> GeneralFn {
    if function_name.0.is_null() {
        std::mem::transmute::<*const (), GeneralFn>(std::ptr::null())
    } else {
        let c_string = CStr::from_ptr(function_name.0);
        let rust_string = match c_string.to_str() {
            Ok(s) => s,
            Err(_) => return std::mem::transmute::<*const (), GeneralFn>(std::ptr::null()),
        };

        let pointer = match rust_string {
            "vkCreateInstance" => vkCreateInstance as *const (),
            "vkDestroyInstance" => vkDestroyInstance as *const (),
            "vkEnumerateInstanceExtensionProperties" => {
                vkEnumerateInstanceExtensionProperties as *const ()
            }
            _ => std::ptr::null(),
        };

        std::mem::transmute::<*const (), GeneralFn>(pointer)
    }
}
