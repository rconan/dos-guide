PHONY: all

all:
	cd ../book/markdown && \
		find . -name \*.md -exec sed -i 's/rust,ignore/rust/g' {} \; && \
		pandoc -t latex introduction.md \
			getting-started.md getting-started/installation.md getting-started/dos-interface.md \
			optical-model.md optical-model/installation.md optical-model/dos-interface.md \
			fem.md fem/installation.md fem/dos-interface.md \
			wind-loading.md wind-loading/installation.md wind-loading/dos-interface.md \
			controllers.md controllers/simulink.md \
			appendix.md dos-io-variants.md | sed '/^\\NormalTok{\\#/d' | sed '/^\\CommentTok{\\#/d' > ../../release/dos.tex
	pdflatex dos.main.tex
	pdflatex dos.main.tex
