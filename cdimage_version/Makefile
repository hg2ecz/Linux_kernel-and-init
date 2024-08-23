IMGFILE=cdimage.iso
PROGNAME=init
BOOTPARAM=-nographic -append "console=ttyS0 root=/dev/vda ro ip=dhcp" -drive file=$(IMGFILE),format=raw,if=virtio -smp 2

# ip link add link enp3s0 name macvtap0 type macvtap mode bridge; ip link set macvtap0 up
#ETH=-netdev tap,id=tap0,ifname=tap0,script=no,downscript=no,vhost=on -device virtio-net-pci,netdev=tap0,mac=52:54:00:12:34:56

# For this *_eth version you need to create a br0 bridge width a dhcp server.
ETH=-netdev bridge,id=net0,br=br0 -device virtio-net-pci,netdev=net0


all:
	cd ../common/ && make && cd ../btrfs_version     # compile "init"
	mkdir -p cdimage/sbin cdimage/proc
	cp -p ../common/target/x86_64-unknown-linux-musl/release/${PROGNAME} cdimage/sbin/init
	mkisofs -input-charset utf-8 -o $(IMGFILE) -V "Test CD" -R cdimage/

run:
	qemu-system-x86_64 -kernel ../common/kernel/bzImage $(BOOTPARAM) -m 64

run_eth:
	sudo qemu-system-x86_64 -kernel ../common/kernel/bzImage $(BOOTPARAM) $(ETH) -m 64

rundeb:
	qemu-system-x86_64 -kernel ../common/kernel/bzImage-debian12 $(BOOTPARAM) -m 128

rundeb_eth:
	sudo qemu-system-x86_64 -kernel ../common/kernel/bzImage-debian12 $(BOOTPARAM) $(ETH) -m 128

clean:
	rm -rf cdimage $(IMGFILE)
	cd ../common/ && cargo clean
