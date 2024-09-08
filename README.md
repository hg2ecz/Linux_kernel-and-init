# Linux kernel and /sbin/init only system

This is a minimal example, how can you run a system with only a Linux kernel and single ELF binary.

## Why?

* This uses minimal resources. It can run with less than 45 MB RAM.
* The `/sbin/init` acts as full user space software written in Rust --> fast, efficient and secure.
* The architecture is more secure because of this single `/sbin/init` system: one can't execute a reverse shell, etc., because it does't contain any binaries for that, and the system is read-only.
* The Linux distributions are very complex with weekly security upgrades. A machine running this minimal system will inherently have a smaller attack surface, thus, updates are less critical (though still important). 
* The start time is very fast, from the start of booting the Linux kernel to "Hello, world from Rust!" it takes only 1.083 seconds on my laptop with i7-1165G7 CPU in Qemu and an `initrd` environment.

Here is a block diagram of the secure architecture:

![Isolated system](initonly_as_isolator.png)

## What is `sysdiag`?

It serves some status information over a TCP port.

    $ echo -e "mounts\n loadavg\n proc\n meminfo" | nc IPv4_or_IPv6_addr 7878

To shut this VM down:

    $ echo "pwroff" | nc IPv4_or_IPv6_addr 7878

## If you don't have a SLAAC router and DHCPv4 support

In one of my tests I use the `docker0` bridge interface for `qemu`. You can connect via the link local interface, but you have to give the name of interface.

    $ echo -e "meminfo\n quit" | nc fe80::5054:ff:fe12:3456%docker0 7878

For IPv4 DHCP, the `udhcpd` is the simplest way on the host Linux.

    $ sudo apt install udhcpd
    # edit /etc/udhcpd/conf (interface docker0) and the /etc/default/udhcp (no -> yes)
    $ sudo systemctl restart udhcpd
