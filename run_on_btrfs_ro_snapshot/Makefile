IMGFILE=btrfs_image.img
RTSNAPSHOT=@runtimesys
PROGNAME=init
BOOTPARAM=-nographic -append "console=ttyS0 root=/dev/vda rootflags=subvol=$(RTSNAPSHOT) ro ip=dhcp" -drive file=$(IMGFILE),format=raw,if=virtio -smp 2
BOOTPARAM+=-device virtio-net-pci,netdev=net0

USERNET=-netdev user,id=net0,hostfwd=tcp::7878-:7878
BRIDGE=-netdev bridge,id=net0,br=br0


all:
	cd ../common/ && make                # compile "init"
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
	cp -p ../common/target/x86_64-unknown-linux-musl/release/${PROGNAME} /mnt/sbin/init
	ldd /mnt/sbin/init  # debug
	if [ -e /mnt/$(RTSNAPSHOT) ]; then btrfs subvolume delete /mnt/$(RTSNAPSHOT); fi
	btrfs subvolume snapshot -r /mnt/ /mnt/$(RTSNAPSHOT)
	umount /mnt

run:
	qemu-system-x86_64 -kernel ../common/kernel/bzImage $(BOOTPARAM) $(USERNET) -m 64

run-bridge:
	sudo qemu-system-x86_64 -kernel ../common/kernel/bzImage $(BOOTPARAM) $(BRIDGE) -m 64

rundeb:
	qemu-system-x86_64 -kernel ../common/kernel/bzImage-debian12 $(BOOTPARAM) $(USERNET) -m 128

rundeb-bridge:
	sudo qemu-system-x86_64 -kernel ../common/kernel/bzImage-debian12 $(BOOTPARAM) $(BRIDGE) -m 128

clean:
	rm -f $(IMGFILE)
	cd ../common/ && cargo clean
