BIN_FOLDER=bin

%: bin/% data/%.in
	$^

$(BIN_FOLDER):
	mkdir -p $(BIN_FOLDER)

bin/%: src/day%.rs | $(BIN_FOLDER)
	rustc -o $@ $<
