#▒▒▒▒▒▒▒▒▒▒▒▒ USER OPTIONS ▒▒▒▒▒▒▒▒▒▒▒▒▒
default_editor = "nano" # a text editor command to call by default
# use "python" to disable prompt and always use native input
text_width = 55 #in characters 55...80 should be good

#▒▒▒▒▒▒▒▒▒▒▒▒ IMPORTS / CONSTANTS ▒▒▒▒▒▒▒▒▒▒▒▒▒
import os, fnmatch, shutil, pathlib, tempfile, subprocess, textwrap

# Must begin with /data/data/com.termux/files/home for Termux. For the desktop feel free to
# set any directory that you see fit, it will keep source, target and output separate.
path_source = "/data/data/com.termux/files/home/storage/shared/project_src/pglowrpg/"
path_target = "/data/data/com.termux/files/home/pglowrpg/"
path_output = "/data/data/com.termux/files/home/storage/shared/project_output/"

main_command = 'busybox time -f "%E %M"  cargo run'
main_command_tts_termux = main_command+" "+"| tee /dev/stderr | termux-tts-speak -r 1.2"

#▒▒▒▒▒▒▒▒▒▒▒▒ WRITING OPS ▒▒▒▒▒▒▒▒▒▒▒▒▒
def write_num_not_empty(type, prompt_str):
	while True:
		num = c_prompt(prompt_str)
		try: 
			if type == 'int': num = int(num); return num
			elif type == 'float': num = float(num); return num
			else: print_wrong_num_type(); p()
		except: print_num_wrong_input()
		
def write_not_empty(inject_text, flag, allow_exit):
	name = '';
	while not name: 
		if not flag == 'prompt': name = write_with_editor(inject_text)
		else: name = write_fallback(inject_text)
		name = parse_off_comments(name)
		if name =='': 
			if not allow_exit:
				print_abort_writing()
				inp = c_prompt('')
				if inp == 'qm': main_menu()
			else: 
				print_abort_writing_quit_allowed()
				inp = c_prompt('')
				if inp == 'qm': main_menu()
				elif inp == 'q': return name
	return name

def write_with_editor(inject_text):
	def write_ext(option, inject_text):
		written = ''
		with tempfile.NamedTemporaryFile(suffix=".tmp") as tf:
			if inject_text: 
				try: tf.write(inject_text)
				except TypeError: tf.write(inject_text.encode("utf-8"))
				finally: tf.flush()
			try: 
				subprocess.call([option, tf.name])
				tf.seek(0); written = tf.read().decode("utf-8")
				return written.strip()
			except: 
				print_no_default_editor(option); p(); 
				return write_fallback(inject_text)
	#BEGIN
	if default_editor == 'python': return write_fallback(inject_text)
	else: return write_ext(default_editor, inject_text)

def write_fallback(inject_text):
	print_fallback_editor(inject_text)
	return s_prompt('enter text')

def parse_off_comments(text):
	out = ''
	for line in text.splitlines(True):
		if line.lstrip().startswith('#'): continue
		else: out += line
	return out

#▒▒▒▒▒▒▒▒▒▒▒▒ MENUS ▒▒▒▒▒▒▒▒▒▒▒▒▒
def git_menu():
	print_git_ops()
	while True:
		inp = c_prompt('GIT')
		print_git_ops()
		if inp == "": git_info()
		elif inp == "l": git_log_f()
		elif inp == "s": git_status()
		elif inp == "c": git_commit_f()
		elif inp == "p": git_push()
		elif inp == "r": git_revert_f()
		elif inp == "ha": git_reset_hard_f()
		elif inp == "u": git_launch_gitui()
		elif inp == "q": break
		
def main_menu():
	print_main_ops()
	while True:
		inp = c_prompt('MENU')
		print_main_ops()
		if inp == 's': sync()
		elif inp == "tts": sync_tts_termux()
		elif inp == "a": sync_all()
		elif inp == "c": check()
		elif inp == "p": clippy_check()
		elif inp == "d": deps()
		elif inp == "e": explain()
		elif inp == "r": rustfmt()
		elif inp == "u": cargo_update()
		elif inp == "clear": clear_bk()
		elif inp == "tree": target_tree()
		elif inp == "cs": clear_save()
		elif inp == "git": git_menu(); print_main_ops()
		elif inp == "q": cl_divider(); quit()
		
#▒▒▒▒▒▒▒▒▒▒▒▒ FORMATTING ▒▒▒▒▒▒▒▒▒▒▒▒▒
if text_width < 30: print('minimal text width value is 30'); text_width = 30
in_tags = ' tags:   '
in_links_out = ' └ to:   '
in_links_in = ' └ by:   '
indent = '         '; ph = '...'
right_indent = 4; ml = 3
tw_tags = textwrap.TextWrapper(text_width-right_indent, 
	initial_indent=in_tags, subsequent_indent=indent,
	placeholder=ph, max_lines=ml)
tw_links_out = textwrap.TextWrapper(text_width-right_indent, 
	initial_indent=in_links_out, subsequent_indent=indent,
	placeholder=ph, max_lines=ml)
tw_links_in = textwrap.TextWrapper(text_width-right_indent, 
	initial_indent=in_links_in, subsequent_indent=indent,
	placeholder=ph, max_lines=ml)
tw = textwrap.TextWrapper(text_width)
tw_w = textwrap.TextWrapper(text_width-1, initial_indent=' ', subsequent_indent=' ', replace_whitespace=False)
tw_i = textwrap.TextWrapper(text_width, subsequent_indent=indent)

#▒▒▒▒▒▒▒▒▒▒▒▒ PRINT OPS ▒▒▒▒▒▒▒▒▒▒▒▒▒
#GIT OPS
def print_git_current_head(): 
	divider() 
	print(tw_i.fill('Current head:'))
	os.system("git log --branches --oneline -n 1")

def print_git_status():
	cl_divider()
	os.system("git status")

def print_git_log(entries):
	cl_divider()
	os.system("git log --branches --oneline -n "+str(entries)); 
	
def print_git_push():
	cl_divider()
	os.system("git push --all")
	
def print_git_add_modified():
	cl_divider()
	print(tw_i.fill('New / modified files:'))
	os.system("git add . ")
	os.system("git status --short")
	
#MENUS
def print_main_ops():
	cl_divider()
	print(tw_i.fill('(s) - sync and "cargo run" the project'))
	print(tw_i.fill('(tts) - sync and "cargo run" the project, output via terminal and text-to-speech (Termux only)'))
	print()
	print(tw_i.fill('(a) - clear target folder and sync files anew'))
	print(tw_i.fill('(c) - sync and "cargo check" it'))
	print(tw_i.fill('(p) - sync and "cargo clippy" it'))
	print(tw_i.fill('(r) - do "rustfmt"'))
	print(tw_i.fill('(clear) - clear ".bk" files'))
	print(tw_i.fill('(tree) - "tree" the target folder'))
	print(tw_i.fill('(cs) - clear target save folder'))
	print()
	print(tw_i.fill('(d) - "cargo dep-graph" the project'))
	print(tw_i.fill('(e) - "rustc --explain"'))
	print(tw_i.fill('(u) - do "cargo update"'))
	print()
	print(tw_i.fill('(git) - git menu'))
	print(tw_i.fill('(q) - quit'))
	
def print_git_ops():
	cl_divider()
	print(tw_i.fill('() - current'))
	print(tw_i.fill('(l) - log'))
	print(tw_i.fill('(s) - status'))
	print(tw_i.fill('(c) - commit all'))
	print(tw_i.fill('(p) - push'))
	print(tw_i.fill('(r) - revert'))
	print(tw_i.fill('(ha) - hard reset'))
	print(tw_i.fill('(u) - launch "gitui" (must be installed)'))
	print_q()

#PROMPTS
def c_prompt(prompt): 
	divider(); 
	try: inp = input(prompt+" : ").rstrip()
	except KeyboardInterrupt: inp = ''
	return inp 
	
def s_prompt(prompt):
	divider(); 
	try: inp = input(prompt+" > ").rstrip()
	except KeyboardInterrupt: inp = ''
	return inp 
	
def p(): divider(); l=(text_width-10)//2; s="░"*l+" CONTINUE "+"░"*l; input(s)
def print_qc(ch): print(tw_i.fill("({0}) - return | confirm".format(ch)))
def print_q(): print(tw_i.fill('(q) - return'))
def print_qm(): print(tw_i.fill('(qm) - return to main menu | abort everything'))

#CLEAR SCREEN AND DIVIDER 
def divider(): 
	d_line = '─' * text_width
	print(d_line)
def cl(): os.system('cls' if os.name == 'nt' else 'clear')
def cl_divider(): cl(); divider()

#▒▒▒▒▒▒▒▒▒▒▒▒ GIT OPS ▒▒▒▒▒▒▒▒▒▒▒▒▒
def git_info(): print_git_current_head()
def git_status(): print_git_status()
def git_log_f(): 
	entries = write_num_not_empty('int', 'commits to print')
	print_git_log(entries)
	
def git_launch_gitui(): os.system('gitui')
def git_push(): print_git_push()

def git_commit_f():
	print_git_add_modified(); print_git_current_head();p()
	comment = '# Enter the new commit name below\n'
	commit_name = write_not_empty(comment, flag=None, allow_exit=True)
	if commit_name =='': return
	inp = c_prompt("really? ('yes' to proceed)")
	if inp == "yes": os.system("git commit -m "+ '\"'+commit_name+'\"')
	
def git_revert_f():
	git_log_f(); p()
	comment = '# Enter the commit name to revert to below\n'
	commit_name = write_not_empty(comment, flag=None, allow_exit=True)
	if commit_name =='': return
	os.system("git revert "+ '\"'+commit_name+'\"')
	
def git_reset_hard_f():
	git_log_f(); p()
	comment = '# Enter the commit name to RESET to below\n'
	commit_name = write_not_empty(comment, flag=None, allow_exit=True)
	if commit_name =='': return
	inp = c_prompt("really? ('yes' to proceed)")
	if inp == "yes": os.system("git reset --hard "+ '\"'+commit_name+'\"')

#▒▒▒▒▒▒▒▒▒▒▒▒ RUST OPS ▒▒▒▒▒▒▒▒▒▒▒▒▒
def flietype_sync(pattern, timestamp_file, path_source, path_target):
	queue_source = []
	queue_names = []
	timestamps_new = []
	timestamps_old = []
	try: #read existing timestamps
		timestamps_file = open(timestamp_file, 'r') 
		lines = timestamps_file.readlines() 
		for line in lines: 
			timestamps_old.append(line.strip())
	except:
		print('no timestamp file: '+timestamp_file+' exist, it will be created')
	#get the current timestamps & paths from source directory
	for root, dirs, files in os.walk(path_source):
		for name in files:
			if fnmatch.fnmatch(name, pattern):
				file_path = os.path.join(root, name)
				queue_source.append(file_path)
				queue_names.append(name)
				timestamps_new.append(os.path.getmtime(file_path))
	#pick up files that were modified
	try: #copy the new files to target
		for entry in range(len(timestamps_new)):
			if float(timestamps_new[entry]) != float(timestamps_old[entry]):
				print('file modified:')
				#make path relative
				if path_source in queue_source[entry]:
					rel_path = queue_source[entry].replace(path_source, '')
					print(timestamps_new[entry], timestamps_old[entry], rel_path)
					#copy modified file to target folder
					shutil.copyfile(queue_source[entry], path_target+rel_path)
	except: #or copy everything
		print('copying all files this time')
		for entry in range(len(timestamps_new)): #make path relative
			if path_source in queue_source[entry]:
				rel_path = queue_source[entry].replace(path_source, '')
				print(timestamps_new[entry], rel_path)
				#copy file to target
				try: shutil.copyfile(queue_source[entry], path_target+rel_path)
				except: #if no folder exist, make it and then copy
					if queue_names[entry] in rel_path:
						rel_path_stripped = rel_path.replace(queue_names[entry], '')
					pathlib.Path(path_target+rel_path_stripped).mkdir(parents=True, exist_ok=True)
					shutil.copyfile(queue_source[entry], path_target+rel_path)
	with open(timestamp_file, 'w') as f: #write (new) timestamps
		for item in timestamps_new: f.write("%s\n" % item)

def dir_remove(folder_name, path):
	shutil.rmtree(path+folder_name, ignore_errors=True)

#Functions-wrappers for commands
def sync_files():
	cl_divider();
	flietype_sync('*.rs', '.timestamp_rs', path_source, path_target)
	flietype_sync('*.toml', '.timestamp_toml', path_source, path_target)
	flietype_sync('*.ron', '.timestamp_ron', path_source, path_target)
	
def clear_target():
	try: #must be removed to trigger copying
		os.remove('.timestamp_rs')
		os.remove('.timestamp_toml')
		os.remove('.timestamp_ron')
	except: pass
	dir_remove('src', path_target)
	dir_remove('apps', path_target)
	dir_remove('libs', path_target)
	dir_remove('locales', path_target)
	dir_remove('presets_default', path_target)
	dir_remove('presets_user', path_target)
	dir_remove('options', path_target)
	#dir_remove('save', path_target)
	print('target directory cleared, remaining files:')
	os.system('ls'+' '+path_target)
	
def clear_save():
	inp = input("Really? » ").strip()
	if inp == "yes":
		dir_remove('save', path_target)
		print('target directory saves cleared')
	
def cargo_deps():
	cl_divider();
	os.system('rm'+' '+path_target+'dep_graph.png || echo "Shell: nothing to remove"')
	os.system('rm -r'+' '+path_output+'dep_graph || echo "Shell: nothing to remove"')
	os.system('mkdir -p'+' '+path_output+'dep_graph')
	os.system('cd'+' '+path_target+' && '+'cargo deps --all-deps | dot -Tpng > "dep_graph.png"')
	os.system('cp'+' '+path_target+'dep_graph.png'+' '+path_output+'dep_graph/dep_graph_$(date "+%Y%m%d-%H%M%S").png')
	print('dependency graph executed')

def result_sync():
	#os.system('rm -r'+' '+path_output+'save || echo "Shell: nothing to remove"')
	#os.system('rm -r'+' '+path_target+'save || echo "Shell: nothing to remove"')
	os.system('mkdir -p'+' '+path_output+'save')
	os.system('mkdir -p'+' '+path_target+'save')
	os.system('cd'+' '+path_target+' && '+main_command)
	os.system('cp -r'+' '+path_target+'save/*'+' '+path_output+'save/')
	print('results copied:')
	os.system('touch'+' '+path_output+'.nomedia')
	os.system('ls'+' '+path_target+'save')
	
def result_sync_tts_termux():
	#os.system('rm -r'+' '+path_output+'save || echo "Shell: nothing to remove"')
	#os.system('rm -r'+' '+path_target+'save || echo "Shell: nothing to remove"')
	os.system('mkdir -p'+' '+path_output+'save')
	os.system('mkdir -p'+' '+path_target+'save')
	os.system('cd'+' '+path_target+' && '+main_command_tts_termux)
	os.system('cp -r'+' '+path_target+'save/*'+' '+path_output+'save/')
	print('results copied:')
	os.system('touch'+' '+path_output+'.nomedia')
	os.system('ls'+' '+path_target+'save')
	
def sync(): sync_files(); result_sync()
def sync_tts_termux(): sync_files(); result_sync_tts_termux()
def sync_all(): clear_target(); sync_files()
def check(): sync_files(); os.system('cd'+' '+path_target+' && '+'cargo check')
def clippy_check(): sync_files(); os.system('cd'+' '+path_target+' && '+'cargo clippy')
def deps(): sync_files(); cargo_deps()
def cargo_update(): cl_divider(); os.system('cd'+' '+path_target+' && '+'cargo update')

def explain():
	cl_divider();
	inp = input("Error code » ").strip()
	os.system('rustc --explain'+' '+inp)
	
def rustfmt():
	cl_divider();
	os.system('find'+' '+path_source+' '+'-type f -name "lib.rs" -o -name "main.rs"  | xargs -r rustfmt --check --config-path='+path_source+'rustfmt.toml')
	os.system('find'+' '+path_source+' '+'-type f -name "lib.rs" -o -name "main.rs"  | xargs -r rustfmt --backup --config-path='+path_source+'rustfmt.toml')
	
def clear_bk():
	cl_divider();
	os.system('find'+' '+path_source+' '+'-name "*.bk" -print0 | xargs -r0 rm -rf')
	print('.bk files cleared')
	
def target_tree():
	cl_divider();
	os.system('tree -I target'+' '+path_target)

#Start
os.system("mkdir -p"+" "+path_target) #make folders if none
os.system("mkdir -p"+" "+path_target+"/save")
while True:
	main_menu()