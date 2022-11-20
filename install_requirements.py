import platform
import subprocess
import shutil
import time

system = platform.system()

if shutil.which('protoc') is not None:
    print("protoc is already installed")
    quit()

if system == 'Windows':
    if shutil.which('choco') is not None:
        print("Installing protoc with Chocolatey...")
        time.sleep(3)
        subprocess.run(['choco', 'install', 'protoc'], shell=True)
    else:
        print("Chocolatey is not installed. Please install it first")
elif system == 'Darwin':
    if shutil.which('brew') is not None:
        print("Installing protoc with Homebrew...")
        time.sleep(3)
        subprocess.run(['brew', 'install', 'protobuf'], shell=True)
    else:
        print("Homebrew is not installed. Please install it first")
elif system == 'Linux':
    if shutil.which('apt') is not None:
        print('Installing protoc with apt...')
        time.sleep(3)
        subprocess.run(['apt', 'install', 'protobuf-compiler'], shell=True)
    else:
        print('apt is not installed. Your system may use a different package manager than what is supported')
else:
    print('Unsupported operating system')
