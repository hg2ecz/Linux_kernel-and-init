PROGNAME=init
BOOTPARAM=-nographic -append "console=ttyS0 rdinit=/sbin/init ip=dhcp"

# ip link add link enp3s0 name macvtap0 type macvtap mode bridge; ip link set macvtap0 up
#ETH=-netdev tap,id=tap0,ifname=tap0,script=no,downscript=no,vhost=on -device virtio-net-pci,netdev=tap0,mac=52:54:00:12:34:56

# For this *_eth version you need to create a br0 bridge width a dhcp server.
ETH=-netdev bridge,id=net0,br=br0 -device virtio-net-pci,netdev=net0

all:
	rustup target add x86_64-unknown-linux-musl
	cargo build --release --target x86_64-unknown-linux-musl
	mkdir -p initrd/sbin initrd/proc
	cp -p target/x86_64-unknown-linux-musl/release/${PROGNAME} initrd/sbin/init
	cd initrd && find . | cpio -H newc -o | zstd > ../initrd.img && cd ..

run:
	qemu-system-x86_64 -kernel kernel/bzImage -initrd initrd.img $(BOOTPARAM) -smp 2 -m 64

run_eth:
	sudo qemu-system-x86_64 -kernel kernel/bzImage -initrd initrd.img $(BOOTPARAM) $(ETH) -smp 2 -m 64

rundeb:
	qemu-system-x86_64 -kernel kernel/bzImage-debian12 -initrd initrd.img $(BOOTPARAM) -smp 2 -m 128

rundeb_eth:
	sudo qemu-system-x86_64 -kernel kernel/bzImage-debian12 -initrd initrd.img $(BOOTPARAM) $(ETH) -smp 2 -m 128

clean:
	cargo clean
	rm -rf initrd initrd.img
