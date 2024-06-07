# PiDucky

Raspberry PI Zero W Duckyscript Payload interpreter using USBHID to execute the payload.

## Requirements

- RaspberryPI Zero
- USB Hat with HID Protocol support.
- [USBHID](https://github.com/sammwyy/usbhid)

## How to use

### Load instruction set from file

```
./piducky instructions.txt
```

### Inline instructions (From CLI)

```
./piducky --payload <instruction_here>
```

### Interactive mode

```
./piducky -i
$ <instruction_here>
$ <instruction_here>
$ <instruction_here>
```
