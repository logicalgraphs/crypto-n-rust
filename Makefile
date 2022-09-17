# wow. Makefile. Wow. We went there.

SCRIPTS_DIR=$(RUST_BOOK)/src/scripts

alles: fetch process
	true

fetch: fetchers
	@echo "Didst fetcheth."

process: encvsify
	@echo "Didst processeth."

# ----- LOADER AND REPORT DEPENDENCIES -----------------------------------

LE_DATE=$(shell date +%Y-%m-%d)

DATA_DIR=$(RUST_BOOK)/data-files

LISTING_FILE=$(DATA_DIR)/listings/listings-$(LE_DATE).json
OUTPUT_CSV_LISTING=$(DATA_DIR)/csv/listings-$(LE_DATE).csv

LIST_CMD="listings/latest?start=1&limit=5000&convert=USD"

CURL_CMD=$(RUST_BOOK)/src/scripts/curl-cmc.sh

fetchers: $(LISTING_FILE)
	@echo "Loading e-coin listing file for $(LE_DATE) ..."; \
	$(CURL_CMD) cryptocurrency/$(LIST_CMD) $(LISTING_FILE)

$(LISTING_FILE): $(LISTING_FILE)
	true

$(OUTPUT_CSV_LISTING): $(OUTPUT_CSV_LISTING)
	@echo "enCVSing JSON quotes ..."; \
	cd $(RUST_BOOK)/src/ch05/cmc_prices/; \
	cargo run $(LISTING_FILE) > $(OUTPUT_CSV_LISTING)

encvsify: $(OUTPUT_CSV_LISTING) $(LISTING_FILE)
	true

# ----- ... and then we:

FORCE:

# ----- ... if need be. Eheh.
