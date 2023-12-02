# `linux-sed-opal-sys`

Low-level Rust bindings to the `linux/sed-opal.h` UAPI header.

As Linux extends the list of available ioctls time to time, `linux-sed-opal-sys` allows
you to specify the Linux version you want to create ioctl wrappers for
using its feature flags.
For example, to support the ioctls available in Linux 6.3, specify
`features = [ "linux_6_1" ]` which is the feature flag with the next lower version number.

The lowess supported version of Linux is 6.0.

linux-sed-opal-sys uses the Linux headers of your system to build its
wrapper.
Whether they should be vendored or something is open for discussion.
