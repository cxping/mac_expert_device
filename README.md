# GET  Mac  Hardware overview 
# 获取Mac 硬件概览信息 


- IOPlatformUUID

## Usage
To use this library, add the following to your Cargo.toml:

```toml

[dependencies]
tree-sitter-stack-graphs = "0.1"

```

Check out our [documentation](https://docs.rs/mac_expert_device/latest/x86_64-apple-darwin/mac_expert_device/all.html) for more details on how to use this library.


## Simple 

```
fn main() {
   let platform_expert_device =  mac_expert_device::os::IOPlatformExpertDevice::new().unwrap();
   //
   println!("name:{}",platform_expert_device.name());
   println!("version:{}",platform_expert_device.version());
   println!("SerialNumber:{}",platform_expert_device.serial_number());
   println!("Manufacturer:{}",platform_expert_device.manufacturer());
   println!("IOBusyInterest:{}",platform_expert_device.io_busy_interest());
   println!("IOInterruptControllers:{}",platform_expert_device.io_interrupt_controllers());
   // println!("IOPlatformSystemSleepPolicy:{}",platform_expert_device.io_platform_system_sleep_policy());
   println!("IOPolledInterface:{}",platform_expert_device.io_polled_interface());
   println!("IOPlatformUuid:{}",platform_expert_device.io_platform_uuid());
}


```



## IOPlatformExpertDevice

```text
     +-o Root  <class IORegistryEntry, id xxxxxx, retain 17>
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
```
