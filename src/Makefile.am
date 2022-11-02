INCLUDES = -DICONDIR=\"$(datadir)/pixmaps\" -DETCDIR=\"$(sysconfdir)/xdg\"

bin_PROGRAMS = $_BINARY
terminal_CFLAGS  = $(GTK_CFLAGS) $(vte_CFLAGS)
terminal_LDADD   = $(GTK_LIBS) $(vte_LIBS) $(INTLLIBS)
terminal_SOURCES = data.h terminal.h \
		   socket.h socket.c \
		   misc.h misc.c \
		   console.h console.c \
		   menu.h menu.c \
		   profile.h profile.c \
		   property.h property.c \
		   dialog.h dialog.c \
		   pagename.h pagename.c \
		   notebook.h notebook.c \
		   font.h font.c \
		   window.h window.c \
		   main.h main.c

