PROFILE=release
TARGET=scop
CARGO_TARGET=target/$(PROFILE)/scop
CARGO=cargo
ifeq ($(PROFILE),release)
CARGO_BUILD=$(CARGO) build --release
else
CARGO_BUILD=$(CARGO) build
endif

all: $(TARGET)

.PHONY: $(TARGET)
$(TARGET):
	$(CARGO_BUILD)
	mv $(CARGO_TARGET) $(TARGET)

.PHONY: clean
clean:
	rm -rf target

.PHONY: fclean
fclean:
	$(MAKE) clean
	rm -rf $(TARGET)

.PHONY: re
re:
	$(MAKE) fclean
	$(MAKE) all
