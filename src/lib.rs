#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/fdkaac.rs"));


#[cfg(test)]
mod test {
    use super::*;
    
    use std::ffi::CStr;


    #[derive(Debug, PartialEq, Eq)]
    pub struct Version {
        pub title: String,
        pub build_date: String,
        pub build_time: String,
        pub module_id: u32,
        pub version: i32,
        pub flags: u32,
        pub version_string: String,
    }
    
    #[test]
    fn test_encoder_version() {
        unsafe {
            let mut encoder_info: LIB_INFO = LIB_INFO::default();
            assert_eq!(aacEncGetLibInfo(&mut encoder_info as *mut LIB_INFO), AACENC_OK);

            let encoder_version = Version {
                title: CStr::from_ptr(encoder_info.title).to_str().unwrap().to_string(),
                build_date: CStr::from_ptr(encoder_info.build_date).to_str().unwrap().to_string(),
                build_time: CStr::from_ptr(encoder_info.build_time).to_str().unwrap().to_string(),
                module_id: encoder_info.module_id,
                version: encoder_info.version,
                flags: encoder_info.flags,
                version_string: CStr::from_ptr(&encoder_info.versionStr as *const i8)
                                    .to_str().unwrap().to_string(),
            };

            assert_eq!(encoder_version,
                       Version {
                            title: "FDK Tools".to_string(),
                            build_date: "Nov  6 2018".to_string(),
                            build_time: "12:14:52".to_string(),
                            module_id: 1,
                            version: 33752576,
                            flags: 0,
                            version_string: "2.3.6".to_string()
                        });
        }
        
    }

    #[test]
    fn test_decoder_version() {
        unsafe {
            let mut decoder_info: LIB_INFO = LIB_INFO::default();
            assert_eq!(aacDecoder_GetLibInfo(&mut decoder_info as *mut LIB_INFO), AAC_DEC_OK as i32);

            let decoder_version = Version {
                title: CStr::from_ptr(decoder_info.title).to_str().unwrap().to_string(),
                build_date: CStr::from_ptr(decoder_info.build_date).to_str().unwrap().to_string(),
                build_time: CStr::from_ptr(decoder_info.build_time).to_str().unwrap().to_string(),
                module_id: decoder_info.module_id,
                version: decoder_info.version,
                flags: decoder_info.flags,
                version_string: CStr::from_ptr(&decoder_info.versionStr as *const i8).to_str().unwrap().to_string(),
            };

            assert_eq!(decoder_version,
                       Version {
                            title: "SBR Decoder".to_string(),
                            build_date: "Nov  6 2018".to_string(),
                            build_time: "12:13:32".to_string(),
                            module_id: 5,
                            version: 33688576,
                            flags: 63,
                            version_string: "2.2.12".to_string()
                        });
        }
    }
}