authordir = $(datadir)/doc/$(PACKAGE)
author_DATA = ../AUTHORS

copyingdir = $(datadir)/doc/$(PACKAGE)
copying_DATA = ../COPYING

changlogdir = $(datadir)/doc/$(PACKAGE)
changlog_DATA = ../ChangeLog

man_MANS=$(PACKAGE).1

etcdir = $(sysconfdir)/xdg
etc_DATA = $(PACKAGE).conf

desktopdir = $(datadir)/applications
desktop_DATA = $(PACKAGE).desktop

pixmapsdir = $(datadir)/pixmaps
pixmaps_DATA = $(PACKAGE).png $(PACKAGE).xpm

examplesdir = $(datadir)/doc/$(PACKAGE)/examples
examples_DATA = $(PACKAGE).conf

EXTRA_DIST = $(author_DATA) $(copying_DATA) $(changlog_DATA) $(etc_DATA) $(desktop_DATA) $(pixmaps_DATA) $(examples_DATA)
