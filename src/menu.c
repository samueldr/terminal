/*
 * Copyright (c) 2008-2014 Lu, Chao-Ming (Tetralet).  All rights reserved.
 *
 * This file is part of LilyTerm.
 *
 * LilyTerm is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * LilyTerm is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with LilyTerm.  If not, see <http://www.gnu.org/licenses/>.
 */

#include "menu.h"

// Sometimes we may popup a menu on a non-active (not focus) window...
extern GList *window_list;

gboolean check_if_win_data_is_still_alive(struct Window *win_data)
{
#ifdef DETAIL
	g_debug("! Launch check_if_win_data_is_still_alive() with win_data = %p!", win_data);
#endif
	GList *win_list = window_list;

	while (win_list)
	{
		if (win_data == win_list->data) return TRUE;
		win_list = win_list->next;
	}
#ifdef DEBUG
	fprintf(stderr, "\033[1;%dm** check_if_win_data_is_still_alive(): win_data (%p) is NOT alive!\033[0m\n",
		ANSI_COLOR_RED, win_data);
#endif
	// g_debug("check_if_win_data_is_still_alive: win_data (%p) is NOT alive!!!", win_data);
	return FALSE;
}
