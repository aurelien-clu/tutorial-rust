SHELL := /bin/bash
.PHONY : all

HEADER ?= "Content-Type: application/json"
TARGET ?= "http://localhost:3000"
JWT_SECRET ?= "secret"
BEARER ?= "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUiLCJleHAiOjEwMDAwMDAwMDAwfQ.M3LAZmrzUkXDC1q5mSzFAs_kJrwuKz3jOoDmjJ0G4gM"

serve:
	JWT_SECRET=$(JWT_SECRET) cargo run

all: token me me_bad_bearer
	echo ""

token:
	curl -s -w '\n' -H $(HEADER) -d '{"client_id":"foo","client_secret":"bar"}' $(TARGET)/authorize

me:
	curl -s -w '\n' -H $(HEADER) -H "Authorization: Bearer $(BEARER)" $(TARGET)/me

me_bad_bearer:
	curl -s -w '\n' -H $(HEADER) -H "Authorization: Bearer OH-NO" $(TARGET)/me
