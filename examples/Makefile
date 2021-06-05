.PHONY: all getting-started optical-model fem wind-loading
GMT_MODES_PATH := /home/rconan/projects/crseo/CEO/gmtMirrors/

all: getting-started optical-model fem wind-loading

getting-started:
	cd $@ && cargo run --release

optical-model:
	cd $@/gmt_optical_model && \
		export GMT_MODES_PATH=/home/rconan/projects/crseo/CEO/gmtMirrors/ && \
		cargo run --release
	cd $@/gmt_optical_sensor_model && \
		export GMT_MODES_PATH=/home/rconan/projects/crseo/CEO/gmtMirrors/ && \
		cargo run --release

fem:
	cd $@ && cargo run --release

wind-loading:
	cd $@ && cargo run --release
