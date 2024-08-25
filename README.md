# Linux kernel and /sbin/init only system

## Whats this?

This is a minimal example, how can running system with only a Linux kernel and single ELF binary.

## Why?

* This use a minimal resources. It can running less than 45 MB RAM.
* You can create a secure architecture, because of the single /sbin/init capable system. So this system can't execute reverse shell, etc., because it does't contains any binaries and the system is readonly.

For example:

![Isolated system](initonly_as_isolator.png)
