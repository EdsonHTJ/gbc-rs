


ifeq ($(shell uname),Darwin)
	OS := macos
else
	OS := linux
endif

# ifeq ($(OS),macos)
# 	export LIBRARY_PATH=/opt/homebrew/lib:$LIBRARY_PATH
# 	export CPATH=/opt/homebrew/include:$CPATH
# 	export PATH=/opt/homebrew/bin:$PATH
# endif

all:
	echo "Building on $(OS)"
	cargo build

run:
	cargo run

