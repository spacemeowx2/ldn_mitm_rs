.PHONY: all clean

all:
	@cd ldn_mitm; cargo nx --release
	@rm -rf $(CURDIR)/SdOut
	@mkdir -p $(CURDIR)/SdOut/atmosphere/contents/4200000000000010/flags
	@touch $(CURDIR)/SdOut/atmosphere/contents/4200000000000010/flags/boot2.flag
	@cp $(CURDIR)/ldn_mitm/target/aarch64-none-elf/release/ldn_mitm.nsp $(CURDIR)/SdOut/atmosphere/contents/4200000000000010/exefs.nsp

clean:
	@rm -rf $(CURDIR)/SdOut
	@cd ldn_mitm; cargo clean
