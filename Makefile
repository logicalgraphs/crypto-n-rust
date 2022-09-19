# wow. Makefile. Wow. We went there.

DATA_DIR=$(RUST_BOOK)/data-files
SRC_DIR=$(RUST_BOOK)/src

SCRIPTS_DIR=$(SRC_DIR)/scripts

alles: fetch process
	@true

fetch: fetchers
	@echo "Didst fetcheth."

process: encvsify
	@echo "Didst processeth."

filter: filterify
	@echo "Can you? Can you trip like I do?"

# ----- LOADER AND REPORT DEPENDENCIES -----------------------------------

LE_DATE=$(shell date +%Y-%m-%d)

JSON_LISTING=$(DATA_DIR)/listings/listings-$(LE_DATE).json
CSV_LISTING=$(DATA_DIR)/csv/alles/listings-$(LE_DATE).csv
PORT_LISTING=$(DATA_DIR)/csv/portfolio/portfolio-$(LE_DATE).csv

HOLDINGS=$(DATA_DIR)/csv/portfolio_coins.lsv

LIST_CMD="listings/latest?start=1&limit=5000&convert=USD"

CURL_CMD=$(SCRIPTS_DIR)/curl-cmc.sh

RUN_RUST=cargo run

fetchers: $(JSON_LISTING)
	@echo "Loading e-coin listing file for $(LE_DATE) ..."; \
	$(CURL_CMD) cryptocurrency/$(LIST_CMD) $(JSON_LISTING)

$(JSON_LISTING): $(JSON_LISTING)
	@true

$(CSV_LISTING): $(CSV_LISTING) $(JSON_LISTING)
	@echo "enCVSing JSON quotes ..."; \
	cd $(SRC_DIR)/ch05/cmc_prices/; \
	$(RUN_RUST) $(JSON_LISTING) > $(CSV_LISTING)

encvsify: $(CSV_LISTING)
	@true

portfolio_listings: $(PORT_LISTING) $(CSV_LISTING) 
	@echo "filtering price-quotes to held assets..."; \
	cd $(SRC_DIR)/ch06/cmc_filter/; \
	$(RUN_RUST) $(CSV_LISTING) $(HOLDINGS) > $(PORT_LISTING)

filterify: portfolio_listings
	@true

# ----- ... and then we:

FORCE:

# ----- ... if need be. Eheh.
