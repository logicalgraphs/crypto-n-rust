# wow. Makefile. Wow. We went there.

DATA_DIR=$(RUST_BOOK)/data-files
SRC_DIR=$(RUST_BOOK)/src

SCRIPTS_DIR=$(SRC_DIR)/scripts

alles: fetch process filter
	@true

fetch: fetchers
	@echo "Didst fetcheth."

process: encsvify
	@echo "Didst processeth."

filter: filterify
	@echo "Can you? Can you trip like I do?"

bases: basic
	@echo "Not FORTRAN. Ah! FORTRAN! The good ol' days!"

pnl: peenelles
	@echo "Pie Jesu Domine, Dona eis requiem."

benqi: benqs
	@echo "Tuppence a bag."

# ----- LOADER AND REPORT DEPENDENCIES -----------------------------------

LE_DATE=$(shell date +%Y-%m-%d)

JSON_LISTING=$(DATA_DIR)/listings/listings-$(LE_DATE).json
CSV_LISTING=$(DATA_DIR)/csv/alles/listings-$(LE_DATE).csv
PORT_LISTING=$(DATA_DIR)/csv/portfolio/portfolio-$(LE_DATE).csv

HOLDINGS=$(DATA_DIR)/csv/portfolio_coins.lsv

LIST_CMD="listings/latest?start=1&limit=5000&convert=USD"

CURL_CMD=$(SCRIPTS_DIR)/curl-cmc.sh

RUN_RUST=cargo run
MARKET=$(FIN_DIR)/market.lsv

clean: FORCE
	rm $(JSON_LISTING); \
	rm $(CSV_LISTING); \
	@echo "cleant"

nuke: FORCE
	rm $(DATA_DIR)/listings/listings-*.json; \
	rm $(DATA_DIR)/csv/alles/listings-*.csv; \
	rm $(DATA_DIR)/csv/portfolio/portfolio-*.csv; \
	@echo "My heart goes boum, boum, boum."

fetchers: $(JSON_LISTING)
	@echo "Loading e-coin listing file for $(LE_DATE) ..."; \
	$(CURL_CMD) cryptocurrency/$(LIST_CMD) $(JSON_LISTING)

$(JSON_LISTING): FORCE
	@true

$(CSV_LISTING): $(JSON_LISTING)
	@echo "enCVSing JSON quotes ..."; \
	cd $(SRC_DIR)/ch05/cmc_prices/; \
	$(RUN_RUST) $(JSON_LISTING) > $(CSV_LISTING)

encsvify: $(CSV_LISTING)
	@true

filterify: $(CSV_LISTING)
	@echo "filtering price-quotes to held assets..."; \
	cd $(SRC_DIR)/ch06/cmc_filter/; \
	$(RUN_RUST) $(CSV_LISTING) $(HOLDINGS) > $(PORT_LISTING); \
	cat $(PORT_LISTING)

basic: FORCE
	@echo "Extracting market prices of assets on FIN..."; \
	cd $(CRYPTO_TOOLS)/bases/; \
	$(RUN_RUST) $(MARKET)

peenelles: FORCE
	@echo "Running profit and lost report for FIN trades..."; \
	cd $(CRYPTO_TOOLS)/pnl/; \
	$(RUN_RUST) $(FIN_DIR)/assets.csv $(FIN_DIR)/trades.csv

benqs: FORCE
	@echo "Benqi marketplace positions"; \
	cd $(CARGO_HOME)/ch07/data_entry; \
	$(RUN_RUST) $(AVAX_DIR)/benqi_positions.lsv

# ----- ... and then we:

FORCE:

# ----- ... if need be. Eheh.
