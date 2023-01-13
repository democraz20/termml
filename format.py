import os

path = '.\\'
extension = '.rs'
ignore_dir = ['target']
for dirpath, dirs, files in os.walk(path):
    for i in ignore_dir:
        trimmed_path = dirpath.replace(path, '')
        if trimmed_path.startswith('\\'+i) == False:
            for file in files:
                if file.endswith(extension):
                    os.system(f'rustfmt {dirpath}\{file}')
                    print(f'formatted : {dirpath}\{file}')