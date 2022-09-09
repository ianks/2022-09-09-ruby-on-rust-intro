setup:
	cd demos && bundle install && bundle exec rake test

deck: 
	npx @marp-team/marp-cli deck.md --pdf

all: deck