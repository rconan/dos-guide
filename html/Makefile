.PHONY: all getting-started optical-model fem wind-loading

all: getting-started optical-model fem wind-loading

getting-started:
	cd $@/examples && cargo run --release

optical-model:
	cd $@/examples/gmt_optical_model && cargo run --release
	cd $@/examples/gmt_optical_sensor_model && cargo run --release

fem:
	cd $@/examples && cargo run --release

wind-loading:
	cd $@/examples && cargo run --release
