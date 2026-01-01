.PHONY: all tr en clean

all: tr en

tr:
	cd tr && pdflatex main.tex

en:
	cd en && pdflatex main.tex

clean:
	rm -f tr/*.aux tr/*.log tr/*.pdf tr/*.out
	rm -f en/*.aux en/*.log en/*.pdf en/*.out
