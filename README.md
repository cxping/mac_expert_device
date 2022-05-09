# GET  Mac  Hardware overview 

- IOPlatformUUID

## Usage
To use this library, add the following to your Cargo.toml:

```toml

[dependencies]
mac_expert_device = "0.1"

```

Check out our [documentation](https://docs.rs/mac_expert_device/latest/x86_64-apple-darwin/mac_expert_device/all.html) for more details on how to use this library.


## Simple 

```
fn main() {
    let expert_device = mac_expert_device::os::IOPlatformExpertDevice::new().unwrap();
    println!("IOInterruptSpecifiers:{}", expert_device.io_interrupt_specifiers());
    println!("IOPolledInterface:{}", expert_device.io_polled_interface());
    println!("IOPlatformUuid:{}", expert_device.io_platform_uuid());
    println!("SerialNumber:{}", expert_device.serial_number());
    println!("PlatformFeature:{}", expert_device.platform_feature());
   //  println!("IOPlatformSystemSleepPolicy:{}",platform_expert_device.io_platform_system_sleep_policy());
    println!("IOBusyInterest:{}", expert_device.io_busy_interest());
    println!("TargetType:{}", expert_device.target_type());
    println!("IOInterruptControllers:{}", expert_device.io_interrupt_controllers());
    println!("Name:{}", expert_device.name());
    println!("Version:{}", expert_device.version());
    println!("Manufacturer:{}", expert_device.manufacturer());
    println!("Compatible:{}", expert_device.compatible());
    println!("ProductName:{}", expert_device.product_name());
    println!("IOPlatformSerialNumber:{}", expert_device.io_platform_serial_number());
    println!("IOConsoleSecurityInterest:{}", expert_device.io_console_security_interest());
    println!("ClockFrequency:{}", expert_device.clock_frequency());
    println!("Model:{}", expert_device.model());
    println!("BoardId:{}", expert_device.board_id());
    println!("SystemType:{}", expert_device.system_type());
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
