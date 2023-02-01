function fish_greeting
	echo It\'s (set_color yellow; date +%T; set_color normal) 
	echo You should (set_color blue; head -1 ~/todo.txt; set_color normal)
end
