use ash::{vk, Instance, InstanceError};

pub struct EntryPortability;

extern "system" fn vkGetInstanceProcAddr(
    instance: vk::Instance,
    p_name: *const std::os::raw::c_char,
) -> vk::PFN_vkVoidFunction {
    use std::mem::transmute;
    unsafe {
        let f = portability::vkGetInstanceProcAddr(transmute(instance), p_name);
        transmute(f)
    }
}
impl EntryPortability {
    pub fn load_instance() -> Result<Instance, InstanceError> {
        unsafe {
            let static_fn = vk::StaticFn {
                get_instance_proc_addr: vkGetInstanceProcAddr,
            };
            Ok(Instance::load(&static_fn, vk::Instance::null()))
        }
    }
}
