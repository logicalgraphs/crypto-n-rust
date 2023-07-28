# wow. Makefile. Wow. We went there.

DATA_DIR=$(RUST_BOOK)/data-files
SRC_DIR=$(RUST_BOOK)/src
LIB_DIR=$(SRC_DIR)/libs
BOOK_LIB=$(LIB_DIR)/book
CRYPTO_LIB=$(LIB_DIR)/crypto
REST_LIB=$(LIB_DIR)/rest

GIT_RAW="https://raw.githubusercontent.com/logicalgraphs/crypto-n-rust/main"

SCRIPTS_DIR=$(SRC_DIR)/scripts

LE_DATE=$(shell date +%Y-%m-%d)

alles: fetch process filter
	@true

fetch: fetchers
	@echo "Didst fetcheth."

process: encsvify
	@echo "Didst processeth."

filter: filterify
	@echo "Can you? Can you trip like I do?"

wallet:
	cd src/ch09/wallet; \
	$(RUN_RUST) $(MARKET) $(BLUE_DIR)/wallet.lsv

bases: basic
	@echo "Not FORTRAN. Ah! FORTRAN! The good ol' days!"

pnl: peenelles
	@echo "Pie Jesu Domine, Dona eis requiem."

benqi: benqs
	@echo "Tuppence a bag."

geist: ghost
	@echo "BOO!"

grain: granary
	@echo "Wheat!"

bow: arrow
	@echo "Rain man."

vols: top
	echo ""; \
	echo "daily report,Team Kujira,Top5s,FIN,Order Book,volume,"

bar: pub
	@true

voronoi: vee
	@true

stride: lsd_report
	@echo "She don' like; she don' like; she don' like... cocaine!"

help: FORCE
	@cat $(RUST_BOOK)/commands.txt

# ----- LOADER AND REPORT DEPENDENCIES -----------------------------------

JSON_LISTING=$(DATA_DIR)/listings/listings-$(LE_DATE).json
CSV_LISTING=$(DATA_DIR)/csv/alles/listings-$(LE_DATE).csv
PORT_LISTING=$(DATA_DIR)/csv/portfolio/portfolio-$(LE_DATE).csv

HOLDINGS=$(DATA_DIR)/csv/portfolio_coins.lsv

LIST_CMD="listings/latest?start=1&limit=5000&convert=USD"

CURL_CMD=$(SCRIPTS_DIR)/curl.sh
CMC_ENDPOINT=https://pro-api.coinmarketcap.com/v1

RUN_RUST=cargo run
BUILD_RUST=cargo build

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
	$(CURL_CMD) $(CMC_ENDPOINT)/cryptocurrency/$(LIST_CMD) $(JSON_LISTING)

$(JSON_LISTING): FORCE
	@true

$(CSV_LISTING): $(JSON_LISTING)
	@echo "enCVSing JSON quotes ..."; \
	cd $(SRC_DIR)/ch05/cmc_prices; \
	$(RUN_RUST) $(JSON_LISTING) > $(CSV_LISTING)

encsvify: $(CSV_LISTING)
	@true

filterify: $(CSV_LISTING)
	@echo "filtering price-quotes to held assets..."; \
	cd $(SRC_DIR)/ch06/cmc_filter; \
	$(RUN_RUST) $(CSV_LISTING) $(HOLDINGS) > $(PORT_LISTING); \
	cat $(PORT_LISTING)

basic: FORCE
	@echo "Extracting market prices of assets on FIN..."; \
	cd $(CRYPTO_TOOLS)/bases; \
	$(RUN_RUST) $(MARKET)

peenelles: FORCE
	@echo "Running profit and lost report for FIN trades..."; \
	cd $(CRYPTO_TOOLS)/pnl; \
	$(RUN_RUST) -- $(debug) $(FIN_DIR)/assets.csv $(FIN_DIR)/trades.tsv

benqs: FORCE
	@echo "Benqi marketplace positions"; \
	cd $(CARGO_HOME)/ch07/data_entry/benqi; \
	$(RUN_RUST) $(MONEY_MARKETS_DIR)/benqi_positions.lsv

ghost: FORCE
	@echo "Geist marketplace positions"; \
	cd $(CARGO_HOME)/ch07/data_entry/benqi; \
	$(RUN_RUST) $(MONEY_MARKETS_DIR)/geist.lsv

granary: FORCE
	@echo "Granary marketplace positions"; \
	cd $(CARGO_HOME)/ch07/data_entry/benqi; \
	$(RUN_RUST) $(MONEY_MARKETS_DIR)/granary.lsv

arrow: FORCE
	@echo "BOW top-5 LPs"; \
	cd $(CRYPTO_TOOLS)/lps/; \
	$(RUN_RUST) $(LE_DATE) $(mode) $(BOW_DIR)/lps.lsv

SPACE_DIR=$(SRC_DIR)/ch09
TOP_DIR=$(SPACE_DIR)/top_order_books

top: FORCE
	@echo "Please be sure $(FIN_VOLUMES_JSON) is updated first!"; \
	cd $(TOP_DIR); \
	$(RUN_RUST) -- --raw $(LE_DATE) $(FIN_VOLUMES_JSON)

LSD_DIR=$(SPACE_DIR)/lsd
LSD_CSV=data/stride.csv

lsd: FORCE
	cd $(LSD_DIR); \
	$(RUN_RUST) $(LE_DATE) > $(LSD_DIR)/$(LSD_CSV); \
	git co main; \
	git add $(LSD_CSV); \
	git commit -m "Today's LSD rates."; \
	git push

lsd_report: lsd
	echo "$(LE_DATE) Latest Stride LSD exchange rates"; \
	echo ""; \
	echo "<p>$(LE_DATE) Latest @stride_zone LSD exchange rates up.</p>"; \
	echo ""; \
	echo "<p><a href='$(GIT_RAW)/src/ch09/lsd/$(LSD_CSV)'>Raw CSV</a>"; \
        echo "of report available and archived.</p>"; \
	echo ""; \
        echo "<a href='https://github.com/logicalgraphs/crypto-n-rust/blob/main/src/ch09/lsd/lsd.rs'>./lsd source code</a>."; \
	echo ""; \
	echo "daily report,liquid staking token,Stride,cryptocurrency";
	echo ""; \
	echo "2023-07-27 Latest Stride LSD exchange rates archived on my blog at "; \
	echo ""; \
	echo "Extracted and transformed by ./lsd: https://github.com/logicalgraphs/crypto-n-rust/blob/main/src/ch09/lsd/lsd.rs"; \
	echo ""

PORT_TSV=$(DATA_DIR)/portfolio/protocols.tsv
CHART_DIR=$(CRYPTO_TOOLS)/charts

define charting
	@echo "Update $(PORT_TSV) FRIST! AHA!"; \
	cd $(CRYPTO_TOOLS)/charts/$(1); \
	$(RUN_RUST) $(PORT_TSV)
endef

pub: FORCE
	$(call charting,bar)

vee: FORCE
	$(call charting,voronoi)

# ----- let's build stuff, ... 'n stuff!

define build
	cd $(1); $(BUILD_RUST); echo "$(2) built."
endef

buildCMC:
	$(call build,$(SRC_DIR)/ch05/cmc_prices,cmc_prices); \
	$(call build,$(SRC_DIR)/ch06/cmc_filter,cmc_filter)

TOOLS=bases pnl lps

buildTools:
	for tool in $(TOOLS); do \
		$(call build,$(CRYPTO_TOOLS)/$$tool,$$tool); \
	done

buildMoneyMarkets:
	$(call build,$(CARGO_HOME)/ch07/data_entry/benqi,money markets)

CHARTS=bar voronoi

buildCharts:
	for chart in $(CHARTS); do \
		$(call build,$(CHART_DIR)/$$chart,$$chart); \
	done

buildTop5s:
	$(call build,$(TOP_DIR),top5s)

buildWallet:
	$(call build,$(SPACE_DIR)/wallet,wallet)

buildSpacey: buildTop5s buildWallet
	@true

buildApps: buildCharts buildSpacey buildTools buildMoneyMarkets buildCMC
	@true

buildBook:
	$(call build,$(BOOK_LIB),book lib)

buildCrypto:
	$(call build,$(CRYPTO_LIB),crypt lib)

buildRest:
	$(call build,$(REST_LIB),rest lib)

buildLibs: buildBook buildCrypto buildRest
	@true

buildAll: buildLibs buildApps
	@true

# ----- ... and then we:

FORCE:

# ----- ... if need be. Eheh.
