# GET  Mac  Hardware overview 
# 获取Mac 硬件概览信息 


- IOPlatformUUID



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

## Simple 

```
       let device = os::IOPlatformExpertDevice::new().unwrap();
       //get 
       let  uuid  = device.io_platform_uuid();
       println!("uuid=>{}",uuid);

```

