import os, fnmatch, shutil, pathlib

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
banner_rust     = '▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒SOURCE MENU▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒'
divider = '-------------------------------------------------------'


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
		inp = input("Really commit? » ").strip()
		if inp == "yes":
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
		print('( ) - sync and "cargo run" the project')
		print('(tts) - sync and "cargo run" the project, output via terminal and text-to-speech (Termux only)')
		print('')
		print('(a) - clear target folder and sync files anew')
		print('(c) - sync and "cargo check" it,')
		print('(p) - sync and "cargo clippy" it,')
		print('(r) - do "rustfmt"')
		print('(clear) - clear ".bk" files')
		print('(tree) - "tree" the target folder')
		print('')
		print('(d) - "cargo dep-graph" the project,')
		print('(e) - "rustc --explain"')
		print('(u) - do "cargo update"')
		print(banner_rust)
		print('')
		print('(t) - git menu')
		print('(q) - quit')
	
	#Main sync function
	def flietype_sync(pattern, timestamp_file, path_source, path_target):
		
		queue_source = []
		queue_names = []
		timestamps_new = []
		timestamps_old = []
		
		#read existing timestamps
		try:
			timestamps_file = open(timestamp_file, 'r') 
			lines = timestamps_file.readlines() 
			for line in lines: 
				timestamps_old.append(line.strip())
		except:
			print(divider)
			print('no timestamp file: '+timestamp_file+' exist, it will be created')
			print(divider)
			
		#get the current timestamps & paths from source directory
		for root, dirs, files in os.walk(path_source):
			for name in files:
				if fnmatch.fnmatch(name, pattern):
					file_path = os.path.join(root, name)
					queue_source.append(file_path)
					queue_names.append(name)
					timestamps_new.append(os.path.getmtime(file_path))
					
		#pick up files that were modified
		try:
			#copy the new files to target
			for entry in range(len(timestamps_new)):
				if float(timestamps_new[entry]) != float(timestamps_old[entry]):
					print(divider)
					print('file modified:')
					#make path relative
					if path_source in queue_source[entry]:
						rel_path = queue_source[entry].replace(path_source, '')
						print(timestamps_new[entry], timestamps_old[entry], rel_path)
						
						#copy modified file to target folder
						shutil.copyfile(queue_source[entry], path_target+rel_path)
						
		
		#or copy everything
		except:
			print(divider)
			print('copying all files this time')
			print(divider)
			for entry in range(len(timestamps_new)):
				
				#make path relative
				if path_source in queue_source[entry]:
					rel_path = queue_source[entry].replace(path_source, '')
					print(timestamps_new[entry], rel_path)
					
					#copy file to target
					try:
						shutil.copyfile(queue_source[entry], path_target+rel_path)
					#if no folder exist, make it and then copy
					except:
						if queue_names[entry] in rel_path:
							rel_path_stripped = rel_path.replace(queue_names[entry], '')
						pathlib.Path(path_target+rel_path_stripped).mkdir(parents=True, exist_ok=True)
						shutil.copyfile(queue_source[entry], path_target+rel_path)
					
		#write (new) timestamps
		with open(timestamp_file, 'w') as f:
		    for item in timestamps_new:
		        f.write("%s\n" % item)
	
	
	def dir_remove(folder_name, path):
		shutil.rmtree(path+folder_name, ignore_errors=True)
	
	
	#Functions-wrappers for commands
	def sync_files():
		flietype_sync('*.rs', '.timestamp_rs', path_source, path_target)
		flietype_sync('*.toml', '.timestamp_toml', path_source, path_target)
		print(divider)
		print('files synchronized')
		print(divider)
		
	def clear_target():
		try:
			#must be removed to trigger copying
			os.remove('.timestamp_rs')
			os.remove('.timestamp_toml')
		except:
			print(divider)
			print('no timestamp file(s) to remove')
			print(divider)
		dir_remove('src', path_target)
		dir_remove('apps', path_target)
		dir_remove('libs', path_target)
		dir_remove('locales', path_target)
		dir_remove('presets_default', path_target)
		dir_remove('presets_user', path_target)
		dir_remove('options', path_target)
		dir_remove('save', path_target)
		print(divider)
		print('target directory cleared, remaining files:')
		os.system('ls'+' '+path_target)
		print(divider)
		
		
	def cargo_deps():
		os.system('rm'+' '+path_target+'dep_graph.png || echo "Shell: nothing to remove"')
		os.system('rm -r'+' '+path_output+'dep_graph || echo "Shell: nothing to remove"')
		os.system('mkdir -p'+' '+path_output+'dep_graph')
		os.system('cd'+' '+path_target+' && '+'cargo deps --all-deps | dot -Tpng > "dep_graph.png"')
		os.system('cp'+' '+path_target+'dep_graph.png'+' '+path_output+'dep_graph/dep_graph_$(date "+%Y%m%d-%H%M%S").png')
		print(divider)
		print('dependency graph executed')
		print(divider)
	
	def result_sync():
		#os.system('rm -r'+' '+path_output+'save || echo "Shell: nothing to remove"')
		#os.system('rm -r'+' '+path_target+'save || echo "Shell: nothing to remove"')
		os.system('mkdir -p'+' '+path_output+'save')
		os.system('mkdir -p'+' '+path_target+'save')
		os.system('cd'+' '+path_target+' && '+main_command)
		os.system('cp -r'+' '+path_target+'save/*'+' '+path_output+'save/')
		print(divider)
		print('results copied:')
		os.system('ls'+' '+path_target+'save')
		print(divider)
		
	def result_sync_tts_termux():
		#os.system('rm -r'+' '+path_output+'save || echo "Shell: nothing to remove"')
		#os.system('rm -r'+' '+path_target+'save || echo "Shell: nothing to remove"')
		os.system('mkdir -p'+' '+path_output+'save')
		os.system('mkdir -p'+' '+path_target+'save')
		os.system('cd'+' '+path_target+' && '+main_command_tts_termux)
		os.system('cp -r'+' '+path_target+'save/*'+' '+path_output+'save/')
		print(divider)
		print('results copied:')
		os.system('ls'+' '+path_target+'save')
		print(divider)
		
	#Higher-level functions
	def sync():
		os.system('clear')
		sync_files()
		result_sync()
	
	def sync_tts_termux():
		os.system('clear')
		sync_files()
		result_sync_tts_termux()
		
	def sync_all():
		os.system('clear')
		clear_target()
		sync_files()
		
	def check():
		os.system('clear')
		sync_files()
		os.system('cd'+' '+path_target+' && '+'cargo check')
		
	def clippy_check():
		os.system('clear')
		sync_files()
		os.system('cd'+' '+path_target+' && '+'cargo clippy')
		
	def deps():
		os.system('clear')
		sync_files()
		cargo_deps()
		
	def cargo_update():
		os.system('clear')
		os.system('cd'+' '+path_target+' && '+'cargo update')
		print(divider)
		print('dependencies updated')
		print(divider)
		
	def explain():
		inp = input("Error code » ").strip()
		os.system('rustc --explain'+' '+inp)
		
	def rustfmt():
		os.system('clear')
		os.system('find'+' '+path_source+' '+'-type f -name "lib.rs" -o -name "main.rs"  | xargs -r rustfmt --check --config-path='+path_source+'rustfmt.toml')
		os.system('find'+' '+path_source+' '+'-type f -name "lib.rs" -o -name "main.rs"  | xargs -r rustfmt --backup --config-path='+path_source+'rustfmt.toml')
		print(divider)
		print('formatting done')
		print(divider)
		
	def clear_bk():
		os.system('clear')
		os.system('find'+' '+path_source+' '+'-name "*.bk" -print0 | xargs -r0 rm -rf')
		print(divider)
		print('.bk files cleared')
		print(divider)
		
	def target_tree():
		os.system('clear')
		os.system('tree -I target'+' '+path_target)
		print(divider)
		print('target foldet tree rendred')
		print(divider)
	
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
		elif inp == "tree":
			target_tree()
		elif inp == "t":
			os.system('clear')
			git_menu()
		elif inp == "?":
			print_main_ops()
		elif inp == "q":
				quit()

#Start
os.system('clear')
while True:
	main_menu()
