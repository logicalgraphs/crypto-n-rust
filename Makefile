# wow. Makefile. Wow. We went there.

SCRIPTS_DIR=$(RUST_BOOK)/src/scripts

fetch: fetchers
	@echo "Didst fetcheth."

# ----- LOADER AND REPORT DEPENDENCIES -----------------------------------

LE_DATE=$(shell date +%Y-%m-%d)

LISTING_FILE=$(RUST_BOOK)/data-files/listings/listings-$(LE_DATE).json

LIST_CMD="listings/latest?start=1&limit=5000&convert=USD"

CURL_CMD=$(RUST_BOOK)/src/scripts/curl-cmc.sh

fetchers: $(LISTING_FILE)
	@echo "Loading e-coin listing file for $(LE_DATE) ..."; \
	$(CURL_CMD) cryptocurrency/$(LIST_CMD) $(LISTING_FILE)

# ----- ... and then we:

FORCE:

# ----- ... if need be. Eheh.
