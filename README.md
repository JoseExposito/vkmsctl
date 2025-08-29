# vkmsctl

Command line tool and Rust crate to configure the VKMS Linux kernel driver.

It requires configfs support in VKMS ([not merged yet](https://lore.kernel.org/dri-devel/20250507135431.53907-1-jose.exposito89@gmail.com/)).

## Usage

```bash
$ vkmsctl --help
Command line tool to configure the VKMS Linux kernel driver

Usage: vkmsctl [OPTIONS] [COMMAND]

Commands:
  create  Create a new VKMS device
  list    List all VKMS devices
  remove  Remove a VKMS device
  help    Print this message or the help of the given subcommand(s)

Options:
      --configfs-path <CONFIGFS_PATH>  Directory were configfs is mounted [default: /sys/kernel/config]
  -h, --help                           Print help
  -V, --version                        Print version
```

### Create a new VKMS device

This command creates a new VKMS device from a JSON file:

```bash
$ sudo vkmsctl create tests/dev1.json
```

The structure of the JSON file is the following:

```json
{
  "name": "<Unique name for the device>",
  "enabled": true, // Whether the device is enabled or not
  "planes": [
    {
      "name": "<Unique name for the plane>",
      "type": "primary|overlay|cursor",
      "possible_crtcs": [
        "<Name of the CRTC>"
      ]
    },
    {
        // Other planes...
    }
  ],
  "crtcs": [
    {
      "name": "<Unique name for the CRTC>",
      "is_writeback_enabled": false // Whether a VKMS CRTC writeback connector is enabled or not
    },
    {
      // Other CRTCs...
    }
  ],
  "encoders": [
    {
      "name": "<Unique name for the encoder>",
      "possible_crtcs": [
        "<Name of the CRTC>"
      ]
    },
    {
      // Other encoders...
    }
  ],
  "connectors": [
    {
      "name": "<Unique name for the connector>",
      "status": "connected|disconnected|unknown",
      "possible_encoders": [
        "<Name of the encoder>"
      ]
    },
    {
      // Other connectors...
    }
  ]
}
```

For a more complete example, see [tests/dev1.json](tests/dev1.json).


### List all VKMS devices

This command lists all VKMS devices as they are currently in the filesystem in
JSON format:

```bash
$ vkmsctl list
[
    {
        // Information about the device as it is in the filesystem
    },
    // Other devices...
]
```


### Remove a VKMS device

This command disables and removes a VKMS device from the filesystem:

```bash
$ sudo vkmsctl remove <device name>
```
