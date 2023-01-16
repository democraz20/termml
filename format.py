import os

path = '.\\'
extension = '.rs'
ignore_dir = ['target']
for dirpath, dirs, files in os.walk(path):
    trimmed_path = dirpath.replace(path, '')
    for i in ignore_dir:
        if trimmed_path.startswith('\\'+i):
            continue
        else:
            for file in files:
                if file.endswith(extension):
                    os.system(f'rustfmt {dirpath}\{file}')
                    print(f'formatted : {dirpath}\{file}') 