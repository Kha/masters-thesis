all: *.tex bib.bib
	latexmk -lualatex -shell-escape main.tex
