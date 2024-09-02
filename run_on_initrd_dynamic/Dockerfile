FROM scratch

COPY initrd/lib/   /lib/
COPY initrd/lib64/ /lib64/

COPY initrd/sbin/  /sbin/
ENTRYPOINT ["/sbin/init"]
