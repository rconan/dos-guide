.PHONY: all getting-started_dos-interface optical-model_dos-interface

all: getting-started_dos-interface optical-model_dos-interface

getting-started_dos-interface:
	cd $@ && cargo run --release

optical-model_dos-interface:
	cd $@/gmt_optical_model && cargo run --release
	cd $@/gmt_optical_sensor_model && cargo run --release
