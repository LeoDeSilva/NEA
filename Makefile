open: build/NEA.pdf
	open build/NEA.pdf

build/NEA.pdf: NEA.tex NEA.bib ./src/*.tex ./src/analysis/*.tex ./src/design/*.tex
	biber --output-directory build NEA
	pdflatex -output-directory=build -aux-directory=build NEA.tex

clean:
	rm -f build/*.aux build/*.fdb_latexmk build/*.fls build/*.log build/*.synctex.gz build/*.toc build/*.pdf
	rm -f *.aux *.fdb_latexmk *.fls *.log *.synctex.gz *.toc *.pdf *.bbl *.bcf *.blg *.run.xml
