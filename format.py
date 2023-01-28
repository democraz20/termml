import os

start = '.\\'
extension = '.rs'
ignore_dir = ['target']
#path to dir, dirs inside, files inside
for dirpath, dirs, files in os.walk(start):
    for file in files:
        if file.endswith(extension):
            path = dirpath+"\\"+file
            for i in ignore_dir:
                if path.startswith(start+i):
                    continue
                else: 
                    os.system(f'rustfmt {path}')
                    print(f'formatted : {dirpath}\{file}') 