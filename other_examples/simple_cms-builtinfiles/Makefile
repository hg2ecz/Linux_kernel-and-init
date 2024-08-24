PROGNAME=init
BOOTPARAM=-nographic -append "console=ttyS0 rdinit=/sbin/init ip=dhcp" -initrd initrd.img -smp 2

# ip link add link enp3s0 name macvtap0 type macvtap mode bridge; ip link set macvtap0 up
#ETH=-netdev tap,id=tap0,ifname=tap0,script=no,downscript=no,vhost=on -device virtio-net-pci,netdev=tap0,mac=52:54:00:12:34:56

# For this *_eth version you need to create a br0 bridge width a dhcp server.
ETH=-netdev bridge,id=net0,br=br0 -device virtio-net-pci,netdev=net0

all:
	cargo install diesel_cli --no-default-features --features mysql
	@echo MANUAL_CMD: diesel setup --database-url mysql://test:testpwd@[fd01::d250:99ff:fe59:ce12]/test
	@echo MANUAL_CMD: diesel migration run --database-url mysql://test:testpwd@[fd01::d250:99ff:fe59:ce12]/test

	#rustup target add x86_64-unknown-linux-musl
	#cargo build --release --target x86_64-unknown-linux-musl
	cargo build --release

	mkdir -p initrd/sbin initrd/proc initrd/lib initrd/run
	ln -sf lib initrd/lib64

	#cp -p /lib/linux-vdso.so.1 initrd/lib/ -- from kernel
	cp -p /lib/x86_64-linux-gnu/libgcc_s.so.1 initrd/lib/libgcc_s.so.1
	cp -p /lib/x86_64-linux-gnu/libc.so.6 initrd/lib/
	cp -p /lib/x86_64-linux-gnu/libm.so.6 initrd/lib/
	cp -p /lib64/ld-linux-x86-64.so.2 initrd/lib/

	cp -p target/release/${PROGNAME} initrd/sbin/init
	cd initrd && find . | cpio -H newc -o | zstd > ../initrd.img && cd ..

run:
	qemu-system-x86_64 -kernel ../../common/kernel/bzImage $(BOOTPARAM) -m 64

run_eth:
	sudo qemu-system-x86_64 -kernel ../../common/kernel/bzImage $(BOOTPARAM) $(ETH) -m 64

clean:
	rm -rf initrd initrd.img
	cargo clean
