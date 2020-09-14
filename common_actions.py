import os

# A small dev wrapper for common tools and commands (rust, cargo, git, etc.)
# I've made it because I am developing on a smartphone and this way it is more convenient.
# Moreover, compiling and running binary files is only possible in /data/data/com.termux/
# directory (I use Termux), due to Android security restrictions, so shifting source code
# from the internal storage to Termux's folder, compiling and bringing output files back to
# internal storage is done via this kind of automation, which is not required (but still could be used)
# on a desktop.

# Yes, this is a Python script to call bash commands.


# Must begin with /data/data/com.termux/files/home for Termux. For the desktop feel free to
# set any directory that you see fit, it will keep source, target and output separate.
path_source = "/data/data/com.termux/files/home/storage/shared/project_src/pglowrpg/"
path_target = "/data/data/com.termux/files/home/pglowrpg/"
path_output = "/data/data/com.termux/files/home/storage/shared/project_output/"

#I put these here for convenience of handling output
main_command = 'busybox time -f "%E %M"  cargo run'

#Works in Termux only, requires termux-api
main_command_tts_termux = main_command+" "+"| tee /dev/stderr | termux-tts-speak -r 1.2"

os.system("mkdir -p"+" "+path_target)
os.system("mkdir -p"+" "+path_target+"/save")

#Just fancy stuff
banner_git      = '▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒GIT MENU▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒'
banner_log      = '▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒LOG▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒'
banner_commit   = '▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒COMMITTING▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒'
banner_revert   = '▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒REVERTING▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒'
banner_hreset   = '▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒HARD RESETTING▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒'
banner_rust     = '▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒RUST MENU▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒'



def git_menu():

	git_log = "git log --branches --oneline -n 20"
	git_log_1 = "git log --branches --oneline -n 1"
	git_status = "git status"
	git_add = "git add . && git status --short"
	git_push = "git push --all"
	
	def print_git_ops():
		print('')
		print(banner_git)
		print('() - current')
		print('(l) - log')
		print('(s) - status')
		print('')
		print('(a) - add')
		print('(t) - commit')
		print('(p) - push')
		print('')
		print('(r) - revert')
		print('(ha) - hard reset')
		print(banner_git)
		print('')
		print('(u) - launch "gitui"')
		print('(q) - quit to main')
	
	def git_log_f():
		print(banner_log)
		os.system(git_log)
		print(banner_log)
	
	def git_commit_f():
		print(banner_commit)
		print('Files added:')
		os.system(git_add)
		print('Current head:')
		os.system(git_log_1)
		print(banner_commit)
		commit_name = input("New commit name (' ' to abort) » ").strip()
		if commit_name =="":
			return
		git_commit = "git commit -m "+commit_name
		os.system(git_commit)
		
	def git_revert_f():
		print(banner_revert)
		print('Commits:')
		os.system(git_log)
		print(banner_revert)
		commit_name = input("Revert to commit name (' ' to abort) » ").strip()
		if commit_name =="":
			return
		git_revert = "git revert "+commit_name
		os.system(git_revert)
		
	def git_reset_hard_f():
		print(banner_hreset)
		print('Commits:')
		os.system(git_log)
		print(banner_hreset)
		commit_name = input("Reset to commit name (' ' to abort) » ").strip()
		if commit_name =="":
			return
		inp = input("Really? » ").strip()
		if inp == "yes":
			git_reset_hard = "git reset --hard "+commit_name
			os.system(git_reset_hard)
	
	#Begin
	print_git_ops()
	while True:
		print('')
		inp = input("GIT MENU ('?' for commands) » ").strip()
		print('')
				
		if inp == "l":
			git_log_f()
		elif inp == "":
			print('Current head:')
			os.system(git_log_1)
		elif inp == "s":
			os.system(git_status)
		elif inp == "a":
			os.system(git_add)
		elif inp == "t":
			git_commit_f()
		elif inp == "p":
			os.system(git_push)
		elif inp == "r":
			git_revert_f()
		elif inp == "ha":
			git_reset_hard_f()
		elif inp == "?":
			print_git_ops()
		elif inp == "u":
			os.system('gitui')
		elif inp == "q":
			break


def main_menu():
	
	def print_main_ops():
		print('')
		print(banner_rust)
		print('( ) - sync presets, locals and "cargo run" the project')
		print('(tts) - sync presets, locals and "cargo run" the project, output via terminal and text-to-speech (Termux only)')
		print('')
		print('(a) - sync all src of the project (apps and libs)')
		print('(c) - sync APPS src ONLY and "cargo check" it,')
		print('(p) - sync APPS src ONLY and "cargo clippy" it,')
		print('(r) - do "rustfmt" on apps and libs')
		print('(clear) - clear ".bk" files')
		print('')
		print('(d) - "cargo dep-graph" the project,')
		print('(e) - "rustc --explain"')
		print('(u) - do "cargo update"')
		print(banner_rust)
		print('')
		print('(t) - git menu')
		print('(q) - quit')
	
	#Lower-level functions
	def copy_locales():
		os.system('rm -r'+' '+path_target+'locales || echo "Shell: nothing to remove"')
		os.system('cp -r'+' '+path_source+'locales'+' '+path_target+'locales')
		print('locales copied:')
		os.system('ls'+' '+path_target+'locales')
		
	def copy_options():
		os.system('rm -r'+' '+path_target+'options || echo "Shell: nothing to remove"')
		os.system('cp -r'+' '+path_source+'options'+' '+path_target+'options')
		print('options copied:')
		os.system('ls'+' '+path_target+'options')
		
	def copy_presets():
		os.system('rm -r'+' '+path_target+'presets || echo "Shell: nothing to remove"')
		os.system('cp -r'+' '+path_source+'presets'+' '+path_target+'presets')
		print('presets copied:')
		os.system('ls'+' '+path_target+'presets')
		
	def copy_source():
		os.system('rm -r'+' '+path_target+'src || echo "Shell: nothing to remove"')
		os.system('cp -r'+' '+path_source+'src'+' '+path_target+'src')
		os.system('rm -r'+' '+path_target+'apps || echo "Shell: nothing to remove"')
		os.system('cp -r'+' '+path_source+'apps'+' '+path_target+'apps')
		os.system('cp'+' '+path_source+'Cargo.toml'+' '+path_target+'Cargo.toml')
		print('src copied (apps):')
		os.system('ls'+' '+path_target+'apps')

	def copy_libs():
		os.system('rm -r'+' '+path_target+'libs || echo "Shell: nothing to remove"')
		os.system('cp -r'+' '+path_source+'libs'+' '+path_target+'libs')
		print('libs copied:')
		os.system('ls'+' '+path_target+'libs')
		
	def cargo_deps():
		os.system('rm'+' '+path_target+'dep_graph.png || echo "Shell: nothing to remove"')
		os.system('rm -r'+' '+path_output+'dep_graph || echo "Shell: nothing to remove"')
		os.system('mkdir -p'+' '+path_output+'dep_graph')
		os.system('cd'+' '+path_target+' && '+'cargo deps --all-deps | dot -Tpng > "dep_graph.png"')
		os.system('cp'+' '+path_target+'dep_graph.png'+' '+path_output+'dep_graph/dep_graph_$(date "+%Y%m%d-%H%M%S").png')
		print('dependency graph executed')
	
	def result_sync():
		#os.system('rm -r'+' '+path_output+'save || echo "Shell: nothing to remove"')
		os.system('rm -r'+' '+path_target+'save || echo "Shell: nothing to remove"')
		os.system('mkdir -p'+' '+path_output+'save')
		os.system('mkdir -p'+' '+path_target+'save')
		os.system('cd'+' '+path_target+' && '+main_command)
		os.system('cp -r'+' '+path_target+'save/*'+' '+path_output+'save/')
		print('\n')
		
		print('results copied:')
		os.system('ls'+' '+path_target+'save')
		
	def result_sync_tts_termux():
		#os.system('rm -r'+' '+path_output+'save || echo "Shell: nothing to remove"')
		os.system('rm -r'+' '+path_target+'save || echo "Shell: nothing to remove"')
		os.system('mkdir -p'+' '+path_output+'save')
		os.system('mkdir -p'+' '+path_target+'save')
		os.system('cd'+' '+path_target+' && '+main_command_tts_termux)
		os.system('cp -r'+' '+path_target+'save/*'+' '+path_output+'save/')
		print('\n')
		
		print('results copied:')
		os.system('ls'+' '+path_target+'save')
		
	#Higher-level functions
	def sync():
		os.system('clear')
		copy_locales()
		copy_options()
		copy_presets()
		result_sync()
	
	def sync_tts_termux():
		os.system('clear')
		copy_locales()
		copy_options()
		copy_presets()
		result_sync_tts_termux()
		
	def sync_all():
		copy_locales()
		copy_options()
		copy_presets()
		copy_source()
		copy_libs()
	
	def check():
		os.system('clear')
		copy_source()
		os.system('cd'+' '+path_target+' && '+'cargo check')
		
	def clippy_check():
		os.system('clear')
		copy_source()
		os.system('cd'+' '+path_target+' && '+'cargo clippy')
		
	def deps():
		copy_source()
		copy_libs()
		cargo_deps()
		
	def cargo_update():
		os.system('cd'+' '+path_target+' && '+'cargo update')
		print('dependencies updated')
		
	def explain():
		inp = input("Error code » ").strip()
		os.system('rustc --explain'+' '+inp)
		
	def rustfmt():
		os.system('clear')
		os.system('find'+' '+path_source+' '+'-type f -name "lib.rs" -o -name "main.rs"  | xargs -r rustfmt --check --config-path='+path_source+'rustfmt.toml')
		os.system('find'+' '+path_source+' '+'-type f -name "lib.rs" -o -name "main.rs"  | xargs -r rustfmt --backup --config-path='+path_source+'rustfmt.toml')
		print('formatting done')
		
	def clear_bk():
		os.system('find'+' '+path_source+' '+'-name "*.bk" -print0 | xargs -r0 rm -rf')
		print('.bk files cleared')
	
	#while True:
	print_main_ops()
	while True:
		print('')
		inp = input("MAIN MENU ('?' for commands) » ").strip()
		print('')
				
		if inp == "":
			sync()
		elif inp == "tts":
			sync_tts_termux()
		elif inp == "a":
			sync_all()
		elif inp == "c":
			check()
		elif inp == "p":
			clippy_check()
		elif inp == "d":
			deps()
		elif inp == "e":
			explain()
		elif inp == "r":
			rustfmt()
		elif inp == "u":
			cargo_update()
		elif inp == "clear":
			clear_bk()
		elif inp == "t":
			os.system('clear')
			clear_bk()
			git_menu()
		elif inp == "?":
			print_main_ops()
		elif inp == "q":
				quit()

#Start
while True:
	main_menu()
