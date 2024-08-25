# Linux kernel and /sbin/init only system

## Whats this?

This is a minimal example, how can running system with only a Linux kernel and single ELF binary.

## Why?

* This use a minimal resources. It can running less than 45 MB RAM.
* You can create a secure architecture, because of the single /sbin/init capable system. So this system can't execute reverse shell, etc., because it does't contains any binaries and the system is readonly.
* The Linux distributions are very complex and weekly comes security upgrades. Security updates are very rare here, because of minimalist system.

For example:

![Isolated system](initonly_as_isolator.png)

## What is sysdiag?

It serve some status information in a TCP port.

    $ echo -e "mounts\n loadavg\n proc\n meminfo" | nc IPv4_or_IPv6_addr 7878

For shutdown this VM:

    $ echo "pwroff" | nc IPv4_or_IPv6_addr 7878
