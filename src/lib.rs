#[cfg(test)]
mod tests {
    use crate::os;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn get_uuid_test() {
       let device = os::IOPlatformExpertDevice::new().unwrap();
       let  uuid  = device.io_platform_uuid();
       println!("uuid=>{}",uuid);
    }
}

//windows, macos(unix), ios, linux, android,

#[cfg(target_os = "windows")]
mod os {}

#[cfg(target_os = "linux")]
mod os {}

// #[derive(Debug)]
// struct SystemInfo {
//     uuid: String,
//     hardware: String,
// }

#[cfg(target_os = "macos")]
pub mod os {
    use std::process::Command;

    #[derive(Debug)]
    pub struct IOPlatformExpertDevice {
        io_interrupt_specifiers: String,
        io_polled_interface: String,
        io_platform_uuid: String,
        serial_number: String,
        platform_feature: String,
        io_platform_system_sleep_policy: String,
        io_busy_interest: String,
        target_type: String,
        io_interrupt_controllers: String,
        name: String,
        version: String,
        manufacturer: String,
        compatible: String,
        product_name: String,
        io_platform_serial_number: String,
        io_console_security_interest: String,
        clock_frequency: String,
        model: String,
        board_id: String,
        system_type: String,
    }

    impl IOPlatformExpertDevice {
        pub fn from(s: String) -> Option<Self> {
            if !s.is_empty() {
                return None;
            }
            Some(s.into())
        }
        /// new create  IOPlatformExpertDevice
        ///
        /// # Examples
        ///
        /// ```
        /// let expertDevice = mac_expert_device::os::IOPlatformExpertDevice::new();
        ///  println!("Platform UUID:{}",expertDevice.io_platform_uuid());
        /// 
        /// ```
        pub fn new() -> Option<Self> {
            let output = match io_platform_expert_device() {
                Some(s) =>{
                    s.into()
                },
                None =>  return None,
            };
            Some(output)
        }
        /// io_polled_interface
        /// IOPolledInterface()
        pub fn io_polled_interface(&self) -> String {
            self.io_polled_interface.to_string()
        }
        ///IOPlatformUuid
        /// io_platform_uuid()
        pub fn io_platform_uuid(&self) -> String {
            self.io_platform_uuid.to_string()
        }
        /// SerialNumber
        /// serial_number()
        pub fn serial_number(&self) -> String {
            self.serial_number.to_string()
        }
        /// PlatformFeature
        /// platform_feature()
        pub fn platform_feature(&self) -> String {
            self.platform_feature.to_string()
        }

        /// IOPlatformSystemSleepPolicy
        /// io_platform_system_sleep_policy()
        pub fn io_platform_system_sleep_policy(&self) -> String {
            self.io_platform_system_sleep_policy.to_string()
        }

        /// IOBusyInterest
        /// io_busy_interest()
        pub fn io_busy_interest(&self) -> String {
            self.io_busy_interest.to_string()
        }
        /// IOInterruptControllers
        /// io_interrupt_controllers()
        pub fn io_interrupt_controllers(&self) -> String {
            self.io_interrupt_controllers.to_string()
        }
        /// Name
        /// name()
        pub fn name(&self) -> String {
            self.name.to_string()
        }
        /// Version
        /// version()
        pub fn version(&self) -> String {
            self.version.to_string()
        }
        /// Manufacturer
        ///
        /// # Examples
        ///
        /// ```
        /// let expertDevice = mac_expert_device::os::IOPlatformExpertDevice::new();
        ///  println!("io_polled_interface UUID:{}",expertDevice.io_polled_interface());
        /// 
        /// ```
        pub fn manufacturer(&self) -> String {
            self.manufacturer.to_string()
        }
    }

    /**
    *
    *
    * +-o Root  <class IORegistryEntry, id xxxxxx, retain 17>
     +-o xxxxxx  <class IOPlatformExpertDevice, id xxxxxx, registered, matched, active, busy 0 (13368 ms), retain 44>
         {
           "IOInterruptSpecifiers" = (<xxxxx>)
           "IOPolledInterface" = "SMCPolledInterface is not serializable"
           "IOPlatformUUID" = "xxxxxx"
           "serial-number" = <xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx>
           "platform-feature" = <xxxxxx>
           "IOPlatformSystemSleepPolicy" = <xxxxxx>
           "IOBusyInterest" = "IOCommand is not serializable"
           "target-type" = <"Mac">
           "IOInterruptControllers" = ("xxxxxx")
           "name" = <"/">
           "version" = <"xxxxxx">
           "manufacturer" = <"Apple Inc.">
           "compatible" = <"xxxxxx">
           "product-name" = <"xxxxxx">
           "IOPlatformSerialNumber" = "xxxxxx"
           "IOConsoleSecurityInterest" = "IOCommand is not serializable"
           "clock-frequency" = <xxxxx>
           "model" = <"xx">
           "board-id" = <"xxxxxx">
           "system-type" = <02>
         }
    *
    */

    impl Default for IOPlatformExpertDevice {
        fn default() -> Self {
            Self {
                io_interrupt_specifiers: Default::default(),
                io_polled_interface: Default::default(),
                io_platform_uuid: Default::default(),
                serial_number: Default::default(),
                platform_feature: Default::default(),
                io_platform_system_sleep_policy: Default::default(),
                io_busy_interest: Default::default(),
                target_type: Default::default(),
                io_interrupt_controllers: Default::default(),
                name: Default::default(),
                version: Default::default(),
                manufacturer: Default::default(),
                compatible: Default::default(),
                product_name: Default::default(),
                io_platform_serial_number: Default::default(),
                io_console_security_interest: Default::default(),
                clock_frequency: Default::default(),
                model: Default::default(),
                board_id: Default::default(),
                system_type: Default::default(),
            }
        }
    }

    fn parse(s: String) -> Vec<String> {
        let mut chars_stack = Vec::<char>::new();
        let mut token_stack = Vec::<String>::new();
        let mut in_quotes: Option<char> = None;
        let mut in_brackets = false;
        let mut in_quote = false;
        for ch in s.chars() {
            if let Some(quote) = in_quotes {
                if ch == quote {
                    let previous_char = *chars_stack
                        .last()
                        .expect("cannot get the last char in chars stack");
                    if previous_char != '\\' {
                        in_quotes = None;
                    }
                }
                chars_stack.push(ch);
            } else {
                match ch {
                    '<' => {
                        in_brackets = true;
                        if !chars_stack.is_empty() {
                            chars_stack = Vec::new();
                        }
                    }
                    '>' => {
                        in_brackets = false;
                        let tag_text = String::from_iter(chars_stack);
                        chars_stack = Vec::new();
                        let tag = tag_text.clone();
                        token_stack.push(tag.clone());
                    }
                    '\"' => {
                        if in_quote {
                            let tag_text = String::from_iter(chars_stack);
                            chars_stack = Vec::new();
                            let tag = tag_text.clone();
                            token_stack.push(tag.clone());
                        } else if !in_brackets && in_quote == false {
                            in_quote = true;
                            chars_stack = Vec::new();
                        }
                    }
                    _ => {
                        if in_brackets {
                            match ch {
                                '\'' => in_quotes = Some('\''),
                                '\"' => in_quotes = Some('\"'),
                                _ => {}
                            }
                        }
                        chars_stack.push(ch)
                    }
                }
            }
        }
        token_stack
    }

    impl From<String> for IOPlatformExpertDevice {
        fn from(s: String) -> Self {
            let st: Vec<&str> = s.split('\n').collect();
            let mut entry = IOPlatformExpertDevice::default();
            for ele in st.iter() {
                let sp = ele.split_once("=");
                let (s1, s2) = match sp {
                    Some((s1, s2)) => (s1.to_string(), s2.to_string()),
                    None => {
                        continue;
                    }
                };
                if s2.is_empty() {
                    continue;
                }
                let v = parse(s2.clone());
                if s1.contains("IOInterruptSpecifiers") {
                    if v.is_empty() {
                        entry.io_interrupt_specifiers = s2.to_string()
                    } else {
                        entry.io_interrupt_specifiers = v.get(0).unwrap().to_string()
                    }
                } else if s1.contains("IOPolledInterface") {
                    if v.is_empty() {
                        entry.io_polled_interface = s2.to_string()
                    } else {
                        entry.io_polled_interface = v.get(0).unwrap().to_string()
                    }
                } else if s1.contains("IOPlatformUUID") {
                    if v.is_empty() {
                        entry.io_platform_uuid = s2.to_string()
                    } else {
                        entry.io_platform_uuid = v.get(0).unwrap().to_string()
                    }
                } else if s1.contains("serial-number") {
                    if v.is_empty() {
                        entry.serial_number = s2.to_string()
                    } else {
                        entry.serial_number = v.get(0).unwrap().to_string()
                    }
                } else if s1.contains("platform-feature") {
                    if v.is_empty() {
                        entry.platform_feature = s2.to_string()
                    } else {
                        entry.platform_feature = v.get(0).unwrap().to_string()
                    }
                } else if s1.contains("OPlatformSystemSleepPolicy") {
                    if v.is_empty() {
                        entry.io_platform_system_sleep_policy = s2.to_string()
                    } else {
                        entry.io_platform_system_sleep_policy = v.get(0).unwrap().to_string()
                    }
                } else if s1.contains("IOBusyInterest") {
                    if v.is_empty() {
                        entry.io_busy_interest = s2.to_string()
                    } else {
                        entry.io_busy_interest = v.get(0).unwrap().to_string()
                    }
                } else if s1.contains("target-type") {
                    if v.is_empty() {
                        entry.target_type = s2.to_string()
                    } else {
                        entry.target_type = v.get(0).unwrap().to_string()
                    }
                } else if s1.contains("IOInterruptControllers") {
                    if v.is_empty() {
                        entry.io_interrupt_controllers = s2.to_string()
                    } else {
                        entry.io_interrupt_controllers = v.get(0).unwrap().to_string()
                    }
                } else if s1.contains("name") {
                    if v.is_empty() {
                        entry.name = s2.to_string()
                    } else {
                        entry.name = v.get(0).unwrap().to_string()
                    }
                } else if s1.contains("version") {
                    if v.is_empty() {
                        entry.version = s2.to_string()
                    } else {
                        entry.version = v.get(0).unwrap().to_string()
                    }
                } else if s1.contains("manufacturer") {
                    if v.is_empty() {
                        entry.manufacturer = s2.to_string()
                    } else {
                        entry.manufacturer = v.get(0).unwrap().to_string()
                    }
                } else if s1.contains("compatible") {
                    if v.is_empty() {
                        entry.compatible = s2.to_string()
                    } else {
                        entry.compatible = v.get(0).unwrap().to_string()
                    }
                } else if s1.contains("product-name") {
                    if v.is_empty() {
                        entry.product_name = s2.to_string()
                    } else {
                        entry.product_name = v.get(0).unwrap().to_string()
                    }
                } else if s1.contains("IOPlatformSerialNumber") {
                    if v.is_empty() {
                        entry.io_platform_serial_number = s2.to_string()
                    } else {
                        entry.io_platform_serial_number = v.get(0).unwrap().to_string()
                    }
                } else if s1.contains("IOConsoleSecurityInterest") {
                    if v.is_empty() {
                        entry.io_console_security_interest = s2.to_string()
                    } else {
                        entry.io_console_security_interest = v.get(0).unwrap().to_string()
                    }
                } else if s1.contains("clock-frequency") {
                    if v.is_empty() {
                        entry.clock_frequency = s2.to_string()
                    } else {
                        entry.clock_frequency = v.get(0).unwrap().to_string()
                    }
                } else if s1.contains("model") {
                    if v.is_empty() {
                        entry.model = s2.to_string()
                    } else {
                        entry.model = v.get(0).unwrap().to_string()
                    }
                } else if s1.contains("board-id") {
                    if v.is_empty() {
                        entry.board_id = s2.to_string()
                    } else {
                        entry.board_id = v.get(0).unwrap().to_string()
                    }
                } else if s1.contains("system-type") {
                    if v.is_empty() {
                        entry.system_type = s2.to_string()
                    } else {
                        entry.system_type = v.get(0).unwrap().to_string()
                    }
                }
            }
            entry
        }
    }
     fn io_platform_expert_device() -> Option<String> {
        let out = Command::new("ioreg")
            .arg("-d2")
            .arg("-c")
            .arg("IOPlatformExpertDevice")
            .output()
            .expect("failed to execute process");
        let output = String::from_utf8(out.stdout);
        Some(output.unwrap())
    }
    pub fn customize_service(service:&str) -> Option<String> {
        let out = Command::new("ioreg")
            .arg("-d2")
            .arg("-c")
            .arg(service)
            .output()
            .expect("failed to execute process");
        let output = String::from_utf8(out.stdout);
        Some(output.unwrap())
    }
}
