IMGFILE=btrfs_image.img
RTSNAPSHOT=@runtimesys
PROGNAME=init
BOOTPARAM=-nographic -append "console=ttyS0 root=/dev/vda rootflags=subvol=$(RTSNAPSHOT) ro ip=dhcp" -drive file=$(IMGFILE),format=raw,if=virtio

# ip link add link enp3s0 name macvtap0 type macvtap mode bridge; ip link set macvtap0 up
#ETH=-netdev tap,id=tap0,ifname=tap0,script=no,downscript=no,vhost=on -device virtio-net-pci,netdev=tap0,mac=52:54:00:12:34:56

# For this *_eth version you need to create a br0 bridge width a dhcp server.
ETH=-netdev bridge,id=net0,br=br0 -device virtio-net-pci,netdev=net0


all:
	rustup target add x86_64-unknown-linux-musl
	cargo build --release --target x86_64-unknown-linux-musl
	@echo "make firstcreate - for new image"
	@if [ -e $(IMGFILE) ]; then sudo make copyinit_as_root; fi

firstcreate:
	if [ $(shell mountpoint -q /mnt; echo $$?) -eq 0 ]; then sudo umount /mnt; fi
	dd if=/dev/zero of=$(IMGFILE) bs=1M count=109  # 109 is the btrfs minimum size
	/sbin/mkfs.btrfs $(IMGFILE)
	sudo make firstcreate_as_root

copyinit:
	sudo make copyinit_as_root

firstcreate_as_root:
	mount -o loop $(IMGFILE) /mnt
	mkdir -p /mnt/sbin /mnt/proc
	make copyinit_as_root

copyinit_as_root:
	if [ $(shell mountpoint -q /mnt; echo $$?) -ne 0 ]; then mount -o loop $(IMGFILE) /mnt; fi
	cp -p target/x86_64-unknown-linux-musl/release/${PROGNAME} /mnt/sbin/init
	if [ -e /mnt/$(RTSNAPSHOT) ]; then btrfs subvolume delete /mnt/$(RTSNAPSHOT); fi
	btrfs subvolume snapshot -r /mnt/ /mnt/$(RTSNAPSHOT)
	umount /mnt

run:
	qemu-system-x86_64 -kernel kernel/bzImage $(BOOTPARAM) -smp 2 -m 64

run_eth:
	sudo qemu-system-x86_64 -kernel kernel/bzImage $(BOOTPARAM) $(ETH) -smp 2 -m 64

rundeb:
	qemu-system-x86_64 -kernel kernel/bzImage-debian12 $(BOOTPARAM) -smp 2 -m 128

rundeb_eth:
	sudo qemu-system-x86_64 -kernel kernel/bzImage-debian12 $(BOOTPARAM) $(ETH) -smp 2 -m 128

clean:
	cargo clean
	rm -f $(IMGFILE)
