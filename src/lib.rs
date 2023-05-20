use std::ffi::{c_char, c_void, CStr};

#[repr(transparent)]
pub struct FunctionName(*const c_char);

#[repr(transparent)]
pub struct Instance(*mut c_void);

type GeneralFn = unsafe extern "C" fn();

#[repr(C)]
pub struct ApplicationInformation {
    structure_type: u32,
    next_structure: *const c_void,
    application_name: *const c_char,
    application_version: u32,
    engine_name: *const c_char,
    engine_version: u32,
    api_version: u32,
}

#[repr(C)]
pub struct InstanceCreateInformation {
    structure_type: u32,
    next_structure: *const c_void,
    flags: u32,
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

#[no_mangle]
pub unsafe extern "C" fn vkCreateInstance(
    _create_information: *const InstanceCreateInformation,
    _allocator: *const AllocationCallbacks,
    _instance_result: *mut Instance,
) -> u32 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn vkEnumerateInstanceExtensionProperties(
    _layer_name: *const c_char,
    _property_count: *mut u32,
    _properties: *mut ExtensionProperties,
) -> u32 {
    0
}

#[no_mangle]
pub unsafe extern "C" fn vk_icdGetInstanceProcAddr(
    _instance: Instance,
    function_name: FunctionName,
) -> Option<GeneralFn> {
    if function_name.0.is_null() {
        return None;
    }

    let c_string = CStr::from_ptr(function_name.0);
    let rust_string = match c_string.to_str() {
        Ok(s) => s,
        Err(_) => return None,
    };

    println!("CALLING : {}", rust_string);
    match rust_string {
        "vkCreateInstance" => {
            let pointer = vkCreateInstance as *const ();
            Some(std::mem::transmute::<*const (), GeneralFn>(pointer))
        }
        "vkEnumerateInstanceExtensionProperties" => {
            let pointer = vkEnumerateInstanceExtensionProperties as *const ();
            Some(std::mem::transmute::<*const (), GeneralFn>(pointer))
        }
        _ => None,
    }
}
