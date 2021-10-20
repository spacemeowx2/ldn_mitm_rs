.PHONY: all clean

all:
	@cd ldn_mitm; cargo nx --release
	@rm -rf $(CURDIR)/SdOut
	@mkdir -p $(CURDIR)/SdOut/atmosphere/contents/4200000000000010/flags
	@touch $(CURDIR)/SdOut/atmosphere/contents/4200000000000010/flags/boot2.flag
	@cp $(CURDIR)/ldn_mitm/target/aarch64-none-elf/release/ldn_mitm.nsp $(CURDIR)/SdOut/atmosphere/contents/4200000000000010/exefs.nsp
	@cd $(CURDIR)/SdOut; zip -r ../ldn_mitm_sdcard.zip atmosphere

doc:
	@cd ./ldn_mitm/; cargo doc --target aarch64-unknown-none
	@cd ./ldn_mitm/target/aarch64-unknown-none/doc/; python3 -m http.server

clean:
	@rm ldn_mitm_sdcard.zip
	@rm -rf $(CURDIR)/SdOut
	@cd ldn_mitm; cargo clean
