use super::Config;

impl Config {
    pub fn new() -> Config {
        Config {
            z3_cfg: unsafe {z3_sys::Z3_mk_config()},
            _private: (),
        }
    }

    pub fn set_param (&mut self, key: &str, value: &str) -> Result<&mut Self, std::ffi::NulError> {
        let key = std::ffi::CString::new(key)?;
        let value = std::ffi::CString::new(value)?;
        unsafe {
            z3_sys::Z3_set_param_value(self.z3_cfg, key.as_ptr(), value.as_ptr());
        }
        Ok(self)
    }    
}

impl Drop for Config {
    fn drop(&mut self) {
        unsafe { z3_sys::Z3_del_config(self.z3_cfg) };
    }
}