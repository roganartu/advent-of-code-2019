FOLDER=day${day}

all:
	cargo new ${FOLDER}
	cp -f skeleton/src/main.rs ${FOLDER}/src/main.rs
	mkdir ${FOLDER}/input
	touch ${FOLDER}/input/input.txt
	touch ${FOLDER}/input/example.txt

run:
	cd ${FOLDER} && cargo run --release < input/input.txt

test:
	cd ${FOLDER} && cargo run --release < input/example.txt
